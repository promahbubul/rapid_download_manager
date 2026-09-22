use crate::error::{RapidError, Result};
use crate::types::Segment;
use futures_util::StreamExt;
use reqwest::header::RANGE;
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
        mut url: String,
        target_path: PathBuf,
        segment: Arc<Mutex<Segment>>,
        progress_tx: UnboundedSender<(usize, u64)>,
        cancel_token: CancellationToken,
        use_range: bool,
        cookies: Option<String>,
        referrer: Option<String>,
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

            if let Some(ref c) = cookies {
                if let Ok(c_val) = reqwest::header::HeaderValue::from_str(c) {
                    req = req.header(reqwest::header::COOKIE, c_val);
                }
            }

            if let Some(ref r) = referrer {
                if let Ok(r_val) = reqwest::header::HeaderValue::from_str(r) {
                    req = req.header(reqwest::header::REFERER, r_val);
                }
            }

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

            // Guard against HTML error/challenge responses (Google BotGuard / Virus warning)
            let content_type = resp
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_lowercase();

            let is_html_response = content_type.contains("text/html");
            let is_expected_html = target_path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.eq_ignore_ascii_case("html") || e.eq_ignore_ascii_case("htm"))
                .unwrap_or(false);

            if is_html_response && !is_expected_html {
                let html_text = resp.text().await.unwrap_or_default();
                if let Some(confirmed_url) = extract_gdrive_confirm_url(&html_text, &url) {
                    url = confirmed_url;
                    sleep(Duration::from_millis(500)).await;
                    continue;
                }

                let err_msg = if html_text.contains("ServiceLogin") || html_text.contains("accounts.google.com") {
                    "Google Drive requires account authentication. Please initiate download from Chrome while logged in.".to_string()
                } else if html_text.contains("Too Many Requests") || html_text.contains("quota") {
                    "Google Drive download quota exceeded or rate limited. Please try again later.".to_string()
                } else {
                    "Google Drive returned an HTML challenge/security page instead of the file. Please trigger the download from Chrome while logged into Google.".to_string()
                };

                return Err(RapidError::Other(err_msg));
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

fn extract_gdrive_confirm_url(html: &str, current_url: &str) -> Option<String> {
    if let Some(pos) = html.find("href=\"/uc?export=download") {
        if let Some(end_pos) = html[pos + 6..].find('"') {
            let link = &html[pos + 6..pos + 6 + end_pos];
            let decoded = link.replace("&amp;", "&");
            return Some(format!("https://drive.google.com{}", decoded));
        }
    }
    if let Some(pos) = html.find("href=\"https://drive.google.com/uc?export=download") {
        if let Some(end_pos) = html[pos + 6..].find('"') {
            let link = &html[pos + 6..pos + 6 + end_pos];
            let decoded = link.replace("&amp;", "&");
            return Some(decoded.to_string());
        }
    }
    if let Some(confirm_pos) = html.find("confirm=") {
        let after = &html[confirm_pos + 8..];
        let token: String = after.chars().take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '-').collect();
        if !token.is_empty() && current_url.contains("drive.google.com") {
            let sep = if current_url.contains('?') { "&" } else { "?" };
            return Some(format!("{}{}confirm={}", current_url, sep, token));
        }
    }
    None
}