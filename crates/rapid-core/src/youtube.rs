use crate::error::{RapidError, Result};
use crate::types::{DownloadProgress, DownloadStatus, Segment};
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct YoutubeMetadata {
    pub id: String,
    pub title: String,
    pub duration_seconds: Option<u64>,
    pub clean_filename: String,
    pub original_url: String,
}

pub struct YoutubeResolver;

impl YoutubeResolver {
    /// Detect if a URL or referrer corresponds to YouTube
    pub fn is_youtube(url_or_ref: &str) -> bool {
        let lower = url_or_ref.to_lowercase();
        lower.contains("youtube.com/watch")
            || lower.contains("youtu.be/")
            || lower.contains("youtube.com/shorts/")
            || lower.contains("youtube.com/live/")
            || lower.contains("youtube.com/embed/")
            || (lower.contains("googlevideo.com") && lower.contains("videoplayback"))
    }

    /// Extract or canonicalize a YouTube page URL
    pub fn canonicalize_url(url: &str, referrer: Option<&str>) -> String {
        if let Some(ref_url) = referrer {
            if Self::is_youtube(ref_url) && !ref_url.contains("googlevideo.com") {
                return ref_url.to_string();
            }
        }
        if url.contains("youtu.be/") {
            if let Some(id) = url.split("youtu.be/").nth(1) {
                let clean_id = id.split('?').next().unwrap_or(id).trim_matches('/');
                if !clean_id.is_empty() {
                    return format!("https://www.youtube.com/watch?v={}", clean_id);
                }
            }
        }
        if url.contains("/shorts/") {
            if let Some(id) = url.split("/shorts/").nth(1) {
                let clean_id = id.split('?').next().unwrap_or(id).trim_matches('/');
                if !clean_id.is_empty() {
                    return format!("https://www.youtube.com/watch?v={}", clean_id);
                }
            }
        }
        url.to_string()
    }

    /// Resolve YouTube video title and metadata using yt-dlp
    pub async fn resolve_metadata(url: &str) -> Result<YoutubeMetadata> {
        let ytdlp_bin = Self::find_ytdlp_binary().ok_or_else(|| {
            RapidError::Other("yt-dlp executable not found on system".to_string())
        })?;

        let mut cmd = tokio::process::Command::new(ytdlp_bin);
        cmd.args(&[
            "--js-runtimes",
            "node",
            "--no-playlist",
            "--print",
            "%(title)s@@@%(id)s@@@%(duration)s",
            url,
        ]);

        #[cfg(windows)]
        {
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        let output = cmd.output().await.map_err(|e| {
            RapidError::Other(format!("Failed to execute yt-dlp resolver: {}", e))
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(RapidError::Other(format!(
                "yt-dlp failed to resolve video metadata: {}",
                stderr.trim()
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let first_line = stdout.lines().next().unwrap_or("").trim();
        let parts: Vec<&str> = first_line.split("@@@").collect();

        let title = parts.get(0).unwrap_or(&"YouTube_Video").trim().to_string();
        let id = parts.get(1).unwrap_or(&"video").trim().to_string();
        let duration_seconds = parts.get(2).and_then(|d| d.trim().parse::<u64>().ok());

        let clean_filename = sanitize_filename(&title);

        Ok(YoutubeMetadata {
            id,
            title,
            duration_seconds,
            clean_filename: format!("{}.mp4", clean_filename),
            original_url: url.to_string(),
        })
    }

    /// Locate yt-dlp binary across common locations
    pub fn find_ytdlp_binary() -> Option<PathBuf> {
        // 1. Next to current executable
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(dir) = exe_path.parent() {
                let candidate = dir.join("yt-dlp.exe");
                if candidate.exists() {
                    return Some(candidate);
                }
                let candidate = dir.join("yt-dlp");
                if candidate.exists() {
                    return Some(candidate);
                }
            }
        }

        // 2. Current working directory
        let cwd_candidate = PathBuf::from("yt-dlp.exe");
        if cwd_candidate.exists() {
            return Some(cwd_candidate);
        }

        // 3. Known project directory
        let proj_candidate = PathBuf::from(r"D:\mahbub\project\rapid_download_manager\yt-dlp.exe");
        if proj_candidate.exists() {
            return Some(proj_candidate);
        }

        // 4. PATH search
        if let Ok(path_var) = std::env::var("PATH") {
            for dir in std::env::split_paths(&path_var) {
                let c = dir.join("yt-dlp.exe");
                if c.exists() {
                    return Some(c);
                }
                let c = dir.join("yt-dlp");
                if c.exists() {
                    return Some(c);
                }
            }
        }

        None
    }

    /// Locate ffmpeg binary or directory
    pub fn find_ffmpeg_location() -> Option<PathBuf> {
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(dir) = exe_path.parent() {
                let candidate = dir.join("ffmpeg.exe");
                if candidate.exists() {
                    return Some(candidate);
                }
            }
        }
        let proj_candidate = PathBuf::from(r"D:\mahbub\project\rapid_download_manager\ffmpeg.exe");
        if proj_candidate.exists() {
            return Some(proj_candidate);
        }
        if let Ok(path_var) = std::env::var("PATH") {
            for dir in std::env::split_paths(&path_var) {
                let c = dir.join("ffmpeg.exe");
                if c.exists() {
                    return Some(c);
                }
            }
        }
        None
    }
}

pub struct YoutubeDownloader {
    pub id: String,
    pub url: String,
    pub filename: String,
    pub target_file: PathBuf,
    pub speed_limit: Option<Arc<AtomicU64>>,
    pub cancel_token: CancellationToken,
}

impl YoutubeDownloader {
    pub fn new(
        id: String,
        url: String,
        filename: String,
        target_file: PathBuf,
        speed_limit: Option<Arc<AtomicU64>>,
        cancel_token: CancellationToken,
    ) -> Self {
        Self {
            id,
            url,
            filename,
            target_file,
            speed_limit,
            cancel_token,
        }
    }

    pub fn is_youtube(url: &str) -> bool {
        YoutubeResolver::is_youtube(url)
    }

    pub async fn run(&self, progress_tx: broadcast::Sender<DownloadProgress>) -> Result<()> {
        let ytdlp_bin = YoutubeResolver::find_ytdlp_binary().ok_or_else(|| {
            RapidError::Other("yt-dlp executable not found. Please ensure yt-dlp.exe is in the application folder.".to_string())
        })?;

        if let Some(parent) = self.target_file.parent() {
            fs::create_dir_all(parent).await?;
        }

        let out_target = self.target_file.to_string_lossy().to_string();

        let mut cmd = tokio::process::Command::new(ytdlp_bin);
        cmd.args(&[
            "--js-runtimes",
            "node",
            "-N",
            "8",
            "--newline",
            "--progress-template",
            "download:%(progress.downloaded_bytes)s/%(progress.total_bytes)s/%(progress.speed)s/%(progress.eta)s",
            "--no-playlist",
            "-f",
            "bv*[ext=mp4]+ba[ext=m4a]/b[ext=mp4]/bv*+ba/b/best",
            "--merge-output-format",
            "mp4",
            "-o",
            &out_target,
            &self.url,
        ]);

        if let Some(ffmpeg_path) = YoutubeResolver::find_ffmpeg_location() {
            if let Some(ffmpeg_dir) = ffmpeg_path.parent() {
                cmd.args(&["--ffmpeg-location", &ffmpeg_dir.to_string_lossy()]);
            }
        }

        #[cfg(windows)]
        {
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| {
            RapidError::Other(format!("Failed to start yt-dlp process: {}", e))
        })?;

        let stdout = child.stdout.take().ok_or_else(|| {
            RapidError::Other("Failed to capture yt-dlp stdout".to_string())
        })?;

        let mut reader = BufReader::new(stdout).lines();
        let mut last_downloaded = 0u64;
        let mut total_bytes_known: Option<u64> = None;

        let mut ui_segments: Vec<Segment> = (0..8)
            .map(|i| Segment::new(i, i as u64, (i + 1) as u64))
            .collect();

        loop {
            tokio::select! {
                _ = self.cancel_token.cancelled() => {
                    let _ = child.kill().await;
                    let _ = progress_tx.send(DownloadProgress {
                        id: self.id.clone(),
                        filename: self.filename.clone(),
                        status: DownloadStatus::Cancelled,
                        downloaded_bytes: last_downloaded,
                        total_bytes: total_bytes_known,
                        progress_percent: 0.0,
                        speed_bps: 0,
                        eta_seconds: None,
                        segments: ui_segments,
                    });
                    return Err(RapidError::Cancelled);
                }
                line_res = reader.next_line() => {
                    match line_res {
                        Ok(Some(line)) => {
                            if line.starts_with("download:") {
                                let payload = &line["download:".len()..];
                                let parts: Vec<&str> = payload.split('/').collect();
                                if parts.len() >= 4 {
                                    let downloaded = parts[0].trim().parse::<u64>().unwrap_or(last_downloaded);
                                    let total = parts[1].trim().parse::<u64>().ok();
                                    let speed = parts[2].trim().parse::<f64>().ok().map(|s| s as u64).unwrap_or(0);
                                    let eta = parts[3].trim().parse::<f64>().ok().map(|e| e as u64);

                                    last_downloaded = downloaded;
                                    if total.is_some() {
                                        total_bytes_known = total;
                                    }

                                    let pct = if let Some(tot) = total_bytes_known {
                                        if tot > 0 {
                                            ((downloaded as f64 / tot as f64) * 100.0) as f32
                                        } else {
                                            0.0
                                        }
                                    } else {
                                        0.0
                                    };

                                    for seg in &mut ui_segments {
                                        seg.downloaded_bytes = downloaded / 8;
                                        seg.is_complete = pct >= 100.0;
                                    }

                                    let _ = progress_tx.send(DownloadProgress {
                                        id: self.id.clone(),
                                        filename: self.filename.clone(),
                                        status: DownloadStatus::Downloading,
                                        downloaded_bytes: downloaded,
                                        total_bytes: total_bytes_known,
                                        progress_percent: pct,
                                        speed_bps: speed,
                                        eta_seconds: eta,
                                        segments: ui_segments.clone(),
                                    });
                                }
                            }
                        }
                        Ok(None) => break, // EOF
                        Err(_) => break,
                    }
                }
            }
        }

        let status = child.wait().await.map_err(|e| {
            RapidError::Other(format!("Failed to wait on yt-dlp: {}", e))
        })?;

        if !status.success() {
            return Err(RapidError::Other(format!(
                "YouTube download failed with exit code: {:?}",
                status.code()
            )));
        }

        let final_size = fs::metadata(&self.target_file).await.ok().map(|m| m.len()).unwrap_or(last_downloaded);
        for seg in &mut ui_segments {
            seg.downloaded_bytes = final_size / 8;
            seg.is_complete = true;
        }

        let _ = progress_tx.send(DownloadProgress {
            id: self.id.clone(),
            filename: self.filename.clone(),
            status: DownloadStatus::Completed,
            downloaded_bytes: final_size,
            total_bytes: Some(final_size),
            progress_percent: 100.0,
            speed_bps: 0,
            eta_seconds: None,
            segments: ui_segments,
        });

        Ok(())
    }
}

fn sanitize_filename(name: &str) -> String {
    let clean: String = name
        .chars()
        .map(|c| {
            if "/\\:*?\"<>|".contains(c) {
                '_'
            } else {
                c
            }
        })
        .collect();

    let trimmed = clean.trim();
    if trimmed.is_empty() {
        "YouTube_Video".to_string()
    } else {
        trimmed.to_string()
    }
}
