use super::{DomainConfig, palette_market, palette_firm, palette_banking};

/// Market system (Ch 6, Figure 6.1).
/// Innovation → Judgment → Production → Exchange.
/// Knowledge = price structure. Anchor = resource scarcity + exchange feedback.
pub fn market_system() -> DomainConfig {
    DomainConfig {
        name: "Market System",
        chapter: "Ch 6: Economic Systems",
        figure: "Figure 6.1",
        palette: palette_market(),

        process_labels: ["Innovation", "Judgment", "Production", "Exchange"],
        process_descriptions: [
            "Development of ideas for new\nproducts and improvements to\nexisting products.",
            "Assessment and selection of\nproposals for viability\nand financing.",
            "Implementation of\nproduction plans.",
            "Transactions between buyers\nand sellers affected by\npreferences, expectations,\nand availabilities.",
        ],
        process_notation: [
            "Prices + Input \u{2192} Proposals",
            "Proposals \u{2192} Plans",
            "Plans \u{2192} Output",
            "Outcomes \u{2192} Prices",
        ],

        flow_labels: ["Proposals", "Plans", "Results", "K"],
        flow_descriptions: [
            "Entrepreneurial initiatives for future\nproducts, services, and production methods",
            "Directives for production, including the\ncreation and organization of firms",
            "Outcomes of production relative to plans,\nand information (including advertising)\nas to their desirability",
            "Updated price structure",
        ],

        knowledge_label: "Price Structure",
        knowledge_description: "The price structure as a model\nof the market, its products and\ntheir brand reputations, and\nits environment.",

        env_input_description: "Includes resources for system\nmaintenance and feedback in\nreaction to output.",

        coupling_label: "Anchor to Environment",
        coupling_tooltip: "\"The market system is firmly anchored to the environment in that its production processes are sensitive to real changes in resource availabilities, and such changes are reflected in changes to the price structure.\" \u{2014} Ch 6, p.87",
    }
}

/// Firm as anticipatory system (Ch 6, Figure 6.2).
/// Innovation → Judgment → Production → Learning.
/// Knowledge = firm's internal model. Anchor = market feedback + financial consequences.
pub fn firm_system() -> DomainConfig {
    DomainConfig {
        name: "Firm",
        chapter: "Ch 6: Economic Systems",
        figure: "Figure 6.2",
        palette: palette_firm(),

        process_labels: ["Innovation", "Judgment", "Production", "Learning"],
        process_descriptions: [
            "Development of ideas for new\nproducts and improvements to\nexisting products.",
            "Assessment and selection\n(by management) of proposals\nfor viability.",
            "Implementation of\nproduction plans.",
            "Transactions sharing information\nabout customers, suppliers,\nproducts, and the market\nand social environment.",
        ],
        process_notation: [
            "Model + Input \u{2192} Proposals",
            "Proposals \u{2192} Plans",
            "Plans \u{2192} Output",
            "Outcomes \u{2192} Model",
        ],

        flow_labels: ["Proposals", "Plans", "Results", "K"],
        flow_descriptions: [
            "Entrepreneurial initiatives for future\nproducts, services, and production methods",
            "Directives for production",
            "Outcomes of production relative to plans,\nincluding financial consequences",
            "Updated internal model of the firm",
        ],

        knowledge_label: "Firm Knowledge",
        knowledge_description: "An internal model of the firm,\nits products and their brand\nreputations, and its environment.",

        env_input_description: "Includes financial and other\nresources for system maintenance\nand feedback in reaction to output.",

        coupling_label: "Anchor to Environment",
        coupling_tooltip: "The firm's anchor is market feedback \u{2014} profitability as a lagging indicator of successful adaptation, plus direct customer and supplier feedback. Bureaucratic hierarchies can attenuate this signal. \u{2014} Ch 6",
    }
}

/// Free banking system (Ch 6, Figure 6.3).
/// Anticipation → Judgment → Banking → Clearing.
/// Knowledge = clearing data + reserve levels + interest rates.
/// Anchor = redemptions + clearing feedback.
pub fn free_banking_system() -> DomainConfig {
    DomainConfig {
        name: "Free Banking System",
        chapter: "Ch 6: Economic Systems",
        figure: "Figure 6.3",
        palette: palette_banking(),

        process_labels: ["Anticipation", "Judgment", "Banking", "Clearing"],
        process_descriptions: [
            "Development of ideas for changes\nin banking practices in the\ncontext of the market environment.",
            "Assessment and selection of\ninitiatives for viability\nand financing.",
            "Issuance and management of\nloans and provision of checking,\nsavings, and other banking services.",
            "Interbank clearing and associated\nassessments of member bank\nfinancial stability.",
        ],
        process_notation: [
            "Reserves + Input \u{2192} Proposals",
            "Proposals \u{2192} Plans",
            "Plans \u{2192} Output",
            "Clearings \u{2192} Reserves",
        ],

        flow_labels: ["Proposals", "Plans", "Results", "K"],
        flow_descriptions: [
            "Initiatives for adjustment of reserve\nbalances, development of loan\nopportunities, and enhancement of\nbanking services",
            "Directives for reserve adjustment,\nlending activity, interest rates,\nand banking services",
            "Information on redemption activity,\ndeposit and reserve levels,\nand profitability",
            "Updated clearing characteristics\nand monetary environment model",
        ],

        knowledge_label: "Clearing Knowledge",
        knowledge_description: "Clearing characteristics, levels of\nbank reserves, interest rates, and\nthe reputations of banks as a model\nof the monetary aspects of\nthe environment.",

        env_input_description: "Includes redemptions and\nfeedback on services provided.",

        coupling_label: "Anchor to Environment",
        coupling_tooltip: "The clearing process generates knowledge of the banking system and the monetary environment. Overissue leads to adverse clearings and reserve loss \u{2014} direct negative feedback. \"Even in the unlikely case of overissue by all banks in concert, there will be negative feedback.\" \u{2014} Ch 6",
    }
}
