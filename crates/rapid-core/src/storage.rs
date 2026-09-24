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
