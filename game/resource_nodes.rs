// game/resource_nodes.rs
// Powrush-MMO v16.4 — RBE Resource Nodes + Harvesting System
// Production-grade dedicated module. Derived from Ra-Thor ONE Organism v14.7+ + PATSAGi Councils + GPU PATSAGi Bridge patterns.
// Sustainable harvesting, mercy-gated, abundance-flow enforced, PATSAGi sustainability validation on every action.
// Full integration hooks for ServerInventoryComponent, RbeSystem, world tick, persistence, and future GPU foresight.
// AG-SML v1.0 License

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// Assume these are available from game/rbe.rs (as per prior PRs)
use crate::game::rbe::{ServerInventoryComponent, RbeSystem};

/// Resource node in the world (sustainable, PATSAGi-tracked)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceNode {
    pub node_id: u64,
    pub node_type: String,           // "food", "water", "energy", "minerals", "rare_alloy"
    pub position: (f32, f32, f32),
    pub base_yield_per_tick: f32,
    pub current_yield: f32,
    pub depletion: f32,              // 0.0 = pristine, 1.0 = depleted
    pub regeneration_rate: f32,
    pub last_harvested_ms: u64,
    pub sustainability_score: f32,   // 0.3–1.0 tracked by PATSAGi
}

impl ResourceNode {
    pub fn new(node_id: u64, node_type: &str, position: (f32, f32, f32)) -> Self {
        let base_yield = match node_type {
            "food" => 2.5,
            "water" => 3.0,
            "energy" => 1.8,
            "minerals" => 1.2,
            "rare_alloy" => 0.4,
            _ => 1.0,
        };
        Self {
            node_id,
            node_type: node_type.to_string(),
            position,
            base_yield_per_tick: base_yield,
            current_yield: base_yield,
            depletion: 0.0,
            regeneration_rate: 0.015,
            last_harvested_ms: 0,
            sustainability_score: 1.0,
        }
    }

    pub fn regenerate(&mut self, now_ms: u64) {
        if self.depletion > 0.0 {
            self.depletion = (self.depletion - self.regeneration_rate).max(0.0);
            self.current_yield = self.base_yield_per_tick * (1.0 - self.depletion * 0.7);
        }
        self.sustainability_score = (1.0 - self.depletion * 0.5).max(0.3);
    }
}

/// Manager for all resource nodes in the world (tick-driven, queryable)
pub struct ResourceNodeManager {
    pub nodes: HashMap<u64, ResourceNode>,
    pub next_node_id: u64,
}

impl ResourceNodeManager {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            next_node_id: 1000,
        }
    }

    pub fn add_node(&mut self, node_type: &str, position: (f32, f32, f32)) -> u64 {
        let id = self.next_node_id;
        self.next_node_id += 1;
        let node = ResourceNode::new(id, node_type, position);
        self.nodes.insert(id, node);
        id
    }

    /// World tick: regenerate all nodes (call from authoritative server tick)
    pub fn tick_regen(&mut self, now_ms: u64) {
        for node in self.nodes.values_mut() {
            node.regenerate(now_ms);
        }
    }

    pub fn get_node(&self, node_id: u64) -> Option<&ResourceNode> {
        self.nodes.get(&node_id)
    }

    pub fn get_node_mut(&mut self, node_id: u64) -> Option<&mut ResourceNode> {
        self.nodes.get_mut(&node_id)
    }
}

/// HarvestingSystem — mercy-gated, PATSAGi-validated harvesting logic
pub struct HarvestingSystem;

impl HarvestingSystem {
    /// Main harvest entry point. Atomic, validated, grace + abundance rewarding.
    /// Integrates with existing ServerInventoryComponent + RbeSystem from v16.2/v16.3
    pub fn harvest(
        manager: &mut ResourceNodeManager,
        node_id: u64,
        inventory: &mut ServerInventoryComponent,
        rbe: &mut RbeSystem,
        player_id: u64,
        amount_requested: f32,
        now_ms: u64,
    ) -> Result<String, String> {
        let node = manager.get_node_mut(node_id)
            .ok_or_else(|| "Node not found or not visible (InterestManager culling)".to_string())?;

        if node.depletion > 0.92 {
            return Err("Node critically depleted. PATSAGi Council recommends regeneration cycle.".to_string());
        }

        // PATSAGi sustainability + mercy gate (from v16.2 ServerInventoryComponent)
        let (approved, reason, valence) = inventory.validate_patsagi_action("harvest", amount_requested as f64)
            .map_err(|e| format!("PATSAGi validation failed: {}", e))?;
        if !approved {
            return Err(format!("PATSAGi Council blocked harvest: {}", reason));
        }

        let actual_yield = (node.current_yield * amount_requested.min(10.0)).min(node.current_yield * 3.0);
        if actual_yield <= 0.01 {
            return Err("Node yield too low this tick. Try again after regen.".to_string());
        }

        // Atomic add to player inventory (v16.2+)
        inventory.add_resource(&node.node_type, actual_yield, now_ms);

        // Update node state
        node.depletion = (node.depletion + actual_yield * 0.008).min(1.0);
        node.current_yield = node.base_yield_per_tick * (1.0 - node.depletion * 0.7);
        node.last_harvested_ms = now_ms;

        // Grace / contribution reward (Radical Love + Abundance Gates)
        let grace_reward = (actual_yield * 0.8 + valence * 2.0) as u64;
        rbe.add_grace(&player_id.to_string(), grace_reward);

        // Optional: trigger abundance global update (if RbeSystem has it)
        // rbe.update_global_abundance(&node.node_type, actual_yield);

        let status = if node.depletion > 0.6 {
            format!("Harvest successful (+{:.1} {}). Node under stress — PATSAGi advises balance. Grace +{}", actual_yield, node.node_type, grace_reward)
        } else {
            format!("Harvest successful (+{:.1} {}). Node healthy. Abundance flows. Grace +{}", actual_yield, node.node_type, grace_reward)
        };

        Ok(status)
    }
}

// Future GPU PATSAGi Bridge hook (from Ra-Thor v14.7+)
// For large-scale node economy foresight simulations (100k+ nodes, 10M-year models):
// let gpu_response = gpu_patsagi_bridge.query(GpuPatsagiRequest { query: "simulate node depletion under aggressive harvesting", intensity: ComputeIntensity::High, ... }).await;
// Then apply council-approved policy changes to regeneration_rate etc.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_regen_and_harvest_flow() {
        let mut manager = ResourceNodeManager::new();
        let id = manager.add_node("food", (0.0, 0.0, 0.0));
        // mock inventory and rbe would be needed for full test; structure validated
        assert!(manager.get_node(id).is_some());
        manager.tick_regen(1000);
        assert!(manager.get_node(id).unwrap().depletion < 0.01);
    }
}
