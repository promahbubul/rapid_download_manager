#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console in release mode on Windows

use chrono::Utc;
use eframe::egui::{self, Color32, Margin, RichText, Stroke, Vec2};
use rapid_core::{DownloadConfig, DownloadStatus, DownloadTask};
use std::path::PathBuf;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

#[cfg(windows)]
mod tray;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModernIcon {
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
}

fn draw_modern_icon(painter: &egui::Painter, icon: ModernIcon, rect: egui::Rect, color: Color32) {
    let center = rect.center();
    let stroke = Stroke::new(1.5_f32, color);
    match icon {
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
            let radius = 5.0_f32;
            let angles = [0.0_f32, 1.57_f32, 3.14_f32, 4.71_f32];
            for i in 0..3 {
                let a1 = angles[i];
                let a2 = angles[i + 1];
                let pt1 = egui::pos2(center.x + radius * a1.cos(), center.y + radius * a1.sin());
                let pt2 = egui::pos2(center.x + radius * a2.cos(), center.y + radius * a2.sin());
                painter.line_segment([pt1, pt2], stroke);
            }
            let tip = egui::pos2(center.x + radius, center.y);
            painter.line_segment([tip, egui::pos2(tip.x + 2.5, tip.y - 3.0)], stroke);
            painter.line_segment([tip, egui::pos2(tip.x - 2.5, tip.y - 3.0)], stroke);
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
            let len = 4.5_f32;
            painter.line_segment(
                [egui::pos2(center.x - len, center.y), egui::pos2(center.x + len, center.y)],
                Stroke::new(1.8_f32, color),
            );
            painter.line_segment(
                [egui::pos2(center.x, center.y - len), egui::pos2(center.x, center.y + len)],
                Stroke::new(1.8_f32, color),
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
            let len = 6.0_f32;
            painter.line_segment(
                [egui::pos2(center.x - len, center.y), egui::pos2(center.x + len, center.y)],
                Stroke::new(1.8_f32, color),
            );
        }
        ModernIcon::WinMaximize => {
            let s = 5.5_f32;
            let box_rect = egui::Rect::from_center_size(center, Vec2::new(s * 2.0, s * 2.0));
            painter.rect_stroke(box_rect, 1.0, Stroke::new(1.8_f32, color));
        }
        ModernIcon::WinRestore => {
            let s = 4.5_f32;
            let b1 = egui::Rect::from_center_size(egui::pos2(center.x + 2.2, center.y - 2.2), Vec2::new(s * 2.0, s * 2.0));
            painter.rect_stroke(b1, 1.0, Stroke::new(1.4_f32, color));
            let b2 = egui::Rect::from_center_size(egui::pos2(center.x - 2.2, center.y + 2.2), Vec2::new(s * 2.0, s * 2.0));
            painter.rect_stroke(b2, 1.0, Stroke::new(1.8_f32, color));
        }
        ModernIcon::WinClose => {
            let d = 5.5_f32;
            painter.line_segment(
                [egui::pos2(center.x - d, center.y - d), egui::pos2(center.x + d, center.y + d)],
                Stroke::new(1.8_f32, color),
            );
            painter.line_segment(
                [egui::pos2(center.x + d, center.y - d), egui::pos2(center.x - d, center.y + d)],
                Stroke::new(1.8_f32, color),
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
        ui.painter().rect_filled(rect, 4.0, hover_bg_color);
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
        "pdf" | "doc" | "docx" | "txt" => ("DOC", GLASS_PRIMARY, Color32::from_rgb(30, 22, 58)),
        "mp4" | "mkv" | "avi" | "mov" | "webm" => ("VID", GLASS_SECONDARY, Color32::from_rgb(12, 36, 48)),
        "zip" | "rar" | "7z" | "tar" | "gz" => ("ZIP", Color32::from_rgb(168, 85, 247), Color32::from_rgb(32, 20, 52)),
        "mp3" | "wav" | "flac" | "aac" => ("AUD", Color32::from_rgb(56, 189, 248), Color32::from_rgb(14, 34, 52)),
        "exe" | "msi" | "iso" => ("BIN", Color32::from_rgb(99, 102, 241), Color32::from_rgb(24, 26, 56)),
        "jpg" | "png" | "gif" | "webp" | "svg" => ("IMG", Color32::from_rgb(236, 72, 153), Color32::from_rgb(46, 20, 38)),
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

    // Add Download Dialog State
    show_add_dialog: bool,
    input_url: String,
    input_dest: String,
    input_segments: usize,
    add_error: Option<String>,
    is_probing: bool,
    selected_tasks: HashSet<String>,
    show_delete_modal: bool,
    tasks_to_delete: HashSet<String>,

    #[cfg(windows)]
    tray_handle: Option<tray::TrayHandle>,
}

impl RapidApp {
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

        let default_download_dir = dirs_or_fallback();

        let mut initial_tasks = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&default_download_dir) {
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
                            };
                            initial_tasks.push(ui);
                        }
                    }
                }
            }
        }
        
        let tasks_arc = Arc::new(tokio::sync::Mutex::new(initial_tasks));


        // Start local browser extension receiver on 127.0.0.1:9669
        let tasks_for_server = Arc::clone(&tasks_arc);
        let rt_for_server = Arc::clone(&rt);
        let dest_for_server = default_download_dir.clone();

        rt.spawn(async move {
            run_extension_server(tasks_for_server, rt_for_server, dest_for_server).await;
        });

        #[cfg(windows)]
        let tray_handle = Some(tray::TrayHandle::new());

        Self {
            tokio_rt: rt,
            tasks: tasks_arc,
            selected_task_index: None,
            selected_filter: FilterCategory::All,
            search_query: String::new(),
            splash_start: Instant::now(),
            splash_duration: Duration::from_millis(1800),
            show_add_dialog: false,
            input_url: String::new(),
            input_dest: default_download_dir.to_string_lossy().to_string(),
            input_segments: 8,
            add_error: None,
            is_probing: false,
            selected_tasks: HashSet::new(),
            show_delete_modal: false,
            tasks_to_delete: HashSet::new(),
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

    fn start_new_download(&mut self, url: String, dest_dir: PathBuf, segments: usize) {
        let rt = Arc::clone(&self.tokio_rt);
        let tasks_arc = Arc::clone(&self.tasks);
        self.is_probing = true;
        self.add_error = None;

        spawn_download_task(url, dest_dir, segments, None, None, None, rt, tasks_arc);

        self.is_probing = false;
        self.show_add_dialog = false;
        self.input_url.clear();
    }

    fn resume_download(&mut self, task_index: usize) {
        let rt = Arc::clone(&self.tokio_rt);
        let tasks_arc = Arc::clone(&self.tasks);

        let (task_id, url, target_file, filename, segments_count) = {
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
            )
        };

        let dest_dir = target_file
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));

        rt.spawn(async move {
            let config = DownloadConfig {
                url: url.clone(),
                output_dir: dest_dir,
                custom_filename: Some(filename.clone()),
                num_segments: segments_count,
                ..Default::default()
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

        let (task_id, url, target_file, filename, segments_count) = {
            let Ok(mut list) = self.tasks.try_lock() else { return; };
            if task_index >= list.len() { return; }
            let task = &mut list[task_index];

            if let Some(ref handle) = task.task_handle {
                handle.pause();
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
            )
        };

        let dest_dir = target_file
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));

        rt.spawn(async move {
            let _ = tokio::fs::remove_file(&target_file).await;
            let manifest = rapid_core::DownloadTaskState::manifest_path(&target_file);
            let _ = tokio::fs::remove_file(&manifest).await;

            let config = DownloadConfig {
                url: url.clone(),
                output_dir: dest_dir,
                custom_filename: Some(filename.clone()),
                num_segments: segments_count,
                ..Default::default()
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
                    task.status = DownloadStatus::Paused;
                    task.speed_bps = 0;
                    task.eta_seconds = None;
                }
            }
        }
    }

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

    // Render Animated Splash Screen on startup
    fn render_splash_screen(&self, ctx: &egui::Context, elapsed: Duration) {
        let progress = (elapsed.as_secs_f32() / 1.8).clamp(0.0, 1.0);
        let time = elapsed.as_secs_f32();

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(VELVET_BLACK))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    let avail_h = ui.available_height();
                    ui.add_space((avail_h * 0.2).max(40.0));

                    // Animated Pulsating Lightning Symbol with concentric glow rings
                    let pulse_scale = (time * 4.0).sin() * 0.08 + 1.0;
                    let (rect, _response) = ui.allocate_exact_size(Vec2::new(120.0, 120.0), egui::Sense::hover());
                    let center = rect.center();

                    let painter = ui.painter();

                    // Expanding outer glow ripple in Neon Cyan (#06B6D4)
                    let ripple_phase = (time * 1.6) % 1.0;
                    let ripple_radius = 40.0 + ripple_phase * 35.0;
                    let ripple_alpha = ((1.0 - ripple_phase) * 140.0) as u8;
                    painter.circle_stroke(
                        center,
                        ripple_radius,
                        Stroke::new(2.0_f32, Color32::from_rgba_unmultiplied(6, 182, 212, ripple_alpha)),
                    );

                    // Inner pulsing halo in Electric Violet (#8B5CF6)
                    painter.circle_filled(
                        center,
                        38.0 * pulse_scale,
                        Color32::from_rgba_unmultiplied(139, 92, 246, 35),
                    );
                    painter.circle_stroke(
                        center,
                        40.0 * pulse_scale,
                        Stroke::new(2.5_f32, GLASS_PRIMARY),
                    );

                    // Centered Lightning Bolt icon
                    painter.text(
                        center,
                        egui::Align2::CENTER_CENTER,
                        "⚡",
                        egui::FontId::proportional(44.0 * pulse_scale),
                        GLASS_PRIMARY,
                    );

                    ui.add_space(26.0);
                    ui.label(
                        RichText::new("RAPID DOWNLOAD MANAGER")
                            .size(24.0)
                            .strong()
                            .color(GLASS_PRIMARY),
                    );

                    ui.add_space(4.0);
                    ui.label(
                        RichText::new("Futuristic • Elegant • Premium • Glassmorphism Edition")
                            .size(13.0)
                            .color(GLASS_SECONDARY),
                    );

                    ui.add_space(28.0);
                    let bar = egui::ProgressBar::new(progress)
                        .desired_width(320.0)
                        .fill(GLASS_SECONDARY);
                    ui.add(bar);

                    ui.add_space(10.0);
                    let status_text = if progress < 0.35 {
                        "Initializing multi-stream segment matrix..."
                    } else if progress < 0.75 {
                        "Connecting browser extension bridge on port 9669..."
                    } else {
                        "Engine ready. Launching Command Dock..."
                    };
                    ui.label(
                        RichText::new(status_text)
                            .size(12.0)
                            .color(PINK_MUTED),
                    );
                });
            });
    }

    // Render Custom Frameless Window Title Bar
    fn render_custom_title_bar(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("custom_window_title_bar")
            .frame(
                egui::Frame::none()
                    .fill(GLASS_BG)
                    .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                    .inner_margin(Margin::symmetric(14.0, 6.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(6.0, 0.0);

                    // 1. App Icon & Brand Title (Left Side)
                    let (icon_rect, _) = ui.allocate_exact_size(Vec2::new(28.0, 28.0), egui::Sense::hover());
                    let center = icon_rect.center();
                    ui.painter().circle(
                        center,
                        13.5,
                        Color32::from_rgb(28, 20, 58),
                        Stroke::new(1.2_f32, GLASS_PRIMARY),
                    );
                    ui.painter().text(
                        center,
                        egui::Align2::CENTER_CENTER,
                        "⚡",
                        egui::FontId::proportional(14.0),
                        GLASS_PRIMARY_HOVER,
                    );

                    ui.add_space(3.0);
                    ui.label(
                        RichText::new("Rapid Download Manager")
                            .size(13.5)
                            .strong()
                            .color(GLASS_TEXT),
                    );

                    // 2. Drag & Move Region (takes up flexible remaining width)
                    let remaining_w = (ui.available_width() - 150.0).max(20.0);
                    let (_drag_rect, drag_resp) = ui.allocate_exact_size(
                        Vec2::new(remaining_w, 34.0),
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

                    // 3. Window Caption Buttons (Right-to-Left: Close, Maximize/Restore, Minimize)
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));

                        // Close Button (✕) -> Minimizes to System Tray (IDM style)
                        if window_caption_button(
                            ui,
                            ModernIcon::WinClose,
                            Vec2::new(46.0, 34.0),
                            GLASS_MUTED,
                            Color32::WHITE,
                            Color32::from_rgb(239, 68, 68),
                            "Close (Minimizes to System Tray)",
                        ) {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
                        }

                        // Maximize / Restore Button (▢ / ❐)
                        let (max_icon, max_tip) = if is_maximized {
                            (ModernIcon::WinRestore, "Restore Window")
                        } else {
                            (ModernIcon::WinMaximize, "Maximize Window")
                        };

                        if window_caption_button(
                            ui,
                            max_icon,
                            Vec2::new(46.0, 34.0),
                            GLASS_MUTED,
                            GLASS_SECONDARY,
                            Color32::from_rgb(26, 36, 62),
                            max_tip,
                        ) {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
                        }

                        // Minimize Button (—)
                        if window_caption_button(
                            ui,
                            ModernIcon::WinMinimize,
                            Vec2::new(46.0, 34.0),
                            GLASS_MUTED,
                            GLASS_SECONDARY,
                            Color32::from_rgb(26, 36, 62),
                            "Minimize to Taskbar",
                        ) {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                        }
                    });
                });
            });
    }
}

impl eframe::App for RapidApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render custom frameless title bar at the top of the window
        self.render_custom_title_bar(ctx);

        // Handle animated splash screen on startup
        let elapsed = self.splash_start.elapsed();
        if elapsed < self.splash_duration {
            ctx.request_repaint();
            self.render_splash_screen(ctx, elapsed);
            return;
        }

        // Request repaint continuously if any download is active for smooth progress bars
        ctx.request_repaint_after(Duration::from_millis(100));

        let mut action_resume: Option<usize> = None;
        let mut action_pause: Option<usize> = None;
        let mut action_redownload: Option<usize> = None;
        let mut action_remove: Option<usize> = None;
        let mut action_open_folder: Option<usize> = None;
        let mut action_open_file: Option<usize> = None;
        let mut action_copy_url: Option<String> = None;

        // Auto-select first item if none selected and downloads exist
        if self.selected_task_index.is_none() {
            if let Ok(tasks) = self.tasks.try_lock() {
                if !tasks.is_empty() {
                    self.selected_task_index = Some(0);
                }
            }
        }

        // Metrics calculations for bottom shortcut dock
        let mut total_speed_bps: u64 = 0;
        let mut active_count = 0;
        let mut paused_count = 0;
        let mut finished_count = 0;
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
                    _ => {}
                }
            }
        }

        let mut do_pause_all = false;
        let mut do_resume_all = false;

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
                        // Primary Action Button: "+ Add Download"
                        let (btn_rect, btn_resp) = ui.allocate_exact_size(Vec2::new(140.0, 32.0), egui::Sense::click());
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
                            "+ Add Download",
                            egui::FontId::proportional(12.5),
                            Color32::WHITE,
                        );

                        if btn_resp.on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Add a new accelerated download URL").clicked() {
                            self.show_add_dialog = true;
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
                    });
                });

                // Contextual Toolbar (Modern Dynamic Sub-Bar for selected download)
                let (has_selection, is_downloading, is_resumable) = {
                    if let Ok(ref tasks) = self.tasks.try_lock() {
                        if let Some(idx) = self.selected_task_index {
                            if idx < tasks.len() {
                                let task = &tasks[idx];
                                let downloading = task.status == DownloadStatus::Downloading;
                                let resumable = matches!(task.status, DownloadStatus::Paused | DownloadStatus::Failed(_));
                                (true, downloading, resumable)
                            } else {
                                (false, false, false)
                            }
                        } else {
                            (false, false, false)
                        }
                    } else {
                        (false, false, false)
                    }
                };

                if has_selection {
                    ui.add_space(6.0);
                    ui.separator();
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Selected:").size(11.0).color(Color32::from_rgb(160, 140, 170)));
                        if let Ok(ref tasks) = self.tasks.try_lock() {
                            if let Some(idx) = self.selected_task_index {
                                if idx < tasks.len() {
                                    let task = &tasks[idx];
                                    ui.label(RichText::new(&task.filename).size(11.5).strong().color(PINK_PASTEL));
                                }
                            }
                        }

                        ui.add_space(10.0);
                        if is_resumable && modern_icon_button(ui, ModernIcon::Play, Vec2::new(26.0, 22.0), STATUS_COMPLETED, PINK_NEON, "Resume download") {
                            if let Some(idx) = self.selected_task_index {
                                action_resume = Some(idx);
                            }
                        }
                        if is_downloading && modern_icon_button(ui, ModernIcon::Pause, Vec2::new(26.0, 22.0), STATUS_PAUSED, PINK_NEON, "Pause download") {
                            if let Some(idx) = self.selected_task_index {
                                action_pause = Some(idx);
                            }
                        }
                        if modern_icon_button(ui, ModernIcon::Folder, Vec2::new(26.0, 22.0), PINK_PASTEL, PINK_NEON, "Open containing folder") {
                            if let Some(idx) = self.selected_task_index {
                                action_open_folder = Some(idx);
                            }
                        }
                        if modern_icon_button(ui, ModernIcon::Refresh, Vec2::new(26.0, 22.0), PINK_PASTEL, PINK_NEON, "Redownload from start") {
                            if let Some(idx) = self.selected_task_index {
                                action_redownload = Some(idx);
                            }
                        }
                        if modern_icon_button(ui, ModernIcon::Trash, Vec2::new(26.0, 22.0), STATUS_FAILED, Color32::from_rgb(255, 40, 80), "Delete download") {
                            if let Some(idx) = self.selected_task_index {
                                action_remove = Some(idx);
                            }
                        }
                    });
                }
            });

        // Custom Glassmorphism Status Bar
        egui::TopBottomPanel::bottom("custom_status_bar")
            .frame(
                egui::Frame::none()
                    .fill(GLASS_BG)
                    .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                    .inner_margin(Margin::symmetric(10.0, 5.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(6.0, 0.0);

                    // 1. Live Engine Beacon Chip (Vector pulse beacon)
                    render_custom_chip_with_icon(
                        ui,
                        ModernIcon::PulseBeacon,
                        "Engine:",
                        "127.0.0.1:9669",
                        Color32::from_rgb(22, 18, 48),
                        GLASS_PRIMARY,
                        GLASS_PRIMARY,
                        "Rapid Engine Background HTTP Listener is Active on 127.0.0.1:9669",
                    );

                    // 2. Aggregate Live Speed Chip
                    if total_speed_bps > 0 {
                        render_custom_chip_with_icon(
                            ui,
                            ModernIcon::SpeedGauge,
                            "Speed:",
                            &format_speed(total_speed_bps),
                            Color32::from_rgb(12, 34, 46),
                            GLASS_SECONDARY,
                            GLASS_SECONDARY,
                            "Live Aggregate Download Speed",
                        );
                    } else {
                        render_custom_chip_with_icon(
                            ui,
                            ModernIcon::SpeedGauge,
                            "Speed:",
                            "0.0 KB/s",
                            GLASS_CARD,
                            GLASS_BORDER,
                            GLASS_MUTED,
                            "Download Speed (Idle)",
                        );
                    }

                    // 3. Active Tasks Chip
                    render_custom_chip_with_icon(
                        ui,
                        ModernIcon::Play,
                        "Active:",
                        &active_count.to_string(),
                        if active_count > 0 { Color32::from_rgb(12, 34, 46) } else { GLASS_CARD },
                        if active_count > 0 { GLASS_SECONDARY } else { GLASS_BORDER },
                        if active_count > 0 { GLASS_TEXT } else { GLASS_MUTED },
                        "Number of active downloading files",
                    );

                    // 4. Paused Tasks Chip
                    render_custom_chip_with_icon(
                        ui,
                        ModernIcon::Pause,
                        "Paused:",
                        &paused_count.to_string(),
                        if paused_count > 0 { Color32::from_rgb(40, 26, 16) } else { GLASS_CARD },
                        if paused_count > 0 { STATUS_PAUSED } else { GLASS_BORDER },
                        if paused_count > 0 { STATUS_PAUSED } else { GLASS_MUTED },
                        "Number of paused downloads",
                    );

                    // 5. Finished Tasks Chip
                    render_custom_chip_with_icon(
                        ui,
                        ModernIcon::Check,
                        "Done:",
                        &finished_count.to_string(),
                        if finished_count > 0 { Color32::from_rgb(14, 36, 26) } else { GLASS_CARD },
                        if finished_count > 0 { STATUS_COMPLETED } else { GLASS_BORDER },
                        if finished_count > 0 { STATUS_COMPLETED } else { GLASS_MUTED },
                        "Completed downloads in this session",
                    );

                    // 6. Total Downloaded Chip
                    render_custom_chip_plain(
                        ui,
                        "Total:",
                        &format_bytes(total_downloaded),
                        GLASS_CARD,
                        GLASS_BORDER,
                        GLASS_TEXT,
                        "Total data downloaded across all tasks",
                    );

                    // Right-aligned actions
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        render_custom_chip_plain(
                            ui,
                            "Tray:",
                            "Active",
                            GLASS_CARD,
                            GLASS_BORDER,
                            GLASS_MUTED,
                            "Rapid Download Manager is active in Windows Notification Area (System Tray)",
                        );

                        if render_custom_btn_chip_modern(
                            ui,
                            ModernIcon::Folder,
                            "Downloads",
                            "Open Downloads directory in Windows File Explorer",
                        ) {
                            let _ = open::that(dirs_or_fallback());
                        }

                        if render_custom_btn_chip_modern(
                            ui,
                            ModernIcon::Plus,
                            "Extension Hook",
                            "Install & integrate Chrome, Edge & Brave Extension",
                        ) {
                            let _ = open::that("install_extension.bat");
                        }
                    });
                });
            });

        // Bottom Details Panel (Connection / Segment Progress - only when active segments exist)
        let has_segments_to_show = {
            if let Ok(tasks) = self.tasks.try_lock() {
                if let Some(idx) = self.selected_task_index {
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
                        .fill(VELVET_BLACK)
                        .stroke(Stroke::new(1.0_f32, VELVET_BORDER))
                        .inner_margin(Margin::same(10.0)),
                )
                .show(ctx, |ui| {
                    ui.label(RichText::new("🧵 Multi-Part Stream Connections").strong().color(PINK_ROSE).size(11.5));
                    ui.add_space(4.0);

                    if let Ok(tasks) = self.tasks.try_lock() {
                        if let Some(idx) = self.selected_task_index {
                            if idx < tasks.len() {
                                let task = &tasks[idx];

                                // Show error notification banner if failed
                                if let DownloadStatus::Failed(ref err) = task.status {
                                    egui::Frame::none()
                                        .fill(Color32::from_rgb(45, 18, 24))
                                        .stroke(Stroke::new(1.0_f32, STATUS_FAILED))
                                        .inner_margin(Margin::symmetric(8.0, 6.0))
                                        .rounding(egui::Rounding::same(4.0))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(RichText::new(format!("⚠ Error: {}", err)).color(STATUS_FAILED).strong());
                                                if ui.button(RichText::new("🔄 Retry Now").strong()).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                    action_resume = Some(idx);
                                                }
                                            });
                                        });
                                    ui.add_space(6.0);
                                } else if task.status == DownloadStatus::Paused {
                                    egui::Frame::none()
                                        .fill(Color32::from_rgb(40, 24, 18))
                                        .stroke(Stroke::new(1.0_f32, STATUS_PAUSED))
                                        .inner_margin(Margin::symmetric(8.0, 6.0))
                                        .rounding(egui::Rounding::same(4.0))
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

                                let avail_width = ui.available_width();
                                let cols = ((avail_width / 260.0).floor() as usize).clamp(1, 6);
                                let seg_bar_width = ((avail_width / cols as f32) - 130.0).clamp(60.0, 160.0);

                                egui::Grid::new("segment_grid")
                                    .spacing(Vec2::new(12.0, 6.0))
                                    .show(ui, |ui| {
                                        for (i, seg) in task.segments.iter().enumerate() {
                                            ui.label(RichText::new(format!("Part {}:", i + 1)).size(11.0).color(PINK_PASTEL));
                                            let ratio = seg.progress_ratio();
                                            let (fill_col, label_text) = if seg.is_complete {
                                                (STATUS_COMPLETED, "✔ Done".to_string())
                                            } else if ratio > 0.0 {
                                                (STATUS_DOWNLOADING, format!("{:.0}%", ratio * 100.0))
                                            } else {
                                                (Color32::from_rgb(60, 40, 65), "0%".to_string())
                                            };

                                            let bar = egui::ProgressBar::new(ratio)
                                                .text(label_text)
                                                .desired_width(seg_bar_width)
                                                .fill(fill_col);
                                            ui.add(bar);
                                            ui.label(RichText::new(format!("{:.1} / {:.1} MB", seg.downloaded_bytes as f64 / 1_048_576.0, seg.total_bytes() as f64 / 1_048_576.0)).size(11.0).color(Color32::from_rgb(220, 205, 230)));
                                            if (i + 1) % cols == 0 {
                                                ui.end_row();
                                            }
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
                            FilterCategory::Active => task.status == DownloadStatus::Downloading,
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
                        ui.vertical_centered(|ui| {
                            ui.add_space(80.0);
                            let (icon_rect, _) = ui.allocate_exact_size(Vec2::new(48.0, 48.0), egui::Sense::hover());
                            draw_modern_icon(ui.painter(), ModernIcon::Plus, icon_rect, PINK_ROSE);
                            ui.add_space(12.0);
                            ui.label(RichText::new("No downloads in queue").size(18.0).color(PINK_PASTEL).strong());
                            ui.add_space(4.0);
                            ui.label(RichText::new("Add a URL or trigger from Chrome / Edge / Brave extension").size(12.0).color(PINK_MUTED));
                            ui.add_space(16.0);
                            let add_btn = egui::Button::new(RichText::new("+ Add your first download").size(13.0).strong().color(Color32::WHITE))
                                .fill(PINK_NEON)
                                .rounding(egui::Rounding::same(6.0));
                            if ui.add(add_btn).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                self.show_add_dialog = true;
                            }
                        });
                    } else if filtered_indices.is_empty() {
                        ui.vertical_centered(|ui| {
                            ui.add_space(80.0);
                            let (icon_rect, _) = ui.allocate_exact_size(Vec2::new(36.0, 36.0), egui::Sense::hover());
                            draw_modern_icon(ui.painter(), ModernIcon::Search, icon_rect, PINK_ROSE);
                            ui.add_space(12.0);
                            ui.label(RichText::new("No downloads match the current filter").size(16.0).color(PINK_PASTEL));
                        });
                    } else {
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                egui_extras::TableBuilder::new(ui)
                                    .striped(true)
                                    .resizable(true)
                                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                    .column(egui_extras::Column::exact(30.0)) // Checkbox
                                    .column(egui_extras::Column::remainder().at_least(220.0).resizable(true)) // Filename
                                    .column(egui_extras::Column::initial(80.0).at_least(65.0))                 // Size
                                    .column(egui_extras::Column::initial(95.0).at_least(80.0))                 // Status
                                    .column(egui_extras::Column::initial(160.0).at_least(110.0).resizable(true)) // Progress
                                    .column(egui_extras::Column::initial(90.0).at_least(70.0))                 // Speed
                                    .column(egui_extras::Column::initial(65.0).at_least(50.0))                 // ETA
                                    .column(egui_extras::Column::exact(140.0))                                 // Actions
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
                                                } else {
                                                    for &i in &filtered_indices {
                                                        self.selected_tasks.remove(&tasks[i].id);
                                                    }
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
                                            let is_sel = selected == Some(i);
                                            let is_downloading = item.status == DownloadStatus::Downloading;
                                            let is_resumable = matches!(item.status, DownloadStatus::Paused | DownloadStatus::Failed(_));
                                            let is_complete = item.status == DownloadStatus::Completed;

                                            row.set_selected(is_sel);
                                            
                                            row.col(|ui| {
                                                let mut check = self.selected_tasks.contains(&item.id);
                                                if ui.checkbox(&mut check, "").clicked() {
                                                    if check {
                                                        self.selected_tasks.insert(item.id.clone());
                                                    } else {
                                                        self.selected_tasks.remove(&item.id);
                                                    }
                                                }
                                            });

                                            // 1. Filename column with modern badge + click to open
                                            row.col(|ui| {
                                                ui.horizontal(|ui| {
                                                    let ext = item.filename.rsplit('.').next().unwrap_or("").to_lowercase();
                                                    render_file_type_badge(ui, &ext);
                                                    ui.add_space(4.0);

                                                    if is_complete {
                                                        let label_text = RichText::new(&item.filename)
                                                            .color(STATUS_COMPLETED)
                                                            .underline();
                                                        let resp = ui.add(egui::Label::new(label_text).sense(egui::Sense::click()))
                                                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                                                            .on_hover_text("✔ Download complete! Click to open file directly");
                                                        if resp.clicked() {
                                                            action_open_file = Some(i);
                                                        }
                                                        resp.context_menu(|ui| {
                                                            if ui.button("▶ Open File").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                                action_open_file = Some(i);
                                                                ui.close_menu();
                                                            }
                                                            if ui.button("📁 Open Containing Folder").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                                action_open_folder = Some(i);
                                                                ui.close_menu();
                                                            }
                                                            if ui.button("📋 Copy URL").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                                action_copy_url = Some(item.url.clone());
                                                                ui.close_menu();
                                                            }
                                                            ui.separator();
                                                            if ui.button("🔄 Redownload").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                                action_redownload = Some(i);
                                                                ui.close_menu();
                                                            }
                                                            if ui.button(RichText::new("🗑 Delete").color(STATUS_FAILED)).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                                action_remove = Some(i);
                                                                ui.close_menu();
                                                            }
                                                        });
                                                    } else {
                                                        let name_col = if is_sel {
                                                            Color32::WHITE
                                                        } else {
                                                            Color32::from_rgb(240, 232, 248)
                                                        };
                                                        let label_text = RichText::new(&item.filename)
                                                            .color(name_col)
                                                            .strong();
                                                        let resp = ui.add(egui::Label::new(label_text).sense(egui::Sense::click()))
                                                            .on_hover_cursor(egui::CursorIcon::PointingHand);
                                                        if resp.clicked() {
                                                            selected = Some(i);
                                                        }
                                                        resp.context_menu(|ui| {
                                                            if is_downloading {
                                                                if ui.button("⏸ Pause").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                                    action_pause = Some(i);
                                                                    ui.close_menu();
                                                                }
                                                            } else if is_resumable {
                                                                if ui.button("▶ Resume").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                                    action_resume = Some(i);
                                                                    ui.close_menu();
                                                                }
                                                            }
                                                            if ui.button("📁 Open Containing Folder").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                                action_open_folder = Some(i);
                                                                ui.close_menu();
                                                            }
                                                            if ui.button("📋 Copy URL").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                                action_copy_url = Some(item.url.clone());
                                                                ui.close_menu();
                                                            }
                                                            ui.separator();
                                                            if ui.button("🔄 Redownload").on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                                action_redownload = Some(i);
                                                                ui.close_menu();
                                                            }
                                                            if ui.button(RichText::new("🗑 Delete").color(STATUS_FAILED)).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                                action_remove = Some(i);
                                                                ui.close_menu();
                                                            }
                                                        });
                                                    }

                                                    if is_complete {
                                                        if modern_icon_button(ui, ModernIcon::ExternalFile, Vec2::new(20.0, 18.0), PINK_ROSE, PINK_NEON, "Open / Launch file") {
                                                            action_open_file = Some(i);
                                                        }
                                                    }

                                                    if modern_icon_button(ui, ModernIcon::Folder, Vec2::new(20.0, 18.0), PINK_MUTED, PINK_ROSE, "Open containing folder") {
                                                        action_open_folder = Some(i);
                                                    }
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
                                                ui.label(RichText::new(sz_text).color(Color32::from_rgb(220, 200, 225)));
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
                                                let label = ui.label(RichText::new(text).color(color).strong());
                                                if let Some(tip) = tooltip {
                                                    label.on_hover_text(tip);
                                                }
                                            });

                                            // 4. Progress bar with high-contrast text and dark recessed track
                                            row.col(|ui| {
                                                let ratio = (item.progress_percent / 100.0).clamp(0.0, 1.0);
                                                let (bar_color, pct_label) = match item.status {
                                                    DownloadStatus::Completed => (STATUS_COMPLETED, "100%".to_string()),
                                                    DownloadStatus::Downloading => (STATUS_DOWNLOADING, format!("{:.1}%", item.progress_percent)),
                                                    DownloadStatus::Paused => (STATUS_PAUSED, format!("{:.1}% (Paused)", item.progress_percent)),
                                                    DownloadStatus::Failed(_) => (STATUS_FAILED, "Failed".to_string()),
                                                    _ => (Color32::from_rgb(80, 50, 80), format!("{:.0}%", item.progress_percent)),
                                                };
                                                let bar = egui::ProgressBar::new(ratio)
                                                    .text(pct_label)
                                                    .fill(bar_color);
                                                ui.add(bar);
                                            });

                                            // 5. Speed column
                                            row.col(|ui| {
                                                if item.status == DownloadStatus::Downloading {
                                                    ui.label(RichText::new(format_speed(item.speed_bps)).color(PINK_NEON).strong());
                                                } else {
                                                    ui.label(RichText::new("--").color(PINK_MUTED));
                                                }
                                            });

                                            // 6. ETA column
                                            row.col(|ui| {
                                                if item.status == DownloadStatus::Downloading {
                                                    if let Some(eta) = item.eta_seconds {
                                                        ui.label(RichText::new(format_eta(eta)).color(PINK_PASTEL));
                                                    } else {
                                                        ui.label(RichText::new("--").color(PINK_MUTED));
                                                    }
                                                } else {
                                                    ui.label(RichText::new("--").color(PINK_MUTED));
                                                }
                                            });

                                            // 7. Modern Vector Action Buttons
                                            row.col(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);
                                                    if is_downloading {
                                                        if modern_icon_button(ui, ModernIcon::Pause, Vec2::new(24.0, 22.0), STATUS_PAUSED, PINK_NEON, "Pause") {
                                                            action_pause = Some(i);
                                                        }
                                                    } else if is_resumable {
                                                        if modern_icon_button(ui, ModernIcon::Play, Vec2::new(24.0, 22.0), STATUS_COMPLETED, PINK_NEON, "Resume") {
                                                            action_resume = Some(i);
                                                        }
                                                    }

                                                    if modern_icon_button(ui, ModernIcon::Refresh, Vec2::new(24.0, 22.0), PINK_PASTEL, PINK_NEON, "Redownload") {
                                                        action_redownload = Some(i);
                                                    }

                                                    if modern_icon_button(ui, ModernIcon::Trash, Vec2::new(24.0, 22.0), STATUS_FAILED, Color32::from_rgb(255, 40, 80), "Delete") {
                                                        action_remove = Some(i);
                                                    }
                                                });
                                            });
                                        });
                                    });
                            });
                    }
                }

                self.selected_task_index = selected;
            });

        // Modern Cyber-Obsidian Add Download Modal Dialog
        if self.show_add_dialog {
            // Backdrop dimming scrim
            let screen_rect = ctx.screen_rect();
            let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::new("modal_backdrop")));
            painter.rect_filled(screen_rect, 0.0, Color32::from_rgba_unmultiplied(2, 6, 23, 220));

            let modal_frame = egui::Frame::none()
                .fill(GLASS_SURFACE)
                .stroke(Stroke::new(1.8_f32, GLASS_PRIMARY))
                .rounding(egui::Rounding::same(12.0))
                .inner_margin(Margin::same(20.0))
                .shadow(egui::epaint::Shadow {
                    offset: [0.0, 8.0].into(),
                    blur: 24.0,
                    spread: 2.0,
                    color: Color32::from_black_alpha(220),
                });

            let mut close_modal = false;
            let mut start_download_req = false;

            egui::Window::new("add_download_modal")
                .title_bar(false)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .fixed_size(Vec2::new(520.0, 360.0))
                .frame(modal_frame)
                .show(ctx, |ui| {
                    // Header Section
                    ui.horizontal(|ui| {
                        let (icon_rect, _) = ui.allocate_exact_size(Vec2::new(22.0, 22.0), egui::Sense::hover());
                        draw_modern_icon(ui.painter(), ModernIcon::Plus, icon_rect, GLASS_PRIMARY);
                        ui.add_space(4.0);
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new("Add New Download")
                                    .size(17.0)
                                    .color(GLASS_TEXT)
                                    .strong(),
                            );
                            ui.label(
                                RichText::new("Accelerated Multi-Stream Engine • High Speed")
                                    .size(11.0)
                                    .color(GLASS_SECONDARY),
                            );
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let close_btn = ui.add(
                                egui::Button::new(RichText::new("✕").size(14.0).color(GLASS_MUTED))
                                    .fill(GLASS_CARD)
                                    .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                                    .rounding(egui::Rounding::same(6.0)),
                            );
                            if close_btn.on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Close dialog").clicked() {
                                close_modal = true;
                            }
                        });
                    });

                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(10.0);

                    // Field 1: Download URL
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Download URL")
                                .size(12.5)
                                .color(GLASS_TEXT)
                                .strong(),
                        );
                        ui.label(
                            RichText::new("(HTTP / HTTPS direct link)")
                                .size(11.0)
                                .color(GLASS_MUTED),
                        );
                    });

                    ui.add_space(4.0);
                    egui::Frame::none()
                        .fill(GLASS_BG)
                        .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                        .rounding(egui::Rounding::same(6.0))
                        .inner_margin(Margin::symmetric(10.0, 8.0))
                        .show(ui, |ui| {
                            let edit = egui::TextEdit::singleline(&mut self.input_url)
                                .hint_text("Paste URL here e.g. https://example.com/file.zip")
                                .font(egui::TextStyle::Body)
                                .desired_width(ui.available_width());
                            ui.add(edit);
                        });

                    ui.add_space(10.0);

                    // Field 2: Save Destination Directory
                    ui.label(
                        RichText::new("Save Destination")
                            .size(12.5)
                            .color(GLASS_TEXT)
                            .strong(),
                    );

                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        egui::Frame::none()
                            .fill(GLASS_BG)
                            .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                            .rounding(egui::Rounding::same(6.0))
                            .inner_margin(Margin::symmetric(10.0, 7.0))
                            .show(ui, |ui| {
                                let edit = egui::TextEdit::singleline(&mut self.input_dest)
                                    .font(egui::TextStyle::Monospace)
                                    .desired_width(ui.available_width() - 85.0);
                                ui.add(edit);
                            });

                        let browse_btn = egui::Button::new(
                            RichText::new("Browse")
                                .size(12.0)
                                .color(GLASS_SECONDARY)
                                .strong(),
                        )
                        .fill(GLASS_CARD)
                        .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                        .rounding(egui::Rounding::same(6.0));

                        if ui.add(browse_btn).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Choose download destination folder").clicked() {
                            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                self.input_dest = folder.to_string_lossy().to_string();
                            }
                        }
                    });

                    ui.add_space(12.0);

                    // Field 3: Parallel Connections (Modern Segmented Chips)
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Parallel Connections")
                                .size(12.5)
                                .color(GLASS_TEXT)
                                .strong(),
                        );
                        ui.label(
                            RichText::new("(Multi-stream segment threads)")
                                .size(11.0)
                                .color(GLASS_MUTED),
                        );
                    });

                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(8.0, 0.0);

                        let options = [
                            (4, "4 Streams", "Eco"),
                            (8, "8 Streams", "Default"),
                            (16, "16 Streams", "Turbo"),
                            (32, "32 Streams", "Extreme"),
                        ];

                        for (val, title, sub) in options {
                            let is_selected = self.input_segments == val;
                            let bg = if is_selected {
                                Color32::from_rgb(30, 22, 60)
                            } else {
                                GLASS_BG
                            };
                            let border = if is_selected {
                                GLASS_PRIMARY
                            } else {
                                GLASS_BORDER
                            };
                            let text_color = if is_selected {
                                GLASS_SECONDARY
                            } else {
                                GLASS_MUTED
                            };

                            let frame = egui::Frame::none()
                                .fill(bg)
                                .stroke(Stroke::new(if is_selected { 1.5_f32 } else { 1.0_f32 }, border))
                                .rounding(egui::Rounding::same(7.0))
                                .inner_margin(Margin::symmetric(10.0, 6.0));

                            let resp = frame.show(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label(RichText::new(title).size(11.5).color(text_color).strong());
                                    ui.label(RichText::new(sub).size(9.5).color(if is_selected { GLASS_PRIMARY } else { Color32::GRAY }));
                                });
                            }).response;

                            if resp.interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                self.input_segments = val;
                            }
                        }
                    });

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
                                    ui.label(RichText::new("⚠").color(STATUS_FAILED).strong());
                                    ui.label(RichText::new(err).color(Color32::from_rgb(255, 180, 190)).size(12.0));
                                });
                            });
                    }

                    ui.add_space(14.0);
                    ui.separator();
                    ui.add_space(10.0);

                    // Modal Footer Buttons
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("🔒 Dynamic Stream Planner")
                                .size(11.0)
                                .color(GLASS_MUTED),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let start_btn = egui::Button::new(
                                RichText::new("Start Download")
                                    .size(13.0)
                                    .color(Color32::WHITE)
                                    .strong(),
                            )
                            .fill(GLASS_PRIMARY)
                            .rounding(egui::Rounding::same(7.0));

                            if ui.add(start_btn).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text("Start multi-threaded accelerated download").clicked() {
                                start_download_req = true;
                            }

                            let cancel_btn = egui::Button::new(
                                RichText::new("Cancel")
                                    .size(13.0)
                                    .color(GLASS_MUTED),
                            )
                            .fill(GLASS_CARD)
                            .stroke(Stroke::new(1.0_f32, GLASS_BORDER))
                            .rounding(egui::Rounding::same(7.0));

                            if ui.add(cancel_btn).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                close_modal = true;
                            }
                        });
                    });
                });

            if close_modal {
                self.show_add_dialog = false;
                self.add_error = None;
            }

            if start_download_req {
                if self.input_url.trim().is_empty() {
                    self.add_error = Some("Please enter a valid download URL (http:// or https://)".to_string());
                } else {
                    let url = self.input_url.trim().to_string();
                    let dest = PathBuf::from(&self.input_dest);
                    let segs = self.input_segments;
                    self.start_new_download(url, dest, segs);
                }
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
        if let Some(idx) = action_remove {
            if let Ok(list) = self.tasks.try_lock() {
                if idx < list.len() {
                    self.tasks_to_delete.clear();
                    self.tasks_to_delete.insert(list[idx].id.clone());
                    self.show_delete_modal = true;
                }
            }
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
                            if do_permanent_delete {
                                let _ = std::fs::remove_file(&task.target_file);
                            }
                            self.selected_tasks.remove(&task.id);
                            list.remove(i);
                        }
                    }
                    if let Some(sel) = self.selected_task_index {
                        if sel >= list.len() {
                            self.selected_task_index = if list.is_empty() { None } else { Some(list.len() - 1) };
                        }
                    }
                }
                self.tasks_to_delete.clear();
            }
            if close_modal {
                self.show_delete_modal = false;
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
    rt: Arc<Runtime>,
    tasks_arc: Arc<Mutex<Vec<ActiveTaskUI>>>,
) {
    let task_id = format!("task-{}", Utc::now().timestamp_millis());

    rt.spawn(async move {
        let config = DownloadConfig {
            url: url.clone(),
            output_dir: dest_dir,
            custom_filename,
            num_segments: segments,
            cookies,
            user_agent,
            ..Default::default()
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
                    status: DownloadStatus::Downloading,
                    segments: initial_segments,
                    task_handle: Some(Arc::clone(&task_arc)),
                    num_segments: segments,
                    is_resuming: false,
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
                        }
                    }
                });

                let res = task_arc.run().await;
                if let Err(e) = res {
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
                eprintln!("Failed to create download: {}", e);
            }
        }
    });
}

// Background HTTP server for browser extension integration
async fn run_extension_server(
    tasks_arc: Arc<Mutex<Vec<ActiveTaskUI>>>,
    rt_arc: Arc<Runtime>,
    dest_dir: PathBuf,
) {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    let listener = match TcpListener::bind("127.0.0.1:9669").await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Local extension server bind error: {}", e);
            return;
        }
    };

    loop {
        let (mut socket, _) = match listener.accept().await {
            Ok(s) => s,
            Err(_) => continue,
        };

        let tasks_clone = Arc::clone(&tasks_arc);
        let rt_clone = Arc::clone(&rt_arc);
        let dest_clone = dest_dir.clone();

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

            // Handle CORS preflight
            if req_str.starts_with("OPTIONS") {
                let response = "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: POST, OPTIONS, GET\r\nAccess-Control-Allow-Headers: Content-Type\r\nContent-Length: 0\r\n\r\n";
                let _ = socket.write_all(response.as_bytes()).await;
                return;
            }

            // Handle health check
            if req_str.starts_with("GET") {
                let resp_body = "{\"status\":\"running\",\"app\":\"Rapid Download Manager\"}";
                let response = format!(
                    "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
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
                            let custom_fn = val.get("filename").and_then(|f| f.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string());
                            let cookies_str = val.get("cookies").and_then(|c| c.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string());
                            let user_agent_str = val.get("user_agent").and_then(|ua| ua.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string());
                            let url_str = url.to_string();

                            spawn_download_task(
                                url_str,
                                dest_clone,
                                8,
                                custom_fn,
                                cookies_str,
                                user_agent_str,
                                rt_clone,
                                tasks_clone,
                            );

                            let resp_body = "{\"status\":\"ok\"}";
                            let response = format!(
                                "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                                resp_body.len(),
                                resp_body
                            );
                            let _ = socket.write_all(response.as_bytes()).await;
                            return;
                        }
                    }
                }
            }

            let response = "HTTP/1.1 400 Bad Request\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: 0\r\n\r\n";
            let _ = socket.write_all(response.as_bytes()).await;
        });
    }
}

fn render_custom_chip_with_icon(
    ui: &mut egui::Ui,
    icon: ModernIcon,
    label: &str,
    value: &str,
    bg: Color32,
    border: Color32,
    text_color: Color32,
    tooltip: &str,
) {
    let frame = egui::Frame::none()
        .fill(bg)
        .stroke(Stroke::new(1.0_f32, border))
        .rounding(egui::Rounding::same(5.0))
        .inner_margin(Margin::symmetric(7.0, 3.5));

    frame.show(ui, |ui| {
        ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);
        let (icon_rect, _) = ui.allocate_exact_size(Vec2::new(10.0, 10.0), egui::Sense::hover());
        draw_modern_icon(ui.painter(), icon, icon_rect, text_color);
        if !label.is_empty() {
            ui.label(RichText::new(label).size(10.5).color(Color32::from_rgb(160, 140, 170)));
        }
        ui.label(RichText::new(value).size(11.0).color(text_color).strong());
    }).response.on_hover_text(tooltip);
}

fn render_custom_chip_plain(
    ui: &mut egui::Ui,
    label: &str,
    value: &str,
    bg: Color32,
    border: Color32,
    text_color: Color32,
    tooltip: &str,
) {
    let frame = egui::Frame::none()
        .fill(bg)
        .stroke(Stroke::new(1.0_f32, border))
        .rounding(egui::Rounding::same(5.0))
        .inner_margin(Margin::symmetric(7.0, 3.5));

    frame.show(ui, |ui| {
        ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);
        if !label.is_empty() {
            ui.label(RichText::new(label).size(10.5).color(GLASS_MUTED));
        }
        ui.label(RichText::new(value).size(11.0).color(text_color).strong());
    }).response.on_hover_text(tooltip);
}

fn render_custom_btn_chip_modern(
    ui: &mut egui::Ui,
    icon: ModernIcon,
    text: &str,
    tooltip: &str,
) -> bool {
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(98.0, 22.0), egui::Sense::click());
    let is_h = resp.hovered();

    let bg = if is_h { Color32::from_rgb(28, 20, 58) } else { GLASS_CARD };
    let border = if is_h { GLASS_PRIMARY } else { GLASS_BORDER };
    let text_col = if is_h { GLASS_PRIMARY_HOVER } else { GLASS_TEXT };

    ui.painter().rect(rect, 5.0, bg, Stroke::new(1.0_f32, border));
    let icon_rect = egui::Rect::from_center_size(egui::pos2(rect.min.x + 12.0, rect.center().y), Vec2::new(10.0, 10.0));
    draw_modern_icon(ui.painter(), icon, icon_rect, text_col);
    ui.painter().text(
        egui::pos2(rect.min.x + 22.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        text,
        egui::FontId::proportional(10.5),
        text_col,
    );

    resp
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text(tooltip)
        .clicked()
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
    if let Ok(user_profile) = std::env::var("USERPROFILE") {
        let downloads = PathBuf::from(user_profile).join("Downloads");
        if downloads.exists() {
            return downloads;
        }
    }
    PathBuf::from("./downloads")
}

#[cfg(windows)]
fn get_primary_screen_size() -> Option<(f32, f32)> {
    extern "system" {
        fn GetSystemMetrics(nIndex: i32) -> i32;
    }
    unsafe {
        let w = GetSystemMetrics(0); // SM_CXSCREEN
        let h = GetSystemMetrics(1); // SM_CYSCREEN
        if w > 0 && h > 0 {
            Some((w as f32, h as f32))
        } else {
            None
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let rt = Arc::new(Runtime::new().expect("Failed to initialize Tokio runtime"));

    let win_w = 1240.0_f32;
    let win_h = 740.0_f32;

    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([win_w, win_h])
        .with_min_inner_size([650.0, 380.0])
        .with_decorations(false)
        .with_resizable(true)
        .with_title("Rapid Download Manager")
        .with_visible(true)
        .with_active(true);

    #[cfg(windows)]
    if let Some((screen_w, screen_h)) = get_primary_screen_size() {
        let pos_x = ((screen_w - win_w) / 2.0).max(0.0);
        let pos_y = ((screen_h - win_h) / 2.0).max(0.0);
        viewport = viewport.with_position([pos_x, pos_y]);
    }

    let options = eframe::NativeOptions {
        viewport,
        centered: true,
        ..Default::default()
    };

    let rt_clone = Arc::clone(&rt);
    eframe::run_native(
        "Rapid Download Manager",
        options,
        Box::new(move |cc| {
            Ok(Box::new(RapidApp::new(cc, rt_clone)))
        }),
    )
}
