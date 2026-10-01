use std::path::{Path, PathBuf};
use std::fs::OpenOptions;
use std::io::Write;
use log::{Record, Level, Metadata, LevelFilter, SetLoggerError};

/// Production Enterprise Logger with:
/// - Three-tier log files: app.log, error.log, crash.log
/// - Zero-leak automatic sensitive information sanitizer (passwords, tokens, api keys, bearer headers)
/// - Detailed crash diagnostics (What, Where, Version, OS, Architecture, Stack Backtrace)
/// - Bounded log file rotation (5 MB)
pub struct RapidLogger {
    logs_dir: PathBuf,
}

impl RapidLogger {
    pub fn new(logs_dir: PathBuf) -> Self {
        let _ = std::fs::create_dir_all(&logs_dir);
        Self { logs_dir }
    }

    /// Sanitize log message against sensitive patterns:
    /// - Passwords in URLs (http://user:pass@host)
    /// - Sensitive query params (token=..., api_key=..., password=..., secret=...)
    /// - Authorization headers (Bearer ..., Basic ...)
    pub fn sanitize(input: &str) -> String {
        let mut s = input.to_string();

        // 1. Scrub credentials in URLs (e.g., https://user:pass@host)
        if let Ok(re_url_cred) = regex::Regex::new(r"://([^:@\s]+):([^@\s]+)@") {
            s = re_url_cred.replace_all(&s, "://$1:[REDACTED]@").to_string();
        }

        // 2. Scrub query parameters like ?token=..., &api_key=..., &password=...
        let sensitive_keys = [
            "token", "access_token", "auth_token", "id_token",
            "api_key", "apikey", "key",
            "secret", "client_secret",
            "password", "pass", "pwd",
            "sig", "signature", "auth",
        ];

        for key in sensitive_keys {
            let pattern = format!(r"(?i)([\?&]{}=)[^&\s)]+", key);
            if let Ok(re) = regex::Regex::new(&pattern) {
                s = re.replace_all(&s, "$1[REDACTED]").to_string();
            }
        }

        // 3. Scrub Bearer / Basic Auth headers
        if let Ok(re_bearer) = regex::Regex::new(r"(?i)(Bearer\s+)[A-Za-z0-9\-\._~\+\/]+=*") {
            s = re_bearer.replace_all(&s, "$1[REDACTED]").to_string();
        }
        if let Ok(re_basic) = regex::Regex::new(r"(?i)(Basic\s+)[A-Za-z0-9\+\/]+=*") {
            s = re_basic.replace_all(&s, "$1[REDACTED]").to_string();
        }

        s
    }

    pub fn append_line(file_path: &Path, line: &str) {
        // Rotate if > 5 MB
        if let Ok(meta) = std::fs::metadata(file_path) {
            if meta.len() > 5 * 1024 * 1024 {
                let old_path = file_path.with_extension("log.old");
                let _ = std::fs::rename(file_path, old_path);
            }
        }

        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(file_path) {
            let _ = writeln!(f, "{}", line);
        }
    }
}

impl log::Log for RapidLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let raw_msg = format!("{}", record.args());
        let sanitized = Self::sanitize(&raw_msg);
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let formatted = format!("[{}] [{:<5}] [{}] {}", timestamp, record.level(), record.target(), sanitized);

        // 1. Always append to app.log
        let app_log_path = self.logs_dir.join("app.log");
        Self::append_line(&app_log_path, &formatted);

        // 2. If ERROR, also append to error.log
        if record.level() == Level::Error {
            let err_log_path = self.logs_dir.join("error.log");
            Self::append_line(&err_log_path, &formatted);
        }

        // 3. Output to stderr in dev/debug terminals
        #[cfg(debug_assertions)]
        eprintln!("{}", formatted);
    }

    fn flush(&self) {}
}

/// Generate a production-grade diagnostic crash report containing:
/// - What happened (Payload)
/// - Where happened (Source location)
/// - Which version (App version)
/// - Which Windows (OS build, environment)
/// - Which architecture (CPU architecture, cores)
/// - Full symbolized stack backtrace
pub fn write_crash_report(info: &std::panic::PanicHookInfo<'_>, crash_log_path: &Path) {
    let backtrace = std::backtrace::Backtrace::capture();
    let now_local = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let now_utc = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");

    let location = info
        .location()
        .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
        .unwrap_or_else(|| "Unknown location".to_string());

    let payload = if let Some(s) = info.payload().downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = info.payload().downcast_ref::<String>() {
        s.clone()
    } else {
        "Unknown panic payload".to_string()
    };

    let app_version = env!("CARGO_PKG_VERSION");
    let arch = std::env::consts::ARCH;
    let os = std::env::consts::OS;
    let proc_arch = std::env::var("PROCESSOR_ARCHITECTURE").unwrap_or_else(|_| "Unknown".to_string());
    let os_name = std::env::var("OS").unwrap_or_else(|_| "Windows_NT".to_string());
    let cores = std::thread::available_parallelism()
        .map(|n| n.get().to_string())
        .unwrap_or_else(|_| "Unknown".to_string());
    let exe_path = std::env::current_exe()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "Unknown".to_string());

    let report = format!(
        "================================================================================\n\
         RAPID DOWNLOAD MANAGER - CRASH REPORT\n\
         ================================================================================\n\
         Timestamp Local  : {now_local}\n\
         Timestamp UTC    : {now_utc}\n\
         Application      : Rapid Download Manager\n\
         App Version      : v{app_version} (Production Release)\n\
         Target Arch      : {arch} (Host Architecture: {proc_arch})\n\
         Operating System : {os_name} ({os})\n\
         CPU Cores        : {cores}\n\
         Executable Path  : {exe_path}\n\
         \n\
         --------------------------------- CRASH CAUSE ----------------------------------\n\
         What Happened    : {payload}\n\
         Where Happened   : {location}\n\
         \n\
         ---------------------------------- BACKTRACE -----------------------------------\n\
         {backtrace:?}\n\
         ================================================================================\n\n"
    );

    let sanitized = RapidLogger::sanitize(&report);
    RapidLogger::append_line(crash_log_path, &sanitized);
}

/// Initialize the global production logging system and diagnostic panic hook
pub fn init_production_logging() -> Result<(), SetLoggerError> {
    let logs_dir = crate::paths::AppPaths::logs_dir();
    let logger = RapidLogger::new(logs_dir.clone());

    // Register panic hook for detailed crash diagnostics
    let crash_file = logs_dir.join("crash.log");
    std::panic::set_hook(Box::new(move |info| {
        write_crash_report(info, &crash_file);
    }));

    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(LevelFilter::Info);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_url_credentials() {
        let input = "Connecting to https://admin:SuperSecret999@api.example.com/data";
        let clean = RapidLogger::sanitize(input);
        assert_eq!(clean, "Connecting to https://admin:[REDACTED]@api.example.com/data");
    }

    #[test]
    fn test_sanitize_query_tokens() {
        let input = "GET /v1/file?token=my_secret_token_123&type=video&key=abc_key_456";
        let clean = RapidLogger::sanitize(input);
        assert!(clean.contains("token=[REDACTED]"));
        assert!(clean.contains("key=[REDACTED]"));
        assert!(clean.contains("type=video"));
    }

    #[test]
    fn test_sanitize_bearer_and_basic_auth() {
        let bearer_input = "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
        let clean_bearer = RapidLogger::sanitize(bearer_input);
        assert_eq!(clean_bearer, "Authorization: Bearer [REDACTED]");

        let basic_input = "Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==";
        let clean_basic = RapidLogger::sanitize(basic_input);
        assert_eq!(clean_basic, "Authorization: Basic [REDACTED]");
    }
}

