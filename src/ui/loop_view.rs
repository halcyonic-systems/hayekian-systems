use egui::{Color32, LayerId, Order, Pos2, Rect, StrokeKind, Ui, Vec2};
use crate::core::system::SystemState;
use crate::domains::{DomainConfig, DomainPalette};

/// Render the anticipatory system loop using domain-specific labels.
/// Layout matches McQuade's figures: rectangular process boxes,
/// labeled intermediate arrows, Knowledge on the L→E edge.
pub fn ales_loop(ui: &mut Ui, state: &SystemState, config: &DomainConfig, light_mode: bool) {
    // Reserve space for metrics strip (~50px), chart (~140px), theory + citation (~60px)
    let max_loop_height = (ui.available_height() - 260.0).max(200.0);
    let desired_size = Vec2::new(
        ui.available_width(),
        max_loop_height.min(ui.available_width() * 0.65),
    );
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
    let painter = ui.painter_at(rect);
    let center = rect.center();

    // Background
    let bg = if light_mode {
        Color32::from_rgb(235, 228, 216)
    } else {
        Color32::from_rgb(48, 52, 62)
    };
    painter.rect_filled(rect, 4.0, bg);

    // System boundary box
    // Semantic colors for light/dark mode
    let text_primary = if light_mode { Color32::from_rgb(30, 30, 40) } else { Color32::WHITE };
    let text_secondary = if light_mode { Color32::from_rgb(55, 58, 72) } else { Color32::from_rgb(180, 190, 200) };
    let boundary_color = if light_mode { Color32::from_rgb(165, 155, 140) } else { Color32::from_rgb(80, 82, 100) };
    let env_label_color = if light_mode { Color32::from_rgb(20, 100, 150) } else { Color32::from_rgb(100, 180, 220) };

    let boundary = Rect::from_center_size(center, Vec2::new(rect.width() * 0.88, rect.height() * 0.85));
    painter.rect_stroke(boundary, 2.0, egui::Stroke::new(1.0, boundary_color), StrokeKind::Inside);

    // Process box dimensions
    let box_w = 120.0;
    let box_h = 52.0;
    let half_box = Vec2::new(box_w / 2.0, box_h / 2.0);

    // Box centers — E (top-right), S (top-left), A (bottom-left), L (bottom-right)
    let margin_x = rect.width() * 0.22;
    let margin_y = rect.height() * 0.22;
    let box_centers: [Pos2; 4] = [
        Pos2::new(center.x + margin_x, center.y - margin_y), // [0] E/Innovation (top-right)
        Pos2::new(center.x - margin_x, center.y - margin_y), // [1] S/Judgment (top-left)
        Pos2::new(center.x - margin_x, center.y + margin_y), // [2] A/Production (bottom-left)
        Pos2::new(center.x + margin_x, center.y + margin_y), // [3] L/Exchange (bottom-right)
    ];

    // Draw flows (arrows between boxes) — uses domain-specific labels
    for i in 0..4 {
        let next = (i + 1) % 4;
        let flow = state.flow_strengths[i];
        let alpha = (flow * 225.0 + 30.0).min(255.0) as u8;
        let thickness = 1.0 + flow * 2.5;
        let color = flow_color(flow, alpha, &config.palette);

        let from = box_centers[i];
        let to = box_centers[next];
        let (start, end) = edge_points(from, to, half_box);

        // Arrow line
        painter.line_segment([start, end], egui::Stroke::new(thickness, color));
        draw_arrowhead(&painter, start, end, color, 7.0);

        // Flow label at midpoint, offset perpendicular
        let mid = Pos2::new((start.x + end.x) * 0.5, (start.y + end.y) * 0.5);
        let dir = Vec2::new(end.x - start.x, end.y - start.y).normalized();
        let perp = Vec2::new(-dir.y, dir.x);
        let label_offset = perp * 12.0;
        let label_pos = Pos2::new(mid.x + label_offset.x, mid.y + label_offset.y);

        painter.text(
            label_pos,
            egui::Align2::CENTER_CENTER,
            config.flow_labels[i],
            egui::FontId::proportional(11.0),
            if light_mode {
                Color32::from_rgba_premultiplied(60, 50, 25, alpha)
            } else {
                Color32::from_rgba_premultiplied(200, 200, 160, alpha)
            },
        );

        // Flow tooltip
        let label_rect = Rect::from_center_size(label_pos, Vec2::new(40.0, 16.0));
        if let Some(pointer) = response.hover_pos() {
            if label_rect.contains(pointer) {
                egui::show_tooltip_at_pointer(ui.ctx(), LayerId::new(Order::Tooltip, ui.id()), ui.id().with(("flow", i)), |ui| {
                    ui.label(egui::RichText::new(config.flow_labels[i]).strong());
                    ui.label(config.flow_descriptions[i]);
                    ui.add_space(4.0);
                    ui.label(format!("Flow strength: {:.0}%", flow * 100.0));
                });
            }
        }
    }

    // Draw process boxes — labels from domain config
    for (i, &bc) in box_centers.iter().enumerate() {
        let activation = state.process_activation[i];
        let bg = node_color(activation, &config.palette, light_mode);
        let box_rect = Rect::from_center_size(bc, Vec2::new(box_w, box_h));

        painter.rect_filled(box_rect, 3.0, bg);
        painter.rect_stroke(box_rect, 3.0, egui::Stroke::new(1.5, config.palette.accent), StrokeKind::Inside);

        // Process name (domain-specific)
        painter.text(
            Pos2::new(bc.x, bc.y - 8.0),
            egui::Align2::CENTER_CENTER,
            config.process_labels[i],
            egui::FontId::new(14.0, egui::FontFamily::Proportional),
            text_primary,
        );

        // Functional notation (domain-specific)
        painter.text(
            Pos2::new(bc.x, bc.y + 12.0),
            egui::Align2::CENTER_CENTER,
            config.process_notation[i],
            egui::FontId::proportional(10.5),
            text_secondary,
        );

        // Process tooltip (domain-specific description)
        if let Some(pointer) = response.hover_pos() {
            if box_rect.contains(pointer) {
                egui::show_tooltip_at_pointer(ui.ctx(), LayerId::new(Order::Tooltip, ui.id()), ui.id().with(("process", i)), |ui| {
                    ui.label(egui::RichText::new(config.process_labels[i]).strong());
                    ui.label(config.process_descriptions[i]);
                    ui.add_space(4.0);
                    ui.label(config.process_notation[i]);
                    ui.label(format!("Activation: {:.0}%", activation * 100.0));
                });
            }
        }
    }

    // Knowledge box — on the L→E edge (right side)
    let k = state.knowledge_quality;
    let k_box_size = Vec2::new(100.0, 40.0);
    let k_center = Pos2::new(
        box_centers[0].x,
        (box_centers[0].y + box_centers[3].y) * 0.5,
    );
    let k_rect = Rect::from_center_size(k_center, k_box_size);
    let k_color = knowledge_color(k, light_mode);
    painter.rect_filled(k_rect, 3.0, k_color);
    painter.rect_stroke(k_rect, 3.0, egui::Stroke::new(2.0, config.palette.knowledge_accent), StrokeKind::Inside);

    // Knowledge label (domain-specific)
    painter.text(
        Pos2::new(k_center.x, k_center.y - 6.0),
        egui::Align2::CENTER_CENTER,
        config.knowledge_label,
        egui::FontId::new(12.0, egui::FontFamily::Proportional),
        text_primary,
    );
    painter.text(
        Pos2::new(k_center.x, k_center.y + 8.0),
        egui::Align2::CENTER_CENTER,
        format!("{:.0}%", k * 100.0),
        egui::FontId::proportional(11.0),
        config.palette.knowledge_accent,
    );

    // Arrow from Knowledge pointing inward (left)
    let k_arrow_start = Pos2::new(k_center.x - k_box_size.x * 0.5, k_center.y);
    let k_arrow_end = Pos2::new(k_center.x - k_box_size.x * 0.5 - 30.0, k_center.y);
    let k_arrow_color = config.palette.knowledge_accent;
    painter.line_segment([k_arrow_start, k_arrow_end], egui::Stroke::new(1.5, k_arrow_color));
    draw_arrowhead(&painter, k_arrow_start, k_arrow_end, k_arrow_color, 6.0);

    // Knowledge tooltip (domain-specific)
    if let Some(pointer) = response.hover_pos() {
        if k_rect.contains(pointer) {
            egui::show_tooltip_at_pointer(ui.ctx(), LayerId::new(Order::Tooltip, ui.id()), ui.id().with("knowledge"), |ui| {
                ui.label(egui::RichText::new(config.knowledge_label).strong());
                ui.label(config.knowledge_description);
                ui.add_space(4.0);
                ui.label(format!("Quality: {:.0}%", k * 100.0));
            });
        }
    }

    // === External feedback loop: Output → Environment → Input ===
    let coupling = state.params.environmental_coupling;
    let env_alpha = (coupling * 200.0 + 55.0).min(255.0) as u8;
    let env_color = if light_mode {
        Color32::from_rgba_premultiplied(30, 120, 170, env_alpha)
    } else {
        Color32::from_rgba_premultiplied(100, 180, 220, env_alpha)
    };
    let env_stroke = egui::Stroke::new(1.5 + coupling * 1.0, env_color);
    let env_margin = 18.0; // gap outside boundary

    // 1. Output arrow from Action (bottom-left) going LEFT out of boundary
    let output_start = Pos2::new(box_centers[2].x - half_box.x, box_centers[2].y);
    let output_exit = Pos2::new(boundary.left() - env_margin, box_centers[2].y);
    painter.line_segment([output_start, output_exit], env_stroke);
    draw_arrowhead(&painter, output_start, output_exit, env_color, 6.0);

    // "Output (O)" label
    painter.text(
        Pos2::new(output_exit.x - 4.0, output_exit.y - 10.0),
        egui::Align2::RIGHT_BOTTOM,
        "Output (O)",
        egui::FontId::proportional(10.0),
        env_label_color,
    );

    // 2. Path down the left side, across the bottom to midpoint between A and L
    let corner_bottom_left = Pos2::new(boundary.left() - env_margin, boundary.bottom() + env_margin);
    // Input enters between A (bottom-left) and L (bottom-right)
    let input_x = (box_centers[2].x + box_centers[3].x) * 0.5;
    let corner_bottom_mid = Pos2::new(input_x, boundary.bottom() + env_margin);

    painter.line_segment([output_exit, corner_bottom_left], env_stroke);
    painter.line_segment([corner_bottom_left, corner_bottom_mid], env_stroke);

    // 3. Input arrow pointing UP into the system between A and L
    let env_entry = Pos2::new(input_x, boundary.bottom());
    painter.line_segment([corner_bottom_mid, env_entry], env_stroke);
    draw_arrowhead(&painter, corner_bottom_mid, env_entry, env_color, 6.0);

    // "Env Input (I)" label below the entry point
    painter.text(
        Pos2::new(input_x + 14.0, boundary.bottom() + env_margin + 2.0),
        egui::Align2::LEFT_CENTER,
        "Env Input (I)",
        egui::FontId::proportional(11.0),
        env_label_color,
    );

    // Env tooltip
    let env_rect = Rect::from_center_size(Pos2::new(input_x, boundary.bottom() + env_margin), Vec2::new(90.0, 24.0));
    if let Some(pointer) = response.hover_pos() {
        if env_rect.contains(pointer) {
            egui::show_tooltip_at_pointer(ui.ctx(), LayerId::new(Order::Tooltip, ui.id()), ui.id().with("env"), |ui| {
                ui.label(egui::RichText::new("Environmental Input (I)").strong());
                ui.label(config.env_input_description);
            });
        }
    }
}

/// Compute start/end points on box edges for an arrow between two box centers.
fn edge_points(from: Pos2, to: Pos2, half_box: Vec2) -> (Pos2, Pos2) {
    let dx = to.x - from.x;
    let dy = to.y - from.y;

    let start = if dx.abs() > dy.abs() {
        let sx = if dx > 0.0 { half_box.x } else { -half_box.x };
        Pos2::new(from.x + sx, from.y + dy * (sx / dx))
    } else {
        let sy = if dy > 0.0 { half_box.y } else { -half_box.y };
        Pos2::new(from.x + dx * (sy / dy), from.y + sy)
    };

    let end = if dx.abs() > dy.abs() {
        let sx = if dx > 0.0 { -half_box.x } else { half_box.x };
        Pos2::new(to.x + sx, to.y - dy * (sx / dx))
    } else {
        let sy = if dy > 0.0 { -half_box.y } else { half_box.y };
        Pos2::new(to.x - dx * (sy / dy), to.y + sy)
    };

    (start, end)
}

fn draw_arrowhead(painter: &egui::Painter, from: Pos2, to: Pos2, color: Color32, size: f32) {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 0.1 { return; }
    let dir = Vec2::new(dx / len, dy / len);
    let perp = Vec2::new(-dir.y, dir.x);

    let tip = to;
    let left = Pos2::new(
        tip.x - dir.x * size + perp.x * size * 0.4,
        tip.y - dir.y * size + perp.y * size * 0.4,
    );
    let right = Pos2::new(
        tip.x - dir.x * size - perp.x * size * 0.4,
        tip.y - dir.y * size - perp.y * size * 0.4,
    );
    painter.add(egui::Shape::convex_polygon(
        vec![tip, left, right],
        color,
        egui::Stroke::NONE,
    ));
}

fn flow_color(strength: f32, alpha: u8, palette: &DomainPalette) -> Color32 {
    let base = if strength > 0.6 {
        palette.flow_healthy
    } else if strength > 0.3 {
        palette.flow_warning
    } else {
        palette.flow_danger
    };
    Color32::from_rgba_premultiplied(base.r(), base.g(), base.b(), alpha)
}

fn node_color(activation: f32, palette: &DomainPalette, light_mode: bool) -> Color32 {
    let a = activation.clamp(0.0, 1.0);
    if light_mode {
        // Light mode: white base tinted toward accent
        let accent = palette.accent;
        let mix = 0.15 + a * 0.2; // 15-35% accent tint
        Color32::from_rgb(
            (255.0 - mix * (255.0 - accent.r() as f32)) as u8,
            (255.0 - mix * (255.0 - accent.g() as f32)) as u8,
            (255.0 - mix * (255.0 - accent.b() as f32)) as u8,
        )
    } else {
        let dim = palette.accent_dim;
        let bright = palette.accent;
        Color32::from_rgb(
            (dim.r() as f32 + a * (bright.r() as f32 - dim.r() as f32) * 0.5) as u8,
            (dim.g() as f32 + a * (bright.g() as f32 - dim.g() as f32) * 0.5) as u8,
            (dim.b() as f32 + a * (bright.b() as f32 - dim.b() as f32) * 0.5) as u8,
        )
    }
}

fn knowledge_color(quality: f32, light_mode: bool) -> Color32 {
    let q = quality.clamp(0.0, 1.0);
    if light_mode {
        // Light warm tint that deepens with quality
        Color32::from_rgb(
            (240.0 - q * 30.0) as u8,
            (235.0 - q * 15.0) as u8,
            (215.0 - q * 25.0) as u8,
        )
    } else {
        Color32::from_rgb(
            (40.0 + q * 40.0) as u8,
            (40.0 + q * 60.0) as u8,
            (30.0 + q * 20.0) as u8,
        )
    }
}
