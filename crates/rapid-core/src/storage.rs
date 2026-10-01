use std::path::Path;
use serde::{Serialize, de::DeserializeOwned};

/// Enterprise-grade resilient local storage engine with atomic writes,
/// schema safety, automatic dual-layer backup, and crash recovery.
pub struct StorageManager;

impl StorageManager {
    /// Atomically writes data to disk using a temporary file and atomic rename.
    /// Before overwriting, automatically maintains a `.bak` backup copy.
    pub fn atomic_write_json<T: Serialize>(path: &Path, data: &T) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        // 1. Maintain backup of current valid state
        if path.exists() {
            let bak_path = path.with_extension("json.bak");
            let _ = std::fs::copy(path, &bak_path);
        }

        // 2. Write to unique temporary file
        let tmp_path = path.with_extension(format!("tmp.{}", std::process::id()));
        let json = serde_json::to_string_pretty(data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        {
            use std::io::Write;
            let mut file = std::fs::File::create(&tmp_path)?;
            file.write_all(json.as_bytes())?;
            file.sync_all()?;
        }

        // 3. Atomically replace target file
        #[cfg(windows)]
        {
            if path.exists() {
                let _ = std::fs::remove_file(path);
            }
            std::fs::rename(&tmp_path, path)?;
        }

        #[cfg(not(windows))]
        {
            std::fs::rename(&tmp_path, path)?;
        }

        Ok(())
    }

    /// Resilient JSON loader that automatically recovers from `.bak` backup
    /// if the primary file is corrupted or truncated.
    pub fn load_json_with_backup<T: DeserializeOwned>(path: &Path) -> Option<T> {
        let bak_path = path.with_extension("json.bak");

        // Try primary path
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok(val) = serde_json::from_str::<T>(&content) {
                    return Some(val);
                }
            }
        }

        // Fallback: recover from backup
        if bak_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&bak_path) {
                if let Ok(val) = serde_json::from_str::<T>(&content) {
                    // Restore corrupted primary from backup
                    let _ = std::fs::copy(&bak_path, path);
                    return Some(val);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct SampleData {
        name: String,
        score: u32,
    }

    #[test]
    fn test_atomic_write_and_read() {
        let temp_dir = std::env::temp_dir().join(format!("rapid_test_storage_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&temp_dir);
        let file_path = temp_dir.join("test_data.json");

        let original = SampleData {
            name: "Rapid Test".to_string(),
            score: 100,
        };

        let write_res = StorageManager::atomic_write_json(&file_path, &original);
        assert!(write_res.is_ok(), "Failed to write: {:?}", write_res);

        let loaded: Option<SampleData> = StorageManager::load_json_with_backup(&file_path);
        assert_eq!(loaded, Some(original));

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_recovery_from_backup_on_corruption() {
        let temp_dir = std::env::temp_dir().join(format!("rapid_test_backup_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&temp_dir);
        let file_path = temp_dir.join("corrupt_test.json");

        let valid_data = SampleData {
            name: "Persistent Config".to_string(),
            score: 42,
        };

        // Write first version
        assert!(StorageManager::atomic_write_json(&file_path, &valid_data).is_ok());

        // Update it so .bak is created
        let updated_data = SampleData {
            name: "Updated Config".to_string(),
            score: 99,
        };
        assert!(StorageManager::atomic_write_json(&file_path, &updated_data).is_ok());

        // Now artificially corrupt the primary file with broken garbage
        std::fs::write(&file_path, b"{ broken json !@@@").unwrap();

        // Loading should automatically recover from .bak!
        let recovered: Option<SampleData> = StorageManager::load_json_with_backup(&file_path);
        assert_eq!(recovered, Some(valid_data));

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}

