# Hayekian Anticipatory Systems Explorer

An interactive tool for exploring how social institutions learn — or fail to learn — about the environments they operate in.

Built on the theoretical framework from McQuade & Butos, [*Hayekian Systems*](https://www.amazon.com/Hayekian-Systems-Structure-Interaction-Foundations-ebook/dp/B0BRYNMSKQ) (Routledge), which argues that markets, firms, scientific communities, legislatures, and bureaucracies all share the same deep structure: an anticipatory loop that accumulates knowledge when it works and degrades when pieces break.

**Try it:** [halcyonic.systems/hayekian-systems](https://halcyonic.systems/hayekian-systems/)

---

## The Core Idea

Every social institution faces the same problem: it must model an environment it can't fully observe, act on that model, and update its understanding based on what happens. McQuade and Butos formalize this as the **ALES loop** — four processes that feed into each other:

```
Expectation  ──D──>  Selection
     ^                    |
     K                    P
     |                    v
  Learning   <──O,C──  Action
```

- **Expectation** generates possible futures from what the system currently knows
- **Selection** evaluates those possibilities and commits to a plan
- **Action** executes the plan, producing output and consequences
- **Learning** updates the system's model based on what actually happened

**Knowledge** sits at the center — it's both an input to every process and the output of the whole cycle. When the loop runs well, knowledge accumulates. When a link breaks (feedback gets distorted, innovation gets suppressed, the system loses contact with reality), knowledge degrades. The system stops anticipating and starts reacting — or worse, stops adapting entirely.

The surprising claim is that this structure is *invariant* across domains. A market's price system and a scientific community's peer review process are doing the same thing at the structural level. The app makes this visible: switch between domains and watch the labels change while the dynamics stay the same.

## Two Modes

### Parameter Explorer

The default mode. Four sliders control the structural health of the ALES loop:

| Parameter | What It Controls |
|-----------|-----------------|
| **Environmental Coupling** | How firmly the system is anchored to external reality |
| **Innovation Freedom** | How unconstrained the expectation process is |
| **Feedback Fidelity** | How accurately consequences feed back into learning |
| **Process Closure** | Whether the four processes form a self-maintaining loop |

Knowledge quality is *not* a slider. It emerges from loop dynamics — rise when the loop is healthy, decay when it's broken. A fifth slider (**Env. Volatility**) introduces environmental shocks to test resilience.

This mode is useful for building intuition about *which structural features matter* and seeing how domain-specific defaults (e.g., bureaucracy's hampered innovation) constrain the system's ceiling.

### Agent Simulation

Click **Agent Simulation** to switch from the equation-driven explorer to an agent-based model where system-level knowledge is no longer computed — it *emerges* from individual interactions.

Each agent runs its own ALES micro-cycle every step:
1. Forms an **expectation** based on its beliefs, what it can perceive of the environment, and creative noise
2. **Evaluates** whether a transaction with its partner looks worthwhile
3. **Transacts** (or doesn't) — success depends on how accurately both agents model reality
4. **Learns** from the outcome — successful transactions reveal clearer signals than failures

Agents differ in perception, creativity, and learning rate (sampled from distributions centered on the slider values). The scatter plot shows each agent's beliefs vs. perception, with a dashed line marking ground truth. When the loop is healthy, agents cluster near truth. Lower process closure and watch them scatter — they can't find transaction partners, so they can't learn.

The sliders now control *distributions*, not equations. The same parameter change that smoothly shifts a curve in Explorer mode produces messy, heterogeneous, emergent behavior in ABM mode. That's the point.

## Domains

Seven instantiations from Chapters 5–8, each with McQuade's exact process labels and a distinct color palette:

| Domain | Source | What Makes It Distinct |
|--------|--------|----------------------|
| Abstract | Ch 5, Fig 5.2 | The template — generic process names |
| Market | Ch 6, Fig 6.1 | Price structure as collective knowledge |
| Firm | Ch 6, Fig 6.2 | Internal model under management judgment |
| Free Banking | Ch 6, Fig 6.3 | Clearing knowledge and reserve dynamics |
| Science | Ch 7, Fig 7.1 | "Probing" as output, strong empirical anchor |
| Legislature | Ch 8, Fig 8.1 | "Political Advertising" as output, weak anchor |
| Bureaucracy | Ch 8, Fig 8.2 | Innovation and Judgment structurally hampered |

Switching domains relabels every process, flow, and variable while preserving your parameter state. The cross-domain invariance becomes immediately visible.

## Things to Try

- **Break the loop**: Set Process Closure to 0 and watch knowledge collapse. This is McQuade's central argument — without closure, no learning.
- **Compare anchors**: Switch between Market (strong environmental coupling) and Legislature (weak). Notice the knowledge ceiling difference.
- **Stress test**: Run a healthy system, then spike Env. Volatility. Well-coupled systems recover; insulated ones don't.
- **ABM mode**: Lower Process Closure gradually and watch the agent scatter plot. Agents near ground truth drift away as they lose transaction partners.
- **Domain defaults**: Reset after switching domains. Bureaucracy starts with hampered processes — you can see the structural disadvantage in the initial knowledge trajectory.

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
  main.rs                Leptos app, SimMode enum, animation loop
  style.css              CSS variables for light/dark theming
  core/
    params.rs            Four structural parameters + derived dynamics
    system.rs            SystemState, equation-driven simulation step
    agent.rs             Agent struct, ALES micro-cycle, B=f(P,E) decomposition
    abm.rs               ABM state, environment, pairing, aggregate projection
    rng.rs               Xorshift64 PRNG (no rand dependency, keeps WASM small)
  domains/
    mod.rs               DomainConfig, DomainPalette, abstract template
    market.rs            Market, Firm, Free Banking (Ch 6)
    science.rs           Science (Ch 7)
    government.rs        Legislature, Bureaucracy (Ch 8)
  components/
    ales_loop.rs         SVG loop diagram (process boxes, flows, knowledge indicator)
    controls.rs          Parameter sliders, domain selector, mode toggle
    dashboard.rs         Knowledge chart with trend indicators
    agent_canvas.rs      Agent scatter plot (beliefs × perception, ABM mode)
    theory_panel.rs      Collapsible process descriptions from source text
```

The simulation engine (`core/`) and domain data (`domains/`) have no UI dependencies. All rendering happens in `components/` via [Leptos](https://leptos.dev/) 0.8 CSR with inline SVG.

The ABM layer (`agent.rs`, `abm.rs`) projects population aggregates into the same `SystemState` struct the Explorer uses, so the ALES diagram and knowledge chart render identically in both modes. No component changes were needed to add agent simulation — just a new data source.

## Deployment

Pushes to `main` trigger a GitHub Actions workflow that builds WASM via Trunk and deploys to GitHub Pages at [halcyonic.systems/hayekian-systems](https://halcyonic.systems/hayekian-systems/).

## License

MIT
