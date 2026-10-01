use eframe::egui::{self, Color32, Margin, RichText, Stroke, Vec2};
use crate::theme::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModernIcon {
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
    Clock,
}

pub fn draw_modern_icon(painter: &egui::Painter, icon: ModernIcon, rect: egui::Rect, color: Color32) {
    let center = rect.center();
    let dim = rect.width().min(rect.height());
    let s = (dim / 22.0).clamp(0.4, 4.0);
    let stroke = Stroke::new((1.5_f32 * s).clamp(1.0, 3.5), color);
    match icon {
        ModernIcon::Clock => {
            painter.circle_stroke(center, 5.5 * s, stroke);
            painter.line_segment([center, egui::pos2(center.x, center.y - 3.2 * s)], stroke);
            painter.line_segment([center, egui::pos2(center.x + 2.8 * s, center.y)], stroke);
            painter.circle_filled(center, 1.0 * s, color);
        }
        ModernIcon::Github => {
            painter.circle_stroke(center, 5.2 * s, stroke);
            // Left cat ear
            painter.line_segment([egui::pos2(center.x - 3.8 * s, center.y - 3.5 * s), egui::pos2(center.x - 2.5 * s, center.y - 6.2 * s)], stroke);
            painter.line_segment([egui::pos2(center.x - 2.5 * s, center.y - 6.2 * s), egui::pos2(center.x - 1.2 * s, center.y - 5.0 * s)], stroke);
            // Right cat ear
            painter.line_segment([egui::pos2(center.x + 1.2 * s, center.y - 5.0 * s), egui::pos2(center.x + 2.5 * s, center.y - 6.2 * s)], stroke);
            painter.line_segment([egui::pos2(center.x + 2.5 * s, center.y - 6.2 * s), egui::pos2(center.x + 3.8 * s, center.y - 3.5 * s)], stroke);
            // Branch/stem
            painter.line_segment([egui::pos2(center.x, center.y - 1.0 * s), egui::pos2(center.x, center.y + 4.2 * s)], Stroke::new(1.3_f32 * s, color));
            painter.circle_filled(egui::pos2(center.x - 2.0 * s, center.y + 1.5 * s), 1.0 * s, color);
            painter.circle_filled(egui::pos2(center.x + 2.0 * s, center.y + 1.5 * s), 1.0 * s, color);
        }
        ModernIcon::Globe => {
            painter.circle_stroke(center, 5.0 * s, stroke);
            painter.line_segment([egui::pos2(center.x - 5.0 * s, center.y), egui::pos2(center.x + 5.0 * s, center.y)], stroke);
            painter.line_segment([egui::pos2(center.x, center.y - 5.0 * s), egui::pos2(center.x, center.y + 5.0 * s)], stroke);
        }
        ModernIcon::Close => {
            let d = 5.0 * s;
            let c_stroke = Stroke::new((2.0_f32 * s).clamp(1.2, 4.0), color);
            painter.line_segment(
                [egui::pos2(center.x - d, center.y - d), egui::pos2(center.x + d, center.y + d)],
                c_stroke,
            );
            painter.line_segment(
                [egui::pos2(center.x + d, center.y - d), egui::pos2(center.x - d, center.y + d)],
                c_stroke,
            );
        }
        ModernIcon::Play => {
            let half_h = 5.0_f32 * s;
            let p1 = egui::pos2(center.x - 3.5 * s, center.y - half_h);
            let p2 = egui::pos2(center.x - 3.5 * s, center.y + half_h);
            let p3 = egui::pos2(center.x + 4.5 * s, center.y);
            painter.add(egui::Shape::convex_polygon(
                vec![p1, p2, p3],
                color,
                Stroke::NONE,
            ));
        }
        ModernIcon::Pause => {
            let bar_w = 2.5_f32 * s;
            let bar_h = 10.0_f32 * s;
            let gap = 2.5_f32 * s;
            let b1 = egui::Rect::from_center_size(egui::pos2(center.x - gap, center.y), egui::vec2(bar_w, bar_h));
            let b2 = egui::Rect::from_center_size(egui::pos2(center.x + gap, center.y), egui::vec2(bar_w, bar_h));
            painter.rect_filled(b1, (1.0 * s).max(0.5), color);
            painter.rect_filled(b2, (1.0 * s).max(0.5), color);
        }
        ModernIcon::Refresh => {
            let radius = 4.8_f32 * s;
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
            painter.line_segment([tip, egui::pos2(tip.x + 2.5 * s, tip.y - 0.5 * s)], stroke);
            painter.line_segment([tip, egui::pos2(tip.x - 0.5 * s, tip.y - 2.5 * s)], stroke);
        }
        ModernIcon::Trash => {
            let w = 9.0_f32 * s;
            let h = 10.0_f32 * s;
            painter.line_segment(
                [egui::pos2(center.x - w * 0.6, center.y - h * 0.4), egui::pos2(center.x + w * 0.6, center.y - h * 0.4)],
                Stroke::new(1.4_f32 * s, color),
            );
            painter.line_segment(
                [egui::pos2(center.x - 2.0 * s, center.y - h * 0.4), egui::pos2(center.x - 2.0 * s, center.y - h * 0.6)],
                Stroke::new(1.2_f32 * s, color),
            );
            painter.line_segment(
                [egui::pos2(center.x - 2.0 * s, center.y - h * 0.6), egui::pos2(center.x + 2.0 * s, center.y - h * 0.6)],
                Stroke::new(1.2_f32 * s, color),
            );
            painter.line_segment(
                [egui::pos2(center.x + 2.0 * s, center.y - h * 0.6), egui::pos2(center.x + 2.0 * s, center.y - h * 0.4)],
                Stroke::new(1.2_f32 * s, color),
            );
            let body_rect = egui::Rect::from_min_max(
                egui::pos2(center.x - w * 0.45, center.y - h * 0.25),
                egui::pos2(center.x + w * 0.45, center.y + h * 0.55),
            );
            painter.rect_stroke(body_rect, 1.5 * s, Stroke::new(1.2_f32 * s, color));
            painter.line_segment(
                [egui::pos2(center.x - 1.5 * s, center.y - h * 0.1), egui::pos2(center.x - 1.5 * s, center.y + h * 0.4)],
                Stroke::new(1.0_f32 * s, color),
            );
            painter.line_segment(
                [egui::pos2(center.x + 1.5 * s, center.y - h * 0.1), egui::pos2(center.x + 1.5 * s, center.y + h * 0.4)],
                Stroke::new(1.0_f32 * s, color),
            );
        }
        ModernIcon::Folder => {
            let w = 11.0_f32 * s;
            let h = 8.5_f32 * s;
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
            let w = 8.0_f32 * s;
            let h = 10.0_f32 * s;
            let doc_rect = egui::Rect::from_min_max(
                egui::pos2(center.x - w * 0.5, center.y - h * 0.5),
                egui::pos2(center.x + w * 0.5, center.y + h * 0.5),
            );
            painter.rect_stroke(doc_rect, 1.0 * s, stroke);
            painter.line_segment(
                [egui::pos2(center.x + w * 0.1, center.y - h * 0.5), egui::pos2(center.x + w * 0.5, center.y - h * 0.1)],
                stroke,
            );
        }
        ModernIcon::Plus => {
            let len = 5.5 * s;
            let stroke_w = (1.8 * s).clamp(1.2, 3.5);
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
            let r = 3.5_f32 * s;
            let c = egui::pos2(center.x - 1.5 * s, center.y - 1.5 * s);
            painter.circle_stroke(c, r, Stroke::new(1.4_f32 * s, color));
            painter.line_segment(
                [egui::pos2(c.x + 2.5 * s, c.y + 2.5 * s), egui::pos2(c.x + 6.0 * s, c.y + 6.0 * s)],
                Stroke::new(1.6_f32 * s, color),
            );
        }
        ModernIcon::MinimizeTray => {
            let d = 4.5 * s;
            painter.line_segment(
                [egui::pos2(center.x - d, center.y - 1.0 * s), egui::pos2(center.x, center.y + 3.0 * s)],
                Stroke::new(1.6_f32 * s, color),
            );
            painter.line_segment(
                [egui::pos2(center.x, center.y + 3.0 * s), egui::pos2(center.x + d, center.y - 1.0 * s)],
                Stroke::new(1.6_f32 * s, color),
            );
        }
        ModernIcon::Check => {
            painter.line_segment(
                [egui::pos2(center.x - 4.0 * s, center.y), egui::pos2(center.x - 1.0 * s, center.y + 3.5 * s)],
                Stroke::new(1.8_f32 * s, color),
            );
            painter.line_segment(
                [egui::pos2(center.x - 1.0 * s, center.y + 3.5 * s), egui::pos2(center.x + 4.5 * s, center.y - 3.5 * s)],
                Stroke::new(1.8_f32 * s, color),
            );
        }
        ModernIcon::PulseBeacon => {
            painter.circle_filled(center, 3.5 * s, color);
        }
        ModernIcon::SpeedGauge => {
            painter.line_segment(
                [egui::pos2(center.x + 1.0 * s, center.y - 5.0 * s), egui::pos2(center.x - 3.0 * s, center.y)],
                Stroke::new(1.6_f32 * s, color),
            );
            painter.line_segment(
                [egui::pos2(center.x - 3.0 * s, center.y), egui::pos2(center.x + 1.0 * s, center.y)],
                Stroke::new(1.6_f32 * s, color),
            );
            painter.line_segment(
                [egui::pos2(center.x + 1.0 * s, center.y), egui::pos2(center.x - 1.0 * s, center.y + 5.0 * s)],
                Stroke::new(1.6_f32 * s, color),
            );
        }
        ModernIcon::WinMinimize => {
            let stroke = Stroke::new((1.2_f32 * s).max(1.0), color);
            painter.line_segment(
                [egui::pos2(center.x - 5.0 * s, center.y + 0.5 * s), egui::pos2(center.x + 5.0 * s, center.y + 0.5 * s)],
                stroke,
            );
        }
        ModernIcon::WinMaximize => {
            let box_rect = egui::Rect::from_center_size(center, Vec2::new(10.0 * s, 10.0 * s));
            painter.rect_stroke(box_rect, 0.0, Stroke::new((1.2_f32 * s).max(1.0), color));
        }
        ModernIcon::WinRestore => {
            let stroke = Stroke::new((1.2_f32 * s).max(1.0), color);
            // Back window (top-right): upper and right borders
            painter.line_segment(
                [egui::pos2(center.x - 2.0 * s, center.y - 5.0 * s), egui::pos2(center.x + 5.0 * s, center.y - 5.0 * s)],
                stroke,
            );
            painter.line_segment(
                [egui::pos2(center.x + 5.0 * s, center.y - 5.0 * s), egui::pos2(center.x + 5.0 * s, center.y + 2.0 * s)],
                stroke,
            );
            // Back window small bottom and left snippets
            painter.line_segment(
                [egui::pos2(center.x + 2.0 * s, center.y + 2.0 * s), egui::pos2(center.x + 5.0 * s, center.y + 2.0 * s)],
                stroke,
            );
            painter.line_segment(
                [egui::pos2(center.x - 2.0 * s, center.y - 5.0 * s), egui::pos2(center.x - 2.0 * s, center.y - 2.0 * s)],
                stroke,
            );
            // Front window (bottom-left)
            let front_rect = egui::Rect::from_min_max(
                egui::pos2(center.x - 5.0 * s, center.y - 2.0 * s),
                egui::pos2(center.x + 2.0 * s, center.y + 5.0 * s),
            );
            painter.rect_filled(front_rect, 0.0, GLASS_BG);
            painter.rect_stroke(front_rect, 0.0, stroke);
        }
        ModernIcon::WinClose => {
            let d = 4.5_f32 * s;
            let stroke = Stroke::new((1.3_f32 * s).max(1.0), color);
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

pub fn window_caption_button(
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

pub fn modern_icon_button(
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

pub fn render_file_type_badge(ui: &mut egui::Ui, ext: &str) {
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
