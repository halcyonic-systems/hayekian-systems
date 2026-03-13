use super::{DomainConfig, palette_science};

/// Science system (Ch 7, Figure 7.1).
/// Anticipation → Judgment → Investigation → Assessment.
/// Knowledge = scientific knowledge + reputations. Anchor = empirical nature.
pub fn science_system() -> DomainConfig {
    DomainConfig {
        name: "Science System",
        chapter: "Ch 7: Science",
        figure: "Figure 7.1",
        palette: palette_science(),

        process_labels: ["Anticipation", "Judgment", "Investigation", "Assessment"],
        process_descriptions: [
            "Development of hypotheses and\nresearch proposals based on\nexisting scientific knowledge.",
            "Evaluation and selection of\nhypotheses chosen for\ninvestigation.",
            "Execution of research plans,\nexperimental probing of the\nnatural environment.",
            "Evaluation of results and\nupdating of scientific knowledge\nand reputations.",
        ],
        process_notation: [
            "Knowledge + Input \u{2192} Hypotheses",
            "Hypotheses \u{2192} Plans",
            "Plans \u{2192} Results + Probing",
            "Results \u{2192} Knowledge",
        ],

        flow_labels: ["Proposals", "Plans", "Results", "K"],
        flow_descriptions: [
            "Papers setting out hypotheses",
            "Hypotheses chosen for investigation",
            "Papers and other presentations",
            "Updated scientific knowledge\nand reputations",
        ],

        knowledge_label: "Scientific Knowledge",
        knowledge_description: "A model of the natural environment\nand of the methods for its\ninvestigation, and the reputations\nof scientists.",

        env_input_description: "Includes funding and other resources\nfor system maintenance and feedback\nin reaction to experimental probing.",

        coupling_label: "Anchor to Environment",
        coupling_tooltip: "Science is anchored to empirical nature \u{2014} experiments constrain theory. The natural environment provides feedback that cannot be wished away or legislated, making science's anchor uniquely strong among social systems. \u{2014} Ch 7",

        output_label: "Probing",
        output_description: "Experimental probing of the\nnatural environment — the\nsystem's distinctive output\nthat generates empirical feedback.",
        env_input_label: "Empirical Feedback (I)",
        hampered_processes: [false; 4],
    }
}
