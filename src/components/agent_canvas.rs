use leptos::prelude::*;
use crate::core::abm::AbmState;
use crate::domains::DomainConfig;

/// SVG scatter plot showing agents in belief-space.
/// X = beliefs (agent's model of environment), Y = perception.
/// Circle size = wealth, color = belief accuracy (red→yellow→green).
/// Ground truth shown as dashed vertical line.
#[component]
pub fn AgentCanvas(
    abm: ReadSignal<Option<AbmState>>,
    domain_config: Memo<DomainConfig>,
) -> impl IntoView {
    let vb_w = 400.0_f32;
    let vb_h = 250.0_f32;
    let pad = 30.0_f32;

    view! {
        <div class="agent-canvas-container">
            <svg
                class="agent-canvas"
                viewBox=format!("0 0 {} {}", vb_w, vb_h)
                preserveAspectRatio="xMidYMid meet"
            >
                {move || {
                    let Some(ref state) = abm.get() else {
                        return view! { <g /> }.into_any();
                    };

                    let palette = domain_config.get().palette;
                    let gt = state.env.ground_truth;
                    let plot_w = vb_w - 2.0 * pad;
                    let plot_h = vb_h - 2.0 * pad;

                    // Ground truth line (dashed vertical)
                    let gt_x = pad + gt * plot_w;

                    // Collect transaction lines first (behind agents)
                    let mut tx_lines: Vec<(f32, f32, f32, f32)> = Vec::new();
                    for agent in &state.agents {
                        if let Some(pid) = agent.last_partner {
                            if agent.id < pid {
                                if let Some(partner) = state.agents.iter().find(|a| a.id == pid) {
                                    let x1 = pad + agent.beliefs.clamp(0.0, 1.0) * plot_w;
                                    let y1 = pad + (1.0 - agent.perception.clamp(0.0, 1.0)) * plot_h;
                                    let x2 = pad + partner.beliefs.clamp(0.0, 1.0) * plot_w;
                                    let y2 = pad + (1.0 - partner.perception.clamp(0.0, 1.0)) * plot_h;
                                    tx_lines.push((x1, y1, x2, y2));
                                }
                            }
                        }
                    }

                    // Build agent circles
                    let agents_view: Vec<_> = state.agents.iter().map(|agent| {
                        let acc = agent.belief_accuracy(gt);
                        let cx = pad + agent.beliefs.clamp(0.0, 1.0) * plot_w;
                        let cy = pad + (1.0 - agent.perception.clamp(0.0, 1.0)) * plot_h;
                        let r = 4.0 + agent.wealth.clamp(0.0, 4.0) * 2.0; // radius 4-12
                        let fill = accuracy_color(acc);
                        let title = format!(
                            "Agent {} | K (beliefs): {:.2} | Accuracy: {:.0}% | Wealth: {:.2} | Env. coupling: {:.0}%",
                            agent.id,
                            agent.beliefs,
                            acc * 100.0,
                            agent.wealth,
                            agent.perception * 100.0,
                        );
                        let stroke = if agent.transacted_this_step { "rgba(255,255,255,0.6)" } else { "none" };
                        view! {
                            <circle cx=cx cy=cy r=r fill=fill stroke=stroke stroke-width="1">
                                <title>{title}</title>
                            </circle>
                        }
                    }).collect();

                    let accent_css = palette.accent.to_css_alpha(0.30);
                    let tx_view: Vec<_> = tx_lines.iter().map(|(x1, y1, x2, y2)| {
                        view! {
                            <line
                                x1=*x1 y1=*y1 x2=*x2 y2=*y2
                                stroke=accent_css.clone()
                                stroke-width="1"
                            />
                        }
                    }).collect();

                    view! {
                        <g>
                            // Plot background
                            <rect x=pad y=pad width=plot_w height=plot_h
                                fill="var(--bg-canvas)" stroke="var(--separator)" stroke-width="0.5" rx="3" />

                            // Ground truth line
                            <line x1=gt_x y1=pad x2=gt_x y2={pad + plot_h}
                                stroke="var(--env-stroke)" stroke-width="1.5" stroke-dasharray="4 3" />
                            <text x=gt_x y={pad - 6.0}
                                fill="var(--env-label)" font-size="9" text-anchor="middle">
                                "Environment State"
                            </text>

                            // Transaction lines
                            {tx_view}

                            // Agent circles
                            {agents_view}

                            // Axis labels
                            <text x={pad + plot_w / 2.0} y={vb_h - 4.0}
                                fill="var(--text-dim)" font-size="10" text-anchor="middle">
                                "Agent Knowledge (K)"
                            </text>
                            <text x="8" y={pad + plot_h / 2.0}
                                fill="var(--text-dim)" font-size="10" text-anchor="middle"
                                transform=format!("rotate(-90, 8, {})", pad + plot_h / 2.0)>
                                "Env. Coupling"
                            </text>

                            // Scale labels
                            <text x=pad y={vb_h - 14.0}
                                fill="var(--text-dim)" font-size="8" text-anchor="start">
                                "0"
                            </text>
                            <text x={pad + plot_w} y={vb_h - 14.0}
                                fill="var(--text-dim)" font-size="8" text-anchor="end">
                                "1"
                            </text>
                        </g>
                    }.into_any()
                }}
            </svg>
        </div>
    }
}

/// Map belief accuracy [0, 1] to a red→yellow→green color string.
fn accuracy_color(acc: f32) -> String {
    let acc = acc.clamp(0.0, 1.0);
    let (r, g, b) = if acc < 0.5 {
        // Red (200,60,60) → Yellow (220,200,60)
        let t = acc * 2.0;
        (
            200.0 + t * 20.0,
            60.0 + t * 140.0,
            60.0,
        )
    } else {
        // Yellow (220,200,60) → Green (60,200,80)
        let t = (acc - 0.5) * 2.0;
        (
            220.0 - t * 160.0,
            200.0,
            60.0 + t * 20.0,
        )
    };
    format!("rgb({:.0},{:.0},{:.0})", r, g, b)
}
