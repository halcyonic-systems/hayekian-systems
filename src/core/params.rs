/// The four structural parameters of any anticipatory system.
/// Each maps directly to McQuade & Butos's theoretical claims.
///
/// Note: In Ch 5, these are abstract parameters of the anticipatory template.
/// When instantiated to specific domains (Ch 6-8), McQuade introduces the
/// concept of "anchor to environment" — how firmly the system is tied to
/// external reality it cannot manipulate. That domain-specific language
/// replaces `environmental_coupling` in later chapters.
#[derive(Clone, Debug, PartialEq)]
pub struct StructuralParams {
    /// How strongly the system's processes are coupled to environmental input.
    /// Strong coupling = system receives and responds to external feedback.
    /// Weak coupling = system becomes insulated from its environment.
    /// (In Ch 6-8, McQuade calls this the system's "anchor to environment".)
    pub environmental_coupling: f32,

    /// How unconstrained the expectation/proposal process is.
    /// Free = dispositions (D) are generated freely from knowledge and input.
    /// Suppressed = expectation process is blocked or centrally directed.
    pub innovation_freedom: f32,

    /// How accurately results flow back to update knowledge.
    /// Clear = consequences (C) feed cleanly into learning.
    /// Distorted = signals are noisy, delayed, or politically filtered.
    pub feedback_fidelity: f32,

    /// Whether each process's conditions are provided by companion processes.
    /// Complete = E→S→A→L form a self-maintaining closed loop.
    /// Broken = one or more links severed; the loop cannot self-maintain.
    /// (Piaget's closure concept, formalized by Rosen as closure to efficient causation.)
    pub process_closure: f32,

    /// How unpredictable the environment is. 0 = perfectly stable,
    /// 1 = highly volatile. Introduces periodic noise shocks to knowledge.
    pub env_volatility: f32,
}

impl Default for StructuralParams {
    fn default() -> Self {
        Self {
            environmental_coupling: 0.8,
            innovation_freedom: 0.8,
            feedback_fidelity: 0.8,
            process_closure: 1.0,
            env_volatility: 0.0,
        }
    }
}

impl StructuralParams {
    /// Overall system health: how well the ALES loop can function.
    /// This is the *emergent* quality — not a slider, but a consequence.
    pub fn system_vitality(&self) -> f32 {
        // Process closure is the sine qua non — if it's broken, everything degrades.
        // The other three contribute multiplicatively: each is necessary but not sufficient.
        let closure_weight = self.process_closure.powf(2.0); // nonlinear: partial closure degrades fast
        let operational = (self.environmental_coupling * self.innovation_freedom * self.feedback_fidelity).powf(0.33);
        closure_weight * operational
    }

    /// Rate at which knowledge accumulates (or decays).
    /// Positive = system is learning. Negative = system is losing anticipatory capacity.
    pub fn knowledge_rate(&self) -> f32 {
        let base_rate = self.feedback_fidelity * self.environmental_coupling;
        let innovation_boost = self.innovation_freedom * 0.5;
        let closure_gate = if self.process_closure < 0.3 { -0.5 } else { self.process_closure };
        (base_rate + innovation_boost - 0.5) * closure_gate
    }

    /// Asymptotic knowledge ceiling — bounded rationality means no system
    /// achieves perfect knowledge. Derived from structural params so that
    /// well-functioning systems (Market, Science) cap ~0.80-0.85 while
    /// degraded systems (Legislature, Bureaucracy) cap much lower.
    pub fn knowledge_cap(&self) -> f32 {
        let raw = (self.environmental_coupling * self.feedback_fidelity * self.process_closure).sqrt();
        // Scale into [0.05, 0.90] — even perfect params can't reach 1.0
        (raw * 0.85 + 0.05).clamp(0.05, 0.90)
    }

    /// How well the system anticipates its environment.
    /// High = predictions match reality. Low = the system is surprised constantly.
    pub fn anticipation_accuracy(&self) -> f32 {
        let signal_quality = self.environmental_coupling * self.feedback_fidelity;
        let learning_capacity = self.innovation_freedom * self.process_closure;
        (signal_quality * 0.6 + learning_capacity * 0.4).clamp(0.0, 1.0)
    }

    /// Whether the system is adapting to reality or to something else (e.g., a Big Player).
    /// 1.0 = fully reality-anchored. 0.0 = fully detached.
    pub fn reality_orientation(&self) -> f32 {
        (self.environmental_coupling * self.feedback_fidelity).clamp(0.0, 1.0)
    }
}
