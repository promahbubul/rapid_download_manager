use crate::error::{RapidError, Result};
use crate::types::DownloadMetadata;
use reqwest::header::{ACCEPT_RANGES, CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_RANGE, RANGE};
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
        if url_str.starts_with("blob:") {
            return Err(RapidError::InvalidUrl(
                "Browser Blob URLs ('blob:...') cannot be downloaded directly. Please play 1-2 seconds of the video in your browser so Rapid's extension can capture the direct stream URL.".to_string(),
            ));
        }

        if crate::youtube::YoutubeResolver::is_youtube(url_str) {
            let canonical = crate::youtube::YoutubeResolver::canonicalize_url(url_str, None);
            if let Ok(meta) = crate::youtube::YoutubeResolver::resolve_metadata(&canonical).await {
                return Ok(DownloadMetadata {
                    url: canonical,
                    filename: meta.clean_filename,
                    content_length: None,
                    accept_ranges: true,
                    etag: None,
                    last_modified: None,
                });
            }
        }

        let parsed_url = Url::parse(url_str)
            .map_err(|e| RapidError::InvalidUrl(format!("{}: {}", url_str, e)))?;

        // 1. Try HEAD request first
        let head_resp = client
            .head(url_str)
            .send()
            .await;

        if let Ok(resp) = head_resp {
            if resp.status().is_success() {
                let final_url = resp.url().to_string();
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

                let content_type = resp.headers().get(reqwest::header::CONTENT_TYPE).and_then(|v| v.to_str().ok());
        let filename = Self::determine_filename(disposition, content_type, &parsed_url);

                // If we got content_length, we are good!
                if content_length.is_some() {
                    return Ok(DownloadMetadata {
                        url: final_url,
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

        let final_url = get_resp.url().to_string();
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
        let content_type = headers.get(reqwest::header::CONTENT_TYPE).and_then(|v| v.to_str().ok());
        let filename = Self::determine_filename(disposition, content_type, &parsed_url);

        let etag = headers
            .get("etag")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let last_modified = headers
            .get("last-modified")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        if content_length.is_none() {
            if let Some(query) = parsed_url.query() {
                for pair in query.split('&') {
                    if let Some(val) = pair.strip_prefix("clen=") {
                        if let Ok(cl) = val.parse::<u64>() {
                            content_length = Some(cl);
                            break;
                        }
                    }
                }
            }
        }

        Ok(DownloadMetadata {
            url: final_url,
            filename,
            content_length,
            accept_ranges,
            etag,
            last_modified,
        })
    }

    pub fn determine_filename(
        content_disposition: Option<&str>,
        content_type: Option<&str>,
        url: &Url,
    ) -> String {
        // 1. Try content-disposition (RFC 6266 / RFC 5987 aware)
        if let Some(cd) = content_disposition {
            if let Some(name) = Self::extract_filename_from_cd(cd) {
                let sanitized = Self::sanitize_filename(&name);
                if !sanitized.is_empty() && !crate::engine::is_generic_placeholder(&sanitized) {
                    if has_file_extension(&sanitized) {
                        return sanitized;
                    } else if let Some(ext) = extension_from_mime(content_type) {
                        return format!("{}.{}", sanitized, ext);
                    } else {
                        return sanitized;
                    }
                }
            }
        }

        // 2. Try URL path segments
        let mut candidate_from_url: Option<String> = None;
        if let Some(segments) = url.path_segments() {
            let last = segments.filter(|s| !s.is_empty()).last();
            if let Some(name) = last {
                if let Ok(decoded) = url_decode(name) {
                    let sanitized = Self::sanitize_filename(&decoded);
                    if !sanitized.is_empty() {
                        let stripped = strip_script_extension(&sanitized);
                        if !crate::engine::is_generic_placeholder(&stripped) {
                            candidate_from_url = Some(stripped);
                        }
                    }
                }
            }
        }

        if let Some(cand) = candidate_from_url {
            if has_file_extension(&cand) {
                return cand;
            } else if let Some(ext) = extension_from_mime(content_type) {
                return format!("{}.{}", cand, ext);
            } else {
                return cand;
            }
        }

        // 3. Deduce from MIME type
        if let Some(ext) = extension_from_mime(content_type) {
            return format!("download.{}", ext);
        }

        "download.bin".to_string()
    }

    pub fn extract_filename_from_cd(cd: &str) -> Option<String> {
        let mut filename_star = None;
        let mut regular_filename = None;

        for part in cd.split(';') {
            let part = part.trim();
            let lower = part.to_ascii_lowercase();

            if lower.starts_with("filename*") {
                if let Some(eq_pos) = part.find('=') {
                    let val = part[eq_pos + 1..].trim().trim_matches('"');
                    if let Some(first_quote) = val.find(|c: char| c == '\'') {
                        if let Some(second_quote) = val[first_quote + 1..].find(|c: char| c == '\'') {
                            let encoded = &val[first_quote + 1 + second_quote + 1..];
                            if let Ok(decoded) = url_decode(encoded) {
                                let name = decoded.into_owned();
                                if !name.is_empty() {
                                    filename_star = Some(name);
                                }
                            }
                        }
                    } else if let Ok(decoded) = url_decode(val) {
                        let name = decoded.into_owned();
                        if !name.is_empty() {
                            filename_star = Some(name);
                        }
                    }
                }
            } else if lower.starts_with("filename") {
                if let Some(eq_pos) = part.find('=') {
                    let val = part[eq_pos + 1..].trim().trim_matches('"').trim_matches(|c: char| c == '\'');
                    if !val.is_empty() {
                        let base = std::path::Path::new(val)
                            .file_name()
                            .and_then(|s| s.to_str())
                            .unwrap_or(val);
                        regular_filename = Some(base.to_string());
                    }
                }
            }
        }

        filename_star.or(regular_filename)
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

fn has_file_extension(name: &str) -> bool {
    if let Some(pos) = name.rfind('.') {
        let ext = &name[pos + 1..];
        !ext.is_empty() && ext.len() <= 5 && ext.chars().all(|c| c.is_alphanumeric())
    } else {
        false
    }
}

fn strip_script_extension(name: &str) -> String {
    let lower = name.to_ascii_lowercase();
    let script_exts = [".php", ".asp", ".aspx", ".jsp", ".do", ".action", ".cgi", ".pl"];
    for ext in script_exts {
        if lower.ends_with(ext) {
            return name[..name.len() - ext.len()].to_string();
        }
    }
    name.to_string()
}

pub fn extension_from_mime(content_type: Option<&str>) -> Option<&'static str> {
    let ct = content_type?.to_ascii_lowercase();
    let mime = ct.split(';').next()?.trim();

    match mime {
        "video/mp4" => Some("mp4"),
        "video/x-matroska" => Some("mkv"),
        "video/webm" => Some("webm"),
        "video/quicktime" => Some("mov"),
        "video/x-msvideo" => Some("avi"),
        "video/x-flv" => Some("flv"),
        "audio/mpeg" | "audio/mp3" => Some("mp3"),
        "audio/wav" | "audio/x-wav" => Some("wav"),
        "audio/flac" | "audio/x-flac" => Some("flac"),
        "audio/aac" => Some("aac"),
        "audio/ogg" | "application/ogg" => Some("ogg"),
        "audio/mp4" | "audio/m4a" | "audio/x-m4a" => Some("m4a"),
        "application/pdf" => Some("pdf"),
        "application/zip" | "application/x-zip-compressed" => Some("zip"),
        "application/x-rar-compressed" | "application/x-rar" | "application/vnd.rar" => Some("rar"),
        "application/x-7z-compressed" => Some("7z"),
        "application/x-tar" => Some("tar"),
        "application/gzip" | "application/x-gzip" => Some("gz"),
        "application/vnd.android.package-archive" => Some("apk"),
        "application/msword" => Some("doc"),
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => Some("docx"),
        "application/vnd.ms-excel" => Some("xls"),
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => Some("xlsx"),
        "application/vnd.ms-powerpoint" => Some("ppt"),
        "application/vnd.openxmlformats-officedocument.presentationml.presentation" => Some("pptx"),
        "image/jpeg" => Some("jpg"),
        "image/png" => Some("png"),
        "image/webp" => Some("webp"),
        "image/gif" => Some("gif"),
        "image/svg+xml" => Some("svg"),
        "text/plain" => Some("txt"),
        "application/json" => Some("json"),
        _ => None,
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn test_rfc5987_filename_precedence() {
        let cd = r#"attachment; filename="fallback.bin"; filename*=UTF-8''My%20Lecture%2001.mp4"#;
        let url = Url::parse("https://example.com/download").unwrap();
        let name = Probe::determine_filename(Some(cd), None, &url);
        assert_eq!(name, "My Lecture 01.mp4");
    }

    #[test]
    fn test_mime_extension_deduction() {
        let url = Url::parse("https://example.com/stream/v1/987654321").unwrap();
        let name = Probe::determine_filename(None, Some("video/mp4"), &url);
        assert_eq!(name, "987654321.mp4");

        let url2 = Url::parse("https://example.com/get_file.php?id=42").unwrap();
        let name2 = Probe::determine_filename(None, Some("application/pdf"), &url2);
        assert_eq!(name2, "get_file.pdf");
    }
}
