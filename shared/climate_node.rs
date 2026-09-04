//! shared/climate_node.rs
//! First-hour climate node + satchel + allocate.
//! Hands only: WASD / E / I / H / R. No server. No currency.
//! AG-SML — Autonomicity Games Inc. | info@Rathor.ai

use serde::{Deserialize, Serialize};

/// Visible node states a human can learn without a manifesto.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeState {
    Idle,
    Glowing,
    Tended,
    Resting,
    Stressed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AllocKind {
    Flow,
    Reserve,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TendResult {
    Taken { item: ClimateTake },
    NoTake { reason: &'static str },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClimateTake {
    pub node_id: u32,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateNode {
    pub id: u32,
    pub state: NodeState,
    /// 0.0 restored … 1.0 exhausted. First hour only needs this one number.
    pub fatigue: f32,
}

impl ClimateNode {
    pub fn glowing(id: u32) -> Self {
        Self {
            id,
            state: NodeState::Glowing,
            fatigue: 0.0,
        }
    }

    /// E on this node.
    pub fn tend(&mut self) -> TendResult {
        match self.state {
            NodeState::Glowing => {
                self.fatigue = (self.fatigue + 0.25).clamp(0.0, 1.0);
                self.state = if self.fatigue >= 0.75 {
                    NodeState::Resting
                } else {
                    NodeState::Tended
                };
                TendResult::Taken {
                    item: ClimateTake {
                        node_id: self.id,
                        kind: "tend".to_string(),
                    },
                }
            }
            NodeState::Tended => {
                self.fatigue = (self.fatigue + 0.35).clamp(0.0, 1.0);
                self.state = if self.fatigue >= 0.75 {
                    NodeState::Stressed
                } else {
                    NodeState::Resting
                };
                TendResult::NoTake {
                    reason: "node is tired — rest it or allocate flow",
                }
            }
            NodeState::Resting | NodeState::Stressed => TendResult::NoTake {
                reason: "node is resting",
            },
            NodeState::Idle => TendResult::NoTake {
                reason: "no glow",
            },
        }
    }

    /// Slow world tick. Flow allocation should call restore() instead of waiting.
    pub fn tick(&mut self) {
        match self.state {
            NodeState::Tended => {
                self.fatigue = (self.fatigue - 0.05).clamp(0.0, 1.0);
                if self.fatigue <= 0.15 {
                    self.state = NodeState::Glowing;
                }
            }
            NodeState::Resting => {
                self.fatigue = (self.fatigue - 0.03).clamp(0.0, 1.0);
                if self.fatigue <= 0.20 {
                    self.state = NodeState::Glowing;
                }
            }
            NodeState::Stressed => {
                self.fatigue = (self.fatigue - 0.01).clamp(0.0, 1.0);
                if self.fatigue <= 0.45 {
                    self.state = NodeState::Resting;
                }
            }
            NodeState::Idle | NodeState::Glowing => {}
        }
    }

    /// R → 1 Flow. Shared field repair. Visible in one short tick burst.
    pub fn restore_from_flow(&mut self) {
        self.fatigue = (self.fatigue - 0.40).clamp(0.0, 1.0);
        self.state = if self.fatigue <= 0.20 {
            NodeState::Glowing
        } else {
            NodeState::Resting
        };
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Satchel {
    pub takes: Vec<ClimateTake>,
}

impl Satchel {
    pub fn push(&mut self, take: ClimateTake) {
        self.takes.push(take);
    }

    pub fn is_empty(&self) -> bool {
        self.takes.is_empty()
    }

    pub fn count(&self) -> usize {
        self.takes.len()
    }

    /// R spend. Returns false if the satchel has nothing to allocate.
    pub fn spend_one(&mut self) -> bool {
        self.takes.pop().is_some()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Allocation {
    pub flow: u32,
    pub reserve: u32,
}

impl Allocation {
    pub fn apply(&mut self, kind: AllocKind, satchel: &mut Satchel) -> bool {
        if !satchel.spend_one() {
            return false;
        }
        match kind {
            AllocKind::Flow => self.flow += 1,
            AllocKind::Reserve => self.reserve += 1,
        }
        true
    }

    /// Reserve is repair-rights. Spending one later is hour-two Ledger work.
    pub fn spend_reserve(&mut self) -> bool {
        if self.reserve == 0 {
            return false;
        }
        self.reserve -= 1;
        true
    }
}

/// One-machine first hour. Serialize this to data/powrush_lived_tick.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivedHour {
    pub nodes: Vec<ClimateNode>,
    pub satchel: Satchel,
    pub allocation: Allocation,
}

impl LivedHour {
    pub fn new_demo() -> Self {
        Self {
            nodes: vec![
                ClimateNode::glowing(1),
                ClimateNode::glowing(2),
                ClimateNode {
                    id: 3,
                    state: NodeState::Idle,
                    fatigue: 0.0,
                },
            ],
            satchel: Satchel::default(),
            allocation: Allocation::default(),
        }
    }

    pub fn tend(&mut self, node_id: u32) -> TendResult {
        match self.nodes.iter_mut().find(|n| n.id == node_id) {
            Some(node) => {
                let result = node.tend();
                if let TendResult::Taken { item } = &result {
                    self.satchel.push(item.clone());
                }
                result
            }
            None => TendResult::NoTake {
                reason: "no such node",
            },
        }
    }

    pub fn allocate(&mut self, kind: AllocKind) -> bool {
        if !self.allocation.apply(kind, &mut self.satchel) {
            return false;
        }
        if kind == AllocKind::Flow {
            if let Some(tired) = self
                .nodes
                .iter_mut()
                .find(|n| matches!(n.state, NodeState::Resting | NodeState::Stressed))
            {
                tired.restore_from_flow();
            }
        }
        true
    }

    pub fn tick(&mut self) {
        for node in &mut self.nodes {
            node.tick();
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_tend_takes() {
        let mut hour = LivedHour::new_demo();
        match hour.tend(1) {
            TendResult::Taken { item } => assert_eq!(item.node_id, 1),
            other => panic!("expected take, got {other:?}"),
        }
        assert_eq!(hour.satchel.count(), 1);
        assert_eq!(hour.nodes[0].state, NodeState::Tended);
    }

    #[test]
    fn extract_only_stresses() {
        let mut hour = LivedHour::new_demo();
        let _ = hour.tend(1);
        let second = hour.tend(1);
        assert!(matches!(second, TendResult::NoTake { .. }));
        assert!(matches!(
            hour.nodes[0].state,
            NodeState::Resting | NodeState::Stressed
        ));
    }

    #[test]
    fn flow_restores_tired_node() {
        let mut hour = LivedHour::new_demo();
        let _ = hour.tend(1);
        let _ = hour.tend(1);
        assert!(hour.allocate(AllocKind::Flow));
        assert_eq!(hour.satchel.count(), 0);
        assert_eq!(hour.allocation.flow, 1);
        assert!(matches!(
            hour.nodes[0].state,
            NodeState::Glowing | NodeState::Resting
        ));
    }

    #[test]
    fn reserve_does_not_spend_the_field() {
        let mut hour = LivedHour::new_demo();
        let _ = hour.tend(1);
        let before = hour.nodes[0].fatigue;
        assert!(hour.allocate(AllocKind::Reserve));
        assert_eq!(hour.allocation.reserve, 1);
        assert_eq!(hour.nodes[0].fatigue, before);
    }

    #[test]
    fn allocate_empty_satchel_fails() {
        let mut hour = LivedHour::new_demo();
        assert!(!hour.allocate(AllocKind::Flow));
    }

    #[test]
    fn json_roundtrip() {
        let mut hour = LivedHour::new_demo();
        let _ = hour.tend(1);
        let json = hour.to_json().unwrap();
        let restored = LivedHour::from_json(&json).unwrap();
        assert_eq!(restored.satchel.count(), 1);
        assert_eq!(restored.nodes[0].state, NodeState::Tended);
    }
}
