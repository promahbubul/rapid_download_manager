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
        if !url.contains("drive.google.com") && !url.contains("googleusercontent.com") && !url.contains("usercontent.google.com") {
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
                .get(&confirmed_url)
                .header(reqwest::header::RANGE, "bytes=0-0")
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
                    .get(reqwest::header::CONTENT_RANGE)
                    .and_then(|v| v.to_str().ok())
                    .and_then(|cr_str| cr_str.rsplit('/').next())
                    .and_then(|tot| tot.trim().parse::<u64>().ok())
                    .or_else(|| {
                        cr.headers()
                            .get(reqwest::header::CONTENT_LENGTH)
                            .and_then(|v| v.to_str().ok())
                            .and_then(|v| v.parse::<u64>().ok())
                    });
                (fn_opt, sz_opt)
            } else {
                (None, None)
            };

            let final_name = filename
                .or_else(|| Self::extract_filename_from_html(&html))
                .unwrap_or_else(|| format!("Google_File_{}.bin", file_id));

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

        /// Extract confirmed URL from Google's virus scan warning form, anchor link, or confirm token
    pub fn extract_confirm_form_url(html: &str, current_url: &str) -> Option<String> {
        let lower = html.to_lowercase();
        let form_starts: Vec<usize> = lower.match_indices("<form").map(|(i, _)| i).collect();

        for f_start in form_starts {
            let f_end = match lower[f_start..].find("</form>") {
                Some(idx) => f_start + idx + 7,
                None => html.len(),
            };
            let form_html = &html[f_start..f_end];
            let form_lower = &lower[f_start..f_end];

            if form_lower.contains("download") || form_lower.contains("confirm") || form_lower.contains("uc-download") {
                let tag_open_end = form_html.find('>').unwrap_or(form_html.len());
                let tag_open = &form_html[..tag_open_end];

                let action_opt = Self::extract_html_attribute(tag_open, "action");

                let mut params = Vec::new();
                for inp_slice in form_html.split("<input ") {
                    let inp_tag = inp_slice.split('>').next().unwrap_or("");
                    let name_val = Self::extract_html_attribute(inp_tag, "name");
                    let value_val = Self::extract_html_attribute(inp_tag, "value");

                    if let (Some(n), Some(v)) = (name_val, value_val) {
                        if !n.is_empty() {
                            params.push(format!("{}={}", n, v));
                        }
                    }
                }

                let base_action = action_opt.unwrap_or_else(|| {
                    if current_url.contains("usercontent.google.com") {
                        "https://drive.usercontent.google.com/download".to_string()
                    } else {
                        "https://drive.google.com/uc?export=download".to_string()
                    }
                });

                let resolved_action = if base_action.starts_with("http://") || base_action.starts_with("https://") {
                    base_action
                } else if base_action.starts_with('/') {
                    if base_action.starts_with("/download") {
                        format!("https://drive.usercontent.google.com{}", base_action)
                    } else {
                        format!("https://drive.google.com{}", base_action)
                    }
                } else {
                    format!("https://drive.google.com/{}", base_action)
                };

                if !params.is_empty() {
                    let sep = if resolved_action.contains('?') { "&" } else { "?" };
                    return Some(format!("{}{}{}", resolved_action, sep, params.join("&")));
                } else {
                    return Some(resolved_action);
                }
            }
        }

        for a_slice in html.split("<a ") {
            let a_tag = a_slice.split('>').next().unwrap_or("");
            let a_lower = a_tag.to_lowercase();
            if a_lower.contains("download") || a_lower.contains("confirm") || a_lower.contains("uc-download-link") {
                if let Some(raw_link) = Self::extract_html_attribute(a_tag, "href") {
                    let decoded = raw_link.replace("&amp;", "&");
                    if decoded.contains("confirm=") || decoded.contains("export=download") || decoded.contains("drive.usercontent.google.com") {
                        if decoded.starts_with("http://") || decoded.starts_with("https://") {
                            return Some(decoded);
                        } else if decoded.starts_with('/') {
                            if decoded.starts_with("/download") {
                                return Some(format!("https://drive.usercontent.google.com{}", decoded));
                            } else {
                                return Some(format!("https://drive.google.com{}", decoded));
                            }
                        }
                    }
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

    fn extract_html_attribute(tag: &str, attr: &str) -> Option<String> {
        let double_q = format!("{}=\"", attr);
        let single_q = format!("{}='", attr);
        let no_q = format!("{}=", attr);

        if let Some(pos) = tag.to_lowercase().find(&double_q) {
            let after = &tag[pos + double_q.len()..];
            return after.split('"').next().map(|s| s.trim().to_string());
        }
        if let Some(pos) = tag.to_lowercase().find(&single_q) {
            let after = &tag[pos + single_q.len()..];
            return after.split("'").next().map(|s| s.trim().to_string());
        }
        if let Some(pos) = tag.to_lowercase().find(&no_q) {
            let after = &tag[pos + no_q.len()..];
            return after.split_whitespace().next().map(|s| s.trim().to_string());
        }
        None
    }

    fn extract_filename_from_html(html: &str) -> Option<String> {
        // Pattern 1: class="uc-name-size"
        if let Some(pos) = html.find("class=\"uc-name-size\"") {
            let slice = &html[pos..];
            if let Some(a_pos) = slice.find("<a ") {
                let after_a = &slice[a_pos..];
                if let Some(text_pos) = after_a.find('>') {
                    let name_slice = &after_a[text_pos + 1..];
                    if let Some(end_tag) = name_slice.find("</a>") {
                        let fn_str = name_slice[..end_tag].trim();
                        if !fn_str.is_empty() && !crate::engine::is_generic_placeholder(fn_str) {
                            return Some(fn_str.to_string());
                        }
                    }
                }
            }
        }

        // Pattern 2: Title tag: <title>Filename.ext - Google Drive</title>
        if let Some(t_start) = html.find("<title>") {
            let slice = &html[t_start + 7..];
            if let Some(t_end) = slice.find("</title>") {
                let title_text = slice[..t_end].trim();
                if let Some(pos) = title_text.find(" - Google Drive") {
                    let fn_str = title_text[..pos].trim();
                    if !fn_str.is_empty() && !crate::engine::is_generic_placeholder(fn_str) {
                        return Some(fn_str.to_string());
                    }
                }
            }
        }

        // Pattern 3: input hidden name="filename" value="..."
        if let Some(fn_pos) = html.find("name=\"filename\"") {
            let slice = &html[fn_pos..];
            if let Some(val_pos) = slice.find("value=\"") {
                let val_slice = &slice[val_pos + 7..];
                if let Some(end_quote) = val_slice.find('"') {
                    let fn_str = val_slice[..end_quote].trim();
                    if !fn_str.is_empty() && !crate::engine::is_generic_placeholder(fn_str) {
                        return Some(fn_str.to_string());
                    }
                }
            }
        }

        None
    }

    fn extract_filename_from_disposition(disposition: &str) -> Option<String> {
        crate::probe::Probe::extract_filename_from_cd(disposition)
    }
}