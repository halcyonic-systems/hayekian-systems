mod core;
mod domains;
mod ui;

use eframe::egui;
use crate::core::system::SystemState;
use crate::domains::DomainConfig;
use crate::ui::{controls, dashboard, loop_view};

// Native entry point
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 780.0])
            .with_title("Hayekian Anticipatory Systems Explorer"),
        ..Default::default()
    };

    eframe::run_native(
        "Hayekian Systems",
        options,
        Box::new(|_cc| Ok(Box::new(HayekianApp::default()))),
    )
}

// Web entry point
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");
        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("No canvas element")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("Not a canvas");
        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|_cc| Ok(Box::new(HayekianApp::default()))),
            )
            .await
            .expect("failed to start eframe");
    });
}

/// Available domain configurations.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Domain {
    Abstract,
    Market,
    Firm,
    FreeBanking,
    Science,
    Legislature,
    Bureaucracy,
}

impl Domain {
    const ALL: [Domain; 7] = [
        Domain::Abstract,
        Domain::Market,
        Domain::Firm,
        Domain::FreeBanking,
        Domain::Science,
        Domain::Legislature,
        Domain::Bureaucracy,
    ];

    fn label(&self) -> &'static str {
        match self {
            Domain::Abstract => "Abstract (Ch 5)",
            Domain::Market => "Market (Ch 6, Fig 6.1)",
            Domain::Firm => "Firm (Ch 6, Fig 6.2)",
            Domain::FreeBanking => "Free Banking (Ch 6, Fig 6.3)",
            Domain::Science => "Science (Ch 7, Fig 7.1)",
            Domain::Legislature => "Legislature (Ch 8, Fig 8.1)",
            Domain::Bureaucracy => "Bureaucracy (Ch 8, Fig 8.2)",
        }
    }

    fn config(&self) -> DomainConfig {
        match self {
            Domain::Abstract => domains::abstract_system(),
            Domain::Market => domains::market::market_system(),
            Domain::Firm => domains::market::firm_system(),
            Domain::FreeBanking => domains::market::free_banking_system(),
            Domain::Science => domains::science::science_system(),
            Domain::Legislature => domains::government::legislature_system(),
            Domain::Bureaucracy => domains::government::bureaucracy_system(),
        }
    }
}

struct HayekianApp {
    state: SystemState,
    running: bool,
    theory_panel_open: bool,
    domain: Domain,
    domain_config: DomainConfig,
    light_mode: bool,
}

impl Default for HayekianApp {
    fn default() -> Self {
        let domain = Domain::Abstract;
        Self {
            state: SystemState::default(),
            running: false,
            theory_panel_open: false,
            domain,
            domain_config: domain.config(),
            light_mode: true,
        }
    }
}

impl eframe::App for HayekianApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Step simulation if running.
        if self.running {
            self.state.step(0.1);
            ctx.request_repaint(); // continuous animation
        }

        // Theme
        if self.light_mode {
            let mut visuals = egui::Visuals::light();
            visuals.panel_fill = egui::Color32::from_rgb(245, 240, 230);  // warm cream
            visuals.window_fill = egui::Color32::from_rgb(245, 240, 230);
            visuals.extreme_bg_color = egui::Color32::from_rgb(235, 228, 216);
            visuals.faint_bg_color = egui::Color32::from_rgb(250, 245, 236);
            ctx.set_visuals(visuals);
        } else {
            let mut visuals = egui::Visuals::dark();
            visuals.panel_fill = egui::Color32::from_rgb(58, 62, 74);
            visuals.window_fill = egui::Color32::from_rgb(58, 62, 74);
            visuals.extreme_bg_color = egui::Color32::from_rgb(48, 52, 62);
            visuals.faint_bg_color = egui::Color32::from_rgb(66, 70, 82);
            ctx.set_visuals(visuals);
        }

        // Left panel: controls
        egui::SidePanel::left("controls_panel")
            .min_width(260.0)
            .max_width(300.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);

                // Theme toggle
                ui.horizontal(|ui| {
                    let label = if self.light_mode { "☀ Light" } else { "☽ Dark" };
                    if ui.button(label).clicked() {
                        self.light_mode = !self.light_mode;
                    }
                });
                ui.add_space(4.0);
                ui.separator();
                ui.add_space(4.0);

                // Domain selector
                ui.heading("Domain");
                ui.add_space(4.0);
                let prev_domain = self.domain;
                for d in Domain::ALL {
                    if ui.radio_value(&mut self.domain, d, d.label()).changed() {
                        // Relabel but keep parameters and state — the invariance payoff
                        self.domain_config = self.domain.config();
                    }
                }
                if self.domain != prev_domain {
                    self.domain_config = self.domain.config();
                }

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(4.0);

                // Chapter/figure info
                ui.label(egui::RichText::new(self.domain_config.name).strong());
                ui.label(
                    egui::RichText::new(format!(
                        "{} \u{2014} {}",
                        self.domain_config.chapter, self.domain_config.figure
                    ))
                    .small()
                    .color(if self.light_mode { egui::Color32::from_rgb(60, 60, 78) } else { egui::Color32::from_rgb(160, 160, 180) }),
                );

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                controls::parameter_controls(
                    ui,
                    &mut self.state.params,
                    &mut self.running,
                    &self.domain_config,
                );

            });

        // Central area: loop + metrics + chart
        egui::CentralPanel::default().show(ctx, |ui| {
            // Citation pinned at very bottom (rendered first in bottom-up pass)
            let cite_color = if self.light_mode {
                egui::Color32::from_rgb(90, 85, 75)
            } else {
                egui::Color32::from_rgb(120, 120, 140)
            };

            // Use a top-down layout for the main content
            ui.add_space(4.0);

            // Loop diagram — takes proportional space
            loop_view::ales_loop(ui, &self.state, &self.domain_config, self.light_mode);

            ui.add_space(6.0);
            ui.separator();
            ui.add_space(4.0);

            // Horizontal metrics strip
            dashboard::metrics_strip(ui, &self.state, &self.domain_config, self.light_mode);

            ui.add_space(6.0);

            // Chart — fills remaining space minus citation and theory
            dashboard::knowledge_chart(ui, &self.state, self.light_mode);

            ui.add_space(4.0);

            // Collapsible theory panel
            let theory_title = format!(
                "Theory \u{2014} McQuade {}: {}",
                self.domain_config.chapter, self.domain_config.name
            );
            egui::CollapsingHeader::new(
                egui::RichText::new(theory_title)
                    .color(if self.light_mode { egui::Color32::from_rgb(60, 60, 78) } else { egui::Color32::from_rgb(160, 160, 180) }),
            )
            .default_open(self.theory_panel_open)
            .show(ui, |ui| {
                self.theory_panel_open = true;
                theory_panel(ui, &self.domain_config, self.light_mode);
            });

            // Source citation
            ui.add_space(4.0);
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("Based on McQuade & Butos, Anticipatory Systems in a Hayekian Framework (Routledge)")
                        .italics()
                        .color(cite_color),
                );
            });
        });
    }
}

/// Theory panel content, adapting to the current domain.
fn theory_panel(ui: &mut egui::Ui, config: &DomainConfig, light_mode: bool) {
    let dim = if light_mode { egui::Color32::from_rgb(60, 60, 78) } else { egui::Color32::from_rgb(160, 160, 180) };

    ui.label(
        egui::RichText::new(format!(
            "{}: Process organization in {}",
            config.figure,
            config.name.to_lowercase()
        ))
        .italics()
        .color(dim),
    );
    ui.add_space(4.0);

    // Show process descriptions from this domain
    for i in 0..4 {
        ui.label(egui::RichText::new(config.process_labels[i]).strong());
        ui.label(egui::RichText::new(config.process_descriptions[i]).color(dim));
        ui.add_space(2.0);
    }
    ui.add_space(4.0);

    ui.label(egui::RichText::new(config.knowledge_label).strong());
    ui.label(egui::RichText::new(config.knowledge_description).color(dim));
    ui.add_space(4.0);

    // Cross-domain insight (only show when not abstract)
    if config.figure != "Figure 5.2" {
        ui.separator();
        ui.add_space(4.0);
        ui.label(egui::RichText::new("Cross-Domain Invariance").strong());
        ui.label(egui::RichText::new(
            "\"At the most general level, what is crucial for adaptation in social systems \
             of all sorts is not the specific ability to form prices but the ability to \
             generate feedback effects which constrain self-interest while at the same time \
             encouraging innovation and growth.\" \u{2014} Ch 6, p.51"
        ).color(dim));
    }
}
