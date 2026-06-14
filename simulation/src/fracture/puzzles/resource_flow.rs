/*!
 * Resource Flow Balancing puzzle implementation.
 */

use crate::fracture::puzzle_trait::{PuzzleState, PuzzleAction, ActionResult, PuzzleError};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct ResourceNode {
    pub id: u32,
    pub production: f32,
    pub consumption: f32,
    pub storage: f32,
    pub max_storage: f32,
    pub is_corrupted: bool,
}

#[derive(Debug, Clone)]
pub struct ResourceConnection {
    pub from: u32,
    pub to: u32,
    pub flow_rate: f32,
    pub capacity: f32,
}

#[derive(Debug, Clone)]
pub struct ResourceFlowState {
    pub nodes: Vec<ResourceNode>,
    pub connections: Vec<ResourceConnection>,
    pub system_stability: f32,
    pub mercy_score: f32,
    pub abundance_level: f32,
}

impl ResourceFlowState {
    pub fn new(num_nodes: usize) -> Self {
        let nodes = (0..num_nodes)
            .map(|i| ResourceNode {
                id: i as u32,
                production: 10.0,
                consumption: 8.0,
                storage: 20.0,
                max_storage: 50.0,
                is_corrupted: false,
            })
            .collect();

        let connections = (0..num_nodes - 1)
            .map(|i| ResourceConnection {
                from: i as u32,
                to: (i + 1) as u32,
                flow_rate: 5.0,
                capacity: 15.0,
            })
            .collect();

        Self {
            nodes,
            connections,
            system_stability: 0.75,
            mercy_score: 0.8,
            abundance_level: 0.7,
        }
    }

    fn recalculate_stability(&mut self) {
        let mut total_imbalance = 0.0;

        for node in &self.nodes {
            let net = node.production - node.consumption;
            let storage_ratio = if node.max_storage > 0.0 {
                node.storage / node.max_storage
            } else {
                0.5
            };

            total_imbalance += (net.abs() * 0.5 + (1.0 - storage_ratio).abs() * 0.5);
        }

        let avg_imbalance = total_imbalance / self.nodes.len() as f32;
        self.system_stability = (1.0 - avg_imbalance.clamp(0.0, 1.0)).max(0.1);

        self.mercy_score = (self.system_stability * 0.8 + 0.2).clamp(0.3, 1.0);
        self.abundance_level = (self.system_stability * 0.7 + 0.3).clamp(0.2, 1.0);
    }
}

impl PuzzleState for ResourceFlowState {
    fn is_solved(&self) -> bool {
        self.system_stability >= 0.92
            && self.mercy_score >= 0.75
            && self.abundance_level >= 0.65
            && self.nodes.iter().all(|n| !n.is_corrupted && n.storage >= 0.0)
    }

    fn is_solvable(&self) -> bool {
        // Heuristic: if more than half the nodes are corrupted and stability is very low, it may be unsolvable.
        let corrupted_count = self.nodes.iter().filter(|n| n.is_corrupted).count();

        if corrupted_count as f32 > (self.nodes.len() as f32 / 2.0) && self.system_stability < 0.3 {
            return false;
        }

        true
    }

    fn apply_action(&mut self, action: PuzzleAction) -> Result<ActionResult, PuzzleError> {
        match action {
            PuzzleAction::AdjustFlow { connection_id, delta } => {
                if (connection_id as usize) >= self.connections.len() {
                    return Err(PuzzleError::Other("Invalid connection".into()));
                }

                let conn = &mut self.connections[connection_id as usize];
                conn.flow_rate = (conn.flow_rate + delta).clamp(0.0, conn.capacity);

                self.recalculate_stability();

                Ok(ActionResult::Success {
                    message: Some(format!("Adjusted flow on connection {}", connection_id)),
                })
            }

            PuzzleAction::Reset => {
                for node in &mut self.nodes {
                    node.storage = 20.0;
                    node.is_corrupted = false;
                }
                for conn in &mut self.connections {
                    conn.flow_rate = 5.0;
                }
                self.system_stability = 0.75;
                self.mercy_score = 0.8;
                self.abundance_level = 0.7;

                Ok(ActionResult::Success { message: Some("Resource network reset".into()) })
            }

            _ => Err(PuzzleError::InvalidActionForPuzzleType),
        }
    }

    fn get_progress(&self) -> f32 {
        let stability_weight = self.system_stability * 0.5;
        let mercy_weight = self.mercy_score * 0.3;
        let abundance_weight = self.abundance_level * 0.2;

        (stability_weight + mercy_weight + abundance_weight).clamp(0.0, 1.0)
    }

    fn get_hints(&self) -> Vec<String> {
        let mut hints = vec![];

        if self.system_stability < 0.6 {
            hints.push("The resource network is highly unstable.".to_string());
        }
        if self.mercy_score < 0.6 {
            hints.push("Mercy principles are being violated in the current flow.".to_string());
        }
        hints
    }

    fn get_current_state_summary(&self) -> String {
        format!(
            "Stability: {:.0}% | Mercy: {:.0}% | Abundance: {:.0}%",
            self.system_stability * 100.0,
            self.mercy_score * 100.0,
            self.abundance_level * 100.0
        )
    }

    fn clone_box(&self) -> Box<dyn PuzzleState> {
        Box::new(self.clone())
    }
}
