use leptos::prelude::*;
use crate::core::system::SystemState;
use crate::domains::{Color, DomainConfig, DomainPalette};

/// SVG viewBox dimensions
const VW: f32 = 600.0;
const VH: f32 = 400.0;

/// Process box dimensions
const BOX_W: f32 = 148.0;
const BOX_H: f32 = 52.0;

/// Box centers: E(top-right), S(top-left), A(bottom-left), L(bottom-right)
const CX: f32 = VW / 2.0;
const CY: f32 = VH / 2.0;
const MX: f32 = 130.0; // horizontal margin from center
const MY: f32 = 85.0;  // vertical margin from center

fn box_centers() -> [(f32, f32); 4] {
    [
        (CX + MX, CY - MY), // [0] E / Innovation (top-right)
        (CX - MX, CY - MY), // [1] S / Judgment (top-left)
        (CX - MX, CY + MY), // [2] A / Production (bottom-left)
        (CX + MX, CY + MY), // [3] L / Exchange (bottom-right)
    ]
}

/// Boundary rectangle
const BOUND_X: f32 = CX - 260.0;
const BOUND_Y: f32 = CY - 160.0;
const BOUND_W: f32 = 520.0;
const BOUND_H: f32 = 320.0;

#[component]
pub fn AlesLoop(
    state: RwSignal<SystemState>,
    domain_config: Memo<DomainConfig>,
    light_mode: ReadSignal<bool>,
) -> impl IntoView {
    let centers = box_centers();

    view! {
        <div class="ales-loop-container">
            <svg
                viewBox=format!("0 0 {} {}", VW, VH)
                class="ales-loop"
                xmlns="http://www.w3.org/2000/svg"
            >
                // Arrow marker definition
                <defs>
                    {move || {
                        let cfg = domain_config.get();
                        let s = state.get();
                        // Generate markers for each flow color
                        let mut markers = Vec::new();
                        for i in 0..4 {
                            let flow = s.flow_strengths[i];
                            let color = flow_color(flow, &cfg.palette);
                            markers.push(view! {
                                <marker
                                    id=format!("arrow-{}", i)
                                    viewBox="0 0 10 10"
                                    refX="10"
                                    refY="5"
                                    markerWidth="7"
                                    markerHeight="7"
                                    orient="auto-start-reverse"
                                >
                                    <polygon points="0,1 10,5 0,9" fill=color.to_css() />
                                </marker>
                            }.into_any());
                        }
                        // Knowledge arrow marker
                        let ka = cfg.palette.knowledge_accent;
                        markers.push(view! {
                            <marker
                                id="arrow-knowledge"
                                viewBox="0 0 10 10"
                                refX="10"
                                refY="5"
                                markerWidth="6"
                                markerHeight="6"
                                orient="auto-start-reverse"
                            >
                                <polygon points="0,1 10,5 0,9" fill=ka.to_css() />
                            </marker>
                        }.into_any());
                        // Env arrow marker
                        markers.push(view! {
                            <marker
                                id="arrow-env"
                                viewBox="0 0 10 10"
                                refX="10"
                                refY="5"
                                markerWidth="6"
                                markerHeight="6"
                                orient="auto-start-reverse"
                            >
                                <polygon points="0,1 10,5 0,9" fill=format!("var(--env-label)") />
                            </marker>
                        }.into_any());
                        markers.collect_view()
                    }}
                </defs>

                // System boundary
                <rect
                    class="boundary"
                    x=BOUND_X
                    y=BOUND_Y
                    width=BOUND_W
                    height=BOUND_H
                    rx="2"
                />

                // Flow arrows (between process boxes)
                {move || {
                    let s = state.get();
                    let cfg = domain_config.get();
                    let mut flows = Vec::new();
                    for i in 0..4 {
                        let next = (i + 1) % 4;
                        let flow = s.flow_strengths[i];
                        let color = flow_color(flow, &cfg.palette);
                        let thickness = 1.0 + flow * 2.5;
                        let alpha = (flow * 0.88 + 0.12).min(1.0);

                        let (x1, y1) = edge_point(centers[i], centers[next], true);
                        let (x2, y2) = edge_point(centers[next], centers[i], true);

                        // Flow line
                        flows.push(view! {
                            <line
                                x1=x1 y1=y1 x2=x2 y2=y2
                                stroke=color.to_css_alpha(alpha)
                                stroke-width=thickness
                                class="flow-line"
                                marker-end=format!("url(#arrow-{})", i)
                            />
                        }.into_any());

                        // Flow label at midpoint, offset perpendicular
                        let (mx, my) = ((x1 + x2) / 2.0, (y1 + y2) / 2.0);
                        let dx = x2 - x1;
                        let dy = y2 - y1;
                        let len = (dx * dx + dy * dy).sqrt().max(0.1);
                        let px = -dy / len * 14.0;
                        let py = dx / len * 14.0;
                        let label_opacity = format!("{:.2}", alpha);
                        flows.push(view! {
                            <text
                                x=mx + px
                                y=my + py
                                class="flow-label"
                                opacity=label_opacity
                            >
                                {cfg.flow_labels[i]}
                            </text>
                        }.into_any());
                    }
                    flows.collect_view()
                }}

                // Process boxes
                {move || {
                    let s = state.get();
                    let cfg = domain_config.get();
                    let lm = light_mode.get();
                    let mut boxes = Vec::new();
                    for (i, &(cx, cy)) in centers.iter().enumerate() {
                        let activation = s.process_activation[i];
                        let bg = node_color(activation, &cfg.palette, lm);
                        let hampered = cfg.hampered_processes[i];
                        let x = cx - BOX_W / 2.0;
                        let y = cy - BOX_H / 2.0;

                        let stroke_css = cfg.palette.accent.to_css();
                        let dash = if hampered { "5 3" } else { "none" };

                        // Tooltip as SVG title
                        let tooltip_text = format!(
                            "{}\n{}\n{}\nActivation: {:.0}%",
                            cfg.process_labels[i],
                            cfg.process_descriptions[i],
                            cfg.process_notation[i],
                            activation * 100.0
                        );

                        // Scale font to fit box width with padding
                        let label_len = cfg.process_labels[i].len();
                        let name_size = if label_len > 18 {
                            "10"
                        } else if label_len > 12 || hampered {
                            "11.5"
                        } else {
                            "13"
                        };

                        let notation_len = cfg.process_notation[i].len();
                        let notation_size = if notation_len > 24 {
                            "8.5"
                        } else if notation_len > 18 {
                            "9.5"
                        } else {
                            "10.5"
                        };

                        boxes.push(view! {
                            <g>
                                <title>{tooltip_text}</title>
                                <rect
                                    class="process-box"
                                    class:hampered=hampered
                                    x=x y=y
                                    width=BOX_W height=BOX_H
                                    fill=bg.to_css()
                                    stroke=stroke_css.clone()
                                    stroke-dasharray=dash
                                />
                                <text class="process-name" x=cx y=cy - 8.0 font-size=name_size>
                                    {cfg.process_labels[i]}
                                </text>
                                <text class="process-notation" x=cx y=cy + 12.0 font-size=notation_size>
                                    {cfg.process_notation[i]}
                                </text>
                            </g>
                        });
                    }
                    boxes.collect_view()
                }}

                // Knowledge box
                {move || {
                    let s = state.get();
                    let cfg = domain_config.get();
                    let lm = light_mode.get();
                    let k = s.knowledge_quality;
                    let label_len = cfg.knowledge_label.len();
                    let kw = if label_len > 14 { 126.0_f32 } else { 110.0_f32 };
                    let kh = 40.0_f32;
                    let k_font = if label_len > 16 { "9.5" } else { "11" };
                    // Positioned on the right side between E and L
                    let kx = centers[0].0;
                    let ky = (centers[0].1 + centers[3].1) / 2.0;
                    let bg = knowledge_color(k, lm);
                    let ka = cfg.palette.knowledge_accent;

                    let tooltip = format!(
                        "{}\n{}\nQuality: {:.0}%",
                        cfg.knowledge_label,
                        cfg.knowledge_description,
                        k * 100.0
                    );

                    view! {
                        <g>
                            <title>{tooltip}</title>
                            <rect
                                class="knowledge-box"
                                x=kx - kw / 2.0
                                y=ky - kh / 2.0
                                width=kw height=kh
                                fill=bg.to_css()
                                stroke=ka.to_css()
                                stroke-width="2"
                            />
                            <text class="knowledge-label" x=kx y=ky - 6.0 font-size=k_font>
                                {cfg.knowledge_label}
                            </text>
                            <text class="knowledge-pct" x=kx y=ky + 10.0 fill=ka.to_css()>
                                {format!("{:.0}%", k * 100.0)}
                            </text>
                            // Arrow pointing left from knowledge box
                            <line
                                x1=kx - kw / 2.0
                                y1=ky
                                x2=kx - kw / 2.0 - 30.0
                                y2=ky
                                stroke=ka.to_css()
                                stroke-width="1.5"
                                marker-end="url(#arrow-knowledge)"
                            />
                        </g>
                    }
                }}

                // External feedback loop: Output → Environment → Input
                {move || {
                    let s = state.get();
                    let cfg = domain_config.get();
                    let coupling = s.params.environmental_coupling;
                    let alpha = (coupling * 0.78 + 0.22).min(1.0);
                    let env_margin = 18.0_f32;

                    // Output arrow from Action (bottom-left) going LEFT
                    let out_x1 = centers[2].0 - BOX_W / 2.0;
                    let out_y = centers[2].1;
                    let out_x2 = BOUND_X - env_margin;

                    // Corner bottom-left
                    let corner_bl_x = BOUND_X - env_margin;
                    let corner_bl_y = BOUND_Y + BOUND_H + env_margin;

                    // Input enters between A and L
                    let input_x = (centers[2].0 + centers[3].0) / 2.0;
                    let corner_bm_x = input_x;
                    let corner_bm_y = BOUND_Y + BOUND_H + env_margin;

                    let entry_x = input_x;
                    let entry_y = BOUND_Y + BOUND_H;

                    let stroke_w = 1.5 + coupling;
                    let env_style = format!("stroke-width: {}; opacity: {:.2};", stroke_w, alpha);

                    let tooltip = format!(
                        "Environmental Input (I)\n{}",
                        cfg.env_input_description
                    );

                    view! {
                        <g>
                            // Output arrow
                            <line
                                x1=out_x1 y1=out_y x2=out_x2 y2=out_y
                                class="env-path" style=env_style.clone()
                                marker-end="url(#arrow-env)"
                            />
                            // Output label with domain-specific subtitle
                            <text
                                x=out_x2 - 4.0
                                y=out_y - 14.0
                                class="env-label"
                                text-anchor="end"
                                font-size="10"
                            >
                                {cfg.output_label}
                            </text>
                            {if cfg.output_label != "Output (O)" { None } else {
                                // Show domain-specific subtitle for generic "Output (O)" labels
                                let subtitle = match cfg.name {
                                    "Market System" => "Goods & Services",
                                    "Firm" => "Products & Services",
                                    "Free Banking System" => "Loans & Services",
                                    "Bureaucracy" => "Agency Services",
                                    _ => "",
                                };
                                if subtitle.is_empty() { None } else {
                                    Some(view! {
                                        <text
                                            x=out_x2 - 4.0
                                            y=out_y - 3.0
                                            class="env-label"
                                            text-anchor="end"
                                            font-size="8.5"
                                            opacity="0.7"
                                        >
                                            {subtitle}
                                        </text>
                                    })
                                }
                            }}

                            // Path down left side, across bottom
                            <polyline
                                points=format!(
                                    "{},{} {},{} {},{}",
                                    corner_bl_x, out_y,
                                    corner_bl_x, corner_bl_y,
                                    corner_bm_x, corner_bm_y
                                )
                                class="env-path" style=env_style.clone()
                                fill="none"
                            />

                            // Input arrow pointing up
                            <line
                                x1=entry_x y1=corner_bm_y
                                x2=entry_x y2=entry_y
                                class="env-path" style=env_style
                                marker-end="url(#arrow-env)"
                            />

                            // Env Input label — domain-specific
                            <text
                                x=entry_x + 14.0
                                y=corner_bm_y - 2.0
                                class="env-label"
                                font-size="10"
                            >
                                {cfg.env_input_label}
                            </text>
                            {if cfg.env_input_label == "Env Input (I)" { None } else {
                                // Show generic "(I)" below the specific label
                                Some(view! {
                                    <text
                                        x=entry_x + 14.0
                                        y=corner_bm_y + 10.0
                                        class="env-label"
                                        font-size="8.5"
                                        opacity="0.7"
                                    >
                                        "Environmental Input"
                                    </text>
                                })
                            }}
                        </g>
                    }
                }}
            </svg>
        </div>
    }
}

/// Compute edge point of a box for an arrow from `from` center to `to` center.
fn edge_point(from: (f32, f32), to: (f32, f32), departing: bool) -> (f32, f32) {
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;
    let half_w = BOX_W / 2.0;
    let half_h = BOX_H / 2.0;

    if departing {
        if dx.abs() > dy.abs() {
            let sx = if dx > 0.0 { half_w } else { -half_w };
            (from.0 + sx, from.1 + dy * (sx / dx))
        } else {
            let sy = if dy > 0.0 { half_h } else { -half_h };
            (from.0 + dx * (sy / dy), from.1 + sy)
        }
    } else {
        // Arriving at `from` from `to`
        let dx2 = from.0 - to.0;
        let dy2 = from.1 - to.1;
        if dx2.abs() > dy2.abs() {
            let sx = if dx2 > 0.0 { half_w } else { -half_w };
            (from.0 + sx, from.1 + dy2 * (sx / dx2))
        } else {
            let sy = if dy2 > 0.0 { half_h } else { -half_h };
            (from.0 + dx2 * (sy / dy2), from.1 + sy)
        }
    }
}

fn flow_color(strength: f32, palette: &DomainPalette) -> Color {
    if strength > 0.6 {
        palette.flow_healthy
    } else if strength > 0.3 {
        palette.flow_warning
    } else {
        palette.flow_danger
    }
}

fn node_color(activation: f32, palette: &DomainPalette, light_mode: bool) -> Color {
    let a = activation.clamp(0.0, 1.0);
    if light_mode {
        let accent = palette.accent;
        let mix = 0.15 + a * 0.2;
        Color(
            (255.0 - mix * (255.0 - accent.r() as f32)) as u8,
            (255.0 - mix * (255.0 - accent.g() as f32)) as u8,
            (255.0 - mix * (255.0 - accent.b() as f32)) as u8,
        )
    } else {
        let dim = palette.accent_dim;
        let bright = palette.accent;
        Color(
            (dim.r() as f32 + a * (bright.r() as f32 - dim.r() as f32) * 0.5) as u8,
            (dim.g() as f32 + a * (bright.g() as f32 - dim.g() as f32) * 0.5) as u8,
            (dim.b() as f32 + a * (bright.b() as f32 - dim.b() as f32) * 0.5) as u8,
        )
    }
}

fn knowledge_color(quality: f32, light_mode: bool) -> Color {
    let q = quality.clamp(0.0, 1.0);
    if light_mode {
        Color(
            (240.0 - q * 30.0) as u8,
            (235.0 - q * 15.0) as u8,
            (215.0 - q * 25.0) as u8,
        )
    } else {
        Color(
            (40.0 + q * 40.0) as u8,
            (40.0 + q * 60.0) as u8,
            (30.0 + q * 20.0) as u8,
        )
    }
}
