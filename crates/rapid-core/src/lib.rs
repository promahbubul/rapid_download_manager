pub mod logger;
pub mod storage;
pub mod paths;
pub mod gdrive;
pub mod hls;
pub mod youtube;
pub mod engine;
pub mod error;
pub mod probe;
pub mod segment;
pub mod types;
pub mod worker;

pub use engine::DownloadTask;
pub use hls::HlsDownloader;
pub use youtube::{YoutubeDownloader, YoutubeResolver, YoutubeMetadata, DownloadQuality};
pub use error::{RapidError, Result};
pub use probe::Probe;
pub use segment::SegmentPlanner;
pub use types::*;
pub use gdrive::*;

pub use paths::AppPaths;
pub use storage::StorageManager;
pub use logger::{RapidLogger, init_production_logging, write_crash_report};
