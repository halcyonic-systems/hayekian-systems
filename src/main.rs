mod core;
mod domains;
mod components;

use leptos::prelude::*;
use crate::core::abm::AbmState;
use crate::core::system::SystemState;
use crate::domains::DomainConfig;
use crate::components::{
    agent_canvas::AgentCanvas,
    ales_loop::AlesLoop,
    controls::ParameterControls,
    dashboard::KnowledgePanel,
    theory_panel::TheoryPanel,
};

/// Available domain configurations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Domain {
    Abstract,
    Market,
    Firm,
    FreeBanking,
    Science,
    Legislature,
    Bureaucracy,
}

impl Domain {
    pub const ALL: [Domain; 7] = [
        Domain::Abstract,
        Domain::Market,
        Domain::Firm,
        Domain::FreeBanking,
        Domain::Science,
        Domain::Legislature,
        Domain::Bureaucracy,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Domain::Abstract => "Abstract (Ch 5)",
            Domain::Market => "Market (Ch 6, Fig 6.1)",
            Domain::Firm => "Firm (Ch 6, Fig 6.2)",
            Domain::FreeBanking => "Free Banking (Ch 6, Fig 6.3)",
            Domain::Science => "Science (Ch 7, Fig 7.1)",
            Domain::Legislature => "Legislature (Ch 8, Fig 8.1)",
            Domain::Bureaucracy => "Bureaucracy (Ch 8, Fig 8.2)",
        }
    }

    pub fn config(&self) -> DomainConfig {
        match self {
            Domain::Abstract => domains::abstract_system(),
            Domain::Market => domains::market::market_system(),
            Domain::Firm => domains::market::firm_system(),
            Domain::FreeBanking => domains::market::free_banking_system(),
            Domain::Science => domains::science::science_system(),
            Domain::Legislature => domains::government::legislature_system(),
            Domain::Bureaucracy => domains::government::bureaucracy_system(),
        }
    }

    pub fn key(&self) -> &'static str {
        match self {
            Domain::Abstract => "abstract",
            Domain::Market => "market",
            Domain::Firm => "firm",
            Domain::FreeBanking => "freebanking",
            Domain::Science => "science",
            Domain::Legislature => "legislature",
            Domain::Bureaucracy => "bureaucracy",
        }
    }
}

/// Simulation mode — parameter explorer or agent-based model.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SimMode {
    Explorer,
    Abm,
}

fn main() {
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (domain, set_domain) = signal(Domain::Abstract);
    let domain_config = Memo::new(move |_| domain.get().config());
    let state = RwSignal::new(SystemState::default());
    let (running, set_running) = signal(false);
    let (light_mode, set_light_mode) = signal(true);
    let (speed, set_speed) = signal(0.05_f32);
    let (sim_mode, set_sim_mode) = signal(SimMode::Explorer);
    let (abm_state, set_abm_state) = signal::<Option<AbmState>>(None);
    let (agent_count, set_agent_count) = signal(30u16);

    // Animation loop using request_animation_frame
    let anim_running = running;
    let anim_state = state;
    let anim_speed = speed;
    let anim_mode = sim_mode;
    let anim_abm = set_abm_state;
    Effect::new(move |_| {
        if anim_running.get() {
            request_animation_frame(anim_state, anim_running, anim_speed, anim_mode, anim_abm);
        }
    });

    let theme_class = move || if light_mode.get() { "app-root" } else { "app-root dark" };
    let theme_label = move || if light_mode.get() { "\u{2600} Light" } else { "\u{263D} Dark" };

    let is_abm = move || sim_mode.get() == SimMode::Abm;

    view! {
        <div class=theme_class>
            <div class="sidebar">
                <button class="theme-toggle" on:click=move |_| set_light_mode.update(|v| *v = !*v)>
                    {theme_label}
                </button>

                <div class="mode-toggle">
                    <button
                        class=move || if sim_mode.get() == SimMode::Explorer { "mode-btn active" } else { "mode-btn" }
                        on:click=move |_| {
                            set_sim_mode.set(SimMode::Explorer);
                            set_abm_state.set(None);
                            state.update(|s| {
                                let cfg = domain.get_untracked().config();
                                s.params = cfg.default_params.unwrap_or_default();
                                s.reset();
                            });
                            set_running.set(false);
                        }
                    >
                        "Explorer"
                    </button>
                    <button
                        class=move || if sim_mode.get() == SimMode::Abm { "mode-btn active" } else { "mode-btn" }
                        on:click=move |_| {
                            set_sim_mode.set(SimMode::Abm);
                            let s = state.get_untracked();
                            let abm = AbmState::new(agent_count.get_untracked(), &s.params, 42);
                            let projected = abm.to_system_state(&s.params);
                            state.set(projected);
                            set_abm_state.set(Some(abm));
                            set_running.set(false);
                        }
                    >
                        "Agent Simulation"
                    </button>
                </div>

                <hr class="section-sep" />

                <ParameterControls
                    state=state
                    running=running
                    set_running=set_running
                    domain=domain
                    set_domain=set_domain
                    domain_config=domain_config
                    speed=speed
                    set_speed=set_speed
                    sim_mode=sim_mode
                    agent_count=agent_count
                    set_agent_count=set_agent_count
                    set_abm_state=set_abm_state
                />
            </div>
            <div class="main-content">
                <div class="diagram-area">
                    <AlesLoop state=state domain_config=domain_config light_mode=light_mode />
                </div>
                {move || {
                    if is_abm() {
                        view! {
                            <div class="agent-area">
                                <AgentCanvas abm=abm_state.into() domain_config=domain_config />
                            </div>
                        }.into_any()
                    } else {
                        view! { <div /> }.into_any()
                    }
                }}
                <div class="chart-area">
                    <KnowledgePanel state=state domain_config=domain_config />
                </div>
                <div class="footer-area">
                    <TheoryPanel domain_config=domain_config />
                    <div class="citation">
                        "Based on McQuade & Butos, "
                        <a
                            href="https://www.amazon.com/Hayekian-Systems-Structure-Interaction-Foundations-ebook/dp/B0BRYNMSKQ"
                            target="_blank"
                            rel="noopener"
                        >
                            <em>"Hayekian Systems: Research into the Structure of Social Interaction"</em>
                        </a>
                        " (Routledge)"
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Schedule the next animation frame.
/// In Explorer mode: step SystemState directly.
/// In ABM mode: step AbmState, then project into SystemState.
fn request_animation_frame(
    state: RwSignal<SystemState>,
    running: ReadSignal<bool>,
    speed: ReadSignal<f32>,
    sim_mode: ReadSignal<SimMode>,
    set_abm: WriteSignal<Option<AbmState>>,
) {
    use wasm_bindgen::prelude::*;

    let window = web_sys::window().expect("no window");
    let closure = Closure::once(move || {
        if running.get_untracked() {
            match sim_mode.get_untracked() {
                SimMode::Explorer => {
                    let dt = speed.get_untracked();
                    state.update(|s| s.step(dt));
                }
                SimMode::Abm => {
                    set_abm.update(|maybe_abm| {
                        if let Some(ref mut abm) = maybe_abm {
                            let params = state.get_untracked().params;
                            abm.step(&params);
                            let projected = abm.to_system_state(&params);
                            state.set(projected);
                        }
                    });
                }
            }
            request_animation_frame(state, running, speed, sim_mode, set_abm);
        }
    });
    window
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .expect("failed to request animation frame");
    closure.forget();
}
