use super::rng::Rng;

/// An individual agent in the ABM — runs its own ALES micro-cycle.
///
/// B = f(P, E): Behavior is a function of Person attributes and Environment.
/// P attributes are sampled from Normal(slider_mean, 0.1) at initialization.
#[derive(Clone)]
pub struct Agent {
    pub id: u16,
    // P attributes (sampled from parameter distributions)
    /// Agent's internal model of the environment — individual knowledge.
    pub beliefs: f32,
    /// How freely the agent generates expectations (maps to innovation_freedom).
    pub creativity: f32,
    /// How well the agent senses environmental signals (maps to environmental_coupling).
    pub perception: f32,
    /// How quickly the agent updates beliefs from outcomes (maps to feedback_fidelity).
    pub learning_rate: f32,
    // Emergent state
    /// Accumulated wealth from transactions (starts at 1.0).
    pub wealth: f32,
    /// Recent belief accuracy values (last 60 steps).
    pub belief_history: Vec<f32>,
    // ALES intermediates (for visualization)
    pub last_expectation: f32,
    pub transacted_this_step: bool,
    pub last_partner: Option<u16>,
}

impl Agent {
    /// Sample a new agent with P attributes drawn from Normal(mean, 0.1).
    pub fn sample(id: u16, coupling: f32, innovation: f32, fidelity: f32, rng: &mut Rng) -> Self {
        let clamp01 = |v: f32| v.clamp(0.05, 0.95);
        Self {
            id,
            beliefs: clamp01(rng.next_f32() * 0.8 + 0.1), // uniform ignorance — no prior
            creativity: clamp01(innovation + rng.next_normal() * 0.1),
            perception: clamp01(coupling + rng.next_normal() * 0.1),
            learning_rate: clamp01(fidelity + rng.next_normal() * 0.1),
            wealth: 1.0,
            belief_history: Vec::with_capacity(60),
            last_expectation: 0.5,
            transacted_this_step: false,
            last_partner: None,
        }
    }

    /// How accurately this agent's beliefs model the ground truth.
    /// 1.0 = perfect, 0.0 = maximally wrong.
    pub fn belief_accuracy(&self, ground_truth: f32) -> f32 {
        1.0 - (self.beliefs - ground_truth).abs().min(1.0)
    }

    /// Run the full ALES micro-cycle with a partner.
    /// Returns Some(true) if transaction succeeded, Some(false) if failed, None if skipped.
    ///
    /// Follows Ch 5 Fig 5.2 functional signatures:
    /// - E(K,I) → D: Expectation from beliefs + perceived signal + creativity noise
    /// - S(D,K,I) → P: Evaluate opportunity, decide whether to transact
    /// - A(P,K,I) → O,C: Bilateral transaction, produces outcome + consequences
    /// - L(C,O,K,I) → K: Update beliefs from transaction signal
    pub fn ales_step(
        &mut self,
        ground_truth: f32,
        partner_accuracy: f32,
        partner_id: u16,
        feedback_fidelity: f32,
        rng: &mut Rng,
    ) -> Option<bool> {
        // --- E phase: Form expectation ---
        // Perceived signal = ground_truth heavily filtered by perception.
        // Low perception = agent mostly sees its own beliefs reflected back.
        let env_weight = self.perception * 0.5; // at best, 50% env signal
        let perceived_signal = ground_truth * env_weight
            + self.beliefs * (1.0 - env_weight)
            + (1.0 - self.perception) * rng.next_normal() * 0.3;
        // Expectation blends beliefs with perceived signal + creativity noise.
        // Creativity generates hypotheses — useful with feedback, dangerous without.
        let expectation = self.beliefs * 0.6
            + perceived_signal * 0.4
            + self.creativity * rng.next_normal() * 0.1;
        self.last_expectation = expectation;

        // --- S phase: Evaluate opportunity ---
        let perceived_opportunity = self.perception * partner_accuracy;
        let threshold = 0.2 + (1.0 - self.perception) * 0.3;
        if perceived_opportunity < threshold {
            // Creativity still causes belief drift even when not transacting —
            // untested hypotheses accumulate as noise.
            self.beliefs += self.creativity * rng.next_normal() * 0.015;
            self.beliefs = self.beliefs.clamp(0.0, 1.0);
            self.transacted_this_step = false;
            self.last_partner = None;
            return None;
        }

        // --- A phase: Bilateral exchange ---
        self.transacted_this_step = true;
        self.last_partner = Some(partner_id);

        let my_accuracy = self.belief_accuracy(ground_truth);
        let joint_accuracy = (my_accuracy * partner_accuracy).sqrt();
        let success = rng.next_f32() < joint_accuracy;

        // --- L phase: Update beliefs from transaction outcome ---
        // feedback_fidelity controls SIGNAL QUALITY, not just speed.
        // Low fidelity = the signal is systematically distorted.
        if success {
            self.wealth += my_accuracy * 0.05;
            // Signal quality depends on feedback fidelity: how much of the true
            // outcome reaches the agent vs. noise/distortion.
            let signal = ground_truth * feedback_fidelity
                + self.beliefs * (1.0 - feedback_fidelity)
                + (1.0 - feedback_fidelity) * rng.next_normal() * 0.2;
            self.beliefs += self.learning_rate * (signal - self.beliefs) * 0.25;
        } else {
            self.wealth = (self.wealth - 0.02).max(0.01);
            // Failed transactions produce misleading signals — biased away
            // from truth, not just noisy around it. The worse the fidelity,
            // the more the agent learns the wrong lesson.
            let bias = (1.0 - feedback_fidelity) * rng.next_normal() * 0.5;
            let signal = ground_truth * feedback_fidelity * 0.5
                + self.beliefs * (1.0 - feedback_fidelity * 0.5)
                + bias;
            self.beliefs += self.learning_rate * (signal - self.beliefs) * 0.08;
        }

        // Creativity-driven drift: even during transactions, the agent's
        // hypothesis generation perturbs beliefs slightly. Good feedback
        // corrects this; bad feedback lets it accumulate.
        self.beliefs += self.creativity * rng.next_normal() * 0.008;
        self.beliefs = self.beliefs.clamp(0.0, 1.0);

        let acc = self.belief_accuracy(ground_truth);
        self.belief_history.push(acc);
        if self.belief_history.len() > 60 {
            self.belief_history.remove(0);
        }

        Some(success)
    }

    /// Passive observation — no transaction partner available.
    /// Without the full ALES loop, learning is severely limited.
    /// `closure` gates whether companion processes even support observation.
    pub fn passive_observe(&mut self, ground_truth: f32, closure: f32, rng: &mut Rng) {
        // Without closure, companion processes can't support observation —
        // the agent is structurally cut off, not just missing a partner.
        let effective_perception = self.perception * closure;

        if effective_perception > 0.1 {
            // Faint signal, mostly the agent's own beliefs reflected back
            let signal = ground_truth * effective_perception * 0.3
                + self.beliefs * (1.0 - effective_perception * 0.3)
                + (1.0 - effective_perception) * rng.next_normal() * 0.3;
            self.beliefs += self.learning_rate * (signal - self.beliefs) * 0.02;
        }

        // Creativity causes belief drift — hypotheses generated without
        // any testing mechanism. The less closure, the more drift dominates.
        let drift_scale = self.creativity * (1.0 - closure * 0.5);
        self.beliefs += drift_scale * rng.next_normal() * 0.02;
        self.beliefs = self.beliefs.clamp(0.0, 1.0);

        self.transacted_this_step = false;
        self.last_partner = None;

        let acc = self.belief_accuracy(ground_truth);
        self.belief_history.push(acc);
        if self.belief_history.len() > 60 {
            self.belief_history.remove(0);
        }
    }
}
