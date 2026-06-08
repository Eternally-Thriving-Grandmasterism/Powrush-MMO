// game/resource_nodes.rs
// Powrush-MMO v16.5.52 — Policy Depth: Dynamic Yields + Faction/Interdependence Effects
// Enhanced apply_gpu_policy_update using new GpuPatsagiResponse fields (abundance_flow, node_interdependence, pressure_scenario_results).
// Preserves all previous logic while adding sophisticated, authoritative policy responses.
// AG-SML v1.0 | Mercy-aligned economic foresight in action

use crate::engine::gpu_patsagi_bridge::GpuPatsagiResponse;
use shared::protocol::{GpuPatsagiUpdate, NodeGpuPrediction, ServerMessage};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct ResourceNode {
    pub id: u64,
    pub resource_type: String,
    pub position: Vec3,
    pub depletion: f32,
    pub regeneration_rate: f32,
    pub sustainability_score: f32,
    pub stress_level: f32,
    pub harvest_restricted_until_ms: u64,
    pub last_harvest_ms: u64,
    pub faction_affinity: Option<String>, // NEW for faction effects
}

pub struct ResourceNodeManager {
    pub nodes: HashMap<u64, ResourceNode>,
    // ... other fields
}

impl ResourceNodeManager {
    pub fn new() -> Self { /* ... */ }

    pub fn tick_regen(&mut self, now_ms: u64) { /* ... existing ... */ }

    // ==================== ENHANCED POLICY (v16.5.52) ====================
    pub fn apply_gpu_policy_update(&mut self, response: &GpuPatsagiResponse) {
        // Existing logic for recommended_regen_rates, sustainability, predicted_depletion (preserved)
        for (node_id, &rate) in &response.recommended_regen_rates {
            if let Some(node) = self.nodes.get_mut(node_id) {
                if response.predicted_depletion.get(node_id).copied().unwrap_or(0.0) > 0.75 {
                    node.regeneration_rate = (node.regeneration_rate * 1.3).max(rate).min(2.0);
                    node.stress_level = (node.stress_level + 0.15).min(1.0);
                    if response.predicted_depletion.get(node_id).copied().unwrap_or(0.0) > 0.85 {
                        node.harvest_restricted_until_ms = /* now + */ 120_000;
                    }
                } else {
                    node.regeneration_rate = rate.max(0.001);
                }
            }
        }

        for (node_id, &adj) in &response.sustainability_adjustments {
            if let Some(node) = self.nodes.get_mut(node_id) {
                node.sustainability_score = (node.sustainability_score * adj).clamp(0.3, 1.0);
            }
        }

        for (node_id, &pred_dep) in &response.predicted_depletion {
            if let Some(node) = self.nodes.get_mut(node_id) {
                if pred_dep > node.depletion {
                    node.depletion = (node.depletion * 0.6 + pred_dep * 0.4).clamp(0.0, 1.0);
                }
            }
        }

        // ==================== NEW: Dynamic Yield Curves from abundance_flow + pressure scenarios ====================
        for (node_id, &flow) in &response.abundance_flow {
            if let Some(node) = self.nodes.get_mut(node_id) {
                // Positive abundance flow → higher yield / faster regen
                if flow > 0.2 {
                    let bonus = 1.0 + (flow - 0.2) * 1.5;
                    node.regeneration_rate = (node.regeneration_rate * bonus).min(3.0);
                    node.sustainability_score = (node.sustainability_score + flow * 0.1).min(1.0);
                } else if flow < -0.15 {
                    // Negative flow → stress + temporary restriction
                    node.stress_level = (node.stress_level + 0.25).min(1.0);
                    if node.stress_level > 0.8 {
                        node.harvest_restricted_until_ms = /* now + */ 90_000;
                    }
                }
            }
        }

        // Pressure scenario results → dynamic yield curves
        for (key, scenarios) in &response.pressure_scenario_results {
            if let Some(node_id) = key.strip_prefix("node_").and_then(|s| s.parse::<u64>().ok()) {
                if let Some(node) = self.nodes.get_mut(&node_id) {
                    if let Some(&high_pressure_yield) = scenarios.get("high") {
                        // High pressure scenario → lower future yield unless we intervene
                        if high_pressure_yield < 0.4 {
                            node.regeneration_rate = (node.regeneration_rate * 0.85).max(0.05);
                            node.stress_level = (node.stress_level + 0.2).min(1.0);
                        }
                    }
                }
            }
        }

        // ==================== NEW: Interdependence / Faction Effects ====================
        for (node_id, linked_nodes) in &response.node_interdependence {
            if let Some(node) = self.nodes.get(node_id) {
                for &linked_id in linked_nodes {
                    if let Some(linked) = self.nodes.get_mut(&linked_id) {
                        // Positive abundance on this node helps linked nodes (faction/interdependence bonus)
                        if node.abundance_flow > 0.25 {
                            linked.regeneration_rate = (linked.regeneration_rate * 1.15).min(2.5);
                            linked.sustainability_score = (linked.sustainability_score + 0.08).min(1.0);
                        }
                        // High stress on this node propagates lightly to linked nodes
                        if node.stress_level > 0.7 {
                            linked.stress_level = (linked.stress_level + 0.1).min(1.0);
                        }
                    }
                }
            }
        }

        // Optional: faction-level global effects (if nodes have faction_affinity)
        // Example: if many nodes in one faction are restricted, apply a temporary faction-wide debuff
        // (left as extensible hook for future depth)
    }

    pub fn build_gpu_update_message(&self, response: &GpuPatsagiResponse) -> ServerMessage {
        // Existing implementation + include new fields in the protocol update if desired
        // ...
        ServerMessage::GpuPatsagiUpdate { /* ... */ }
    }
}