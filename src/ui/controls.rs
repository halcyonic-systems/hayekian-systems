use egui::Ui;
use crate::core::params::StructuralParams;
use crate::domains::DomainConfig;

/// Render the four structural parameter sliders, with domain-specific labels.
pub fn parameter_controls(
    ui: &mut Ui,
    params: &mut StructuralParams,
    running: &mut bool,
    config: &DomainConfig,
) {
    ui.heading("Structural Parameters");
    ui.add_space(4.0);

    // Environmental coupling / Anchor — label changes per domain
    slider_with_explanation(
        ui,
        config.coupling_label,
        &mut params.environmental_coupling,
        config.coupling_tooltip,
    );

    slider_with_explanation(
        ui,
        "Innovation Freedom",
        &mut params.innovation_freedom,
        "How unconstrained the expectation/proposal process is. \"Development of possible future scenarios based on existing knowledge.\" E(K,I) \u{2192} D \u{2014} Ch 5, Fig 5.2",
    );

    slider_with_explanation(
        ui,
        "Feedback Fidelity",
        &mut params.feedback_fidelity,
        "How accurately consequences flow back to update knowledge. \"Updating of the internal model ... based on sensory input and the results of action.\" L(C,O,K,I) \u{2192} K \u{2014} Ch 5, Fig 5.2",
    );

    slider_with_explanation(
        ui,
        "Process Closure",
        &mut params.process_closure,
        "Whether each process's conditions are provided by companion processes. Piaget: \"a closed cycle ... characteristic of the organism\" where processes \"reconstitute each other and thus maintain the operation of the system.\" \u{2014} Ch 5",
    );

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(4.0);

    ui.horizontal(|ui| {
        if ui.button(if *running { "⏸ Pause" } else { "▶ Run" }).clicked() {
            *running = !*running;
        }
        if ui.button("↺ Reset").clicked() {
            *params = StructuralParams::default();
        }
    });
}

fn slider_with_explanation(ui: &mut Ui, label: &str, value: &mut f32, tooltip: &str) {
    ui.add_space(2.0);
    ui.label(label).on_hover_text(tooltip);
    ui.add(egui::Slider::new(value, 0.0..=1.0).show_value(true));
}
