mod core;
mod domains;
mod components;

use leptos::prelude::*;
use crate::core::system::SystemState;
use crate::domains::DomainConfig;
use crate::components::{ales_loop::AlesLoop, controls::ParameterControls, dashboard::KnowledgePanel, theory_panel::TheoryPanel};

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

    // Animation loop using request_animation_frame
    let anim_running = running;
    let anim_state = state;
    Effect::new(move |_| {
        if anim_running.get() {
            request_animation_frame(anim_state, anim_running);
        }
    });

    let theme_class = move || if light_mode.get() { "app-root" } else { "app-root dark" };
    let theme_label = move || if light_mode.get() { "\u{2600} Light" } else { "\u{263D} Dark" };

    view! {
        <div class=theme_class>
            <div class="sidebar">
                <button class="theme-toggle" on:click=move |_| set_light_mode.update(|v| *v = !*v)>
                    {theme_label}
                </button>
                <hr class="section-sep" />

                <ParameterControls
                    state=state
                    running=running
                    set_running=set_running
                    domain=domain
                    set_domain=set_domain
                    domain_config=domain_config
                />
            </div>
            <div class="main-content">
                <div class="diagram-area">
                    <AlesLoop state=state domain_config=domain_config light_mode=light_mode />
                </div>
                <div class="chart-area">
                    <KnowledgePanel state=state domain_config=domain_config />
                </div>
                <div class="footer-area">
                    <TheoryPanel domain_config=domain_config />
                    <div class="citation">
                        "Based on McQuade & Butos, "
                        <em>"Anticipatory Systems in a Hayekian Framework"</em>
                        " (Routledge)"
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Schedule the next animation frame: step the simulation and request another frame if still running.
fn request_animation_frame(state: RwSignal<SystemState>, running: ReadSignal<bool>) {
    use wasm_bindgen::prelude::*;

    let window = web_sys::window().expect("no window");
    let closure = Closure::once(move || {
        if running.get_untracked() {
            state.update(|s| s.step(0.1));
            // Schedule next frame
            request_animation_frame(state, running);
        }
    });
    window
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .expect("failed to request animation frame");
    closure.forget();
}
