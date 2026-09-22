use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DownloadStatus {
    Queued,
    Probing,
    Downloading,
    Paused,
    Completed,
    Failed(String),
    Cancelled,
}

impl std::fmt::Display for DownloadStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DownloadStatus::Queued => write!(f, "Queued"),
            DownloadStatus::Probing => write!(f, "Probing..."),
            DownloadStatus::Downloading => write!(f, "Downloading"),
            DownloadStatus::Paused => write!(f, "Paused"),
            DownloadStatus::Completed => write!(f, "Completed"),
            DownloadStatus::Failed(err) => write!(f, "Failed: {}", err),
            DownloadStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    pub index: usize,
    pub start_byte: u64,
    pub end_byte: u64,
    pub downloaded_bytes: u64,
    pub is_complete: bool,
}

impl Segment {
    pub fn new(index: usize, start_byte: u64, end_byte: u64) -> Self {
        Self {
            index,
            start_byte,
            end_byte,
            downloaded_bytes: 0,
            is_complete: false,
        }
    }

    pub fn total_bytes(&self) -> u64 {
        if self.end_byte >= self.start_byte {
            (self.end_byte - self.start_byte) + 1
        } else {
            0
        }
    }

    pub fn progress_ratio(&self) -> f32 {
        let total = self.total_bytes();
        if total == 0 {
            0.0
        } else {
            (self.downloaded_bytes as f32 / total as f32).min(1.0)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadMetadata {
    pub url: String,
    pub filename: String,
    pub content_length: Option<u64>,
    pub accept_ranges: bool,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub id: String,
    pub filename: String,
    pub total_bytes: Option<u64>,
    pub downloaded_bytes: u64,
    pub progress_percent: f32,
    pub speed_bps: u64,
    pub eta_seconds: Option<u64>,
    pub status: DownloadStatus,
    pub segments: Vec<Segment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTaskState {
    pub id: String,
    pub url: String,
    pub target_file: PathBuf,
    pub total_bytes: Option<u64>,
    pub accept_ranges: bool,
    pub segments: Vec<Segment>,
    pub status: DownloadStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DownloadTaskState {
    pub fn manifest_path(target_file: &std::path::Path) -> PathBuf {
        let mut path = target_file.to_path_buf();
        let filename = path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "download".to_string());
        path.set_file_name(format!("{}.rapid", filename));
        path
    }

    pub fn total_downloaded(&self) -> u64 {
        self.segments.iter().map(|s| s.downloaded_bytes).sum()
    }
}

#[derive(Debug, Clone)]
pub struct DownloadConfig {
    pub url: String,
    pub output_dir: PathBuf,
    pub custom_filename: Option<String>,
    pub num_segments: usize,
    pub user_agent: Option<String>,
    pub referrer: Option<String>,
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            output_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            custom_filename: None,
            num_segments: 8,
            user_agent: Some("RapidDownloadManager/1.0 (Windows NT 10.0; Win64; x64)".to_string()),
            referrer: None,
        }
    }
}
