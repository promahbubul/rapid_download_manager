#![windows_subsystem = "windows"]

use chrono::Timelike;
use serde::{Deserialize, Serialize};


use chrono::Utc;
use eframe::egui::{self, Color32, Margin, RichText, Stroke, Vec2};
use rapid_core::{DownloadConfig, DownloadStatus, DownloadTask};
use std::path::PathBuf;
use std::collections::{HashSet, VecDeque};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

#[cfg(windows)]
mod tray;
mod bengali;
use bengali::shape_bengali;

// =========================================================================
// GLASSMORPHISM MASTER PALETTE (Futuristic • Elegant • Premium)
// =========================================================================
const GLASS_PRIMARY: Color32 = Color32::from_rgb(139, 92, 246);        // Electric Violet (#8B5CF6)
const GLASS_PRIMARY_HOVER: Color32 = Color32::from_rgb(167, 139, 250);  // Light Violet Glow (#A78BFA)
const GLASS_SECONDARY: Color32 = Color32::from_rgb(6, 182, 212);       // Neon Cyan / Electric Aqua (#06B6D4)
const GLASS_SECONDARY_HOVER: Color32 = Color32::from_rgb(34, 211, 238); // Light Cyan Glow (#22D3EE)
const GLASS_BG: Color32 = Color32::from_rgb(2, 6, 23);                 // Deep Midnight Slate-950 (#020617)
const GLASS_SURFACE: Color32 = Color32::from_rgb(11, 18, 36);          // Translucent Dark Glass Surface
const GLASS_CARD: Color32 = Color32::from_rgb(14, 23, 46);             // Frosted Glass Card Fill
const GLASS_BORDER: Color32 = Color32::from_rgb(32, 45, 74);           // Glass Rim Border
const GLASS_TEXT: Color32 = Color32::from_rgb(249, 250, 251);          // Crisp Bright White (#F9FAFB)
const GLASS_MUTED: Color32 = Color32::from_rgb(156, 163, 175);         // Cool Slate Muted (#9CA3AF)
const _GLASS_ACCENT_BG: Color32 = Color32::from_rgb(28, 20, 58);       // Violet Aura Pill Fill
const _GLASS_ACCENT_BORDER: Color32 = Color32::from_rgb(124, 58, 237); // Glowing Violet Rim

// Backward-compatible aliases for seamless rendering
const PINK_NEON: Color32 = GLASS_PRIMARY;                              // Electric Violet (#8B5CF6)
const PINK_ROSE: Color32 = GLASS_SECONDARY;                            // Neon Cyan (#06B6D4)
const PINK_PASTEL: Color32 = GLASS_TEXT;                               // Crisp White (#F9FAFB)
const PINK_MUTED: Color32 = GLASS_MUTED;                               // Cool Slate Muted (#9CA3AF)
const _PINK_ACCENT_BG: Color32 = _GLASS_ACCENT_BG;                     // Violet Aura Pill Fill
const _PINK_ACCENT_BORDER: Color32 = _GLASS_ACCENT_BORDER;             // Glowing Violet Rim

const VELVET_BLACK: Color32 = GLASS_BG;                                // Deep Midnight (#020617)
const VELVET_SURFACE: Color32 = GLASS_SURFACE;                         // Glass Surface (#0B1224)
const _VELVET_CARD: Color32 = GLASS_CARD;                              // Frosted Card (#0E172E)
const VELVET_BORDER: Color32 = GLASS_BORDER;                           // Glass Rim Border (#202D4A)

const STATUS_DOWNLOADING: Color32 = GLASS_SECONDARY;                   // Neon Cyan (#06B6D4)
const STATUS_COMPLETED: Color32 = Color32::from_rgb(16, 185, 129);     // Emerald Mint (#10B981)
const STATUS_PAUSED: Color32 = Color32::from_rgb(245, 158, 11);        // Warm Amber (#F59E0B)
const STATUS_FAILED: Color32 = Color32::from_rgb(244, 63, 94);         // Crimson Rose (#F43F5E)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FilterCategory {
    All,
    Active,
    Paused,
    Completed,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModernIcon {
    Globe,
    Close,
    Play,
    Pause,
    Refresh,
    Trash,
    Folder,
    ExternalFile,
    Plus,
    Search,
    MinimizeTray,
    Check,
    PulseBeacon,
    SpeedGauge,
    WinMinimize,
    WinMaximize,
    WinRestore,
    WinClose,
    Github,
}

fn draw_modern_icon(painter: &egui::Painter, icon: ModernIcon, rect: egui::Rect, color: Color32) {
    let center = rect.center();
    let stroke = Stroke::new(1.5_f32, color);
    match icon {
        ModernIcon::Github => {
            painter.circle_stroke(center, 5.2, stroke);
            // Left cat ear
            painter.line_segment([egui::pos2(center.x - 3.8, center.y - 3.5), egui::pos2(center.x - 2.5, center.y - 6.2)], stroke);
            painter.line_segment([egui::pos2(center.x - 2.5, center.y - 6.2), egui::pos2(center.x - 1.2, center.y - 5.0)], stroke);
            // Right cat ear
            painter.line_segment([egui::pos2(center.x + 1.2, center.y - 5.0), egui::pos2(center.x + 2.5, center.y - 6.2)], stroke);
            painter.line_segment([egui::pos2(center.x + 2.5, center.y - 6.2), egui::pos2(center.x + 3.8, center.y - 3.5)], stroke);
            // Branch/stem
            painter.line_segment([egui::pos2(center.x, center.y - 1.0), egui::pos2(center.x, center.y + 4.2)], Stroke::new(1.3_f32, color));
            painter.circle_filled(egui::pos2(center.x - 2.0, center.y + 1.5), 1.0, color);
            painter.circle_filled(egui::pos2(center.x + 2.0, center.y + 1.5), 1.0, color);
        }
        ModernIcon::Globe => {
            painter.circle_stroke(center, 5.0, stroke);
            painter.line_segment([egui::pos2(center.x - 5.0, center.y), egui::pos2(center.x + 5.0, center.y)], stroke);
            painter.line_segment([egui::pos2(center.x, center.y - 5.0), egui::pos2(center.x, center.y + 5.0)], stroke);
        }
        ModernIcon::Close => {
            painter.line_segment(
                [egui::pos2(center.x - 5.0, center.y - 5.0), egui::pos2(center.x + 5.0, center.y + 5.0)],
                Stroke::new(2.0_f32, color),
            );
            painter.line_segment(
                [egui::pos2(center.x + 5.0, center.y - 5.0), egui::pos2(center.x - 5.0, center.y + 5.0)],
                Stroke::new(2.0_f32, color),
            );
        }
        ModernIcon::Play => {
            let half_h = 5.0_f32;
            let p1 = egui::pos2(center.x - 3.5, center.y - half_h);
            let p2 = egui::pos2(center.x - 3.5, center.y + half_h);
            let p3 = egui::pos2(center.x + 4.5, center.y);
            painter.add(egui::Shape::convex_polygon(
                vec![p1, p2, p3],
                color,
                Stroke::NONE,
            ));
        }
        ModernIcon::Pause => {
            let bar_w = 2.5_f32;
            let bar_h = 10.0_f32;
            let gap = 2.5_f32;
            let b1 = egui::Rect::from_center_size(egui::pos2(center.x - gap, center.y), egui::vec2(bar_w, bar_h));
            let b2 = egui::Rect::from_center_size(egui::pos2(center.x + gap, center.y), egui::vec2(bar_w, bar_h));
            painter.rect_filled(b1, 1.0, color);
            painter.rect_filled(b2, 1.0, color);
        }
        ModernIcon::Refresh => {
            let radius = 4.8_f32;
            let n = 10;
            for i in 0..n {
                let a1 = std::f32::consts::PI * 0.25 + (i as f32) * (std::f32::consts::PI * 1.5 / n as f32);
                let a2 = std::f32::consts::PI * 0.25 + ((i + 1) as f32) * (std::f32::consts::PI * 1.5 / n as f32);
                let pt1 = egui::pos2(center.x + radius * a1.cos(), center.y + radius * a1.sin());
                let pt2 = egui::pos2(center.x + radius * a2.cos(), center.y + radius * a2.sin());
                painter.line_segment([pt1, pt2], stroke);
            }
            let end_a = std::f32::consts::PI * 1.75;
            let tip = egui::pos2(center.x + radius * end_a.cos(), center.y + radius * end_a.sin());
            painter.line_segment([tip, egui::pos2(tip.x + 2.5, tip.y - 0.5)], stroke);
            painter.line_segment([tip, egui::pos2(tip.x - 0.5, tip.y - 2.5)], stroke);
        }
        ModernIcon::Trash => {
            let w = 9.0_f32;
            let h = 10.0_f32;
            painter.line_segment(
                [egui::pos2(center.x - w * 0.6, center.y - h * 0.4), egui::pos2(center.x + w * 0.6, center.y - h * 0.4)],
                Stroke::new(1.4_f32, color),
            );
            painter.line_segment(
                [egui::pos2(center.x - 2.0, center.y - h * 0.4), egui::pos2(center.x - 2.0, center.y - h * 0.6)],
                Stroke::new(1.2_f32, color),
            );
            painter.line_segment(
                [egui::pos2(center.x - 2.0, center.y - h * 0.6), egui::pos2(center.x + 2.0, center.y - h * 0.6)],
                Stroke::new(1.2_f32, color),
            );
            painter.line_segment(
                [egui::pos2(center.x + 2.0, center.y - h * 0.6), egui::pos2(center.x + 2.0, center.y - h * 0.4)],
                Stroke::new(1.2_f32, color),
            );
            let body_rect = egui::Rect::from_min_max(
                egui::pos2(center.x - w * 0.45, center.y - h * 0.25),
                egui::pos2(center.x + w * 0.45, center.y + h * 0.55),
            );
            painter.rect_stroke(body_rect, 1.5, Stroke::new(1.2_f32, color));
            painter.line_segment(
                [egui::pos2(center.x - 1.5, center.y - h * 0.1), egui::pos2(center.x - 1.5, center.y + h * 0.4)],
                Stroke::new(1.0_f32, color),
            );
            painter.line_segment(
                [egui::pos2(center.x + 1.5, center.y - h * 0.1), egui::pos2(center.x + 1.5, center.y + h * 0.4)],
                Stroke::new(1.0_f32, color),
            );
        }
        ModernIcon::Folder => {
            let w = 11.0_f32;
            let h = 8.5_f32;
            let top_left = egui::pos2(center.x - w * 0.5, center.y - h * 0.5);
            let tab_right = egui::pos2(center.x - w * 0.1, center.y - h * 0.5);
            let tab_down = egui::pos2(center.x + w * 0.05, center.y - h * 0.2);
            let top_right = egui::pos2(center.x + w * 0.5, center.y - h * 0.2);
            let bot_right = egui::pos2(center.x + w * 0.5, center.y + h * 0.5);
            let bot_left = egui::pos2(center.x - w * 0.5, center.y + h * 0.5);

            painter.line_segment([top_left, tab_right], stroke);
            painter.line_segment([tab_right, tab_down], stroke);
            painter.line_segment([tab_down, top_right], stroke);
            painter.line_segment([top_right, bot_right], stroke);
            painter.line_segment([bot_right, bot_left], stroke);
            painter.line_segment([bot_left, top_left], stroke);
        }
        ModernIcon::ExternalFile => {
            let w = 8.0_f32;
            let h = 10.0_f32;
            let doc_rect = egui::Rect::from_min_max(
                egui::pos2(center.x - w * 0.5, center.y - h * 0.5),
                egui::pos2(center.x + w * 0.5, center.y + h * 0.5),
            );
            painter.rect_stroke(doc_rect, 1.0, stroke);
            painter.line_segment(
                [egui::pos2(center.x + w * 0.1, center.y - h * 0.5), egui::pos2(center.x + w * 0.5, center.y - h * 0.1)],
                stroke,
            );
        }
        ModernIcon::Plus => {
            let len = (rect.width().min(rect.height()) * 0.22).max(4.5);
            let stroke_w = (rect.width().min(rect.height()) * 0.045).max(1.8);
            painter.line_segment(
                [egui::pos2(center.x - len, center.y), egui::pos2(center.x + len, center.y)],
                Stroke::new(stroke_w, color),
            );
            painter.line_segment(
                [egui::pos2(center.x, center.y - len), egui::pos2(center.x, center.y + len)],
                Stroke::new(stroke_w, color),
            );
        }
        ModernIcon::Search => {
            let r = 3.5_f32;
            let c = egui::pos2(center.x - 1.5, center.y - 1.5);
            painter.circle_stroke(c, r, Stroke::new(1.4_f32, color));
            painter.line_segment(
                [egui::pos2(c.x + 2.5, c.y + 2.5), egui::pos2(c.x + 6.0, c.y + 6.0)],
                Stroke::new(1.6_f32, color),
            );
        }
        ModernIcon::MinimizeTray => {
            painter.line_segment(
                [egui::pos2(center.x - 4.5, center.y - 1.0), egui::pos2(center.x, center.y + 3.0)],
                Stroke::new(1.6_f32, color),
            );
            painter.line_segment(
                [egui::pos2(center.x, center.y + 3.0), egui::pos2(center.x + 4.5, center.y - 1.0)],
                Stroke::new(1.6_f32, color),
            );
        }
        ModernIcon::Check => {
            painter.line_segment(
                [egui::pos2(center.x - 4.0, center.y), egui::pos2(center.x - 1.0, center.y + 3.5)],
                Stroke::new(1.8_f32, color),
            );
            painter.line_segment(
                [egui::pos2(center.x - 1.0, center.y + 3.5), egui::pos2(center.x + 4.5, center.y - 3.5)],
                Stroke::new(1.8_f32, color),
            );
        }
        ModernIcon::PulseBeacon => {
            painter.circle_filled(center, 3.5, color);
        }
        ModernIcon::SpeedGauge => {
            painter.line_segment(
                [egui::pos2(center.x + 1.0, center.y - 5.0), egui::pos2(center.x - 3.0, center.y)],
                Stroke::new(1.6_f32, color),
            );
            painter.line_segment(
                [egui::pos2(center.x - 3.0, center.y), egui::pos2(center.x + 1.0, center.y)],
                Stroke::new(1.6_f32, color),
            );
            painter.line_segment(
                [egui::pos2(center.x + 1.0, center.y), egui::pos2(center.x - 1.0, center.y + 5.0)],
                Stroke::new(1.6_f32, color),
            );
        }
        ModernIcon::WinMinimize => {
            let stroke = Stroke::new(1.2_f32, color);
            painter.line_segment(
                [egui::pos2(center.x - 5.0, center.y + 0.5), egui::pos2(center.x + 5.0, center.y + 0.5)],
                stroke,
            );
        }
        ModernIcon::WinMaximize => {
            let box_rect = egui::Rect::from_center_size(center, Vec2::new(10.0, 10.0));
            painter.rect_stroke(box_rect, 0.0, Stroke::new(1.2_f32, color));
        }
        ModernIcon::WinRestore => {
            let stroke = Stroke::new(1.2_f32, color);
            // Back window (top-right): upper and right borders
            painter.line_segment(
                [egui::pos2(center.x - 2.0, center.y - 5.0), egui::pos2(center.x + 5.0, center.y - 5.0)],
                stroke,
            );
            painter.line_segment(
                [egui::pos2(center.x + 5.0, center.y - 5.0), egui::pos2(center.x + 5.0, center.y + 2.0)],
                stroke,
            );
            // Back window small bottom and left snippets
            painter.line_segment(
                [egui::pos2(center.x + 2.0, center.y + 2.0), egui::pos2(center.x + 5.0, center.y + 2.0)],
                stroke,
            );
            painter.line_segment(
                [egui::pos2(center.x - 2.0, center.y - 5.0), egui::pos2(center.x - 2.0, center.y - 2.0)],
                stroke,
            );
            // Front window (bottom-left)
            let front_rect = egui::Rect::from_min_max(
                egui::pos2(center.x - 5.0, center.y - 2.0),
                egui::pos2(center.x + 2.0, center.y + 5.0),
            );
            painter.rect_filled(front_rect, 0.0, GLASS_BG);
            painter.rect_stroke(front_rect, 0.0, stroke);
        }
        ModernIcon::WinClose => {
            let d = 4.5_f32;
            let stroke = Stroke::new(1.3_f32, color);
            painter.line_segment(
                [egui::pos2(center.x - d, center.y - d), egui::pos2(center.x + d, center.y + d)],
                stroke,
            );
            painter.line_segment(
                [egui::pos2(center.x + d, center.y - d), egui::pos2(center.x - d, center.y + d)],
                stroke,
            );
        }
    }
}

fn window_caption_button(
    ui: &mut egui::Ui,
    icon: ModernIcon,
    size: Vec2,
    default_color: Color32,
    hover_icon_color: Color32,
    hover_bg_color: Color32,
    tooltip: &str,
) -> bool {
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let is_hovered = response.hovered();

    if is_hovered {
        ui.painter().rect_filled(rect, 0.0, hover_bg_color);
    }

    let icon_color = if is_hovered {
        hover_icon_color
    } else {
        default_color
    };

    draw_modern_icon(ui.painter(), icon, rect, icon_color);

    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text(tooltip)
        .clicked()
}

fn modern_icon_button(
    ui: &mut egui::Ui,
    icon: ModernIcon,
    size: Vec2,
    default_color: Color32,
    hover_color: Color32,
    tooltip: &str,
) -> bool {
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let is_hovered = response.hovered();

    let bg_color = if is_hovered {
        Color32::from_rgb(26, 36, 62)
    } else {
        GLASS_CARD
    };

    let border_stroke = if is_hovered {
        Stroke::new(1.2_f32, GLASS_SECONDARY)
    } else {
        Stroke::new(1.0_f32, GLASS_BORDER)
    };

    let icon_color = if is_hovered {
        hover_color
    } else {
        default_color
    };

    ui.painter().rect(rect, 5.0, bg_color, border_stroke);
    draw_modern_icon(ui.painter(), icon, rect, icon_color);

    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text(tooltip)
        .clicked()
}

fn render_file_type_badge(ui: &mut egui::Ui, ext: &str) {
    let (tag, color, bg) = match ext {
        "exe" => ("EXE", Color32::from_rgb(16, 185, 129), Color32::from_rgb(16, 44, 38)),          // Crisp Emerald
        "msi" => ("MSI", Color32::from_rgb(20, 184, 166), Color32::from_rgb(16, 42, 42)),          // Teal
        "iso" | "img" => ("ISO", Color32::from_rgb(245, 158, 11), Color32::from_rgb(46, 32, 16)), // Amber
        "apk" => ("APK", Color32::from_rgb(132, 204, 22), Color32::from_rgb(30, 42, 16)),          // Lime
        "pdf" | "doc" | "docx" | "txt" => ("DOC", GLASS_PRIMARY, Color32::from_rgb(30, 22, 58)),
        "mp4" | "mkv" | "avi" | "mov" | "webm" => ("VID", GLASS_SECONDARY, Color32::from_rgb(12, 36, 48)),
        "zip" | "rar" | "7z" | "tar" | "gz" => ("ZIP", Color32::from_rgb(168, 85, 247), Color32::from_rgb(32, 20, 52)),
        "mp3" | "wav" | "flac" | "aac" => ("AUD", Color32::from_rgb(56, 189, 248), Color32::from_rgb(14, 34, 52)),
        "jpg" | "png" | "gif" | "webp" | "svg" => ("IMG", Color32::from_rgb(236, 72, 153), Color32::from_rgb(46, 20, 38)),
        "bin" => ("BIN", Color32::from_rgb(99, 102, 241), Color32::from_rgb(24, 26, 56)),
        _ => ("FILE", GLASS_MUTED, Color32::from_rgb(18, 26, 42)),
    };

    egui::Frame::none()
        .fill(bg)
        .stroke(Stroke::new(1.0_f32, color))
        .rounding(egui::Rounding::same(3.5))
        .inner_margin(Margin::symmetric(4.5, 2.0))
        .show(ui, |ui| {
            ui.label(RichText::new(tag).size(9.5).color(color).strong());
        });
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerConfig {
    pub enabled: bool,
    pub start_hour: u32,
    pub start_minute: u32,
    pub stop_hour: u32,
    pub stop_minute: u32,
    pub auto_shutdown: bool,
    #[serde(skip)]
    pub has_triggered_start: bool,
    #[serde(skip)]
    pub has_triggered_stop: bool,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            start_hour: 23,
            start_minute: 0,
            stop_hour: 6,
            stop_minute: 0,
            auto_shutdown: false,
            has_triggered_start: false,
            has_triggered_stop: false,
        }
    }
}

fn get_scheduler_file_path() -> PathBuf {
    let appdata_file = rapid_core::AppPaths::scheduler_file();
    let old_file = dirs_or_fallback().join(".rapid_scheduler.json");
    if old_file.exists() && !appdata_file.exists() {
        let _ = std::fs::copy(&old_file, &appdata_file);
        let _ = std::fs::remove_file(&old_file);
    }
    appdata_file
}

fn load_scheduler_config() -> SchedulerConfig {
    let path = get_scheduler_file_path();
    rapid_core::StorageManager::load_json_with_backup::<SchedulerConfig>(&path).unwrap_or_default()
}

fn save_scheduler_config(config: &SchedulerConfig) {
    let path = get_scheduler_file_path();
    let _ = rapid_core::StorageManager::atomic_write_json(&path, config);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HistoryRecord {
    pub id: String,
    pub filename: String,
    pub url: String,
    pub target_file: PathBuf,
    #[serde(default)]
    pub total_bytes: Option<u64>,
    #[serde(default = "chrono::Utc::now")]
    pub completed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub category: Option<String>,
}

fn get_history_file_path() -> PathBuf {
    let appdata_file = rapid_core::AppPaths::history_file();
    let old_file = dirs_or_fallback().join(".rapid_history.json");
    if old_file.exists() && !appdata_file.exists() {
        let _ = std::fs::copy(&old_file, &appdata_file);
        let _ = std::fs::remove_file(&old_file);
    }
    appdata_file
}

fn load_history() -> Vec<HistoryRecord> {
    let path = get_history_file_path();
    rapid_core::StorageManager::load_json_with_backup::<Vec<HistoryRecord>>(&path).unwrap_or_default()
}

fn save_task_to_history(item: &ActiveTaskUI) {
    let path = get_history_file_path();
    let mut records = load_history();
    if let Some(existing) = records.iter_mut().find(|r| r.id == item.id || r.target_file == item.target_file) {
        existing.total_bytes = item.total_bytes;
        existing.completed_at = chrono::Utc::now();
    } else {
        records.push(HistoryRecord {
            id: item.id.clone(),
            filename: item.filename.clone(),
            url: item.url.clone(),
            target_file: item.target_file.clone(),
            total_bytes: item.total_bytes,
            completed_at: chrono::Utc::now(),
            category: None,
        });
    }
    let _ = rapid_core::StorageManager::atomic_write_json(&path, &records);
}

fn remove_from_history(id: &str) {
    let path = get_history_file_path();
    let mut records = load_history();
    records.retain(|r| r.id != id);
    let _ = rapid_core::StorageManager::atomic_write_json(&path, &records);
}

fn clear_all_history() {
    let path = get_history_file_path();
    let _ = std::fs::remove_file(&path);
}

struct ActiveTaskUI {
    id: String,
    filename: String,
    url: String,
    target_file: PathBuf,
    total_bytes: Option<u64>,
    downloaded_bytes: u64,
    progress_percent: f32,
    speed_bps: u64,
    eta_seconds: Option<u64>,
    status: DownloadStatus,
    segments: Vec<rapid_core::Segment>,
    task_handle: Option<Arc<DownloadTask>>,
    num_segments: usize,
    is_resuming: bool,
    cookies: Option<String>,
    referrer: Option<String>,
    user_agent: Option<String>,
    is_youtube: bool,
    quality: Option<rapid_core::youtube::DownloadQuality>,
    yt_cancel_token: Option<tokio_util::sync::CancellationToken>,
}

#[derive(Clone, Debug)]
struct PendingBrowserDownload {
    url: String,
    filename: String,
    dest_dir: String,
    segments: usize,
    cookies: Option<String>,
    user_agent: Option<String>,
    referrer: Option<String>,
    is_gdrive: bool,
    is_youtube: bool,
    quality: rapid_core::youtube::DownloadQuality,
}

fn extract_filename_from_url(url_str: &str) -> String {
    let trimmed = url_str.trim();
    if trimmed.is_empty() || rapid_core::youtube::YoutubeResolver::is_extractable_platform(trimmed) {
        return String::new();
    }
    let base = trimmed.split('?').next().unwrap_or(trimmed);
    let base = base.split('#').next().unwrap_or(base);
    let trimmed_base = base.trim_end_matches('/');
    if let Some(pos) = trimmed_base.rfind('/') {
        let segment = &trimmed_base[pos + 1..];
        if !segment.is_empty() {
            if let Ok(decoded) = urlencoding::decode(segment) {
                let clean = decoded.trim();
                if !clean.is_empty() && clean != "/" {
                    return clean.to_string();
                }
            }
            return segment.to_string();
        }
    }
    String::new()
}

pub fn categorize_filename(filename: &str) -> &'static str {
    let ext = std::path::Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "exe" | "msi" | "apk" | "bat" | "cmd" | "app" | "dmg" | "deb" | "rpm" => "Programs",
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" | "iso" | "img" | "cab" => "Compressed",
        "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" | "3gp" | "ts" => "Videos",
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" | "wma" | "mid" | "opus" => "Music",
        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "rtf" | "csv" | "epub" => "Documents",
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg" | "bmp" | "ico" | "tiff" => "Images",
        _ => "",
    }
}

pub fn get_categorized_destination(base_dir: &std::path::Path, filename: &str) -> PathBuf {
    let cat = categorize_filename(filename);
    if cat.is_empty() {
        base_dir.to_path_buf()
    } else {
        if base_dir.file_name().and_then(|f| f.to_str()) == Some(cat) {
            base_dir.to_path_buf()
        } else {
            base_dir.join(cat)
        }
    }
}

#[derive(Clone, Debug)]
pub enum UpdateStatus {
    Idle,
    Checking,
    UpToDate { checked_at: String },
    Available {
        version: String,
        download_url: String,
        release_notes: String,
        published_at: String,
    },
    Error(String),
}

fn parse_ver_triplet(v: &str) -> (u32, u32, u32) {
    let clean = v.trim_start_matches('v').split('-').next().unwrap_or("");
    let mut parts = clean.split('.').filter_map(|s| s.parse::<u32>().ok());
    (parts.next().unwrap_or(0), parts.next().unwrap_or(0), parts.next().unwrap_or(0))
}

fn is_remote_version_newer(remote: &str, current: &str) -> bool {
    parse_ver_triplet(remote) > parse_ver_triplet(current)
}

struct RapidApp {
    tokio_rt: Arc<Runtime>,
    tasks: Arc<Mutex<Vec<ActiveTaskUI>>>,
    selected_task_index: Option<usize>,
    selected_filter: FilterCategory,
    search_query: String,

    // Splash Screen State
    splash_start: Instant,
    splash_duration: Duration,
    splash_logo: egui::TextureHandle,
    titlebar_logo: egui::TextureHandle,

    // Add Download Dialog State
    show_add_dialog: bool,
    input_url: String,
    input_filename: String,
    input_dest: String,
    input_segments: usize,
    input_quality: rapid_core::youtube::DownloadQuality,
    base_download_dir: PathBuf,
    max_concurrent_downloads: usize,
    add_error: Option<String>,
    is_probing: bool,
    selected_tasks: HashSet<String>,
    show_delete_modal: bool,
    show_scheduler_modal: bool,
    scheduler: SchedulerConfig,
    tasks_to_delete: HashSet<String>,
    has_requested_initial_focus: bool,
    show_browser_integration_modal: bool,
    pub show_about_modal: bool,
    pub update_status: Arc<std::sync::Mutex<UpdateStatus>>,
    browser_integration_msg: Option<(String, bool)>,
    pub speed_limit_bps: Arc<AtomicU64>,
    pub selected_speed_limit_idx: usize,

    // Browser Extension Incoming Downloads
    pending_browser_queue: Arc<std::sync::Mutex<VecDeque<PendingBrowserDownload>>>,
    current_browser_prompt: Option<PendingBrowserDownload>,
    filename_resolver_rx: std::sync::mpsc::Receiver<(String, String)>,
    filename_resolver_tx: std::sync::mpsc::Sender<(String, String)>,

    #[cfg(windows)]
    tray_handle: Option<tray::TrayHandle>,
}

static CACHED_GOOGLE_COOKIES: std::sync::RwLock<Option<String>> = std::sync::RwLock::new(None);

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
        .unwrap_or_else(|| std::path::PathBuf::from("extension"));

    if !ext_dir.exists() {
        let alt = std::path::PathBuf::from(r"D:\mahbub\project\rapid_download_manager\extension");
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

impl RapidApp {
    fn check_for_updates(&self) {
        let status_arc = Arc::clone(&self.update_status);
        if let Ok(mut s) = status_arc.lock() {
            *s = UpdateStatus::Checking;
        }
        self.tokio_rt.spawn(async move {
            let client = match reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(8))
                .user_agent("RapidDownloadManager-UpdateChecker/1.0")
                .build()
            {
                Ok(c) => c,
                Err(e) => {
                    if let Ok(mut s) = status_arc.lock() {
                        *s = UpdateStatus::Error(format!("Failed to build client: {}", e));
                    }
                    return;
                }
            };

            let res = client
                .get("https://api.github.com/repos/promahbubul/rapid_download_manager/releases/latest")
                .send()
                .await;

            match res {
                Ok(resp) => {
                    if resp.status().is_success() {
                        if let Ok(text) = resp.text().await {
                            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                                let tag = json.get("tag_name")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .trim_start_matches('v')
                                    .to_string();
                                let html_url = json.get("html_url")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("https://github.com/promahbubul/rapid_download_manager/releases")
                                    .to_string();
                                let body = json.get("body")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("Release improvements and bug fixes.")
                                    .to_string();
                                let published = json.get("published_at")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();

                                let current = env!("CARGO_PKG_VERSION");
                                let newer = is_remote_version_newer(&tag, current);

                                if let Ok(mut s) = status_arc.lock() {
                                    if newer {
                                        *s = UpdateStatus::Available {
                                            version: tag,
                                            download_url: html_url,
                                            release_notes: body,
                                            published_at: published,
                                        };
                                    } else {
                                        *s = UpdateStatus::UpToDate {
                                            checked_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                                        };
                                    }
                                }
                                return;
                            }
                        }
                    }
                    if let Ok(mut s) = status_arc.lock() {
                        *s = UpdateStatus::UpToDate {
                            checked_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        };
                    }
                }
                Err(e) => {
                    if let Ok(mut s) = status_arc.lock() {
                        *s = UpdateStatus::Error(format!("Network connection error: {}", e));
                    }
                }
            }
        });
    }

    fn open_add_download_dialog(&mut self, ctx: &egui::Context) {
        self.show_add_dialog = true;
        self.add_error = None;
        self.input_url.clear();
        self.input_filename.clear();
        self.input_quality = rapid_core::youtube::DownloadQuality::Best;
        self.input_dest = self.base_download_dir.to_string_lossy().to_string();

        // Auto-check system clipboard for URL
        if let Ok(mut clipboard) = arboard::Clipboard::new() {
            if let Ok(text) = clipboard.get_text() {
                let trimmed = text.trim();
                if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
                    self.input_url = trimmed.to_string();
                    if rapid_core::youtube::YoutubeResolver::is_extractable_platform(&self.input_url) {
                        self.input_filename = "Resolving video title...".to_string();
                        let cat_dest = get_categorized_destination(&self.base_download_dir, "video.mp4");
                        self.input_dest = cat_dest.to_string_lossy().to_string();
                    } else {
                        let candidate = extract_filename_from_url(&self.input_url);
                        if !candidate.is_empty() && !rapid_core::engine::is_generic_placeholder(&candidate) {
                            self.input_filename = candidate.clone();
                            let cat_dest = get_categorized_destination(&self.base_download_dir, &candidate);
                            self.input_dest = cat_dest.to_string_lossy().to_string();
                        }
                    }
                    self.probe_url_filename(trimmed, ctx);
                }
            }
        }
    }

    fn probe_url_filename(&self, url: &str, ctx: &egui::Context) {
        let tx = self.filename_resolver_tx.clone();
        let url_clone = url.to_string();
        let ctx_clone = ctx.clone();
        let is_gd = url.contains("drive.google.com")
            || url.contains("googleusercontent.com")
            || url.contains("docs.google.com");

        self.tokio_rt.spawn(async move {
            let cb = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(12))
                .redirect(reqwest::redirect::Policy::limited(10));

            if let Ok(client) = cb.build() {
                let detected_name = if is_gd {
                    let res_type = rapid_core::gdrive::GDriveResolver::parse_resource_type(&url_clone);
                    if let rapid_core::gdrive::GDriveResourceType::File(file_id) = res_type {
                        rapid_core::gdrive::GDriveResolver::resolve_file_download_url(&client, &file_id)
                            .await
                            .ok()
                            .map(|r| r.name)
                    } else {
                        None
                    }
                } else if rapid_core::youtube::YoutubeResolver::is_extractable_platform(&url_clone) {
                    let canonical = rapid_core::youtube::YoutubeResolver::canonicalize_url(&url_clone, None);
                    rapid_core::youtube::YoutubeResolver::resolve_metadata(&canonical)
                        .await
                        .ok()
                        .map(|m| m.clean_filename)
                } else {
                    rapid_core::Probe::inspect(&client, &url_clone)
                        .await
                        .ok()
                        .map(|m| m.filename)
                };

                if let Some(fname) = detected_name {
                    let clean = fname.trim();
                    if !clean.is_empty() && !rapid_core::engine::is_generic_placeholder(clean) {
                        let _ = tx.send((url_clone, clean.to_string()));
                        ctx_clone.request_repaint();
                    }
                }
            }
        });
    }

    fn is_task_visible(&self, task: &ActiveTaskUI) -> bool {
        let cat_ok = match self.selected_filter {
            FilterCategory::All => true,
            FilterCategory::Active => matches!(task.status, DownloadStatus::Downloading | DownloadStatus::Queued),
            FilterCategory::Paused => matches!(task.status, DownloadStatus::Paused | DownloadStatus::Failed(_)),
            FilterCategory::Completed => task.status == DownloadStatus::Completed,
        };
        if !cat_ok {
            return false;
        }
        if !self.search_query.trim().is_empty() {
            let q = self.search_query.trim().to_lowercase();
            return task.filename.to_lowercase().contains(&q) || task.url.to_lowercase().contains(&q);
        }
        true
    }

    fn new(cc: &eframe::CreationContext<'_>, rt: Arc<Runtime>) -> Self {
        // Master Color Palette: Glassmorphism Dark Theme (Futuristic • Elegant • Premium)
        let mut visuals = egui::Visuals::dark();
        visuals.window_rounding = egui::Rounding::same(8.0);
        visuals.panel_fill = GLASS_SURFACE;                          // Translucent Glass Surface (#0B1224)
        visuals.faint_bg_color = Color32::from_rgb(14, 22, 44);      // Subtle alternate glass row
        visuals.extreme_bg_color = GLASS_BG;                         // Deep Midnight Groove (#020617)
        visuals.widgets.noninteractive.bg_fill = GLASS_SURFACE;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, GLASS_BORDER);
        visuals.selection.bg_fill = Color32::from_rgb(30, 24, 64);   // Translucent Violet Aura
        visuals.selection.stroke = Stroke::new(1.5_f32, GLASS_PRIMARY); // 1.5px glowing Electric Violet border
        cc.egui_ctx.set_visuals(visuals);

        // Unicode Bangla Font Configuration (Noto Sans Bengali)
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "noto_bengali".to_owned(),
            egui::FontData::from_static(include_bytes!("../../../assets/fonts/kalpurush-pua.ttf")),
        );
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .push("noto_bengali".to_owned());
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("noto_bengali".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        let default_download_dir = dirs_or_fallback();

        let mut initial_tasks = Vec::new();
        let mut dirs_to_scan = vec![default_download_dir.clone()];
        for cat in &["Programs", "Compressed", "Videos", "Music", "Documents", "Images"] {
            dirs_to_scan.push(default_download_dir.join(cat));
        }

        for scan_dir in dirs_to_scan {
            if let Ok(entries) = std::fs::read_dir(&scan_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|e| e.to_str()) == Some("rapid") {
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            if let Ok(mut state) = serde_json::from_str::<rapid_core::DownloadTaskState>(&content) {
                                let downloaded_bytes: u64 = state.segments.iter().map(|s| s.downloaded_bytes).sum();
                                let total = state.total_bytes.unwrap_or(0);
                                let progress_percent = if total > 0 { (downloaded_bytes as f32 / total as f32) * 100.0 } else { 0.0 };
                                
                                if state.status == rapid_core::DownloadStatus::Downloading {
                                    state.status = rapid_core::DownloadStatus::Paused;
                                }
                                
                                let is_yt = rapid_core::youtube::YoutubeResolver::is_extractable_platform(&state.url);
                                let ui = ActiveTaskUI {
                                    id: state.id,
                                    filename: state.target_file.file_name().unwrap_or_default().to_string_lossy().into_owned(),
                                    url: state.url,
                                    target_file: state.target_file,
                                    total_bytes: state.total_bytes,
                                    downloaded_bytes,
                                    progress_percent,
                                    speed_bps: 0,
                                    eta_seconds: None,
                                    status: state.status,
                                    num_segments: state.segments.len(),
                                    segments: state.segments,
                                    task_handle: None,
                                    is_resuming: false,
                                    cookies: state.cookies,
                                    referrer: state.referrer,
                                    user_agent: state.user_agent,
                                    is_youtube: is_yt,
                                    quality: None,
                                    yt_cancel_token: None,
                                };
                                initial_tasks.push(ui);
                            }
                        }
                    }
                }
            }
        }
        
        // Load persistent history records before wrapping initial_tasks in Mutex
        let history_records = load_history();
        for record in history_records {
            if !initial_tasks.iter().any(|t| t.id == record.id || t.target_file == record.target_file) {
                let is_yt = rapid_core::youtube::YoutubeResolver::is_extractable_platform(&record.url);
                initial_tasks.push(ActiveTaskUI {
                    id: record.id,
                    filename: record.filename,
                    url: record.url,
                    target_file: record.target_file,
                    total_bytes: record.total_bytes,
                    downloaded_bytes: record.total_bytes.unwrap_or(0),
                    progress_percent: 100.0,
                    speed_bps: 0,
                    eta_seconds: None,
                    status: DownloadStatus::Completed,
                    segments: Vec::new(),
                    task_handle: None,
                    num_segments: 1,
                    is_resuming: false,
                    cookies: None,
                    referrer: None,
                    user_agent: None,
                    is_youtube: is_yt,
                    quality: None,
                    yt_cancel_token: None,
                });
            }
        }

        let tasks_arc = Arc::new(tokio::sync::Mutex::new(initial_tasks));

        let pending_browser_queue = Arc::new(std::sync::Mutex::new(VecDeque::new()));
        let (filename_resolver_tx, filename_resolver_rx) = std::sync::mpsc::channel();

        // Start local browser extension receiver on 127.0.0.1:9669
        let pending_for_server = Arc::clone(&pending_browser_queue);
        let dest_for_server = default_download_dir.clone();
        let ctx_for_server = cc.egui_ctx.clone();
        let tx_for_server = filename_resolver_tx.clone();
        let tasks_for_server = Arc::clone(&tasks_arc);

        rt.spawn(async move {
            run_extension_server(pending_for_server, dest_for_server, ctx_for_server, tx_for_server, tasks_for_server).await;
        });

        #[cfg(windows)]
        let tray_handle = Some(tray::TrayHandle::new());

        let splash_logo = cc.egui_ctx.load_texture(
            "rapid_splash_logo",
            egui::ColorImage::from_rgba_unmultiplied(
                [934, 398],
                include_bytes!("../assets/master_logo.raw"),
            ),
            egui::TextureOptions::LINEAR,
        );

        let titlebar_logo = cc.egui_ctx.load_texture(
            "rapid_titlebar_logo",
            egui::ColorImage::from_rgba_unmultiplied(
                [1204, 240],
                include_bytes!("../assets/titlebar_logo.raw"),
            ),
            egui::TextureOptions::LINEAR,
        );

        Self {
            tokio_rt: rt,
            tasks: tasks_arc,
            selected_task_index: None,
            selected_filter: FilterCategory::All,
            search_query: String::new(),
            splash_start: Instant::now(),
            splash_duration: Duration::from_millis(2400),
            splash_logo,
            titlebar_logo,
            show_add_dialog: false,
            input_url: String::new(),
            input_filename: String::new(),
            input_dest: default_download_dir.to_string_lossy().to_string(),
            input_segments: 8,
            input_quality: rapid_core::youtube::DownloadQuality::Best,
            base_download_dir: default_download_dir.clone(),
            max_concurrent_downloads: 3,
            add_error: None,
            is_probing: false,
            selected_tasks: HashSet::new(),
            show_delete_modal: false,
            show_scheduler_modal: false,
            scheduler: load_scheduler_config(),
            tasks_to_delete: HashSet::new(),
            has_requested_initial_focus: false,
            show_browser_integration_modal: false,
            show_about_modal: false,
            update_status: Arc::new(std::sync::Mutex::new(UpdateStatus::Idle)),
            browser_integration_msg: None,
            speed_limit_bps: Arc::new(AtomicU64::new(0)),
            selected_speed_limit_idx: 0,
            pending_browser_queue,
            current_browser_prompt: None,
            filename_resolver_rx,
            filename_resolver_tx,
            #[cfg(windows)]
            tray_handle,
        }
    }

    fn pause_all(&mut self) {
        if let Ok(mut list) = self.tasks.try_lock() {
            for task in list.iter_mut() {
                if task.status == DownloadStatus::Downloading {
                    if let Some(ref handle) = task.task_handle {
                        handle.pause();
                    }
                    task.status = DownloadStatus::Paused;
                    task.speed_bps = 0;
                }
            }
        }
    }

    fn resume_all(&mut self) {
        let indices_to_resume: Vec<usize> = {
            if let Ok(list) = self.tasks.try_lock() {
                list.iter()
                    .enumerate()
                    .filter(|(_, t)| matches!(t.status, DownloadStatus::Paused | DownloadStatus::Failed(_)))
                    .map(|(i, _)| i)
                    .collect()
            } else {
                Vec::new()
            }
        };

        for idx in indices_to_resume {
            self.resume_download(idx);
        }
    }

    fn start_new_download(
        &mut self,
        url: String,
        dest_dir: PathBuf,
        segments: usize,
        custom_filename: Option<String>,
        quality: Option<rapid_core::youtube::DownloadQuality>,
    ) {
        if url.trim().starts_with("blob:") {
            self.add_error = Some("Invalid URL: 'blob:' URLs are internal browser memory objects. Please play the video in your browser so Rapid captures the real stream, or use a direct HTTP/HTTPS link.".to_string());
            return;
        }

        let res_type = rapid_core::GDriveResolver::parse_resource_type(&url);
        if let rapid_core::GDriveResourceType::Folder(folder_id) = res_type {
            let rt_crawl = Arc::clone(&self.tokio_rt);
            let tasks_crawl = Arc::clone(&self.tasks);
            let dest_crawl = dest_dir.clone();
            let rt_inner = Arc::clone(&rt_crawl);
            rt_crawl.spawn(async move {
                if let Ok(files) = rapid_core::GDriveResolver::crawl_folder_default(&folder_id, 2).await {
                    for file in files {
                        let file_url = format!("https://drive.google.com/file/d/{}/view", file.id);
                        spawn_download_task(
                            file_url,
                            dest_crawl.clone(),
                            1,
                            Some(file.name),
                            None,
                            None,
                            None,
                            true,
                            false,
                            None,
                            None,
                            Arc::clone(&rt_inner),
                            Arc::clone(&tasks_crawl),
                        );
                    }
                }
            });
            self.show_add_dialog = false;
            self.input_url.clear();
            self.input_filename.clear();
            self.input_quality = rapid_core::youtube::DownloadQuality::Best;
            return;
        }

        let rt = Arc::clone(&self.tokio_rt);
        let tasks_arc = Arc::clone(&self.tasks);
        self.is_probing = true;
        self.add_error = None;

        let is_vp = rapid_core::youtube::YoutubeDownloader::is_extractable_platform(&url);
        spawn_download_task(
            url,
            dest_dir,
            segments,
            custom_filename,
            None,
            None,
            None,
            false,
            is_vp,
            Some(Arc::clone(&self.speed_limit_bps)),
            quality,
            rt,
            tasks_arc,
        );

        self.is_probing = false;
        self.show_add_dialog = false;
        self.input_url.clear();
        self.input_filename.clear();
        self.input_quality = rapid_core::youtube::DownloadQuality::Best;
    }

    fn resume_download(&mut self, task_index: usize) {
        let rt = Arc::clone(&self.tokio_rt);
        let tasks_arc = Arc::clone(&self.tasks);

        let (task_id, url, target_file, filename, segments_count, cookies, referrer, user_agent, is_yt, quality) = {
            let Ok(mut list) = self.tasks.try_lock() else { return; };
            if task_index >= list.len() { return; }
            let task = &mut list[task_index];
            if task.status == DownloadStatus::Downloading {
                return;
            }
            task.status = DownloadStatus::Downloading;
            task.speed_bps = 0;
            (
                task.id.clone(),
                task.url.clone(),
                task.target_file.clone(),
                task.filename.clone(),
                task.num_segments,
                task.cookies.clone(),
                task.referrer.clone(),
                task.user_agent.clone(),
                task.is_youtube || rapid_core::youtube::YoutubeResolver::is_extractable_platform(&task.url),
                task.quality,
            )
        };

        if is_yt {
            let speed_limit_for_yt = Arc::clone(&self.speed_limit_bps);
            rt.spawn(async move {
                let cancel_token = tokio_util::sync::CancellationToken::new();
                {
                    let mut list = tasks_arc.lock().await;
                    if let Some(item) = list.iter_mut().find(|t| t.id == task_id) {
                        item.status = DownloadStatus::Downloading;
                        item.yt_cancel_token = Some(cancel_token.clone());
                    }
                }

                let selected_quality = quality.unwrap_or_else(|| {
                    if filename.ends_with(".mp3") {
                        rapid_core::youtube::DownloadQuality::AudioMp3
                    } else if filename.ends_with(".m4a") {
                        rapid_core::youtube::DownloadQuality::AudioM4a
                    } else {
                        rapid_core::youtube::DownloadQuality::Best
                    }
                });

                let (progress_tx, mut rx) = tokio::sync::broadcast::channel::<rapid_core::DownloadProgress>(100);

                let downloader = rapid_core::youtube::YoutubeDownloader::new(
                    task_id.clone(),
                    url.clone(),
                    filename.clone(),
                    target_file.clone(),
                    selected_quality,
                    Some(speed_limit_for_yt),
                    cancel_token,
                    cookies,
                    user_agent,
                    referrer,
                );

                let tasks_for_progress = Arc::clone(&tasks_arc);
                let task_id_for_progress = task_id.clone();

                tokio::spawn(async move {
                    while let Ok(prog) = rx.recv().await {
                        let mut list = tasks_for_progress.lock().await;
                        if let Some(item) = list.iter_mut().find(|t| t.id == task_id_for_progress) {
                            item.downloaded_bytes = prog.downloaded_bytes;
                            item.progress_percent = prog.progress_percent;
                            item.speed_bps = prog.speed_bps;
                            item.eta_seconds = prog.eta_seconds;
                            item.status = prog.status.clone();
                            item.segments = prog.segments;
                            if let Some(tot) = prog.total_bytes {
                                item.total_bytes = Some(tot);
                            }
                        }
                    }
                });

                let res = downloader.run(progress_tx).await;
                let mut list = tasks_arc.lock().await;
                if let Some(item) = list.iter_mut().find(|t| t.id == task_id) {
                    match res {
                        Ok(_) => {
                            item.status = DownloadStatus::Completed;
                            item.progress_percent = 100.0;
                            item.speed_bps = 0;
                            item.eta_seconds = None;
                            save_task_to_history(item);
                        }
                        Err(e) => {
                            if !matches!(e, rapid_core::RapidError::Cancelled) {
                                eprintln!("[Rapid] Resume YouTube failed for '{}': {}", filename, e);
                                item.status = DownloadStatus::Failed(e.to_string());
                            }
                            item.speed_bps = 0;
                            item.eta_seconds = None;
                        }
                    }
                }
            });
            return;
        }

        let dest_dir = target_file
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));

        let is_gd = url.contains("drive.google.com")
            || url.contains("googleusercontent.com")
            || url.contains("usercontent.google.com")
            || url.contains("docs.google.com")
            || url.contains("takeout-download-drive");

        let speed_limit_arc = Arc::clone(&self.speed_limit_bps);
        rt.spawn(async move {
            let config = DownloadConfig {
                url: url.clone(),
                output_dir: dest_dir,
                custom_filename: Some(filename.clone()),
                num_segments: if is_gd { 1 } else { segments_count },
                cookies,
                referrer,
                user_agent,
                is_gdrive: is_gd,
                speed_limit: Some(speed_limit_arc),
            };

            match DownloadTask::create(task_id.clone(), config).await {
                Ok(task) => {
                    let task_arc = Arc::new(task);
                    let mut rx = task_arc.subscribe();

                    {
                        let mut list = tasks_arc.lock().await;
                        if let Some(item) = list.iter_mut().find(|t| t.id == task_id) {
                            item.task_handle = Some(Arc::clone(&task_arc));
                            item.status = DownloadStatus::Downloading;
                            item.is_resuming = false;
                        }
                    }

                    let tasks_for_progress = Arc::clone(&tasks_arc);
                    let task_id_for_progress = task_id.clone();

                    tokio::spawn(async move {
                        while let Ok(prog) = rx.recv().await {
                            let mut list = tasks_for_progress.lock().await;
                            if let Some(item) = list.iter_mut().find(|t| t.id == task_id_for_progress) {
                                item.downloaded_bytes = prog.downloaded_bytes;
                                item.progress_percent = prog.progress_percent;
                                item.speed_bps = prog.speed_bps;
                                item.eta_seconds = prog.eta_seconds;
                                item.status = prog.status.clone();
                                item.segments = prog.segments;
                            }
                        }
                    });

                    let res = task_arc.run().await;
                    if let Err(e) = res {
                        eprintln!("[Rapid] Resume failed for '{}': {}", filename, e);
                        let mut list = tasks_arc.lock().await;
                        if let Some(item) = list.iter_mut().find(|t| t.id == task_id) {
                            if item.status == DownloadStatus::Downloading {
                                item.status = DownloadStatus::Failed(e.to_string());
                                item.speed_bps = 0;
                                item.eta_seconds = None;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[Rapid] Failed to create task for resume '{}': {}", filename, e);
                    let mut list = tasks_arc.lock().await;
                    if let Some(item) = list.iter_mut().find(|t| t.id == task_id) {
                        item.status = DownloadStatus::Failed(e.to_string());
                        item.speed_bps = 0;
                        item.eta_seconds = None;
                    }
                }
            }
        });
    }

    fn redownload(&mut self, task_index: usize) {
        let rt = Arc::clone(&self.tokio_rt);
        let tasks_arc = Arc::clone(&self.tasks);

        let (task_id, url, target_file, filename, segments_count, cookies, referrer, user_agent, is_yt, quality) = {
            let Ok(mut list) = self.tasks.try_lock() else { return; };
            if task_index >= list.len() { return; }
            let task = &mut list[task_index];

            if let Some(ref handle) = task.task_handle {
                handle.pause();
            }
            if let Some(ref cancel) = task.yt_cancel_token {
                cancel.cancel();
            }

            task.downloaded_bytes = 0;
            task.progress_percent = 0.0;
            task.speed_bps = 0;
            task.eta_seconds = None;
            task.status = DownloadStatus::Downloading;
            task.segments.clear();

            (
                task.id.clone(),
                task.url.clone(),
                task.target_file.clone(),
                task.filename.clone(),
                task.num_segments,
                task.cookies.clone(),
                task.referrer.clone(),
                task.user_agent.clone(),
                task.is_youtube || rapid_core::youtube::YoutubeResolver::is_extractable_platform(&task.url),
                task.quality,
            )
        };

        if is_yt {
            let speed_limit_for_yt = Arc::clone(&self.speed_limit_bps);
            rt.spawn(async move {
                let _ = tokio::fs::remove_file(&target_file).await;
                if let Some(stem) = target_file.file_stem().and_then(|s| s.to_str()) {
                    if let Some(parent) = target_file.parent() {
                        if let Ok(mut entries) = tokio::fs::read_dir(parent).await {
                            while let Ok(Some(entry)) = entries.next_entry().await {
                                let path = entry.path();
                                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                                    if name.starts_with(stem) && (name.ends_with(".part") || name.ends_with(".ytdl") || name.contains(".f")) {
                                        let _ = tokio::fs::remove_file(&path).await;
                                    }
                                }
                            }
                        }
                    }
                }

                let cancel_token = tokio_util::sync::CancellationToken::new();
                {
                    let mut list = tasks_arc.lock().await;
                    if let Some(item) = list.iter_mut().find(|t| t.id == task_id) {
                        item.status = DownloadStatus::Downloading;
                        item.downloaded_bytes = 0;
                        item.progress_percent = 0.0;
                        item.speed_bps = 0;
                        item.eta_seconds = None;
                        item.yt_cancel_token = Some(cancel_token.clone());
                    }
                }

                let selected_quality = quality.unwrap_or_else(|| {
                    if filename.ends_with(".mp3") {
                        rapid_core::youtube::DownloadQuality::AudioMp3
                    } else if filename.ends_with(".m4a") {
                        rapid_core::youtube::DownloadQuality::AudioM4a
                    } else {
                        rapid_core::youtube::DownloadQuality::Best
                    }
                });

                let (progress_tx, mut rx) = tokio::sync::broadcast::channel::<rapid_core::DownloadProgress>(100);

                let downloader = rapid_core::youtube::YoutubeDownloader::new(
                    task_id.clone(),
                    url.clone(),
                    filename.clone(),
                    target_file.clone(),
                    selected_quality,
                    Some(speed_limit_for_yt),
                    cancel_token,
                    cookies,
                    user_agent,
                    referrer,
                );

                let tasks_for_progress = Arc::clone(&tasks_arc);
                let task_id_for_progress = task_id.clone();

                tokio::spawn(async move {
                    while let Ok(prog) = rx.recv().await {
                        let mut list = tasks_for_progress.lock().await;
                        if let Some(item) = list.iter_mut().find(|t| t.id == task_id_for_progress) {
                            item.downloaded_bytes = prog.downloaded_bytes;
                            item.progress_percent = prog.progress_percent;
                            item.speed_bps = prog.speed_bps;
                            item.eta_seconds = prog.eta_seconds;
                            item.status = prog.status.clone();
                            item.segments = prog.segments;
                            if let Some(tot) = prog.total_bytes {
                                item.total_bytes = Some(tot);
                            }
                        }
                    }
                });

                let res = downloader.run(progress_tx).await;
                let mut list = tasks_arc.lock().await;
                if let Some(item) = list.iter_mut().find(|t| t.id == task_id) {
                    match res {
                        Ok(_) => {
                            item.status = DownloadStatus::Completed;
                            item.progress_percent = 100.0;
                            item.speed_bps = 0;
                            item.eta_seconds = None;
                            save_task_to_history(item);
                        }
                        Err(e) => {
                            if !matches!(e, rapid_core::RapidError::Cancelled) {
                                eprintln!("[Rapid] Redownload YouTube failed for '{}': {}", filename, e);
                                item.status = DownloadStatus::Failed(e.to_string());
                            }
                            item.speed_bps = 0;
                            item.eta_seconds = None;
                        }
                    }
                }
            });
            return;
        }

        let dest_dir = target_file
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));

        let is_gd = url.contains("drive.google.com")
            || url.contains("googleusercontent.com")
            || url.contains("usercontent.google.com")
            || url.contains("docs.google.com")
            || url.contains("takeout-download-drive");

        let effective_cookies = if is_gd {
            load_google_cookies().or(cookies)
        } else {
            cookies
        };

        let speed_limit_arc = Arc::clone(&self.speed_limit_bps);
        rt.spawn(async move {
            let _ = tokio::fs::remove_file(&target_file).await;
            let manifest = rapid_core::DownloadTaskState::manifest_path(&target_file);
            let _ = tokio::fs::remove_file(&manifest).await;

            let config = DownloadConfig {
                url: url.clone(),
                output_dir: dest_dir,
                custom_filename: Some(filename.clone()),
                num_segments: if is_gd { 1 } else { segments_count },
                cookies: effective_cookies,
                referrer,
                user_agent,
                is_gdrive: is_gd,
                speed_limit: Some(speed_limit_arc),
            };

            match DownloadTask::create(task_id.clone(), config).await {
                Ok(task) => {
                    let task_arc = Arc::new(task);
                    let mut rx = task_arc.subscribe();

                    let segs = task_arc.snapshot_segments().await;
                    {
                        let mut list = tasks_arc.lock().await;
                        if let Some(item) = list.iter_mut().find(|t| t.id == task_id) {
                            item.task_handle = Some(Arc::clone(&task_arc));
                            item.status = DownloadStatus::Downloading;
                            item.total_bytes = task_arc.total_bytes;
                            item.segments = segs;
                        }
                    }

                    let tasks_for_progress = Arc::clone(&tasks_arc);
                    let task_id_for_progress = task_id.clone();

                    tokio::spawn(async move {
                        while let Ok(prog) = rx.recv().await {
                            let mut list = tasks_for_progress.lock().await;
                            if let Some(item) = list.iter_mut().find(|t| t.id == task_id_for_progress) {
                                item.downloaded_bytes = prog.downloaded_bytes;
                                item.progress_percent = prog.progress_percent;
                                item.speed_bps = prog.speed_bps;
                                item.eta_seconds = prog.eta_seconds;
                                item.status = prog.status.clone();
                                item.segments = prog.segments;
                            }
                        }
                    });

                    let res = task_arc.run().await;
                    if let Err(e) = res {
                        eprintln!("[Rapid] Redownload failed for '{}': {}", filename, e);
                        let mut list = tasks_arc.lock().await;
                        if let Some(item) = list.iter_mut().find(|t| t.id == task_id) {
                            if item.status == DownloadStatus::Downloading {
                                item.status = DownloadStatus::Failed(e.to_string());
                                item.speed_bps = 0;
                                item.eta_seconds = None;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[Rapid] Failed to recreate task for redownload '{}': {}", filename, e);
                    let mut list = tasks_arc.lock().await;
                    if let Some(item) = list.iter_mut().find(|t| t.id == task_id) {
                        item.status = DownloadStatus::Failed(e.to_string());
                        item.speed_bps = 0;
                        item.eta_seconds = None;
                    }
                }
            }
        });
    }
    fn pause_download(&mut self, task_index: usize) {
        if let Ok(mut list) = self.tasks.try_lock() {
            if let Some(task) = list.get_mut(task_index) {
                if task.status == DownloadStatus::Downloading {
                    if let Some(ref handle) = task.task_handle {
                        handle.pause();
                    }
                    if let Some(ref cancel) = task.yt_cancel_token {
                        cancel.cancel();
                    }
                    task.status = DownloadStatus::Paused;
                    task.speed_bps = 0;
                    task.eta_seconds = None;
                }
            }
        }
    }

    #[allow(dead_code)]
    fn remove_download(&mut self, task_index: usize) {
        if let Ok(mut list) = self.tasks.try_lock() {
            if task_index < list.len() {
                let task = &list[task_index];
                if let Some(ref handle) = task.task_handle {
                    handle.pause();
                }
                list.remove(task_index);
                if let Some(sel) = self.selected_task_index {
                    if sel >= list.len() {
                        self.selected_task_index = if list.is_empty() { None } else { Some(list.len() - 1) };
                    }
                }
            }
        }
    }

    fn open_download_file(&self, task_index: usize) {
        if let Ok(list) = self.tasks.try_lock() {
            if let Some(task) = list.get(task_index) {
                if task.target_file.exists() {
                    let _ = open::that(&task.target_file);
                }
            }
        }
    }

    fn open_download_folder(&self, task_index: usize) {
        if let Ok(list) = self.tasks.try_lock() {
            if let Some(task) = list.get(task_index) {
                if let Some(parent) = task.target_file.parent() {
                    let _ = open::that(parent);
                }
            }
        }
    }

    // Render Animated Splash Screen on startup (Centered, Smooth Fade & Ambient Backlight)
    // Render Splash Screen on startup: Pure, clean, minimalist center-center logo
    // Render Animated Splash Screen on startup: Centered, Animated Logo, Stages & Glowing Progress Bar
    // Render Animated Splash Screen on startup: Centered, Animated Logo, Stages & Glowing Progress Bar
    fn render_splash_screen(&self, ctx: &egui::Context, elapsed: Duration) {
        let time = elapsed.as_secs_f32();
        let total_dur = 2.4_f32;
        let progress = (time / total_dur).clamp(0.0, 1.0);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(VELVET_BLACK))
            .show(ctx, |ui| {
                // Dimensions of authentic master logo (aspect ratio: 2.3467)
                let base_w = 440.0_f32;
                let base_h = (base_w / 2.3467_f32).round();

                // Animated Micro-Scale & Smooth Opacity Entrance
                let t_entrance = (time / 0.45).clamp(0.0, 1.0);
                let ease_out = 1.0 - (1.0 - t_entrance).powi(3);
                let scale = 0.96 + 0.04 * ease_out;
                let logo_w = base_w * scale;
                let logo_h = base_h * scale;

                // Total splash block height for absolute Center-Center alignment
                let total_block_h = 280.0_f32;
                let avail_h = ui.available_height();
                let top_space = ((avail_h - total_block_h) * 0.5).max(10.0);
                ui.add_space(top_space);

                ui.vertical_centered(|ui| {
                    let (logo_rect, _) = ui.allocate_exact_size(Vec2::new(logo_w, logo_h), egui::Sense::hover());

                    // Subtle breathing ambient aura behind logo
                    let breathe = (time * 2.8).sin() * 0.15 + 0.85;
                    let glow_alpha = ((24.0 * t_entrance * breathe) as u8).min(255);
                    let glow_rect = logo_rect.expand(16.0);

                    // Dual ambient backlight (cyan left, purple right)
                    let left_half = egui::Rect::from_min_max(
                        glow_rect.min,
                        egui::pos2(glow_rect.center().x, glow_rect.max.y),
                    );
                    ui.painter().rect_filled(
                        left_half,
                        egui::Rounding { nw: 24.0, sw: 24.0, ne: 0.0, se: 0.0 },
                        Color32::from_rgba_unmultiplied(0, 210, 255, glow_alpha),
                    );

                    let right_half = egui::Rect::from_min_max(
                        egui::pos2(glow_rect.center().x, glow_rect.min.y),
                        glow_rect.max,
                    );
                    ui.painter().rect_filled(
                        right_half,
                        egui::Rounding { nw: 0.0, sw: 0.0, ne: 24.0, se: 24.0 },
                        Color32::from_rgba_unmultiplied(139, 92, 246, ((glow_alpha as f32) * 0.85) as u8),
                    );

                    // Draw the authentic, crisp master logo image
                    let logo_alpha = (t_entrance * 255.0) as u8;
                    ui.painter().image(
                        self.splash_logo.id(),
                        logo_rect,
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        Color32::from_rgba_unmultiplied(255, 255, 255, logo_alpha),
                    );

                    ui.add_space(16.0);

                    // Animated Expanding Cyan/Violet Laser Accent Divider
                    let laser_t = (time / 0.6).clamp(0.0, 1.0);
                    let laser_w = 400.0_f32 * (1.0 - (1.0 - laser_t).powi(2));
                    let (laser_rect, _) = ui.allocate_exact_size(Vec2::new(laser_w, 2.0), egui::Sense::hover());
                    ui.painter().rect_filled(
                        laser_rect,
                        egui::Rounding::same(1.0),
                        Color32::from_rgba_unmultiplied(0, 210, 255, (t_entrance * 160.0) as u8),
                    );

                    ui.add_space(20.0);

                    // Step Info calculation based on time
                    let (stage_icon, stage_title, stage_detail, stage_col) = if progress < 0.28 {
                        ("⚡", "STEP 1/4: INITIALIZING ENGINE CORE", "Allocating multi-stream segment matrix & TCP pool...", Color32::from_rgb(0, 210, 255))
                    } else if progress < 0.58 {
                        ("🌐", "STEP 2/4: CONNECTING BROWSER BRIDGE", "Binding native extension listener on http://127.0.0.1:9669...", Color32::from_rgb(56, 189, 248))
                    } else if progress < 0.86 {
                        ("🎬", "STEP 3/4: VERIFYING MEDIA PIPELINE", "Calibrating yt-dlp sniffer & FFmpeg muxing engine...", Color32::from_rgb(168, 85, 247))
                    } else {
                        ("🚀", "STEP 4/4: SYSTEM READY & INTERCEPTING", "Launching Rapid Command Dock interface...", Color32::from_rgb(34, 197, 94))
                    };

                    let bar_w = 420.0_f32;

                    // Info Header Row (Stage name on left, Percentage on right)
                    let pct = ((progress * 100.0) as usize).min(100);
                    ui.allocate_ui_with_layout(
                        Vec2::new(bar_w, 20.0),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(stage_icon).size(13.0));
                                ui.add_space(3.0);
                                ui.label(
                                    RichText::new(stage_title)
                                        .size(11.5)
                                        .strong()
                                        .color(stage_col),
                                );
                            });
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(
                                    RichText::new(format!("{:3}%", pct))
                                        .size(12.5)
                                        .strong()
                                        .monospace()
                                        .color(Color32::from_rgb(0, 210, 255)),
                                );
                            });
                        },
                    );

                    ui.add_space(8.0);

                    // High-Tech Glowing Progress Bar (420px x 6px)
                    let bar_h = 6.0_f32;
                    let (bar_rect, _) = ui.allocate_exact_size(Vec2::new(bar_w, bar_h), egui::Sense::hover());

                    // Outer glass track
                    ui.painter().rect_filled(
                        bar_rect,
                        egui::Rounding::same(3.0),
                        Color32::from_rgb(16, 22, 34),
                    );
                    ui.painter().rect_stroke(
                        bar_rect,
                        egui::Rounding::same(3.0),
                        Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(35, 48, 72, 160)),
                    );

                    // Smooth animated progress fill
                    let fill_w = (bar_w * progress).clamp(0.0, bar_w);
                    if fill_w > 0.0 {
                        let fill_rect = egui::Rect::from_min_size(bar_rect.min, Vec2::new(fill_w, bar_h));
                        ui.painter().rect_filled(
                            fill_rect,
                            egui::Rounding::same(3.0),
                            Color32::from_rgb(0, 210, 255),
                        );
                        // Glowing leading spark point
                        let spark_pos = egui::pos2(fill_rect.max.x, fill_rect.center().y);
                        ui.painter().circle_filled(
                            spark_pos,
                            4.5,
                            Color32::from_rgba_unmultiplied(0, 210, 255, 240),
                        );
                        ui.painter().circle_stroke(
                            spark_pos,
                            7.0,
                            Stroke::new(1.5_f32, Color32::from_rgba_unmultiplied(139, 92, 246, 160)),
                        );
                    }

                    ui.add_space(10.0);

                    // Diagnostic Step Detail Subtext
                    ui.label(
                        RichText::new(stage_detail)
                            .size(11.5)
                            .color(Color32::from_rgb(140, 160, 185)),
                    );
                });
            });
    }

    // Render Custom Frameless Window Title Bar
    fn render_custom_title_bar(&self, ctx: &egui::Context) {
        let titlebar_h = 46.0_f32;
        egui::TopBottomPanel::top("custom_window_title_bar")
            .frame(
                egui::Frame::none()
                    .fill(GLASS_BG)
                    .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                    .inner_margin(Margin {
                        left: 14.0,
                        right: 0.0,
                        top: 0.0,
                        bottom: 0.0,
                    }),
            )
            .show(ctx, |ui| {
                ui.set_height(titlebar_h);

                // Right to left layout for Window Caption Buttons (Flush at top-right: x=width, y=0)
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    let btn_size = Vec2::new(48.0, titlebar_h);
                    let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));

                    // 1. Close Button (Rightmost: touches top-right 0, 0 exactly)
                    if window_caption_button(
                        ui,
                        ModernIcon::WinClose,
                        btn_size,
                        GLASS_MUTED,
                        Color32::WHITE,
                        Color32::from_rgb(232, 17, 35),
                        "Close (Minimizes to System Tray)",
                    ) {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
                    }

                    // 2. Maximize / Restore Button
                    let (max_icon, max_tip) = if is_maximized {
                        (ModernIcon::WinRestore, "Restore Window")
                    } else {
                        (ModernIcon::WinMaximize, "Maximize Window")
                    };

                    if window_caption_button(
                        ui,
                        max_icon,
                        btn_size,
                        GLASS_MUTED,
                        GLASS_SECONDARY,
                        Color32::from_rgba_unmultiplied(255, 255, 255, 25),
                        max_tip,
                    ) {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
                    }

                    // 3. Minimize Button
                    if window_caption_button(
                        ui,
                        ModernIcon::WinMinimize,
                        btn_size,
                        GLASS_MUTED,
                        GLASS_SECONDARY,
                        Color32::from_rgba_unmultiplied(255, 255, 255, 25),
                        "Minimize to Taskbar",
                    ) {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                    }

                    // Left-to-right section for Logo and draggable area
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        let logo_h: f32 = 38.0;
                        let logo_w: f32 = (logo_h * 5.0167_f32).round();
                        let (logo_rect, _) = ui.allocate_exact_size(Vec2::new(logo_w, logo_h), egui::Sense::hover());
                        ui.painter().image(
                            self.titlebar_logo.id(),
                            logo_rect,
                            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                            Color32::WHITE,
                        );

                        let rem_w = ui.available_width().max(10.0);
                        let (_drag_rect, drag_resp) = ui.allocate_exact_size(
                            Vec2::new(rem_w, titlebar_h),
                            egui::Sense::click_and_drag(),
                        );
                        let drag_resp = drag_resp.on_hover_cursor(egui::CursorIcon::Grab);

                        if drag_resp.drag_started_by(egui::PointerButton::Primary) {
                            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                        }
                        if drag_resp.double_clicked() {
                            let is_max = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
                            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!is_max));
                        }
                    });
                });
            });
    }
}

// --- AUTOMATED SCREENSHOT HOOK ---
fn rdm_setup_screenshots(app: &mut RapidApp, _ctx: &egui::Context) {
    let step = std::env::var("RDM_SHOT_STEP").unwrap_or_else(|_| "1".to_string());
    
    // Set up mock tasks once
    if let Ok(mut tasks) = app.tasks.try_lock() {
        if tasks.is_empty() {
            let total_1 = 6_871_947_673u64;
            let downloaded_1 = 4_398_046_511u64;
            let mut segments_1 = Vec::new();
            let chunk_size = total_1 / 16;
            for i in 0..16 {
                let start = i as u64 * chunk_size;
                let end = if i == 15 { total_1 - 1 } else { start + chunk_size - 1 };
                let mut seg = rapid_core::Segment::new(i, start, end);
                if i < 8 {
                    seg.downloaded_bytes = seg.total_bytes();
                    seg.is_complete = true;
                } else if i < 14 {
                    seg.downloaded_bytes = (seg.total_bytes() as f64 * 0.65) as u64;
                    seg.is_complete = false;
                } else {
                    seg.downloaded_bytes = (seg.total_bytes() as f64 * 0.25) as u64;
                    seg.is_complete = false;
                }
                segments_1.push(seg);
            }

            tasks.push(ActiveTaskUI {
                id: "task-win11-iso".to_string(),
                filename: "Windows_11_24H2_English_x64.iso".to_string(),
                url: "https://software.download.prss.microsoft.com/db/Win11_24H2_English_x64.iso".to_string(),
                target_file: std::path::PathBuf::from("C:\\Users\\User\\Downloads\\Windows_11_24H2_English_x64.iso"),
                total_bytes: Some(total_1),
                downloaded_bytes: downloaded_1,
                progress_percent: 64.0,
                speed_bps: 45_200_000,
                eta_seconds: Some(54),
                status: rapid_core::DownloadStatus::Downloading,
                segments: segments_1,
                task_handle: None,
                num_segments: 16,
                is_resuming: false,
                cookies: None,
                referrer: None,
                user_agent: None,
                is_youtube: false,
                quality: None,
                yt_cancel_token: None,
            });

            tasks.push(ActiveTaskUI {
                id: "task-rustrover".to_string(),
                filename: "RustRover-2026.1.1.exe".to_string(),
                url: "https://download.jetbrains.com/rustrover/RustRover-2026.1.1.exe".to_string(),
                target_file: std::path::PathBuf::from("C:\\Users\\User\\Downloads\\RustRover-2026.1.1.exe"),
                total_bytes: Some(1_240_500_000),
                downloaded_bytes: 980_000_000,
                progress_percent: 79.0,
                speed_bps: 18_400_000,
                eta_seconds: Some(14),
                status: rapid_core::DownloadStatus::Downloading,
                segments: Vec::new(),
                task_handle: None,
                num_segments: 8,
                is_resuming: false,
                cookies: None,
                referrer: None,
                user_agent: None,
                is_youtube: false,
                quality: None,
                yt_cancel_token: None,
            });

            tasks.push(ActiveTaskUI {
                id: "task-ubuntu".to_string(),
                filename: "ubuntu-24.04-desktop-amd64.iso".to_string(),
                url: "https://releases.ubuntu.com/24.04/ubuntu-24.04-desktop-amd64.iso".to_string(),
                target_file: std::path::PathBuf::from("C:\\Users\\User\\Downloads\\ubuntu-24.04-desktop-amd64.iso"),
                total_bytes: Some(6_120_000_000),
                downloaded_bytes: 6_120_000_000,
                progress_percent: 100.0,
                speed_bps: 0,
                eta_seconds: None,
                status: rapid_core::DownloadStatus::Completed,
                segments: Vec::new(),
                task_handle: None,
                num_segments: 16,
                is_resuming: false,
                cookies: None,
                referrer: None,
                user_agent: None,
                is_youtube: false,
                quality: None,
                yt_cancel_token: None,
            });
        }
    }

    app.selected_task_index = Some(0);

    match step.as_str() {
        "1" => {
            app.show_add_dialog = false;
            app.show_browser_integration_modal = false;
            app.show_scheduler_modal = false;
            app.show_about_modal = false;
        }
        "2" => {
            app.show_add_dialog = true;
            app.input_url = "https://releases.ubuntu.com/24.04/ubuntu-24.04-desktop-amd64.iso".to_string();
            app.input_filename = "ubuntu-24.04-desktop-amd64.iso".to_string();
            app.input_segments = 16;
            app.show_browser_integration_modal = false;
            app.show_scheduler_modal = false;
            app.show_about_modal = false;
        }
        "3" => {
            app.show_add_dialog = false;
            app.show_browser_integration_modal = true;
            app.show_scheduler_modal = false;
            app.show_about_modal = false;
        }
        "4" => {
            app.show_add_dialog = false;
            app.show_browser_integration_modal = false;
            app.show_scheduler_modal = true;
            app.scheduler.enabled = true;
            app.scheduler.start_hour = 2;
            app.scheduler.start_minute = 0;
            app.scheduler.stop_hour = 6;
            app.scheduler.stop_minute = 0;
            app.show_about_modal = false;
        }
        "5" => {
            app.show_add_dialog = false;
            app.show_browser_integration_modal = false;
            app.show_scheduler_modal = false;
            app.show_about_modal = true;
        }
        _ => {}
    }
}

impl eframe::App for RapidApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Automated Screenshot Capture Engine
        if std::env::var("RDM_CAPTURE_SCREENSHOTS").is_ok() {
            self.splash_duration = std::time::Duration::ZERO;
            let step = std::env::var("RDM_SHOT_STEP").unwrap_or_else(|_| "1".to_string());
            rdm_setup_screenshots(self, ctx);

            // Check for incoming screenshot events
            for event in &ctx.input(|i| i.raw.events.clone()) {
                if let egui::Event::Screenshot { image, .. } = event {
                    let out_path = format!("D:/mahbub/project/rapid_download_manager/store_listing/assets/raw_shot_{}.raw", step);
                    let w = image.size[0] as u32;
                    let h = image.size[1] as u32;
                    let mut bytes = w.to_le_bytes().to_vec();
                    bytes.extend_from_slice(&h.to_le_bytes());
                    for p in &image.pixels {
                        bytes.extend_from_slice(&p.to_array());
                    }
                    let _ = std::fs::write(&out_path, bytes);
                    log::info!("CAPTURED SCREENSHOT STEP {}: {}x{} to {}", step, w, h, out_path);
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    return;
                }
            }

            static FRAME_COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let frame = FRAME_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if frame < 10 {
                ctx.request_repaint();
            } else if frame == 10 {
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot);
                ctx.request_repaint();
            } else if frame > 25 {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
        if !self.has_requested_initial_focus {
            self.has_requested_initial_focus = true;
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        }

        // Handle animated splash screen on startup (No title bar during splash, full center-center window)
        let elapsed = self.splash_start.elapsed();
        if elapsed < self.splash_duration {
            ctx.request_repaint();
            self.render_splash_screen(ctx, elapsed);
            return;
        }

        // Render custom frameless title bar at the top of the main window
        self.render_custom_title_bar(ctx);

        // 1. Time-based Download Scheduler Tick (Section 32)
        if self.scheduler.enabled {
            let now = chrono::Local::now();
            let cur_h = now.hour();
            let cur_m = now.minute();
            let cur_s = now.second();

            // Trigger Start
            if cur_s == 0 && cur_h == self.scheduler.start_hour && cur_m == self.scheduler.start_minute {
                if !self.scheduler.has_triggered_start {
                    self.scheduler.has_triggered_start = true;
                    self.resume_all();
                }
            } else if cur_m != self.scheduler.start_minute {
                self.scheduler.has_triggered_start = false;
            }

            // Trigger Stop
            if cur_s == 0 && cur_h == self.scheduler.stop_hour && cur_m == self.scheduler.stop_minute {
                if !self.scheduler.has_triggered_stop {
                    self.scheduler.has_triggered_stop = true;
                    self.pause_all();
                    if self.scheduler.auto_shutdown {
                        #[cfg(windows)]
                        {
                            let _ = std::process::Command::new("shutdown").args(&["/s", "/t", "60"]).spawn();
                        }
                    }
                }
            } else if cur_m != self.scheduler.stop_minute {
                self.scheduler.has_triggered_stop = false;
            }
        }

        // 2. Drag & Drop Link/File Interception (Section 27)
        let dropped_files = ctx.input(|i| i.raw.dropped_files.clone());
        if !dropped_files.is_empty() {
            for dropped in dropped_files {
                let mut detected_url: Option<String> = None;
                if let Some(ref path) = dropped.path {
                    if path.extension().and_then(|e| e.to_str()).map(|e| e.eq_ignore_ascii_case("url")).unwrap_or(false) {
                        if let Ok(content) = std::fs::read_to_string(path) {
                            for line in content.lines() {
                                if line.trim().starts_with("URL=") {
                                    detected_url = Some(line.trim()[4..].trim().to_string());
                                    break;
                                }
                            }
                        }
                    } else if path.extension().and_then(|e| e.to_str()).map(|e| e.eq_ignore_ascii_case("txt")).unwrap_or(false) {
                        if let Ok(content) = std::fs::read_to_string(path) {
                            let first_line = content.lines().next().unwrap_or("").trim();
                            if first_line.starts_with("http://") || first_line.starts_with("https://") {
                                detected_url = Some(first_line.to_string());
                            }
                        }
                    }
                }
                if let Some(url) = detected_url {
                    self.open_add_download_dialog(ctx);
                    self.input_url = url.clone();
                    self.probe_url_filename(&url, ctx);
                }
            }
        }

        // Visual Drag & Drop Hover Overlay
        if ctx.input(|i| !i.raw.hovered_files.is_empty()) {
            let screen_rect = ctx.screen_rect();
            let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("drag_drop_overlay")));
            painter.rect_filled(screen_rect, 0.0, Color32::from_rgba_unmultiplied(2, 6, 23, 210));
            painter.rect_stroke(
                screen_rect.shrink(18.0),
                12.0,
                Stroke::new(2.5_f32, GLASS_SECONDARY),
            );
            painter.text(
                screen_rect.center(),
                egui::Align2::CENTER_CENTER,
                "📥 Drop Link or File here to Download with Rapid!",
                egui::FontId::proportional(18.0),
                Color32::WHITE,
            );
        }

        // Request repaint continuously if any download is active for smooth progress bars
        ctx.request_repaint_after(Duration::from_millis(100));

        // Receive any asynchronously resolved filenames from background probe
        while let Ok((p_url, p_name)) = self.filename_resolver_rx.try_recv() {
            if let Some(ref mut prompt) = self.current_browser_prompt {
                if prompt.url == p_url {
                    if prompt.filename.is_empty()
                        || prompt.filename == "Downloading..."
                        || prompt.filename == "Resolving filename..."
                        || prompt.filename == "Resolving original filename..."
                        || prompt.filename == "Detecting filename..."
                        || rapid_core::engine::is_generic_placeholder(&prompt.filename) {
                        prompt.filename = p_name.clone();
                        let cat_dest = get_categorized_destination(&self.base_download_dir, &p_name);
                        prompt.dest_dir = cat_dest.to_string_lossy().to_string();
                    }
                }
            }
            if let Ok(mut q) = self.pending_browser_queue.lock() {
                for item in q.iter_mut() {
                    if item.url == p_url {
                        if item.filename.is_empty()
                            || item.filename == "Downloading..."
                            || item.filename == "Resolving filename..."
                            || item.filename == "Resolving original filename..."
                            || item.filename == "Detecting filename..."
                            || rapid_core::engine::is_generic_placeholder(&item.filename) {
                            item.filename = p_name.clone();
                        }
                    }
                }
            }

            if self.show_add_dialog && (self.input_url == p_url || self.input_url.trim() == p_url.trim()) {
                if self.input_filename.is_empty()
                    || self.input_filename == "watch"
                    || self.input_filename == "Resolving video title..."
                    || self.input_filename == "Resolving filename..."
                    || self.input_filename == "Resolving original filename..."
                    || rapid_core::engine::is_generic_placeholder(&self.input_filename)
                    || !self.input_filename.contains('.')
                {
                    self.input_filename = p_name.clone();
                    let cat_dest = get_categorized_destination(&self.base_download_dir, &p_name);
                    self.input_dest = cat_dest.to_string_lossy().to_string();
                }
            }
        }

        // Pop next pending browser download if not currently prompting
        if self.current_browser_prompt.is_none() {
            if let Ok(mut q) = self.pending_browser_queue.lock() {
                if let Some(item) = q.pop_front() {
                    if item.filename.is_empty()
                        || item.filename == "Downloading..."
                        || item.filename == "Resolving filename..."
                        || item.filename == "Resolving original filename..."
                        || item.filename == "Detecting filename..."
                        || rapid_core::engine::is_generic_placeholder(&item.filename) {
                        let tx = self.filename_resolver_tx.clone();
                        let url_clone = item.url.clone();
                        let cookies_clone = item.cookies.clone();
                        let ua_clone = item.user_agent.clone();
                        let ref_clone = item.referrer.clone();
                        let is_gd = item.is_gdrive;
                        let is_yt_item = item.is_youtube;
                        let ctx_clone = ctx.clone();

                        self.tokio_rt.spawn(async move {
                            let mut cb = reqwest::Client::builder()
                                .timeout(std::time::Duration::from_secs(12))
                                .redirect(reqwest::redirect::Policy::limited(10));

                            let mut headers = reqwest::header::HeaderMap::new();
                            if let Some(ref ua) = ua_clone {
                                if let Ok(v) = reqwest::header::HeaderValue::from_str(ua) {
                                    headers.insert(reqwest::header::USER_AGENT, v);
                                }
                            }
                            if let Some(ref r) = ref_clone {
                                if let Ok(v) = reqwest::header::HeaderValue::from_str(r) {
                                    headers.insert(reqwest::header::REFERER, v);
                                }
                            }
                            if let Some(ref c) = cookies_clone {
                                if let Ok(v) = reqwest::header::HeaderValue::from_str(c) {
                                    headers.insert(reqwest::header::COOKIE, v);
                                }
                            }
                            cb = cb.default_headers(headers);

                            if let Ok(client) = cb.build() {
                                let is_media_vid = is_yt_item || rapid_core::youtube::YoutubeResolver::is_extractable_platform(&url_clone);
                                let detected_name = if is_gd {
                                    let res_type = rapid_core::gdrive::GDriveResolver::parse_resource_type(&url_clone);
                                    if let rapid_core::gdrive::GDriveResourceType::File(file_id) = res_type {
                                        rapid_core::gdrive::GDriveResolver::resolve_file_download_url(&client, &file_id)
                                            .await
                                            .ok()
                                            .map(|r| r.name)
                                    } else {
                                        None
                                    }
                                } else if is_media_vid {
                                    rapid_core::youtube::YoutubeResolver::resolve_metadata(&url_clone)
                                        .await
                                        .ok()
                                        .map(|m| m.clean_filename)
                                } else {
                                    rapid_core::Probe::inspect(&client, &url_clone)
                                        .await
                                        .ok()
                                        .map(|m| m.filename)
                                };

                                if let Some(fname) = detected_name {
                                    if !rapid_core::engine::is_generic_placeholder(&fname) {
                                        let _ = tx.send((url_clone, fname));
                                        ctx_clone.request_repaint();
                                    }
                                }
                            }
                        });
                    }
                    self.current_browser_prompt = Some(item);
                }
            }
        }

        // -------------------------------------------------------------
        // Queue Manager: Auto-dispatch queued downloads up to max_concurrent_downloads
        // -------------------------------------------------------------
        let mut active_downloading = {
            if let Ok(list) = self.tasks.try_lock() {
                list.iter().filter(|t| t.status == DownloadStatus::Downloading).count()
            } else {
                self.max_concurrent_downloads
            }
        };

        while active_downloading < self.max_concurrent_downloads {
            let next_queued_idx = {
                if let Ok(list) = self.tasks.try_lock() {
                    list.iter().position(|t| t.status == DownloadStatus::Queued)
                } else {
                    None
                }
            };

            if let Some(idx) = next_queued_idx {
                self.resume_download(idx);
                active_downloading += 1;
            } else {
                break;
            }
        }

        let mut action_resume: Option<usize> = None;
        let mut action_pause: Option<usize> = None;
        let mut action_redownload: Option<usize> = None;
        let mut action_remove_ids: Vec<String> = Vec::new();
        let mut action_open_folder: Option<usize> = None;
        let mut action_open_file: Option<usize> = None;
        let mut action_copy_url: Option<String> = None;

// Auto-selection removed: only user selection is valid

        // Metrics calculations for bottom shortcut dock
        let mut total_speed_bps: u64 = 0;
        let mut active_count = 0;
        let mut paused_count = 0;
        let mut finished_count = 0;
        let mut queued_count = 0;
        let mut total_downloaded: u64 = 0;

        if let Ok(tasks) = self.tasks.try_lock() {
            for t in tasks.iter() {
                total_downloaded += t.downloaded_bytes;
                match t.status {
                    DownloadStatus::Downloading => {
                        active_count += 1;
                        total_speed_bps += t.speed_bps;
                    }
                    DownloadStatus::Paused => paused_count += 1,
                    DownloadStatus::Completed => finished_count += 1,
                    DownloadStatus::Queued => queued_count += 1,
                    _ => {}
                }
            }
        }

        let mut do_pause_all = false;
        let mut do_resume_all = false;
        let mut do_open_add_dialog = false;

        // System Tray synchronization
        #[cfg(windows)]
        if let Some(ref tray) = self.tray_handle {
            if tray.should_restore() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
                ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            }
            if tray.should_pause_all() {
                do_pause_all = true;
            }
            if tray.should_resume_all() {
                do_resume_all = true;
            }
            let tip = if active_count > 0 {
                format!("Rapid Download Manager\n⚡ Speed: {} | Active: {}\n✔ Done: {}", format_speed(total_speed_bps), active_count, finished_count)
            } else {
                format!("Rapid Download Manager\n⚡ Ready | Done: {}", finished_count)
            };
            tray.update_tooltip(&tip);
        }

        // Minimize to System Tray when window close "X" is clicked (IDM style)
        if ctx.input(|i| i.viewport().close_requested()) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
        }

        // =========================================================================
        // PROPRIETARY COMMAND DOCK (TOP BAR)
        // =========================================================================
        egui::TopBottomPanel::top("command_dock")
            .frame(
                egui::Frame::none()
                    .fill(VELVET_SURFACE)
                    .stroke(Stroke::new(1.0_f32, VELVET_BORDER))
                    .inner_margin(Margin::symmetric(14.0, 10.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // 1. High-Precision Segmented Filter Control (All Downloads, Downloading, Paused, Completed)
                    let total_tasks_count = if let Ok(ref t) = self.tasks.try_lock() { t.len() } else { 0 };
                    let filter_tabs = [
                        (FilterCategory::All, "All Downloads", total_tasks_count, "Show all downloads"),
                        (FilterCategory::Active, "Downloading", active_count, "Show actively downloading files"),
                        (FilterCategory::Paused, "Paused", paused_count, "Show paused and interrupted downloads"),
                        (FilterCategory::Completed, "Completed", finished_count, "Show completed downloads"),
                    ];

                    egui::Frame::none()
                        .fill(GLASS_BG)
                        .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                        .rounding(egui::Rounding::same(8.0))
                        .inner_margin(Margin::symmetric(3.0, 3.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing = Vec2::new(2.0, 0.0);
                                for (cat, label, count, tip) in filter_tabs {
                                    let is_active = self.selected_filter == cat;
                                    let tab_text = format!("{} ({})", label, count);
                                    let text_widget = RichText::new(tab_text)
                                        .size(11.5)
                                        .color(if is_active { Color32::WHITE } else { GLASS_MUTED })
                                        .strong();

                                    let btn = if is_active {
                                        egui::Button::new(text_widget)
                                            .fill(GLASS_PRIMARY)
                                            .rounding(egui::Rounding::same(6.0))
                                    } else {
                                        egui::Button::new(text_widget)
                                            .fill(Color32::TRANSPARENT)
                                            .rounding(egui::Rounding::same(6.0))
                                    };

                                    if ui.add(btn).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text(tip).clicked() {
                                        self.selected_filter = cat;
                                        self.selected_task_index = None;
                                        self.selected_tasks.clear();
                                    }
                                }
                            });
                        });

                    // 2. Search Filter Box
                    ui.add_space(8.0);
                    egui::Frame::none()
                        .fill(GLASS_BG)
                        .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                        .rounding(egui::Rounding::same(8.0))
                        .inner_margin(Margin::symmetric(9.0, 5.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                let (rect, _) = ui.allocate_exact_size(Vec2::new(13.0, 13.0), egui::Sense::hover());
                                draw_modern_icon(ui.painter(), ModernIcon::Search, rect, GLASS_SECONDARY);
                                ui.add_space(4.0);
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.search_query)
                                        .hint_text("Search downloads (name, URL)...")
                                        .desired_width(170.0)
                                        .frame(false)
                                );
                                if !self.search_query.is_empty() && ui.small_button("✕").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                    self.search_query.clear();
                                }
                            });
                        });

                    // 3. Right-side action controls
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Primary Action Button: "Add Download"
                        let (btn_rect, btn_resp) = ui.allocate_exact_size(Vec2::new(130.0, 32.0), egui::Sense::click());
                        let is_btn_h = btn_resp.hovered();
                        let btn_bg = if is_btn_h { GLASS_PRIMARY_HOVER } else { GLASS_PRIMARY };
                        ui.painter().rect_filled(btn_rect, 7.0, btn_bg);
                        let plus_rect = egui::Rect::from_center_size(
                            egui::pos2(btn_rect.min.x + 18.0, btn_rect.center().y),
                            Vec2::new(12.0, 12.0),
                        );
                        draw_modern_icon(ui.painter(), ModernIcon::Plus, plus_rect, Color32::WHITE);
                        ui.painter().text(
                            egui::pos2(btn_rect.min.x + 32.0, btn_rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            "Add Download",
                            egui::FontId::proportional(12.5),
                            Color32::WHITE,
                        );

                        if btn_resp.on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Add a new accelerated download URL").clicked() {
                            do_open_add_dialog = true;
                        }

                        ui.add_space(4.0);

                        // Pause All
                        if modern_icon_button(ui, ModernIcon::Pause, Vec2::new(32.0, 30.0), STATUS_PAUSED, GLASS_PRIMARY, "Pause all active downloads") {
                            do_pause_all = true;
                        }

                        // Resume All
                        if modern_icon_button(ui, ModernIcon::Play, Vec2::new(32.0, 30.0), GLASS_SECONDARY, GLASS_SECONDARY_HOVER, "Resume all paused downloads") {
                            do_resume_all = true;
                        }

                        // Minimize to System Tray
                        if modern_icon_button(ui, ModernIcon::MinimizeTray, Vec2::new(32.0, 30.0), GLASS_MUTED, GLASS_SECONDARY, "Minimize to Windows System Tray") {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
                        }

                        ui.add_space(8.0);

                        // Bandwidth Speed Limiter Dropdown
                        let speed_options = [
                            ("⚡ Unlimited", 0u64),
                            ("🐢 512 KB/s", 512 * 1024),
                            ("🚀 1 MB/s", 1024 * 1024),
                            ("🏎️ 2 MB/s", 2 * 1024 * 1024),
                            ("⚡ 5 MB/s", 5 * 1024 * 1024),
                            ("⚡ 10 MB/s", 10 * 1024 * 1024),
                        ];

                        let current_limit_label = speed_options[self.selected_speed_limit_idx.min(speed_options.len() - 1)].0;
                        egui::ComboBox::from_id_source("toolbar_speed_limiter")
                            .selected_text(RichText::new(current_limit_label).size(11.5).color(GLASS_SECONDARY).strong())
                            .height(180.0)
                            .width(105.0)
                            .show_ui(ui, |ui| {
                                for (idx, (label, val)) in speed_options.iter().enumerate() {
                                    let is_sel = self.selected_speed_limit_idx == idx;
                                    if ui.selectable_label(is_sel, RichText::new(*label).size(11.5)).clicked() {
                                        self.selected_speed_limit_idx = idx;
                                        self.speed_limit_bps.store(*val, Ordering::Relaxed);
                                    }
                                }
                            });

                        if self.selected_filter == FilterCategory::Completed {
                            ui.add_space(6.0);
                            let clear_btn = egui::Button::new(RichText::new("🗑 Clear History").size(11.5).color(Color32::from_rgb(255, 120, 120)))
                                .fill(Color32::from_rgba_unmultiplied(45, 18, 24, 180))
                                .stroke(Stroke::new(1.0_f32, STATUS_FAILED))
                                .rounding(egui::Rounding::same(5.0));
                            if ui.add(clear_btn).on_hover_text("Remove all completed downloads from history").clicked() {
                                clear_all_history();
                                if let Ok(mut list) = self.tasks.try_lock() {
                                    list.retain(|t| t.status != DownloadStatus::Completed);
                                }
                            }
                        }
                    });
                });

                // Contextual Toolbar (Modern Dynamic Sub-Bar for selected download - visible tasks only)
                let (has_selection, display_name, effective_selected_idx, is_multi, is_downloading, is_resumable) = {
                    if let Ok(ref tasks) = self.tasks.try_lock() {
                        let visible_selected_ids: Vec<&String> = self.selected_tasks
                            .iter()
                            .filter(|id| tasks.iter().any(|t| &t.id == *id && self.is_task_visible(t)))
                            .collect();

                        let visible_count = visible_selected_ids.len();

                        if visible_count > 1 {
                            let any_downloading = tasks.iter().any(|t| self.selected_tasks.contains(&t.id) && self.is_task_visible(t) && t.status == DownloadStatus::Downloading);
                            let any_resumable = tasks.iter().any(|t| self.selected_tasks.contains(&t.id) && self.is_task_visible(t) && matches!(t.status, DownloadStatus::Paused | DownloadStatus::Failed(_)));
                            (true, format!("{} items", visible_count), None, true, any_downloading, any_resumable)
                        } else if visible_count == 1 {
                            let id = visible_selected_ids[0];
                            if let Some(pos) = tasks.iter().position(|t| &t.id == id && self.is_task_visible(t)) {
                                let task = &tasks[pos];
                                let downloading = task.status == DownloadStatus::Downloading;
                                let resumable = matches!(task.status, DownloadStatus::Paused | DownloadStatus::Failed(_));
                                (true, task.filename.clone(), Some(pos), false, downloading, resumable)
                            } else {
                                (false, String::new(), None, false, false, false)
                            }
                        } else if let Some(idx) = self.selected_task_index {
                            if idx < tasks.len() && self.is_task_visible(&tasks[idx]) {
                                let task = &tasks[idx];
                                let downloading = task.status == DownloadStatus::Downloading;
                                let resumable = matches!(task.status, DownloadStatus::Paused | DownloadStatus::Failed(_));
                                (true, task.filename.clone(), Some(idx), false, downloading, resumable)
                            } else {
                                (false, String::new(), None, false, false, false)
                            }
                        } else {
                            (false, String::new(), None, false, false, false)
                        }
                    } else {
                        (false, String::new(), None, false, false, false)
                    }
                };

                if has_selection {
                    ui.add_space(6.0);
                    ui.separator();
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Selected:").size(11.0).color(Color32::from_rgb(160, 140, 170)));
                        let shaped_display = shape_bengali(&display_name);
                        ui.label(RichText::new(&shaped_display).size(11.5).strong().color(PINK_PASTEL));

                        ui.add_space(10.0);
                        if is_resumable && modern_icon_button(ui, ModernIcon::Play, Vec2::new(26.0, 22.0), STATUS_COMPLETED, PINK_NEON, if is_multi { "Resume selected" } else { "Resume download" }) {
                            if let Some(idx) = effective_selected_idx {
                                action_resume = Some(idx);
                            }
                        }
                        if is_downloading && modern_icon_button(ui, ModernIcon::Pause, Vec2::new(26.0, 22.0), STATUS_PAUSED, PINK_NEON, if is_multi { "Pause selected" } else { "Pause download" }) {
                            if let Some(idx) = effective_selected_idx {
                                action_pause = Some(idx);
                            }
                        }
                        if !is_multi {
                            if modern_icon_button(ui, ModernIcon::Folder, Vec2::new(26.0, 22.0), PINK_PASTEL, PINK_NEON, "Open containing folder") {
                                if let Some(idx) = effective_selected_idx {
                                    action_open_folder = Some(idx);
                                }
                            }
                            if modern_icon_button(ui, ModernIcon::Refresh, Vec2::new(26.0, 22.0), PINK_PASTEL, PINK_NEON, "Redownload from start") {
                                if let Some(idx) = effective_selected_idx {
                                    action_redownload = Some(idx);
                                }
                            }
                        }
                        if modern_icon_button(ui, ModernIcon::Trash, Vec2::new(26.0, 22.0), STATUS_FAILED, Color32::from_rgb(255, 40, 80), if is_multi { "Delete selected" } else { "Delete download" }) {
                            if !self.selected_tasks.is_empty() {
                                action_remove_ids = self.selected_tasks.iter().cloned().collect();
                            } else if let Some(idx) = effective_selected_idx {
                                if let Ok(ref tasks) = self.tasks.try_lock() {
                                    if idx < tasks.len() {
                                        action_remove_ids.push(tasks[idx].id.clone());
                                    }
                                }
                            }
                        }
                    });
                }
            });

        // Ultra-Clean, Non-Colliding Modern Status Bar (VS Code / Arc Inspired)
        egui::TopBottomPanel::bottom("custom_status_bar")
            .exact_height(28.0)
            .frame(
                egui::Frame::none()
                    .fill(Color32::from_rgb(9, 12, 22))
                    .stroke(Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(148, 163, 184, 35)))
                    .inner_margin(Margin::symmetric(12.0, 4.0)),
            )
            .show(ctx, |ui| {
                let (full_rect, _) = ui.allocate_exact_size(
                    Vec2::new(ui.available_width(), 20.0),
                    egui::Sense::hover(),
                );
                let total_w = full_rect.width();
                let bar_h = full_rect.height();

                // Determine if we are in compact mode (< 600px width)
                let is_compact = total_w < 600.0;

                // Right action dock width:
                // 3 action buttons (Downloads, Browser, About) + 1 status indicator (Tray)
                let right_w = if is_compact { 115.0_f32 } else { 255.0_f32 }.min(total_w * 0.55);
                let left_w = (total_w - right_w - 16.0).max(0.0);

                let left_rect = egui::Rect::from_min_size(full_rect.min, Vec2::new(left_w, bar_h));
                let right_rect = egui::Rect::from_min_size(
                    egui::pos2(full_rect.max.x - right_w, full_rect.min.y),
                    Vec2::new(right_w, bar_h),
                );

                // 1. LEFT REGION: Single-line Status & Metrics (Mathematically bounded and clipped to left_rect)
                ui.allocate_ui_at_rect(left_rect, |ui| {
                    ui.set_clip_rect(left_rect);
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(8.0, 0.0);

                        // Status dot & summary label
                        let (dot_color, has_glow, status_label) = if active_count > 0 {
                            let spd = format_speed(total_speed_bps);
                            (
                                Color32::from_rgb(56, 189, 248), // Sky blue
                                true,
                                format!("{} active  •  {}", active_count, spd),
                            )
                        } else if paused_count > 0 {
                            (
                                Color32::from_rgb(251, 191, 36), // Amber
                                false,
                                format!("{} paused", paused_count),
                            )
                        } else {
                            (
                                Color32::from_rgb(52, 211, 153), // Emerald
                                false,
                                "Ready".to_string(),
                            )
                        };

                        // Dot icon
                        let (dot_rect, _) = ui.allocate_exact_size(Vec2::new(10.0, 10.0), egui::Sense::hover());
                        let center = dot_rect.center();
                        if has_glow {
                            ui.painter().circle_stroke(
                                center,
                                4.5,
                                Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(56, 189, 248, 80)),
                            );
                        }
                        ui.painter().circle_filled(center, 3.0, dot_color);

                        // Primary status label
                        ui.label(
                            RichText::new(&status_label)
                                .size(11.0)
                                .color(if active_count > 0 { Color32::from_rgb(241, 245, 249) } else { Color32::from_rgb(226, 232, 240) })
                                .strong(),
                        );

                        // Subtle vertical separator
                        let sep_color = Color32::from_rgba_unmultiplied(148, 163, 184, 45);

                        // Engine HTTP listener info (if at least 130px remain)
                        if ui.available_width() >= 130.0 {
                            ui.label(RichText::new("│").size(10.0).color(sep_color));
                            let (b_rect, _) = ui.allocate_exact_size(Vec2::new(9.0, 9.0), egui::Sense::hover());
                            draw_modern_icon(ui.painter(), ModernIcon::PulseBeacon, b_rect, GLASS_PRIMARY);
                            ui.add_space(-2.0);
                            ui.label(RichText::new("127.0.0.1:9669").size(10.5).color(Color32::from_rgb(148, 163, 184)))
                                .on_hover_text("Rapid Engine HTTP Listener active on 127.0.0.1:9669");
                        }

                        // Completed tasks info (if at least 95px remain)
                        if finished_count > 0 && ui.available_width() >= 95.0 {
                            ui.label(RichText::new("│").size(10.0).color(sep_color));
                            let (c_rect, _) = ui.allocate_exact_size(Vec2::new(9.0, 9.0), egui::Sense::hover());
                            draw_modern_icon(ui.painter(), ModernIcon::Check, c_rect, STATUS_COMPLETED);
                            ui.add_space(-2.0);
                            ui.label(RichText::new(format!("{} done", finished_count)).size(10.5).color(STATUS_COMPLETED))
                                .on_hover_text("Completed downloads in this session");
                        }

                        // Total downloaded session data (if at least 110px remain)
                        if total_downloaded > 0 && ui.available_width() >= 110.0 {
                            ui.label(RichText::new("│").size(10.0).color(sep_color));
                            ui.label(
                                RichText::new(format!("{} total", format_bytes(total_downloaded)))
                                    .size(10.5)
                                    .color(Color32::from_rgb(148, 163, 184)),
                            ).on_hover_text("Total bytes downloaded across all tasks");
                        }
                    });
                });

                // 2. RIGHT REGION: Quick Action Dock & System Tray indicator (Anchored to right)
                ui.allocate_ui_at_rect(right_rect, |ui| {
                    ui.set_clip_rect(right_rect);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(6.0, 0.0);

                        // Tray Status (Far Right)
                        render_status_tray_pill(ui, is_compact);

                        // About Modal Action Button
                        if render_status_action_btn(
                            ui,
                            ModernIcon::PulseBeacon,
                            "About",
                            "Check for updates, release notes, and application identity",
                            is_compact,
                        ) {
                            self.show_about_modal = true;
                            let should_check = {
                                let s = self.update_status.lock().unwrap();
                                matches!(*s, UpdateStatus::Idle)
                            };
                            if should_check {
                                self.check_for_updates();
                            }
                        }

                        // Browser Integration Action Button
                        if render_status_action_btn(
                            ui,
                            ModernIcon::Globe,
                            "Browser",
                            "Auto-integrate extension into Chrome, Edge, Brave, Opera & Firefox",
                            is_compact,
                        ) {
                            self.show_browser_integration_modal = true;
                            self.browser_integration_msg = None;
                        }

                        // Downloads Directory Action Button
                        if render_status_action_btn(
                            ui,
                            ModernIcon::Folder,
                            "Downloads",
                            "Open Downloads folder in Windows File Explorer",
                            is_compact,
                        ) {
                            let _ = open::that(dirs_or_fallback());
                        }
                    });
                });
            });

        // Bottom Details Panel (Connection / Segment Progress - only when an active task is selected and visible)
        let effective_details_idx = {
            if let Ok(tasks) = self.tasks.try_lock() {
                if let Some(first_checked) = self.selected_tasks.iter().next() {
                    tasks.iter().position(|t| &t.id == first_checked && self.is_task_visible(t))
                } else if let Some(idx) = self.selected_task_index {
                    if idx < tasks.len() && self.is_task_visible(&tasks[idx]) {
                        Some(idx)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        };

        let has_segments_to_show = {
            if let Some(idx) = effective_details_idx {
                if let Ok(tasks) = self.tasks.try_lock() {
                    idx < tasks.len() && !tasks[idx].segments.is_empty()
                } else {
                    false
                }
            } else {
                false
            }
        };

        if has_segments_to_show {
            egui::TopBottomPanel::bottom("bottom_panel")
                .frame(
                    egui::Frame::none()
                        .fill(GLASS_SURFACE)
                        .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                        .inner_margin(Margin::symmetric(14.0, 10.0)),
                )
                .show(ctx, |ui| {
                    if let Ok(tasks) = self.tasks.try_lock() {
                        if let Some(idx) = effective_details_idx {
                            if idx < tasks.len() {
                                let task = &tasks[idx];

                                // Header row: Title + Task info badge + Speed
                                ui.horizontal(|ui| {
                                    let (beacon_rect, _) = ui.allocate_exact_size(Vec2::new(14.0, 14.0), egui::Sense::hover());
                                    draw_modern_icon(ui.painter(), ModernIcon::PulseBeacon, beacon_rect, GLASS_SECONDARY);
                                    ui.add_space(4.0);
                                    ui.label(RichText::new("Multi-Part Stream Connections").strong().color(GLASS_TEXT).size(12.5));
                                    ui.add_space(6.0);

                                    // Active threads badge
                                    egui::Frame::none()
                                        .fill(Color32::from_rgb(26, 20, 52))
                                        .stroke(Stroke::new(1.0_f32, GLASS_PRIMARY))
                                        .rounding(egui::Rounding::same(4.0))
                                        .inner_margin(Margin::symmetric(6.0, 2.0))
                                        .show(ui, |ui| {
                                            ui.label(RichText::new(format!("{} Streams", task.segments.len())).size(10.0).color(GLASS_PRIMARY).strong());
                                        });

                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if task.status == DownloadStatus::Downloading && task.speed_bps > 0 {
                                            ui.label(RichText::new(format_speed(task.speed_bps)).color(GLASS_SECONDARY).size(12.0).strong());
                                            ui.label(RichText::new("Overall Speed:").color(GLASS_MUTED).size(11.0));
                                        }
                                    });
                                });
                                ui.add_space(6.0);

                                // Show error notification banner if failed
                                if let DownloadStatus::Failed(ref err) = task.status {
                                    egui::Frame::none()
                                        .fill(Color32::from_rgb(45, 18, 24))
                                        .stroke(Stroke::new(1.0_f32, STATUS_FAILED))
                                        .inner_margin(Margin::symmetric(10.0, 6.0))
                                        .rounding(egui::Rounding::same(6.0))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(RichText::new(format!("⚠ Error: {}", err)).color(STATUS_FAILED).strong());
                                                if ui.button(RichText::new("🔄 Retry Now").strong()).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                    action_redownload = Some(idx);
                                                }
                                            });
                                        });
                                    ui.add_space(6.0);
                                } else if task.status == DownloadStatus::Paused {
                                    egui::Frame::none()
                                        .fill(Color32::from_rgb(40, 24, 18))
                                        .stroke(Stroke::new(1.0_f32, STATUS_PAUSED))
                                        .inner_margin(Margin::symmetric(10.0, 6.0))
                                        .rounding(egui::Rounding::same(6.0))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(RichText::new("⏸ Download paused").color(STATUS_PAUSED).strong());
                                                if ui.button(RichText::new("▶ Resume Download").strong()).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                    action_resume = Some(idx);
                                                }
                                            });
                                        });
                                    ui.add_space(6.0);
                                }

                                // Stream Matrix - Simple & Compact
                                let avail_width = ui.available_width().max(300.0);
                                let num_segs = task.segments.len();
                                let cols = if num_segs <= 4 {
                                    num_segs.max(1)
                                } else {
                                    ((avail_width / 135.0).floor() as usize).clamp(2, 8)
                                };
                                let gap = 6.0_f32;
                                let item_width = ((avail_width - ((cols - 1) as f32 * gap)) / cols as f32).max(110.0);

                                egui::ScrollArea::vertical()
                                    .max_height(80.0)
                                    .auto_shrink([false, true])
                                    .show(ui, |ui| {
                                        for chunk in task.segments.chunks(cols) {
                                            ui.horizontal(|ui| {
                                                ui.spacing_mut().item_spacing = Vec2::new(gap, 0.0);
                                                for seg in chunk {
                                                    let ratio = seg.progress_ratio();
                                                    let (status_color, status_text) = if seg.is_complete {
                                                        (STATUS_COMPLETED, "Done".to_string())
                                                    } else if ratio > 0.0 {
                                                        (STATUS_DOWNLOADING, format!("{:.0}%", ratio * 100.0))
                                                    } else {
                                                        (GLASS_MUTED, "-".to_string())
                                                    };

                                                    let dl_str = format_bytes(seg.downloaded_bytes);
                                                    let tot_str = format_bytes(seg.total_bytes());
                                                    let tooltip = format!("Part {}: {} / {} ({})", seg.index + 1, dl_str, tot_str, status_text);

                                                    egui::Frame::none()
                                                        .fill(Color32::from_rgba_unmultiplied(15, 23, 42, 200))
                                                        .stroke(Stroke::new(1.0_f32, if seg.is_complete { Color32::from_rgba_unmultiplied(16, 185, 129, 140) } else { GLASS_BORDER }))
                                                        .rounding(egui::Rounding::same(4.0))
                                                        .inner_margin(Margin::symmetric(7.0, 4.0))
                                                        .show(ui, |ui| {
                                                            ui.set_width(item_width - 14.0);
                                                            ui.horizontal(|ui| {
                                                                ui.label(RichText::new(format!("P{}", seg.index + 1)).size(10.5).color(GLASS_TEXT).strong());
                                                                let bar = egui::ProgressBar::new(ratio)
                                                                    .fill(if seg.is_complete { STATUS_COMPLETED } else { GLASS_PRIMARY })
                                                                    .desired_height(4.0)
                                                                    .desired_width(ui.available_width() - 36.0);
                                                                ui.add(bar);
                                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                                    ui.label(RichText::new(&status_text).size(9.5).color(status_color).strong());
                                                                });
                                                            });
                                                        }).response.on_hover_text(tooltip);
                                                }
                                            });
                                            ui.add_space(4.0);
                                        }
                                    });
                            }
                        }
                    }
                });
        }

        // Central Table Panel
        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(VELVET_SURFACE)
                    .inner_margin(Margin::symmetric(14.0, 8.0)),
            )
            .show(ctx, |ui| {
                let mut selected = self.selected_task_index;

                if let Ok(tasks) = self.tasks.try_lock() {
                    let filtered_indices: Vec<usize> = tasks.iter().enumerate().filter(|(_, task)| {
                        // Category filter
                        let cat_ok = match self.selected_filter {
                            FilterCategory::All => true,
                            FilterCategory::Active => matches!(task.status, DownloadStatus::Downloading | DownloadStatus::Queued),
                            FilterCategory::Paused => matches!(task.status, DownloadStatus::Paused | DownloadStatus::Failed(_)),
                            FilterCategory::Completed => task.status == DownloadStatus::Completed,
                        };
                        if !cat_ok { return false; }
                        // Search query filter
                        if !self.search_query.trim().is_empty() {
                            let q = self.search_query.trim().to_lowercase();
                            return task.filename.to_lowercase().contains(&q) || task.url.to_lowercase().contains(&q);
                        }
                        true
                    }).map(|(i, _)| i).collect();

                    if tasks.is_empty() {
                        selected = None;
                        self.selected_tasks.clear();
                        ui.vertical_centered(|ui| {
                            let avail_h = ui.available_height();
                            let content_h = 86.0_f32;
                            let top_space = ((avail_h - content_h) * 0.5).max(20.0);
                            ui.add_space(top_space);

                            let btn_size = Vec2::new(54.0, 54.0);
                            let (rect, resp) = ui.allocate_exact_size(btn_size, egui::Sense::click());
                            let hovered = resp.hovered();

                            if hovered {
                                ui.painter().rect_filled(
                                    rect.expand(3.0),
                                    egui::Rounding::same(18.0),
                                    Color32::from_rgba_unmultiplied(139, 92, 246, 45),
                                );
                            }

                            let bg_color = if hovered {
                                GLASS_PRIMARY
                            } else {
                                Color32::from_rgba_unmultiplied(139, 92, 246, 35)
                            };
                            let border_color = if hovered {
                                Color32::WHITE
                            } else {
                                GLASS_PRIMARY
                            };
                            let icon_color = if hovered {
                                Color32::WHITE
                            } else {
                                GLASS_PRIMARY
                            };

                            ui.painter().rect(
                                rect,
                                egui::Rounding::same(16.0),
                                bg_color,
                                Stroke::new(if hovered { 2.0_f32 } else { 1.5_f32 }, border_color),
                            );
                            draw_modern_icon(ui.painter(), ModernIcon::Plus, rect, icon_color);

                            let mut clicked = resp
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .on_hover_text("Add Download (Ctrl+N)")
                                .clicked();

                            ui.add_space(12.0);
                            let label_resp = ui.add(
                                egui::Label::new(
                                    RichText::new("Add new download")
                                        .size(13.5)
                                        .color(if hovered { GLASS_TEXT } else { GLASS_MUTED })
                                )
                                .sense(egui::Sense::click())
                            );
                            if label_resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                clicked = true;
                            }

                            if clicked {
                                do_open_add_dialog = true;
                            }
                        });
                    } else if filtered_indices.is_empty() {
                        selected = None;
                        self.selected_tasks.clear();
                        ui.vertical_centered(|ui| {
                            ui.add_space(80.0);
                            let (icon_rect, _) = ui.allocate_exact_size(Vec2::new(36.0, 36.0), egui::Sense::hover());
                            draw_modern_icon(ui.painter(), ModernIcon::Search, icon_rect, PINK_ROSE);
                            ui.add_space(12.0);
                            ui.label(RichText::new("No downloads match the current filter").size(16.0).color(PINK_PASTEL));
                        });
                    } else {
                        let total_width = ui.available_width();
                        let total_height = ui.available_height();
                        let progress_width = (180.0 + (total_width - 1000.0) * 0.12).clamp(160.0, 300.0);
                        let fixed_columns_sum = 32.0 + 85.0 + 100.0 + progress_width + 90.0 + 70.0 + 135.0;
                        let filename_width = (total_width - fixed_columns_sum - 10.0).max(220.0);

                        egui_extras::TableBuilder::new(ui)
                            .striped(true)
                            .max_scroll_height(total_height.max(300.0))
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(egui_extras::Column::exact(32.0)) // Checkbox
                            .column(egui_extras::Column::exact(filename_width).clip(true)) // Filename
                            .column(egui_extras::Column::exact(85.0))  // Size
                            .column(egui_extras::Column::exact(100.0)) // Status
                            .column(egui_extras::Column::exact(progress_width)) // Progress
                            .column(egui_extras::Column::exact(90.0))  // Speed
                            .column(egui_extras::Column::exact(70.0))  // ETA
                            .column(egui_extras::Column::exact(135.0)) // Actions
                                    .header(26.0, |mut header| {
                                        header.col(|ui| {
                                            let all_selected = !filtered_indices.is_empty() && filtered_indices.iter().all(|&i| {
                                                let id = &tasks[i].id;
                                                self.selected_tasks.contains(id)
                                            });
                                            let mut check = all_selected;
                                            if ui.checkbox(&mut check, "").clicked() {
                                                if check {
                                                    for &i in &filtered_indices {
                                                        self.selected_tasks.insert(tasks[i].id.clone());
                                                    }
                                                    if !filtered_indices.is_empty() {
                                                        selected = Some(filtered_indices[0]);
                                                    }
                                                } else {
                                                    for &i in &filtered_indices {
                                                        self.selected_tasks.remove(&tasks[i].id);
                                                    }
                                                    selected = None;
                                                }
                                            }
                                        });
                                        header.col(|ui| { ui.strong(RichText::new("Filename").color(PINK_PASTEL)); });
                                        header.col(|ui| { ui.strong(RichText::new("Size").color(PINK_PASTEL)); });
                                        header.col(|ui| { ui.strong(RichText::new("Status").color(PINK_PASTEL)); });
                                        header.col(|ui| { ui.strong(RichText::new("Progress").color(PINK_PASTEL)); });
                                        header.col(|ui| { ui.strong(RichText::new("Speed").color(PINK_PASTEL)); });
                                        header.col(|ui| { ui.strong(RichText::new("ETA").color(PINK_PASTEL)); });
                                        header.col(|ui| { ui.strong(RichText::new("Actions").color(PINK_PASTEL)); });
                                    })
                                    .body(|body| {
                                        body.rows(32.0, filtered_indices.len(), |mut row| {
                                            let filter_idx = row.index();
                                            let i = filtered_indices[filter_idx];
                                            let item = &tasks[i];
                                            let is_checked = self.selected_tasks.contains(&item.id);
                                            let is_sel = selected == Some(i) || is_checked;
                                            let is_downloading = item.status == DownloadStatus::Downloading;
                                            let is_resumable = matches!(item.status, DownloadStatus::Paused | DownloadStatus::Failed(_));
                                            let is_complete = item.status == DownloadStatus::Completed;

                                            row.set_selected(is_sel);
                                            
                                            // 0. Checkbox column
                                            row.col(|ui| {
                                                let mut check = is_checked;
                                                if ui.checkbox(&mut check, "").clicked() {
                                                    if check {
                                                        self.selected_tasks.insert(item.id.clone());
                                                        selected = Some(i);
                                                    } else {
                                                        self.selected_tasks.remove(&item.id);
                                                        if selected == Some(i) {
                                                            selected = self.selected_tasks.iter().find_map(|checked_id| {
                                                                tasks.iter().position(|t| &t.id == checked_id)
                                                            });
                                                        }
                                                    }
                                                }
                                            });

                                            // 1. Filename column with modern badge + click to select / double-click to open
                                            row.col(|ui| {
                                                ui.horizontal(|ui| {
                                                    let ext = item.filename.rsplit('.').next().unwrap_or("").to_lowercase();
                                                    render_file_type_badge(ui, &ext);
                                                    ui.add_space(4.0);

                                                    let name_col = if is_sel {
                                                        Color32::WHITE
                                                    } else if is_complete {
                                                        STATUS_COMPLETED
                                                    } else {
                                                        Color32::from_rgb(240, 232, 248)
                                                    };
                                                    let shaped_filename = shape_bengali(&item.filename);
                                                    let label_text = if is_complete {
                                                        RichText::new(&shaped_filename).color(name_col).strong().underline()
                                                    } else {
                                                        RichText::new(&shaped_filename).color(name_col).strong()
                                                    };
                                                    let resp = ui.add(egui::Label::new(label_text).sense(egui::Sense::click()).truncate())
                                                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                                                        .on_hover_text(if is_complete { "Click to select, double-click to open file" } else { "Click to select" });

                                                    if is_complete && resp.double_clicked() {
                                                        action_open_file = Some(i);
                                                    } else if resp.clicked() {
                                                        selected = Some(i);
                                                        self.selected_tasks.clear();
                                                        self.selected_tasks.insert(item.id.clone());
                                                    }

                                                    resp.context_menu(|ui| {
                                                        if is_complete {
                                                            if ui.button("▶  Open File").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                                action_open_file = Some(i);
                                                                ui.close_menu();
                                                            }
                                                        }
                                                        if is_downloading {
                                                            if ui.button("⏸  Pause").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                                action_pause = Some(i);
                                                                ui.close_menu();
                                                            }
                                                        } else if is_resumable {
                                                            if ui.button("▶  Resume").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                                action_resume = Some(i);
                                                                ui.close_menu();
                                                            }
                                                        }
                                                        if ui.button("📁 Open Containing Folder").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                            action_open_folder = Some(i);
                                                            ui.close_menu();
                                                        }
                                                        if ui.button("🔗 Copy URL").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                            action_copy_url = Some(item.url.clone());
                                                            ui.close_menu();
                                                        }
                                                        ui.separator();
                                                        if ui.button("🔄 Redownload").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                            action_redownload = Some(i);
                                                            ui.close_menu();
                                                        }
                                                        if ui.button(RichText::new("🗑 Delete").color(STATUS_FAILED)).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                            action_remove_ids.push(item.id.clone());
                                                            ui.close_menu();
                                                        }
                                                    });
                                                });
                                            });

                                            // 2. Size column
                                            row.col(|ui| {
                                                let sz_text = if let Some(total) = item.total_bytes {
                                                    format_bytes(total)
                                                } else if item.downloaded_bytes > 0 {
                                                    format_bytes(item.downloaded_bytes)
                                                } else {
                                                    "--".to_string()
                                                };
                                                let resp = ui.add(egui::Label::new(RichText::new(sz_text).color(Color32::from_rgb(226, 232, 240)).strong()).sense(egui::Sense::click()));
                                                if resp.clicked() {
                                                    selected = Some(i);
                                                    self.selected_tasks.clear();
                                                    self.selected_tasks.insert(item.id.clone());
                                                }
                                            });

                                            // 3. Status column
                                            row.col(|ui| {
                                                let (color, text, tooltip) = match &item.status {
                                                    DownloadStatus::Downloading => (STATUS_DOWNLOADING, "Downloading".to_string(), None),
                                                    DownloadStatus::Completed => (STATUS_COMPLETED, "Completed".to_string(), None),
                                                    DownloadStatus::Paused => (STATUS_PAUSED, "Paused".to_string(), Some("Download is paused. Click Resume to continue.".to_string())),
                                                    DownloadStatus::Failed(err) => (STATUS_FAILED, "Failed".to_string(), Some(format!("Failure reason: {}\nClick Resume/Retry to retry.", err))),
                                                    _ => (PINK_MUTED, "Queued".to_string(), None),
                                                };
                                                let mut label = ui.add(egui::Label::new(RichText::new(text).color(color).strong()).sense(egui::Sense::click()));
                                                if let Some(tip) = tooltip {
                                                    label = label.on_hover_text(tip);
                                                }
                                                if label.clicked() {
                                                    selected = Some(i);
                                                    self.selected_tasks.clear();
                                                    self.selected_tasks.insert(item.id.clone());
                                                }
                                            });

                                            // 4. Progress bar with high-contrast text and sleek bar
                                            row.col(|ui| {
                                                let ratio = (item.progress_percent / 100.0).clamp(0.0, 1.0);
                                                let (bar_color, pct_label) = match item.status {
                                                    DownloadStatus::Completed => (STATUS_COMPLETED, "100%".to_string()),
                                                    DownloadStatus::Downloading => (STATUS_DOWNLOADING, format!("{:.1}%", item.progress_percent)),
                                                    DownloadStatus::Paused => (STATUS_PAUSED, format!("{:.1}%", item.progress_percent)),
                                                    DownloadStatus::Failed(_) => (STATUS_FAILED, format!("{:.0}%", item.progress_percent)),
                                                    _ => (Color32::from_rgb(80, 50, 80), format!("{:.0}%", item.progress_percent)),
                                                };
                                                let resp = ui.horizontal(|ui| {
                                                    ui.spacing_mut().item_spacing = Vec2::new(8.0, 0.0);
                                                    let bar_width = (ui.available_width() - 50.0).max(40.0);
                                                    let bar = egui::ProgressBar::new(ratio)
                                                        .fill(bar_color)
                                                        .desired_width(bar_width)
                                                        .desired_height(7.0);
                                                    ui.add(bar);
                                                    ui.label(RichText::new(pct_label).color(Color32::WHITE).size(12.0).strong());
                                                }).response;
                                                let interact_resp = ui.interact(resp.rect, ui.id().with(("progress_click", i)), egui::Sense::click());
                                                if interact_resp.clicked() {
                                                    selected = Some(i);
                                                    self.selected_tasks.clear();
                                                    self.selected_tasks.insert(item.id.clone());
                                                }
                                            });

                                            // 5. Speed column - Electric Emerald Mint for high contrast
                                            row.col(|ui| {
                                                let speed_text = if item.status == DownloadStatus::Downloading {
                                                    format_speed(item.speed_bps)
                                                } else {
                                                    "--".to_string()
                                                };
                                                let resp = ui.add(egui::Label::new(RichText::new(speed_text).color(Color32::from_rgb(52, 211, 153)).strong()).sense(egui::Sense::click()));
                                                if resp.clicked() {
                                                    selected = Some(i);
                                                    self.selected_tasks.clear();
                                                    self.selected_tasks.insert(item.id.clone());
                                                }
                                            });

                                            // 6. ETA column - Crisp Light Slate
                                            row.col(|ui| {
                                                let eta_text = if item.status == DownloadStatus::Downloading {
                                                    if let Some(eta) = item.eta_seconds {
                                                        format_eta(eta)
                                                    } else {
                                                        "--".to_string()
                                                    }
                                                } else {
                                                    "--".to_string()
                                                };
                                                let resp = ui.add(egui::Label::new(RichText::new(eta_text).color(Color32::from_rgb(203, 213, 225)).strong()).sense(egui::Sense::click()));
                                                if resp.clicked() {
                                                    selected = Some(i);
                                                    self.selected_tasks.clear();
                                                    self.selected_tasks.insert(item.id.clone());
                                                }
                                            });

                                            // 7. Modern Vector Action Buttons
                                            row.col(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);
                                                    if is_downloading {
                                                        if modern_icon_button(ui, ModernIcon::Pause, Vec2::new(24.0, 22.0), STATUS_PAUSED, Color32::WHITE, "Pause") {
                                                            action_pause = Some(i);
                                                        }
                                                    } else if is_resumable {
                                                        if modern_icon_button(ui, ModernIcon::Play, Vec2::new(24.0, 22.0), STATUS_COMPLETED, Color32::WHITE, "Resume") {
                                                            action_resume = Some(i);
                                                        }
                                                    }

                                                    if modern_icon_button(ui, ModernIcon::Refresh, Vec2::new(24.0, 22.0), Color32::from_rgb(180, 195, 220), Color32::WHITE, "Redownload") {
                                                        action_redownload = Some(i);
                                                    }

                                                    if let DownloadStatus::Failed(_) = &item.status {
                                                        if item.url.contains("google") || item.url.contains("drive") || item.url.contains("takeout") {
                                                            if modern_icon_button(ui, ModernIcon::Globe, Vec2::new(24.0, 22.0), Color32::from_rgb(56, 189, 248), Color32::WHITE, "Bypass with Chromium Engine") {
                                                                launch_chrome_with_extension(Some(&item.url));
                                                            }
                                                        }
                                                    }

                                                    if modern_icon_button(ui, ModernIcon::Trash, Vec2::new(24.0, 22.0), STATUS_FAILED, Color32::WHITE, "Delete") {
                                                        action_remove_ids.push(item.id.clone());
                                                    }
                                                });
                                            });
                                        });
                                    });
                            }
                        }

                self.selected_task_index = selected;
            });

        // Modern Cyber-Obsidian Add Download Modal Dialog

fn get_quality_display(q: rapid_core::youtube::DownloadQuality) -> (&'static str, &'static str) {
    match q {
        rapid_core::youtube::DownloadQuality::Best => ("Best Available (1080p+)", "✨"),
        rapid_core::youtube::DownloadQuality::P1080 => ("1080p Full HD", "🎬"),
        rapid_core::youtube::DownloadQuality::P720 => ("720p HD", "📺"),
        rapid_core::youtube::DownloadQuality::P480 => ("480p SD", "📹"),
        rapid_core::youtube::DownloadQuality::P360 => ("360p", "📱"),
        rapid_core::youtube::DownloadQuality::AudioMp3 => ("Audio MP3", "🎵"),
        rapid_core::youtube::DownloadQuality::AudioM4a => ("Audio M4A", "🎧"),
    }
}

fn render_quality_selector_combobox(
    ui: &mut egui::Ui,
    id_source: &str,
    quality: &mut rapid_core::youtube::DownloadQuality,
    filename: &mut String,
    dest_dir: &mut String,
    base_download_dir: &PathBuf,
) {
    let (current_title, current_icon) = get_quality_display(*quality);

    let all_options = [
        rapid_core::youtube::DownloadQuality::Best,
        rapid_core::youtube::DownloadQuality::P1080,
        rapid_core::youtube::DownloadQuality::P720,
        rapid_core::youtube::DownloadQuality::P480,
        rapid_core::youtube::DownloadQuality::P360,
        rapid_core::youtube::DownloadQuality::AudioMp3,
        rapid_core::youtube::DownloadQuality::AudioM4a,
    ];

    ui.scope(|ui| {
        ui.visuals_mut().widgets.inactive.weak_bg_fill = Color32::from_rgb(18, 24, 44);
        ui.visuals_mut().widgets.inactive.bg_stroke = Stroke::new(1.0_f32, GLASS_BORDER);
        ui.visuals_mut().widgets.inactive.rounding = egui::Rounding::same(8.0);
        ui.visuals_mut().widgets.inactive.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);

        ui.visuals_mut().widgets.hovered.weak_bg_fill = Color32::from_rgb(26, 34, 60);
        ui.visuals_mut().widgets.hovered.bg_stroke = Stroke::new(1.2_f32, GLASS_PRIMARY);
        ui.visuals_mut().widgets.hovered.rounding = egui::Rounding::same(8.0);
        ui.visuals_mut().widgets.hovered.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);

        ui.visuals_mut().widgets.open.weak_bg_fill = Color32::from_rgb(32, 40, 72);
        ui.visuals_mut().widgets.open.bg_stroke = Stroke::new(1.5_f32, GLASS_PRIMARY);
        ui.visuals_mut().widgets.open.rounding = egui::Rounding::same(8.0);
        ui.visuals_mut().widgets.open.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);

        ui.spacing_mut().button_padding = Vec2::new(14.0, 9.0);

        ui.visuals_mut().window_fill = Color32::from_rgb(15, 20, 36);
        ui.visuals_mut().window_stroke = Stroke::new(1.2_f32, Color32::from_rgb(55, 70, 105));
        ui.visuals_mut().window_rounding = egui::Rounding::same(10.0);
        ui.visuals_mut().popup_shadow = egui::epaint::Shadow {
            offset: [0.0, 12.0].into(),
            blur: 32.0,
            spread: 3.0,
            color: Color32::from_black_alpha(245),
        };
        ui.spacing_mut().item_spacing = Vec2::new(0.0, 4.0);
        ui.spacing_mut().window_margin = Margin::same(8.0);

        let selected_text = format!("{}  {}", current_icon, current_title);

        let combo = egui::ComboBox::from_id_source(id_source)
            .selected_text(RichText::new(selected_text).size(14.0).color(Color32::WHITE).strong())
            .width(ui.available_width() - 4.0)
            .height(230.0) // Sized large so that 4 to 5 options are visible simultaneously!
            .icon(|ui, rect, visuals, _is_open, _above_or_below| {
                let center = rect.center();
                let stroke = Stroke::new(1.8_f32, visuals.fg_stroke.color);
                ui.painter().line_segment(
                    [egui::pos2(center.x - 4.5, center.y - 2.5), egui::pos2(center.x, center.y + 2.5)],
                    stroke,
                );
                ui.painter().line_segment(
                    [egui::pos2(center.x, center.y + 2.5), egui::pos2(center.x + 4.5, center.y - 2.5)],
                    stroke,
                );
            });

        let mut changed_quality: Option<rapid_core::youtube::DownloadQuality> = None;

        combo.show_ui(ui, |ui| {
            ui.set_min_width(ui.available_width().max(480.0));

            for opt in all_options {
                let (title, icon) = get_quality_display(opt);
                let is_selected = *quality == opt;

                let (bg, border, text_col, stroke_w) = if is_selected {
                    (
                        Color32::from_rgb(46, 32, 92),
                        Color32::from_rgb(168, 85, 247),
                        Color32::WHITE,
                        1.5_f32,
                    )
                } else {
                    (
                        Color32::from_rgba_unmultiplied(20, 26, 44, 200),
                        Color32::from_rgb(40, 50, 75),
                        GLASS_TEXT,
                        1.0_f32,
                    )
                };

                let item_frame = egui::Frame::none()
                    .fill(bg)
                    .stroke(Stroke::new(stroke_w, border))
                    .rounding(egui::Rounding::same(8.0))
                    .inner_margin(Margin::symmetric(14.0, 9.0));

                let item_resp = item_frame.show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.horizontal(|ui| {
                        if is_selected {
                            ui.label(RichText::new("✓").size(14.0).color(Color32::from_rgb(56, 189, 248)).strong());
                        } else {
                            ui.label(RichText::new(" ").size(14.0));
                        }
                        ui.add_space(4.0);
                        ui.label(RichText::new(icon).size(15.0));
                        ui.add_space(6.0);
                        ui.label(
                            RichText::new(title)
                                .size(14.0)
                                .color(text_col)
                                .strong(),
                        );
                    });
                }).response;

                let click_resp = ui.interact(
                    item_resp.rect,
                    ui.make_persistent_id(format!("{}_{:?}", id_source, opt)),
                    egui::Sense::click(),
                );
                if click_resp.hovered() {
                    ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                }
                if click_resp.clicked() {
                    changed_quality = Some(opt);
                }
                ui.add_space(3.0);
            }
        });

        if let Some(new_q) = changed_quality {
            *quality = new_q;
            let new_ext = new_q.target_extension();
            if !filename.is_empty() {
                let stem = if let Some(dot_idx) = filename.rfind('.') {
                    &filename[..dot_idx]
                } else {
                    filename.as_str()
                };
                *filename = format!("{}.{}", stem, new_ext);
                let cat_dest = get_categorized_destination(base_download_dir, filename);
                *dest_dir = cat_dest.to_string_lossy().to_string();
            }
        }
    });
}

fn render_parallel_connections_combobox(
    ui: &mut egui::Ui,
    id_source: &str,
    segments: &mut usize,
    is_gdrive: bool,
) {
    if is_gdrive {
        egui::Frame::none()
            .fill(Color32::from_rgb(18, 30, 48))
            .stroke(Stroke::new(1.0_f32, GLASS_SECONDARY))
            .rounding(egui::Rounding::same(6.0))
            .inner_margin(Margin::symmetric(10.0, 7.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("🛡️ 1 Stream (Google Drive Single Stream Mode Enforced)")
                            .size(12.5)
                            .color(GLASS_SECONDARY)
                            .strong(),
                    );
                });
            });
        return;
    }

    let stream_label = match *segments {
        1 => "1 Stream (Single • Safe)".to_string(),
        4 => "4 Streams (Eco • Balanced)".to_string(),
        8 => "8 Streams (Default • Recommended)".to_string(),
        16 => "16 Streams (Turbo Speed)".to_string(),
        32 => "32 Streams (Extreme Speed)".to_string(),
        n => format!("{} Streams", n),
    };

    let options = [
        (1, "1 Stream", "Single connection • Safe", "🛡️"),
        (4, "4 Streams", "Eco • Balanced connections", "🌱"),
        (8, "8 Streams", "Default • Recommended", "★"),
        (16, "16 Streams", "Turbo • High performance", "⚡"),
        (32, "32 Streams", "Extreme Speed • Max", "🚀"),
    ];

    ui.scope(|ui| {
        // Set widget visuals to match exactly the input fields (GLASS_BG, GLASS_BORDER, 6.0 rounding)
        ui.visuals_mut().widgets.inactive.weak_bg_fill = GLASS_BG;
        ui.visuals_mut().widgets.inactive.bg_stroke = Stroke::new(1.0_f32, GLASS_BORDER);
        ui.visuals_mut().widgets.inactive.rounding = egui::Rounding::same(6.0);
        ui.visuals_mut().widgets.inactive.fg_stroke = Stroke::new(1.0_f32, GLASS_TEXT);

        ui.visuals_mut().widgets.hovered.weak_bg_fill = Color32::from_rgb(16, 24, 46);
        ui.visuals_mut().widgets.hovered.bg_stroke = Stroke::new(1.0_f32, GLASS_PRIMARY);
        ui.visuals_mut().widgets.hovered.rounding = egui::Rounding::same(6.0);
        ui.visuals_mut().widgets.hovered.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);

        ui.visuals_mut().widgets.open.weak_bg_fill = Color32::from_rgb(20, 28, 54);
        ui.visuals_mut().widgets.open.bg_stroke = Stroke::new(1.5_f32, GLASS_PRIMARY);
        ui.visuals_mut().widgets.open.rounding = egui::Rounding::same(6.0);
        ui.visuals_mut().widgets.open.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);

        ui.spacing_mut().button_padding = Vec2::new(10.0, 7.0);

        // Modern luxury dropdown popup styling
        ui.visuals_mut().window_fill = Color32::from_rgb(15, 23, 42); // GLASS_SURFACE
        ui.visuals_mut().window_stroke = Stroke::new(1.0_f32, Color32::from_rgb(45, 60, 95));
        ui.visuals_mut().window_rounding = egui::Rounding::same(8.0);
        ui.visuals_mut().popup_shadow = egui::epaint::Shadow {
            offset: [0.0, 10.0].into(),
            blur: 28.0,
            spread: 2.0,
            color: Color32::from_black_alpha(240),
        };
        ui.spacing_mut().item_spacing = Vec2::new(0.0, 3.0);
        ui.spacing_mut().window_margin = Margin::same(6.0);

        let combo = egui::ComboBox::from_id_source(id_source)
            .selected_text(RichText::new(stream_label).size(12.5).color(GLASS_TEXT).strong())
            .width(ui.available_width() - 4.0)
            .height(32.0)
            .icon(|ui, rect, visuals, _is_open, _above_or_below| {
                let center = rect.center();
                let stroke = Stroke::new(1.5_f32, visuals.fg_stroke.color);
                ui.painter().line_segment(
                    [egui::pos2(center.x - 4.0, center.y - 2.0), egui::pos2(center.x, center.y + 2.5)],
                    stroke,
                );
                ui.painter().line_segment(
                    [egui::pos2(center.x, center.y + 2.5), egui::pos2(center.x + 4.0, center.y - 2.0)],
                    stroke,
                );
            });

        combo.show_ui(ui, |ui| {
            ui.set_min_width(ui.available_width().max(420.0));
            for (val, title, desc, icon) in options {
                let is_selected = *segments == val;
                let (bg, border, text_col) = if is_selected {
                    (
                        Color32::from_rgb(42, 26, 85),
                        Color32::from_rgb(140, 95, 245),
                        Color32::WHITE,
                    )
                } else {
                    (
                        Color32::TRANSPARENT,
                        Color32::TRANSPARENT,
                        GLASS_TEXT,
                    )
                };

                let item_frame = egui::Frame::none()
                    .fill(bg)
                    .stroke(Stroke::new(if is_selected { 1.0_f32 } else { 0.0_f32 }, border))
                    .rounding(egui::Rounding::same(6.0))
                    .inner_margin(Margin::symmetric(10.0, 7.0))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(icon).size(13.0));
                            ui.add_space(4.0);
                            ui.label(RichText::new(title).size(12.5).color(text_col).strong());
                            ui.add_space(6.0);
                            ui.label(
                                RichText::new(format!("•  {}", desc))
                                    .size(11.5)
                                    .color(if is_selected { Color32::from_rgb(200, 180, 255) } else { GLASS_MUTED }),
                            );

                            if is_selected {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(
                                        RichText::new("✓")
                                            .size(13.0)
                                            .color(Color32::from_rgb(160, 115, 255))
                                            .strong(),
                                    );
                                });
                            }
                        });
                    });

                let resp = ui.interact(
                    item_frame.response.rect,
                    ui.make_persistent_id(format!("{}_{}", id_source, val)),
                    egui::Sense::click(),
                );
                if resp.hovered() && !is_selected {
                    ui.painter().rect_filled(resp.rect, 6.0, Color32::from_rgba_unmultiplied(255, 255, 255, 15));
                }
                if resp.hovered() {
                    ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                }
                if resp.clicked() {
                    *segments = val;
                    ui.memory_mut(|mem| mem.close_popup());
                }
            }
        });
    });
}

        if do_open_add_dialog {
            self.open_add_download_dialog(ctx);
        }

        if self.show_add_dialog {
            // Backdrop dimming scrim - strong solid backdrop
            let screen_rect = ctx.screen_rect();
            let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::new("modal_backdrop")));
            painter.rect_filled(screen_rect, 0.0, Color32::from_rgba_unmultiplied(2, 6, 23, 248));

            let modal_frame = egui::Frame::none()
                .fill(Color32::from_rgb(11, 17, 32))
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(30, 41, 59)))
                .rounding(egui::Rounding::ZERO)
                .inner_margin(Margin::ZERO)
                .shadow(egui::epaint::Shadow {
                    offset: [0.0, 10.0].into(),
                    blur: 32.0,
                    spread: 2.0,
                    color: Color32::from_black_alpha(230),
                });

            let mut close_modal = false;
            let mut start_download_req = false;
            let mut url_to_probe: Option<String> = None;

            let is_input_yt = rapid_core::youtube::YoutubeResolver::is_youtube(&self.input_url)
                || rapid_core::youtube::YoutubeResolver::is_extractable_platform(&self.input_url);
            let add_modal_w = if is_input_yt { 680.0 } else { 600.0 };
            let add_modal_h = if is_input_yt { 580.0 } else { 365.0 };

            egui::Window::new("add_download_modal")
                .title_bar(false)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .fixed_size(Vec2::new(add_modal_w, add_modal_h))
                .frame(modal_frame)
                .show(ctx, |ui| {
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    ui.spacing_mut().window_margin = Margin::ZERO;

                    let win_rect = ui.max_rect();
                    let close_w = 46.0_f32;
                    let close_h = 30.0_f32;
                    let close_rect = egui::Rect::from_min_max(
                        egui::pos2(win_rect.max.x - close_w, win_rect.min.y),
                        egui::pos2(win_rect.max.x, win_rect.min.y + close_h),
                    );

                    let close_resp = ui.interact(close_rect, ui.make_persistent_id("add_modal_close_btn_top_right"), egui::Sense::click());
                    let is_close_hovered = close_resp.hovered();
                    if is_close_hovered {
                        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                    }
                    if close_resp.clicked() {
                        close_modal = true;
                    }

                    // Background: Red (bright red on hover)
                    let bg_color = if is_close_hovered {
                        Color32::from_rgb(255, 60, 60)
                    } else {
                        Color32::from_rgb(220, 38, 38)
                    };

                    // Border radius removed as requested: 0.0
                    ui.painter().rect_filled(close_rect, egui::Rounding::ZERO, bg_color);

                    // Icon: White 'X'
                    let icon_center = close_rect.center();
                    let stroke = Stroke::new(1.8_f32, Color32::WHITE);
                    let sz = 4.5_f32;
                    ui.painter().line_segment(
                        [egui::pos2(icon_center.x - sz, icon_center.y - sz), egui::pos2(icon_center.x + sz, icon_center.y + sz)],
                        stroke,
                    );
                    ui.painter().line_segment(
                        [egui::pos2(icon_center.x + sz, icon_center.y - sz), egui::pos2(icon_center.x - sz, icon_center.y + sz)],
                        stroke,
                    );

                    ui.add_space(8.0);

                    egui::Frame::none()
                        .inner_margin(Margin { left: 22.0, right: 22.0, top: 4.0, bottom: 20.0 })
                        .show(ui, |ui| {
                            // Field 1: Source URL + Copy Button (Right-aligned center)
                            ui.label(RichText::new("Source URL").size(12.5).color(GLASS_TEXT).strong());
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                egui::Frame::none()
                                    .fill(GLASS_BG)
                                    .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                                    .rounding(egui::Rounding::same(6.0))
                                    .inner_margin(Margin::symmetric(10.0, 7.0))
                                    .show(ui, |ui| {
                                        let prev_url = self.input_url.clone();
                                        let edit = egui::TextEdit::singleline(&mut self.input_url)
                                            .hint_text("Paste URL here e.g. https://example.com/file.zip")
                                            .font(egui::TextStyle::Body)
                                            .frame(false)
                                            .desired_width(ui.available_width() - 85.0);
                                        ui.add(edit);

                                        if self.input_url != prev_url {
                                            if rapid_core::youtube::YoutubeResolver::is_extractable_platform(&self.input_url) {
                                                self.input_filename = "Resolving video title...".to_string();
                                                let cat_dest = get_categorized_destination(&self.base_download_dir, "video.mp4");
                                                self.input_dest = cat_dest.to_string_lossy().to_string();
                                            } else {
                                                let candidate = extract_filename_from_url(&self.input_url);
                                                if !candidate.is_empty() && !rapid_core::engine::is_generic_placeholder(&candidate) {
                                                    self.input_filename = candidate.clone();
                                                    let cat_dest = get_categorized_destination(&self.base_download_dir, &candidate);
                                                    self.input_dest = cat_dest.to_string_lossy().to_string();
                                                } else {
                                                    self.input_filename.clear();
                                                    self.input_dest = self.base_download_dir.to_string_lossy().to_string();
                                                }
                                            }
                                            if self.input_url.starts_with("http://") || self.input_url.starts_with("https://") {
                                                url_to_probe = Some(self.input_url.clone());
                                            }
                                        }
                                    });

                                let copy_btn = egui::Button::new(
                                    RichText::new("Copy")
                                        .size(12.0)
                                        .color(GLASS_SECONDARY)
                                        .strong(),
                                )
                                .min_size(Vec2::new(75.0, 32.0))
                                .fill(GLASS_CARD)
                                .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                                .rounding(egui::Rounding::same(6.0));

                                if ui.add(copy_btn).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Copy Source URL to clipboard").clicked() {
                                    ui.output_mut(|o| o.copied_text = self.input_url.clone());
                                }
                            });

                            ui.add_space(10.0);

                            // Field 2: File Name (Full width)
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("File Name").size(12.5).color(GLASS_TEXT).strong());
                                let ext = std::path::Path::new(&self.input_filename)
                                    .extension()
                                    .and_then(|e| e.to_str())
                                    .unwrap_or("")
                                    .to_lowercase();
                                if !ext.is_empty() && ext != "bin" && ext != "tmp" {
                                    render_file_type_badge(ui, &ext);
                                }
                            });
                            ui.add_space(4.0);
                            egui::Frame::none()
                                .fill(GLASS_BG)
                                .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                                .rounding(egui::Rounding::same(6.0))
                                .inner_margin(Margin::symmetric(10.0, 7.0))
                                .show(ui, |ui| {
                                    let edit = egui::TextEdit::singleline(&mut self.input_filename)
                                        .hint_text(if self.input_url.is_empty() { "Enter file name or paste URL" } else { "Resolving original filename..." })
                                        .font(egui::TextStyle::Body)
                                        .frame(false)
                                        .desired_width(ui.available_width());
                                    if ui.add(edit).changed() {
                                        if !self.input_filename.trim().is_empty() {
                                            let cat_dest = get_categorized_destination(&self.base_download_dir, &self.input_filename);
                                            self.input_dest = cat_dest.to_string_lossy().to_string();
                                        }
                                    }
                                });

                            ui.add_space(10.0);

                            // Field 3: Save To + Browse Button (Right-aligned center)
                            ui.label(RichText::new("Save To").size(12.5).color(GLASS_TEXT).strong());
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                egui::Frame::none()
                                    .fill(GLASS_BG)
                                    .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                                    .rounding(egui::Rounding::same(6.0))
                                    .inner_margin(Margin::symmetric(10.0, 7.0))
                                    .show(ui, |ui| {
                                        let edit = egui::TextEdit::singleline(&mut self.input_dest)
                                            .font(egui::TextStyle::Body)
                                            .frame(false)
                                            .desired_width(ui.available_width() - 85.0);
                                        ui.add(edit);
                                    });

                                let browse_btn = egui::Button::new(
                                    RichText::new("Browse...")
                                        .size(12.0)
                                        .color(GLASS_SECONDARY)
                                        .strong(),
                                )
                                .min_size(Vec2::new(75.0, 32.0))
                                .fill(GLASS_CARD)
                                .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                                .rounding(egui::Rounding::same(6.0));

                                if ui.add(browse_btn).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Select download folder").clicked() {
                                    let start_dir = PathBuf::from(&self.input_dest);
                                    let mut dialog = rfd::FileDialog::new();
                                    if start_dir.exists() {
                                        dialog = dialog.set_directory(&start_dir);
                                    }
                                    if let Some(folder) = dialog.pick_folder() {
                                        self.input_dest = folder.to_string_lossy().to_string();
                                    }
                                }
                            });

                            if is_input_yt {
                                ui.add_space(10.0);
                                ui.label(
                                    RichText::new("Quality / Format")
                                        .size(12.5)
                                        .color(GLASS_TEXT)
                                        .strong(),
                                );
                                ui.add_space(4.0);
                                render_quality_selector_combobox(
                                    ui,
                                    "add_modal_quality_select",
                                    &mut self.input_quality,
                                    &mut self.input_filename,
                                    &mut self.input_dest,
                                    &self.base_download_dir,
                                );
                            }

                            ui.add_space(10.0);

                            // Field 4: Parallel Connections (Modern Dropdown Selector)
                            let is_gdrive = self.input_url.contains("drive.google.com")
                                || self.input_url.contains("googleusercontent.com")
                                || self.input_url.contains("docs.google.com");
                            ui.label(
                                RichText::new("Parallel Connections")
                                    .size(12.5)
                                    .color(GLASS_TEXT)
                                    .strong(),
                            );
                            ui.add_space(4.0);
                            render_parallel_connections_combobox(ui, "add_modal_segments_select", &mut self.input_segments, is_gdrive);

                            // Error Message Banner (if any)
                            if let Some(ref err) = self.add_error {
                                ui.add_space(8.0);
                                egui::Frame::none()
                                    .fill(Color32::from_rgb(45, 18, 24))
                                    .stroke(Stroke::new(1.0_f32, STATUS_FAILED))
                                    .rounding(egui::Rounding::same(6.0))
                                    .inner_margin(Margin::symmetric(10.0, 6.0))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new("!").color(STATUS_FAILED).strong());
                                            ui.label(RichText::new(err).color(Color32::from_rgb(255, 180, 190)).size(12.0));
                                        });
                                    });
                            }

                            let is_blob_input = self.input_url.trim().starts_with("blob:");
                            if is_blob_input {
                                ui.add_space(6.0);
                                egui::Frame::none()
                                    .fill(Color32::from_rgba_unmultiplied(239, 68, 68, 35))
                                    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(239, 68, 68)))
                                    .rounding(egui::Rounding::same(6.0))
                                    .inner_margin(egui::Margin::symmetric(10.0, 6.0))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new("⚠ Browser Media Stream (blob:)").color(Color32::from_rgb(248, 113, 113)).size(12.0).strong());
                                        });
                                        ui.label(RichText::new("Browser memory buffer cannot be downloaded directly. Please play 1-2 seconds of the video in Chrome so Rapid can capture the direct high-speed stream.").color(Color32::from_rgb(252, 165, 165)).size(11.0));
                                    });
                            }

                            ui.add_space(14.0);
                            ui.separator();
                            ui.add_space(10.0);

                            // Modal Footer - Prominent Start Download Button
                            ui.horizontal(|ui| {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let btn_color = if is_blob_input {
                                        Color32::from_rgb(71, 85, 105)
                                    } else {
                                        GLASS_PRIMARY
                                    };
                                    let start_btn = egui::Button::new(
                                        RichText::new("Start Download")
                                            .size(13.0)
                                            .color(Color32::WHITE)
                                            .strong(),
                                    )
                                    .min_size(Vec2::new(140.0, 34.0))
                                    .fill(btn_color)
                                    .rounding(egui::Rounding::same(7.0));

                                    let resp = ui.add_enabled(!is_blob_input, start_btn);
                                    if is_blob_input {
                                        resp.on_hover_text("Cannot download internal browser blob memory. Play video in Chrome to capture direct stream.");
                                    } else {
                                        if resp.on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Start multi-threaded accelerated download").clicked() {
                                            start_download_req = true;
                                        }
                                    }
                                });
                            });
                        });
                });
            if let Some(ref u) = url_to_probe {
                self.probe_url_filename(u, ctx);
            }

            if close_modal {
                self.show_add_dialog = false;
                self.add_error = None;
                self.input_filename.clear();
                ctx.request_repaint();
            }

            if start_download_req {
                if self.input_url.trim().is_empty() {
                    self.add_error = Some("Please enter a valid download URL (http:// or https://)".to_string());
                } else {
                    let url = self.input_url.trim().to_string();
                    let dest = PathBuf::from(&self.input_dest);
                    let segs = self.input_segments;
                    let custom_fname = if self.input_filename.trim().is_empty()
                        || self.input_filename == "Resolving video title..."
                        || self.input_filename == "Resolving filename..."
                        || self.input_filename == "Resolving original filename..."
                        || self.input_filename == "Detecting filename..."
                        || rapid_core::engine::is_generic_placeholder(&self.input_filename) {
                        None
                    } else {
                        Some(self.input_filename.trim().to_string())
                    };
                    let q = self.input_quality;
                    self.start_new_download(url, dest, segs, custom_fname, Some(q));
                    self.show_add_dialog = false;
                    self.input_url.clear();
                    self.input_filename.clear();
                    ctx.request_repaint();
                }
            }
        }

        // IDM-Style Browser Download Confirmation Modal
        if self.current_browser_prompt.is_some() {
            self.show_add_dialog = false;
            // Backdrop dimming scrim - strong solid backdrop
            let screen_rect = ctx.screen_rect();
            let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::new("browser_prompt_backdrop")));
            painter.rect_filled(screen_rect, 0.0, Color32::from_rgba_unmultiplied(2, 6, 23, 248));

            let modal_frame = egui::Frame::none()
                .fill(Color32::from_rgb(11, 17, 32))
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(30, 41, 59)))
                .rounding(egui::Rounding::ZERO)
                .inner_margin(Margin::ZERO)
                .shadow(egui::epaint::Shadow {
                    offset: [0.0, 10.0].into(),
                    blur: 32.0,
                    spread: 2.0,
                    color: Color32::from_black_alpha(230),
                });

            let mut close_modal = false;
            let mut start_download_req = false;

            let is_prompt_yt = {
                let p = self.current_browser_prompt.as_ref().unwrap();
                p.is_youtube
                    || rapid_core::youtube::YoutubeResolver::is_youtube(&p.url)
                    || rapid_core::youtube::YoutubeResolver::is_extractable_platform(&p.url)
            };
            let browser_modal_w = if is_prompt_yt { 680.0 } else { 600.0 };
            let browser_modal_h = if is_prompt_yt { 580.0 } else { 355.0 };

            egui::Window::new("browser_download_prompt_modal")
                .title_bar(false)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .fixed_size(Vec2::new(browser_modal_w, browser_modal_h))
                .frame(modal_frame)
                .show(ctx, |ui| {
                    let prompt = self.current_browser_prompt.as_mut().unwrap();

                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    ui.spacing_mut().window_margin = Margin::ZERO;

                    let win_rect = ui.max_rect();
                    let close_w = 46.0_f32;
                    let close_h = 30.0_f32;
                    let close_rect = egui::Rect::from_min_max(
                        egui::pos2(win_rect.max.x - close_w, win_rect.min.y),
                        egui::pos2(win_rect.max.x, win_rect.min.y + close_h),
                    );

                    let close_resp = ui.interact(close_rect, ui.make_persistent_id("modal_close_btn_top_right"), egui::Sense::click());
                    let is_close_hovered = close_resp.hovered();
                    if is_close_hovered {
                        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                    }
                    if close_resp.clicked() {
                        close_modal = true;
                    }

                    // Background: Red (bright red on hover)
                    let bg_color = if is_close_hovered {
                        Color32::from_rgb(255, 60, 60)
                    } else {
                        Color32::from_rgb(220, 38, 38)
                    };

                    // Border radius removed: 0.0
                    ui.painter().rect_filled(close_rect, egui::Rounding::ZERO, bg_color);

                    // Icon: White 'X'
                    let icon_center = close_rect.center();
                    let stroke = Stroke::new(1.8_f32, Color32::WHITE);
                    let sz = 4.5_f32;
                    ui.painter().line_segment(
                        [egui::pos2(icon_center.x - sz, icon_center.y - sz), egui::pos2(icon_center.x + sz, icon_center.y + sz)],
                        stroke,
                    );
                    ui.painter().line_segment(
                        [egui::pos2(icon_center.x + sz, icon_center.y - sz), egui::pos2(icon_center.x - sz, icon_center.y + sz)],
                        stroke,
                    );

                    ui.add_space(8.0);
                    egui::Frame::none()
                        .inner_margin(Margin { left: 22.0, right: 22.0, top: 4.0, bottom: 20.0 })
                        .show(ui, |ui| {
                            // Field 1: Source URL + Copy Button (Right-aligned center)
                            ui.label(RichText::new("Source URL").size(12.5).color(GLASS_TEXT).strong());
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                egui::Frame::none()
                                    .fill(GLASS_BG)
                                    .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                                    .rounding(egui::Rounding::same(6.0))
                                    .inner_margin(Margin::symmetric(10.0, 7.0))
                                    .show(ui, |ui| {
                                        let edit = egui::TextEdit::singleline(&mut prompt.url)
                                            .font(egui::TextStyle::Body)
                                            .frame(false)
                                            .desired_width(ui.available_width() - 85.0);
                                        ui.add(edit);
                                    });

                                let copy_btn = egui::Button::new(
                                    RichText::new("Copy")
                                        .size(12.0)
                                        .color(GLASS_SECONDARY)
                                        .strong(),
                                )
                                .min_size(Vec2::new(75.0, 32.0))
                                .fill(GLASS_CARD)
                                .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                                .rounding(egui::Rounding::same(6.0));

                                if ui.add(copy_btn).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Copy Source URL to clipboard").clicked() {
                                    ui.output_mut(|o| o.copied_text = prompt.url.clone());
                                }
                            });

                            ui.add_space(10.0);

                            // Field 2: File Name (Full width)
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("File Name").size(12.5).color(GLASS_TEXT).strong());
                                let ext = std::path::Path::new(&prompt.filename)
                                    .extension()
                                    .and_then(|e| e.to_str())
                                    .unwrap_or("")
                                    .to_lowercase();
                                if !ext.is_empty() && ext != "bin" && ext != "tmp" {
                                    render_file_type_badge(ui, &ext);
                                }
                            });
                            ui.add_space(4.0);
                            egui::Frame::none()
                                .fill(GLASS_BG)
                                .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                                .rounding(egui::Rounding::same(6.0))
                                .inner_margin(Margin::symmetric(10.0, 7.0))
                                .show(ui, |ui| {
                                    let edit = egui::TextEdit::singleline(&mut prompt.filename)
                                        .hint_text("Resolving original filename...")
                                        .font(egui::TextStyle::Body)
                                        .frame(false)
                                        .desired_width(ui.available_width());
                                    if ui.add(edit).changed() {
                                        if !prompt.filename.trim().is_empty() {
                                            let cat_dest = get_categorized_destination(&self.base_download_dir, &prompt.filename);
                                            prompt.dest_dir = cat_dest.to_string_lossy().to_string();
                                        }
                                    }
                                });

                            ui.add_space(10.0);

                            // Field 3: Save To + Browse Button (Right-aligned center)
                            ui.label(RichText::new("Save To").size(12.5).color(GLASS_TEXT).strong());
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                egui::Frame::none()
                                    .fill(GLASS_BG)
                                    .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                                    .rounding(egui::Rounding::same(6.0))
                                    .inner_margin(Margin::symmetric(10.0, 7.0))
                                    .show(ui, |ui| {
                                        let edit = egui::TextEdit::singleline(&mut prompt.dest_dir)
                                            .font(egui::TextStyle::Body)
                                            .frame(false)
                                            .desired_width(ui.available_width() - 85.0);
                                        ui.add(edit);
                                    });

                                let browse_btn = egui::Button::new(
                                    RichText::new("Browse...")
                                        .size(12.0)
                                        .color(GLASS_SECONDARY)
                                        .strong(),
                                )
                                .min_size(Vec2::new(75.0, 32.0))
                                .fill(GLASS_CARD)
                                .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                                .rounding(egui::Rounding::same(6.0));

                                if ui.add(browse_btn).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Select download folder").clicked() {
                                    let start_dir = PathBuf::from(&prompt.dest_dir);
                                    let mut dialog = rfd::FileDialog::new();
                                    if start_dir.exists() {
                                        dialog = dialog.set_directory(&start_dir);
                                    }
                                    if let Some(folder) = dialog.pick_folder() {
                                        prompt.dest_dir = folder.to_string_lossy().to_string();
                                    }
                                }
                            });

                            if is_prompt_yt {
                                ui.add_space(10.0);
                                ui.label(
                                    RichText::new("Quality / Format")
                                        .size(12.5)
                                        .color(GLASS_TEXT)
                                        .strong(),
                                );
                                ui.add_space(4.0);
                                render_quality_selector_combobox(
                                    ui,
                                    "browser_prompt_quality_select",
                                    &mut prompt.quality,
                                    &mut prompt.filename,
                                    &mut prompt.dest_dir,
                                    &self.base_download_dir,
                                );
                            }

                            ui.add_space(10.0);

                            // Field 4: Parallel Connections (Modern Dropdown Selector)
                            ui.label(
                                RichText::new("Parallel Connections")
                                    .size(12.5)
                                    .color(GLASS_TEXT)
                                    .strong(),
                            );
                            ui.add_space(4.0);
                            render_parallel_connections_combobox(ui, "browser_prompt_segments_select", &mut prompt.segments, prompt.is_gdrive);

                            let is_prompt_blob = prompt.url.starts_with("blob:");
                            if is_prompt_blob {
                                ui.add_space(6.0);
                                egui::Frame::none()
                                    .fill(Color32::from_rgba_unmultiplied(239, 68, 68, 35))
                                    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(239, 68, 68)))
                                    .rounding(egui::Rounding::same(6.0))
                                    .inner_margin(egui::Margin::symmetric(10.0, 6.0))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new("⚠ Browser Media Stream (blob:)").color(Color32::from_rgb(248, 113, 113)).size(12.0).strong());
                                        });
                                        ui.label(RichText::new("Browser memory buffer cannot be downloaded directly. Please play 1-2 seconds of the video in Chrome so Rapid can capture the direct high-speed stream.").color(Color32::from_rgb(252, 165, 165)).size(11.0));
                                    });
                            }

                            ui.add_space(14.0);
                            ui.separator();
                            ui.add_space(10.0);

                            // Modal Footer - Prominent Start Download Button
                            ui.horizontal(|ui| {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let btn_color = if is_prompt_blob {
                                        Color32::from_rgb(71, 85, 105)
                                    } else {
                                        GLASS_PRIMARY
                                    };
                                    let start_btn = egui::Button::new(
                                        RichText::new("Start Download")
                                            .size(13.0)
                                            .color(Color32::WHITE)
                                            .strong(),
                                    )
                                    .min_size(Vec2::new(140.0, 34.0))
                                    .fill(btn_color)
                                    .rounding(egui::Rounding::same(7.0));

                                    let resp = ui.add_enabled(!is_prompt_blob, start_btn);
                                    if is_prompt_blob {
                                        resp.on_hover_text("Cannot download internal browser blob memory. Play video in Chrome to capture direct stream.");
                                    } else {
                                        if resp.on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Start multi-threaded accelerated download").clicked() {
                                            start_download_req = true;
                                        }
                                    }
                                });
                            });
                        });
                });

            if close_modal {
                if let Some(prompt) = self.current_browser_prompt.take() {
                    let prompt_url = prompt.url.clone();
                    if let Ok(mut q) = self.pending_browser_queue.lock() {
                        q.retain(|item| item.url != prompt_url);
                    }
                }
                self.current_browser_prompt = None;
                ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::Normal));
                ctx.request_repaint();
            }

            if start_download_req {
                ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::Normal));
                if let Some(prompt) = self.current_browser_prompt.take() {
                    let prompt_url = prompt.url.clone();
                    let url = prompt.url;
                    let dest = PathBuf::from(&prompt.dest_dir);
                    let segs = prompt.segments;
                    let custom_fn = if prompt.filename.trim().is_empty()
                        || prompt.filename == "Downloading..."
                        || prompt.filename == "Resolving filename..."
                        || prompt.filename == "Resolving original filename..."
                        || prompt.filename == "Detecting filename..."
                        || rapid_core::engine::is_generic_placeholder(&prompt.filename) {
                        None
                    } else {
                        Some(prompt.filename.trim().to_string())
                    };
                    let cookies = prompt.cookies;
                    let ua = prompt.user_agent;
                    let referrer = prompt.referrer;
                    let is_gd = prompt.is_gdrive;
                    let quality = prompt.quality;

                    // Deduplicate any other pending items with this URL so modal never re-pops
                    if let Ok(mut q) = self.pending_browser_queue.lock() {
                        q.retain(|item| item.url != prompt_url);
                    }

                    let is_vp = prompt.is_youtube;
                    spawn_download_task(
                        url,
                        dest,
                        segs,
                        custom_fn,
                        cookies,
                        ua,
                        referrer,
                        is_gd,
                        is_vp,
                        Some(Arc::clone(&self.speed_limit_bps)),
                        Some(quality),
                        Arc::clone(&self.tokio_rt),
                        Arc::clone(&self.tasks),
                    );
                }
                self.current_browser_prompt = None;
                ctx.request_repaint();
            }
        }

        // Execute deferred actions outside of lock
        if let Some(idx) = action_resume {
            self.resume_download(idx);
        }
        if let Some(idx) = action_pause {
            self.pause_download(idx);
        }
        if let Some(idx) = action_redownload {
            self.redownload(idx);
        }
        if !action_remove_ids.is_empty() {
            self.tasks_to_delete = action_remove_ids.into_iter().collect();
            self.show_delete_modal = true;
        }
        if let Some(idx) = action_open_folder {
            self.open_download_folder(idx);
        }
        if let Some(idx) = action_open_file {
            self.open_download_file(idx);
        }
        if let Some(url) = action_copy_url {
            ctx.output_mut(|o| o.copied_text = url);
        }

        if do_pause_all {
            self.pause_all();
        }
        if do_resume_all {
            self.resume_all();
        }
        if self.show_delete_modal {
            let screen_rect = ctx.screen_rect();
            let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::new("delete_modal_backdrop")));
            painter.rect_filled(screen_rect, 0.0, Color32::from_rgba_unmultiplied(2, 6, 23, 220));

            let mut close_modal = false;
            let mut do_history_delete = false;
            let mut do_permanent_delete = false;
            
            let modal_frame = egui::Frame::none()
                .fill(GLASS_SURFACE)
                .stroke(Stroke::NONE)
                .rounding(egui::Rounding::same(12.0))
                .inner_margin(Margin::same(0.0)); // No inner margin so close btn can be at edge

            egui::Window::new("Confirm Delete")
                .frame(modal_frame)
                .title_bar(false)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.allocate_ui(Vec2::new(400.0, 240.0), |ui| {
                        // Absolute positioned Close button at top right
                        let rect = ui.max_rect();
                        let close_rect = egui::Rect::from_min_size(
                            rect.right_top() + Vec2::new(-36.0, 12.0),
                            Vec2::new(24.0, 24.0),
                        );
                        ui.allocate_ui_at_rect(close_rect, |ui| {
                            if modern_icon_button(ui, ModernIcon::Close, Vec2::new(24.0, 24.0), GLASS_MUTED, PINK_NEON, "Close") {
                                close_modal = true;
                            }
                        });

                        // Main Content (Centered vertically & horizontally)
                        ui.vertical_centered(|ui| {
                            ui.add_space(25.0);
                            
                            // Trash Icon inside a subtle circle
                            let (icon_rect, _) = ui.allocate_exact_size(Vec2::new(48.0, 48.0), egui::Sense::hover());
                            ui.painter().circle_filled(icon_rect.center(), 24.0, Color32::from_rgba_unmultiplied(255, 60, 60, 20));
                            let trash_rect = egui::Rect::from_center_size(icon_rect.center(), Vec2::new(24.0, 24.0));
                            draw_modern_icon(ui.painter(), ModernIcon::Trash, trash_rect, Color32::from_rgb(255, 80, 80));
                            
                            ui.add_space(15.0);
                            ui.label(RichText::new("Confirm Delete").size(20.0).strong().color(Color32::WHITE));
                            ui.add_space(8.0);
                            ui.label(RichText::new(format!("Are you sure you want to delete {} download(s)?", self.tasks_to_delete.len())).size(14.0).color(GLASS_MUTED));
                            
                            ui.add_space(35.0);
                        });
                        
                        // Bottom Action Buttons aligned to Bottom Right
                        ui.horizontal(|ui| {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.add_space(24.0); // right padding
                                
                                // Permanent Delete Button
                                let perm_btn = egui::Button::new(RichText::new("Permanent Delete").size(13.0).strong().color(Color32::WHITE))
                                    .min_size(Vec2::new(140.0, 36.0))
                                    .fill(Color32::from_rgb(220, 50, 70))
                                    .rounding(egui::Rounding::same(6.0));
                                if ui.add(perm_btn).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Delete completely from disk").clicked() {
                                    do_permanent_delete = true;
                                    close_modal = true;
                                }
                                
                                ui.add_space(12.0);
                                
                                // History Only Button
                                let hist_btn = egui::Button::new(RichText::new("Remove from History").size(13.0).strong().color(Color32::WHITE))
                                    .min_size(Vec2::new(140.0, 36.0))
                                    .fill(GLASS_CARD)
                                    .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                                    .rounding(egui::Rounding::same(6.0));
                                if ui.add(hist_btn).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Keep files on disk").clicked() {
                                    do_history_delete = true;
                                    close_modal = true;
                                }
                            });
                        });
                        ui.add_space(20.0);
                    });
                });
                
            if do_history_delete || do_permanent_delete {
                if let Ok(mut list) = self.tasks.try_lock() {
                    for i in (0..list.len()).rev() {
                        let task = &list[i];
                        if self.tasks_to_delete.contains(&task.id) {
                            if let Some(ref handle) = task.task_handle {
                                handle.pause();
                            }
                            let rapid_file = rapid_core::DownloadTaskState::manifest_path(&task.target_file);
                            let _ = std::fs::remove_file(&rapid_file);
                            remove_from_history(&task.id);
                            if do_permanent_delete {
                                let _ = std::fs::remove_file(&task.target_file);
                            }
                            self.selected_tasks.remove(&task.id);
                            list.remove(i);
                        }
                    }
                    if list.is_empty() {
                        self.selected_task_index = None;
                        self.selected_tasks.clear();
                    } else if let Some(first_checked) = self.selected_tasks.iter().next() {
                        if let Some(pos) = list.iter().position(|t| &t.id == first_checked) {
                            self.selected_task_index = Some(pos);
                        } else {
                            self.selected_task_index = Some(0);
                        }
                    } else if let Some(sel) = self.selected_task_index {
                        if sel >= list.len() {
                            self.selected_task_index = Some(list.len() - 1);
                        }
                    } else {
                        self.selected_task_index = None;
                    }
                }
                self.tasks_to_delete.clear();
            }
            if close_modal {
                self.show_delete_modal = false;
            }
        }

        // Modern Cyber-Obsidian Scheduler Modal Dialog (Section 32)
        if self.show_scheduler_modal {
            let screen_rect = ctx.screen_rect();
            let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::new("scheduler_modal_backdrop")));
            painter.rect_filled(screen_rect, 0.0, Color32::from_rgba_unmultiplied(2, 6, 23, 220));

            let mut close_modal = false;
            let mut save_schedule = false;

            let modal_frame = egui::Frame::none()
                .fill(GLASS_SURFACE)
                .stroke(Stroke::NONE)
                .rounding(egui::Rounding::same(12.0))
                .inner_margin(Margin::same(0.0))
                .shadow(egui::epaint::Shadow {
                    offset: [0.0, 10.0].into(),
                    blur: 32.0,
                    spread: 2.0,
                    color: Color32::from_black_alpha(230),
                });

            egui::Window::new("scheduler_modal_window")
                .title_bar(false)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .fixed_size(Vec2::new(540.0, 380.0))
                .frame(modal_frame)
                .show(ctx, |ui| {
                    let top_bar_h = 34.0_f32;
                    let (top_bar_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), top_bar_h), egui::Sense::hover());
                    let close_w = 42.0_f32;
                    let close_rect = egui::Rect::from_min_max(
                        egui::pos2(top_bar_rect.max.x - close_w, top_bar_rect.min.y),
                        egui::pos2(top_bar_rect.max.x, top_bar_rect.min.y + top_bar_h),
                    );
                    let close_resp = ui.interact(close_rect, ui.make_persistent_id("sched_close_btn"), egui::Sense::click());
                    if close_resp.hovered() {
                        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                    }
                    if close_resp.clicked() {
                        close_modal = true;
                    }
                    let bg_color = if close_resp.hovered() { Color32::from_rgb(255, 60, 60) } else { Color32::from_rgb(220, 38, 38) };
                    ui.painter().rect_filled(close_rect, egui::Rounding { nw: 0.0, ne: 12.0, sw: 0.0, se: 0.0 }, bg_color);
                    ui.painter().text(close_rect.center(), egui::Align2::CENTER_CENTER, "✕", egui::FontId::proportional(14.0), Color32::WHITE);

                    ui.painter().text(
                        egui::pos2(top_bar_rect.min.x + 20.0, top_bar_rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        "⏰ Download Scheduler",
                        egui::FontId::proportional(15.0),
                        GLASS_SECONDARY,
                    );

                    ui.add_space(14.0);

                    egui::Frame::none().inner_margin(Margin::symmetric(24.0, 10.0)).show(ui, |ui| {
                        egui::Frame::none()
                            .fill(Color32::from_rgba_unmultiplied(15, 23, 42, 200))
                            .stroke(Stroke::new(1.0_f32, if self.scheduler.enabled { GLASS_SECONDARY } else { GLASS_BORDER }))
                            .rounding(egui::Rounding::same(8.0))
                            .inner_margin(Margin::symmetric(14.0, 10.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    let status_dot = if self.scheduler.enabled { "🟢" } else { "⚪" };
                                    ui.label(RichText::new(status_dot).size(16.0));
                                    ui.vertical(|ui| {
                                        ui.label(RichText::new("Automated Queue Scheduler").size(13.5).strong().color(Color32::WHITE));
                                        ui.label(RichText::new("Start and stop download queues automatically at preset hours").size(11.0).color(GLASS_MUTED));
                                    });
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        let toggle_text = if self.scheduler.enabled { "ACTIVE (ON)" } else { "DISABLED (OFF)" };
                                        let toggle_color = if self.scheduler.enabled { Color32::from_rgb(16, 185, 129) } else { GLASS_MUTED };
                                        if ui.button(RichText::new(toggle_text).strong().size(11.5).color(toggle_color)).clicked() {
                                            self.scheduler.enabled = !self.scheduler.enabled;
                                            save_schedule = true;
                                        }
                                    });
                                });
                            });

                        ui.add_space(12.0);

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Presets:").size(11.5).color(GLASS_MUTED));
                            if ui.button(RichText::new("🌙 Night Hours (23:00 - 06:00)").size(11.0)).clicked() {
                                self.scheduler.start_hour = 23;
                                self.scheduler.start_minute = 0;
                                self.scheduler.stop_hour = 6;
                                self.scheduler.stop_minute = 0;
                                self.scheduler.enabled = true;
                                save_schedule = true;
                            }
                            if ui.button(RichText::new("☀️ Daytime (09:00 - 18:00)").size(11.0)).clicked() {
                                self.scheduler.start_hour = 9;
                                self.scheduler.start_minute = 0;
                                self.scheduler.stop_hour = 18;
                                self.scheduler.stop_minute = 0;
                                self.scheduler.enabled = true;
                                save_schedule = true;
                            }
                        });

                        ui.add_space(14.0);

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("▶ Start Downloads at:").size(13.0).strong().color(GLASS_TEXT));
                            ui.add_space(10.0);
                            let mut sh = self.scheduler.start_hour as i32;
                            let mut sm = self.scheduler.start_minute as i32;
                            ui.label("Hour:");
                            if ui.add(egui::DragValue::new(&mut sh).range(0..=23)).changed() {
                                self.scheduler.start_hour = sh as u32;
                                save_schedule = true;
                            }
                            ui.label("Min:");
                            if ui.add(egui::DragValue::new(&mut sm).range(0..=59)).changed() {
                                self.scheduler.start_minute = sm as u32;
                                save_schedule = true;
                            }
                            ui.label(RichText::new(format!("({:02}:{:02})", self.scheduler.start_hour, self.scheduler.start_minute)).color(GLASS_SECONDARY).strong());
                        });

                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("⏸ Stop Downloads at:").size(13.0).strong().color(GLASS_TEXT));
                            ui.add_space(10.0);
                            let mut eh = self.scheduler.stop_hour as i32;
                            let mut em = self.scheduler.stop_minute as i32;
                            ui.label("Hour:");
                            if ui.add(egui::DragValue::new(&mut eh).range(0..=23)).changed() {
                                self.scheduler.stop_hour = eh as u32;
                                save_schedule = true;
                            }
                            ui.label("Min:");
                            if ui.add(egui::DragValue::new(&mut em).range(0..=59)).changed() {
                                self.scheduler.stop_minute = em as u32;
                                save_schedule = true;
                            }
                            ui.label(RichText::new(format!("({:02}:{:02})", self.scheduler.stop_hour, self.scheduler.stop_minute)).color(STATUS_PAUSED).strong());
                        });

                        ui.add_space(14.0);

                        ui.checkbox(&mut self.scheduler.auto_shutdown, RichText::new("🔌 Auto-shutdown PC when schedule finishes").size(12.0).color(GLASS_TEXT));

                        ui.add_space(16.0);

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let done_btn = egui::Button::new(RichText::new("✓ Done").size(13.0).strong().color(Color32::WHITE))
                                .min_size(Vec2::new(100.0, 32.0))
                                .fill(GLASS_PRIMARY)
                                .rounding(egui::Rounding::same(6.0));
                            if ui.add(done_btn).clicked() {
                                save_schedule = true;
                                close_modal = true;
                            }
                        });
                    });
                });

            if save_schedule {
                save_scheduler_config(&self.scheduler);
            }
            if close_modal {
                self.show_scheduler_modal = false;
            }
        }

        // Modern Cyber-Obsidian Browser Integration Modal Dialog
        if self.show_browser_integration_modal {
            let screen_rect = ctx.screen_rect();
            let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::new("browser_modal_backdrop")));
            painter.rect_filled(screen_rect, 0.0, Color32::from_rgba_unmultiplied(2, 6, 23, 220));

            let mut close_modal = false;
            let mut run_auto_integrate = false;
            let mut export_zip = false;
            let mut launch_browser_url: Option<String> = None;

            let modal_frame = egui::Frame::none()
                .fill(GLASS_SURFACE)
                .stroke(Stroke::NONE)
                .rounding(egui::Rounding::same(12.0))
                .inner_margin(Margin::same(0.0))
                .shadow(egui::epaint::Shadow {
                    offset: [0.0, 10.0].into(),
                    blur: 32.0,
                    spread: 2.0,
                    color: Color32::from_black_alpha(230),
                });

            egui::Window::new("browser_integration_modal_window")
                .title_bar(false)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .default_width(580.0)
                .min_width(580.0)
                .max_width(580.0)
                .frame(modal_frame)
                .show(ctx, |ui| {
                    let top_bar_h = 40.0_f32;
                    let (top_bar_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), top_bar_h), egui::Sense::hover());
                    let close_w = 40.0_f32;
                    let close_rect = egui::Rect::from_min_max(
                        egui::pos2(top_bar_rect.max.x - close_w, top_bar_rect.min.y),
                        egui::pos2(top_bar_rect.max.x, top_bar_rect.max.y),
                    );

                    let close_resp = ui.allocate_rect(close_rect, egui::Sense::click());
                    let close_hov = close_resp.hovered();

                    ui.painter().rect_filled(
                        top_bar_rect,
                        egui::Rounding { nw: 12.0, ne: 12.0, sw: 0.0, se: 0.0 },
                        Color32::from_rgba_unmultiplied(15, 23, 42, 245),
                    );

                    ui.painter().text(
                        egui::pos2(top_bar_rect.min.x + 18.0, top_bar_rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        "Browser Integration & Auto-Hook",
                        egui::FontId::proportional(14.0),
                        Color32::WHITE,
                    );

                    if close_hov {
                        ui.painter().rect_filled(
                            close_rect,
                            egui::Rounding { nw: 0.0, ne: 12.0, sw: 0.0, se: 0.0 },
                            Color32::from_rgb(239, 68, 68),
                        );
                    }
                    draw_modern_icon(ui.painter(), ModernIcon::Close, egui::Rect::from_center_size(close_rect.center(), Vec2::splat(11.0)), if close_hov { Color32::WHITE } else { GLASS_MUTED });
                    if close_resp.clicked() {
                        close_modal = true;
                    }

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.add_space(18.0);
                        ui.label(RichText::new("Auto-detect and register Rapid Extension across all installed browsers like IDM").size(11.5).color(GLASS_MUTED));
                    });

                    ui.add_space(10.0);

                    let local_appdata = std::env::var("LOCALAPPDATA").unwrap_or_default();
                    let prog_files = std::env::var("ProgramFiles").unwrap_or_default();
                    let prog_files_x86 = std::env::var("ProgramFiles(x86)").unwrap_or_default();

                    let is_chrome = [
                        format!("{}/Google/Chrome/Application/chrome.exe", local_appdata),
                        format!("{}/Google/Chrome/Application/chrome.exe", prog_files),
                        format!("{}/Google/Chrome/Application/chrome.exe", prog_files_x86),
                    ].iter().any(|p| !p.is_empty() && std::path::Path::new(p).exists());

                    let is_edge = [
                        format!("{}/Microsoft/Edge/Application/msedge.exe", prog_files_x86),
                        format!("{}/Microsoft/Edge/Application/msedge.exe", prog_files),
                        format!("{}/Microsoft/Edge/Application/msedge.exe", local_appdata),
                    ].iter().any(|p| !p.is_empty() && std::path::Path::new(p).exists());

                    let is_brave = [
                        format!("{}/BraveSoftware/Brave-Browser/Application/brave.exe", prog_files),
                        format!("{}/BraveSoftware/Brave-Browser/Application/brave.exe", prog_files_x86),
                        format!("{}/BraveSoftware/Brave-Browser/Application/brave.exe", local_appdata),
                    ].iter().any(|p| !p.is_empty() && std::path::Path::new(p).exists());

                    let is_opera = [
                        format!("{}/Programs/Opera/launcher.exe", local_appdata),
                        format!("{}/Programs/Opera GX/launcher.exe", local_appdata),
                        format!("{}/Opera/launcher.exe", prog_files),
                    ].iter().any(|p| !p.is_empty() && std::path::Path::new(p).exists());

                    let is_firefox = [
                        format!("{}/Mozilla Firefox/firefox.exe", prog_files),
                        format!("{}/Mozilla Firefox/firefox.exe", prog_files_x86),
                    ].iter().any(|p| !p.is_empty() && std::path::Path::new(p).exists());

                    let is_vivaldi = [
                        format!("{}/Vivaldi/Application/vivaldi.exe", local_appdata),
                        format!("{}/Vivaldi/Application/vivaldi.exe", prog_files),
                    ].iter().any(|p| !p.is_empty() && std::path::Path::new(p).exists());

                    let browser_items = [
                        ("Google Chrome", "chrome://extensions", is_chrome),
                        ("Microsoft Edge", "edge://extensions", is_edge),
                        ("Brave Browser", "brave://extensions", is_brave),
                        ("Opera / Opera GX", "opera://extensions", is_opera),
                        ("Mozilla Firefox", "about:addons", is_firefox),
                        ("Vivaldi", "vivaldi://extensions", is_vivaldi),
                    ];

                    ui.horizontal(|ui| {
                        ui.add_space(16.0);
                        egui::Grid::new("browser_grid").num_columns(2).spacing(Vec2::new(14.0, 10.0)).show(ui, |ui| {
                            for (idx, (name, ext_url, is_detected)) in browser_items.iter().enumerate() {
                                egui::Frame::none()
                                    .fill(GLASS_CARD)
                                    .stroke(Stroke::new(1.0_f32, if *is_detected { Color32::from_rgb(30, 58, 86) } else { Color32::from_rgb(30, 41, 59) }))
                                    .rounding(egui::Rounding::same(8.0))
                                    .inner_margin(Margin::symmetric(14.0, 10.0))
                                    .show(ui, |ui| {
                                        ui.set_width(252.0);
                                        // Top Row: Browser Name + Status Badge
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(*name).size(13.0).strong().color(if *is_detected { Color32::WHITE } else { GLASS_MUTED }));
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                if *is_detected {
                                                    egui::Frame::none()
                                                        .fill(Color32::from_rgba_unmultiplied(16, 185, 129, 30))
                                                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(16, 185, 129)))
                                                        .rounding(egui::Rounding::same(4.0))
                                                        .inner_margin(Margin::symmetric(6.0, 2.0))
                                                        .show(ui, |ui| {
                                                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                                                let (icon_r, _) = ui.allocate_exact_size(Vec2::splat(10.0), egui::Sense::hover());
                                                                draw_modern_icon(ui.painter(), ModernIcon::Check, icon_r, Color32::from_rgb(16, 185, 129));
                                                                ui.add_space(3.0);
                                                                ui.label(RichText::new("Detected").size(10.5).color(Color32::from_rgb(52, 211, 153)).strong());
                                                            });
                                                        });
                                                } else {
                                                    egui::Frame::none()
                                                        .fill(Color32::from_rgba_unmultiplied(71, 85, 105, 30))
                                                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(51, 65, 85)))
                                                        .rounding(egui::Rounding::same(4.0))
                                                        .inner_margin(Margin::symmetric(6.0, 2.0))
                                                        .show(ui, |ui| {
                                                            ui.label(RichText::new("Not Installed").size(10.0).color(GLASS_MUTED));
                                                        });
                                                }
                                            });
                                        });

                                        ui.add_space(6.0);

                                        // Bottom Row: Registry Hook Badge + Action Button
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new("Registry Hook:").size(10.5).color(GLASS_MUTED));
                                            ui.label(RichText::new("Active (HKCU)").size(10.5).color(Color32::from_rgb(56, 189, 248)).strong());

                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                if *is_detected {
                                                    let open_btn = egui::Button::new(RichText::new("Open").size(10.5).color(Color32::WHITE))
                                                        .fill(Color32::from_rgb(30, 41, 59))
                                                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(71, 85, 105)))
                                                        .rounding(egui::Rounding::same(4.0))
                                                        .min_size(Vec2::new(48.0, 20.0));
                                                    if ui.add(open_btn).on_hover_text("Open browser extensions page").clicked() {
                                                        launch_browser_url = Some(ext_url.to_string());
                                                    }
                                                }
                                            });
                                        });
                                    });

                                if idx % 2 == 1 {
                                    ui.end_row();
                                }
                            }
                        });
                    });

                    ui.add_space(10.0);

                    if let Some((ref msg, is_success)) = self.browser_integration_msg {
                        ui.horizontal(|ui| {
                            ui.add_space(16.0);
                            egui::Frame::none()
                                .fill(if is_success { Color32::from_rgba_unmultiplied(16, 185, 129, 30) } else { Color32::from_rgba_unmultiplied(239, 68, 68, 30) })
                                .stroke(Stroke::new(1.0_f32, if is_success { Color32::from_rgb(16, 185, 129) } else { Color32::from_rgb(239, 68, 68) }))
                                .rounding(egui::Rounding::same(6.0))
                                .inner_margin(Margin::symmetric(14.0, 8.0))
                                .show(ui, |ui| {
                                    ui.set_width(520.0);
                                    ui.horizontal(|ui| {
                                        let (icon_r, _) = ui.allocate_exact_size(Vec2::splat(12.0), egui::Sense::hover());
                                        draw_modern_icon(ui.painter(), if is_success { ModernIcon::Check } else { ModernIcon::Close }, icon_r, if is_success { Color32::from_rgb(16, 185, 129) } else { Color32::from_rgb(239, 68, 68) });
                                        ui.add_space(4.0);
                                        ui.label(RichText::new(msg).size(11.5).color(if is_success { Color32::from_rgb(167, 243, 208) } else { Color32::from_rgb(254, 202, 202) }).strong());
                                    });
                                });
                        });
                        ui.add_space(8.0);
                    }

                    ui.horizontal(|ui| {
                        ui.add_space(16.0);
                        ui.spacing_mut().item_spacing = Vec2::new(10.0, 0.0);

                        let auto_btn = egui::Button::new(
                            RichText::new("Auto-Integrate All Browsers")
                                .size(12.0)
                                .color(Color32::WHITE)
                                .strong(),
                        )
                        .fill(Color32::from_rgb(14, 116, 144))
                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(6, 182, 212)))
                        .rounding(egui::Rounding::same(6.0))
                        .min_size(Vec2::new(230.0, 32.0));

                        if ui.add(auto_btn).clicked() {
                            run_auto_integrate = true;
                        }

                        let launch_chrome_btn = egui::Button::new(
                            RichText::new("⚡ Launch Chrome Hook")
                                .size(11.5)
                                .color(Color32::WHITE)
                                .strong(),
                        )
                        .fill(Color32::from_rgb(14, 165, 233))
                        .rounding(egui::Rounding::same(6.0))
                        .min_size(Vec2::new(145.0, 32.0));

                        if ui.add(launch_chrome_btn).on_hover_text("Launch Chrome with Rapid Download Manager hook active").clicked() {
                            launch_chrome_with_extension(None);
                        }

                        let export_btn = egui::Button::new(
                            RichText::new("Export Webstore Zip")
                                .size(11.5)
                                .color(Color32::from_rgb(226, 232, 240)),
                        )
                        .fill(GLASS_CARD)
                        .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                        .rounding(egui::Rounding::same(6.0))
                        .min_size(Vec2::new(165.0, 32.0));

                        if ui.add(export_btn).clicked() {
                            export_zip = true;
                        }

                        let close_btn = egui::Button::new(
                            RichText::new("Close")
                                .size(11.5)
                                .color(Color32::from_rgb(203, 213, 225)),
                        )
                        .fill(Color32::from_rgb(30, 41, 59))
                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(51, 65, 85)))
                        .rounding(egui::Rounding::same(6.0))
                        .min_size(Vec2::new(85.0, 32.0));

                        if ui.add(close_btn).clicked() {
                            close_modal = true;
                        }
                    });
                });

            if run_auto_integrate {
                #[cfg(windows)]
                {
                    let mut cmd = std::process::Command::new("powershell");
                    cmd.args(&[
                        "-ExecutionPolicy", "Bypass",
                        "-NoProfile",
                        "-File", "auto_integrate_browsers.ps1",
                    ]);
                    const CREATE_NO_WINDOW: u32 = 0x08000000;
                    std::os::windows::process::CommandExt::creation_flags(&mut cmd, CREATE_NO_WINDOW);
                    match cmd.output() {
                        Ok(out) if out.status.success() => {
                            self.browser_integration_msg = Some(("Registry keys registered for all browsers! Restart browser to activate.".to_string(), true));
                        }
                        Ok(out) => {
                            let err = String::from_utf8_lossy(&out.stderr);
                            self.browser_integration_msg = Some((format!("Integration notice: {}", err.trim()), false));
                        }
                        Err(e) => {
                            self.browser_integration_msg = Some((format!("Failed to run script: {}", e), false));
                        }
                    }
                }
            }

            if export_zip {
                let zip_path = std::path::PathBuf::from("dist").join("RapidExtension_StoreReady.zip");
                if zip_path.exists() {
                    let _ = open::that(&zip_path);
                } else {
                    let _ = open::that("extension");
                }
            }

            if let Some(target) = launch_browser_url {
                let _ = open::that(&target);
            }

            if close_modal {
                self.show_browser_integration_modal = false;
            }
        }

        // Cyber-Obsidian Modern About & Updates Modal
        if self.show_about_modal {
            let mut close_modal = false;
            let mut open_url_target: Option<&str> = None;

            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                close_modal = true;
            }

            egui::Window::new("about_rapid_modal")
                .title_bar(false)
                .resizable(false)
                .collapsible(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .fixed_size(Vec2::new(540.0, 410.0))
                .frame(
                    egui::Frame::none()
                        // Transparent glassmorphic background & NO border stroke
                        .fill(Color32::from_rgba_unmultiplied(12, 18, 34, 230))
                        .stroke(Stroke::NONE)
                        .rounding(egui::Rounding::ZERO)
                        .shadow(egui::epaint::Shadow {
                            offset: egui::vec2(0.0, 12.0),
                            blur: 32.0_f32,
                            spread: 2.0_f32,
                            color: Color32::from_rgba_unmultiplied(0, 0, 0, 230),
                        })
                        .inner_margin(Margin::ZERO),
                )
                .show(ctx, |ui| {
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    ui.spacing_mut().window_margin = Margin::ZERO;

                    let win_rect = ui.clip_rect();

                    // Absolute Top-Right Corner Close Button (Position 0,0 - 0 margin, 0 padding, 0 border-radius)
                    let btn_w = 42.0_f32;
                    let btn_h = 30.0_f32;
                    let close_rect = egui::Rect::from_min_max(
                        egui::pos2(win_rect.max.x - btn_w, win_rect.min.y),
                        egui::pos2(win_rect.max.x, win_rect.min.y + btn_h),
                    );

                    let close_resp = ui.interact(close_rect, ui.id().with("about_corner_00_close_btn"), egui::Sense::click());
                    let close_bg = if close_resp.is_pointer_button_down_on() {
                        Color32::from_rgb(185, 28, 28)
                    } else if close_resp.hovered() {
                        Color32::from_rgb(239, 68, 68)
                    } else {
                        Color32::from_rgb(220, 38, 38)
                    };

                    ui.painter().rect_filled(close_rect, egui::Rounding::ZERO, close_bg);

                    // Crisp White Vector Cross
                    let icon_box = egui::Rect::from_center_size(close_rect.center(), Vec2::splat(11.0));
                    draw_modern_icon(ui.painter(), ModernIcon::Close, icon_box, Color32::WHITE);

                    if close_resp.clicked() {
                        close_modal = true;
                    }

                    // Main Content Container (Full Width with Inner Margin)
                    egui::Frame::none()
                        .fill(Color32::TRANSPARENT)
                        .inner_margin(Margin::symmetric(24.0, 16.0))
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                // 1. Centered Hero Header (Logo + Name + Version + Subtitle)
                                ui.vertical_centered(|ui| {
                                    ui.image((self.titlebar_logo.id(), Vec2::new(180.0, 36.0)));
                                    ui.add_space(5.0);
                                    ui.horizontal(|ui| {
                                        let full_w = ui.available_width();
                                        ui.add_space(full_w * 0.5 - 120.0);
                                        ui.label(
                                            RichText::new("Rapid Download Manager")
                                                .size(16.0)
                                                .color(Color32::WHITE)
                                                .strong(),
                                        );
                                        ui.add_space(6.0);
                                        egui::Frame::none()
                                            .fill(Color32::from_rgba_unmultiplied(99, 102, 241, 40))
                                            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(129, 140, 248)))
                                            .rounding(egui::Rounding::same(3.0))
                                            .inner_margin(Margin::symmetric(6.0, 1.5))
                                            .show(ui, |ui| {
                                                ui.label(
                                                    RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                                                        .size(10.5)
                                                        .color(Color32::from_rgb(165, 180, 252))
                                                        .strong(),
                                                );
                                            });
                                    });
                                    ui.add_space(1.0);
                                    ui.label(
                                        RichText::new("High-Speed Multi-Stream Accelerated Download Accelerator")
                                            .size(11.0)
                                            .color(GLASS_MUTED),
                                    );
                                });

                                ui.add_space(12.0);

                                // 2. Full-Width Details Section (Border and Background REMOVED)
                                let avail_w = ui.available_width();
                                egui::Frame::none()
                                    .fill(Color32::TRANSPARENT)
                                    .stroke(Stroke::NONE)
                                    .inner_margin(Margin::symmetric(4.0, 2.0))
                                    .show(ui, |ui| {
                                        ui.set_width(avail_w);
                                        ui.label(
                                            RichText::new("System Architecture & Engine Capabilities")
                                                .size(12.0)
                                                .color(Color32::WHITE)
                                                .strong(),
                                        );
                                        ui.add_space(6.0);
                                        ui.label(
                                            RichText::new("• Up to 32 Parallel Sockets with Dynamic TCP Stream Balancing")
                                                .size(10.5)
                                                .color(Color32::from_rgb(226, 232, 240)),
                                        );
                                        ui.label(
                                            RichText::new("• Asynchronous Non-Blocking Tokio Runtime + Rustls Security Stack")
                                                .size(10.5)
                                                .color(Color32::from_rgb(226, 232, 240)),
                                        );
                                        ui.label(
                                            RichText::new("• Native HarfBuzz-grade Bengali & Multilingual Font Shaping")
                                                .size(10.5)
                                                .color(Color32::from_rgb(226, 232, 240)),
                                        );
                                        ui.label(
                                            RichText::new("• High-Efficiency WGPU Hardware-Accelerated Rendering")
                                                .size(10.5)
                                                .color(Color32::from_rgb(226, 232, 240)),
                                        );
                                        ui.label(
                                            RichText::new("• 100% Memory-Safe, Zero-Telemetry & Privacy-First Architecture")
                                                .size(10.5)
                                                .color(Color32::from_rgb(52, 211, 153))
                                                .strong(),
                                        );
                                    });

                                ui.add_space(8.0);

                                // 3. Full-Width Update Status Section (Border and Background REMOVED)
                                egui::Frame::none()
                                    .fill(Color32::TRANSPARENT)
                                    .stroke(Stroke::NONE)
                                    .inner_margin(Margin::symmetric(4.0, 2.0))
                                    .show(ui, |ui| {
                                        ui.set_width(avail_w);
                                        ui.horizontal(|ui| {
                                            let (icon_r, _) = ui.allocate_exact_size(Vec2::splat(12.0), egui::Sense::hover());
                                            draw_modern_icon(ui.painter(), ModernIcon::Check, icon_r, Color32::from_rgb(16, 185, 129));
                                            ui.add_space(6.0);
                                            ui.label(
                                                RichText::new(format!("Application is up to date: Version {} (Production Release)", env!("CARGO_PKG_VERSION")))
                                                    .size(11.0)
                                                    .color(Color32::from_rgb(167, 243, 208))
                                                    .strong(),
                                            );
                                        });
                                    });

                                ui.add_space(14.0);

                                // 4. Full-Width 3 Action Buttons in 3 Distinct Colors with Icons
                                let btn_spacing = 8.0_f32;
                                let btn_w = (ui.available_width() - 2.0 * btn_spacing) / 3.0_f32;

                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing.x = btn_spacing;

                                    // Button 1: Official Website (Ocean Blue / Cyan)
                                    let (web_rect, web_resp) = ui.allocate_exact_size(Vec2::new(btn_w, 34.0), egui::Sense::click());
                                    let web_bg = if web_resp.is_pointer_button_down_on() {
                                        Color32::from_rgb(12, 74, 96)
                                    } else if web_resp.hovered() {
                                        Color32::from_rgb(8, 145, 178)
                                    } else {
                                        Color32::from_rgb(14, 116, 144)
                                    };
                                    ui.painter().rect_filled(web_rect, egui::Rounding::same(5.0), web_bg);
                                    let web_icon_r = egui::Rect::from_center_size(egui::pos2(web_rect.left() + 20.0, web_rect.center().y), Vec2::splat(13.0));
                                    draw_modern_icon(ui.painter(), ModernIcon::Globe, web_icon_r, Color32::WHITE);
                                    ui.painter().text(
                                        egui::pos2(web_rect.left() + 34.0, web_rect.center().y),
                                        egui::Align2::LEFT_CENTER,
                                        "Official Website",
                                        egui::FontId::proportional(11.5),
                                        Color32::WHITE,
                                    );
                                    if web_resp.clicked() {
                                        open_url_target = Some("https://promahbubul.github.io/rapid_download_manager/");
                                    }

                                    // Button 2: GitHub Repository (Royal Violet / Purple with GitHub Icon)
                                    let (gh_rect, gh_resp) = ui.allocate_exact_size(Vec2::new(btn_w, 34.0), egui::Sense::click());
                                    let gh_bg = if gh_resp.is_pointer_button_down_on() {
                                        Color32::from_rgb(67, 56, 202)
                                    } else if gh_resp.hovered() {
                                        Color32::from_rgb(129, 140, 248)
                                    } else {
                                        Color32::from_rgb(99, 102, 241)
                                    };
                                    ui.painter().rect_filled(gh_rect, egui::Rounding::same(5.0), gh_bg);
                                    let gh_icon_r = egui::Rect::from_center_size(egui::pos2(gh_rect.left() + 18.0, gh_rect.center().y), Vec2::splat(14.0));
                                    draw_modern_icon(ui.painter(), ModernIcon::Github, gh_icon_r, Color32::WHITE);
                                    ui.painter().text(
                                        egui::pos2(gh_rect.left() + 32.0, gh_rect.center().y),
                                        egui::Align2::LEFT_CENTER,
                                        "GitHub Repository",
                                        egui::FontId::proportional(11.5),
                                        Color32::WHITE,
                                    );
                                    if gh_resp.clicked() {
                                        open_url_target = Some("https://github.com/promahbubul/rapid_download_manager");
                                    }

                                    // Button 3: Privacy Policy (Emerald Green)
                                    let (priv_rect, priv_resp) = ui.allocate_exact_size(Vec2::new(btn_w, 34.0), egui::Sense::click());
                                    let priv_bg = if priv_resp.is_pointer_button_down_on() {
                                        Color32::from_rgb(4, 120, 87)
                                    } else if priv_resp.hovered() {
                                        Color32::from_rgb(16, 185, 129)
                                    } else {
                                        Color32::from_rgb(5, 150, 105)
                                    };
                                    ui.painter().rect_filled(priv_rect, egui::Rounding::same(5.0), priv_bg);
                                    let priv_icon_r = egui::Rect::from_center_size(egui::pos2(priv_rect.left() + 20.0, priv_rect.center().y), Vec2::splat(13.0));
                                    draw_modern_icon(ui.painter(), ModernIcon::ExternalFile, priv_icon_r, Color32::WHITE);
                                    ui.painter().text(
                                        egui::pos2(priv_rect.left() + 34.0, priv_rect.center().y),
                                        egui::Align2::LEFT_CENTER,
                                        "Privacy Policy",
                                        egui::FontId::proportional(11.5),
                                        Color32::WHITE,
                                    );
                                    if priv_resp.clicked() {
                                        open_url_target = Some("https://github.com/promahbubul/rapid_download_manager/blob/main/store_listing/PRIVACY_POLICY.md");
                                    }
                                });
                            });
                        });
                });

            if let Some(target) = open_url_target {
                let _ = open::that(target);
            }

            if close_modal {
                self.show_about_modal = false;
            }
        }


    }
}

fn spawn_download_task(
    url: String,
    dest_dir: PathBuf,
    segments: usize,
    custom_filename: Option<String>,
    cookies: Option<String>,
    user_agent: Option<String>,
    referrer: Option<String>,
    is_gdrive: bool,
    is_video_platform: bool,
    speed_limit: Option<Arc<AtomicU64>>,
    quality: Option<rapid_core::youtube::DownloadQuality>,
    rt: Arc<Runtime>,
    tasks_arc: Arc<Mutex<Vec<ActiveTaskUI>>>,
) {
    let task_id = format!("task-{}", Utc::now().timestamp_millis());

    let is_hls = rapid_core::hls::HlsDownloader::is_hls(&url)
        || custom_filename.as_deref().map(|f| f.to_lowercase().ends_with(".m3u8")).unwrap_or(false);

    if is_hls {
        let speed_limit_for_hls = speed_limit.clone();
        rt.spawn(async move {
            let resolved_fn = if let Some(ref cf) = custom_filename {
                if cf.to_lowercase().ends_with(".m3u8") {
                    format!("{}.mp4", &cf[..cf.len() - 5])
                } else {
                    cf.clone()
                }
            } else {
                "HLS_Stream.mp4".to_string()
            };

            let target_file = dest_dir.join(&resolved_fn);
            let cancel_token = tokio_util::sync::CancellationToken::new();
            let (progress_tx, mut rx) = tokio::sync::broadcast::channel::<rapid_core::DownloadProgress>(100);

            let downloader = rapid_core::hls::HlsDownloader::new(
                task_id.clone(),
                url.clone(),
                resolved_fn.clone(),
                target_file.clone(),
                cookies.clone(),
                referrer.clone(),
                user_agent.clone(),
                speed_limit_for_hls,
                cancel_token,
            );

            let ui_entry = ActiveTaskUI {
                id: task_id.clone(),
                filename: resolved_fn.clone(),
                url: url.clone(),
                target_file: target_file.clone(),
                total_bytes: None,
                downloaded_bytes: 0,
                progress_percent: 0.0,
                speed_bps: 0,
                eta_seconds: None,
                status: DownloadStatus::Downloading,
                segments: Vec::new(),
                task_handle: None,
                num_segments: 1,
                is_resuming: false,
                cookies: cookies.clone(),
                referrer: referrer.clone(),
                user_agent: user_agent.clone(),
                is_youtube: false,
                quality: None,
                yt_cancel_token: None,
            };

            {
                let mut list = tasks_arc.lock().await;
                list.push(ui_entry);
            }

            let tasks_for_progress = Arc::clone(&tasks_arc);
            let task_id_for_progress = task_id.clone();

            tokio::spawn(async move {
                while let Ok(prog) = rx.recv().await {
                    let mut list = tasks_for_progress.lock().await;
                    if let Some(item) = list.iter_mut().find(|t| t.id == task_id_for_progress) {
                        item.downloaded_bytes = prog.downloaded_bytes;
                        item.progress_percent = prog.progress_percent;
                        item.speed_bps = prog.speed_bps;
                        item.eta_seconds = prog.eta_seconds;
                        item.status = prog.status.clone();
                        item.segments = prog.segments;
                        if let Some(tot) = prog.total_bytes {
                            item.total_bytes = Some(tot);
                        }
                    }
                }
            });

            let res = downloader.run(progress_tx).await;
            let mut list = tasks_arc.lock().await;
            if let Some(item) = list.iter_mut().find(|t| t.id == task_id) {
                match res {
                    Ok(_) => {
                        item.status = DownloadStatus::Completed;
                        item.progress_percent = 100.0;
                        item.speed_bps = 0;
                        item.eta_seconds = None;
                        save_task_to_history(item);
                    }
                    Err(e) => {
                        item.status = DownloadStatus::Failed(e.to_string());
                        item.speed_bps = 0;
                        item.eta_seconds = None;
                    }
                }
            }
        });
        return;
    }

    let is_yt = is_video_platform
        || rapid_core::youtube::YoutubeDownloader::is_extractable_platform(&url)
        || referrer.as_deref().map(|r| rapid_core::youtube::YoutubeDownloader::is_extractable_platform(r)).unwrap_or(false);

    if is_yt {
        let speed_limit_for_yt = speed_limit.clone();
        let canonical_url = if rapid_core::youtube::YoutubeResolver::is_youtube(&url) || referrer.as_deref().map(|r| rapid_core::youtube::YoutubeResolver::is_youtube(r)).unwrap_or(false) {
            rapid_core::youtube::YoutubeResolver::canonicalize_url(&url, referrer.as_deref())
        } else {
            url.clone()
        };
        let selected_quality = quality.unwrap_or(rapid_core::youtube::DownloadQuality::Best);
        let target_ext = selected_quality.target_extension();

        let yt_cookies = cookies.clone();
        let yt_ua = user_agent.clone();
        let yt_ref = referrer.clone();

        rt.spawn(async move {
            let (resolved_fn, resolved_url) = if let Some(ref cf) = custom_filename {
                if rapid_core::engine::is_generic_placeholder(cf) {
                    match rapid_core::youtube::YoutubeResolver::resolve_metadata(&canonical_url).await {
                        Ok(meta) => {
                            let stem = if let Some(dot) = meta.clean_filename.rfind('.') {
                                &meta.clean_filename[..dot]
                            } else {
                                &meta.clean_filename
                            };
                            (format!("{}.{}", stem, target_ext), canonical_url)
                        }
                        Err(_) => (format!("Media_Download.{}", target_ext), canonical_url),
                    }
                } else {
                    let name = if cf.to_lowercase().ends_with(&format!(".{}", target_ext)) {
                        cf.clone()
                    } else {
                        let stem = if let Some(dot) = cf.rfind('.') {
                            &cf[..dot]
                        } else {
                            cf.as_str()
                        };
                        format!("{}.{}", stem, target_ext)
                    };
                    (name, canonical_url)
                }
            } else {
                match rapid_core::youtube::YoutubeResolver::resolve_metadata(&canonical_url).await {
                    Ok(meta) => {
                        let stem = if let Some(dot) = meta.clean_filename.rfind('.') {
                            &meta.clean_filename[..dot]
                        } else {
                            &meta.clean_filename
                        };
                        (format!("{}.{}", stem, target_ext), canonical_url)
                    }
                    Err(_) => (format!("Media_Download.{}", target_ext), canonical_url),
                }
            };

            let target_file = dest_dir.join(&resolved_fn);
            let cancel_token = tokio_util::sync::CancellationToken::new();
            let (progress_tx, mut rx) = tokio::sync::broadcast::channel::<rapid_core::DownloadProgress>(100);

            let downloader = rapid_core::youtube::YoutubeDownloader::new(
                task_id.clone(),
                resolved_url.clone(),
                resolved_fn.clone(),
                target_file.clone(),
                selected_quality,
                speed_limit_for_yt,
                cancel_token.clone(),
                yt_cookies,
                yt_ua,
                yt_ref,
            );

            let ui_entry = ActiveTaskUI {
                id: task_id.clone(),
                filename: resolved_fn.clone(),
                url: resolved_url.clone(),
                target_file: target_file.clone(),
                total_bytes: None,
                downloaded_bytes: 0,
                progress_percent: 0.0,
                speed_bps: 0,
                eta_seconds: None,
                status: DownloadStatus::Downloading,
                segments: Vec::new(),
                task_handle: None,
                num_segments: 8,
                is_resuming: false,
                cookies: cookies.clone(),
                referrer: referrer.clone(),
                user_agent: user_agent.clone(),
                is_youtube: true,
                quality: Some(selected_quality),
                yt_cancel_token: Some(cancel_token),
            };

            {
                let mut list = tasks_arc.lock().await;
                list.push(ui_entry);
            }

            let tasks_for_progress = Arc::clone(&tasks_arc);
            let task_id_for_progress = task_id.clone();

            tokio::spawn(async move {
                while let Ok(prog) = rx.recv().await {
                    let mut list = tasks_for_progress.lock().await;
                    if let Some(item) = list.iter_mut().find(|t| t.id == task_id_for_progress) {
                        item.downloaded_bytes = prog.downloaded_bytes;
                        item.progress_percent = prog.progress_percent;
                        item.speed_bps = prog.speed_bps;
                        item.eta_seconds = prog.eta_seconds;
                        item.status = prog.status.clone();
                        item.segments = prog.segments;
                        if let Some(tot) = prog.total_bytes {
                            item.total_bytes = Some(tot);
                        }
                    }
                }
            });

            let res = downloader.run(progress_tx).await;
            let mut list = tasks_arc.lock().await;
            if let Some(item) = list.iter_mut().find(|t| t.id == task_id) {
                match res {
                    Ok(_) => {
                        item.status = DownloadStatus::Completed;
                        item.progress_percent = 100.0;
                        item.speed_bps = 0;
                        item.eta_seconds = None;
                        save_task_to_history(item);
                    }
                    Err(e) => {
                        item.status = DownloadStatus::Failed(e.to_string());
                        item.speed_bps = 0;
                        item.eta_seconds = None;
                    }
                }
            }
        });
        return;
    }

    rt.spawn(async move {
        let gdrive = is_gdrive
            || url.contains("drive.google.com")
            || url.contains("googleusercontent.com")
            || url.contains("usercontent.google.com")
            || url.contains("docs.google.com")
            || url.contains("takeout-download-drive");
        let effective_cookies = if gdrive && cookies.is_none() {
            load_google_cookies()
        } else {
            cookies.clone()
        };
        let config = DownloadConfig {
            url: url.clone(),
            output_dir: dest_dir.clone(),
            custom_filename: custom_filename.clone(),
            num_segments: if gdrive { 1 } else { segments },
            cookies: effective_cookies,
            user_agent: user_agent.clone(),
            referrer: referrer.clone(),
            is_gdrive: gdrive,
            speed_limit: speed_limit.clone(),
        };

        match DownloadTask::create(task_id.clone(), config).await {
            Ok(task) => {
                let task_arc = Arc::new(task);
                let mut rx = task_arc.subscribe();

                let filename = task_arc
                    .target_file
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "download".to_string());

                let initial_segments = task_arc.snapshot_segments().await;

                let should_queue = {
                    let list = tasks_arc.lock().await;
                    list.iter().filter(|t| t.status == DownloadStatus::Downloading).count() >= 3
                };

                let initial_status = if should_queue {
                    DownloadStatus::Queued
                } else {
                    DownloadStatus::Downloading
                };

                let ui_entry = ActiveTaskUI {
                    id: task_id.clone(),
                    filename: filename.clone(),
                    url: url.clone(),
                    target_file: task_arc.target_file.clone(),
                    total_bytes: task_arc.total_bytes,
                    downloaded_bytes: 0,
                    progress_percent: 0.0,
                    speed_bps: 0,
                    eta_seconds: None,
                    status: initial_status.clone(),
                    segments: initial_segments,
                    task_handle: Some(Arc::clone(&task_arc)),
                    num_segments: segments,
                    is_resuming: false,
                    cookies: cookies.clone(),
                    referrer: referrer.clone(),
                    user_agent: user_agent.clone(),
                    is_youtube: false,
                    quality: None,
                    yt_cancel_token: None,
                };

                {
                    let mut list = tasks_arc.lock().await;
                    list.push(ui_entry);
                }

                if should_queue {
                    return;
                }

                let tasks_for_progress = Arc::clone(&tasks_arc);
                let task_id_for_progress = task_id.clone();

                tokio::spawn(async move {
                    while let Ok(prog) = rx.recv().await {
                        let mut list = tasks_for_progress.lock().await;
                        if let Some(item) = list.iter_mut().find(|t| t.id == task_id_for_progress) {
                            item.downloaded_bytes = prog.downloaded_bytes;
                            item.progress_percent = prog.progress_percent;
                            item.speed_bps = prog.speed_bps;
                            item.eta_seconds = prog.eta_seconds;
                            item.status = prog.status.clone();
                            item.segments = prog.segments;
                        }
                    }
                });

                let res = task_arc.run().await;
                let mut list = tasks_arc.lock().await;
                if let Some(item) = list.iter_mut().find(|t| t.id == task_id) {
                    match res {
                        Ok(_) => {
                            item.status = DownloadStatus::Completed;
                            item.progress_percent = 100.0;
                            item.speed_bps = 0;
                            item.eta_seconds = None;
                            save_task_to_history(item);
                        }
                        Err(e) => {
                            if item.status == DownloadStatus::Downloading {
                                item.status = DownloadStatus::Failed(e.to_string());
                                item.speed_bps = 0;
                                item.eta_seconds = None;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to create download: {}", e);
                let fallback_name = custom_filename
                    .clone()
                    .unwrap_or_else(|| {
                        let clean = url.split('?').next().unwrap_or(&url);
                        clean.rsplit('/').next().unwrap_or("Failed_Download").to_string()
                    });
                let ui_entry = ActiveTaskUI {
                    id: task_id.clone(),
                    filename: fallback_name,
                    url: url.clone(),
                    target_file: dest_dir.join("failed_download"),
                    total_bytes: None,
                    downloaded_bytes: 0,
                    progress_percent: 0.0,
                    speed_bps: 0,
                    eta_seconds: None,
                    status: DownloadStatus::Failed(format!("Failed to start: {}", e)),
                    segments: Vec::new(),
                    task_handle: None,
                    num_segments: segments,
                    is_resuming: false,
                    cookies: cookies.clone(),
                    referrer: referrer.clone(),
                    user_agent: user_agent.clone(),
                    is_youtube: false,
                    quality: None,
                    yt_cancel_token: None,
                };
                let mut list = tasks_arc.lock().await;
                list.push(ui_entry);
            }
        }
    });
}

// Background HTTP server for browser extension integration
async fn run_extension_server(
    pending_queue: Arc<std::sync::Mutex<VecDeque<PendingBrowserDownload>>>,
    dest_dir: PathBuf,
    ctx: egui::Context,
    resolver_tx: std::sync::mpsc::Sender<(String, String)>,
    tasks_arc: Arc<tokio::sync::Mutex<Vec<ActiveTaskUI>>>,
) {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    let listener = loop {
        match TcpListener::bind("127.0.0.1:9669").await {
            Ok(l) => {
                println!("[Rapid] Extension server listening on 127.0.0.1:9669");
                break l;
            }
            Err(e) => {
                eprintln!("[Rapid] Extension server bind retry on 9669: {}. Retrying in 1s...", e);
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            }
        }
    };

    loop {
        let (mut socket, _) = match listener.accept().await {
            Ok(s) => s,
            Err(_) => continue,
        };

        let queue_clone = Arc::clone(&pending_queue);
        let dest_clone = dest_dir.clone();
        let ctx_clone = ctx.clone();
        let resolver_tx_clone = resolver_tx.clone();
        let tasks_clone = Arc::clone(&tasks_arc);

        tokio::spawn(async move {
            let mut buffer = Vec::new();
            let mut temp = [0u8; 2048];
            let mut content_length: Option<usize> = None;
            let mut header_end: Option<usize> = None;

            loop {
                let n = match socket.read(&mut temp).await {
                    Ok(n) if n > 0 => n,
                    _ => break,
                };
                buffer.extend_from_slice(&temp[..n]);

                if header_end.is_none() {
                    if let Some(pos) = buffer.windows(4).position(|w| w == b"\r\n\r\n") {
                        header_end = Some(pos + 4);
                        let headers_str = String::from_utf8_lossy(&buffer[..pos]);
                        for line in headers_str.lines() {
                            if line.to_ascii_lowercase().starts_with("content-length:") {
                                if let Some(len_str) = line.split(':').nth(1) {
                                    content_length = len_str.trim().parse::<usize>().ok();
                                }
                            }
                        }
                    }
                }

                if let Some(hend) = header_end {
                    let needed_len = hend + content_length.unwrap_or(0);
                    if buffer.len() >= needed_len {
                        break;
                    }
                }

                if buffer.len() > 65536 {
                    break;
                }
            }

            let req_str = String::from_utf8_lossy(&buffer);

            // Extract Origin header to echo back (Mandatory for Private Network Access in Chrome MV3)
            let mut origin = "*".to_string();
            for line in req_str.lines() {
                let trimmed = line.trim();
                if trimmed.to_ascii_lowercase().starts_with("origin:") {
                    let parts: Vec<&str> = trimmed.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        let val = parts[1].trim();
                        if !val.is_empty() {
                            origin = val.to_string();
                        }
                    }
                }
            }

            // Handle CORS preflight (Crucial for Chrome MV3 Private Network Access)
            if req_str.starts_with("OPTIONS") {
                let response = format!(
                    "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: {}\r\nAccess-Control-Allow-Methods: POST, OPTIONS, GET\r\nAccess-Control-Allow-Headers: Content-Type, Authorization, X-Requested-With, Access-Control-Request-Private-Network, *\r\nAccess-Control-Allow-Private-Network: true\r\nAccess-Control-Allow-Credentials: true\r\nVary: Origin, Access-Control-Request-Private-Network\r\nContent-Length: 0\r\n\r\n",
                    origin
                );
                let _ = socket.write_all(response.as_bytes()).await;
                return;
            }

            if req_str.contains("/sync_cookies") {
                if let Some(hend) = header_end {
                    let body_slice = &buffer[hend..];
                    if let Ok(body_str) = std::str::from_utf8(body_slice) {
                        if let Ok(val) = serde_json::from_str::<serde_json::Value>(body_str.trim()) {
                            if let Some(cookies) = val.get("cookies").and_then(|c| c.as_str()) {
                                save_google_cookies(cookies);
                                let resp = "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: application/json\r\nContent-Length: 15\r\n\r\n{\"status\":\"ok\"}";
                                let _ = socket.write_all(resp.as_bytes()).await;
                                return;
                            }
                        }
                    }
                }
                let resp = "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: application/json\r\nContent-Length: 15\r\n\r\n{\"status\":\"ok\"}";
                let _ = socket.write_all(resp.as_bytes()).await;
                return;
            }

            // Handle health check and wake
            if req_str.starts_with("GET") {
                if req_str.contains("/tasks") {
                    let list = tasks_clone.lock().await;
                    let summary: Vec<serde_json::Value> = list.iter().map(|t| {
                        let status_str = match &t.status {
                            DownloadStatus::Downloading => "Downloading".to_string(),
                            DownloadStatus::Completed => "Completed".to_string(),
                            DownloadStatus::Paused => "Paused".to_string(),
                            DownloadStatus::Queued => "Queued".to_string(),
                            DownloadStatus::Probing => "Probing".to_string(),
                            DownloadStatus::Cancelled => "Cancelled".to_string(),
                            DownloadStatus::Failed(e) => format!("Failed: {}", e),
                        };
                        serde_json::json!({
                            "id": t.id,
                            "filename": t.filename,
                            "url": t.url,
                            "status": status_str,
                            "target_file": t.target_file.to_string_lossy(),
                            "downloaded_bytes": t.downloaded_bytes,
                            "total_bytes": t.total_bytes,
                            "progress_percent": t.progress_percent,
                        })
                    }).collect();
                    let resp_body = serde_json::to_string_pretty(&summary).unwrap_or_else(|_| "[]".to_string());
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                        resp_body.len(),
                        resp_body
                    );
                    let _ = socket.write_all(response.as_bytes()).await;
                    return;
                }
                if req_str.contains("/wake") {
                    ctx_clone.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                    ctx_clone.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
                    ctx_clone.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));
                    ctx_clone.send_viewport_cmd(egui::ViewportCommand::Focus);
                    ctx_clone.send_viewport_cmd(egui::ViewportCommand::RequestUserAttention(egui::UserAttentionType::Critical));
                    ctx_clone.request_repaint();
                }
                let resp_body = "{\"status\":\"running\",\"app\":\"Rapid Download Manager\"}";
                let response = format!(
                    "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: {}\r\nAccess-Control-Allow-Private-Network: true\r\nAccess-Control-Allow-Credentials: true\r\nVary: Origin, Access-Control-Request-Private-Network\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    origin,
                    resp_body.len(),
                    resp_body
                );
                let _ = socket.write_all(response.as_bytes()).await;
                return;
            }

            // Extract JSON body
            if let Some(hend) = header_end {
                let body_slice = &buffer[hend..];
                if let Ok(body_str) = std::str::from_utf8(body_slice) {
                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(body_str.trim()) {
                        if let Some(url) = val.get("url").and_then(|u| u.as_str()) {
                            let custom_fn = val.get("filename")
                                .and_then(|f| f.as_str())
                                .map(|s| s.trim())
                                .filter(|s| !s.is_empty())
                                .filter(|s| !rapid_core::engine::is_generic_placeholder(s))
                                .map(|s| s.to_string());
                            let cookies_str = val.get("cookies").and_then(|c| c.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string());
                            let user_agent_str = val.get("user_agent").and_then(|ua| ua.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string());
                            let referrer_str = val.get("referrer").and_then(|r| r.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string());
                            let url_str = url.to_string();

                            if url_str.starts_with("blob:") {
                                eprintln!("[Rapid] Extension sent unresolvable blob: URL. Rejecting with HTTP 400.");
                                let err_resp = b"HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n{\"status\":\"error\",\"message\":\"blob_urls_not_supported\"}";
                                let _ = socket.write_all(err_resp).await;
                                return;
                            }

                            let is_gdrive = val.get("is_gdrive").and_then(|g| g.as_bool()).unwrap_or(false)
                                || url_str.contains("drive.google.com")
                                || url_str.contains("googleusercontent.com")
                                || url_str.contains("usercontent.google.com")
                                || url_str.contains("docs.google.com")
                                || url_str.contains("takeout-download-drive");

                            if let Some(ref c) = cookies_str {
                                if is_gdrive {
                                    save_google_cookies(c);
                                }
                            }
                            let cookies_str = if cookies_str.is_none() && is_gdrive {
                                load_google_cookies()
                            } else {
                                cookies_str
                            };

                            let is_youtube = val.get("is_youtube").and_then(|y| y.as_bool()).unwrap_or(false)
                                || val.get("is_video_platform").and_then(|p| p.as_bool()).unwrap_or(false)
                                || rapid_core::youtube::YoutubeResolver::is_extractable_platform(&url_str)
                                || referrer_str.as_deref().map(|r| rapid_core::youtube::YoutubeResolver::is_extractable_platform(r)).unwrap_or(false);

                            let url_str = if is_youtube && (rapid_core::youtube::YoutubeResolver::is_youtube(&url_str) || referrer_str.as_deref().map(|r| rapid_core::youtube::YoutubeResolver::is_youtube(r)).unwrap_or(false)) {
                                rapid_core::youtube::YoutubeResolver::canonicalize_url(&url_str, referrer_str.as_deref())
                            } else {
                                url_str
                            };

                            let segments_count = if is_gdrive { 1 } else { 8 };

                            let display_filename = if let Some(ref f) = custom_fn {
                                f.clone()
                            } else {
                                let clean = url_str.split('?').next().unwrap_or(&url_str).split('#').next().unwrap_or(&url_str);
                                if let Some(last_seg) = clean.trim_end_matches('/').rsplit('/').next() {
                                    if !last_seg.is_empty() && !last_seg.contains(':') && !rapid_core::engine::is_generic_placeholder(last_seg) {
                                        urlencoding::decode(last_seg).map(|d| d.into_owned()).unwrap_or_else(|_| last_seg.to_string())
                                    } else {
                                        "Resolving filename...".to_string()
                                    }
                                } else {
                                    "Resolving filename...".to_string()
                                }
                            };

                            let default_quality = if val.get("media_type").and_then(|m| m.as_str()) == Some("audio")
                                || display_filename.to_lowercase().ends_with(".mp3")
                                || display_filename.to_lowercase().ends_with(".m4a") {
                                rapid_core::youtube::DownloadQuality::AudioMp3
                            } else {
                                rapid_core::youtube::DownloadQuality::Best
                            };

                            let pending = PendingBrowserDownload {
                                url: url_str.clone(),
                                filename: display_filename.clone(),
                                dest_dir: dest_clone.to_string_lossy().to_string(),
                                segments: segments_count,
                                cookies: cookies_str.clone(),
                                user_agent: user_agent_str.clone(),
                                referrer: referrer_str.clone(),
                                is_gdrive,
                                is_youtube,
                                quality: default_quality,
                            };

                            let already_queued = if let Ok(q) = queue_clone.lock() {
                                q.iter().any(|item| item.url == url_str)
                            } else {
                                false
                            };

                            if !already_queued {
                                if let Ok(mut q) = queue_clone.lock() {
                                    q.push_back(pending);
                                }
                            }

                            // Immediately spawn background probe to detect real original filename
                            if custom_fn.is_none() || rapid_core::engine::is_generic_placeholder(&display_filename) {
                                let tx_p = resolver_tx_clone.clone();
                                let u_p = url_str.clone();
                                let c_p = cookies_str.clone();
                                let ua_p = user_agent_str.clone();
                                let r_p = referrer_str.clone();
                                let is_gd_p = is_gdrive;
                                let ctx_p = ctx_clone.clone();

                                tokio::spawn(async move {
                                    let mut cb = reqwest::Client::builder()
                                        .timeout(std::time::Duration::from_secs(12))
                                        .redirect(reqwest::redirect::Policy::limited(10));

                                    let mut hdrs = reqwest::header::HeaderMap::new();
                                    if let Some(ref ua) = ua_p {
                                        if let Ok(v) = reqwest::header::HeaderValue::from_str(ua) {
                                            hdrs.insert(reqwest::header::USER_AGENT, v);
                                        }
                                    }
                                    if let Some(ref r) = r_p {
                                        if let Ok(v) = reqwest::header::HeaderValue::from_str(r) {
                                            hdrs.insert(reqwest::header::REFERER, v);
                                        }
                                    }
                                    if let Some(ref c) = c_p {
                                        if let Ok(v) = reqwest::header::HeaderValue::from_str(c) {
                                            hdrs.insert(reqwest::header::COOKIE, v);
                                        }
                                    }
                                    cb = cb.default_headers(hdrs);

                                    if let Ok(client) = cb.build() {
                                        let detected_name = if is_gd_p {
                                            let res_type = rapid_core::gdrive::GDriveResolver::parse_resource_type(&u_p);
                                            if let rapid_core::gdrive::GDriveResourceType::File(file_id) = res_type {
                                                rapid_core::gdrive::GDriveResolver::resolve_file_download_url(&client, &file_id)
                                                    .await
                                                    .ok()
                                                    .map(|r| r.name)
                                            } else {
                                                None
                                            }
                                        } else if rapid_core::youtube::YoutubeResolver::is_youtube(&u_p) {
                                            rapid_core::youtube::YoutubeResolver::resolve_metadata(&u_p)
                                                .await
                                                .ok()
                                                .map(|m| m.clean_filename)
                                        } else {
                                            rapid_core::Probe::inspect(&client, &u_p)
                                                .await
                                                .ok()
                                                .map(|m| m.filename)
                                        };

                                        if let Some(fname) = detected_name {
                                            if !rapid_core::engine::is_generic_placeholder(&fname) {
                                                let _ = tx_p.send((u_p, fname));
                                                ctx_p.request_repaint();
                                            }
                                        }
                                    }
                                });
                            }

                            // Force window visible, restore from minimized, and elevate to AlwaysOnTop so it pops over Chrome
                            ctx_clone.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                            ctx_clone.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
                            ctx_clone.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));
                            ctx_clone.send_viewport_cmd(egui::ViewportCommand::Focus);
                            ctx_clone.send_viewport_cmd(egui::ViewportCommand::RequestUserAttention(egui::UserAttentionType::Critical));
                            ctx_clone.request_repaint();

                            let resp_body = "{\"status\":\"ok\"}";
                            let response = format!(
                                "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: {}\r\nAccess-Control-Allow-Private-Network: true\r\nAccess-Control-Allow-Credentials: true\r\nVary: Origin, Access-Control-Request-Private-Network\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                                origin,
                                resp_body.len(),
                                resp_body
                            );
                            let _ = socket.write_all(response.as_bytes()).await;
                            return;
                        }
                    }
                }
            }

            let response = format!(
                "HTTP/1.1 400 Bad Request\r\nAccess-Control-Allow-Origin: {}\r\nAccess-Control-Allow-Private-Network: true\r\nContent-Length: 0\r\n\r\n",
                origin
            );
            let _ = socket.write_all(response.as_bytes()).await;
        });
    }
}

fn render_status_action_btn(
    ui: &mut egui::Ui,
    icon: ModernIcon,
    label: &str,
    tooltip: &str,
    is_compact: bool,
) -> bool {
    let font_id = egui::FontId::proportional(10.5);
    let text_w = if is_compact {
        0.0
    } else {
        ui.fonts(|f| f.layout_no_wrap(label.to_string(), font_id.clone(), Color32::WHITE).size().x)
    };
    let btn_w = if is_compact { 22.0 } else { (text_w + 24.0).max(46.0) };
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(btn_w, 20.0), egui::Sense::click());
    let is_h = resp.hovered();

    let bg = if is_h {
        Color32::from_rgba_unmultiplied(139, 92, 246, 35)
    } else {
        Color32::from_rgba_unmultiplied(255, 255, 255, 6)
    };
    let border = if is_h {
        Color32::from_rgba_unmultiplied(167, 139, 250, 80)
    } else {
        Color32::from_rgba_unmultiplied(148, 163, 184, 25)
    };
    let text_col = if is_h {
        Color32::from_rgb(255, 255, 255)
    } else {
        Color32::from_rgb(180, 172, 205)
    };
    let icon_col = if is_h {
        Color32::from_rgb(196, 181, 253)
    } else {
        Color32::from_rgb(155, 145, 185)
    };

    ui.painter().rect(rect, egui::Rounding::same(4.0), bg, Stroke::new(1.0_f32, border));

    let icon_rect = if is_compact {
        egui::Rect::from_center_size(rect.center(), Vec2::new(10.0, 10.0))
    } else {
        egui::Rect::from_center_size(egui::pos2(rect.min.x + 10.0, rect.center().y), Vec2::new(10.0, 10.0))
    };
    draw_modern_icon(ui.painter(), icon, icon_rect, icon_col);

    if !is_compact {
        ui.painter().text(
            egui::pos2(rect.min.x + 20.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            font_id,
            text_col,
        );
    }

    resp
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text(tooltip)
        .clicked()
}

fn render_status_tray_pill(ui: &mut egui::Ui, is_compact: bool) {
    let font_id = egui::FontId::proportional(10.5);
    let label = if is_compact { "" } else { "Tray" };
    let text_w = if is_compact {
        0.0
    } else {
        ui.fonts(|f| f.layout_no_wrap(label.to_string(), font_id.clone(), Color32::WHITE).size().x) + 4.0
    };
    let total_w = if is_compact { 18.0 } else { 18.0 + text_w + 6.0 };
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(total_w, 20.0), egui::Sense::hover());

    let bg = Color32::from_rgba_unmultiplied(255, 255, 255, 6);
    let border = Color32::from_rgba_unmultiplied(148, 163, 184, 25);
    ui.painter().rect(rect, egui::Rounding::same(4.0), bg, Stroke::new(1.0_f32, border));

    let dot_center = if is_compact {
        rect.center()
    } else {
        egui::pos2(rect.min.x + 8.0, rect.center().y)
    };
    ui.painter().circle_filled(dot_center, 3.0, Color32::from_rgb(52, 211, 153));

    if !is_compact {
        ui.painter().text(
            egui::pos2(rect.min.x + 16.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            font_id,
            Color32::from_rgb(160, 150, 185),
        );
    }

    resp.on_hover_text("Rapid Download Manager is active in Windows Notification Area (System Tray)");
}

fn format_bytes(bytes: u64) -> String {
    let kb = bytes as f64 / 1024.0;
    if kb < 1024.0 {
        format!("{:.1} KB", kb)
    } else {
        let mb = kb / 1024.0;
        if mb < 1024.0 {
            format!("{:.1} MB", mb)
        } else {
            format!("{:.2} GB", mb / 1024.0)
        }
    }
}

fn format_speed(bps: u64) -> String {
    if bps == 0 {
        return "--".to_string();
    }
    let kb = bps as f64 / 1024.0;
    if kb < 1024.0 {
        format!("{:.1} KB/s", kb)
    } else {
        format!("{:.2} MB/s", kb / 1024.0)
    }
}

fn format_eta(seconds: u64) -> String {
    if seconds == 0 {
        return "--".to_string();
    }
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        let mins = seconds / 60;
        let secs = seconds % 60;
        format!("{}m {}s", mins, secs)
    } else {
        let hours = seconds / 3600;
        let mins = (seconds % 3600) / 60;
        format!("{}h {}m", hours, mins)
    }
}

fn dirs_or_fallback() -> PathBuf {
    rapid_core::AppPaths::default_downloads_dir()
}

fn main() -> Result<(), eframe::Error> {
    let _ = rapid_core::init_production_logging();
    log::info!("Rapid Download Manager v1.0.0 initializing...");

    let rt = Arc::new(Runtime::new().expect("Failed to initialize Tokio runtime"));

    let (win_w, win_h) = if std::env::var("RDM_CAPTURE_SCREENSHOTS").is_ok() {
        (1366.0_f32, 768.0_f32)
    } else {
        (1100.0_f32, 680.0_f32)
    };

    let raw_icon_128 = include_bytes!("../assets/icon_128.raw");
    let icon_data = egui::IconData {
        rgba: raw_icon_128.to_vec(),
        width: 128,
        height: 128,
    };

    let viewport = egui::ViewportBuilder::default()
        .with_inner_size([win_w, win_h])
        .with_min_inner_size([650.0, 380.0])
        .with_decorations(false)
        .with_taskbar(true)
        .with_resizable(true)
        .with_title("Rapid Download Manager")
        .with_icon(icon_data)
        .with_visible(true)
        .with_active(true);

    let options = eframe::NativeOptions {
        viewport,
        centered: true,
        ..Default::default()
    };

    let rt_clone = Arc::clone(&rt);
    let res = eframe::run_native(
        "Rapid Download Manager",
        options,
        Box::new(move |cc| {
            Ok(Box::new(RapidApp::new(cc, rt_clone)))
        }),
    );
    let _ = std::fs::write(rapid_core::AppPaths::logs_dir().join("run_native_result.log"), format!("run_native returned: {:?}", res));
    res
}
