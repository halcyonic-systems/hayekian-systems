# Hayekian Anticipatory Systems Explorer

Interactive visualization of the anticipatory systems framework from McQuade & Butos, *Anticipatory Systems in a Hayekian Framework* (Routledge). Users manipulate four structural parameters of an ALES loop and observe how knowledge accumulates or degrades across seven domain instantiations.

**Live demo:** [halcyonic.systems/hayekian-systems](https://halcyonic.systems/hayekian-systems/)

---

## What It Does

The app renders a single simulation engine — the Expectation-Selection-Action-Learning loop from Chapter 5 — and lets users switch between seven domain configurations drawn from Chapters 5 through 8. Switching domains relabels every process, flow, and knowledge variable while preserving parameter state, so the cross-domain structural invariance becomes immediately apparent.

### Domains

| Domain | Source | Key Feature |
|--------|--------|-------------|
| Abstract | Ch 5, Fig 5.2 | Template ALES loop |
| Market | Ch 6, Fig 6.1 | Price structure as knowledge |
| Firm | Ch 6, Fig 6.2 | Internal model, management judgment |
| Free Banking | Ch 6, Fig 6.3 | Clearing knowledge, reserve dynamics |
| Science | Ch 7, Fig 7.1 | "Probing" output, empirical anchor |
| Legislature | Ch 8, Fig 8.1 | "Political Advertising" output, weak anchor |
| Bureaucracy | Ch 8, Fig 8.2 | Innovation and Judgment hampered (dashed borders) |

### Parameters

The four sliders correspond directly to the structural conditions McQuade identifies:

- **Environmental Coupling** (Ch 6+: "Anchor to Environment") — how firmly the system is tied to external reality it cannot manipulate
- **Innovation Freedom** — how unconstrained the expectation/proposal process is
- **Feedback Fidelity** — how accurately consequences feed back into learning
- **Process Closure** — whether E, S, A, L form a self-maintaining closed loop (Piaget/Rosen)

Knowledge quality is not a parameter. It is the emergent outcome of loop dynamics.

## Running Locally

Requires [Rust](https://rustup.rs/) and [Trunk](https://trunkrs.dev/):

```
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve
```

Opens at `http://127.0.0.1:8080/`.

## Architecture

```
src/
  main.rs              Leptos app, domain enum, animation loop
  style.css            CSS variables for light/dark theming
  core/
    params.rs          Four structural parameters + derived dynamics
    system.rs          SystemState, simulation step function
  domains/
    mod.rs             DomainConfig, DomainPalette, abstract template
    market.rs          Market, Firm, Free Banking (Ch 6)
    science.rs         Science (Ch 7)
    government.rs      Legislature, Bureaucracy (Ch 8)
  components/
    ales_loop.rs       SVG loop diagram
    controls.rs        Parameter sliders, domain selector
    dashboard.rs       Knowledge chart with metric indicators
    theory_panel.rs    Collapsible process descriptions
```

The simulation engine (`core/`) and domain data (`domains/`) have no UI dependencies. All rendering happens in `components/` via Leptos 0.8 CSR with inline SVG.

## Deployment

Pushes to `main` trigger a GitHub Actions workflow that builds WASM via Trunk and deploys to GitHub Pages. No manual steps required.

## License

MIT
