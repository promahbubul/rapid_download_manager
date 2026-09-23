use crate::error::{RapidError, Result};
use crate::types::{DownloadProgress, DownloadStatus, Segment};
use futures_util::StreamExt;
use reqwest::header::{COOKIE, REFERER, USER_AGENT};
use reqwest::Client;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncWriteExt;
use tokio::sync::broadcast;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use url::Url;

pub struct HlsDownloader {
    pub id: String,
    pub playlist_url: String,
    pub filename: String,
    pub target_file: PathBuf,
    pub cookies: Option<String>,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
    pub speed_limit: Option<Arc<AtomicU64>>,
    pub cancel_token: CancellationToken,
}

impl HlsDownloader {
    pub fn new(
        id: String,
        playlist_url: String,
        filename: String,
        target_file: PathBuf,
        cookies: Option<String>,
        referrer: Option<String>,
        user_agent: Option<String>,
        speed_limit: Option<Arc<AtomicU64>>,
        cancel_token: CancellationToken,
    ) -> Self {
        Self {
            id,
            playlist_url,
            filename,
            target_file,
            cookies,
            referrer,
            user_agent,
            speed_limit,
            cancel_token,
        }
    }

    /// Check if a URL or filename indicates an HLS stream
    pub fn is_hls(url: &str) -> bool {
        let clean = url.split('?').next().unwrap_or(url).to_lowercase();
        clean.ends_with(".m3u8") || url.to_lowercase().contains(".m3u8?")
    }

    /// Parse HLS playlist (handles both Master and Media playlists)
    pub async fn fetch_and_parse_playlist(
        client: &Client,
        url_str: &str,
        cookies: Option<&str>,
        referrer: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<(String, Vec<String>)> {
        let mut req = client.get(url_str);
        if let Some(c) = cookies {
            if let Ok(v) = reqwest::header::HeaderValue::from_str(c) {
                req = req.header(COOKIE, v);
            }
        }
        if let Some(r) = referrer {
            if let Ok(v) = reqwest::header::HeaderValue::from_str(r) {
                req = req.header(REFERER, v);
            }
        }
        if let Some(ua) = user_agent {
            if let Ok(v) = reqwest::header::HeaderValue::from_str(ua) {
                req = req.header(USER_AGENT, v);
            }
        }

        let resp = req.send().await.map_err(RapidError::Network)?;
        if !resp.status().is_success() {
            return Err(RapidError::Other(format!(
                "HLS playlist request failed with status: {}",
                resp.status()
            )));
        }

        let body = resp.text().await.map_err(RapidError::Network)?;
        let base_url = Url::parse(url_str)
            .map_err(|e| RapidError::InvalidUrl(format!("{}: {}", url_str, e)))?;

        // 1. Check if this is a Master Playlist (contains variant streams)
        if body.contains("#EXT-X-STREAM-INF:") {
            let mut best_variant_url: Option<String> = None;
            let mut highest_bandwidth: u64 = 0;
            let lines: Vec<&str> = body.lines().collect();

            for i in 0..lines.len() {
                let line = lines[i].trim();
                if line.starts_with("#EXT-X-STREAM-INF:") {
                    // Extract BANDWIDTH=...
                    let mut bw = 0u64;
                    for part in line.split(',') {
                        if part.contains("BANDWIDTH=") {
                            if let Some(bw_str) = part.split('=').nth(1) {
                                bw = bw_str.trim().parse::<u64>().unwrap_or(0);
                            }
                        }
                    }

                    // Next non-comment line is the playlist URL
                    let mut j = i + 1;
                    while j < lines.len() {
                        let next_line = lines[j].trim();
                        if !next_line.is_empty() && !next_line.starts_with('#') {
                            if bw >= highest_bandwidth || best_variant_url.is_none() {
                                highest_bandwidth = bw;
                                let resolved = base_url
                                    .join(next_line)
                                    .map(|u| u.to_string())
                                    .unwrap_or_else(|_| next_line.to_string());
                                best_variant_url = Some(resolved);
                            }
                            break;
                        }
                        j += 1;
                    }
                }
            }

            if let Some(variant_url) = best_variant_url {
                // Recursively fetch the selected media playlist
                return Box::pin(Self::fetch_and_parse_playlist(
                    client,
                    &variant_url,
                    cookies,
                    referrer,
                    user_agent,
                ))
                .await;
            }
        }

        // 2. Parse Media Playlist segments
        let mut segments = Vec::new();
        let mut init_segment: Option<String> = None;

        for line in body.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Check for fMP4 initialization segment (#EXT-X-MAP:URI="...")
            if trimmed.starts_with("#EXT-X-MAP:") {
                for part in trimmed.split(',') {
                    if part.contains("URI=") {
                        if let Some(uri_part) = part.split('=').nth(1) {
                            let clean_uri = uri_part.trim_matches('"').trim();
                            if let Ok(resolved) = base_url.join(clean_uri) {
                                init_segment = Some(resolved.to_string());
                            }
                        }
                    }
                }
                continue;
            }

            // Ignore comments and tags
            if trimmed.starts_with('#') {
                continue;
            }

            // Segment URL
            match base_url.join(trimmed) {
                Ok(resolved) => segments.push(resolved.to_string()),
                Err(_) => segments.push(trimmed.to_string()),
            }
        }

        if let Some(init) = init_segment {
            segments.insert(0, init);
        }

        Ok((url_str.to_string(), segments))
    }

    /// Execute the HLS stream download, joining all chunks into the target file
    pub async fn run(
        &self,
        progress_tx: broadcast::Sender<DownloadProgress>,
    ) -> Result<()> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(RapidError::Network)?;

        // Ensure parent destination directory exists
        if let Some(parent) = self.target_file.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Fetch and parse all segment URLs
        let (_resolved_playlist_url, segment_urls) = Self::fetch_and_parse_playlist(
            &client,
            &self.playlist_url,
            self.cookies.as_deref(),
            self.referrer.as_deref(),
            self.user_agent.as_deref(),
        )
        .await?;

        let total_segments = segment_urls.len();
        if total_segments == 0 {
            return Err(RapidError::Other(
                "No media segments found in HLS playlist".to_string(),
            ));
        }

        // Open target file for writing chunks
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.target_file)
            .await?;

        // Initialize UI segments
        let mut ui_segments: Vec<Segment> = (0..total_segments)
            .map(|i| Segment::new(i, i as u64, (i + 1) as u64))
            .collect();

        let mut total_downloaded_bytes: u64 = 0;
        let mut speed_calc_bytes: u64 = 0;
        let mut last_speed_check = Instant::now();
        let mut current_speed_bps: u64 = 0;

        for (idx, seg_url) in segment_urls.iter().enumerate() {
            if self.cancel_token.is_cancelled() {
                return Err(RapidError::Cancelled);
            }

            // Download segment with up to 3 retries
            let mut attempts = 0;
            let mut seg_data: Option<Vec<u8>> = None;

            while attempts < 3 {
                attempts += 1;
                let mut req = client.get(seg_url);
                if let Some(ref c) = self.cookies {
                    if let Ok(v) = reqwest::header::HeaderValue::from_str(c) {
                        req = req.header(COOKIE, v);
                    }
                }
                if let Some(ref r) = self.referrer {
                    if let Ok(v) = reqwest::header::HeaderValue::from_str(r) {
                        req = req.header(REFERER, v);
                    }
                }
                if let Some(ref ua) = self.user_agent {
                    if let Ok(v) = reqwest::header::HeaderValue::from_str(ua) {
                        req = req.header(USER_AGENT, v);
                    }
                }

                match req.send().await {
                    Ok(resp) if resp.status().is_success() => {
                        let mut stream = resp.bytes_stream();
                        let mut chunk_buf = Vec::new();

                        while let Some(chunk_res) = stream.next().await {
                            if self.cancel_token.is_cancelled() {
                                return Err(RapidError::Cancelled);
                            }

                            if let Ok(bytes) = chunk_res {
                                let len = bytes.len() as u64;
                                chunk_buf.extend_from_slice(&bytes);
                                total_downloaded_bytes += len;
                                speed_calc_bytes += len;

                                // Bandwidth rate limiter
                                if let Some(ref limiter) = self.speed_limit {
                                    let limit_bps = limiter.load(Ordering::Relaxed);
                                    if limit_bps > 0 {
                                        let delay_ms = (len * 1000) / limit_bps;
                                        if delay_ms > 0 {
                                            sleep(Duration::from_millis(delay_ms.min(500))).await;
                                        }
                                    }
                                }
                            }
                        }
                        seg_data = Some(chunk_buf);
                        break;
                    }
                    _ => {
                        sleep(Duration::from_millis(400 * attempts as u64)).await;
                    }
                }
            }

            let data = match seg_data {
                Some(d) => d,
                None if attempts >= 3 => {
                    return Err(RapidError::Other(format!(
                        "Failed to download HLS segment {} after 3 retries",
                        idx + 1
                    )));
                }
                _ => Vec::new(),
            };

            // Write chunk to final video file
            file.write_all(&data).await?;

            // Mark segment as complete
            if idx < ui_segments.len() {
                ui_segments[idx].is_complete = true;
                ui_segments[idx].downloaded_bytes = data.len() as u64;
            }

            // Calculate live speed and ETA
            let now = Instant::now();
            let elapsed = now.duration_since(last_speed_check).as_millis();
            if elapsed >= 500 {
                current_speed_bps = ((speed_calc_bytes as f64 / (elapsed as f64 / 1000.0)) as u64).max(0);
                speed_calc_bytes = 0;
                last_speed_check = now;
            }

            let progress_percent = ((idx + 1) as f32 / total_segments as f32) * 100.0;
            let remaining_segs = total_segments.saturating_sub(idx + 1);
            let avg_seg_size = if idx > 0 {
                total_downloaded_bytes / (idx as u64 + 1)
            } else {
                data.len() as u64
            };
            let estimated_remaining_bytes = remaining_segs as u64 * avg_seg_size;
            let eta_seconds = if current_speed_bps > 0 {
                Some(estimated_remaining_bytes / current_speed_bps)
            } else {
                None
            };

            let prog = DownloadProgress {
                id: self.id.clone(),
                filename: self.filename.clone(),
                total_bytes: Some(total_downloaded_bytes + estimated_remaining_bytes),
                downloaded_bytes: total_downloaded_bytes,
                progress_percent,
                speed_bps: current_speed_bps,
                eta_seconds,
                status: DownloadStatus::Downloading,
                segments: ui_segments.clone(),
            };

            let _ = progress_tx.send(prog);
        }

        file.flush().await?;
        file.sync_all().await?;

        // Emit final completion progress
        let complete_prog = DownloadProgress {
            id: self.id.clone(),
            filename: self.filename.clone(),
            total_bytes: Some(total_downloaded_bytes),
            downloaded_bytes: total_downloaded_bytes,
            progress_percent: 100.0,
            speed_bps: 0,
            eta_seconds: None,
            status: DownloadStatus::Completed,
            segments: ui_segments,
        };
        let _ = progress_tx.send(complete_prog);

        Ok(())
    }
}
