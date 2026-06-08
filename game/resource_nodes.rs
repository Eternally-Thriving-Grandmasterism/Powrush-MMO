// game/resource_nodes.rs
// Powrush-MMO v16.5.31 — Make GPU Output More Directly Actionable
// Automatically apply smart recommendations from GPU simulations.
// AG-SML v1.0

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::game::rbe::{ServerInventoryComponent, RbeSystem};
use crate::engine::gpu_patsagi_bridge::{GpuPatsagiBridge, GpuPatsagiRequest, GpuPatsagiResponse, ComputeIntensity};
use shared::protocol::{ServerMessage, GpuPatsagiUpdate, NodeGpuPrediction};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceNode {
    pub node_id: u64,
    pub node_type: String,
    pub position: (f32, f32, f32),
    pub base_yield_per_tick: f32,
    pub current_yield: f32,
    pub depletion: f32,
    pub regeneration_rate: f32,
    pub last_harvested_ms: u64,
    pub sustainability_score: f32,
}

impl ResourceNode {
    pub fn new(node_id: u64, node_type: &str, position: (f32, f32, f32)) -> Self {
        let base_yield = match node_type {
            "food" => 2.5, "water" => 3.0, "energy" => 1.8,
            "minerals" => 1.2, "rare_alloy" => 0.4, _ => 1.0,
        };
        Self {
            node_id, node_type: node_type.to_string(), position,
            base_yield_per_tick: base_yield, current_yield: base_yield,
            depletion: 0.0, regeneration_rate: 0.015,
            last_harvested_ms: 0, sustainability_score: 1.0,
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

pub struct ResourceNodeManager {
    pub nodes: HashMap<u64, ResourceNode>,
    pub next_node_id: u64,
}

impl ResourceNodeManager {
    pub fn new() -> Self {
        Self { nodes: HashMap::new(), next_node_id: 1000 }
    }

    pub fn add_node(&mut self, node_type: &str, position: (f32, f32, f32)) -> u64 {
        let id = self.next_node_id; self.next_node_id += 1;
        let node = ResourceNode::new(id, node_type, position);
        self.nodes.insert(id, node); id
    }

    pub fn tick_regen(&mut self, now_ms: u64) {
        for node in self.nodes.values_mut() { node.regenerate(now_ms); }
    }

    pub fn get_node(&self, node_id: u64) -> Option<&ResourceNode> { self.nodes.get(&node_id) }
    pub fn get_node_mut(&mut self, node_id: u64) -> Option<&mut ResourceNode> { self.nodes.get_mut(&node_id) }

    pub fn apply_gpu_policy_update(&mut self, response: &GpuPatsagiResponse) {
        for (node_id, &new_rate) in &response.recommended_regen_rates {
            if let Some(node) = self.nodes.get_mut(node_id) {
                // Smart application: only increase regen if future depletion is high
                if let Some(&future_dep) = response.predicted_depletion.get(node_id) {
                    if future_dep > 0.6 {
                        node.regeneration_rate = (new_rate * 1.2).max(node.regeneration_rate);
                    }
                } else {
                    node.regeneration_rate = new_rate.max(0.001);
                }
            }
        }

        for (node_id, &adjustment) in &response.sustainability_adjustments {
            if let Some(node) = self.nodes.get_mut(node_id) {
                node.sustainability_score = (node.sustainability_score * adjustment).clamp(0.3, 1.0);
            }
        }

        // Auto-apply predicted depletion as a soft target
        for (node_id, &predicted) in &response.predicted_depletion {
            if let Some(node) = self.nodes.get_mut(node_id) {
                // Blend predicted future state into current depletion
                if predicted > node.depletion {
                    node.depletion = (node.depletion * 0.6 + predicted * 0.4).clamp(0.0, 1.0);
                }
            }
        }
    }

    pub fn request_and_apply_gpu_update<G: GpuPatsagiBridge>(
        &mut self, bridge: &G,
    ) -> Result<String, String> {
        let request = GpuPatsagiRequest {
            query: "optimize long-term abundance and node health".to_string(),
            intensity: ComputeIntensity::Medium,
            context: HashMap::from([("node_count".to_string(), self.nodes.len() as f32)]),
            node_ids: self.nodes.keys().cloned().collect(),
        };

        let response = bridge.run_simulation(request)?;
        self.apply_gpu_policy_update(&response);

        Ok(format!("Smart GPU policy applied (confidence: {:.2})", response.confidence))
    }

    pub fn build_gpu_update_message(&self, response: &GpuPatsagiResponse) -> ServerMessage {
        let mut node_predictions = HashMap::new();
        for (node_id, node) in &self.nodes {
            node_predictions.insert(*node_id, NodeGpuPrediction {
                predicted_depletion: response.predicted_depletion.get(node_id).copied().unwrap_or(node.depletion),
                recommended_regen_rate: response.recommended_regen_rates.get(node_id).copied().unwrap_or(node.regeneration_rate),
                sustainability_forecast: response.sustainability_adjustments.get(node_id).copied().unwrap_or(node.sustainability_score),
            });
        }
        ServerMessage::GpuPatsagiUpdate {
            global_confidence: response.confidence,
            node_predictions,
            notes: response.notes.clone(),
        }
    }
}

pub struct HarvestingSystem;

impl HarvestingSystem {
    pub fn harvest(
        manager: &mut ResourceNodeManager,
        node_id: u64,
        inventory: &mut ServerInventoryComponent,
        rbe: &mut RbeSystem,
        player_id: u64,
        amount_requested: f32,
        now_ms: u64,
    ) -> Result<String, String> {
        let node = manager.get_node_mut(node_id).ok_or_else(|| "Node not found".to_string())?;
        if node.depletion > 0.92 { return Err("Node critically depleted".to_string()); }

        let actual_yield = (node.current_yield * amount_requested.min(10.0)).min(node.current_yield * 3.0);
        if actual_yield <= 0.01 { return Err("Node yield too low".to_string()); }

        inventory.add_resource(&node.node_type, actual_yield, now_ms);
        node.depletion = (node.depletion + actual_yield * 0.008).min(1.0);
        node.current_yield = node.base_yield_per_tick * (1.0 - node.depletion * 0.7);
        node.last_harvested_ms = now_ms;

        let grace_reward = (actual_yield * 0.8) as u64;
        rbe.add_grace(&player_id.to_string(), grace_reward);

        Ok(format!("Harvest successful (+{:.1} {}). Grace +{}", actual_yield, node.node_type, grace_reward))
    }
}