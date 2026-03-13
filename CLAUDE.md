# Hayekian Anticipatory Systems Explorer

Interactive visualization of McQuade & Butos's anticipatory systems framework. Users control structural parameters of an abstract anticipatory system and see how the ALES loop behaves — when closure is complete and feedback is strong, knowledge accumulates; break a link and the system degrades.

## Quick Start

```bash
cargo run              # Launch the app (debug build)
cargo build --release  # Rebuild optimized binary (needed after code changes)
```

**Live web app**: https://halcyonic.systems/hayekian-systems/ (auto-deploys on push to main)
**Desktop launcher**: `~/Desktop/Hayekian Systems.app` — double-click or Dock it. After code changes, run `cargo build --release` to update.
**Repo**: https://github.com/halcyonic-systems/hayekian-systems

**Theme**: Light (cream) and dark (slate) modes with toggle button at top-left of controls panel. Default is light/cream. All UI colors (process boxes, text, flow labels, knowledge box, env indicators) adapt to the active theme via `light_mode` bool passed through the UI layer.

## Source Material

**Book**: McQuade & Butos, *Hayekian Systems* (Routledge)
**Chapter files**: `~/Desktop/halcyonic/operations/political-economy/hayekian-systems/chapters/`
**Parameter analysis**: `~/Desktop/halcyonic/operations/political-economy/hayekian-systems/implementation/mcquade-parameters.md`

Always read the relevant chapter before implementing. The plan's parameter names and layouts should be cross-checked against McQuade's actual figures and vocabulary.

## Architecture

```
src/
├── main.rs              # eframe app, domain selector, theory panel, light/dark toggle
├── core/
│   ├── mod.rs
│   ├── params.rs        # 4 structural parameters + derived dynamics
│   └── system.rs        # SystemState, ALES process enum, simulation step
├── domains/
│   ├── mod.rs           # DomainConfig, DomainPalette, abstract_system(), palette functions
│   ├── market.rs        # Market (Fig 6.1), Firm (Fig 6.2), Free Banking (Fig 6.3)
│   ├── science.rs       # Science (Fig 7.1)
│   └── government.rs    # Legislature (Fig 8.1), Bureaucracy (Fig 8.2)
└── ui/
    ├── mod.rs
    ├── loop_view.rs     # Loop diagram (rectangular boxes, palette-driven, light/dark aware)
    ├── controls.rs      # Parameter sliders with domain-specific labels/tooltips
    └── dashboard.rs     # System health metrics + knowledge sparkline (light/dark aware)
```

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

When implementing a new chapter's domain:

1. **Read the chapter** — use McQuade's exact process labels from the figure
2. **Create a palette function** in `domains/mod.rs` (e.g., `palette_science()`)
3. **Create a domain config function** in the appropriate `domains/*.rs` file
4. **Add the variant** to the `Domain` enum in `main.rs`
5. **Color scheme should be visually distinct** from existing domains — each domain should feel different at a glance

### DomainPalette Fields

| Field | Used For |
|-------|----------|
| `accent` | Process box borders, active UI elements |
| `accent_dim` | Process box fill (lerped with activation level) |
| `knowledge_accent` | Knowledge box border, percentage text, arrow |
| `flow_healthy` | Flow arrows when strength > 0.6 |
| `flow_warning` | Flow arrows when strength 0.3–0.6 |
| `flow_danger` | Flow arrows when strength < 0.3 |

### Planned Palettes for Future Chapters

| Domain | Chapter | Suggested Palette |
|--------|---------|-------------------|
| Big Player | Ch 9-10 | Orange/warning — intervention, disruption |

## Chapter Build Order

Each chapter = a working app on its own. Review gate between each.

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

Two modes toggled via button at top-left of controls panel:

| Mode | Panel Fill | Loop Background | Text | Box Fills |
|------|-----------|----------------|------|-----------|
| Light (default) | Cream `(245, 240, 230)` | Warm `(235, 228, 216)` | Dark `(40, 40, 50)` | White tinted toward palette accent |
| Dark | Slate `(58, 62, 74)` | Dark `(48, 52, 62)` | White | Dark, lerped from `accent_dim` |

**Implementation**: `light_mode: bool` on `HayekianApp`, passed to `loop_view::ales_loop()` and `dashboard::system_dashboard()`. Helper colors (`text_primary`, `text_secondary`, `boundary_color`, `env_label_color`) computed at top of `ales_loop()`. `node_color()`, `knowledge_color()` accept `light_mode` param.

**When adding new UI**: Always branch on `light_mode` for any hardcoded colors. Use semantic variables, not raw RGB in paint calls.

## Desktop App Launcher

Built via `osacompile` — AppleScript wrapper that launches the release binary directly (no Terminal window):
```bash
osacompile -o ~/Desktop/"Hayekian Systems.app" -e \
  'do shell script "/Users/home/Desktop/halcyonic-projects/active/hayekian-systems/target/release/hayekian-systems &> /dev/null &"'
```
Rebuild after code changes: `cargo build --release`

**Note**: `.command` files don't work reliably with oh-my-zsh Terminal resume. Use `.app` bundles instead.

## Conventions

- **Terminology**: Use McQuade's actual words from the relevant chapter, not synthesized abstractions
- **Diagrams**: Match book figures (rectangular boxes, not circles/diamonds)
- **Theory surfacing**: Static tooltips on hover + collapsible panel, not rotating text
- **Color schemes**: Each domain gets a distinct palette; new domains MUST define a palette
- **Theme**: All new UI must work in both light and dark modes
- **Commits**: Conventional commits with Claude co-author attribution
- **egui version**: 0.31 — note API differences from tutorials (StrokeKind, LayerId args)

## Key egui 0.31 Patterns

```rust
// rect_stroke needs StrokeKind as 4th arg
painter.rect_stroke(rect, radius, stroke, StrokeKind::Inside);

// show_tooltip_at_pointer needs LayerId as 2nd arg
egui::show_tooltip_at_pointer(ctx, LayerId::new(Order::Tooltip, id), widget_id, |ui| { ... });

// Color32 channel access for palette lerping
let r = color.r();  // not .r, it's a method in 0.31
```
