use super::params::StructuralParams;

/// The state of a running anticipatory system.
/// Knowledge quality is the key emergent variable — it rises or falls
/// based on whether the loop is intact and well-coupled to its environment.
#[derive(Clone, Debug)]
pub struct SystemState {
    pub params: StructuralParams,
    /// Accumulated knowledge quality (0.0 = empty, 1.0 = perfect classification).
    /// This is NOT a slider — it's the emergent outcome of the loop running.
    pub knowledge_quality: f32,
    /// Simulation time (arbitrary units).
    pub time: f32,
    /// History of knowledge quality for plotting.
    pub knowledge_history: Vec<f32>,
    /// Per-process activation levels in loop order: [E, S, A, L].
    pub process_activation: [f32; 4],
    /// Flow strengths between processes in loop order: [E→S, S→A, A→L, L→E].
    pub flow_strengths: [f32; 4],
}

/// The four processes of the anticipatory loop, in McQuade's Figure 5.2 order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlesProcess {
    Expectation,
    Selection,
    Action,
    Learning,
}

impl AlesProcess {
    pub const ALL: [AlesProcess; 4] = [
        AlesProcess::Expectation,
        AlesProcess::Selection,
        AlesProcess::Action,
        AlesProcess::Learning,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            AlesProcess::Expectation => "Expectation",
            AlesProcess::Selection => "Selection",
            AlesProcess::Action => "Action",
            AlesProcess::Learning => "Learning",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            AlesProcess::Expectation => "Development of possible future scenarios based on existing knowledge. E(K,I) → D",
            AlesProcess::Selection => "Evaluation of possible responses based on current situation. S(D,K,I) → P",
            AlesProcess::Action => "Implementation of plans for action. A(P,K,I) → O,C",
            AlesProcess::Learning => "Updating of the internal model based on sensory input and the results of action. L(C,O,K,I) → K",
        }
    }
}

impl Default for SystemState {
    fn default() -> Self {
        Self {
            params: StructuralParams::default(),
            knowledge_quality: 0.3, // start with some baseline knowledge
            time: 0.0,
            knowledge_history: vec![0.3],
            process_activation: [0.5; 4],
            flow_strengths: [1.0; 4],
        }
    }
}

impl SystemState {
    pub fn new(params: StructuralParams) -> Self {
        Self {
            params,
            ..Default::default()
        }
    }

    /// Advance the simulation by one time step.
    /// The core dynamic: structural parameters determine how the loop runs,
    /// which determines whether knowledge accumulates or decays.
    ///
    /// Process order: E → S → A → L → (back to E via Knowledge)
    /// Flow indices: [0: E→S, 1: S→A, 2: A→L, 3: L→E]
    pub fn step(&mut self, dt: f32) {
        self.time += dt;

        // Update flow strengths based on structural parameters.
        // Each flow depends on process closure (all flows degrade together)
        // plus the specific parameter most relevant to that link.
        let closure = self.params.process_closure;

        // E→S: Dispositions flow to Selection. Depends on innovation freedom
        // (how freely Expectation generates dispositions).
        self.flow_strengths[0] = closure * self.params.innovation_freedom;
        // S→A: Plans flow to Action. Depends on environmental coupling
        // (selection is informed by environmental input).
        self.flow_strengths[1] = closure * self.params.environmental_coupling;
        // A→L: Output/Consequences flow to Learning. Depends on feedback fidelity
        // (how accurately results feed back).
        self.flow_strengths[2] = closure * self.params.feedback_fidelity;
        // L→E: Updated Knowledge feeds back to Expectation. Depends on both
        // environmental coupling and feedback fidelity (learning from reality).
        self.flow_strengths[3] = closure * self.params.environmental_coupling * self.params.feedback_fidelity;

        // Update process activations — each process is driven by its incoming flow.
        // With some inertia so changes aren't instantaneous.
        let inertia = 0.9_f32;
        for i in 0..4 {
            let incoming_flow = self.flow_strengths[(i + 3) % 4]; // flow from predecessor
            let target = incoming_flow * self.knowledge_quality.max(0.1); // knowledge modulates everything
            self.process_activation[i] =
                self.process_activation[i] * inertia + target * (1.0 - inertia);
        }

        // Knowledge dynamics: the key emergent variable.
        let rate = self.params.knowledge_rate();
        let knowledge_delta = rate * dt * 0.3; // scaled for reasonable dynamics

        // Knowledge has inertia and bounds.
        self.knowledge_quality = (self.knowledge_quality + knowledge_delta).clamp(0.0, 1.0);

        // Record history (keep last 300 points).
        self.knowledge_history.push(self.knowledge_quality);
        if self.knowledge_history.len() > 300 {
            self.knowledge_history.remove(0);
        }
    }

    /// Reset to initial conditions with current parameters.
    pub fn reset(&mut self) {
        self.knowledge_quality = 0.3;
        self.time = 0.0;
        self.knowledge_history = vec![0.3];
        self.process_activation = [0.5; 4];
        self.flow_strengths = [1.0; 4];
    }
}
