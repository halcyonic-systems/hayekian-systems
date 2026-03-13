# Hayekian Anticipatory Systems Explorer

Interactive visualization of McQuade & Butos's anticipatory systems framework. Users control structural parameters of an abstract anticipatory system and see how the ALES loop behaves — when closure is complete and feedback is strong, knowledge accumulates; break a link and the system degrades.

## Quick Start

```bash
trunk serve              # Dev server with hot-reload
trunk build --release --public-url /hayekian-systems/  # Production WASM build
```

**Live web app**: https://halcyonic.systems/hayekian-systems/ (auto-deploys on push to main)
**Desktop launcher**: `~/Desktop/Hayekian Systems.app` — opens the web URL in default browser
**Repo**: https://github.com/halcyonic-systems/hayekian-systems

**Framework**: Leptos 0.8 CSR (client-side rendering). Migrated from egui 0.31 on 2026-03-13.
**Theme**: Light (cream) and dark (slate) modes via CSS variables + `.dark` class toggle. Default is light.

## Source Material

**Book**: McQuade & Butos, *Hayekian Systems* (Routledge)
**Chapter files**: `~/Desktop/halcyonic/operations/political-economy/hayekian-systems/chapters/`
**Parameter analysis**: `~/Desktop/halcyonic/operations/political-economy/hayekian-systems/implementation/mcquade-parameters.md`

Always read the relevant chapter before implementing. The plan's parameter names and layouts should be cross-checked against McQuade's actual figures and vocabulary.

## Architecture

```
src/
├── main.rs              # Leptos mount, root <App>, Domain enum, animation loop
├── style.css            # All styling — CSS variables for light/dark theming
├── core/                # PURE SIMULATION — no UI imports
│   ├── mod.rs
│   ├── params.rs        # 4 structural parameters + derived dynamics
│   └── system.rs        # SystemState, ALES process enum, simulation step
├── domains/             # DOMAIN DATA — no UI imports (uses Color struct, not egui)
│   ├── mod.rs           # Color, DomainConfig, DomainPalette, abstract_system(), palettes
│   ├── market.rs        # Market (Fig 6.1), Firm (Fig 6.2), Free Banking (Fig 6.3)
│   ├── science.rs       # Science (Fig 7.1)
│   └── government.rs    # Legislature (Fig 8.1), Bureaucracy (Fig 8.2)
└── components/          # LEPTOS COMPONENTS — all rendering happens here
    ├── mod.rs
    ├── ales_loop.rs     # <AlesLoop> SVG diagram (process boxes, flows, knowledge, env feedback)
    ├── controls.rs      # <ParameterControls> domain selector, sliders, run/pause/reset
    ├── dashboard.rs     # <MetricsStrip> CSS bars + <KnowledgeChart> SVG polyline
    └── theory_panel.rs  # <TheoryPanel> collapsible theory content
```

### Key Design Decisions (Leptos migration)

- **Color struct**: `domains::Color(u8, u8, u8)` replaces `egui::Color32`. Convert to CSS at render time via `.to_css()` / `.to_css_alpha()`.
- **SVG for diagrams**: ALES loop and knowledge chart use inline SVG with `viewBox` for responsiveness. No canvas, no paint calls.
- **CSS variables for theming**: `style.css` defines `--bg-panel`, `--text-primary`, etc. Dark mode via `.dark` class on root div.
- **Domain palette → CSS**: Set via inline `style` attributes or computed in component code. No egui Color32 anywhere in rendering.
- **Animation**: `request_animation_frame` recursive loop in `main.rs`, only active when `running` signal is true.
- **Tooltips**: SVG `<title>` elements (native browser tooltips). Simpler than egui's `show_tooltip_at_pointer`.

## The Four Structural Parameters

| Parameter | Field Name | Ch 5 Concept | Later Chapters |
|-----------|-----------|--------------|----------------|
| Environmental Coupling | `environmental_coupling` | How strongly processes couple to environmental input (I) | Ch 6-8: McQuade calls this the system's "anchor to environment" |
| Innovation Freedom | `innovation_freedom` | How freely Expectation generates dispositions (D) | |
| Feedback Fidelity | `feedback_fidelity` | How accurately consequences (C) feed into Learning | |
| Process Closure | `process_closure` | Whether E→S→A→L form a self-maintaining loop (Piaget/Rosen) | |

## Domain System

Each domain is a `DomainConfig` (labels, descriptions, tooltips, palette) that plugs into the same simulation engine. Switching domains relabels everything but keeps parameters and state — the user discovers cross-domain invariance by switching.

### Available Domains

| Domain | Figure | Palette | Accent |
|--------|--------|---------|--------|
| Abstract (Ch 5) | Fig 5.2 | Blue-grey | Neutral, theoretical |
| Market (Ch 6) | Fig 6.1 | Green | Growth, exchange, money |
| Firm (Ch 6) | Fig 6.2 | Teal/cyan | Corporate, contained |
| Free Banking (Ch 6) | Fig 6.3 | Gold/amber | Currency, reserves |
| Science (Ch 7) | Fig 7.1 | Purple/violet | Inquiry, knowledge |
| Legislature (Ch 8) | Fig 8.1 | Red/crimson | Authority, law |
| Bureaucracy (Ch 8) | Fig 8.2 | Muted brown | Institutional, constrained |

### Adding New Domains

1. **Read the chapter** — use McQuade's exact process labels from the figure
2. **Create a palette function** in `domains/mod.rs` (e.g., `palette_science()`)
3. **Create a domain config function** in the appropriate `domains/*.rs` file
4. **Add the variant** to the `Domain` enum in `main.rs`
5. **Color scheme should be visually distinct** from existing domains

### DomainPalette Fields

| Field | Used For |
|-------|----------|
| `accent` | Process box borders (`stroke`), active UI elements |
| `accent_dim` | Process box fill lerped with activation (dark mode) |
| `knowledge_accent` | Knowledge box border, percentage text, arrow marker |
| `flow_healthy` | Flow arrows when strength > 0.6 |
| `flow_warning` | Flow arrows when strength 0.3–0.6 |
| `flow_danger` | Flow arrows when strength < 0.3 |

## Chapter Build Order

| Ch | Book Source | Status | What It Adds |
|----|-----------|--------|-------------|
| 1 | Ch 5 (Biological Systems Theory) | **Done** | Abstract ALES loop, 4 parameters, knowledge indicator |
| 2 | Ch 6 (Economic Systems) | **Done** | Market/Firm/Banking domains, domain selector, color palettes |
| 3 | Ch 7 (Science Systems) | **Done** | Science domain, "Probing" output label, purple palette |
| 4 | Ch 8 (Government Systems) | **Done** | Legislature + bureaucracy, hampered process visuals, output_label field |
| 5 | Ch 9-10 (Interactions/State-Sponsored) | Planned | Big Player coupling across domains |
| 6 | Ch 11 (Hayekian Systems) | Planned | Side-by-side comparison dashboard |
| 7 | Real-world mapping | Planned | Historical scenarios (2008, climate science, etc.) |

## Process Flow (Figure 5.2)

```
  E (top-right) ——D——> S (top-left)
       ^                    |
       |                    P
       K                    |
       |                    v
  L (bottom-right) <—O,C— A (bottom-left)
```

Knowledge box sits on the L→E edge (right side), arrow pointing inward.

## Theme System

CSS variables in `style.css`. Two modes toggled via `.dark` class on root div:

| Mode | `--bg-panel` | `--bg-canvas` | `--text-primary` | Box Fills |
|------|-------------|--------------|-----------------|-----------|
| Light (default) | `#f5f0e6` | `#ebe4d8` | `#1e1e28` | White tinted toward accent |
| Dark | `#3a3e4a` | `#30343e` | `#ffffff` | Dark, lerped from `accent_dim` |

**When adding new UI**: Use CSS variables from `style.css`, not hardcoded colors. Domain-specific colors come from `DomainPalette` via `Color::to_css()`.

## Desktop App Launcher

Opens the live web URL in default browser:
```bash
osacompile -o ~/Desktop/"Hayekian Systems.app" -e \
  'do shell script "open https://halcyonic.systems/hayekian-systems/"'
```

## Conventions

- **Terminology**: Use McQuade's actual words from the relevant chapter, not synthesized abstractions
- **Diagrams**: Match book figures (rectangular boxes, not circles/diamonds)
- **Theory surfacing**: Native `<details>/<summary>` for collapse, SVG `<title>` for tooltips
- **Color schemes**: Each domain gets a distinct palette; new domains MUST define a palette
- **Theme**: All new UI must use CSS variables from `style.css`
- **Commits**: Conventional commits with Claude co-author attribution
- **Leptos**: 0.8 CSR mode, `features = ["csr"]`. Trunk for build/serve.

## Leptos Patterns

```rust
// Reactive signals
let (domain, set_domain) = signal(Domain::Abstract);
let domain_config = Memo::new(move |_| domain.get().config());
let state = RwSignal::new(SystemState::default());

// SVG with dynamic attributes
view! {
    <rect fill=color.to_css() stroke=palette.accent.to_css() />
}

// Mixed view types in Vec — use .into_any()
markers.push(view! { <marker ...>...</marker> }.into_any());

// Animation via request_animation_frame (recursive, not Effect loop)
fn request_animation_frame(state: RwSignal<SystemState>, running: ReadSignal<bool>) { ... }
```
