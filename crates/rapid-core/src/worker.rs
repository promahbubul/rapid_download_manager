use crate::error::{RapidError, Result};
use crate::types::Segment;
use futures_util::StreamExt;
use reqwest::header::RANGE;
use reqwest::Client;
use std::io::SeekFrom;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
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
        speed_limit: Option<Arc<AtomicU64>>,
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

            // Ensure destination folder exists
            if let Some(parent) = target_path.parent() {
                let _ = tokio::fs::create_dir_all(parent).await;
            }

            // Open target file for random-access writing at current offset
            let file_res = OpenOptions::new()
                .write(true)
                .create(true)
                .open(&target_path)
                .await;

            let mut file = match file_res {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("[Rapid Worker] Failed to open target file {:?}: {}", target_path, e);
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
                let clean_c: String = c.chars().filter(|ch| ch.is_ascii() && !ch.is_ascii_control()).collect();
                if let Ok(c_val) = reqwest::header::HeaderValue::from_str(&clean_c) {
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
                eprintln!("[Rapid Worker] Range request status {}: {}", status, url);
                let err_msg = if status == reqwest::StatusCode::FORBIDDEN {
                    "HTTP 403 Forbidden: Link expired or access restricted (session cookies needed)".to_string()
                } else if status == reqwest::StatusCode::UNAUTHORIZED {
                    "HTTP 401 Unauthorized: Authentication required".to_string()
                } else if status == reqwest::StatusCode::NOT_FOUND {
                    "HTTP 404 Not Found: File not found on server".to_string()
                } else {
                    format!("HTTP status {}", status)
                };
                last_error = Some(RapidError::Other(err_msg));
                sleep(Duration::from_millis(500 * attempts as u64)).await;
                continue;
            } else if !use_range && !status.is_success() {
                eprintln!("[Rapid Worker] Request error status {}: {}", status, url);
                let err_msg = if status == reqwest::StatusCode::FORBIDDEN {
                    "HTTP 403 Forbidden: Link expired or access restricted (session cookies needed)".to_string()
                } else if status == reqwest::StatusCode::UNAUTHORIZED {
                    "HTTP 401 Unauthorized: Authentication required".to_string()
                } else if status == reqwest::StatusCode::NOT_FOUND {
                    "HTTP 404 Not Found: File not found on server".to_string()
                } else {
                    format!("HTTP status {}", status)
                };
                last_error = Some(RapidError::Other(err_msg));
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

                // Bandwidth rate limiter
                if let Some(ref limiter) = speed_limit {
                    let limit_bps = limiter.load(Ordering::Relaxed);
                    if limit_bps > 0 {
                        let delay_ms = (len * 1000) / limit_bps;
                        if delay_ms > 0 {
                            sleep(Duration::from_millis(delay_ms.min(500))).await;
                        }
                    }
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
    crate::gdrive::GDriveResolver::extract_confirm_form_url(html, current_url)
}