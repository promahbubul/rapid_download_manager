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

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DownloadQuality {
    Best,
    P1080,
    P720,
    P480,
    P360,
    AudioMp3,
    AudioM4a,
}

impl Default for DownloadQuality {
    fn default() -> Self {
        Self::Best
    }
}

impl DownloadQuality {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Best => "Best Available (1080p+ • MP4)",
            Self::P1080 => "1080p Full HD (MP4)",
            Self::P720 => "720p HD (MP4)",
            Self::P480 => "480p SD (MP4)",
            Self::P360 => "360p (MP4)",
            Self::AudioMp3 => "Audio Only (MP3 • High Quality)",
            Self::AudioM4a => "Audio Only (M4A / AAC)",
        }
    }

    pub fn is_audio(&self) -> bool {
        matches!(self, Self::AudioMp3 | Self::AudioM4a)
    }

    pub fn target_extension(&self) -> &'static str {
        match self {
            Self::AudioMp3 => "mp3",
            Self::AudioM4a => "m4a",
            _ => "mp4",
        }
    }
}

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

    /// Detect if a URL corresponds to any platform extractable via yt-dlp
    pub fn is_extractable_platform(url_or_ref: &str) -> bool {
        let lower = url_or_ref.to_lowercase();
        lower.contains("youtube.com")
            || lower.contains("youtu.be")
            || lower.contains("googlevideo.com")
            || lower.contains("linkedin.com")
            || lower.contains("facebook.com")
            || lower.contains("fb.watch")
            || lower.contains("instagram.com")
            || lower.contains("twitter.com")
            || lower.contains("x.com")
            || lower.contains("tiktok.com")
            || lower.contains("reddit.com")
            || lower.contains("vimeo.com")
            || lower.contains("dailymotion.com")
            || lower.contains("twitch.tv")
            || lower.contains("pinterest.com")
    }

    /// Extract or canonicalize a YouTube page URL (including Shorts & youtu.be)
    pub fn canonicalize_url(url: &str, referrer: Option<&str>) -> String {
        let is_specific_video = |u: &str| {
            u.contains("/watch")
                || u.contains("/shorts/")
                || u.contains("youtu.be/")
                || u.contains("/live/")
                || u.contains("/embed/")
        };

        let target = if is_specific_video(url) {
            url
        } else if let Some(r) = referrer {
            if is_specific_video(r) {
                r
            } else {
                url
            }
        } else {
            url
        };

        if target.contains("youtu.be/") {
            if let Some(id) = target.split("youtu.be/").nth(1) {
                let clean_id = id.split("?").next().unwrap_or(id).split("&").next().unwrap_or(id).trim_matches('/');
                if !clean_id.is_empty() {
                    return format!("https://www.youtube.com/watch?v={}", clean_id);
                }
            }
        }
        if target.contains("/shorts/") {
            if let Some(id) = target.split("/shorts/").nth(1) {
                let clean_id = id.split("?").next().unwrap_or(id).split("&").next().unwrap_or(id).trim_matches('/');
                if !clean_id.is_empty() {
                    return format!("https://www.youtube.com/watch?v={}", clean_id);
                }
            }
        }
        target.to_string()
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
        let proj_candidate = PathBuf::from("D:\\mahbub\\project\\rapid_download_manager\\yt-dlp.exe");
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
        let proj_candidate = PathBuf::from("D:\\mahbub\\project\\rapid_download_manager\\ffmpeg.exe");
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
    pub quality: DownloadQuality,
    pub speed_limit: Option<Arc<AtomicU64>>,
    pub cancel_token: CancellationToken,
    pub cookies: Option<String>,
    pub user_agent: Option<String>,
    pub referrer: Option<String>,
}

impl YoutubeDownloader {
    pub fn new(
        id: String,
        url: String,
        filename: String,
        target_file: PathBuf,
        quality: DownloadQuality,
        speed_limit: Option<Arc<AtomicU64>>,
        cancel_token: CancellationToken,
        cookies: Option<String>,
        user_agent: Option<String>,
        referrer: Option<String>,
    ) -> Self {
        Self {
            id,
            url,
            filename,
            target_file,
            quality,
            speed_limit,
            cancel_token,
            cookies,
            user_agent,
            referrer,
        }
    }

    pub fn is_youtube(url: &str) -> bool {
        YoutubeResolver::is_youtube(url)
    }

    pub fn is_extractable_platform(url: &str) -> bool {
        YoutubeResolver::is_extractable_platform(url)
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
            "download:RAPIDPROG:%(progress.downloaded_bytes)s/%(progress.total_bytes,progress.total_bytes_estimate)s/%(progress.speed)s/%(progress.eta)s",
            "--no-playlist",
        ]);
        if let Some(ref c) = self.cookies {
            let trimmed = c.trim();
            if !trimmed.is_empty() {
                cmd.args(&["--add-header", &format!("Cookie: {}", trimmed)]);
            }
        }
        if let Some(ref ua) = self.user_agent {
            let trimmed = ua.trim();
            if !trimmed.is_empty() {
                cmd.args(&["--user-agent", trimmed]);
            }
        }
        if let Some(ref r) = self.referrer {
            let trimmed = r.trim();
            if !trimmed.is_empty() {
                cmd.args(&["--referer", trimmed]);
            }
        }

        match self.quality {
            DownloadQuality::Best => {
                cmd.args(&[
                    "-f",
                    "bv*[ext=mp4]+ba[ext=m4a]/b[ext=mp4]/bv*+ba/b/best",
                    "--merge-output-format",
                    "mp4",
                ]);
            }
            DownloadQuality::P1080 => {
                cmd.args(&[
                    "-f",
                    "bv*[height<=1080][ext=mp4]+ba[ext=m4a]/bv*[height<=1080]+ba/b[height<=1080][ext=mp4]/b[height<=1080]/best",
                    "--merge-output-format",
                    "mp4",
                ]);
            }
            DownloadQuality::P720 => {
                cmd.args(&[
                    "-f",
                    "bv*[height<=720][ext=mp4]+ba[ext=m4a]/bv*[height<=720]+ba/b[height<=720][ext=mp4]/b[height<=720]/best",
                    "--merge-output-format",
                    "mp4",
                ]);
            }
            DownloadQuality::P480 => {
                cmd.args(&[
                    "-f",
                    "bv*[height<=480][ext=mp4]+ba[ext=m4a]/bv*[height<=480]+ba/b[height<=480][ext=mp4]/b[height<=480]/best",
                    "--merge-output-format",
                    "mp4",
                ]);
            }
            DownloadQuality::P360 => {
                cmd.args(&[
                    "-f",
                    "bv*[height<=360][ext=mp4]+ba[ext=m4a]/bv*[height<=360]+ba/b[height<=360][ext=mp4]/b[height<=360]/best",
                    "--merge-output-format",
                    "mp4",
                ]);
            }
            DownloadQuality::AudioMp3 => {
                cmd.args(&[
                    "-x",
                    "--audio-format",
                    "mp3",
                    "--audio-quality",
                    "0",
                ]);
            }
            DownloadQuality::AudioM4a => {
                cmd.args(&[
                    "-f",
                    "ba[ext=m4a]/ba",
                    "-x",
                    "--audio-format",
                    "m4a",
                ]);
            }
        }

        cmd.args(&[
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

        let stderr = child.stderr.take().ok_or_else(|| {
            RapidError::Other("Failed to capture yt-dlp stderr".to_string())
        })?;

        let err_buffer = Arc::new(tokio::sync::Mutex::new(String::new()));
        let err_buffer_clone = Arc::clone(&err_buffer);

        tokio::spawn(async move {
            let mut err_reader = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = err_reader.next_line().await {
                let mut buf = err_buffer_clone.lock().await;
                if buf.len() < 4096 {
                    buf.push_str(&line);
                    buf.push('\n');
                }
            }
        });
        let mut reader = BufReader::new(stdout).lines();
        let mut last_downloaded = 0u64;
        let mut total_bytes_known: Option<u64> = None;
        let mut prev_completed_bytes = 0u64;
        let mut prev_completed_total = 0u64;
        let mut cur_stream_downloaded = 0u64;
        let mut cur_stream_total = 0u64;

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
                            let trimmed_line = line.trim();
                            if let Some(idx) = trimmed_line.find("RAPIDPROG:") {
                                let payload = &trimmed_line[idx + "RAPIDPROG:".len()..];
                                let parts: Vec<&str> = payload.split('/').collect();
                                if parts.len() >= 4 {
                                    let stream_dl = parts[0].trim().parse::<u64>().unwrap_or(cur_stream_downloaded);
                                    let stream_tot = parts[1].trim().parse::<u64>().ok().unwrap_or(0);
                                    let speed = parts[2].trim().parse::<f64>().ok().map(|s| s as u64).unwrap_or(0);
                                    let eta = parts[3].trim().parse::<f64>().ok().map(|e| e as u64);

                                    // Detect transition between streams (e.g. video completed, audio stream started)
                                    if (stream_dl < cur_stream_downloaded && cur_stream_downloaded > 0)
                                        || (stream_tot > 0 && cur_stream_total > 0 && stream_tot != cur_stream_total && stream_dl < cur_stream_downloaded) {
                                        prev_completed_bytes += cur_stream_total.max(cur_stream_downloaded);
                                        prev_completed_total += cur_stream_total;
                                        cur_stream_downloaded = stream_dl;
                                        cur_stream_total = stream_tot;
                                    } else {
                                        cur_stream_downloaded = stream_dl;
                                        if stream_tot > 0 {
                                            cur_stream_total = stream_tot;
                                        }
                                    }

                                    let total_downloaded = prev_completed_bytes + cur_stream_downloaded;
                                    last_downloaded = total_downloaded;

                                    if prev_completed_total + cur_stream_total > 0 {
                                        total_bytes_known = Some(prev_completed_total + cur_stream_total);
                                    }

                                    let pct = if let Some(tot) = total_bytes_known {
                                        if tot > 0 {
                                            ((total_downloaded as f64 / tot as f64) * 100.0).clamp(0.0, 99.5) as f32
                                        } else {
                                            0.0
                                        }
                                    } else {
                                        0.0
                                    };

                                    for seg in &mut ui_segments {
                                        seg.downloaded_bytes = total_downloaded / 8;
                                        seg.is_complete = pct >= 100.0;
                                    }

                                    let _ = progress_tx.send(DownloadProgress {
                                        id: self.id.clone(),
                                        filename: self.filename.clone(),
                                        status: DownloadStatus::Downloading,
                                        downloaded_bytes: total_downloaded,
                                        total_bytes: total_bytes_known,
                                        progress_percent: pct,
                                        speed_bps: speed,
                                        eta_seconds: eta,
                                        segments: ui_segments.clone(),
                                    });
                                }
                            } else if trimmed_line.contains("[Merger]") || trimmed_line.contains("Merging formats") {
                                let _ = progress_tx.send(DownloadProgress {
                                    id: self.id.clone(),
                                    filename: self.filename.clone(),
                                    status: DownloadStatus::Downloading,
                                    downloaded_bytes: last_downloaded,
                                    total_bytes: total_bytes_known,
                                    progress_percent: 99.8,
                                    speed_bps: 0,
                                    eta_seconds: Some(1),
                                    segments: ui_segments.clone(),
                                });
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
            let raw_err = err_buffer.lock().await.clone();
            let err_msg = if !raw_err.trim().is_empty() {
                raw_err.trim().to_string()
            } else {
                format!("Video download failed with exit code: {:?}", status.code())
            };
            return Err(RapidError::Other(err_msg));
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



#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_youtube_progress_event() {
        let (tx, mut rx) = broadcast::channel(64);
        let cancel = CancellationToken::new();
        let target = std::env::temp_dir().join("rapid_test_yt_prog.mp4");
        let dl = YoutubeDownloader::new(
            "test_yt_id".to_string(),
            "https://www.youtube.com/watch?v=jNQXAC9IVRw".to_string(),
            "rapid_test_yt_prog.mp4".to_string(),
            target.clone(),
            DownloadQuality::Best,
            None,
            cancel,
            None,
            None,
            None,
        );

        let handle = tokio::spawn(async move {
            dl.run(tx).await
        });

        let mut progress_events = Vec::new();
        while let Ok(prog) = rx.recv().await {
            println!(
                "PROGRESS: pct={:.1}%, dl={} bytes, tot={:?}, speed={} B/s, eta={:?}s, status={:?}",
                prog.progress_percent, prog.downloaded_bytes, prog.total_bytes, prog.speed_bps, prog.eta_seconds, prog.status
            );
            progress_events.push(prog.clone());
            if prog.status == DownloadStatus::Completed {
                break;
            }
        }

        let res = handle.await.expect("Task join failed");
        assert!(res.is_ok(), "Download task failed: {:?}", res);
        assert!(!progress_events.is_empty(), "No progress events received!");
        assert!(progress_events.len() >= 2, "Expected multiple progress events, got {}", progress_events.len());
        let _ = tokio::fs::remove_file(target).await;
        println!("TEST PASSED: Received {} progress events successfully!", progress_events.len());
    }
}
