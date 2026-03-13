pub mod market;

use egui::Color32;

/// Color palette for a domain — provides visual differentiation at a glance.
/// Each domain gets its own accent color scheme so switching domains
/// feels like entering a different system, not just relabeling text.
#[derive(Clone, Debug)]
pub struct DomainPalette {
    /// Primary accent for process box borders and active elements
    pub accent: Color32,
    /// Dimmer version for process box fill at full activation
    pub accent_dim: Color32,
    /// Knowledge box border color
    pub knowledge_accent: Color32,
    /// Flow arrow color when healthy (strength > 0.6)
    pub flow_healthy: Color32,
    /// Flow arrow color when degraded (strength 0.3–0.6)
    pub flow_warning: Color32,
    /// Flow arrow color when broken (strength < 0.3)
    pub flow_danger: Color32,
}

/// Domain-specific configuration for an anticipatory system.
/// Same simulation engine, different labels — the user discovers
/// that these are structurally the same kind of system.
#[derive(Clone, Debug)]
pub struct DomainConfig {
    /// Display name for this domain (e.g., "Market System")
    pub name: &'static str,
    /// Which chapter this comes from
    pub chapter: &'static str,
    /// McQuade figure reference
    pub figure: &'static str,
    /// Visual color scheme for this domain
    pub palette: DomainPalette,

    /// Process labels in loop order: [E, S, A, L]
    pub process_labels: [&'static str; 4],
    /// Process descriptions (for tooltips) in loop order
    pub process_descriptions: [&'static str; 4],
    /// Functional notation per process in loop order
    pub process_notation: [&'static str; 4],

    /// Intermediate flow labels in loop order: [E→S, S→A, A→L, L→E]
    pub flow_labels: [&'static str; 4],
    /// Flow descriptions (for tooltips)
    pub flow_descriptions: [&'static str; 4],

    /// Knowledge box label
    pub knowledge_label: &'static str,
    /// Knowledge description (for tooltip)
    pub knowledge_description: &'static str,

    /// Environmental input description
    pub env_input_description: &'static str,

    /// Label for the environmental_coupling parameter in this domain.
    /// Ch 5 = "Environmental Coupling"; Ch 6+ = "Anchor to Environment" with domain-specific text.
    pub coupling_label: &'static str,
    /// Tooltip for the coupling parameter in this domain
    pub coupling_tooltip: &'static str,
}

/// Blue-grey palette — neutral, theoretical (Ch 5 abstract template).
pub fn palette_abstract() -> DomainPalette {
    DomainPalette {
        accent: Color32::from_rgb(140, 160, 200),
        accent_dim: Color32::from_rgb(50, 60, 90),
        knowledge_accent: Color32::from_rgb(180, 180, 200),
        flow_healthy: Color32::from_rgb(100, 200, 160),
        flow_warning: Color32::from_rgb(200, 200, 100),
        flow_danger: Color32::from_rgb(200, 80, 80),
    }
}

/// Green palette — growth, exchange, money (Ch 6 market).
pub fn palette_market() -> DomainPalette {
    DomainPalette {
        accent: Color32::from_rgb(100, 200, 120),
        accent_dim: Color32::from_rgb(35, 70, 45),
        knowledge_accent: Color32::from_rgb(120, 210, 140),
        flow_healthy: Color32::from_rgb(80, 210, 130),
        flow_warning: Color32::from_rgb(200, 200, 80),
        flow_danger: Color32::from_rgb(200, 80, 80),
    }
}

/// Teal/cyan palette — corporate, contained (Ch 6 firm).
pub fn palette_firm() -> DomainPalette {
    DomainPalette {
        accent: Color32::from_rgb(80, 190, 200),
        accent_dim: Color32::from_rgb(30, 60, 70),
        knowledge_accent: Color32::from_rgb(100, 200, 210),
        flow_healthy: Color32::from_rgb(80, 200, 200),
        flow_warning: Color32::from_rgb(200, 200, 80),
        flow_danger: Color32::from_rgb(200, 80, 80),
    }
}

/// Gold/amber palette — currency, reserves (Ch 6 free banking).
pub fn palette_banking() -> DomainPalette {
    DomainPalette {
        accent: Color32::from_rgb(220, 180, 80),
        accent_dim: Color32::from_rgb(70, 55, 30),
        knowledge_accent: Color32::from_rgb(230, 190, 90),
        flow_healthy: Color32::from_rgb(220, 190, 80),
        flow_warning: Color32::from_rgb(200, 160, 60),
        flow_danger: Color32::from_rgb(200, 80, 80),
    }
}

/// Abstract anticipatory system template (Ch 5, Figure 5.2).
pub fn abstract_system() -> DomainConfig {
    DomainConfig {
        name: "Abstract Anticipatory System",
        chapter: "Ch 5: Biological Systems Theory",
        figure: "Figure 5.2",
        palette: palette_abstract(),

        process_labels: ["Expectation", "Selection", "Action", "Learning"],
        process_descriptions: [
            "Development of possible future\nscenarios based on existing\nknowledge.",
            "Evaluation of possible responses\nbased on current situation.",
            "Implementation of plans\nfor action.",
            "Updating of the internal model\nbased on sensory input and\nthe results of action.",
        ],
        process_notation: [
            "E(K, I) \u{2192} D",
            "S(D, K, I) \u{2192} P",
            "A(P, K, I) \u{2192} O, C",
            "L(C, O, K, I) \u{2192} K",
        ],

        flow_labels: ["D", "P", "O, C", "K"],
        flow_descriptions: [
            "Dispositions — potential adaptive responses",
            "Plans — directives for action",
            "Output, Consequences — results relative to action plans",
            "Knowledge — updated internal model",
        ],

        knowledge_label: "Knowledge (K)",
        knowledge_description: "A model of the system and\nits environment.",

        env_input_description: "Includes resources for system\nmaintenance and feedback in\nreaction to output.",

        coupling_label: "Environmental Coupling",
        coupling_tooltip: "How strongly the system's processes are coupled to environmental input. \"The system's input from the environment may be processed within the system to confront, and perhaps modify, the model.\" \u{2014} Ch 5",
    }
}
