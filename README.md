# Hayekian Anticipatory Systems Explorer

An interactive tool for exploring how social institutions learn — or fail to learn — about the environments they operate in.

Built on the framework from McQuade & Butos, [*Hayekian Systems*](https://www.amazon.com/Hayekian-Systems-Structure-Interaction-Foundations-ebook/dp/B0BRYNMSKQ) (Routledge), which argues that markets, scientific communities, legislatures, and bureaucracies all share the same deep structure: an anticipatory loop that accumulates knowledge when it works and degrades when pieces break.

**Live demo:** [halcyonic.systems/hayekian-systems](https://halcyonic.systems/hayekian-systems/)

---

## The ALES Loop

Every social institution must model an environment it can't fully observe, act on that model, and update based on what happens. McQuade and Butos formalize this as the ALES loop:

```
Expectation  ──D──>  Selection
     ^                    |
     K                    P
     |                    v
  Learning   <──O,C──  Action
```

Knowledge sits at the center — both input and output. When the loop runs well, knowledge accumulates. When a link breaks (feedback gets distorted, innovation gets suppressed), knowledge degrades and the system stops adapting.

The structural claim is that this loop is invariant across domains. A market's price system and a scientific community's peer review process are doing the same thing at a structural level. Switch between domains in the app and watch the labels change while the dynamics stay the same.

## Two Modes

**Parameter Explorer** — four sliders control the structural health of the ALES loop: environmental coupling, innovation freedom, feedback fidelity, and process closure. Knowledge quality emerges from the dynamics — it's not a slider. Useful for building intuition about which structural features matter and how domain defaults (e.g., bureaucracy's hampered innovation) constrain the system's ceiling.

**Agent Simulation** — system-level knowledge emerges from individual interactions instead of equations. Each agent runs its own ALES micro-cycle: form expectations, evaluate transaction partners, transact (or don't), learn from the outcome. Agents differ in perception, creativity, and learning rate. The same parameter change that smoothly shifts a curve in Explorer mode produces messy, heterogeneous behavior in ABM mode.

## Domains

Seven instantiations from Chapters 5–8, each with McQuade's process labels:

| Domain | Source | Distinct Feature |
|--------|--------|-----------------|
| Abstract | Ch 5 | Generic template |
| Market | Ch 6 | Price structure as collective knowledge |
| Firm | Ch 6 | Internal model under management judgment |
| Free Banking | Ch 6 | Clearing knowledge and reserve dynamics |
| Science | Ch 7 | Strong empirical anchor, "probing" as output |
| Legislature | Ch 8 | Weak anchor, "political advertising" as output |
| Bureaucracy | Ch 8 | Innovation and judgment structurally hampered |

## Things to Try

- Set Process Closure to 0 and watch knowledge collapse — McQuade's central argument
- Compare Market (strong coupling) to Legislature (weak) — notice the knowledge ceiling difference
- Run a healthy system, then spike Env. Volatility — well-coupled systems recover, insulated ones don't
- In ABM mode, lower Process Closure gradually and watch agents drift from ground truth

## Known Limitations & Open Questions

The simulation captures the ALES loop structure but the agent dynamics need grounding in the theory. Several areas where domain expertise would sharpen the model:

- **Knowledge converges too fast and too completely.** Under healthy parameters, agents reach near-perfect knowledge quickly. Real institutional learning has friction, forgetting, and structural ceilings that vary by domain. The convergence dynamics need rethinking — possibly bounded rationality constraints, knowledge depreciation, or domain-specific carrying capacities.
- **Agent interaction model is underspecified.** Agents currently pair randomly and learn from transaction success/failure. McQuade's framework implies richer structure: different transaction types, varying agent capabilities within each ALES process, and institutional constraints on who can transact with whom.
- **Environment model is too simple.** Environmental state is a single scalar with optional noise. Real environments are multi-dimensional, partially observable, and shift in ways that differ across domains (resource shocks vs. preference drift vs. paradigm shifts).
- **Cross-domain structural differences need calibration.** Each domain has different default parameters, but these were set heuristically rather than derived from the theory's claims about *why* certain institutions learn better than others.

If any of this is interesting to you, issues and conversations are welcome.

## Running Locally

Requires [Rust](https://rustup.rs/) and [Trunk](https://trunkrs.dev/):

```
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve
```

Opens at `http://127.0.0.1:8080/`.

## Deployment

Pushes to `main` trigger GitHub Actions → Trunk WASM build → GitHub Pages.

## License

MIT
