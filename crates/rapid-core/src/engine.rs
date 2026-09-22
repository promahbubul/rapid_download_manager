use crate::error::{RapidError, Result};
use crate::probe::Probe;
use crate::segment::SegmentPlanner;
use crate::types::{
    DownloadConfig, DownloadProgress, DownloadStatus, DownloadTaskState, Segment,
};
use crate::worker::DownloadWorker;
use chrono::Utc;
use reqwest::Client;
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
}

impl DownloadTask {
    pub async fn create(id: String, config: DownloadConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        // 1. Probe the URL for metadata
        let metadata = Probe::inspect(&client, &config.url).await?;

        let filename = config
            .custom_filename
            .clone()
            .unwrap_or_else(|| {
                Self::resolve_unique_filename(&config.output_dir, &metadata.filename)
            });

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
                metadata.content_length,
                config.num_segments,
                metadata.accept_ranges,
            );

            // Pre-allocate destination file if size is known
            if let Some(size) = metadata.content_length {
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
            (segs, metadata.content_length, metadata.accept_ranges)
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

        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()?;

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

                let handle = tokio::spawn(async move {
                    DownloadWorker::run(worker_client, url, path, seg, tx, token, use_range).await
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
            created_at: Utc::now(),
            updated_at: Utc::now(),
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
