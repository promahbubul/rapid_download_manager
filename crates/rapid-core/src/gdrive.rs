use crate::error::{RapidError, Result};
use reqwest::header::CONTENT_DISPOSITION;
use reqwest::Client;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GDriveResourceType {
    File(String),
    Folder(String),
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ResolvedGDriveFile {
    pub id: String,
    pub name: String,
    pub download_url: String,
    pub size_bytes: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct GDriveItem {
    pub id: String,
    pub name: String,
    pub is_folder: bool,
}

pub struct GDriveResolver;

impl GDriveResolver {
    /// Detect if a URL is a Google Drive link and identify whether it is a File or a Folder
    pub fn parse_resource_type(url: &str) -> GDriveResourceType {
        if !url.contains("drive.google.com") && !url.contains("googleusercontent.com") {
            return GDriveResourceType::Unknown;
        }

        // 1. Folder patterns: /drive/folders/{ID} or /drive/u/0/folders/{ID}
        if let Some(pos) = url.find("/folders/") {
            let after = &url[pos + 9..];
            let id: String = after
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
                .collect();
            if !id.is_empty() {
                return GDriveResourceType::Folder(id);
            }
        }

        // 2. File patterns: /file/d/{ID}/view or /file/d/{ID}
        if let Some(pos) = url.find("/file/d/") {
            let after = &url[pos + 8..];
            let id: String = after
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
                .collect();
            if !id.is_empty() {
                return GDriveResourceType::File(id);
            }
        }

        // 3. Query patterns: ?id={ID} or &id={ID}
        if let Some(pos) = url.find("id=") {
            let after = &url[pos + 3..];
            let id: String = after
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
                .collect();
            if !id.is_empty() {
                return GDriveResourceType::File(id);
            }
        }

        GDriveResourceType::Unknown
    }

    /// Resolve a Google Drive file ID into a direct confirmed streaming URL,
    /// extracting the real filename and bypassing virus scan warning pages.
    pub async fn resolve_file_download_url(
        client: &Client,
        file_id: &str,
    ) -> Result<ResolvedGDriveFile> {
        let direct_url = format!(
            "https://drive.usercontent.google.com/download?id={}&export=download",
            file_id
        );

        let resp = client
            .get(&direct_url)
            .send()
            .await
            .map_err(RapidError::Network)?;

        let status = resp.status();
        let content_type = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_lowercase();

        // If it's already binary/octet-stream/video/audio:
        if !content_type.contains("text/html") && (status.is_success() || status == reqwest::StatusCode::SEE_OTHER) {
            let filename = resp
                .headers()
                .get(CONTENT_DISPOSITION)
                .and_then(|v| v.to_str().ok())
                .and_then(Self::extract_filename_from_disposition)
                .unwrap_or_else(|| format!("{}.bin", file_id));

            let size = resp
                .headers()
                .get(reqwest::header::CONTENT_LENGTH)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok());

            return Ok(ResolvedGDriveFile {
                id: file_id.to_string(),
                name: filename,
                download_url: resp.url().to_string(),
                size_bytes: size,
            });
        }

        // If it's HTML, check for the virus warning form:
        let html = resp.text().await.unwrap_or_default();

        if let Some(confirmed_url) = Self::extract_confirm_form_url(&html, &direct_url) {
            let confirmed_resp = client
                .head(&confirmed_url)
                .send()
                .await;

            let (filename, size) = if let Ok(cr) = confirmed_resp {
                let fn_opt = cr
                    .headers()
                    .get(CONTENT_DISPOSITION)
                    .and_then(|v| v.to_str().ok())
                    .and_then(Self::extract_filename_from_disposition);
                let sz_opt = cr
                    .headers()
                    .get(reqwest::header::CONTENT_LENGTH)
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok());
                (fn_opt, sz_opt)
            } else {
                (None, None)
            };

            let final_name = filename
                .or_else(|| Self::extract_filename_from_html(&html))
                .unwrap_or_else(|| format!("file_{}.bin", file_id));

            return Ok(ResolvedGDriveFile {
                id: file_id.to_string(),
                name: final_name,
                download_url: confirmed_url,
                size_bytes: size,
            });
        }

        let filename = Self::extract_filename_from_html(&html)
            .unwrap_or_else(|| format!("Google_File_{}.bin", file_id));

        Ok(ResolvedGDriveFile {
            id: file_id.to_string(),
            name: filename,
            download_url: direct_url,
            size_bytes: None,
        })
    }

    pub async fn crawl_folder_default(
        folder_id: &str,
        max_depth: usize,
    ) -> Result<Vec<GDriveItem>> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(RapidError::Network)?;
        Self::crawl_folder(&client, folder_id, max_depth).await
    }

    /// Crawl a Google Drive folder and its subfolders to collect all downloadable files
    pub async fn crawl_folder(
        client: &Client,
        folder_id: &str,
        max_depth: usize,
    ) -> Result<Vec<GDriveItem>> {
        let mut all_files = Vec::new();
        let mut visited_folders = HashSet::new();
        let mut queue = vec![(folder_id.to_string(), 0usize)];

        while let Some((curr_folder_id, depth)) = queue.pop() {
            if visited_folders.contains(&curr_folder_id) || depth > max_depth {
                continue;
            }
            visited_folders.insert(curr_folder_id.clone());

            let folder_url = format!("https://drive.google.com/drive/folders/{}", curr_folder_id);
            let resp = match client.get(&folder_url).send().await {
                Ok(r) => r,
                Err(_) => continue,
            };

            let html = match resp.text().await {
                Ok(t) => t,
                Err(_) => continue,
            };

            let items = Self::parse_folder_items(&html);
            for item in items {
                if item.is_folder {
                    if depth + 1 <= max_depth {
                        queue.push((item.id, depth + 1));
                    }
                } else {
                    all_files.push(item);
                }
            }
        }

        Ok(all_files)
    }

    /// Parse folder HTML items using data-id and aria-label
    pub fn parse_folder_items(html: &str) -> Vec<GDriveItem> {
        let mut items = Vec::new();
        let mut seen_ids = HashSet::new();

        let mut search_idx = 0;
        while let Some(data_pos) = html[search_idx..].find("data-id=\"") {
            let abs_pos = search_idx + data_pos + 9;
            let end_id = match html[abs_pos..].find('"') {
                Some(p) => p,
                None => break,
            };
            let id = &html[abs_pos..abs_pos + end_id];
            search_idx = abs_pos + end_id;

            if id.len() < 25 || seen_ids.contains(id) {
                continue;
            }

            let chunk_len = 1500.min(html.len() - search_idx);
            let chunk = &html[search_idx..search_idx + chunk_len];

            if let Some(aria_pos) = chunk.find("aria-label=\"") {
                let aria_start = aria_pos + 12;
                if let Some(aria_end) = chunk[aria_start..].find('"') {
                    let full_label = &chunk[aria_start..aria_start + aria_end];
                    
                    if full_label.starts_with("Modified") || full_label.contains("à¦¶à§‡à§Ÿà¦¾à¦°") {
                        continue;
                    }

                    seen_ids.insert(id.to_string());
                    let is_folder = full_label.contains("Shared folder") || full_label.contains("Folder");
                    
                    let clean_name = full_label
                        .replace("Shared folder", "")
                        .replace("Folder", "")
                        .replace("Video Shared", "")
                        .replace("PDF Shared", "")
                        .replace("Text Shared", "")
                        .replace("Image Shared", "")
                        .trim()
                        .to_string();

                    if !clean_name.is_empty() {
                        items.push(GDriveItem {
                            id: id.to_string(),
                            name: clean_name,
                            is_folder,
                        });
                    }
                }
            }
        }

        items
    }

    /// Extract confirmed URL from Google's virus scan warning form
    pub fn extract_confirm_form_url(html: &str, current_url: &str) -> Option<String> {
        if let Some(form_pos) = html.find("id=\"download-form\"") {
            let form_slice = &html[form_pos..];
            let action_url = if let Some(act_pos) = form_slice.find("action=\"") {
                let act_slice = &form_slice[act_pos + 8..];
                if let Some(act_end) = act_slice.find('"') {
                    Some(act_slice[..act_end].replace("&amp;", "&"))
                } else {
                    None
                }
            } else {
                None
            };

            let form_end = form_slice.find("</form>").unwrap_or(form_slice.len());
            let inner_form = &form_slice[..form_end];

            let mut params = Vec::new();
            for input in inner_form.split("<input ") {
                if let Some(name_pos) = input.find("name=\"") {
                    let n_slice = &input[name_pos + 6..];
                    if let Some(name_end) = n_slice.find('"') {
                        let name = &n_slice[..name_end];
                        if let Some(val_pos) = input.find("value=\"") {
                            let v_slice = &input[val_pos + 7..];
                            if let Some(val_end) = v_slice.find('"') {
                                let val = &v_slice[..val_end];
                                params.push(format!("{}={}", name, val));
                            }
                        }
                    }
                }
            }

            if let Some(action) = action_url {
                if !params.is_empty() {
                    let sep = if action.contains('?') { "&" } else { "?" };
                    return Some(format!("{}{}{}", action, sep, params.join("&")));
                }
            }
        }

        if let Some(confirm_pos) = html.find("confirm=") {
            let after = &html[confirm_pos + 8..];
            let token: String = after
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
                .collect();
            if !token.is_empty() {
                let sep = if current_url.contains('?') { "&" } else { "?" };
                return Some(format!("{}{}confirm={}", current_url, sep, token));
            }
        }

        None
    }

    fn extract_filename_from_html(html: &str) -> Option<String> {
        if let Some(pos) = html.find("class=\"uc-name-size\"") {
            let slice = &html[pos..];
            if let Some(a_pos) = slice.find("<a ") {
                let after_a = &slice[a_pos..];
                if let Some(text_pos) = after_a.find('>') {
                    let name_slice = &after_a[text_pos + 1..];
                    if let Some(end_tag) = name_slice.find("</a>") {
                        let fn_str = name_slice[..end_tag].trim();
                        if !fn_str.is_empty() {
                            return Some(fn_str.to_string());
                        }
                    }
                }
            }
        }
        None
    }

    fn extract_filename_from_disposition(disposition: &str) -> Option<String> {
        for part in disposition.split(';') {
            let part = part.trim();
            if part.to_ascii_lowercase().starts_with("filename*=") {
                let val = &part[10..];
                if let Some(idx) = val.find("''") {
                    let encoded = val[idx + 2..].trim_matches('"');
                    if let Ok(decoded) = urlencoding::decode(encoded) {
                        return Some(decoded.into_owned());
                    }
                }
            } else if part.to_ascii_lowercase().starts_with("filename=") {
                let val = part[9..].trim_matches('"').trim();
                if !val.is_empty() {
                    return Some(val.to_string());
                }
            }
        }
        None
    }
}