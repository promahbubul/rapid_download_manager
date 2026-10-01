use std::path::PathBuf;
use std::sync::RwLock;

static CACHED_GOOGLE_COOKIES: RwLock<Option<String>> = RwLock::new(None);

pub fn save_google_cookies(cookies: &str) {
    let clean = cookies.trim();
    if clean.len() > 10 && (clean.contains("SID=") || clean.contains("HSID=") || clean.contains("OSID=") || clean.contains("download_warning_")) {
        if let Ok(mut lock) = CACHED_GOOGLE_COOKIES.write() {
            *lock = Some(clean.to_string());
        }
        let cookie_file = rapid_core::AppPaths::app_data_dir().join("google_cookies.txt");
        let _ = std::fs::write(cookie_file, clean);
    }
}

pub fn load_google_cookies() -> Option<String> {
    if let Ok(lock) = CACHED_GOOGLE_COOKIES.read() {
        if let Some(ref c) = *lock {
            return Some(c.clone());
        }
    }
    let cookie_file = rapid_core::AppPaths::app_data_dir().join("google_cookies.txt");
    if let Ok(content) = std::fs::read_to_string(cookie_file) {
        let trimmed = content.trim().to_string();
        if !trimmed.is_empty() {
            if let Ok(mut lock) = CACHED_GOOGLE_COOKIES.write() {
                *lock = Some(trimmed.clone());
            }
            return Some(trimmed);
        }
    }
    None
}

pub fn launch_chrome_with_extension(target_url: Option<&str>) {
    let chrome_paths = [
        r"C:\Program Files\Google\Chrome\Application\chrome.exe",
        r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        r"C:\Program Files\BraveSoftware\Brave-Browser\Application\brave.exe",
        r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
    ];

    let mut browser_exe = None;
    for p in &chrome_paths {
        if std::path::Path::new(p).exists() {
            browser_exe = Some(p.to_string());
            break;
        }
    }

    if browser_exe.is_none() {
        if let Ok(local_app) = std::env::var("LOCALAPPDATA") {
            let chrome_local = format!(r"{}\Google\Chrome\Application\chrome.exe", local_app);
            if std::path::Path::new(&chrome_local).exists() {
                browser_exe = Some(chrome_local);
            }
        }
    }

    let Some(exe) = browser_exe else {
        if let Some(u) = target_url {
            let _ = open::that(u);
        }
        return;
    };

    let mut ext_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.join("extension")))
        .unwrap_or_else(|| PathBuf::from("extension"));

    if !ext_dir.exists() {
        let alt = PathBuf::from(r"D:\mahbub\project\rapid_download_manager\extension");
        if alt.exists() {
            ext_dir = alt;
        }
    }

    let ext_arg = format!("--load-extension={}", ext_dir.to_string_lossy());
    let mut cmd = std::process::Command::new(exe);
    cmd.arg(ext_arg);
    if let Some(u) = target_url {
        cmd.arg(u);
    }
    let _ = cmd.spawn();
}
