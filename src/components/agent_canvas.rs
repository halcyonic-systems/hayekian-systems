use leptos::prelude::*;
use crate::core::abm::AbmState;
use crate::domains::DomainConfig;

const NUM_BINS: usize = 20;

/// Belief distribution histogram — shows where agents think truth is
/// vs. where it actually is. Tight cluster near the marker = population
/// has learned. Wide spread = loop is broken.
#[component]
pub fn AgentCanvas(
    abm: ReadSignal<Option<AbmState>>,
    domain_config: Memo<DomainConfig>,
) -> impl IntoView {
    let vb_w = 440.0_f32;
    let vb_h = 200.0_f32;
    let pad_l = 32.0_f32;
    let pad_r = 12.0_f32;
    let pad_t = 24.0_f32;
    let pad_b = 32.0_f32;

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
                    let n = state.agents.len() as f32;
                    let plot_w = vb_w - pad_l - pad_r;
                    let plot_h = vb_h - pad_t - pad_b;

                    // Bin agents by belief value
                    let mut bins = [0u32; NUM_BINS];
                    for agent in &state.agents {
                        let idx = ((agent.beliefs.clamp(0.0, 0.999)) * NUM_BINS as f32) as usize;
                        bins[idx.min(NUM_BINS - 1)] += 1;
                    }
                    let max_bin = bins.iter().copied().max().unwrap_or(1).max(1);

                    // Compute summary stats
                    let accuracies: Vec<f32> = state.agents.iter()
                        .map(|a| a.belief_accuracy(gt))
                        .collect();
                    let mean_acc = accuracies.iter().sum::<f32>() / n.max(1.0);
                    let tx_count = state.agents.iter()
                        .filter(|a| a.transacted_this_step).count();
                    let tx_pct = tx_count as f32 / n.max(1.0) * 100.0;

                    // Build histogram bars
                    let bar_w = plot_w / NUM_BINS as f32;
                    let bar_gap = 1.0_f32;

                    let bars: Vec<_> = bins.iter().enumerate().map(|(i, &count)| {
                        let x = pad_l + i as f32 * bar_w;
                        let bar_height = (count as f32 / max_bin as f32) * plot_h;
                        let y = pad_t + plot_h - bar_height;

                        // Color by distance from ground truth
                        let bin_center = (i as f32 + 0.5) / NUM_BINS as f32;
                        let acc = 1.0 - (bin_center - gt).abs().min(1.0);
                        let fill = accuracy_color(acc);

                        view! {
                            <rect
                                x={x + bar_gap / 2.0}
                                y=y
                                width={bar_w - bar_gap}
                                height=bar_height
                                fill=fill
                                rx="1"
                            />
                        }
                    }).collect();

                    // Ground truth marker
                    let gt_x = pad_l + gt * plot_w;

                    // Y-axis tick labels (agent count)
                    let y_mid = max_bin / 2;
                    let accent_css = palette.accent.to_css();

                    view! {
                        <g>
                            // Plot background
                            <rect x=pad_l y=pad_t width=plot_w height=plot_h
                                fill="var(--bg-canvas)" stroke="var(--separator)" stroke-width="0.5" rx="2" />

                            // Histogram bars
                            {bars}

                            // Ground truth marker (in front of bars)
                            <line x1=gt_x y1=pad_t x2=gt_x y2={pad_t + plot_h}
                                stroke=accent_css.clone() stroke-width="2" stroke-dasharray="6 3" />
                            <text x=gt_x y={pad_t - 5.0}
                                fill=accent_css font-size="9" text-anchor="middle" font-weight="500">
                                "Env. State"
                            </text>

                            // X-axis labels
                            <text x=pad_l y={vb_h - 8.0}
                                fill="var(--text-dim)" font-size="8" text-anchor="start">
                                "0"
                            </text>
                            <text x={pad_l + plot_w / 2.0} y={vb_h - 8.0}
                                fill="var(--text-dim)" font-size="8" text-anchor="middle">
                                "0.5"
                            </text>
                            <text x={pad_l + plot_w} y={vb_h - 8.0}
                                fill="var(--text-dim)" font-size="8" text-anchor="end">
                                "1"
                            </text>
                            <text x={pad_l + plot_w / 2.0} y={vb_h - 0.0}
                                fill="var(--text-dim)" font-size="9" text-anchor="middle">
                                "Agent Knowledge (K)"
                            </text>

                            // Y-axis labels
                            <text x={pad_l - 4.0} y={pad_t + 4.0}
                                fill="var(--text-dim)" font-size="8" text-anchor="end">
                                {max_bin.to_string()}
                            </text>
                            <text x={pad_l - 4.0} y={pad_t + plot_h / 2.0 + 3.0}
                                fill="var(--text-dim)" font-size="8" text-anchor="end">
                                {y_mid.to_string()}
                            </text>
                            <text x={pad_l - 4.0} y={pad_t + plot_h}
                                fill="var(--text-dim)" font-size="8" text-anchor="end">
                                "0"
                            </text>

                            // Summary stats (top-right)
                            <text x={pad_l + plot_w - 2.0} y={pad_t + 12.0}
                                fill="var(--text-secondary)" font-size="10" text-anchor="end">
                                {format!("{} agents | {:.0}% transacting | accuracy {:.0}%",
                                    state.agents.len(), tx_pct, mean_acc * 100.0)}
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
