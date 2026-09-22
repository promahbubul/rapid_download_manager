use crate::error::{RapidError, Result};
use crate::types::DownloadMetadata;
use reqwest::header::{ACCEPT_RANGES, CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_RANGE, RANGE, USER_AGENT};
use reqwest::Client;
use url::Url;
use urlencoding::decode as url_decode;

pub struct Probe;

impl Probe {
    pub async fn inspect_url(url_str: &str) -> Result<DownloadMetadata> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(20))
            .build()?;
        Self::inspect(&client, url_str).await
    }

    pub async fn inspect(client: &Client, url_str: &str) -> Result<DownloadMetadata> {
        let parsed_url = Url::parse(url_str)
            .map_err(|e| RapidError::InvalidUrl(format!("{}: {}", url_str, e)))?;

        // 1. Try HEAD request first
        let head_resp = client
            .head(url_str)
            .send()
            .await;

        if let Ok(resp) = head_resp {
            if resp.status().is_success() {
                let content_length = resp
                    .headers()
                    .get(CONTENT_LENGTH)
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok());

                let accept_ranges = resp
                    .headers()
                    .get(ACCEPT_RANGES)
                    .and_then(|v| v.to_str().ok())
                    .map(|v| v.to_lowercase().contains("bytes"))
                    .unwrap_or(false);

                let etag = resp
                    .headers()
                    .get("etag")
                    .and_then(|v| v.to_str().ok())
                    .map(|s| s.to_string());

                let last_modified = resp
                    .headers()
                    .get("last-modified")
                    .and_then(|v| v.to_str().ok())
                    .map(|s| s.to_string());

                let disposition = resp
                    .headers()
                    .get(CONTENT_DISPOSITION)
                    .and_then(|v| v.to_str().ok());

                let filename = Self::determine_filename(disposition, &parsed_url);

                // If we got content_length, we are good!
                if content_length.is_some() {
                    return Ok(DownloadMetadata {
                        url: url_str.to_string(),
                        filename,
                        content_length,
                        accept_ranges,
                        etag,
                        last_modified,
                    });
                }
            }
        }

        // 2. Fallback: GET with Range: bytes=0-0 to inspect headers
        let get_resp = client
            .get(url_str)
            .header(RANGE, "bytes=0-0")
            .send()
            .await?;

        let status = get_resp.status();
        let headers = get_resp.headers();

        let mut content_length = None;
        let mut accept_ranges = false;

        if status == reqwest::StatusCode::PARTIAL_CONTENT {
            accept_ranges = true;
            if let Some(cr) = headers.get(CONTENT_RANGE).and_then(|v| v.to_str().ok()) {
                // Example: bytes 0-0/10485760
                if let Some(total_str) = cr.rsplit('/').next() {
                    content_length = total_str.trim().parse::<u64>().ok();
                }
            }
        } else if status.is_success() {
            content_length = headers
                .get(CONTENT_LENGTH)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok());

            accept_ranges = headers
                .get(ACCEPT_RANGES)
                .and_then(|v| v.to_str().ok())
                .map(|v| v.to_lowercase().contains("bytes"))
                .unwrap_or(false);
        }

        let disposition = headers
            .get(CONTENT_DISPOSITION)
            .and_then(|v| v.to_str().ok());
        let filename = Self::determine_filename(disposition, &parsed_url);

        let etag = headers
            .get("etag")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let last_modified = headers
            .get("last-modified")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        Ok(DownloadMetadata {
            url: url_str.to_string(),
            filename,
            content_length,
            accept_ranges,
            etag,
            last_modified,
        })
    }

    fn determine_filename(content_disposition: Option<&str>, url: &Url) -> String {
        // Try content-disposition
        if let Some(cd) = content_disposition {
            if let Some(name) = Self::extract_filename_from_cd(cd) {
                let sanitized = Self::sanitize_filename(&name);
                if !sanitized.is_empty() {
                    return sanitized;
                }
            }
        }

        // Try URL path segments
        if let Some(segments) = url.path_segments() {
            let last = segments.filter(|s| !s.is_empty()).last();
            if let Some(name) = last {
                if let Ok(decoded) = url_decode(name) {
                    let sanitized = Self::sanitize_filename(&decoded);
                    if !sanitized.is_empty() {
                        return sanitized;
                    }
                }
            }
        }

        "download.bin".to_string()
    }

    fn extract_filename_from_cd(cd: &str) -> Option<String> {
        // e.g. attachment; filename*=UTF-8''my%20file.zip
        for part in cd.split(';') {
            let part = part.trim();
            if part.to_lowercase().starts_with("filename*=") {
                let value = &part[10..].trim();
                let value = value.trim_matches('"');
                if let Some(rest) = value.strip_prefix("UTF-8''").or_else(|| value.strip_prefix("utf-8''")) {
                    if let Ok(decoded) = url_decode(rest) {
                        return Some(decoded.into_owned());
                    }
                }
            } else if part.to_lowercase().starts_with("filename=") {
                let value = &part[9..].trim();
                let value = value.trim_matches('"').trim_matches('\'');
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
        None
    }

    pub fn sanitize_filename(name: &str) -> String {
        let invalid = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
        name.chars()
            .map(|c| if invalid.contains(&c) || c.is_control() { '_' } else { c })
            .collect::<String>()
            .trim()
            .to_string()
    }
}

// url_decode removed - now using the `urlencoding` crate (url_decode alias above).
