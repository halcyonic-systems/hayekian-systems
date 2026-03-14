use leptos::prelude::*;
use crate::Domain;
use crate::core::system::SystemState;
use crate::domains::DomainConfig;

#[component]
pub fn ParameterControls(
    state: RwSignal<SystemState>,
    running: ReadSignal<bool>,
    set_running: WriteSignal<bool>,
    domain: ReadSignal<Domain>,
    set_domain: WriteSignal<Domain>,
    domain_config: Memo<DomainConfig>,
    speed: ReadSignal<f32>,
    set_speed: WriteSignal<f32>,
) -> impl IntoView {
    view! {
        <h3>"Domain"</h3>
        <div class="domain-list">
            {Domain::ALL.map(|d| {
                let is_selected = move || domain.get() == d;
                view! {
                    <label>
                        <input
                            type="radio"
                            name="domain"
                            checked=is_selected
                            on:change=move |_| {
                                set_domain.set(d);
                                let cfg = d.config();
                                state.update(|s| {
                                    s.params = cfg.default_params
                                        .unwrap_or_default();
                                    s.reset();
                                });
                            }
                        />
                        {d.label()}
                    </label>
                }
            }).collect_view()}
        </div>

        <div class="domain-info">
            <span class="name">{move || domain_config.get().name}</span>
            {move || {
                let cfg = domain_config.get();
                format!("{} \u{2014} {}", cfg.chapter, cfg.figure)
            }}
        </div>

        <hr class="section-sep" />

        <h3>"Structural Parameters"</h3>
        <div class="param-group">
            <ParamSlider
                label=Signal::derive(move || domain_config.get().coupling_label.to_string())
                tooltip=Signal::derive(move || domain_config.get().coupling_tooltip.to_string())
                value=Signal::derive(move || state.get().params.environmental_coupling)
                set_value=move |v| state.update(|s| s.params.environmental_coupling = v)
            />
            <ParamSlider
                label=Signal::derive(|| "Innovation Freedom".to_string())
                tooltip=Signal::derive(|| "How unconstrained the expectation/proposal process is. \"Development of possible future scenarios based on existing knowledge.\" E(K,I) \u{2192} D \u{2014} Ch 5, Fig 5.2".to_string())
                value=Signal::derive(move || state.get().params.innovation_freedom)
                set_value=move |v| state.update(|s| s.params.innovation_freedom = v)
            />
            <ParamSlider
                label=Signal::derive(|| "Feedback Fidelity".to_string())
                tooltip=Signal::derive(|| "How accurately consequences flow back to update knowledge. \"Updating of the internal model ... based on sensory input and the results of action.\" L(C,O,K,I) \u{2192} K \u{2014} Ch 5, Fig 5.2".to_string())
                value=Signal::derive(move || state.get().params.feedback_fidelity)
                set_value=move |v| state.update(|s| s.params.feedback_fidelity = v)
            />
            <ParamSlider
                label=Signal::derive(|| "Process Closure".to_string())
                tooltip=Signal::derive(|| "Whether each process's conditions are provided by companion processes. Piaget: \"a closed cycle ... characteristic of the organism\" where processes \"reconstitute each other and thus maintain the operation of the system.\" \u{2014} Ch 5".to_string())
                value=Signal::derive(move || state.get().params.process_closure)
                set_value=move |v| state.update(|s| s.params.process_closure = v)
            />
            <ParamSlider
                label=Signal::derive(|| "Env. Volatility".to_string())
                tooltip=Signal::derive(|| "How unpredictable the environment is. At 0 the environment is perfectly stable; at 1 it delivers frequent shocks that can degrade knowledge quality.".to_string())
                value=Signal::derive(move || state.get().params.env_volatility)
                set_value=move |v| state.update(|s| s.params.env_volatility = v)
            />
        </div>

        <hr class="section-sep" />

        <h3>"Simulation"</h3>
        <div class="param-group">
            <div class="param-slider">
                <label title="Simulation speed — controls time step per frame. Low values show gradual dynamics.">
                    "Speed"
                    <span class="value">{move || format!("{:.2}", speed.get())}</span>
                </label>
                <input
                    type="range"
                    min="1"
                    max="100"
                    prop:value=move || (speed.get() * 100.0) as i32
                    on:input=move |ev| {
                        use wasm_bindgen::JsCast;
                        let target = ev.target().unwrap();
                        let input = target.unchecked_into::<web_sys::HtmlInputElement>();
                        let v: f32 = input.value().parse().unwrap_or(5.0);
                        set_speed.set(v / 100.0);
                    }
                />
            </div>
        </div>

        <div class="controls-buttons">
            <button on:click=move |_| set_running.update(|v| *v = !*v)>
                {move || if running.get() { "\u{23F8} Pause" } else { "\u{25B6} Run" }}
            </button>
            <button on:click=move |_| {
                let dt = speed.get();
                state.update(|s| s.step(dt));
            }>
                "\u{23ED} Step"
            </button>
            <button on:click=move |_| {
                let cfg = domain.get().config();
                state.update(|s| {
                    s.params = cfg.default_params
                        .unwrap_or_default();
                    s.reset();
                });
                set_running.set(false);
            }>
                "\u{21BA} Reset"
            </button>
        </div>
    }
}

#[component]
fn ParamSlider(
    label: Signal<String>,
    tooltip: Signal<String>,
    value: Signal<f32>,
    set_value: impl Fn(f32) + 'static + Copy,
) -> impl IntoView {
    view! {
        <div class="param-slider">
            <label title=tooltip>
                {label}
                <span class="value">{move || format!("{:.0}%", value.get() * 100.0)}</span>
            </label>
            <input
                type="range"
                min="0"
                max="100"
                prop:value=move || (value.get() * 100.0) as i32
                on:input=move |ev| {
                    use wasm_bindgen::JsCast;
                    let target = ev.target().unwrap();
                    let input = target.unchecked_into::<web_sys::HtmlInputElement>();
                    let v: f32 = input.value().parse().unwrap_or(80.0);
                    set_value(v / 100.0);
                }
            />
        </div>
    }
}
