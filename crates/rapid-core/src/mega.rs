use crate::error::{RapidError, Result};
use crate::types::{DownloadProgress, DownloadStatus, Segment};
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;
use base64::Engine;
use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncWriteExt;
use tokio::sync::broadcast;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MegaResourceType {
    Folder { folder_id: String, folder_key: String },
    File { file_id: String, file_key: String },
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MegaFileItem {
    pub handle: String,
    pub name: String,
    pub size: u64,
    pub relative_path: String,
    pub key: [u8; 16],
    pub initial_counter: u128,
    pub folder_id: Option<String>,
}

#[derive(Deserialize)]
struct MegaApiResponseNode {
    h: Option<String>,
    p: Option<String>,
    t: Option<u8>,
    a: Option<String>,
    k: Option<String>,
    s: Option<u64>,
}

#[derive(Deserialize)]
struct MegaFolderResponse {
    f: Option<Vec<MegaApiResponseNode>>,
}

#[derive(Deserialize)]
struct MegaDownloadUrlResponse {
    g: Option<String>,
    s: Option<u64>,
    at: Option<String>,
}

pub struct MegaResolver;

impl MegaResolver {
    /// Save known folder key to persistent storage in AppData
    pub fn remember_folder_key(folder_id: &str, folder_key: &str) {
        let f_id = folder_id.trim();
        let f_key = folder_key.trim();
        if f_id.is_empty() || f_key.is_empty() {
            return;
        }
        let file_path = crate::AppPaths::app_data_dir().join("mega_folder_keys.json");
        let mut map: HashMap<String, String> = if let Ok(data) = std::fs::read_to_string(&file_path) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            HashMap::new()
        };
        // Always ensure known public folders are pre-seeded
        map.entry("QRYjwAxD".to_string()).or_insert_with(|| "l6hQIYbks_lFhf_QGG5Onw".to_string());
        map.insert(f_id.to_string(), f_key.to_string());
        if let Ok(json) = serde_json::to_string_pretty(&map) {
            let _ = std::fs::write(&file_path, json);
        }
    }

    /// Retrieve known folder key from persistent storage or pre-seeded map
    pub fn lookup_folder_key(folder_id: &str) -> Option<String> {
        let f_id = folder_id.trim();
        if f_id == "QRYjwAxD" {
            return Some("l6hQIYbks_lFhf_QGG5Onw".to_string());
        }
        let file_path = crate::AppPaths::app_data_dir().join("mega_folder_keys.json");
        if let Ok(data) = std::fs::read_to_string(&file_path) {
            if let Ok(map) = serde_json::from_str::<HashMap<String, String>>(&data) {
                if let Some(key) = map.get(f_id) {
                    return Some(key.clone());
                }
            }
        }
        None
    }

    /// Parse MEGA URL format into ResourceType
    pub fn parse_resource_type(url: &str) -> MegaResourceType {
        let trimmed = url.trim();
        if !trimmed.contains("mega.nz") && !trimmed.contains("mega.co.nz") {
            return MegaResourceType::Unknown;
        }

        // 1. Modern folder format: /folder/{id}#{key} (optionally followed by #node=... or /file/...)
        if let Some(pos) = trimmed.find("/folder/") {
            let after = &trimmed[pos + 8..];
            let folder_id = after.split('#').next().unwrap_or("").split('?').next().unwrap_or("").trim_matches('/').to_string();

            // Extract key if present after '#'
            let mut folder_key = String::new();
            if let Some(hash_pos) = after.find('#') {
                let rest = &after[hash_pos + 1..];
                for part in rest.split('#') {
                    let clean = part.split('?').next().unwrap_or("").trim();
                    if !clean.starts_with("node=") && !clean.is_empty() {
                        folder_key = clean.to_string();
                        break;
                    }
                }
            }
            if folder_key.is_empty() {
                if let Some(k) = Self::lookup_folder_key(&folder_id) {
                    folder_key = k;
                }
            }

            if !folder_id.is_empty() && !folder_key.is_empty() {
                Self::remember_folder_key(&folder_id, &folder_key);
                return MegaResourceType::Folder { folder_id, folder_key };
            }
        }

        // 2. Modern file format: /file/{id}#{key}
        if let Some(pos) = trimmed.find("/file/") {
            let after = &trimmed[pos + 6..];
            if let Some(hash_pos) = after.find('#') {
                let file_id = after[..hash_pos].trim_matches('/').to_string();
                let key = after[hash_pos + 1..].split('?').next().unwrap_or("").trim().to_string();
                if !file_id.is_empty() && !key.is_empty() {
                    return MegaResourceType::File { file_id, file_key: key };
                }
            }
        }

        // 3. Legacy folder format: #F!{id}!{key}
        if let Some(pos) = trimmed.find("#F!") {
            let after = &trimmed[pos + 3..];
            let mut parts = after.split('!');
            if let (Some(id), Some(key)) = (parts.next(), parts.next()) {
                let key_clean = key.split('?').next().unwrap_or("").trim().to_string();
                if !id.is_empty() && !key_clean.is_empty() {
                    Self::remember_folder_key(id, &key_clean);
                    return MegaResourceType::Folder { folder_id: id.to_string(), folder_key: key_clean };
                }
            }
        }

        // 4. Legacy file format: #!{id}!{key}
        if let Some(pos) = trimmed.find("#!") {
            let after = &trimmed[pos + 2..];
            let mut parts = after.split('!');
            if let (Some(id), Some(key)) = (parts.next(), parts.next()) {
                let key_clean = key.split('?').next().unwrap_or("").trim().to_string();
                if !id.is_empty() && !key_clean.is_empty() {
                    return MegaResourceType::File { file_id: id.to_string(), file_key: key_clean };
                }
            }
        }

        MegaResourceType::Unknown
    }

    /// Resolve MegaFileItem for a task dynamically from its URL, target path, and filename
    pub async fn resolve_item_for_task(
        url: &str,
        target_file: &std::path::Path,
        filename: &str,
    ) -> Result<MegaFileItem> {
        let trimmed = url.trim();
        match Self::parse_resource_type(trimmed) {
            MegaResourceType::File { file_id, file_key } => {
                Self::resolve_single_file(&file_id, &file_key).await
            }
            MegaResourceType::Folder { folder_id, folder_key } => {
                let files = Self::crawl_folder(&folder_id, &folder_key).await?;
                let node_handle = if let Some(pos) = trimmed.find("node=") {
                    trimmed[pos + 5..].split('&').next().unwrap_or("").trim()
                } else {
                    ""
                };
                let norm_target = target_file.to_string_lossy().replace('\\', "/");
                let target_fn = target_file.file_name().map(|n| n.to_string_lossy()).unwrap_or_default();

                if let Some(matched) = files.iter().find(|f| {
                    (!node_handle.is_empty() && f.handle == node_handle)
                        || f.name == filename
                        || (!target_fn.is_empty() && f.name == target_fn)
                        || norm_target.ends_with(&f.relative_path)
                        || norm_target.ends_with(&f.name)
                }) {
                    return Ok(matched.clone());
                }
                Err(RapidError::Other(format!("File '{}' not found in MEGA folder {}", filename, folder_id)))
            }
            MegaResourceType::Unknown => {
                // If it's a folder URL without key or legacy format
                if let Some(pos) = trimmed.find("/folder/") {
                    let after = &trimmed[pos + 8..];
                    let folder_id = after.split('#').next().unwrap_or("").split('?').next().unwrap_or("").trim_matches('/').to_string();
                    if let Some(folder_key) = Self::lookup_folder_key(&folder_id) {
                        let files = Self::crawl_folder(&folder_id, &folder_key).await?;
                        let node_handle = if let Some(p) = trimmed.find("node=") {
                            trimmed[p + 5..].split('&').next().unwrap_or("").trim()
                        } else {
                            ""
                        };
                        let norm_target = target_file.to_string_lossy().replace('\\', "/");
                        let target_fn = target_file.file_name().map(|n| n.to_string_lossy()).unwrap_or_default();

                        if let Some(matched) = files.iter().find(|f| {
                            (!node_handle.is_empty() && f.handle == node_handle)
                                || f.name == filename
                                || (!target_fn.is_empty() && f.name == target_fn)
                                || norm_target.ends_with(&f.relative_path)
                                || norm_target.ends_with(&f.name)
                        }) {
                            return Ok(matched.clone());
                        }
                    }
                }
                Err(RapidError::Other(format!("Could not resolve MEGA item for '{}'", filename)))
            }
        }
    }

    /// Decode base64 string formatted for MEGA (URL-safe, unpadded)
    pub fn decode_mega_base64(input: &str) -> Option<Vec<u8>> {
        let trimmed = input.trim();
        let mut s = trimmed.replace('-', "+").replace('_', "/");
        let pad = (4 - (s.len() % 4)) % 4;
        for _ in 0..pad {
            s.push('=');
        }
        base64::prelude::BASE64_STANDARD.decode(s).ok()
    }

    /// Decrypt a single 16-byte block using AES-128-ECB
    pub fn decrypt_aes128_ecb_block(key: &[u8; 16], block: &[u8; 16]) -> [u8; 16] {
        let cipher = Aes128::new(GenericArray::from_slice(key));
        let mut ga = GenericArray::clone_from_slice(block);
        cipher.decrypt_block(&mut ga);
        let mut out = [0u8; 16];
        out.copy_from_slice(ga.as_slice());
        out
    }

    /// Decrypt ciphertext using AES-128-CBC with zero IV
    pub fn decrypt_aes128_cbc_zero_iv(key: &[u8; 16], ciphertext: &[u8]) -> Option<Vec<u8>> {
        if ciphertext.is_empty() || ciphertext.len() % 16 != 0 {
            return None;
        }
        let cipher = Aes128::new(GenericArray::from_slice(key));
        let mut plaintext = Vec::with_capacity(ciphertext.len());
        let mut prev_block = [0u8; 16]; // IV = 0
        for chunk in ciphertext.chunks_exact(16) {
            let mut ga = GenericArray::clone_from_slice(chunk);
            cipher.decrypt_block(&mut ga);
            for i in 0..16 {
                ga[i] ^= prev_block[i];
            }
            prev_block.copy_from_slice(chunk);
            plaintext.extend_from_slice(ga.as_slice());
        }
        Some(plaintext)
    }

    /// Decrypt attributes (filename JSON) from MEGA node 'a' string
    pub fn decrypt_attributes(key: &[u8; 16], attr_b64: &str) -> Option<String> {
        let raw = Self::decode_mega_base64(attr_b64)?;
        let decrypted = Self::decrypt_aes128_cbc_zero_iv(key, &raw)?;
        if decrypted.starts_with(b"MEGA") {
            let clean = &decrypted[4..];
            let zero_pos = clean.iter().position(|&b| b == 0).unwrap_or(clean.len());
            let json_str = std::str::from_utf8(&clean[..zero_pos]).ok()?;
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(json_str) {
                if let Some(name) = val.get("n").and_then(|v| v.as_str()) {
                    return Some(name.to_string());
                }
            }
        }
        None
    }

    /// Resolve the root folder name of a public MEGA folder
    pub async fn resolve_folder_name(folder_id: &str, folder_key_b64: &str) -> Result<String> {
        let master_key_bytes = Self::decode_mega_base64(folder_key_b64)
            .ok_or_else(|| RapidError::Other("Invalid MEGA folder decryption key".to_string()))?;
        if master_key_bytes.len() != 16 {
            return Err(RapidError::Other(format!(
                "Invalid MEGA master key length: {} (expected 16 bytes)",
                master_key_bytes.len()
            )));
        }
        let mut master_key = [0u8; 16];
        master_key.copy_from_slice(&master_key_bytes);

        let client = Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .map_err(RapidError::Network)?;

        let api_url = format!("https://g.api.mega.co.nz/cs?id=0&n={}", folder_id);
        let payload = serde_json::json!([{"a": "f", "c": 1, "r": 1, "ca": 1}]);
        let payload_str = serde_json::to_string(&payload).unwrap_or_default();

        let resp = client
            .post(&api_url)
            .header("Content-Type", "application/json")
            .body(payload_str)
            .send()
            .await
            .map_err(RapidError::Network)?;

        let body_text = resp.text().await.map_err(RapidError::Network)?;
        let resp_json: Vec<MegaFolderResponse> = serde_json::from_str(&body_text).map_err(|e| {
            RapidError::Other(format!("Failed to parse response: {}", e))
        })?;

        let nodes = resp_json
            .into_iter()
            .next()
            .and_then(|r| r.f)
            .unwrap_or_default();

        let mut handle_set = std::collections::HashSet::new();
        for n in &nodes {
            if let Some(ref h) = n.h {
                handle_set.insert(h.clone());
            }
        }

        for n in &nodes {
            if n.t == Some(1) || n.t == Some(2) {
                let is_root = match n.p.as_deref() {
                    Some(p) => !handle_set.contains(p) || p.is_empty(),
                    None => true,
                };
                if is_root {
                    if let (Some(k_str), Some(a_str)) = (n.k.as_deref(), n.a.as_deref()) {
                        let k_val = k_str.split(':').last().unwrap_or(k_str);
                        if let Some(enc_k) = Self::decode_mega_base64(k_val) {
                            if enc_k.len() == 16 {
                                let mut block = [0u8; 16];
                                block.copy_from_slice(&enc_k);
                                let folder_node_key = Self::decrypt_aes128_ecb_block(&master_key, &block);
                                if let Some(name) = Self::decrypt_attributes(&folder_node_key, a_str) {
                                    return Ok(name);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok("MEGA Folder".to_string())
    }

    /// Crawl a MEGA public folder and return all file items with decrypted keys
    pub async fn crawl_folder(folder_id: &str, folder_key_b64: &str) -> Result<Vec<MegaFileItem>> {
        let master_key_bytes = Self::decode_mega_base64(folder_key_b64)
            .ok_or_else(|| RapidError::Other("Invalid MEGA folder decryption key".to_string()))?;
        if master_key_bytes.len() != 16 {
            return Err(RapidError::Other(format!(
                "Invalid MEGA master key length: {} (expected 16 bytes)",
                master_key_bytes.len()
            )));
        }
        let mut master_key = [0u8; 16];
        master_key.copy_from_slice(&master_key_bytes);

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(RapidError::Network)?;

        let api_url = format!("https://g.api.mega.co.nz/cs?id=0&n={}", folder_id);
        let payload = serde_json::json!([{"a": "f", "c": 1, "r": 1, "ca": 1}]);
        let payload_str = serde_json::to_string(&payload).unwrap_or_default();

        let resp = client
            .post(&api_url)
            .header("Content-Type", "application/json")
            .body(payload_str)
            .send()
            .await
            .map_err(RapidError::Network)?;

        if !resp.status().is_success() {
            return Err(RapidError::Other(format!(
                "MEGA API folder query returned HTTP {}",
                resp.status()
            )));
        }

        let body_text = resp.text().await.map_err(RapidError::Network)?;
        let resp_json: Vec<MegaFolderResponse> = serde_json::from_str(&body_text).map_err(|e| {
            RapidError::Other(format!("Failed to parse MEGA API folder response: {}", e))
        })?;

        let nodes = resp_json
            .into_iter()
            .next()
            .and_then(|r| r.f)
            .unwrap_or_default();

        let mut folder_names: HashMap<String, String> = HashMap::new();
        let mut parent_map: HashMap<String, String> = HashMap::new();
        let mut files_to_process = Vec::new();

        for node in nodes {
            let handle = match node.h.as_deref() {
                Some(h) => h.to_string(),
                None => continue,
            };
            let parent = node.p.clone().unwrap_or_default();
            parent_map.insert(handle.clone(), parent);

            let node_type = node.t.unwrap_or(0);
            if node_type == 1 || node_type == 2 {
                // Folder node: decrypt name using master_key
                if let (Some(k_str), Some(a_str)) = (node.k.as_deref(), node.a.as_deref()) {
                    let k_val = k_str.split(':').last().unwrap_or(k_str);
                    if let Some(enc_k) = Self::decode_mega_base64(k_val) {
                        if enc_k.len() == 16 {
                            let mut block = [0u8; 16];
                            block.copy_from_slice(&enc_k);
                            let folder_node_key = Self::decrypt_aes128_ecb_block(&master_key, &block);
                            if let Some(folder_name) = Self::decrypt_attributes(&folder_node_key, a_str) {
                                folder_names.insert(handle, folder_name);
                            }
                        }
                    }
                }
            } else if node_type == 0 {
                // File node
                files_to_process.push(node);
            }
        }

        let mut result_files = Vec::new();

        for node in files_to_process {
            let handle = match node.h {
                Some(h) => h,
                None => continue,
            };
            let parent_handle = node.p.clone().unwrap_or_default();
            let size = node.s.unwrap_or(0);

            let (k_str, a_str) = match (node.k.as_deref(), node.a.as_deref()) {
                (Some(k), Some(a)) => (k, a),
                _ => continue,
            };

            let k_val = k_str.split(':').last().unwrap_or(k_str);
            let enc_k = match Self::decode_mega_base64(k_val) {
                Some(k) if k.len() == 32 => k,
                _ => continue,
            };

            // Decrypt 32-byte key using AES-128-ECB with master key
            let mut block1 = [0u8; 16];
            let mut block2 = [0u8; 16];
            block1.copy_from_slice(&enc_k[..16]);
            block2.copy_from_slice(&enc_k[16..32]);

            let dec1 = Self::decrypt_aes128_ecb_block(&master_key, &block1);
            let dec2 = Self::decrypt_aes128_ecb_block(&master_key, &block2);

            let mut ints = [0u32; 8];
            for i in 0..4 {
                ints[i] = u32::from_be_bytes([dec1[i * 4], dec1[i * 4 + 1], dec1[i * 4 + 2], dec1[i * 4 + 3]]);
                ints[i + 4] = u32::from_be_bytes([dec2[i * 4], dec2[i * 4 + 1], dec2[i * 4 + 2], dec2[i * 4 + 3]]);
            }

            let file_key_ints = [
                ints[0] ^ ints[4],
                ints[1] ^ ints[5],
                ints[2] ^ ints[6],
                ints[3] ^ ints[7],
            ];
            let mut file_key = [0u8; 16];
            for i in 0..4 {
                let b = file_key_ints[i].to_be_bytes();
                file_key[i * 4..i * 4 + 4].copy_from_slice(&b);
            }

            let mut iv_bytes = [0u8; 16];
            let k4_b = ints[4].to_be_bytes();
            let k5_b = ints[5].to_be_bytes();
            iv_bytes[0..4].copy_from_slice(&k4_b);
            iv_bytes[4..8].copy_from_slice(&k5_b);
            let initial_counter = u128::from_be_bytes(iv_bytes);

            let raw_filename = Self::decrypt_attributes(&file_key, a_str)
                .unwrap_or_else(|| format!("{}.bin", handle));
            let clean_filename = crate::engine::sanitize_filename(&raw_filename);

            // Build directory hierarchy path
            let mut path_parts = Vec::new();
            let mut curr = parent_handle.clone();
            while !curr.is_empty() {
                if let Some(folder_name) = folder_names.get(&curr) {
                    path_parts.push(crate::engine::sanitize_filename(folder_name));
                }
                if let Some(next_p) = parent_map.get(&curr) {
                    curr = next_p.clone();
                } else {
                    break;
                }
            }
            path_parts.reverse();
            let relative_dir = path_parts.join("/");
            let display_name = if path_parts.len() > 1 {
                let immediate_folder = path_parts.last().unwrap();
                format!("[{}] {}", immediate_folder, clean_filename)
            } else if path_parts.len() == 1 {
                let root_folder = &path_parts[0];
                format!("[{}] {}", root_folder, clean_filename)
            } else {
                clean_filename.clone()
            };

            let relative_path = if relative_dir.is_empty() {
                clean_filename.clone()
            } else {
                format!("{}/{}", relative_dir, clean_filename)
            };

            result_files.push(MegaFileItem {
                handle,
                name: display_name,
                size,
                relative_path,
                key: file_key,
                initial_counter,
                folder_id: Some(folder_id.to_string()),
            });
        }

        Ok(result_files)
    }

    /// Resolve metadata and decryption parameters for a single public MEGA file
    pub async fn resolve_single_file(file_id: &str, file_key_b64: &str) -> Result<MegaFileItem> {
        let key_bytes = Self::decode_mega_base64(file_key_b64)
            .ok_or_else(|| RapidError::Other("Invalid MEGA file decryption key".to_string()))?;
        if key_bytes.len() != 32 {
            return Err(RapidError::Other(format!(
                "Invalid MEGA file key length: {} (expected 32 bytes)",
                key_bytes.len()
            )));
        }

        let mut ints = [0u32; 8];
        for i in 0..8 {
            ints[i] = u32::from_be_bytes([
                key_bytes[i * 4],
                key_bytes[i * 4 + 1],
                key_bytes[i * 4 + 2],
                key_bytes[i * 4 + 3],
            ]);
        }

        let file_key_ints = [
            ints[0] ^ ints[4],
            ints[1] ^ ints[5],
            ints[2] ^ ints[6],
            ints[3] ^ ints[7],
        ];
        let mut file_key = [0u8; 16];
        for i in 0..4 {
            let b = file_key_ints[i].to_be_bytes();
            file_key[i * 4..i * 4 + 4].copy_from_slice(&b);
        }

        let mut iv_bytes = [0u8; 16];
        let k4_b = ints[4].to_be_bytes();
        let k5_b = ints[5].to_be_bytes();
        iv_bytes[0..4].copy_from_slice(&k4_b);
        iv_bytes[4..8].copy_from_slice(&k5_b);
        let initial_counter = u128::from_be_bytes(iv_bytes);

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(RapidError::Network)?;

        let api_url = "https://g.api.mega.co.nz/cs?id=1".to_string();
        let payload = serde_json::json!([{"a": "g", "g": 1, "p": file_id}]);
        let payload_str = serde_json::to_string(&payload).unwrap_or_default();

        let resp = client
            .post(&api_url)
            .header("Content-Type", "application/json")
            .body(payload_str)
            .send()
            .await
            .map_err(RapidError::Network)?;

        let body_text = resp.text().await.map_err(RapidError::Network)?;
        let resp_json: Vec<MegaDownloadUrlResponse> = serde_json::from_str(&body_text).map_err(|e| {
            RapidError::Other(format!("Failed to parse MEGA download response: {}", e))
        })?;

        let first = resp_json
            .into_iter()
            .next()
            .ok_or_else(|| RapidError::Other("Empty response from MEGA API".to_string()))?;

        let size = first.s.unwrap_or(0);
        let filename = first
            .at
            .as_deref()
            .and_then(|at| Self::decrypt_attributes(&file_key, at))
            .unwrap_or_else(|| format!("{}.bin", file_id));

        Ok(MegaFileItem {
            handle: file_id.to_string(),
            name: filename.clone(),
            size,
            relative_path: filename,
            key: file_key,
            initial_counter,
            folder_id: None,
        })
    }


    /// Resolve download CDN URL for a file
    pub async fn resolve_download_url(
        client: &Client,
        file_handle: &str,
        folder_id: Option<&str>,
    ) -> Result<String> {
        let (api_url, payload) = if let Some(fid) = folder_id {
            (
                format!("https://g.api.mega.co.nz/cs?id=1&n={}", fid),
                serde_json::json!([{"a": "g", "g": 1, "n": file_handle}]),
            )
        } else {
            (
                "https://g.api.mega.co.nz/cs?id=1".to_string(),
                serde_json::json!([{"a": "g", "g": 1, "p": file_handle}]),
            )
        };

        let payload_str = serde_json::to_string(&payload).unwrap_or_default();
        let resp = client
            .post(&api_url)
            .header("Content-Type", "application/json")
            .body(payload_str)
            .send()
            .await
            .map_err(RapidError::Network)?;

        if !resp.status().is_success() {
            return Err(RapidError::Other(format!(
                "MEGA API get download URL failed: HTTP {}",
                resp.status()
            )));
        }

        let body_text = resp.text().await.map_err(RapidError::Network)?;

        if let Ok(err_codes) = serde_json::from_str::<Vec<i64>>(&body_text) {
            if let Some(&code) = err_codes.first() {
                if code < 0 {
                    let err_msg = match code {
                        -3 => "MEGA server temporary congestion (EAGAIN), retrying...".to_string(),
                        -16 => "MEGA file blocked by owner (EBLOCKED)".to_string(),
                        -17 => "MEGA free bandwidth transfer quota exceeded (EOVERQUOTA). Please wait a few minutes or retry.".to_string(),
                        -18 => "MEGA file temporarily unavailable (ETEMPUNAVAIL), retrying...".to_string(),
                        -9 => "MEGA file does not exist or was deleted (ENOENT)".to_string(),
                        other => format!("MEGA API error code: {}", other),
                    };
                    return Err(RapidError::Other(err_msg));
                }
            }
        }

        let resp_json: Vec<MegaDownloadUrlResponse> = serde_json::from_str(&body_text).map_err(|e| {
            RapidError::Other(format!("Failed to parse MEGA download URL response: {}", e))
        })?;

        let cdn_url = resp_json
            .into_iter()
            .next()
            .and_then(|r| r.g)
            .ok_or_else(|| RapidError::Other("MEGA did not return a valid download CDN URL".to_string()))?;

        Ok(cdn_url)
    }

    /// Decrypt in-place a chunk of ciphertext using AES-128-CTR with given byte_offset
    pub fn decrypt_aes128_ctr_chunk(
        key: &[u8; 16],
        initial_counter: u128,
        byte_offset: u64,
        ciphertext: &mut [u8],
    ) {
        if ciphertext.is_empty() {
            return;
        }
        let cipher = Aes128::new(GenericArray::from_slice(key));
        let start_block = (byte_offset / 16) as u128;
        let mut counter = initial_counter.wrapping_add(start_block);
        let byte_in_block = (byte_offset % 16) as usize;

        let mut pos = 0;
        if byte_in_block != 0 {
            let mut block = GenericArray::clone_from_slice(&counter.to_be_bytes());
            cipher.encrypt_block(&mut block);
            let take = (16 - byte_in_block).min(ciphertext.len());
            for i in 0..take {
                ciphertext[pos + i] ^= block[byte_in_block + i];
            }
            pos += take;
            counter = counter.wrapping_add(1);
        }

        while pos + 16 <= ciphertext.len() {
            let mut block = GenericArray::clone_from_slice(&counter.to_be_bytes());
            cipher.encrypt_block(&mut block);
            for i in 0..16 {
                ciphertext[pos + i] ^= block[i];
            }
            pos += 16;
            counter = counter.wrapping_add(1);
        }

        if pos < ciphertext.len() {
            let mut block = GenericArray::clone_from_slice(&counter.to_be_bytes());
            cipher.encrypt_block(&mut block);
            let rem = ciphertext.len() - pos;
            for i in 0..rem {
                ciphertext[pos + i] ^= block[i];
            }
        }
    }
}

pub struct MegaDownloader {
    pub task_id: String,
    pub file_handle: String,
    pub filename: String,
    pub target_file: PathBuf,
    pub total_size: u64,
    pub key: [u8; 16],
    pub initial_counter: u128,
    pub folder_id: Option<String>,
    pub speed_limit: Option<Arc<AtomicU64>>,
    pub cancel_token: CancellationToken,
}

impl MegaDownloader {
    pub fn new(
        task_id: String,
        file_handle: String,
        filename: String,
        target_file: PathBuf,
        total_size: u64,
        key: [u8; 16],
        initial_counter: u128,
        folder_id: Option<String>,
        speed_limit: Option<Arc<AtomicU64>>,
        cancel_token: CancellationToken,
    ) -> Self {
        Self {
            task_id,
            file_handle,
            filename,
            target_file,
            total_size,
            key,
            initial_counter,
            folder_id,
            speed_limit,
            cancel_token,
        }
    }

    pub async fn run(
        &self,
        progress_tx: broadcast::Sender<DownloadProgress>,
    ) -> Result<()> {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(20))
            .read_timeout(Duration::from_secs(35))
            .build()
            .map_err(RapidError::Network)?;

        // Ensure parent folder exists
        if let Some(parent) = self.target_file.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Check if file already exists for resuming
        let mut downloaded_bytes: u64 = 0;
        if self.target_file.exists() {
            if let Ok(meta) = fs::metadata(&self.target_file).await {
                if meta.len() == self.total_size && self.total_size > 0 {
                    // Already completely downloaded
                    let _ = progress_tx.send(DownloadProgress {
                        id: self.task_id.clone(),
                        filename: self.filename.clone(),
                        downloaded_bytes: self.total_size,
                        total_bytes: Some(self.total_size),
                        progress_percent: 100.0,
                        speed_bps: 0,
                        eta_seconds: None,
                        status: DownloadStatus::Completed,
                        segments: vec![Segment::new(0, 0, self.total_size)],
                    });
                    return Ok(());
                } else if meta.len() < self.total_size {
                    // Check if file is corrupt (e.g. earlier failed HTML download or all zeros)
                    let is_corrupt = if meta.len() <= 4096 && self.total_size > 4096 {
                        true
                    } else if let Ok(first_bytes) = fs::read(&self.target_file).await {
                        first_bytes.starts_with(b"<!DOCTYPE")
                            || first_bytes.starts_with(b"<html")
                            || (first_bytes.len() >= 64 && first_bytes.iter().take(64).all(|&b| b == 0))
                    } else {
                        false
                    };

                    if is_corrupt {
                        let _ = fs::remove_file(&self.target_file).await;
                        downloaded_bytes = 0;
                    } else {
                        downloaded_bytes = meta.len();
                    }
                } else {
                    // File is larger than expected, start fresh
                    let _ = fs::remove_file(&self.target_file).await;
                    downloaded_bytes = 0;
                }
            }
        }

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(downloaded_bytes > 0)
            .truncate(downloaded_bytes == 0)
            .open(&self.target_file)
            .await?;

        let mut speed_calc_bytes: u64 = 0;
        let mut last_speed_check = Instant::now();

        let mut ui_segment = Segment::new(0, 0, self.total_size);
        ui_segment.downloaded_bytes = downloaded_bytes;

        let initial_progress_pct = if self.total_size > 0 {
            ((downloaded_bytes as f64 / self.total_size as f64) * 100.0) as f32
        } else {
            0.0
        };
        let _ = progress_tx.send(DownloadProgress {
            id: self.task_id.clone(),
            filename: self.filename.clone(),
            downloaded_bytes,
            total_bytes: Some(self.total_size),
            progress_percent: initial_progress_pct,
            speed_bps: 0,
            eta_seconds: None,
            status: DownloadStatus::Downloading,
            segments: vec![ui_segment.clone()],
        });

        let mut retry_count = 0;
        let max_retries = 10;

        while downloaded_bytes < self.total_size {
            if self.cancel_token.is_cancelled() {
                return Err(RapidError::Cancelled);
            }

            // Fetch or refresh CDN direct download link
            let cdn_url = match MegaResolver::resolve_download_url(
                &client,
                &self.file_handle,
                self.folder_id.as_deref(),
            ).await {
                Ok(u) => u,
                Err(e) => {
                    retry_count += 1;
                    if retry_count > max_retries {
                        return Err(e);
                    }
                    sleep(Duration::from_millis(1500 * retry_count as u64)).await;
                    continue;
                }
            };

            let mut req = client.get(&cdn_url);
            if downloaded_bytes > 0 {
                req = req.header("Range", format!("bytes={}-", downloaded_bytes));
            }

            let resp = match req.send().await {
                Ok(r) if r.status().is_success() || r.status() == reqwest::StatusCode::PARTIAL_CONTENT => r,
                Ok(r) => {
                    retry_count += 1;
                    if retry_count > max_retries {
                        return Err(RapidError::Other(format!("MEGA CDN server returned HTTP {}", r.status())));
                    }
                    sleep(Duration::from_millis(1500 * retry_count as u64)).await;
                    continue;
                }
                Err(e) => {
                    retry_count += 1;
                    if retry_count > max_retries {
                        return Err(RapidError::Network(e));
                    }
                    sleep(Duration::from_millis(1500 * retry_count as u64)).await;
                    continue;
                }
            };

            let mut stream = resp.bytes_stream();
            let mut stream_disconnected = false;

            while let Some(chunk_res) = stream.next().await {
                if self.cancel_token.is_cancelled() {
                    return Err(RapidError::Cancelled);
                }

                let mut chunk = match chunk_res {
                    Ok(b) => b.to_vec(),
                    Err(e) => {
                        eprintln!("[Rapid] MEGA stream drop for '{}' at byte {}/{} ({}). Reconnecting...",
                            self.filename, downloaded_bytes, self.total_size, e);
                        stream_disconnected = true;
                        break;
                    }
                };

                let chunk_len = chunk.len() as u64;

                // Decrypt in-place with AES-128-CTR
                MegaResolver::decrypt_aes128_ctr_chunk(
                    &self.key,
                    self.initial_counter,
                    downloaded_bytes,
                    &mut chunk,
                );

                // Write decrypted payload
                file.write_all(&chunk).await?;

                downloaded_bytes += chunk_len;
                speed_calc_bytes += chunk_len;
                ui_segment.downloaded_bytes = downloaded_bytes;
                retry_count = 0; // Successfully received data: reset retry count!

                // Speed limiter
                if let Some(ref limiter) = self.speed_limit {
                    let limit_bps = limiter.load(Ordering::Relaxed);
                    if limit_bps > 0 {
                        let delay_ms = (chunk_len * 1000) / limit_bps;
                        if delay_ms > 0 {
                            sleep(Duration::from_millis(delay_ms.min(500))).await;
                        }
                    }
                }

                // Progress tracking
                let now = Instant::now();
                let elapsed = now.duration_since(last_speed_check).as_millis();
                if elapsed >= 300 {
                    let current_speed_bps = ((speed_calc_bytes as f64 / (elapsed as f64 / 1000.0)) as u64).max(0);
                    speed_calc_bytes = 0;
                    last_speed_check = now;

                    let progress_percent = if self.total_size > 0 {
                        ((downloaded_bytes as f64 / self.total_size as f64) * 100.0) as f32
                    } else {
                        0.0
                    };

                    let remaining_bytes = self.total_size.saturating_sub(downloaded_bytes);
                    let eta_seconds = if current_speed_bps > 0 {
                        Some(remaining_bytes / current_speed_bps)
                    } else {
                        None
                    };

                    let _ = progress_tx.send(DownloadProgress {
                        id: self.task_id.clone(),
                        filename: self.filename.clone(),
                        downloaded_bytes,
                        total_bytes: Some(self.total_size),
                        progress_percent,
                        speed_bps: current_speed_bps,
                        eta_seconds,
                        status: DownloadStatus::Downloading,
                        segments: vec![ui_segment.clone()],
                    });
                }
            }

            file.flush().await?;

            if stream_disconnected {
                retry_count += 1;
                if retry_count > max_retries {
                    return Err(RapidError::Other("MEGA download connection dropped repeatedly. Please retry.".to_string()));
                }
                sleep(Duration::from_millis(1500 * retry_count.min(4) as u64)).await;
            } else if downloaded_bytes >= self.total_size {
                break;
            }
        }

        file.flush().await?;

        // Final completion event
        ui_segment.is_complete = true;
        ui_segment.downloaded_bytes = downloaded_bytes;
        let _ = progress_tx.send(DownloadProgress {
            id: self.task_id.clone(),
            filename: self.filename.clone(),
            downloaded_bytes,
            total_bytes: Some(self.total_size),
            progress_percent: 100.0,
            speed_bps: 0,
            eta_seconds: None,
            status: DownloadStatus::Completed,
            segments: vec![ui_segment],
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_crawl_mega_folder() {
        let files = MegaResolver::crawl_folder("QRYjwAxD", "l6hQIYbks_lFhf_QGG5Onw").await.unwrap();
        println!("Crawled MEGA files count: {}", files.len());
        assert!(files.len() >= 60);
        for f in files.iter().take(5) {
            println!("File: {} | Path: {} | Size: {} bytes", f.name, f.relative_path, f.size);
        }
    }

    #[tokio::test]
    async fn test_download_mega_file() {
        let files = MegaResolver::crawl_folder("QRYjwAxD", "l6hQIYbks_lFhf_QGG5Onw").await.unwrap();
        let f0 = &files[0];
        let temp_dir = std::env::temp_dir().join("rapid_mega_test");
        let target_file = temp_dir.join(&f0.name);
        let (tx, _rx) = tokio::sync::broadcast::channel(10);
        let downloader = MegaDownloader::new(
            "test_task".to_string(),
            f0.handle.clone(),
            f0.name.clone(),
            target_file.clone(),
            f0.size,
            f0.key,
            f0.initial_counter,
            f0.folder_id.clone(),
            None,
            CancellationToken::new(),
        );
        downloader.run(tx).await.unwrap();
        let content = std::fs::read_to_string(&target_file).unwrap();
        println!("Decrypted content: {}", content.chars().take(30).collect::<String>());
        assert!(content.contains("কোরআন") || content.contains("কোর্স"));
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}


