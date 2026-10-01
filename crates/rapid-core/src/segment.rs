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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_planner_fallback_when_ranges_disabled() {
        let segs = SegmentPlanner::plan(Some(100_000_000), 8, false);
        assert_eq!(segs.len(), 1);
        assert_eq!(segs[0].start_byte, 0);
        assert_eq!(segs[0].end_byte, 99_999_999);
    }

    #[test]
    fn test_segment_planner_fallback_when_size_none() {
        let segs = SegmentPlanner::plan(None, 8, true);
        assert_eq!(segs.len(), 1);
        assert_eq!(segs[0].start_byte, 0);
    }

    #[test]
    fn test_segment_planner_small_file_clamping() {
        // Less than MIN_CHUNK_SIZE (512 KB) should produce only 1 segment
        let segs = SegmentPlanner::plan(Some(200_000), 8, true);
        assert_eq!(segs.len(), 1);
        assert_eq!(segs[0].end_byte, 199_999);
    }

    #[test]
    fn test_segment_planner_exact_byte_coverage() {
        let total = 10_000_000u64;
        let num_segs = 8;
        let segs = SegmentPlanner::plan(Some(total), num_segs, true);
        assert_eq!(segs.len(), num_segs);

        // Verify continuity with 0 gaps or overlaps
        assert_eq!(segs[0].start_byte, 0);
        for i in 0..segs.len() - 1 {
            assert_eq!(segs[i].end_byte + 1, segs[i + 1].start_byte);
        }
        assert_eq!(segs.last().unwrap().end_byte, total - 1);
    }

    #[tokio::test]
    async fn test_manifest_lifecycle() {
        let temp_dir = std::env::temp_dir().join(format!("rapid_test_manifest_{}", std::process::id()));
        let _ = tokio::fs::create_dir_all(&temp_dir).await;
        let target_file = temp_dir.join("ubuntu.iso");

        let state = DownloadTaskState {
            id: "task_test_123".to_string(),
            url: "https://example.com/ubuntu.iso".to_string(),
            target_file: target_file.clone(),
            total_bytes: Some(1024),
            accept_ranges: true,
            segments: vec![Segment::new(0, 0, 1023)],
            status: crate::types::DownloadStatus::Paused,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            cookies: None,
            referrer: None,
            user_agent: None,
        };

        // Save
        assert!(SegmentPlanner::save_manifest(&state).await.is_ok());

        // Load
        let loaded = SegmentPlanner::load_manifest(&target_file).await.unwrap();
        assert!(loaded.is_some());
        let loaded_state = loaded.unwrap();
        assert_eq!(loaded_state.id, "task_test_123");
        assert_eq!(loaded_state.total_bytes, Some(1024));

        // Delete
        assert!(SegmentPlanner::delete_manifest(&target_file).await.is_ok());
        let deleted = SegmentPlanner::load_manifest(&target_file).await.unwrap();
        assert!(deleted.is_none());

        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }
}

