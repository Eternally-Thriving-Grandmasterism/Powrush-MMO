// game/resource_nodes.rs
// Powrush-MMO v21.47.0 — Realm Abundance Snapshot + Observatory Bridge Helper
// Previous: v21.44.0 Realm Abundance Snapshot | v21.42.0 ResourceNode Realm-Keying
// v21.47: into_view_tuple() — one-liner path into simulation::RealmAbundanceObservatory
// AG-SML v1.0 | Mercy-aligned economic foresight | Eternally-Thriving-Grandmasterism
// Thunder locked in. Yoi ⚡

use crate::engine::gpu_patsagi_bridge::GpuPatsagiResponse;
use shared::protocol::{GpuPatsagiUpdate, NodeGpuPrediction, ServerMessage};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Realm identifier (aligned with MultiRealmHarness::RealmId).
pub type RealmId = u8;

/// Lightweight, living snapshot of abundance within a single realm.
/// Designed for observability (dashboard, telemetry, MultiRealmHarness bridge).
/// Pure data — never mutates the world.
#[derive(Debug, Clone, Default)]
pub struct RealmAbundanceSnapshot {
    pub realm_id: RealmId,
    pub node_count: u32,
    pub total_current_yield: f32,
    pub average_sustainability: f32,
    pub average_abundance_flow: f32,
    pub average_stress: f32,
    pub restricted_node_count: u32,
    pub thriving_node_count: u32, // sustainability > 0.75 && stress < 0.3
}

impl RealmAbundanceSnapshot {
    pub fn is_thriving(&self) -> bool {
        self.node_count > 0
            && self.average_sustainability > 0.72
            && self.average_stress < 0.35
            && self.average_abundance_flow > -0.05
    }

    pub fn health_label(&self) -> &'static str {
        if self.node_count == 0 {
            "Empty"
        } else if self.is_thriving() {
            "Thriving"
        } else if self.average_stress > 0.6 || self.average_sustainability < 0.45 {
            "Stressed"
        } else if self.average_abundance_flow > 0.15 {
            "Abundant"
        } else {
            "Steady"
        }
    }

    /// Flatten into the exact field order expected by
    /// `simulation::multi_realm_harness::RealmAbundanceView::from_raw`.
    ///
    /// Live call site (when both crates are in scope):
    /// ```ignore
    /// let views = manager.snapshot_all_realms(now_ms).into_iter().map(|s| {
    ///     let (id, n, y, sus, flow, stress, rest, thr) = s.into_view_tuple();
    ///     RealmAbundanceView::from_raw(id, n, y, sus, flow, stress, rest, thr)
    /// });
    /// observatory.ingest_many(views, tick);
    /// ```
    pub fn into_view_tuple(self) -> (RealmId, u32, f32, f32, f32, f32, u32, u32) {
        (
            self.realm_id,
            self.node_count,
            self.total_current_yield,
            self.average_sustainability,
            self.average_abundance_flow,
            self.average_stress,
            self.restricted_node_count,
            self.thriving_node_count,
        )
    }
}

// For broader compatibility, position kept as tuple (old iterations); Vec3 can be added via glam if needed in crate root.
#[derive(Debug, Clone)]
pub struct ResourceNode {
    pub id: u64,
    pub node_id: u64, // legacy alias for compatibility
    pub resource_type: String,
    pub node_type: String, // legacy
    pub position: (f32, f32, f32),
    /// The realm this node lives in. Defaults to 0 (Sanctuary Prime).
    pub realm_id: RealmId,
    pub base_yield_per_tick: f32,
    pub current_yield: f32,
    pub depletion: f32,
    pub regeneration_rate: f32,
    pub last_harvested_ms: u64,
    pub last_harvest_ms: u64, // legacy alias
    pub sustainability_score: f32,
    pub stress_level: f32,
    pub harvest_restricted_until_ms: u64,
    pub faction_affinity: Option<String>,
    pub abundance_flow: f32,
}

impl ResourceNode {
    pub fn new(node_id: u64, node_type: &str, position: (f32, f32, f32)) -> Self {
        Self::new_in_realm(node_id, node_type, position, 0)
    }

    /// Create a node that belongs to a specific realm.
    pub fn new_in_realm(node_id: u64, node_type: &str, position: (f32, f32, f32), realm_id: RealmId) -> Self {
        let base_yield = match node_type {
            "food" => 2.5, "water" => 3.0, "energy" => 1.8,
            "minerals" => 1.2, "rare_alloy" => 0.4, _ => 1.0,
        };
        Self {
            id: node_id,
            node_id,
            resource_type: node_type.to_string(),
            node_type: node_type.to_string(),
            position,
            realm_id,
            base_yield_per_tick: base_yield,
            current_yield: base_yield,
            depletion: 0.0,
            regeneration_rate: 0.015,
            last_harvested_ms: 0,
            last_harvest_ms: 0,
            sustainability_score: 1.0,
            stress_level: 0.0,
            harvest_restricted_until_ms: 0,
            faction_affinity: None,
            abundance_flow: 0.0,
        }
    }

    pub fn regenerate(&mut self, now_ms: u64) {
        if self.depletion > 0.0 {
            self.depletion = (self.depletion - self.regeneration_rate).max(0.0);
            self.current_yield = self.base_yield_per_tick * (1.0 - self.depletion * 0.7);
        }
        self.sustainability_score = (1.0 - self.depletion * 0.5).max(0.3);

        if self.depletion < 0.3 {
            self.stress_level = (self.stress_level - 0.02).max(0.0);
        }

        // Clear harvest restriction if time has passed (from v16.5.35)
        if self.harvest_restricted_until_ms > 0 && now_ms > self.harvest_restricted_until_ms {
            self.harvest_restricted_until_ms = 0;
            self.stress_level = (self.stress_level * 0.5).max(0.0);
        }
    }
}

pub struct ResourceNodeManager {
    pub nodes: HashMap<u64, ResourceNode>,
    pub next_node_id: u64,
    pub last_global_update_ms: u64,
    pub faction_debuff_until_ms: HashMap<String, u64>,
}

impl ResourceNodeManager {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            next_node_id: 1000,
            last_global_update_ms: 0,
            faction_debuff_until_ms: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node_type: &str, position: (f32, f32, f32)) -> u64 {
        self.add_node_in_realm(node_type, position, 0)
    }

    /// Spawn a resource node that belongs to a specific realm.
    pub fn add_node_in_realm(&mut self, node_type: &str, position: (f32, f32, f32), realm_id: RealmId) -> u64 {
        let id = self.next_node_id;
        self.next_node_id += 1;
        let node = ResourceNode::new_in_realm(id, node_type, position, realm_id);
        self.nodes.insert(id, node);
        id
    }

    pub fn get_node(&self, node_id: u64) -> Option<&ResourceNode> {
        self.nodes.get(&node_id)
    }

    pub fn get_node_mut(&mut self, node_id: u64) -> Option<&mut ResourceNode> {
        self.nodes.get_mut(&node_id)
    }

    /// All nodes that live in the given realm.
    pub fn nodes_in_realm(&self, realm_id: RealmId) -> impl Iterator<Item = &ResourceNode> {
        self.nodes.values().filter(move |n| n.realm_id == realm_id)
    }

    /// Mutable access to all nodes in a realm.
    pub fn nodes_in_realm_mut(&mut self, realm_id: RealmId) -> impl Iterator<Item = &mut ResourceNode> {
        self.nodes.values_mut().filter(move |n| n.realm_id == realm_id)
    }

    /// Count of living nodes in a realm.
    pub fn count_in_realm(&self, realm_id: RealmId) -> usize {
        self.nodes.values().filter(|n| n.realm_id == realm_id).count()
    }

    /// Produce a living abundance snapshot for a single realm.
    /// Pure observation — never mutates nodes.
    pub fn snapshot_realm(&self, realm_id: RealmId, now_ms: u64) -> RealmAbundanceSnapshot {
        let mut count = 0u32;
        let mut total_yield = 0.0f32;
        let mut total_sust = 0.0f32;
        let mut total_flow = 0.0f32;
        let mut total_stress = 0.0f32;
        let mut restricted = 0u32;
        let mut thriving = 0u32;

        for node in self.nodes_in_realm(realm_id) {
            count += 1;
            total_yield += node.current_yield;
            total_sust += node.sustainability_score;
            total_flow += node.abundance_flow;
            total_stress += node.stress_level;

            if node.harvest_restricted_until_ms > now_ms {
                restricted += 1;
            }
            if node.sustainability_score > 0.75 && node.stress_level < 0.3 {
                thriving += 1;
            }
        }

        if count == 0 {
            return RealmAbundanceSnapshot {
                realm_id,
                ..Default::default()
            };
        }

        let n = count as f32;
        RealmAbundanceSnapshot {
            realm_id,
            node_count: count,
            total_current_yield: total_yield,
            average_sustainability: total_sust / n,
            average_abundance_flow: total_flow / n,
            average_stress: total_stress / n,
            restricted_node_count: restricted,
            thriving_node_count: thriving,
        }
    }

    /// Produce abundance snapshots for every realm that currently has nodes.
    pub fn snapshot_all_realms(&self, now_ms: u64) -> Vec<RealmAbundanceSnapshot> {
        let mut realm_ids: Vec<RealmId> = self.nodes.values().map(|n| n.realm_id).collect();
        realm_ids.sort_unstable();
        realm_ids.dedup();

        realm_ids
            .into_iter()
            .map(|id| self.snapshot_realm(id, now_ms))
            .collect()
    }

    pub fn tick_regen(&mut self, now_ms: u64) {
        for node in self.nodes.values_mut() {
            node.regenerate(now_ms);
        }
    }

    /// Ultimate production-ready GPU policy. Merges all prior policy depth + now_ms timestamps + full economic variables.
    pub fn apply_gpu_policy_update(&mut self, response: &GpuPatsagiResponse, now_ms: u64) {
        self.last_global_update_ms = now_ms;

        // Core GPU predictions (recommended_regen, sustainability, predicted_depletion) - strengthened
        for (node_id, &rate) in &response.recommended_regen_rates {
            if let Some(node) = self.nodes.get_mut(node_id) {
                let pred_dep = response.predicted_depletion.get(node_id).copied().unwrap_or(0.0);
                if pred_dep > 0.75 {
                    node.regeneration_rate = (node.regeneration_rate * 1.3).max(rate).min(2.5);
                    node.stress_level = (node.stress_level + 0.15).min(1.0);
                    if pred_dep > 0.85 {
                        node.harvest_restricted_until_ms = now_ms + 120_000;
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

        // Dynamic Yield Curves from abundance_flow (full)
        for (node_id, &flow) in &response.abundance_flow {
            if let Some(node) = self.nodes.get_mut(node_id) {
                node.abundance_flow = flow;
                if flow > 0.2 {
                    let bonus = 1.0 + (flow - 0.2) * 1.8;
                    node.regeneration_rate = (node.regeneration_rate * bonus).min(3.5);
                    node.sustainability_score = (node.sustainability_score + flow * 0.12).min(1.0);
                } else if flow < -0.15 {
                    node.stress_level = (node.stress_level + 0.28).min(1.0);
                    if node.stress_level > 0.75 {
                        node.harvest_restricted_until_ms = now_ms + 90_000;
                    }
                }
            }
        }

        // Pressure scenario results → dynamic yield curves (full)
        for (key, scenarios) in &response.pressure_scenario_results {
            if let Some(node_id) = key.strip_prefix("node_").and_then(|s| s.parse::<u64>().ok()) {
                if let Some(node) = self.nodes.get_mut(&node_id) {
                    if let Some(&high_pressure_yield) = scenarios.get("high") {
                        if high_pressure_yield < 0.4 {
                            node.regeneration_rate = (node.regeneration_rate * 0.82).max(0.03);
                            node.stress_level = (node.stress_level + 0.22).min(1.0);
                            if node.stress_level > 0.65 {
                                node.harvest_restricted_until_ms = now_ms + 60_000;
                            }
                        }
                    }
                }
            }
        }

        // Interdependence propagation (full live)
        for (node_id, linked_nodes) in &response.node_interdependence {
            if let Some(node) = self.nodes.get(node_id) {
                for &linked_id in linked_nodes {
                    if let Some(linked) = self.nodes.get_mut(&linked_id) {
                        if node.abundance_flow > 0.25 {
                            linked.regeneration_rate = (linked.regeneration_rate * 1.18).min(2.8);
                            linked.sustainability_score = (linked.sustainability_score + 0.09).min(1.0);
                        }
                        if node.stress_level > 0.7 {
                            linked.stress_level = (linked.stress_level + 0.12).min(1.0);
                        }
                    }
                }
            }
        }

        // Faction-level global debuffs (implemented from restoration)
        let mut faction_stress_counts: HashMap<String, u32> = HashMap::new();
        for node in self.nodes.values() {
            if let Some(ref faction) = node.faction_affinity {
                if node.stress_level > 0.8 || node.harvest_restricted_until_ms > now_ms {
                    *faction_stress_counts.entry(faction.clone()).or_insert(0) += 1;
                }
            }
        }
        for (faction, count) in faction_stress_counts {
            if count >= 3 {
                let debuff_end = now_ms + 180_000;
                self.faction_debuff_until_ms.insert(faction.clone(), debuff_end);
                for node in self.nodes.values_mut() {
                    if node.faction_affinity.as_ref() == Some(&faction) {
                        node.stress_level = (node.stress_level + 0.08).min(1.0);
                    }
                }
            }
        }
        self.faction_debuff_until_ms.retain(|_, &mut end| end > now_ms);
    }

    pub fn request_and_apply_gpu_update<G: crate::engine::gpu_patsagi_bridge::GpuPatsagiBridge>(
        &mut self,
        bridge: &G,
    ) -> Result<String, String> {
        let request = crate::engine::gpu_patsagi_bridge::GpuPatsagiRequest {
            query: "optimize long-term abundance and node health".to_string(),
            intensity: crate::engine::gpu_patsagi_bridge::ComputeIntensity::Medium,
            context: HashMap::from([("node_count".to_string(), self.nodes.len() as f32)]),
            node_ids: self.nodes.keys().cloned().collect(),
            harvesting_pressure: None,
        };
        let response = bridge.run_simulation(request)?;
        self.apply_gpu_policy_update(&response, /* current time */ 0); // caller should pass real now_ms in production
        Ok(format!("Advanced GPU policy applied (confidence: {:.2})", response.confidence))
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
        inventory: &mut crate::game::rbe::ServerInventoryComponent,
        rbe: &mut crate::game::rbe::RbeSystem,
        player_id: u64,
        amount_requested: f32,
        now_ms: u64,
    ) -> Result<String, String> {
        let node = manager.get_node_mut(node_id).ok_or_else(|| "Node not found".to_string())?;

        // Enforce temporary harvest restriction from GPU policy (v16.5.35 + enhanced)
        if node.harvest_restricted_until_ms > now_ms {
            return Err("Node is under temporary harvest restriction from PATSAGi Council recommendation.".to_string());
        }

        if node.depletion > 0.92 {
            return Err("Node critically depleted. PATSAGi recommends regeneration.".to_string());
        }

        let stress_multiplier = 1.0 - (node.stress_level * 0.5);
        let actual_yield = (node.current_yield * amount_requested.min(10.0) * stress_multiplier)
            .min(node.current_yield * 3.0);

        if actual_yield <= 0.01 {
            return Err("Node yield too low this tick.".to_string());
        }

        inventory.add_resource(&node.resource_type, actual_yield, now_ms);
        node.depletion = (node.depletion + actual_yield * 0.008).min(1.0);
        node.current_yield = node.base_yield_per_tick * (1.0 - node.depletion * 0.7);
        node.last_harvested_ms = now_ms;
        node.last_harvest_ms = now_ms;

        if node.stress_level > 0.4 {
            node.stress_level = (node.stress_level + 0.15).min(1.0);
        }

        // Grace + RBE reward (preserved from historical iterations)
        let grace_reward = (actual_yield * 0.8) as u64;
        rbe.add_grace(&player_id.to_string(), grace_reward);

        let status = if node.harvest_restricted_until_ms > 0 {
            format!("Harvest successful under restriction (+{:.1} {}). Grace +{}", actual_yield, node.resource_type, grace_reward)
        } else if node.stress_level > 0.5 {
            format!("Harvest successful (+{:.1} {}) but node is stressed. Yield reduced. Grace +{}", actual_yield, node.resource_type, grace_reward)
        } else {
            format!("Harvest successful (+{:.1} {}). Grace +{}", actual_yield, node.resource_type, grace_reward)
        };

        Ok(status)
    }
}
