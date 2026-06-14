/*!
 * TOLC Gate Alignment with optimized AC-3 + bitmask domains.
 */

use crate::fracture::puzzle_trait::{PuzzleState, PuzzleAction, ActionResult, PuzzleError};
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateState {
    Aligned = 0,
    Inverted = 1,
    Overpowered = 2,
    Conflicted = 3,
}

impl GateState {
    pub fn all_states() -> [GateState; 4] {
        [GateState::Aligned, GateState::Inverted, GateState::Overpowered, GateState::Conflicted]
    }

    pub fn to_bit(self) -> u8 {
        1 << (self as u8)
    }

    pub fn from_bit(bit: u8) -> Option<GateState> {
        match bit {
            1 => Some(GateState::Aligned),
            2 => Some(GateState::Inverted),
            4 => Some(GateState::Overpowered),
            8 => Some(GateState::Conflicted),
            _ => None,
        }
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
    /// Bitmask domains for performance (bit 0=Aligned, 1=Inverted, 2=Overpowered, 3=Conflicted)
    pub domains: Vec<u8>,
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

        // Start with all 4 states possible
        let domains = vec![0b1111; num_gates];

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

    /// Enforce Arc Consistency using bitmasks
    fn enforce_arc_consistency(&mut self) -> bool {
        let mut queue: Vec<(usize, usize)> = vec![];

        for conn in &self.connections {
            queue.push((conn.from, conn.to));
            queue.push((conn.to, conn.from));
        }

        while let Some((xi, xj)) = queue.pop() {
            if self.revise(xi, xj) {
                if self.domains[xi] == 0 {
                    return false;
                }
                for conn in &self.connections {
                    if conn.to == xi && conn.from != xj {
                        queue.push((conn.from, xi));
                    }
                    if conn.from == xi && conn.to != xj {
                        queue.push((conn.to, xi));
                    }
                }
            }
        }

        true
    }

    fn revise(&mut self, xi: usize, xj: usize) -> bool {
        let mut revised = false;
        let mut new_domain = 0u8;

        let xj_domain = self.domains[xj];

        for bit in 0..4 {
            let mask = 1 << bit;
            if (self.domains[xi] & mask) != 0 {
                let vi = GateState::from_bit(mask).unwrap();
                let mut supported = false;

                for jbit in 0..4 {
                    let jmask = 1 << jbit;
                    if (xj_domain & jmask) != 0 {
                        let vj = GateState::from_bit(jmask).unwrap();
                        if self.is_consistent(vi, vj, xi, xj) {
                            supported = true;
                            break;
                        }
                    }
                }

                if supported {
                    new_domain |= mask;
                } else {
                    revised = true;
                }
            }
        }

        if revised {
            self.domains[xi] = new_domain;
        }

        revised
    }

    fn is_consistent(&self, vi: GateState, vj: GateState, xi: usize, xj: usize) -> bool {
        if vi == GateState::Conflicted && vj == GateState::Conflicted {
            if self.gates[xi].locked && self.gates[xj].locked && self.mercy_charges == 0 {
                return false;
            }
        }
        true
    }

    fn solve_ac3_fc_mrv(
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

        if !current.enforce_arc_consistency() {
            return false;
        }

        // MRV
        let mut best_idx: Option<usize> = None;
        let mut min_size = usize::MAX;

        for i in 0..current.gates.len() {
            if current.gates[i].locked {
                continue;
            }
            let dsize = current.domains[i].count_ones() as usize;
            if dsize > 0 && dsize < min_size {
                min_size = dsize;
                best_idx = Some(i);
            }
        }

        let var = match best_idx {
            Some(idx) => idx,
            None => return false,
        };

        let current_domain = current.domains[var];

        for bit in 0..4 {
            let mask = 1 << bit;
            if (current_domain & mask) == 0 {
                continue;
            }

            let value = GateState::from_bit(mask).unwrap();
            let backup = current.clone();

            current.gates[var].state = value;

            if current.enforce_arc_consistency() {
                let action = PuzzleAction::RotateGate {
                    gate_index: var,
                    amount: 1,
                };
                solution.push(action);

                if Self::solve_ac3_fc_mrv(current, depth + 1, max_depth, solution) {
                    return true;
                }

                solution.pop();
            }

            *current = backup;
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
        state_copy.solve_ac3_fc_mrv(&mut state_copy, 0, 25, &mut solution)
    }

    fn find_solution(&self) -> Option<Vec<PuzzleAction>> {
        let mut state_copy = self.clone();
        let mut solution = vec![];

        if state_copy.solve_ac3_fc_mrv(&mut state_copy, 0, 28, &mut solution) {
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

                for d in &mut self.domains {
                    *d = 0b1111;
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
