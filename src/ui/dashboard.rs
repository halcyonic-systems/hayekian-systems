use egui::{Color32, Ui, Vec2};
use crate::core::system::SystemState;
use crate::domains::DomainConfig;

/// Render the system health dashboard — emergent metrics, not controls.
pub fn system_dashboard(ui: &mut Ui, state: &SystemState, _config: &DomainConfig, light_mode: bool) {
    ui.heading("System Health");
    ui.add_space(4.0);

    let vitality = state.params.system_vitality();
    let anticipation = state.params.anticipation_accuracy();
    let reality = state.params.reality_orientation();
    let k_rate = state.params.knowledge_rate();

    metric_bar(ui, "Knowledge Quality", state.knowledge_quality, quality_color(state.knowledge_quality), light_mode);
    metric_bar(ui, "System Vitality", vitality, quality_color(vitality), light_mode);
    metric_bar(ui, "Anticipation Accuracy", anticipation, quality_color(anticipation), light_mode);
    metric_bar(ui, "Reality Orientation", reality, quality_color(reality), light_mode);

    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label("Knowledge Rate:");
        let color = if k_rate > 0.01 {
            Color32::from_rgb(80, 200, 80)
        } else if k_rate < -0.01 {
            Color32::from_rgb(200, 80, 80)
        } else {
            Color32::from_rgb(180, 180, 80)
        };
        ui.colored_label(color, format!("{:+.3}/t", k_rate));
    });

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(4.0);

    // Knowledge history sparkline.
    ui.label("Knowledge Over Time");
    let history = &state.knowledge_history;
    if history.len() > 1 {
        let desired_size = Vec2::new(ui.available_width(), 60.0);
        let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
        let painter = ui.painter_at(rect);

        // Background
        let spark_bg = if light_mode {
            Color32::from_rgb(225, 218, 205)
        } else {
            Color32::from_rgb(48, 52, 62)
        };
        painter.rect_filled(rect, 2.0, spark_bg);

        // Plot line
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

        for window in points.windows(2) {
            let color = quality_color(history[history.len() - 1]);
            painter.line_segment([window[0], window[1]], egui::Stroke::new(1.5, color));
        }
    }
}

fn metric_bar(ui: &mut Ui, label: &str, value: f32, color: Color32, light_mode: bool) {
    ui.horizontal(|ui| {
        ui.label(format!("{label}:"));
        let desired = Vec2::new(120.0, 14.0);
        let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
        let painter = ui.painter_at(rect);
        let bar_bg = if light_mode {
            Color32::from_rgb(218, 212, 200)
        } else {
            Color32::from_rgb(52, 56, 66)
        };
        painter.rect_filled(rect, 2.0, bar_bg);
        let fill_rect = egui::Rect::from_min_size(
            rect.min,
            Vec2::new(rect.width() * value.clamp(0.0, 1.0), rect.height()),
        );
        painter.rect_filled(fill_rect, 2.0, color);
        ui.label(format!("{:.0}%", value * 100.0));
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
