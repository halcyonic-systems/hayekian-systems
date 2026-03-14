use super::agent::Agent;
use super::params::StructuralParams;
use super::rng::Rng;
use super::system::SystemState;

/// The shared environment all agents interact with.
#[derive(Clone)]
pub struct Environment {
    /// Actual state of the world agents try to model (drifts via random walk).
    pub ground_truth: f32,
    /// How quickly ground truth drifts (from env_volatility slider).
    pub volatility: f32,
}

/// Aggregate history tracked per step for visualization.
#[derive(Clone)]
pub struct AbmHistory {
    /// Mean belief accuracy across all agents → maps to knowledge_quality.
    pub mean_accuracy: Vec<f32>,
    /// Standard deviation of belief accuracy (population diversity).
    pub accuracy_spread: Vec<f32>,
    /// Fraction of agents that transacted this step.
    pub transaction_rate: Vec<f32>,
    /// Mean agent wealth.
    pub mean_wealth: Vec<f32>,
    /// Gini coefficient of wealth distribution.
    pub wealth_gini: Vec<f32>,
    /// Ground truth trajectory (so user can see what agents chase).
    pub ground_truth: Vec<f32>,
}

impl AbmHistory {
    fn new() -> Self {
        Self {
            mean_accuracy: Vec::new(),
            accuracy_spread: Vec::new(),
            transaction_rate: Vec::new(),
            mean_wealth: Vec::new(),
            wealth_gini: Vec::new(),
            ground_truth: Vec::new(),
        }
    }

    fn trim(&mut self, max_len: usize) {
        let trim = |v: &mut Vec<f32>| {
            if v.len() > max_len {
                v.drain(..v.len() - max_len);
            }
        };
        trim(&mut self.mean_accuracy);
        trim(&mut self.accuracy_spread);
        trim(&mut self.transaction_rate);
        trim(&mut self.mean_wealth);
        trim(&mut self.wealth_gini);
        trim(&mut self.ground_truth);
    }
}

/// The full ABM state — agents + environment + history.
#[derive(Clone)]
pub struct AbmState {
    pub agents: Vec<Agent>,
    pub env: Environment,
    pub rng: Rng,
    pub time: f32,
    pub history: AbmHistory,
    /// From process_closure: probability that any given pair can interact.
    pub interaction_probability: f32,
}

impl AbmState {
    /// Initialize N agents from structural parameter distributions.
    pub fn new(n: u16, params: &StructuralParams, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let agents = (0..n)
            .map(|i| {
                Agent::sample(
                    i,
                    params.environmental_coupling,
                    params.innovation_freedom,
                    params.feedback_fidelity,
                    &mut rng,
                )
            })
            .collect();

        // Ground truth starts at a random position — agents must discover it.
        let ground_truth = rng.next_f32() * 0.6 + 0.2; // [0.2, 0.8]

        Self {
            agents,
            env: Environment {
                ground_truth,
                volatility: params.env_volatility,
            },
            rng,
            time: 0.0,
            history: AbmHistory::new(),
            interaction_probability: params.process_closure,
        }
    }

    /// Core ABM step — advance one time unit.
    pub fn step(&mut self, params: &StructuralParams) {
        self.time += 1.0;
        let n = self.agents.len();
        if n < 2 {
            return;
        }

        // 1. Environment drifts (random walk scaled by volatility)
        self.env.volatility = params.env_volatility;
        let drift = self.rng.next_normal() * self.env.volatility * 0.02;
        self.env.ground_truth = (self.env.ground_truth + drift).clamp(0.05, 0.95);
        let gt = self.env.ground_truth;

        // 2. Update interaction probability from process_closure
        self.interaction_probability = params.process_closure;

        // 3. Fisher-Yates shuffle for pairing order (avoid positional bias)
        let mut order: Vec<usize> = (0..n).collect();
        for i in (1..n).rev() {
            let j = (self.rng.next_f32() * (i + 1) as f32) as usize;
            let j = j.min(i); // safety clamp
            order.swap(i, j);
        }

        // 4. Pair agents; closure gate determines which pairs connect
        let mut transacted_count = 0u32;

        // Process pairs from the shuffled order
        let mut i = 0;
        while i + 1 < n {
            let idx_a = order[i];
            let idx_b = order[i + 1];
            i += 2;

            // Closure gate: can this pair find each other?
            let closure = self.interaction_probability;
            if self.rng.next_f32() > closure {
                // Pair blocked — both get passive observation (gated by closure)
                let (lo, hi) = if idx_a < idx_b {
                    (idx_a, idx_b)
                } else {
                    (idx_b, idx_a)
                };
                let (left, right) = self.agents.split_at_mut(hi);
                left[lo].passive_observe(gt, closure, &mut self.rng);
                right[0].passive_observe(gt, closure, &mut self.rng);
                continue;
            }

            // 5. Connected pair runs ALES micro-cycle (split_at_mut for dual &mut)
            let (lo, hi) = if idx_a < idx_b {
                (idx_a, idx_b)
            } else {
                (idx_b, idx_a)
            };
            let (left, right) = self.agents.split_at_mut(hi);
            let agent_a = &mut left[lo];
            let agent_b = &mut right[0];

            let acc_a = agent_a.belief_accuracy(gt);
            let acc_b = agent_b.belief_accuracy(gt);
            let id_a = agent_a.id;
            let id_b = agent_b.id;
            let fidelity = params.feedback_fidelity;

            let result_a = agent_a.ales_step(gt, acc_b, id_b, fidelity, &mut self.rng);
            let result_b = agent_b.ales_step(gt, acc_a, id_a, fidelity, &mut self.rng);

            if result_a.is_some() {
                transacted_count += 1;
            }
            if result_b.is_some() {
                transacted_count += 1;
            }
        }

        // 6. Unpaired agent (odd count) gets passive observation
        if n % 2 == 1 {
            let last_idx = order[n - 1];
            let closure = self.interaction_probability;
            self.agents[last_idx].passive_observe(gt, closure, &mut self.rng);
        }

        // 7. Collect aggregates into history
        let accuracies: Vec<f32> = self.agents.iter().map(|a| a.belief_accuracy(gt)).collect();
        let mean_acc = accuracies.iter().sum::<f32>() / n as f32;
        let variance =
            accuracies.iter().map(|a| (a - mean_acc).powi(2)).sum::<f32>() / n as f32;
        let spread = variance.sqrt();

        let tx_rate = transacted_count as f32 / n as f32;

        let wealths: Vec<f32> = self.agents.iter().map(|a| a.wealth).collect();
        let mean_wealth = wealths.iter().sum::<f32>() / n as f32;
        let gini = gini_coefficient(&wealths);

        self.history.mean_accuracy.push(mean_acc);
        self.history.accuracy_spread.push(spread);
        self.history.transaction_rate.push(tx_rate);
        self.history.mean_wealth.push(mean_wealth);
        self.history.wealth_gini.push(gini);
        self.history.ground_truth.push(gt);
        self.history.trim(300);
    }

    /// Project ABM aggregates into SystemState so existing components render unchanged.
    pub fn to_system_state(&self, params: &StructuralParams) -> SystemState {
        let n = self.agents.len() as f32;
        let gt = self.env.ground_truth;

        let mean_acc = if n > 0.0 {
            self.agents.iter().map(|a| a.belief_accuracy(gt)).sum::<f32>() / n
        } else {
            0.0
        };

        let mean_creativity = self.agents.iter().map(|a| a.creativity).sum::<f32>() / n.max(1.0);
        let mean_perception = self.agents.iter().map(|a| a.perception).sum::<f32>() / n.max(1.0);
        let mean_lr = self.agents.iter().map(|a| a.learning_rate).sum::<f32>() / n.max(1.0);

        let tx_rate = self.history.transaction_rate.last().copied().unwrap_or(0.0);
        let closure = self.interaction_probability;

        // Process activations — emergent from agent population
        let process_activation = [
            mean_creativity * mean_acc,     // E: Expectation modulated by knowledge
            mean_perception * mean_acc,     // S: Selection driven by sensing
            tx_rate,                        // A: Action = actual exchanges
            mean_lr * tx_rate.max(0.1),     // L: Learning requires transaction signal
        ];

        // Flow strengths — all gated by closure
        let flow_strengths = [
            closure * mean_creativity,                  // E→S
            closure * mean_perception,                  // S→A
            closure * mean_lr,                          // A→L
            closure * mean_perception * mean_lr,        // L→E
        ];

        SystemState {
            params: params.clone(),
            knowledge_quality: mean_acc,
            time: self.time,
            knowledge_history: self.history.mean_accuracy.clone(),
            process_activation,
            flow_strengths,
            rng: Rng::new(42), // unused — ABM has its own rng
        }
    }
}

/// Compute Gini coefficient for a slice of non-negative values.
fn gini_coefficient(values: &[f32]) -> f32 {
    let n = values.len();
    if n == 0 {
        return 0.0;
    }
    let mean = values.iter().sum::<f32>() / n as f32;
    if mean < 1e-10 {
        return 0.0;
    }
    let mut sum_diff = 0.0_f32;
    for i in 0..n {
        for j in 0..n {
            sum_diff += (values[i] - values[j]).abs();
        }
    }
    sum_diff / (2.0 * n as f32 * n as f32 * mean)
}
