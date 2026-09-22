#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console in release mode on Windows

use chrono::Utc;
use eframe::egui::{self, Color32, Margin, RichText, Stroke, Vec2};
use rapid_core::{DownloadConfig, DownloadStatus, DownloadTask};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

#[cfg(windows)]
mod tray;

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
}

struct RapidApp {
    tokio_rt: Arc<Runtime>,
    tasks: Arc<Mutex<Vec<ActiveTaskUI>>>,
    selected_task_index: Option<usize>,

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

    #[cfg(windows)]
    tray_handle: Option<tray::TrayHandle>,
}

impl RapidApp {
    fn new(cc: &eframe::CreationContext<'_>, rt: Arc<Runtime>) -> Self {
        // Master Color Palette: Cyber-Obsidian Dark Theme
        let mut visuals = egui::Visuals::dark();
        visuals.window_rounding = egui::Rounding::same(8.0);
        visuals.panel_fill = Color32::from_rgb(11, 14, 20);      // Deep Obsidian
        visuals.faint_bg_color = Color32::from_rgb(21, 25, 34);  // Midnight Slate
        visuals.extreme_bg_color = Color32::from_rgb(7, 9, 13);  // Pure Obsidian
        visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(16, 20, 28);
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, Color32::from_rgb(32, 38, 52));
        cc.egui_ctx.set_visuals(visuals);

        let default_download_dir = dirs_or_fallback();
        let tasks_arc = Arc::new(Mutex::new(Vec::new()));

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
            splash_start: Instant::now(),
            splash_duration: Duration::from_millis(1800),
            show_add_dialog: false,
            input_url: String::new(),
            input_dest: default_download_dir.to_string_lossy().to_string(),
            input_segments: 8,
            add_error: None,
            is_probing: false,
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

        spawn_download_task(url, dest_dir, segments, None, rt, tasks_arc);

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
            .frame(egui::Frame::none().fill(Color32::from_rgb(11, 14, 20)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    let avail_h = ui.available_height();
                    ui.add_space((avail_h * 0.2).max(40.0));

                    // Animated Pulsating Lightning Symbol with concentric glow rings
                    let pulse_scale = (time * 4.0).sin() * 0.08 + 1.0;
                    let (rect, _response) = ui.allocate_exact_size(Vec2::new(120.0, 120.0), egui::Sense::hover());
                    let center = rect.center();

                    let painter = ui.painter();

                    // Expanding outer glow ripple
                    let ripple_phase = (time * 1.6) % 1.0;
                    let ripple_radius = 40.0 + ripple_phase * 35.0;
                    let ripple_alpha = ((1.0 - ripple_phase) * 140.0) as u8;
                    painter.circle_stroke(
                        center,
                        ripple_radius,
                        Stroke::new(2.0_f32, Color32::from_rgba_unmultiplied(0, 210, 255, ripple_alpha)),
                    );

                    // Inner pulsing halo
                    painter.circle_filled(
                        center,
                        38.0 * pulse_scale,
                        Color32::from_rgba_unmultiplied(0, 210, 255, 30),
                    );
                    painter.circle_stroke(
                        center,
                        40.0 * pulse_scale,
                        Stroke::new(2.5_f32, Color32::from_rgb(0, 210, 255)),
                    );

                    // Centered Lightning Bolt icon
                    painter.text(
                        center,
                        egui::Align2::CENTER_CENTER,
                        "⚡",
                        egui::FontId::proportional(44.0 * pulse_scale),
                        Color32::from_rgb(0, 220, 255),
                    );

                    ui.add_space(26.0);
                    ui.label(
                        RichText::new("RAPID DOWNLOAD MANAGER")
                            .size(24.0)
                            .strong()
                            .color(Color32::from_rgb(0, 215, 255)),
                    );

                    ui.add_space(4.0);
                    ui.label(
                        RichText::new("High-Speed Multi-Thread Engine • IDM Grade Architecture")
                            .size(13.0)
                            .color(Color32::from_rgb(138, 153, 173)),
                    );

                    ui.add_space(28.0);
                    let bar = egui::ProgressBar::new(progress)
                        .desired_width(320.0)
                        .fill(Color32::from_rgb(0, 210, 255));
                    ui.add(bar);

                    ui.add_space(10.0);
                    let status_text = if progress < 0.35 {
                        "Initializing multi-segment planner..."
                    } else if progress < 0.75 {
                        "Starting browser extension bridge on port 9669..."
                    } else {
                        "Engine ready. Launching dashboard..."
                    };
                    ui.label(
                        RichText::new(status_text)
                            .size(12.0)
                            .color(Color32::from_rgb(110, 125, 145)),
                    );
                });
            });
    }
}

impl eframe::App for RapidApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

        // Top Toolbar Panel
        egui::TopBottomPanel::top("top_panel")
            .frame(
                egui::Frame::none()
                    .fill(Color32::from_rgb(18, 22, 30))
                    .inner_margin(Margin::symmetric(14.0, 10.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(RichText::new("⚡ Rapid Download Manager").strong().color(Color32::from_rgb(0, 210, 255)));
                    ui.add_space(20.0);

                    if ui.button(RichText::new("➕ Add URL").strong()).clicked() {
                        self.show_add_dialog = true;
                    }

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

                    ui.add_space(6.0);

                    // Resume / Retry button
                    if ui.add_enabled(is_resumable, egui::Button::new("▶ Resume")).on_hover_text("Resume or retry selected download").clicked() {
                        if let Some(idx) = self.selected_task_index {
                            action_resume = Some(idx);
                        }
                    }

                    // Pause button
                    if ui.add_enabled(is_downloading, egui::Button::new("⏸ Pause")).on_hover_text("Pause active download").clicked() {
                        if let Some(idx) = self.selected_task_index {
                            action_pause = Some(idx);
                        }
                    }

                    // Open Folder button
                    if ui.add_enabled(has_selection, egui::Button::new("📁 Open Folder")).on_hover_text("Open containing folder").clicked() {
                        if let Some(idx) = self.selected_task_index {
                            action_open_folder = Some(idx);
                        }
                    }

                    // Delete button
                    if ui.add_enabled(has_selection, egui::Button::new("🗑 Delete")).on_hover_text("Remove download from list").clicked() {
                        if let Some(idx) = self.selected_task_index {
                            action_remove = Some(idx);
                        }
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(RichText::new("🗕 Tray").size(12.0))
                            .on_hover_text("Minimize to Windows System Tray (continues in background)")
                            .clicked()
                        {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
                        }

                        if ui.button(RichText::new("▶ All").size(12.0))
                            .on_hover_text("Resume all paused downloads")
                            .clicked()
                        {
                            do_resume_all = true;
                        }

                        if ui.button(RichText::new("⏸ All").size(12.0))
                            .on_hover_text("Pause all active downloads")
                            .clicked()
                        {
                            do_pause_all = true;
                        }
                    });
                });
            });

        // Custom Cyber-Obsidian Status Bar (Replacing default status bar)
        egui::TopBottomPanel::bottom("custom_status_bar")
            .frame(
                egui::Frame::none()
                    .fill(Color32::from_rgb(8, 10, 15))
                    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(26, 33, 46)))
                    .inner_margin(Margin::symmetric(10.0, 5.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(6.0, 0.0);

                    // 1. Live Engine Beacon Chip
                    render_custom_chip(
                        ui,
                        "●",
                        "Engine:",
                        "127.0.0.1:9669",
                        Color32::from_rgb(9, 26, 18),
                        Color32::from_rgb(0, 230, 118),
                        Color32::from_rgb(0, 230, 118),
                        "Rapid Engine Background HTTP Listener is Active on 127.0.0.1:9669",
                    );

                    // 2. Aggregate Live Speed Chip
                    if total_speed_bps > 0 {
                        render_custom_chip(
                            ui,
                            "⚡",
                            "Speed:",
                            &format_speed(total_speed_bps),
                            Color32::from_rgb(10, 34, 48),
                            Color32::from_rgb(0, 210, 255),
                            Color32::from_rgb(0, 210, 255),
                            "Live Aggregate Download Speed",
                        );
                    } else {
                        render_custom_chip(
                            ui,
                            "⚡",
                            "Speed:",
                            "0.0 KB/s",
                            Color32::from_rgb(14, 18, 25),
                            Color32::from_rgb(30, 38, 52),
                            Color32::from_rgb(100, 115, 135),
                            "Download Speed (Idle)",
                        );
                    }

                    // 3. Active Tasks Chip
                    render_custom_chip(
                        ui,
                        "📥",
                        "Active:",
                        &active_count.to_string(),
                        if active_count > 0 { Color32::from_rgb(14, 35, 56) } else { Color32::from_rgb(14, 18, 25) },
                        if active_count > 0 { Color32::from_rgb(33, 150, 243) } else { Color32::from_rgb(30, 38, 52) },
                        if active_count > 0 { Color32::from_rgb(66, 165, 245) } else { Color32::from_rgb(100, 115, 135) },
                        "Number of downloads currently transferring",
                    );

                    // 4. Paused Tasks Chip
                    render_custom_chip(
                        ui,
                        "⏸",
                        "Paused:",
                        &paused_count.to_string(),
                        if paused_count > 0 { Color32::from_rgb(38, 30, 10) } else { Color32::from_rgb(14, 18, 25) },
                        if paused_count > 0 { Color32::from_rgb(255, 179, 0) } else { Color32::from_rgb(30, 38, 52) },
                        if paused_count > 0 { Color32::from_rgb(255, 193, 7) } else { Color32::from_rgb(100, 115, 135) },
                        "Number of paused downloads",
                    );

                    // 5. Finished Tasks Chip
                    render_custom_chip(
                        ui,
                        "✔",
                        "Done:",
                        &finished_count.to_string(),
                        if finished_count > 0 { Color32::from_rgb(13, 34, 22) } else { Color32::from_rgb(14, 18, 25) },
                        if finished_count > 0 { Color32::from_rgb(0, 200, 83) } else { Color32::from_rgb(30, 38, 52) },
                        if finished_count > 0 { Color32::from_rgb(76, 175, 80) } else { Color32::from_rgb(100, 115, 135) },
                        "Completed downloads in this session",
                    );

                    // 6. Total Downloaded Chip
                    render_custom_chip(
                        ui,
                        "📊",
                        "Total:",
                        &format_bytes(total_downloaded),
                        Color32::from_rgb(16, 20, 28),
                        Color32::from_rgb(38, 46, 62),
                        Color32::from_rgb(180, 195, 215),
                        "Total data downloaded across all tasks",
                    );

                    // Right-aligned actions
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        render_custom_chip(
                            ui,
                            "🛡",
                            "",
                            "Tray Active",
                            Color32::from_rgb(14, 18, 26),
                            Color32::from_rgb(34, 42, 58),
                            Color32::from_rgb(140, 155, 180),
                            "Rapid Download Manager is active in Windows Notification Area (System Tray)",
                        );

                        if render_custom_btn_chip(
                            ui,
                            "📁",
                            "Downloads",
                            "Open Downloads directory in Windows File Explorer",
                        ) {
                            let _ = open::that(dirs_or_fallback());
                        }

                        if render_custom_btn_chip(
                            ui,
                            "🧩",
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
                        .fill(Color32::from_rgb(12, 16, 24))
                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(28, 36, 50)))
                        .inner_margin(Margin::same(10.0)),
                )
                .show(ctx, |ui| {
                    ui.label(RichText::new("🧵 Multi-Part Connection Segments").strong().color(Color32::from_rgb(0, 210, 255)).size(11.5));
                    ui.add_space(4.0);

                    if let Ok(tasks) = self.tasks.try_lock() {
                        if let Some(idx) = self.selected_task_index {
                            if idx < tasks.len() {
                                let task = &tasks[idx];

                                // Show error notification banner if failed
                                if let DownloadStatus::Failed(ref err) = task.status {
                                    egui::Frame::none()
                                        .fill(Color32::from_rgb(45, 20, 24))
                                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(231, 76, 60)))
                                        .inner_margin(Margin::symmetric(8.0, 6.0))
                                        .rounding(egui::Rounding::same(4.0))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(RichText::new(format!("⚠ Error: {}", err)).color(Color32::from_rgb(231, 76, 60)).strong());
                                                if ui.button(RichText::new("🔄 Retry Now").strong()).clicked() {
                                                    action_resume = Some(idx);
                                                }
                                            });
                                        });
                                    ui.add_space(6.0);
                                } else if task.status == DownloadStatus::Paused {
                                    egui::Frame::none()
                                        .fill(Color32::from_rgb(40, 35, 18))
                                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(241, 196, 15)))
                                        .inner_margin(Margin::symmetric(8.0, 6.0))
                                        .rounding(egui::Rounding::same(4.0))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(RichText::new("⏸ Download paused").color(Color32::from_rgb(241, 196, 15)).strong());
                                                if ui.button(RichText::new("▶ Resume Download").strong()).clicked() {
                                                    action_resume = Some(idx);
                                                }
                                            });
                                        });
                                    ui.add_space(6.0);
                                }

                                let avail_width = ui.available_width();
                                let cols = ((avail_width / 270.0).floor() as usize).clamp(1, 6);
                                let seg_bar_width = ((avail_width / cols as f32) - 130.0).clamp(60.0, 260.0);

                                egui::Grid::new("segment_grid")
                                    .spacing(Vec2::new(12.0, 6.0))
                                    .show(ui, |ui| {
                                        for (i, seg) in task.segments.iter().enumerate() {
                                            ui.label(RichText::new(format!("Part {}:", i + 1)).size(11.0));
                                            let ratio = seg.progress_ratio();
                                            let bar = egui::ProgressBar::new(ratio)
                                                .show_percentage()
                                                .desired_width(seg_bar_width)
                                                .fill(if seg.is_complete { Color32::from_rgb(46, 204, 113) } else { Color32::from_rgb(52, 152, 219) });
                                            ui.add(bar);
                                            ui.label(RichText::new(format!("{:.1} / {:.1} MB", seg.downloaded_bytes as f64 / 1_048_576.0, seg.total_bytes() as f64 / 1_048_576.0)).size(11.0));
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
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut selected = self.selected_task_index;

            if let Ok(tasks) = self.tasks.try_lock() {
                if tasks.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(60.0);
                        ui.label(RichText::new("No active downloads").size(18.0).color(Color32::GRAY));
                        ui.add_space(10.0);
                        if ui.button(RichText::new("➕ Add your first download").size(14.0)).clicked() {
                            self.show_add_dialog = true;
                        }
                    });
                } else {
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            egui_extras::TableBuilder::new(ui)
                                .striped(true)
                                .resizable(true)
                                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                .column(egui_extras::Column::remainder().at_least(180.0).resizable(true)) // Filename (expands dynamically)
                                .column(egui_extras::Column::initial(80.0).at_least(60.0))                 // Size
                                .column(egui_extras::Column::initial(95.0).at_least(75.0))                 // Status
                                .column(egui_extras::Column::remainder().at_least(140.0))                  // Progress (expands dynamically)
                                .column(egui_extras::Column::initial(85.0).at_least(65.0))                 // Speed
                                .column(egui_extras::Column::initial(65.0).at_least(45.0))                 // ETA
                                .column(egui_extras::Column::exact(135.0))                                 // Compact Icon Actions
                                .header(24.0, |mut header| {
                                    header.col(|ui| { ui.strong("Filename"); });
                                    header.col(|ui| { ui.strong("Size"); });
                                    header.col(|ui| { ui.strong("Status"); });
                                    header.col(|ui| { ui.strong("Progress"); });
                                header.col(|ui| { ui.strong("Speed"); });
                                header.col(|ui| { ui.strong("ETA"); });
                                header.col(|ui| { ui.strong("Actions"); });
                            })
                            .body(|body| {
                                body.rows(30.0, tasks.len(), |mut row| {
                                    let i = row.index();
                                    let item = &tasks[i];
                                    let is_sel = selected == Some(i);
                                    let is_downloading = item.status == DownloadStatus::Downloading;
                                    let is_resumable = matches!(item.status, DownloadStatus::Paused | DownloadStatus::Failed(_));
                                    let is_complete = item.status == DownloadStatus::Completed;

                                    row.set_selected(is_sel);

                                    // 1. Filename column (Click to open file after download, folder icon to open in folder)
                                    row.col(|ui| {
                                        ui.horizontal(|ui| {
                                            let icon = match item.filename.rsplit('.').next().unwrap_or("").to_lowercase().as_str() {
                                                "pdf" | "doc" | "docx" | "txt" => "📄",
                                                "mp4" | "mkv" | "avi" | "mov" => "🎬",
                                                "zip" | "rar" | "7z" | "tar" | "gz" => "📦",
                                                "mp3" | "wav" | "flac" => "🎵",
                                                "exe" | "msi" => "⚙",
                                                "jpg" | "png" | "gif" | "webp" => "🖼",
                                                _ => "💾",
                                            };

                                            if is_complete {
                                                let label_text = RichText::new(format!("{} {}", icon, &item.filename))
                                                    .color(Color32::from_rgb(0, 210, 255))
                                                    .underline();
                                                let resp = ui.add(egui::Label::new(label_text).sense(egui::Sense::click()))
                                                    .on_hover_text("✔ Download complete! Click to open file directly");
                                                if resp.clicked() {
                                                    action_open_file = Some(i);
                                                }
                                                resp.context_menu(|ui| {
                                                    if ui.button("▶ Open File").clicked() {
                                                        action_open_file = Some(i);
                                                        ui.close_menu();
                                                    }
                                                    if ui.button("📁 Open Containing Folder").clicked() {
                                                        action_open_folder = Some(i);
                                                        ui.close_menu();
                                                    }
                                                    if ui.button("📋 Copy URL").clicked() {
                                                        action_copy_url = Some(item.url.clone());
                                                        ui.close_menu();
                                                    }
                                                    ui.separator();
                                                    if ui.button("🔄 Redownload").clicked() {
                                                        action_redownload = Some(i);
                                                        ui.close_menu();
                                                    }
                                                    if ui.button(RichText::new("🗑 Delete").color(Color32::from_rgb(231, 76, 60))).clicked() {
                                                        action_remove = Some(i);
                                                        ui.close_menu();
                                                    }
                                                });
                                            } else {
                                                let resp = ui.selectable_label(is_sel, format!("{} {}", icon, &item.filename));
                                                if resp.clicked() {
                                                    selected = Some(i);
                                                }
                                                resp.context_menu(|ui| {
                                                    if is_downloading {
                                                        if ui.button("⏸ Pause").clicked() {
                                                            action_pause = Some(i);
                                                            ui.close_menu();
                                                        }
                                                    } else if is_resumable {
                                                        if ui.button("▶ Resume").clicked() {
                                                            action_resume = Some(i);
                                                            ui.close_menu();
                                                        }
                                                    }
                                                    if ui.button("📁 Open Containing Folder").clicked() {
                                                        action_open_folder = Some(i);
                                                        ui.close_menu();
                                                    }
                                                    if ui.button("📋 Copy URL").clicked() {
                                                        action_copy_url = Some(item.url.clone());
                                                        ui.close_menu();
                                                    }
                                                    ui.separator();
                                                    if ui.button("🔄 Redownload").clicked() {
                                                        action_redownload = Some(i);
                                                        ui.close_menu();
                                                    }
                                                    if ui.button(RichText::new("🗑 Delete").color(Color32::from_rgb(231, 76, 60))).clicked() {
                                                        action_remove = Some(i);
                                                        ui.close_menu();
                                                    }
                                                });
                                            }

                                            let folder_btn = ui.small_button("📁").on_hover_text("Open containing folder in Explorer");
                                            if folder_btn.clicked() {
                                                action_open_folder = Some(i);
                                            }
                                        });
                                    });

                                    // 2. Size column
                                    row.col(|ui| {
                                        if let Some(total) = item.total_bytes {
                                            ui.label(format_bytes(total));
                                        } else if item.downloaded_bytes > 0 {
                                            ui.label(format_bytes(item.downloaded_bytes));
                                        } else {
                                            ui.label("--");
                                        }
                                    });

                                    // 3. Status column with hover details
                                    row.col(|ui| {
                                        let (color, text, tooltip) = match &item.status {
                                            DownloadStatus::Downloading => (Color32::from_rgb(52, 152, 219), "Downloading".to_string(), None),
                                            DownloadStatus::Completed => (Color32::from_rgb(46, 204, 113), "Completed".to_string(), None),
                                            DownloadStatus::Paused => (Color32::from_rgb(241, 196, 15), "Paused".to_string(), Some("Download is paused. Click Resume to continue.".to_string())),
                                            DownloadStatus::Failed(err) => (Color32::from_rgb(231, 76, 60), "Failed ⚠".to_string(), Some(format!("Failure reason: {}\nClick Resume/Retry to retry.", err))),
                                            _ => (Color32::GRAY, "Queued".to_string(), None),
                                        };
                                        let label = ui.label(RichText::new(text).color(color).strong());
                                        if let Some(tip) = tooltip {
                                            label.on_hover_text(tip);
                                        }
                                    });

                                    // 4. Progress bar with status color
                                    row.col(|ui| {
                                        let ratio = (item.progress_percent / 100.0).clamp(0.0, 1.0);
                                        let bar_color = match item.status {
                                            DownloadStatus::Completed => Color32::from_rgb(46, 204, 113),
                                            DownloadStatus::Downloading => Color32::from_rgb(0, 210, 255),
                                            DownloadStatus::Paused => Color32::from_rgb(241, 196, 15),
                                            DownloadStatus::Failed(_) => Color32::from_rgb(231, 76, 60),
                                            _ => Color32::from_rgb(100, 110, 130),
                                        };
                                        let bar = egui::ProgressBar::new(ratio)
                                            .show_percentage()
                                            .fill(bar_color);
                                        ui.add(bar);
                                    });

                                    // 5. Speed column
                                    row.col(|ui| {
                                        if item.status == DownloadStatus::Downloading {
                                            ui.label(format_speed(item.speed_bps));
                                        } else {
                                            ui.label("--");
                                        }
                                    });

                                    // 6. ETA column
                                    row.col(|ui| {
                                        if item.status == DownloadStatus::Downloading {
                                            if let Some(eta) = item.eta_seconds {
                                                ui.label(format_eta(eta));
                                            } else {
                                                ui.label("--");
                                            }
                                        } else {
                                            ui.label("--");
                                        }
                                    });

                                    // 7. Compact Icon Actions column (only icons, hover shows name)
                                    row.col(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.spacing_mut().item_spacing = Vec2::new(5.0, 0.0);
                                            if is_downloading {
                                                if ui.button(RichText::new("⏸").size(13.0).color(Color32::from_rgb(241, 196, 15)))
                                                    .on_hover_text("Pause")
                                                    .clicked()
                                                {
                                                    action_pause = Some(i);
                                                }
                                            } else if is_resumable {
                                                if ui.button(RichText::new("▶").size(13.0).color(Color32::from_rgb(46, 204, 113)))
                                                    .on_hover_text("Resume")
                                                    .clicked()
                                                {
                                                    action_resume = Some(i);
                                                }
                                            }

                                            if ui.button(RichText::new("🔄").size(13.0).color(Color32::from_rgb(0, 210, 255)))
                                                .on_hover_text("Redownload")
                                                .clicked()
                                            {
                                                action_redownload = Some(i);
                                            }

                                            if ui.button(RichText::new("🗑").size(13.0).color(Color32::from_rgb(231, 76, 60)))
                                                .on_hover_text("Delete")
                                                .clicked()
                                            {
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
            painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(175));

            let modal_frame = egui::Frame::none()
                .fill(Color32::from_rgb(13, 17, 24))
                .stroke(Stroke::new(1.5_f32, Color32::from_rgb(0, 210, 255)))
                .rounding(egui::Rounding::same(12.0))
                .inner_margin(Margin::same(20.0))
                .shadow(egui::epaint::Shadow {
                    offset: [0.0, 8.0].into(),
                    blur: 24.0,
                    spread: 2.0,
                    color: Color32::from_black_alpha(200),
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
                        ui.label(
                            RichText::new("⚡")
                                .size(22.0)
                                .color(Color32::from_rgb(0, 210, 255))
                                .strong(),
                        );
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new("Add New Download")
                                    .size(17.0)
                                    .color(Color32::from_rgb(240, 245, 255))
                                    .strong(),
                            );
                            ui.label(
                                RichText::new("High-Speed Multi-Thread Engine • Instant Allocation")
                                    .size(11.0)
                                    .color(Color32::from_rgb(110, 125, 145)),
                            );
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let close_btn = ui.add(
                                egui::Button::new(RichText::new("✕").size(14.0).color(Color32::from_rgb(150, 165, 185)))
                                    .fill(Color32::from_rgb(20, 26, 36))
                                    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(38, 48, 66)))
                                    .rounding(egui::Rounding::same(6.0)),
                            );
                            if close_btn.on_hover_text("Close dialog").clicked() {
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
                            RichText::new("🔗 Download URL")
                                .size(12.5)
                                .color(Color32::from_rgb(200, 215, 235))
                                .strong(),
                        );
                        ui.label(
                            RichText::new("(HTTP / HTTPS direct link)")
                                .size(11.0)
                                .color(Color32::from_rgb(100, 115, 135)),
                        );
                    });

                    ui.add_space(4.0);
                    egui::Frame::none()
                        .fill(Color32::from_rgb(8, 10, 15))
                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(36, 46, 64)))
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
                        RichText::new("📁 Save Destination")
                            .size(12.5)
                            .color(Color32::from_rgb(200, 215, 235))
                            .strong(),
                    );

                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        egui::Frame::none()
                            .fill(Color32::from_rgb(8, 10, 15))
                            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(36, 46, 64)))
                            .rounding(egui::Rounding::same(6.0))
                            .inner_margin(Margin::symmetric(10.0, 7.0))
                            .show(ui, |ui| {
                                let edit = egui::TextEdit::singleline(&mut self.input_dest)
                                    .font(egui::TextStyle::Monospace)
                                    .desired_width(ui.available_width() - 85.0);
                                ui.add(edit);
                            });

                        let browse_btn = egui::Button::new(
                            RichText::new("📂 Browse")
                                .size(12.0)
                                .color(Color32::from_rgb(0, 210, 255))
                                .strong(),
                        )
                        .fill(Color32::from_rgb(18, 24, 34))
                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(0, 210, 255)))
                        .rounding(egui::Rounding::same(6.0));

                        if ui.add(browse_btn).on_hover_text("Choose download destination folder").clicked() {
                            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                self.input_dest = folder.to_string_lossy().to_string();
                            }
                        }
                    });

                    ui.add_space(12.0);

                    // Field 3: Parallel Connections (Modern Segmented Chips)
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("⚡ Parallel Connections")
                                .size(12.5)
                                .color(Color32::from_rgb(200, 215, 235))
                                .strong(),
                        );
                        ui.label(
                            RichText::new("(Multi-stream segment threads)")
                                .size(11.0)
                                .color(Color32::from_rgb(100, 115, 135)),
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
                                Color32::from_rgb(11, 38, 54)
                            } else {
                                Color32::from_rgb(16, 21, 30)
                            };
                            let border = if is_selected {
                                Color32::from_rgb(0, 210, 255)
                            } else {
                                Color32::from_rgb(32, 40, 56)
                            };
                            let text_color = if is_selected {
                                Color32::from_rgb(0, 230, 255)
                            } else {
                                Color32::from_rgb(160, 175, 195)
                            };

                            let frame = egui::Frame::none()
                                .fill(bg)
                                .stroke(Stroke::new(if is_selected { 1.5_f32 } else { 1.0_f32 }, border))
                                .rounding(egui::Rounding::same(7.0))
                                .inner_margin(Margin::symmetric(10.0, 6.0));

                            let resp = frame.show(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label(RichText::new(title).size(11.5).color(text_color).strong());
                                    ui.label(RichText::new(sub).size(9.5).color(if is_selected { Color32::from_rgb(0, 210, 255) } else { Color32::GRAY }));
                                });
                            }).response;

                            if resp.interact(egui::Sense::click()).clicked() {
                                self.input_segments = val;
                            }
                        }
                    });

                    // Error Message Banner (if any)
                    if let Some(ref err) = self.add_error {
                        ui.add_space(8.0);
                        egui::Frame::none()
                            .fill(Color32::from_rgb(45, 18, 22))
                            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(231, 76, 60)))
                            .rounding(egui::Rounding::same(6.0))
                            .inner_margin(Margin::symmetric(10.0, 6.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("⚠").color(Color32::from_rgb(231, 76, 60)).strong());
                                    ui.label(RichText::new(err).color(Color32::from_rgb(255, 180, 185)).size(12.0));
                                });
                            });
                    }

                    ui.add_space(14.0);
                    ui.separator();
                    ui.add_space(10.0);

                    // Modal Footer Buttons
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("🔒 Dynamic Segment Planner")
                                .size(11.0)
                                .color(Color32::from_rgb(90, 105, 125)),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let start_btn = egui::Button::new(
                                RichText::new("🚀 Start Download")
                                    .size(13.0)
                                    .color(Color32::from_rgb(8, 12, 18))
                                    .strong(),
                            )
                            .fill(Color32::from_rgb(0, 210, 255))
                            .rounding(egui::Rounding::same(7.0));

                            if ui.add(start_btn).on_hover_text("Start multi-threaded accelerated download").clicked() {
                                start_download_req = true;
                            }

                            let cancel_btn = egui::Button::new(
                                RichText::new("Cancel")
                                    .size(13.0)
                                    .color(Color32::from_rgb(180, 195, 215)),
                            )
                            .fill(Color32::from_rgb(22, 28, 38))
                            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(42, 52, 70)))
                            .rounding(egui::Rounding::same(7.0));

                            if ui.add(cancel_btn).clicked() {
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
            self.remove_download(idx);
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
    }
}

fn spawn_download_task(
    url: String,
    dest_dir: PathBuf,
    segments: usize,
    custom_filename: Option<String>,
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
                            let custom_fn = val.get("filename").and_then(|f| f.as_str()).map(|s| s.to_string());
                            let url_str = url.to_string();

                            spawn_download_task(
                                url_str,
                                dest_clone,
                                8,
                                custom_fn,
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

fn render_custom_chip(
    ui: &mut egui::Ui,
    icon: &str,
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
        if !icon.is_empty() {
            ui.label(RichText::new(icon).size(11.0).color(text_color).strong());
        }
        if !label.is_empty() {
            ui.label(RichText::new(label).size(10.5).color(Color32::from_rgb(140, 152, 172)));
        }
        ui.label(RichText::new(value).size(11.0).color(text_color).strong());
    }).response.on_hover_text(tooltip);
}

fn render_custom_btn_chip(
    ui: &mut egui::Ui,
    icon: &str,
    text: &str,
    tooltip: &str,
) -> bool {
    let btn = egui::Button::new(
        RichText::new(format!("{} {}", icon, text))
            .size(11.0)
            .color(Color32::from_rgb(220, 235, 255))
            .strong(),
    )
    .fill(Color32::from_rgb(18, 24, 34))
    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(38, 48, 66)))
    .rounding(egui::Rounding::same(5.0));

    ui.add(btn).on_hover_text(tooltip).clicked()
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

fn main() -> Result<(), eframe::Error> {
    let rt = Arc::new(Runtime::new().expect("Failed to initialize Tokio runtime"));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 750.0])
            .with_min_inner_size([650.0, 380.0])
            .with_maximized(true)
            .with_title("Rapid Download Manager")
            .with_visible(true)
            .with_active(true),
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
