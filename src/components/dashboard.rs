use leptos::prelude::*;
use crate::core::system::SystemState;
use crate::domains::DomainConfig;

/// Unified knowledge panel: metric pills header + area chart.
#[component]
pub fn KnowledgePanel(
    state: RwSignal<SystemState>,
    domain_config: Memo<DomainConfig>,
) -> impl IntoView {
    // Chart viewBox — wide aspect, generous height
    let cw = 800.0_f32;
    let ch = 200.0_f32;
    let pad_left = 40.0_f32;  // space for Y-axis labels
    let pad_right = 8.0_f32;
    let pad_top = 8.0_f32;
    let pad_bottom = 20.0_f32; // space for X-axis label
    let plot_w = cw - pad_left - pad_right;
    let plot_h = ch - pad_top - pad_bottom;

    view! {
        <div class="knowledge-panel">
            // Metric pills row
            {move || {
                let s = state.get();
                let cfg = domain_config.get();
                let k = s.knowledge_quality;
                let closure = s.flow_strengths.iter().cloned().fold(f32::INFINITY, f32::min);
                let k_rate = s.params.knowledge_rate();

                let k_color = quality_color(k);
                let c_color = closure_color(closure);

                let (trend_text, trend_class) = if k_rate > 0.01 {
                    ("\u{25B2} Rising", "pill trend-rising")
                } else if k_rate < -0.01 {
                    ("\u{25BC} Falling", "pill trend-falling")
                } else {
                    ("\u{2014} Stable", "pill trend-stable")
                };

                view! {
                    <div class="metric-pills">
                        <div class="pill" style=format!("--pill-color: {};", k_color)>
                            <span class="pill-label">{cfg.knowledge_label}</span>
                            <span class="pill-value">{format!("{:.0}%", k * 100.0)}</span>
                        </div>
                        <div class="pill" style=format!("--pill-color: {};", c_color)>
                            <span class="pill-label">"Loop Closure"</span>
                            <span class="pill-value">{format!("{:.0}%", closure * 100.0)}</span>
                        </div>
                        <div class=trend_class>
                            <span class="pill-label">"Trend"</span>
                            <span class="pill-value">{trend_text}</span>
                        </div>
                    </div>
                }
            }}

            // Area chart
            <svg
                class="knowledge-chart-svg"
                viewBox=format!("0 0 {} {}", cw, ch)
                preserveAspectRatio="xMidYMid meet"
            >
                <defs>
                    // Gradient fill for area under the line
                    {move || {
                        let s = state.get();
                        let last = s.knowledge_history.last().copied().unwrap_or(0.3);
                        let color = quality_color(last);
                        view! {
                            <linearGradient id="area-fill" x1="0" y1="0" x2="0" y2="1">
                                <stop offset="0%" stop-color=color.clone() stop-opacity="0.35" />
                                <stop offset="100%" stop-color=color stop-opacity="0.03" />
                            </linearGradient>
                        }
                    }}
                </defs>

                // Chart background
                <rect
                    x=pad_left y=pad_top
                    width=plot_w height=plot_h
                    class="chart-bg"
                />

                // Y-axis grid lines + labels at 0%, 25%, 50%, 75%, 100%
                {[0.0_f32, 0.25, 0.5, 0.75, 1.0].map(|pct| {
                    let y = pad_top + plot_h - pct * plot_h;
                    view! {
                        <line
                            x1=pad_left y1=y
                            x2=pad_left + plot_w y2=y
                            class="chart-grid"
                        />
                        <text
                            x=pad_left - 6.0 y=y + 1.0
                            class="chart-grid-label"
                            text-anchor="end"
                            dominant-baseline="middle"
                        >
                            {format!("{:.0}%", pct * 100.0)}
                        </text>
                    }
                }).collect_view()}

                // Area fill + line
                {move || {
                    let s = state.get();
                    let history = &s.knowledge_history;
                    if history.len() < 2 {
                        return view! { <g /> }.into_any();
                    }
                    let n = history.len();
                    let last = history[n - 1];
                    let color = quality_color(last);

                    // Build polyline points
                    let line_points: String = history
                        .iter()
                        .enumerate()
                        .map(|(i, &v)| {
                            let x = pad_left + (i as f32 / (n - 1) as f32) * plot_w;
                            let y = pad_top + plot_h - v.clamp(0.0, 1.0) * plot_h;
                            format!("{:.1},{:.1}", x, y)
                        })
                        .collect::<Vec<_>>()
                        .join(" ");

                    // Area polygon: line points + bottom-right + bottom-left
                    let first_x = pad_left;
                    let last_x = pad_left + plot_w;
                    let bottom_y = pad_top + plot_h;
                    let area_points = format!(
                        "{} {:.1},{:.1} {:.1},{:.1}",
                        line_points, last_x, bottom_y, first_x, bottom_y
                    );

                    view! {
                        <g>
                            <polygon
                                points=area_points
                                fill="url(#area-fill)"
                            />
                            <polyline
                                points=line_points
                                class="chart-line"
                                stroke=color
                            />
                        </g>
                    }.into_any()
                }}

                // "Knowledge Quality" legend top-left inside plot area
                {move || {
                    let s = state.get();
                    let last = s.knowledge_history.last().copied().unwrap_or(0.3);
                    let color = quality_color(last);
                    view! {
                        <text
                            x=pad_left + 8.0
                            y=pad_top + 16.0
                            class="chart-legend"
                            fill=color
                        >
                            "Knowledge Quality"
                        </text>
                    }
                }}

                // X-axis label
                <text
                    x=pad_left + plot_w / 2.0
                    y=ch - 2.0
                    class="chart-axis-label"
                    text-anchor="middle"
                >
                    "Time"
                </text>
            </svg>
        </div>
    }
}

fn quality_color(value: f32) -> String {
    let v = value.clamp(0.0, 1.0);
    if v > 0.6 {
        let t = (v - 0.6) / 0.4;
        format!(
            "rgb({}, {}, {})",
            (120.0 - t * 40.0) as u8,
            (160.0 + t * 60.0) as u8,
            (80.0 + t * 40.0) as u8,
        )
    } else if v > 0.3 {
        "rgb(200, 180, 60)".to_string()
    } else {
        let t = v / 0.3;
        format!(
            "rgb(200, {}, {})",
            (60.0 + t * 100.0) as u8,
            (40.0 + t * 20.0) as u8,
        )
    }
}

fn closure_color(value: f32) -> String {
    let v = value.clamp(0.0, 1.0);
    if v > 0.6 {
        "rgb(100, 160, 220)".to_string()
    } else if v > 0.3 {
        "rgb(200, 160, 80)".to_string()
    } else {
        "rgb(200, 80, 80)".to_string()
    }
}
