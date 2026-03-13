use super::{DomainConfig, palette_legislature, palette_bureaucracy};

/// Legislature system (Ch 8, Figure 8.1).
/// Political Entrepreneurship → Resolution → Drafting → Engagement.
/// Knowledge = legislation. Anchor = constituent preferences (weak, self-modifiable).
pub fn legislature_system() -> DomainConfig {
    DomainConfig {
        name: "Legislature",
        chapter: "Ch 8: Government Systems",
        figure: "Figure 8.1",
        palette: palette_legislature(),

        process_labels: ["Political Entrepreneurship", "Resolution", "Drafting", "Engagement"],
        process_descriptions: [
            "Development of potential additions\nor amendments to existing\nlegislation.",
            "Evaluation and advancement of\nproposals for implementation.",
            "Preparation of items for\nlegislative action.",
            "Interaction with constituents\nand the political environment.",
        ],
        process_notation: [
            "Legislation + Input \u{2192} Initiatives",
            "Initiatives \u{2192} Plans",
            "Plans \u{2192} Drafts",
            "Drafts \u{2192} Legislation",
        ],

        flow_labels: ["Initiatives", "Plans", "Drafts", "K"],
        flow_descriptions: [
            "Potential additions or amendments\nto existing legislation",
            "Proposals advanced for\nimplementation",
            "Items for legislative action",
            "Updated body of legislation",
        ],

        knowledge_label: "Legislation",
        knowledge_description: "A model of the system and\nthe political environment.",

        env_input_description: "Includes observed constituent\npreferences and feedback in reaction\nto legislation and political\nadvertising as well as resources\nfor system maintenance.",

        coupling_label: "Anchor to Environment",
        coupling_tooltip: "\"The only serious fully external constraint on legislatures is the preferences of constituents, and these are notoriously divergent and usually expressed as generalities lacking in specificity.\" The legislature's anchor is ideological, not physical \u{2014} legislation modifies the very environment that produces its feedback. \u{2014} Ch 8",

        output_label: "Political Advertising",
        output_description: "Communication to constituents\nshaping political preferences —\nthe legislature modifies the very\nenvironment that constrains it.",
        env_input_label: "Constituent Preferences (I)",
        hampered_processes: [false; 4],
    }
}

/// Bureaucracy system (Ch 8, Figure 8.2).
/// Innovation → Judgment → Action → Learning.
/// Innovation and Judgment are "hampered" — constrained by external overseers.
/// Knowledge = agency knowledge. Anchor = directives from legislature, executive, judiciary.
pub fn bureaucracy_system() -> DomainConfig {
    DomainConfig {
        name: "Bureaucracy",
        chapter: "Ch 8: Government Systems",
        figure: "Figure 8.2",
        palette: palette_bureaucracy(),

        process_labels: ["Innovation", "Judgment", "Action", "Learning"],
        process_descriptions: [
            "Development of initiatives,\nconstrained by external oversight\nfrom legislature, executive,\nand judiciary.",
            "Assessment and selection of\nplans, constrained by external\noversight and directives.",
            "Implementation of agency\noperations and services.",
            "Updating of the agency's shared\nunderstanding based on consequences\nand client feedback.",
        ],
        process_notation: [
            "Knowledge + Input \u{2192} Initiatives",
            "Initiatives \u{2192} Plans",
            "Plans \u{2192} Output",
            "Consequences \u{2192} Knowledge",
        ],

        flow_labels: ["Initiatives", "Plans", "Consequences", "K"],
        flow_descriptions: [
            "Potential changes to agency\noperations and methods",
            "Directives for agency action",
            "Results of agency operations\nand client reactions",
            "Updated agency knowledge",
        ],

        knowledge_label: "Agency Knowledge",
        knowledge_description: "A shared understanding of the\nagency's mission, methods,\nenvironment, and outputs.",

        env_input_description: "Includes resources for system\nmaintenance, direct feedback from\nclients, and directives from\nlegislature, executive, and judiciary.",

        coupling_label: "Anchor to Environment",
        coupling_tooltip: "\"Government bureaucracies face constraints which, while more forgiving in terms of survival and growth, are both more invasive of their internal operations and more likely to change unpredictably.\" \u{2014} Ch 8",

        output_label: "Output (O)",
        output_description: "Agency services, regulatory\nactions, and operations\ndelivered to clients and\nthe public.",
        env_input_label: "Directives & Feedback (I)",
        hampered_processes: [true, true, false, false],
    }
}
