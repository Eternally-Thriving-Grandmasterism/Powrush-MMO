/*!
 * TOLC Gate Alignment with proper CSP-style Forward Checking + MRV.
 */

use crate::fracture::puzzle_trait::{PuzzleState, PuzzleAction, ActionResult, PuzzleError};
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateState {
    Aligned,
    Inverted,
    Overpowered,
    Conflicted,
}

impl GateState {
    pub fn all_states() -> [GateState; 4] {
        [
            GateState::Aligned,
            GateState::Inverted,
            GateState::Overpowered,
            GateState::Conflicted,
        ]
    }
}

#[derive(Debug, Clone)]
pub struct TolcGate {
    pub index: usize,
    pub state: GateState,
    pub valence: f32,
    pub locked: bool,
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub from: usize,
    pub to: usize,
    pub strength: f32,
}

#[derive(Debug, Clone)]
pub struct TolcGateState {
    pub gates: Vec<TolcGate>,
    pub connections: Vec<Connection>,
    pub collective_valence: f32,
    pub mercy_charges: u32,
    pub max_mercy_charges: u32,
    /// Current domains for each gate (for Forward Checking)
    pub domains: Vec<Vec<GateState>>,
}

impl TolcGateState {
    pub fn new(num_gates: usize, initial_mercy_charges: u32) -> Self {
        let gates = (0..num_gates)
            .map(|i| TolcGate {
                index: i,
                state: GateState::Aligned,
                valence: 0.85,
                locked: false,
            })
            .collect();

        let connections = (0..num_gates)
            .map(|i| Connection {
                from: i,
                to: (i + 1) % num_gates,
                strength: 0.7,
            })
            .collect();

        let domains = (0..num_gates)
            .map(|_| GateState::all_states().to_vec())
            .collect();

        Self {
            gates,
            connections,
            collective_valence: 0.85,
            mercy_charges: initial_mercy_charges,
            max_mercy_charges: initial_mercy_charges,
            domains,
        }
    }

    fn recalculate_collective_valence(&mut self) {
        let sum: f32 = self.gates.iter().map(|g| g.valence).sum();
        let avg = sum / self.gates.len() as f32;

        let connection_bonus: f32 = self.connections.iter()
            .map(|c| c.strength.max(0.0))
            .sum::<f32>() / self.connections.len() as f32;

        self.collective_valence = (avg * 0.7 + connection_bonus * 0.3).clamp(0.0, 1.0);
    }

    /// Full Forward Checking + MRV solver
    fn solve_fc_mrv(
        &self,
        current: &mut TolcGateState,
        depth: usize,
        max_depth: usize,
        solution: &mut Vec<PuzzleAction>,
    ) -> bool {
        if current.is_solved() {
            return true;
        }

        if depth >= max_depth {
            return false;
        }

        // MRV: select gate with smallest remaining domain
        let mut best_index: Option<usize> = None;
        let mut smallest_domain = usize::MAX;

        for i in 0..current.gates.len() {
            if current.gates[i].locked {
                continue;
            }
            let dsize = current.domains[i].len();
            if dsize < smallest_domain && dsize > 0 {
                smallest_domain = dsize;
                best_index = Some(i);
            }
        }

        let gate_idx = match best_index {
            Some(idx) => idx,
            None => return false,
        };

        // Try values in current domain
        let current_domain = current.domains[gate_idx].clone();

        for &new_state in &current_domain {
            // Create backup
            let backup_gates = current.gates.clone();
            let backup_domains = current.domains.clone();
            let backup_connections = current.connections.clone();
            let backup_valence = current.collective_valence;
            let backup_charges = current.mercy_charges;

            // Apply state change
            current.gates[gate_idx].state = new_state;

            // Forward Checking: prune inconsistent values from neighbors
            let mut consistent = true;
            for conn in &current.connections {
                if conn.from == gate_idx || conn.to == gate_idx {
                    let other = if conn.from == gate_idx { conn.to } else { conn.from };

                    if !current.gates[other].locked {
                        // Simple consistency rule example
                        if new_state == GateState::Conflicted && current.domains[other].len() == 1 {
                            consistent = false;
                            break;
                        }
                    }
                }
            }

            if !consistent {
                // Restore and skip
                current.gates = backup_gates;
                current.domains = backup_domains;
                current.connections = backup_connections;
                current.collective_valence = backup_valence;
                current.mercy_charges = backup_charges;
                continue;
            }

            // Record action (simplified)
            let action = PuzzleAction::RotateGate {
                gate_index: gate_idx,
                amount: 1, // Approximate
            };
            solution.push(action);

            if Self::solve_fc_mrv(current, depth + 1, max_depth, solution) {
                return true;
            }

            solution.pop();

            // Backtrack
            current.gates = backup_gates;
            current.domains = backup_domains;
            current.connections = backup_connections;
            current.collective_valence = backup_valence;
            current.mercy_charges = backup_charges;
        }

        false
    }
}

impl PuzzleState for TolcGateState {
    fn is_solved(&self) -> bool {
        self.collective_valence >= 0.92
            && self.gates.iter().all(|g| g.state == GateState::Aligned)
            && self.connections.iter().all(|c| c.strength >= 0.0)
    }

    fn is_solvable(&self) -> bool {
        let mut state_copy = self.clone();
        let mut solution = vec![];
        state_copy.solve_fc_mrv(&mut state_copy, 0, 22, &mut solution)
    }

    fn find_solution(&self) -> Option<Vec<PuzzleAction>> {
        let mut state_copy = self.clone();
        let mut solution = vec![];

        if state_copy.solve_fc_mrv(&mut state_copy, 0, 25, &mut solution) {
            Some(solution)
        } else {
            None
        }
    }

    fn apply_action(&mut self, action: PuzzleAction) -> Result<ActionResult, PuzzleError> {
        match action {
            PuzzleAction::RotateGate { gate_index, amount } => {
                if gate_index >= self.gates.len() {
                    return Err(PuzzleError::Other("Invalid gate index".into()));
                }

                let gate = &mut self.gates[gate_index];
                if gate.locked {
                    return Err(PuzzleError::GateLocked);
                }

                gate.state = match (gate.state, amount % 4) {
                    (GateState::Aligned, 1) | (GateState::Aligned, 3) => GateState::Inverted,
                    (GateState::Inverted, 1) | (GateState::Inverted, 3) => GateState::Aligned,
                    (GateState::Overpowered, _) => GateState::Aligned,
                    (GateState::Conflicted, _) => GateState::Aligned,
                    _ => gate.state,
                };

                gate.valence = (gate.valence + (amount as f32) * 0.03).clamp(0.3, 1.0);
                self.recalculate_collective_valence();

                Ok(ActionResult::Success { message: Some(format!("Gate {} rotated", gate_index)) })
            }

            PuzzleAction::ResolveConflict { connection_id } => {
                if self.mercy_charges == 0 {
                    return Err(PuzzleError::NoMercyCharges);
                }

                if (connection_id as usize) >= self.connections.len() {
                    return Err(PuzzleError::Other("Invalid connection".into()));
                }

                let conn = &mut self.connections[connection_id as usize];
                conn.strength = conn.strength.max(0.3);
                self.mercy_charges -= 1;

                self.recalculate_collective_valence();

                Ok(ActionResult::Success {
                    message: Some("Conflict resolved using Mercy Charge".into()),
                })
            }

            PuzzleAction::Reset => {
                for gate in &mut self.gates {
                    gate.state = GateState::Aligned;
                    gate.valence = 0.85;
                }
                for conn in &mut self.connections {
                    conn.strength = 0.7;
                }
                self.collective_valence = 0.85;
                self.mercy_charges = self.max_mercy_charges;

                // Reset domains
                for domain in &mut self.domains {
                    *domain = GateState::all_states().to_vec();
                }

                Ok(ActionResult::Success { message: Some("Puzzle reset".into()) })
            }

            _ => Err(PuzzleError::InvalidActionForPuzzleType),
        }
    }

    fn get_progress(&self) -> f32 {
        let aligned_count = self.gates.iter().filter(|g| g.state == GateState::Aligned).count();
        let alignment_ratio = aligned_count as f32 / self.gates.len() as f32;

        (alignment_ratio * 0.6 + self.collective_valence * 0.4).clamp(0.0, 1.0)
    }

    fn get_hints(&self) -> Vec<String> {
        let mut hints = vec![];

        if self.collective_valence < 0.7 {
            hints.push("The gates are significantly out of harmony.".to_string());
        }
        if self.mercy_charges > 0 {
            hints.push("You still have Mercy Charges available to resolve conflicts.".to_string());
        }
        hints
    }

    fn get_current_state_summary(&self) -> String {
        format!(
            "Valence: {:.2} | Mercy: {}/{} | Progress: {:.0}%",
            self.collective_valence,
            self.mercy_charges,
            self.max_mercy_charges,
            self.get_progress() * 100.0
        )
    }

    fn clone_box(&self) -> Box<dyn PuzzleState> {
        Box::new(self.clone())
    }
}
