use leptos::prelude::*;
use crate::core::abm::AbmState;
use crate::domains::DomainConfig;

const NUM_BINS: usize = 25;

/// Belief distribution histogram — shows where agents think the environment
/// state is vs. where it actually is. Tight cluster near the marker = the
/// population has learned. Wide spread = the loop is broken.
#[component]
pub fn AgentCanvas(
    abm: ReadSignal<Option<AbmState>>,
    domain_config: Memo<DomainConfig>,
) -> impl IntoView {
    let vb_w = 600.0_f32;
    let vb_h = 340.0_f32;
    let pad_l = 44.0_f32;
    let pad_r = 16.0_f32;
    let pad_t = 36.0_f32;
    let pad_b = 44.0_f32;

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
                    // Round up max to nearest multiple of 5 for clean gridlines
                    let raw_max = bins.iter().copied().max().unwrap_or(1).max(1);
                    let max_bin = ((raw_max + 4) / 5 * 5).max(5);

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

                        // Color by distance from environment state
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

                    // Y-axis gridlines
                    let y_ticks: Vec<u32> = (0..=max_bin).step_by(5).collect();
                    let gridlines: Vec<_> = y_ticks.iter().filter(|&&v| v > 0).map(|&v| {
                        let y = pad_t + plot_h - (v as f32 / max_bin as f32) * plot_h;
                        view! {
                            <line x1=pad_l x2={pad_l + plot_w} y1=y y2=y
                                class="chart-grid" />
                        }
                    }).collect();

                    // Y-axis labels
                    let y_labels: Vec<_> = y_ticks.iter().map(|&v| {
                        let y = pad_t + plot_h - (v as f32 / max_bin as f32) * plot_h;
                        view! {
                            <text x={pad_l - 6.0} y={y + 3.5}
                                fill="var(--text-dim)" font-size="10" text-anchor="end">
                                {v.to_string()}
                            </text>
                        }
                    }).collect();

                    let accent_css = palette.accent.to_css();

                    view! {
                        <g>
                            // Plot background
                            <rect x=pad_l y=pad_t width=plot_w height=plot_h
                                fill="var(--bg-canvas)" stroke="var(--separator)" stroke-width="0.5" rx="3" />

                            // Y gridlines
                            {gridlines}

                            // Histogram bars
                            {bars}

                            // Environment state: triangle marker on x-axis + label
                            <polygon
                                points=format!("{},{} {},{} {},{}",
                                    gt_x, pad_t + plot_h - 8.0,
                                    gt_x - 5.0, pad_t + plot_h + 1.0,
                                    gt_x + 5.0, pad_t + plot_h + 1.0)
                                fill=accent_css.clone()
                            />
                            <text x=gt_x y={pad_t + plot_h + 28.0}
                                fill=accent_css.clone() font-size="9" text-anchor="middle" font-weight="600">
                                {format!("\u{25B2} Env: {:.2}", gt)}
                            </text>

                            // Title row: chart title left, stats right
                            <text x=pad_l y={pad_t - 14.0}
                                fill="var(--text-primary)" font-size="13" font-weight="600">
                                "Belief Distribution"
                            </text>
                            <text x={pad_l + plot_w} y={pad_t - 14.0}
                                fill="var(--text-secondary)" font-size="11" text-anchor="end">
                                {format!("{} agents  \u{00B7}  {:.0}% transacting  \u{00B7}  mean accuracy {:.0}%",
                                    state.agents.len(), tx_pct, mean_acc * 100.0)}
                            </text>

                            // X-axis
                            <text x=pad_l y={pad_t + plot_h + 16.0}
                                fill="var(--text-dim)" font-size="10" text-anchor="start">
                                "0"
                            </text>
                            <text x={pad_l + plot_w * 0.25} y={pad_t + plot_h + 16.0}
                                fill="var(--text-dim)" font-size="10" text-anchor="middle">
                                "0.25"
                            </text>
                            <text x={pad_l + plot_w * 0.5} y={pad_t + plot_h + 16.0}
                                fill="var(--text-dim)" font-size="10" text-anchor="middle">
                                "0.50"
                            </text>
                            <text x={pad_l + plot_w * 0.75} y={pad_t + plot_h + 16.0}
                                fill="var(--text-dim)" font-size="10" text-anchor="middle">
                                "0.75"
                            </text>
                            <text x={pad_l + plot_w} y={pad_t + plot_h + 16.0}
                                fill="var(--text-dim)" font-size="10" text-anchor="end">
                                "1.0"
                            </text>
                            <text x={pad_l + plot_w / 2.0} y={pad_t + plot_h + 34.0}
                                fill="var(--text-secondary)" font-size="11" text-anchor="middle">
                                "Agent Knowledge (K)"
                            </text>

                            // Y-axis labels
                            {y_labels}
                            <text x="10" y={pad_t + plot_h / 2.0}
                                fill="var(--text-secondary)" font-size="11" text-anchor="middle"
                                transform=format!("rotate(-90, 10, {})", pad_t + plot_h / 2.0)>
                                "Agents"
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
        let t = acc * 2.0;
        (200.0 + t * 20.0, 60.0 + t * 140.0, 60.0)
    } else {
        let t = (acc - 0.5) * 2.0;
        (220.0 - t * 160.0, 200.0, 60.0 + t * 20.0)
    };
    format!("rgb({:.0},{:.0},{:.0})", r, g, b)
}
