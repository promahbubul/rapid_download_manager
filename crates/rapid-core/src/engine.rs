use crate::error::{RapidError, Result};
use crate::probe::Probe;
use crate::segment::SegmentPlanner;
use crate::types::{
    DownloadConfig, DownloadProgress, DownloadStatus, DownloadTaskState, Segment,
};
use crate::worker::DownloadWorker;
use chrono::{DateTime, Utc};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE, USER_AGENT, REFERER};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::fs::{self, OpenOptions};
use tokio::sync::{broadcast, mpsc, Mutex};
use tokio_util::sync::CancellationToken;

pub struct DownloadTask {
    pub id: String,
    pub config: DownloadConfig,
    pub target_file: PathBuf,
    pub total_bytes: Option<u64>,
    pub accept_ranges: bool,
    pub segments: Vec<Arc<Mutex<Segment>>>,
    pub status: Arc<Mutex<DownloadStatus>>,
    cancel_token: CancellationToken,
    progress_tx: broadcast::Sender<DownloadProgress>,
    created_at: DateTime<Utc>,
}

impl DownloadTask {
    pub async fn create(id: String, mut config: DownloadConfig) -> Result<Self> {
        let is_gdrive = config.is_gdrive
            || config.url.contains("drive.google.com")
            || config.url.contains("googleusercontent.com")
            || config.url.contains("usercontent.google.com")
            || config.url.contains("docs.google.com");
        config.is_gdrive = is_gdrive;
        if is_gdrive {
            config.num_segments = 1;
        }

        let client = build_client_with_config(&config, 30)?;

        let (final_url, filename, total_bytes_probe, accept_ranges_probe) = if is_gdrive {
            let res_type = crate::gdrive::GDriveResolver::parse_resource_type(&config.url);
            match res_type {
                crate::gdrive::GDriveResourceType::File(file_id) => {
                    match crate::gdrive::GDriveResolver::resolve_file_download_url(&client, &file_id).await {
                        Ok(resolved) => {
                            let fname = config
                                .custom_filename
                                .clone()
                                .filter(|s| !s.is_empty())
                                .unwrap_or(resolved.name);
                            (resolved.download_url, fname, resolved.size_bytes, false)
                        }
                        Err(_) => {
                            let resolved_filename = config
                                .custom_filename
                                .clone()
                                .filter(|s| !s.is_empty())
                                .unwrap_or_else(|| {
                                    Self::resolve_unique_filename(&config.output_dir, "Google_Drive_Download.zip")
                                });
                            (config.url.clone(), resolved_filename, None, false)
                        }
                    }
                }
                _ => {
                    let resolved_filename = config
                        .custom_filename
                        .clone()
                        .filter(|s| !s.is_empty())
                        .unwrap_or_else(|| {
                            Self::resolve_unique_filename(&config.output_dir, "Google_Drive_Download.zip")
                        });
                    (config.url.clone(), resolved_filename, None, false)
                }
            }
        } else {
            let metadata = Probe::inspect(&client, &config.url).await?;
            let resolved_filename = config
                .custom_filename
                .clone()
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| {
                    Self::resolve_unique_filename(&config.output_dir, &metadata.filename)
                });
            (
                metadata.url,
                resolved_filename,
                metadata.content_length,
                metadata.accept_ranges,
            )
        };

        config.url = final_url;

        fs::create_dir_all(&config.output_dir).await?;
        let target_file = config.output_dir.join(&filename);

        // 2. Check if a previous download manifest exists for resume
        let (segments, total_bytes, accept_ranges) = if let Ok(Some(existing)) =
            SegmentPlanner::load_manifest(&target_file).await
        {
            let segs = existing
                .segments
                .into_iter()
                .map(|s| Arc::new(Mutex::new(s)))
                .collect();
            (segs, existing.total_bytes, existing.accept_ranges)
        } else {
            // Plan fresh segments
            let seg_list = SegmentPlanner::plan(
                total_bytes_probe,
                config.num_segments,
                accept_ranges_probe,
            );

            // Pre-allocate destination file if size is known
            if let Some(size) = total_bytes_probe {
                if size > 0 {
                    let file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(&target_file)
                        .await?;
                    file.set_len(size).await?;
                }
            }

            let segs = seg_list
                .into_iter()
                .map(|s| Arc::new(Mutex::new(s)))
                .collect();
            (segs, total_bytes_probe, accept_ranges_probe)
        };

        let (progress_tx, _) = broadcast::channel(100);

        Ok(Self {
            id,
            config,
            target_file,
            total_bytes,
            accept_ranges,
            segments,
            status: Arc::new(Mutex::new(DownloadStatus::Queued)),
            cancel_token: CancellationToken::new(),
            progress_tx,
            created_at: Utc::now(),
        })
    }

    pub fn subscribe(&self) -> broadcast::Receiver<DownloadProgress> {
        self.progress_tx.subscribe()
    }

    pub fn cancel_token(&self) -> CancellationToken {
        self.cancel_token.clone()
    }

    pub async fn current_status(&self) -> DownloadStatus {
        self.status.lock().await.clone()
    }

    pub fn pause(&self) {
        self.cancel_token.cancel();
    }

    pub async fn run(&self) -> Result<()> {
        {
            let mut s = self.status.lock().await;
            *s = DownloadStatus::Downloading;
        }

        let client = build_client_with_config(&self.config, 60)?;

        let (chunk_tx, mut chunk_rx) = mpsc::unbounded_channel::<(usize, u64)>();

        // Spawn a worker for each incomplete segment
        let mut worker_handles = Vec::new();

        for seg_arc in &self.segments {
            let is_complete = {
                let seg = seg_arc.lock().await;
                seg.is_complete
            };

            if !is_complete {
                let worker_client = client.clone();
                let url = self.config.url.clone();
                let path = self.target_file.clone();
                let seg = Arc::clone(seg_arc);
                let tx = chunk_tx.clone();
                let token = self.cancel_token.clone();
                let use_range = self.accept_ranges;
                let worker_cookies = self.config.cookies.clone();
                let worker_referrer = self.config.referrer.clone();

                let handle = tokio::spawn(async move {
                    DownloadWorker::run(worker_client, url, path, seg, tx, token, use_range, worker_cookies, worker_referrer).await
                });
                worker_handles.push(handle);
            }
        }

        drop(chunk_tx); // Close original tx so rx knows when all workers finish

        // Monitor progress and metrics
        let mut last_tick = Instant::now();
        let mut bytes_since_last_tick: u64 = 0;

        let filename = self
            .target_file
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "download".to_string());

        let mut manifest_save_timer = Instant::now();

        // Main progress loop
        loop {
            tokio::select! {
                Some((_seg_idx, bytes_read)) = chunk_rx.recv() => {
                    bytes_since_last_tick += bytes_read;

                    if last_tick.elapsed() >= Duration::from_millis(500) {
                        let elapsed_sec = last_tick.elapsed().as_secs_f64();
                        let speed_bps = if elapsed_sec > 0.0 {
                            (bytes_since_last_tick as f64 / elapsed_sec) as u64
                        } else {
                            0
                        };
                        bytes_since_last_tick = 0;
                        last_tick = Instant::now();

                        let progress = self.calculate_progress(&filename, speed_bps).await;
                        let _ = self.progress_tx.send(progress);
                    }

                    // Save state checkpoint every 2 seconds
                    if manifest_save_timer.elapsed() >= Duration::from_secs(2) {
                        manifest_save_timer = Instant::now();
                        let _ = self.save_state().await;
                    }
                }
                else => {
                    // All workers completed or aborted
                    break;
                }
            }
        }

        // Wait for workers and collect errors
        let mut any_cancelled = false;
        let mut first_error = None;

        for handle in worker_handles {
            match handle.await {
                Ok(Ok(())) => {}
                Ok(Err(RapidError::Cancelled)) => {
                    any_cancelled = true;
                }
                Ok(Err(e)) => {
                    if first_error.is_none() {
                        first_error = Some(e);
                    }
                }
                Err(join_err) => {
                    if first_error.is_none() {
                        first_error = Some(RapidError::Other(format!("Worker panicked: {}", join_err)));
                    }
                }
            }
        }

        // Final state evaluation
        let is_fully_completed = self.is_all_segments_complete().await;

        let final_status = if is_fully_completed {
            DownloadStatus::Completed
        } else if any_cancelled || self.cancel_token.is_cancelled() {
            DownloadStatus::Paused
        } else if let Some(e) = first_error {
            DownloadStatus::Failed(e.to_string())
        } else {
            DownloadStatus::Failed("Download incomplete: connection interrupted".to_string())
        };

        {
            let mut s = self.status.lock().await;
            *s = final_status.clone();
        }

        // Save or clean up manifest
        if final_status == DownloadStatus::Completed {
            let _ = SegmentPlanner::delete_manifest(&self.target_file).await;
        } else {
            let _ = self.save_state().await;
        }

        // Emit final progress notification
        let final_progress = self.calculate_progress(&filename, 0).await;
        let _ = self.progress_tx.send(final_progress);

        match final_status {
            DownloadStatus::Completed => Ok(()),
            DownloadStatus::Paused => Err(RapidError::Cancelled),
            DownloadStatus::Failed(msg) => Err(RapidError::Other(msg)),
            _ => Ok(()),
        }
    }

    pub async fn snapshot_segments(&self) -> Vec<Segment> {
        let mut list = Vec::with_capacity(self.segments.len());
        for seg_arc in &self.segments {
            let s = seg_arc.lock().await;
            list.push(s.clone());
        }
        list
    }

    pub async fn is_all_segments_complete(&self) -> bool {
        for seg_arc in &self.segments {
            let s = seg_arc.lock().await;
            if !s.is_complete {
                return false;
            }
        }
        true
    }

    async fn calculate_progress(&self, filename: &str, speed_bps: u64) -> DownloadProgress {
        let segments = self.snapshot_segments().await;
        let downloaded: u64 = segments.iter().map(|s| s.downloaded_bytes).sum();
        let total = self.total_bytes;

        let progress_percent = match total {
            Some(t) if t > 0 => ((downloaded as f32 / t as f32) * 100.0).clamp(0.0, 100.0),
            _ => 0.0,
        };

        let eta_seconds = match (total, speed_bps) {
            (Some(t), s) if s > 0 && t >= downloaded => Some((t - downloaded) / s),
            _ => None,
        };

        let current_status = self.status.lock().await.clone();

        DownloadProgress {
            id: self.id.clone(),
            filename: filename.to_string(),
            total_bytes: total,
            downloaded_bytes: downloaded,
            progress_percent,
            speed_bps,
            eta_seconds,
            status: current_status,
            segments,
        }
    }

    async fn save_state(&self) -> Result<()> {
        let segments = self.snapshot_segments().await;
        let current_status = self.status.lock().await.clone();

        let state = DownloadTaskState {
            id: self.id.clone(),
            url: self.config.url.clone(),
            target_file: self.target_file.clone(),
            total_bytes: self.total_bytes,
            accept_ranges: self.accept_ranges,
            segments,
            status: current_status,
            created_at: self.created_at,
            updated_at: Utc::now(),
            cookies: self.config.cookies.clone(),
            referrer: self.config.referrer.clone(),
            user_agent: self.config.user_agent.clone(),
        };

        SegmentPlanner::save_manifest(&state).await
    }

    pub fn resolve_unique_filename(dir: &std::path::Path, base_name: &str) -> String {
        let target = dir.join(base_name);
        let manifest = DownloadTaskState::manifest_path(&target);

        if !target.exists() && !manifest.exists() {
            return base_name.to_string();
        }

        let path = std::path::Path::new(base_name);
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or(base_name);
        let ext = path.extension().and_then(|s| s.to_str());

        for i in 1..10000 {
            let candidate = match ext {
                Some(e) => format!("{} ({}).{}", stem, i, e),
                None => format!("{} ({})", stem, i),
            };
            let c_target = dir.join(&candidate);
            let c_manifest = DownloadTaskState::manifest_path(&c_target);
            if !c_target.exists() && !c_manifest.exists() {
                return candidate;
            }
        }
        base_name.to_string()
    }
}


fn build_client_with_config(config: &DownloadConfig, timeout_secs: u64) -> Result<Client> {
    let mut headers = HeaderMap::new();
    
    if let Some(ua) = &config.user_agent {
        if let Ok(val) = HeaderValue::from_str(ua) {
            headers.insert(USER_AGENT, val);
        }
    } else {
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36"));
    }
    
    // Additional browser-like headers to bypass BotGuard / 403 Forbidden
    headers.insert(reqwest::header::ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    headers.insert(reqwest::header::ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert("Sec-Ch-Ua", HeaderValue::from_static("\"Chromium\";v=\"128\", \"Not;A=Brand\";v=\"24\", \"Google Chrome\";v=\"128\""));
    headers.insert("Sec-Ch-Ua-Mobile", HeaderValue::from_static("?0"));
    headers.insert("Sec-Ch-Ua-Platform", HeaderValue::from_static("\"Windows\""));
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("document"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("navigate"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("none"));
    headers.insert("Sec-Fetch-User", HeaderValue::from_static("?1"));
    headers.insert("Upgrade-Insecure-Requests", HeaderValue::from_static("1"));
    
    // We intentionally DO NOT add the COOKIE header to default_headers here.
    // reqwest strips sensitive headers (like Cookie) on cross-domain redirects.
    // Instead, we will use a reqwest::cookie::Jar to handle them natively below.
    
    if let Some(cookie_str) = &config.cookies {
        if let Ok(c_val) = HeaderValue::from_str(cookie_str) {
            headers.insert(COOKIE, c_val);
        }
    }

    if let Some(referrer) = &config.referrer {
        if let Ok(val) = HeaderValue::from_str(referrer) {
            headers.insert(REFERER, val);
            headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-site"));
        }
    } else if config.is_gdrive {
        headers.insert(REFERER, HeaderValue::from_static("https://drive.google.com/"));
        headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-site"));
    }
    
    let mut client_builder = Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .default_headers(headers);

    // Setup Cookie Jar for redirects
    if let Some(cookie_str) = &config.cookies {
        let jar = reqwest::cookie::Jar::default();
        if let Ok(url) = reqwest::Url::parse(&config.url) {
            for c in cookie_str.split(';') {
                let trimmed = c.trim();
                if !trimmed.is_empty() {
                    jar.add_cookie_str(trimmed, &url);
                    if config.is_gdrive || config.url.contains("google") {
                        if let Ok(g_url) = reqwest::Url::parse("https://drive.google.com") {
                            jar.add_cookie_str(trimmed, &g_url);
                        }
                        if let Ok(c_url) = reqwest::Url::parse("https://googleusercontent.com") {
                            jar.add_cookie_str(trimmed, &c_url);
                        }
                        if let Ok(u_url) = reqwest::Url::parse("https://usercontent.google.com") {
                            jar.add_cookie_str(trimmed, &u_url);
                        }
                        if let Ok(t_url) = reqwest::Url::parse("https://takeout-download-drive.usercontent.google.com") {
                            jar.add_cookie_str(trimmed, &t_url);
                        }
                        if let Ok(root_g) = reqwest::Url::parse("https://google.com") {
                            jar.add_cookie_str(trimmed, &root_g);
                        }
                    }
                }
            }
        }
        client_builder = client_builder.cookie_provider(Arc::new(jar));
    }
    
    client_builder
        .build()
        .map_err(|e| crate::error::RapidError::Network(e))
}
