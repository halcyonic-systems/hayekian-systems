pub mod market;
pub mod science;
pub mod government;

/// UI-agnostic color — 3 u8s, convertible to CSS at render time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn to_css(self) -> String {
        format!("rgb({}, {}, {})", self.0, self.1, self.2)
    }
    pub fn to_css_alpha(self, alpha: f32) -> String {
        format!("rgba({}, {}, {}, {:.2})", self.0, self.1, self.2, alpha)
    }
    pub fn r(self) -> u8 { self.0 }
    pub fn g(self) -> u8 { self.1 }
    pub fn b(self) -> u8 { self.2 }
}

/// Color palette for a domain — provides visual differentiation at a glance.
#[derive(Clone, Debug, PartialEq)]
pub struct DomainPalette {
    pub accent: Color,
    pub accent_dim: Color,
    pub knowledge_accent: Color,
    pub flow_healthy: Color,
    pub flow_warning: Color,
    pub flow_danger: Color,
}

/// Domain-specific configuration for an anticipatory system.
/// Same simulation engine, different labels — the user discovers
/// that these are structurally the same kind of system.
#[derive(Clone, Debug, PartialEq)]
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

    /// Label for the external output arrow (default "Output (O)", Science = "Probing", etc.)
    pub output_label: &'static str,
    /// Domain-specific description of what the output represents (for tooltip)
    pub output_description: &'static str,
    /// Domain-specific label for the environmental input arrow
    pub env_input_label: &'static str,
    /// Which processes are "hampered" by external constraint (drawn with dashed border).
    /// Bureaucracy has Innovation and Judgment hampered. Default: all false.
    pub hampered_processes: [bool; 4],

    /// Domain-specific default structural parameters encoding McQuade's theoretical claims.
    /// If None, uses StructuralParams::default().
    pub default_params: Option<crate::core::params::StructuralParams>,
}

/// Blue-grey palette — neutral, theoretical (Ch 5 abstract template).
pub fn palette_abstract() -> DomainPalette {
    DomainPalette {
        accent: Color(140, 160, 200),
        accent_dim: Color(50, 60, 90),
        knowledge_accent: Color(180, 180, 200),
        flow_healthy: Color(100, 200, 160),
        flow_warning: Color(200, 200, 100),
        flow_danger: Color(200, 80, 80),
    }
}

/// Green palette — growth, exchange, money (Ch 6 market).
pub fn palette_market() -> DomainPalette {
    DomainPalette {
        accent: Color(100, 200, 120),
        accent_dim: Color(35, 70, 45),
        knowledge_accent: Color(120, 210, 140),
        flow_healthy: Color(80, 210, 130),
        flow_warning: Color(200, 200, 80),
        flow_danger: Color(200, 80, 80),
    }
}

/// Teal/cyan palette — corporate, contained (Ch 6 firm).
pub fn palette_firm() -> DomainPalette {
    DomainPalette {
        accent: Color(80, 190, 200),
        accent_dim: Color(30, 60, 70),
        knowledge_accent: Color(100, 200, 210),
        flow_healthy: Color(80, 200, 200),
        flow_warning: Color(200, 200, 80),
        flow_danger: Color(200, 80, 80),
    }
}

/// Gold/amber palette — currency, reserves (Ch 6 free banking).
pub fn palette_banking() -> DomainPalette {
    DomainPalette {
        accent: Color(220, 180, 80),
        accent_dim: Color(70, 55, 30),
        knowledge_accent: Color(230, 190, 90),
        flow_healthy: Color(220, 190, 80),
        flow_warning: Color(200, 160, 60),
        flow_danger: Color(200, 80, 80),
    }
}

/// Purple/violet palette — inquiry, knowledge (Ch 7 science).
pub fn palette_science() -> DomainPalette {
    DomainPalette {
        accent: Color(160, 120, 200),
        accent_dim: Color(55, 40, 75),
        knowledge_accent: Color(180, 140, 220),
        flow_healthy: Color(140, 120, 210),
        flow_warning: Color(200, 180, 80),
        flow_danger: Color(200, 80, 80),
    }
}

/// Red/crimson palette — authority, law (Ch 8 legislature).
pub fn palette_legislature() -> DomainPalette {
    DomainPalette {
        accent: Color(200, 90, 90),
        accent_dim: Color(70, 35, 35),
        knowledge_accent: Color(210, 110, 110),
        flow_healthy: Color(200, 100, 100),
        flow_warning: Color(200, 160, 80),
        flow_danger: Color(180, 60, 60),
    }
}

/// Muted brown/grey palette — institutional, constrained (Ch 8 bureaucracy).
pub fn palette_bureaucracy() -> DomainPalette {
    DomainPalette {
        accent: Color(160, 140, 110),
        accent_dim: Color(55, 48, 38),
        knowledge_accent: Color(170, 150, 120),
        flow_healthy: Color(160, 145, 115),
        flow_warning: Color(180, 150, 80),
        flow_danger: Color(180, 80, 80),
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
            "K + Input \u{2192} Dispositions",
            "Dispositions \u{2192} Plans",
            "Plans \u{2192} Output",
            "Outcomes \u{2192} K",
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

        output_label: "Output (O)",
        output_description: "The system's actions directed at\nthe environment, generating\nconsequences that feed back\ninto learning.",
        env_input_label: "Env Input (I)",
        hampered_processes: [false; 4],
        default_params: None,
    }
}
