use egui::{Color32, Ui, Vec2};
use crate::core::system::SystemState;
use crate::domains::DomainConfig;

/// Compact horizontal metrics strip — sits between loop and chart.
pub fn metrics_strip(ui: &mut Ui, state: &SystemState, config: &DomainConfig, light_mode: bool) {
    let k_rate = state.params.knowledge_rate();
    let closure = state.flow_strengths.iter().cloned().fold(f32::INFINITY, f32::min);

    ui.horizontal(|ui| {
        let available = ui.available_width();
        let bar_width = (available * 0.38).min(280.0);

        // Knowledge bar
        metric_bar(ui, config.knowledge_label, state.knowledge_quality, quality_color(state.knowledge_quality), light_mode, bar_width);

        ui.add_space(12.0);

        // Loop Closure bar
        metric_bar(ui, "Loop Closure", closure, closure_color(closure), light_mode, bar_width);

        ui.add_space(12.0);

        // Trend indicator
        let trend = if k_rate > 0.01 { "▲ Rising" } else if k_rate < -0.01 { "▼ Falling" } else { "— Stable" };
        let color = if k_rate > 0.01 {
            Color32::from_rgb(60, 180, 60)
        } else if k_rate < -0.01 {
            Color32::from_rgb(200, 70, 70)
        } else {
            if light_mode { Color32::from_rgb(140, 135, 100) } else { Color32::from_rgb(180, 180, 80) }
        };
        ui.vertical(|ui| {
            let dim = if light_mode { Color32::from_rgb(60, 60, 75) } else { Color32::from_rgb(160, 160, 175) };
            ui.label(egui::RichText::new("Trend").color(dim).small());
            ui.colored_label(color, egui::RichText::new(trend).strong());
        });
    });
}

/// Full-width knowledge chart — uses all available space.
pub fn knowledge_chart(ui: &mut Ui, state: &SystemState, light_mode: bool) {
    let history = &state.knowledge_history;
    if history.len() < 2 {
        return;
    }

    let desired_size = Vec2::new(
        ui.available_width(),
        ui.available_height().min(160.0).max(60.0),
    );
    let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
    let painter = ui.painter_at(rect);

    // Background
    let bg = if light_mode {
        Color32::from_rgb(230, 224, 212)
    } else {
        Color32::from_rgb(46, 50, 60)
    };
    painter.rect_filled(rect, 4.0, bg);

    // Subtle grid lines at 25%, 50%, 75%
    let grid_color = if light_mode {
        Color32::from_rgba_premultiplied(160, 155, 140, 50)
    } else {
        Color32::from_rgba_premultiplied(80, 80, 100, 50)
    };
    let label_color = if light_mode {
        Color32::from_rgb(100, 95, 80)
    } else {
        Color32::from_rgba_premultiplied(100, 100, 120, 180)
    };
    for pct in [0.25_f32, 0.5, 0.75] {
        let y = rect.bottom() - pct * rect.height();
        painter.line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            egui::Stroke::new(0.5, grid_color),
        );
        // Grid label
        painter.text(
            egui::pos2(rect.right() - 4.0, y - 2.0),
            egui::Align2::RIGHT_BOTTOM,
            format!("{:.0}%", pct * 100.0),
            egui::FontId::proportional(9.0),
            label_color,
        );
    }

    // Knowledge line
    let n = history.len();
    let points: Vec<egui::Pos2> = history
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            let x = rect.left() + (i as f32 / (n - 1) as f32) * rect.width();
            let y = rect.bottom() - v.clamp(0.0, 1.0) * rect.height();
            egui::pos2(x, y)
        })
        .collect();

    let k_color = quality_color(history[history.len() - 1]);
    for window in points.windows(2) {
        painter.line_segment([window[0], window[1]], egui::Stroke::new(2.0, k_color));
    }

    // "Knowledge" legend
    painter.text(
        egui::pos2(rect.left() + 8.0, rect.top() + 10.0),
        egui::Align2::LEFT_CENTER,
        "Knowledge",
        egui::FontId::proportional(11.0),
        k_color,
    );

    // X-axis label
    painter.text(
        egui::pos2(rect.center().x, rect.bottom() - 4.0),
        egui::Align2::CENTER_BOTTOM,
        "Time",
        egui::FontId::proportional(10.0),
        label_color,
    );

    // Y-axis label
    painter.text(
        egui::pos2(rect.left() + 4.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        "Quality",
        egui::FontId::proportional(10.0),
        label_color,
    );
}

fn metric_bar(ui: &mut Ui, label: &str, value: f32, color: Color32, light_mode: bool, width: f32) {
    ui.vertical(|ui| {
        let dim = if light_mode { Color32::from_rgb(60, 60, 75) } else { Color32::from_rgb(160, 160, 175) };
        ui.label(egui::RichText::new(label).color(dim).small());

        let desired = Vec2::new(width, 18.0);
        let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
        let painter = ui.painter_at(rect);
        let bar_bg = if light_mode {
            Color32::from_rgb(218, 212, 200)
        } else {
            Color32::from_rgb(52, 56, 66)
        };
        painter.rect_filled(rect, 3.0, bar_bg);
        let fill_rect = egui::Rect::from_min_size(
            rect.min,
            Vec2::new(rect.width() * value.clamp(0.0, 1.0), rect.height()),
        );
        painter.rect_filled(fill_rect, 3.0, color);

        // Percentage on bar
        let text_color = if value > 0.4 {
            Color32::WHITE
        } else if light_mode {
            Color32::from_rgb(60, 60, 70)
        } else {
            Color32::from_rgb(200, 200, 210)
        };
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            format!("{:.0}%", value * 100.0),
            egui::FontId::proportional(10.0),
            text_color,
        );
    });
}

fn quality_color(value: f32) -> Color32 {
    let v = value.clamp(0.0, 1.0);
    if v > 0.6 {
        let t = (v - 0.6) / 0.4;
        Color32::from_rgb(
            (120.0 - t * 40.0) as u8,
            (160.0 + t * 60.0) as u8,
            (80.0 + t * 40.0) as u8,
        )
    } else if v > 0.3 {
        Color32::from_rgb(200, 180, 60)
    } else {
        let t = v / 0.3;
        Color32::from_rgb(200, (60.0 + t * 100.0) as u8, (40.0 + t * 20.0) as u8)
    }
}

fn closure_color(value: f32) -> Color32 {
    let v = value.clamp(0.0, 1.0);
    if v > 0.6 {
        Color32::from_rgb(100, 160, 220)
    } else if v > 0.3 {
        Color32::from_rgb(200, 160, 80)
    } else {
        Color32::from_rgb(200, 80, 80)
    }
}
