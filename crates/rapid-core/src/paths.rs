use std::path::PathBuf;

/// Enterprise-grade Windows Application Data & Directory Structure
pub struct AppPaths;

impl AppPaths {
    /// %APPDATA%\RapidDownloadManager (Roaming: user configs, scheduler, history)
    pub fn data_dir() -> PathBuf {
        Self::app_data_dir()
    }

    pub fn app_data_dir() -> PathBuf {
        if let Ok(roaming) = std::env::var("APPDATA") {
            let p = PathBuf::from(roaming).join("RapidDownloadManager");
            let _ = std::fs::create_dir_all(&p);
            return p;
        }
        if let Ok(user_profile) = std::env::var("USERPROFILE") {
            let p = PathBuf::from(user_profile).join(".rapid_download_manager");
            let _ = std::fs::create_dir_all(&p);
            return p;
        }
        PathBuf::from(".")
    }

    /// %LOCALAPPDATA%\RapidDownloadManager (Local: logs, cache, binaries)
    pub fn local_data_dir() -> PathBuf {
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            let p = PathBuf::from(local).join("RapidDownloadManager");
            let _ = std::fs::create_dir_all(&p);
            return p;
        }
        Self::app_data_dir()
    }

    /// %LOCALAPPDATA%\RapidDownloadManager\logs
    pub fn logs_dir() -> PathBuf {
        let p = Self::local_data_dir().join("logs");
        let _ = std::fs::create_dir_all(&p);
        p
    }

    /// %LOCALAPPDATA%\RapidDownloadManager\cache
    pub fn cache_dir() -> PathBuf {
        let p = Self::local_data_dir().join("cache");
        let _ = std::fs::create_dir_all(&p);
        p
    }

    /// %LOCALAPPDATA%\RapidDownloadManager\bin (Optional helper binaries like yt-dlp, ffmpeg)
    pub fn bin_dir() -> PathBuf {
        let p = Self::local_data_dir().join("bin");
        let _ = std::fs::create_dir_all(&p);
        p
    }

    /// %APPDATA%\RapidDownloadManager\config.json
    pub fn config_file() -> PathBuf {
        Self::app_data_dir().join("config.json")
    }

    /// %APPDATA%\RapidDownloadManager\scheduler.json
    pub fn scheduler_file() -> PathBuf {
        Self::app_data_dir().join("scheduler.json")
    }

    /// %APPDATA%\RapidDownloadManager\history.json
    pub fn history_file() -> PathBuf {
        Self::app_data_dir().join("history.json")
    }

    /// %LOCALAPPDATA%\RapidDownloadManager\logs\crash.log
    pub fn crash_log_file() -> PathBuf {
        Self::logs_dir().join("crash.log")
    }

    /// Standard user personal Downloads directory (%USERPROFILE%\Downloads)
    pub fn default_downloads_dir() -> PathBuf {
        if let Ok(user_profile) = std::env::var("USERPROFILE") {
            let p = PathBuf::from(user_profile).join("Downloads");
            if p.exists() {
                return p;
            }
        }
        PathBuf::from("./downloads")
    }
}
