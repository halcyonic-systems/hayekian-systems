use leptos::prelude::*;
use crate::domains::DomainConfig;

#[component]
pub fn TheoryPanel(
    domain_config: Memo<DomainConfig>,
) -> impl IntoView {
    view! {
        <details class="theory-panel">
            <summary>
                {move || {
                    let cfg = domain_config.get();
                    format!("Theory \u{2014} McQuade {}: {}", cfg.chapter, cfg.name)
                }}
            </summary>
            <div class="theory-content">
                {move || {
                    let cfg = domain_config.get();
                    let mut entries = Vec::new();

                    // Figure reference
                    entries.push(view! {
                        <p style="font-style: italic; margin-bottom: 8px;">
                            {format!("{}: Process organization in {}", cfg.figure, cfg.name.to_lowercase())}
                        </p>
                    }.into_any());

                    // Process descriptions
                    for i in 0..4 {
                        entries.push(view! {
                            <div class="process-entry">
                                <strong>{cfg.process_labels[i]}</strong>
                                {cfg.process_descriptions[i]}
                            </div>
                        }.into_any());
                    }

                    // Knowledge
                    entries.push(view! {
                        <div class="process-entry" style="margin-top: 8px;">
                            <strong>{cfg.knowledge_label}</strong>
                            {cfg.knowledge_description}
                        </div>
                    }.into_any());

                    // Cross-domain insight (only when not abstract)
                    if cfg.figure != "Figure 5.2" {
                        entries.push(view! {
                            <div class="cross-domain">
                                <strong>"Cross-Domain Invariance"</strong>
                                <p>
                                    "\"At the most general level, what is crucial for adaptation in social systems \
                                     of all sorts is not the specific ability to form prices but the ability to \
                                     generate feedback effects which constrain self-interest while at the same time \
                                     encouraging innovation and growth.\" \u{2014} Ch 6, p.51"
                                </p>
                            </div>
                        }.into_any());
                    }

                    entries.collect_view()
                }}
            </div>
        </details>
    }
}
