use crate::error::{RapidError, Result};
use crate::types::Segment;
use futures_util::StreamExt;
use reqwest::header::{RANGE, USER_AGENT};
use reqwest::Client;
use std::io::SeekFrom;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

pub struct DownloadWorker;

impl DownloadWorker {
    pub async fn run(
        client: Client,
        url: String,
        target_path: PathBuf,
        segment: Arc<Mutex<Segment>>,
        progress_tx: UnboundedSender<(usize, u64)>,
        cancel_token: CancellationToken,
        use_range: bool,
    ) -> Result<()> {
        let (seg_index, start_byte, end_byte, mut downloaded) = {
            let s = segment.lock().await;
            (s.index, s.start_byte, s.end_byte, s.downloaded_bytes)
        };

        let max_retries = 5;
        let mut attempts = 0;
        let mut last_error: Option<RapidError> = None;

        while attempts < max_retries {
            if cancel_token.is_cancelled() {
                return Err(RapidError::Cancelled);
            }

            let current_offset = start_byte + downloaded;
            if use_range && end_byte > 0 && current_offset > end_byte {
                let mut s = segment.lock().await;
                s.is_complete = true;
                return Ok(());
            }

            attempts += 1;

            // Open target file for random-access writing at current offset
            let file_res = OpenOptions::new()
                .write(true)
                .create(true)
                .open(&target_path)
                .await;

            let mut file = match file_res {
                Ok(f) => f,
                Err(e) => {
                    last_error = Some(RapidError::Io(e));
                    sleep(Duration::from_millis(500 * attempts as u64)).await;
                    continue;
                }
            };

            if let Err(e) = file.seek(SeekFrom::Start(current_offset)).await {
                last_error = Some(RapidError::Io(e));
                sleep(Duration::from_millis(500 * attempts as u64)).await;
                continue;
            }

            let mut req = client.get(&url);

            if use_range && end_byte > 0 {
                let range_val = format!("bytes={}-{}", current_offset, end_byte);
                req = req.header(RANGE, range_val);
            }

            let resp = match req.send().await {
                Ok(r) => r,
                Err(e) => {
                    last_error = Some(RapidError::Network(e));
                    sleep(Duration::from_millis(500 * attempts as u64)).await;
                    continue;
                }
            };

            let status = resp.status();
            if use_range && status != reqwest::StatusCode::PARTIAL_CONTENT && status != reqwest::StatusCode::OK {
                last_error = Some(RapidError::HttpStatus(status));
                sleep(Duration::from_millis(500 * attempts as u64)).await;
                continue;
            } else if !use_range && !status.is_success() {
                last_error = Some(RapidError::HttpStatus(status));
                sleep(Duration::from_millis(500 * attempts as u64)).await;
                continue;
            }

            let mut stream = resp.bytes_stream();
            let mut stream_interrupted = false;

            while let Some(chunk_result) = stream.next().await {
                if cancel_token.is_cancelled() {
                    let _ = file.flush().await;
                    return Err(RapidError::Cancelled);
                }

                let chunk = match chunk_result {
                    Ok(c) => c,
                    Err(e) => {
                        last_error = Some(RapidError::Network(e));
                        stream_interrupted = true;
                        break;
                    }
                };

                let len = chunk.len() as u64;

                if let Err(e) = file.write_all(&chunk).await {
                    last_error = Some(RapidError::Io(e));
                    stream_interrupted = true;
                    break;
                }

                downloaded += len;
                {
                    let mut s = segment.lock().await;
                    s.downloaded_bytes = downloaded;
                    if use_range && end_byte > 0 && downloaded >= s.total_bytes() {
                        s.is_complete = true;
                    }
                }

                let _ = progress_tx.send((seg_index, len));
            }

            let _ = file.flush().await;

            if stream_interrupted {
                // Network stream broke mid-way: backoff and retry remaining bytes from updated offset
                sleep(Duration::from_millis(500 * attempts as u64)).await;
                continue;
            }

            // Successfully received full stream for this segment
            let mut s = segment.lock().await;
            if use_range && end_byte > 0 {
                s.is_complete = s.downloaded_bytes >= s.total_bytes();
            } else {
                s.is_complete = true;
            }

            return Ok(());
        }

        Err(last_error.unwrap_or_else(|| RapidError::Other("Maximum retry attempts reached".to_string())))
    }
}
