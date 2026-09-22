use crate::error::{RapidError, Result};
use crate::types::{DownloadTaskState, Segment};
use std::path::Path;
use tokio::fs;

pub struct SegmentPlanner;

impl SegmentPlanner {
    pub const MIN_CHUNK_SIZE: u64 = 1024 * 512; // 512 KB minimum chunk size

    pub fn plan(total_size: Option<u64>, requested_segments: usize, accept_ranges: bool) -> Vec<Segment> {
        let segments_count = match (total_size, accept_ranges) {
            (Some(size), true) if size > 0 => {
                let max_possible = (size / Self::MIN_CHUNK_SIZE).max(1) as usize;
                requested_segments.clamp(1, max_possible)
            }
            _ => 1, // Single stream fallback if unknown size or range not supported
        };

        if segments_count <= 1 || total_size.is_none() {
            return vec![Segment::new(0, 0, total_size.unwrap_or(0).saturating_sub(1))];
        }

        let total = total_size.unwrap();
        let chunk_size = total / (segments_count as u64);
        let mut segments = Vec::with_capacity(segments_count);

        for i in 0..segments_count {
            let start = (i as u64) * chunk_size;
            let end = if i == segments_count - 1 {
                total - 1
            } else {
                ((i + 1) as u64) * chunk_size - 1
            };
            segments.push(Segment::new(i, start, end));
        }

        segments
    }

    pub async fn save_manifest(state: &DownloadTaskState) -> Result<()> {
        let manifest_path = DownloadTaskState::manifest_path(&state.target_file);
        let json = serde_json::to_string_pretty(state)?;
        fs::write(&manifest_path, json).await?;
        Ok(())
    }

    pub async fn load_manifest(target_file: &Path) -> Result<Option<DownloadTaskState>> {
        let manifest_path = DownloadTaskState::manifest_path(target_file);
        if !manifest_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&manifest_path).await?;
        let state: DownloadTaskState = serde_json::from_str(&content)
            .map_err(|e| RapidError::Other(format!("Failed to parse manifest: {}", e)))?;
        Ok(Some(state))
    }

    pub async fn delete_manifest(target_file: &Path) -> Result<()> {
        let manifest_path = DownloadTaskState::manifest_path(target_file);
        if manifest_path.exists() {
            let _ = fs::remove_file(manifest_path).await;
        }
        Ok(())
    }
}
