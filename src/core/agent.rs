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
            beliefs: clamp01(0.5 + rng.next_normal() * 0.2), // start with vague beliefs
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
        rng: &mut Rng,
    ) -> Option<bool> {
        // --- E phase: Form expectation ---
        // Perceived signal = ground_truth filtered by perception (noisy observation)
        let perceived_signal = ground_truth + (1.0 - self.perception) * rng.next_normal() * 0.3;
        // Expectation = beliefs (60%) + perceived signal (40%) + creativity noise
        let expectation = self.beliefs * 0.6
            + perceived_signal * 0.4
            + self.creativity * rng.next_normal() * 0.1;
        self.last_expectation = expectation;

        // --- S phase: Evaluate opportunity ---
        // Perceived opportunity depends on how well agent senses the environment.
        // Skip if opportunity seems too low (perception-dependent threshold).
        let perceived_opportunity = self.perception * partner_accuracy;
        let threshold = 0.2 + (1.0 - self.perception) * 0.3; // better perception → lower threshold
        if perceived_opportunity < threshold {
            self.transacted_this_step = false;
            self.last_partner = None;
            return None;
        }

        // --- A phase: Bilateral exchange ---
        self.transacted_this_step = true;
        self.last_partner = Some(partner_id);

        // Success probability = joint belief accuracy (both need good models)
        let my_accuracy = self.belief_accuracy(ground_truth);
        let joint_accuracy = (my_accuracy * partner_accuracy).sqrt();
        let success = rng.next_f32() < joint_accuracy;

        if success {
            // Both gain wealth proportional to individual accuracy
            self.wealth += my_accuracy * 0.05;
            // Clean signal from successful transaction: ground truth revealed
            let signal = ground_truth;
            // --- L phase: Update beliefs toward clean signal ---
            self.beliefs += self.learning_rate * (signal - self.beliefs) * 0.3;
        } else {
            // Small wealth loss on failure
            self.wealth = (self.wealth - 0.02).max(0.01);
            // Noisy signal from failed transaction
            let noisy_signal = ground_truth + rng.next_normal() * 0.4;
            // --- L phase: Update beliefs toward noisy signal (slower) ---
            self.beliefs += self.learning_rate * (noisy_signal - self.beliefs) * 0.1;
        }

        self.beliefs = self.beliefs.clamp(0.0, 1.0);

        // Record accuracy history
        let acc = self.belief_accuracy(ground_truth);
        self.belief_history.push(acc);
        if self.belief_history.len() > 60 {
            self.belief_history.remove(0);
        }

        Some(success)
    }

    /// Passive learning — agent observes environment without transacting.
    /// Small belief update toward perceived signal.
    pub fn passive_observe(&mut self, ground_truth: f32, rng: &mut Rng) {
        let perceived = ground_truth + (1.0 - self.perception) * rng.next_normal() * 0.4;
        self.beliefs += self.learning_rate * (perceived - self.beliefs) * 0.05;
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
