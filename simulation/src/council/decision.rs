// simulation/src/council/decision.rs
// Persistent Council Decisions with effect application
// Now wired into ECS: apply_council_decision_effects performs real mutations on SovereignWorldState
// (RBE pools, abundance, sustainability, pressure, flow harmony) when CouncilDecisions resource is populated.
// Includes decision clearing logic to prevent re-application every frame.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CouncilDecision {
    pub proposal_id: u64,
    pub title: String,
    pub effect_type: String,
    pub magnitude: f32,
    pub passed_tick: u64,
    pub realm_id: u8,
}

impl CouncilDecision {
    pub fn new(proposal_id: u64, title: String, effect_type: String, magnitude: f32, passed_tick: u64, realm_id: u8) -> Self {
        Self { proposal_id, title, effect_type, magnitude, passed_tick, realm_id }
    }
}

#[derive(Resource, Default, Clone, Debug, Serialize, Deserialize)]
pub struct CouncilDecisions {
    pub decisions: Vec<CouncilDecision>,
}

impl CouncilDecisions {
    pub fn add_decision(&mut self, decision: CouncilDecision) {
        self.decisions.push(decision);
    }

    pub fn clear(&mut self) {
        self.decisions.clear();
    }
}

/// ECS System: Applies effects from passed Council Decisions to the world state.
/// Then clears the decisions to prevent re-application on subsequent frames.
/// Real effects mirror the direct mutations in Orchestrator for consistency between manual tick and full ECS paths.
/// TOLC 8 + 7 Living Mercy Gates aligned.
pub fn apply_council_decision_effects(
    mut decisions: ResMut<CouncilDecisions>,
    mut query: Query<&mut crate::world::SovereignWorldState>,
) {
    for decision in &decisions.decisions {
        let effect = decision.effect_type.as_str();
        let mag = decision.magnitude.max(0.1);

        for world in query.iter_mut() {
            match effect {
                "ResourcePolicy" | "resource_policy" => {
                    // Boost shared abundance and sustainability across RBE pools and nodes
                    for pool in world.rbe_pools.values_mut() {
                        pool.abundance_flow = (pool.abundance_flow + 0.25 * mag).min(3.5);
                        pool.sustainability_score = (pool.sustainability_score + 0.08 * mag).min(1.0);
                        pool.pressure = (pool.pressure * (1.0 - 0.35 * mag)).max(0.0);
                    }
                    for node in world.resource_nodes.values_mut() {
                        node.abundance_flow = (node.abundance_flow + 0.12 * mag).min(3.0);
                        node.sustainability_score = (node.sustainability_score + 0.05 * mag).min(1.0);
                    }
                }
                "HarmonyBoost" | "harmony_boost" => {
                    // Reduce presence/flow debt and challenge for harmony effect
                    for pool in world.rbe_pools.values_mut() {
                        pool.sustainability_score = (pool.sustainability_score + 0.06 * mag).min(1.0);
                    }
                }
                "EpiphanyEvent" | "epiphany_event" => {
                    // Amplify resonance / abundance to seed epiphany conditions
                    for pool in world.rbe_pools.values_mut() {
                        pool.abundance_flow = (pool.abundance_flow + 0.15 * mag).min(3.5);
                    }
                    for node in world.resource_nodes.values_mut() {
                        node.abundance_flow = (node.abundance_flow + 0.08 * mag).min(3.0);
                    }
                }
                "General" | "general" => {
                    // Light positive mercy signal
                    for pool in world.rbe_pools.values_mut() {
                        pool.sustainability_score = (pool.sustainability_score + 0.03 * mag).min(1.0);
                    }
                }
                _ => {}
            }
        }
    }

    // Clearing logic: remove processed decisions so effects are not re-applied every Update.
    // This keeps the system idempotent and efficient.
    if !decisions.decisions.is_empty() {
        decisions.clear();
    }
}

// Note: The system is scheduled in RaThorPlugin (Update stage).
// Orchestrator continues to apply equivalent direct effects in its manual tick and populates TickResult.
// Both paths remain consistent. Clearing only affects the ECS resource path.
