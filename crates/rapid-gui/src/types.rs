use rapid_core::{DownloadStatus, DownloadTask};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterCategory {
    All,
    Active,
    Paused,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerConfig {
    pub enabled: bool,
    pub start_hour: u32,
    pub start_minute: u32,
    pub stop_hour: u32,
    pub stop_minute: u32,
    pub auto_shutdown: bool,
    #[serde(skip)]
    pub has_triggered_start: bool,
    #[serde(skip)]
    pub has_triggered_stop: bool,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            start_hour: 23,
            start_minute: 0,
            stop_hour: 6,
            stop_minute: 0,
            auto_shutdown: false,
            has_triggered_start: false,
            has_triggered_stop: false,
        }
    }
}

pub fn dirs_or_fallback() -> PathBuf {
    rapid_core::AppPaths::default_downloads_dir()
}

pub fn get_scheduler_file_path() -> PathBuf {
    let appdata_file = rapid_core::AppPaths::scheduler_file();
    let old_file = dirs_or_fallback().join(".rapid_scheduler.json");
    if old_file.exists() && !appdata_file.exists() {
        let _ = std::fs::copy(&old_file, &appdata_file);
        let _ = std::fs::remove_file(&old_file);
    }
    appdata_file
}

pub fn load_scheduler_config() -> SchedulerConfig {
    let path = get_scheduler_file_path();
    rapid_core::StorageManager::load_json_with_backup::<SchedulerConfig>(&path).unwrap_or_default()
}

pub fn save_scheduler_config(config: &SchedulerConfig) {
    let path = get_scheduler_file_path();
    let _ = rapid_core::StorageManager::atomic_write_json(&path, config);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryRecord {
    pub id: String,
    pub filename: String,
    pub url: String,
    pub target_file: PathBuf,
    #[serde(default)]
    pub total_bytes: Option<u64>,
    #[serde(default = "chrono::Utc::now")]
    pub completed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub category: Option<String>,
}

pub fn get_history_file_path() -> PathBuf {
    let appdata_file = rapid_core::AppPaths::history_file();
    let old_file = dirs_or_fallback().join(".rapid_history.json");
    if old_file.exists() && !appdata_file.exists() {
        let _ = std::fs::copy(&old_file, &appdata_file);
        let _ = std::fs::remove_file(&old_file);
    }
    appdata_file
}

pub fn load_history() -> Vec<HistoryRecord> {
    let path = get_history_file_path();
    rapid_core::StorageManager::load_json_with_backup::<Vec<HistoryRecord>>(&path).unwrap_or_default()
}

pub fn save_task_to_history(item: &ActiveTaskUI) {
    let path = get_history_file_path();
    let mut records = load_history();
    if let Some(existing) = records.iter_mut().find(|r| r.id == item.id || r.target_file == item.target_file) {
        existing.total_bytes = item.total_bytes;
        existing.completed_at = chrono::Utc::now();
    } else {
        records.push(HistoryRecord {
            id: item.id.clone(),
            filename: item.filename.clone(),
            url: item.url.clone(),
            target_file: item.target_file.clone(),
            total_bytes: item.total_bytes,
            completed_at: chrono::Utc::now(),
            category: None,
        });
    }
    let _ = rapid_core::StorageManager::atomic_write_json(&path, &records);
}

pub fn remove_from_history(id: &str) {
    let path = get_history_file_path();
    let mut records = load_history();
    records.retain(|r| r.id != id);
    let _ = rapid_core::StorageManager::atomic_write_json(&path, &records);
}

pub fn clear_all_history() {
    let path = get_history_file_path();
    let _ = std::fs::remove_file(&path);
}

pub struct ActiveTaskUI {
    pub id: String,
    pub filename: String,
    pub url: String,
    pub target_file: PathBuf,
    pub total_bytes: Option<u64>,
    pub downloaded_bytes: u64,
    pub progress_percent: f32,
    pub speed_bps: u64,
    pub eta_seconds: Option<u64>,
    pub status: DownloadStatus,
    pub segments: Vec<rapid_core::Segment>,
    pub task_handle: Option<Arc<DownloadTask>>,
    pub num_segments: usize,
    pub is_resuming: bool,
    pub cookies: Option<String>,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
    pub is_youtube: bool,
    pub quality: Option<rapid_core::youtube::DownloadQuality>,
    pub yt_cancel_token: Option<tokio_util::sync::CancellationToken>,
}

#[derive(Clone, Debug)]
pub struct PendingBrowserDownload {
    pub url: String,
    pub filename: String,
    pub dest_dir: String,
    pub segments: usize,
    pub cookies: Option<String>,
    pub user_agent: Option<String>,
    pub referrer: Option<String>,
    pub is_gdrive: bool,
    pub is_youtube: bool,
    pub quality: rapid_core::youtube::DownloadQuality,
}

pub fn extract_filename_from_url(url_str: &str) -> String {
    let trimmed = url_str.trim();
    if trimmed.is_empty() || rapid_core::youtube::YoutubeResolver::is_extractable_platform(trimmed) {
        return String::new();
    }
    let base = trimmed.split('?').next().unwrap_or(trimmed);
    let base = base.split('#').next().unwrap_or(base);
    let trimmed_base = base.trim_end_matches('/');
    if let Some(pos) = trimmed_base.rfind('/') {
        let segment = &trimmed_base[pos + 1..];
        if !segment.is_empty() {
            if let Ok(decoded) = urlencoding::decode(segment) {
                let clean = decoded.trim();
                if !clean.is_empty() && clean != "/" {
                    return clean.to_string();
                }
            }
            return segment.to_string();
        }
    }
    String::new()
}

pub fn categorize_filename(filename: &str) -> &'static str {
    let ext = std::path::Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "exe" | "msi" | "apk" | "bat" | "cmd" | "app" | "dmg" | "deb" | "rpm" => "Programs",
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" | "iso" | "img" | "cab" => "Compressed",
        "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" | "3gp" | "ts" => "Videos",
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" | "wma" | "mid" | "opus" => "Music",
        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "rtf" | "csv" | "epub" => "Documents",
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg" | "bmp" | "ico" | "tiff" => "Images",
        _ => "",
    }
}

pub fn get_categorized_destination(base_dir: &std::path::Path, filename: &str) -> PathBuf {
    let cat = categorize_filename(filename);
    if cat.is_empty() {
        base_dir.to_path_buf()
    } else {
        if base_dir.file_name().and_then(|f| f.to_str()) == Some(cat) {
            base_dir.to_path_buf()
        } else {
            base_dir.join(cat)
        }
    }
}

#[derive(Clone, Debug)]
pub enum UpdateStatus {
    Idle,
    Checking,
    UpToDate { checked_at: String },
    Available {
        version: String,
        download_url: String,
        release_notes: String,
        published_at: String,
    },
    Error(String),
}

pub fn parse_ver_triplet(v: &str) -> (u32, u32, u32) {
    let clean = v.trim_start_matches('v').split('-').next().unwrap_or("");
    let mut parts = clean.split('.').filter_map(|s| s.parse::<u32>().ok());
    (parts.next().unwrap_or(0), parts.next().unwrap_or(0), parts.next().unwrap_or(0))
}

pub fn is_remote_version_newer(remote: &str, current: &str) -> bool {
    parse_ver_triplet(remote) > parse_ver_triplet(current)
}

pub fn format_bytes(bytes: u64) -> String {
    let kb = bytes as f64 / 1024.0;
    if kb < 1024.0 {
        format!("{:.1} KB", kb)
    } else {
        let mb = kb / 1024.0;
        if mb < 1024.0 {
            format!("{:.1} MB", mb)
        } else {
            format!("{:.2} GB", mb / 1024.0)
        }
    }
}

pub fn format_speed(bps: u64) -> String {
    if bps == 0 {
        return "--".to_string();
    }
    let kb = bps as f64 / 1024.0;
    if kb < 1024.0 {
        format!("{:.1} KB/s", kb)
    } else {
        format!("{:.2} MB/s", kb / 1024.0)
    }
}

pub fn format_eta(seconds: u64) -> String {
    if seconds == 0 {
        return "--".to_string();
    }
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        let mins = seconds / 60;
        let secs = seconds % 60;
        format!("{}m {}s", mins, secs)
    } else {
        let hours = seconds / 3600;
        let mins = (seconds % 3600) / 60;
        format!("{}h {}m", hours, mins)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorize_filename() {
        assert_eq!(categorize_filename("setup.exe"), "Programs");
        assert_eq!(categorize_filename("archive.tar.gz"), "Compressed");
        assert_eq!(categorize_filename("movie.mp4"), "Videos");
        assert_eq!(categorize_filename("song.mp3"), "Music");
        assert_eq!(categorize_filename("document.pdf"), "Documents");
        assert_eq!(categorize_filename("photo.png"), "Images");
        assert_eq!(categorize_filename("unknown.xyz"), "");
    }

    #[test]
    fn test_extract_filename_from_url() {
        assert_eq!(
            extract_filename_from_url("https://example.com/downloads/Rust_Setup.exe?v=1.2#top"),
            "Rust_Setup.exe"
        );
        assert_eq!(
            extract_filename_from_url("https://example.com/files/My%20Lecture%2001.mp4"),
            "My Lecture 01.mp4"
        );
    }

    #[test]
    fn test_version_comparison() {
        assert!(is_remote_version_newer("v1.0.5", "v1.0.4"));
        assert!(is_remote_version_newer("2.0.0", "1.9.9"));
        assert!(!is_remote_version_newer("v1.0.4", "v1.0.4"));
        assert!(!is_remote_version_newer("1.0.3", "1.0.4"));
    }

    #[test]
    fn test_formatting_helpers() {
        assert_eq!(format_bytes(512), "0.5 KB");
        assert_eq!(format_bytes(5 * 1024 * 1024), "5.0 MB");
        assert_eq!(format_bytes(2 * 1024 * 1024 * 1024), "2.00 GB");

        assert_eq!(format_speed(0), "--");
        assert_eq!(format_speed(500 * 1024), "500.0 KB/s");
        assert_eq!(format_speed(10 * 1024 * 1024), "10.00 MB/s");

        assert_eq!(format_eta(0), "--");
        assert_eq!(format_eta(45), "45s");
        assert_eq!(format_eta(150), "2m 30s");
        assert_eq!(format_eta(3665), "1h 1m");
    }
}

