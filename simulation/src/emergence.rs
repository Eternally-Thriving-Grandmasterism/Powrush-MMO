/*!
 * Realtime Emergence System (Phase 1 + Initial Phase 2 Council Stub)
 *
 * Implements the foundational layer for council-driven, mercy-gated, context-aware
 * dynamic events and emergence as specified in REALTIME_GENERATION.md v2.0.
 *
 * Integrates directly with:
 * - player_persistence (PlayerSaveData, EpiphanyRecord)
 * - epiphany_catalyst (EpiphanyOutcome)
 * - harvest systems
 * - mercy scoring
 *
 * Phase 2 addition: First council query stub in EmergenceOrchestrator.
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Re-exports from existing modules for clean integration
pub use crate::player_persistence::data::{EpiphanyRecord, PlayerSaveData, PersistenceUpdated};
pub use crate::epiphany_catalyst::EpiphanyOutcome;
pub use crate::mercy::MercyGate;

/// Lightweight trigger for potential emergence events.
#[derive(Debug, Clone, Component)]
pub struct EmergenceSeed {
    pub source: EmergenceSource,
    pub location: Option<Vec3>,
    pub valence_delta: f32,
    pub intensity: f32,
    pub biome: String,
    pub group_size: u32,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmergenceSource {
    Harvest,
    Epiphany,
    CouncilParticipation,
    BiomeResonance,
    PlayerAction(String),
}

/// Phases of a dynamic emergence event.
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub enum DynamicEmergenceEventPhase {
    Proposal,
    CouncilReview { guidance: Option<String> },
    Resolution { effects_applied: bool },
    PersistenceApplied,
}

/// A living, council-influenced event that can affect RBE, epiphany, audio, and persistence.
#[derive(Debug, Clone, Component)]
pub struct DynamicEmergenceEvent {
    pub id: u64,
    pub phase: DynamicEmergenceEventPhase,
    pub seed: EmergenceSeed,
    pub proposed_effects: Vec<EmergenceEffect>,
    pub mercy_score: f32,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmergenceEffect {
    EpiphanyTrigger { scenario_id: String, intensity_boost: f32 },
    ResourceDelta { resource: String, amount: f32, is_abundance: bool },
    TemporaryMultiplier { multiplier: f32, duration_seconds: u64 },
    AudioResonanceSeed { seed: f32, biome: String },
    DivineWhisperInjection { flavor: String },
    BiomeResonance { intensity: f32 },
    MuscleMemoryConsolidation { boost: f32 },
}

/// Central orchestrator resource.
/// Phase 2: Now contains the first council query stub.
#[derive(Resource, Default)]
pub struct EmergenceOrchestrator {
    pub mercy_budget: MercyBudget,
    pub event_counter: u64,
    // Phase 2 stub: Will be replaced with real Ra-Thor / PATSAGi query client
    pub council_query_enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct MercyBudget {
    pub remaining: f32,
    pub max_per_event: f32,
}

impl Default for MercyBudget {
    fn default() -> Self {
        Self {
            remaining: 100.0,
            max_per_event: 25.0,
        }
    }
}

impl MercyBudget {
    pub fn can_afford(&self, intensity: f32) -> bool {
        let cost = intensity * 8.0;
        cost <= self.remaining && cost <= self.max_per_event
    }

    pub fn spend(&mut self, intensity: f32) {
        let cost = intensity * 8.0;
        self.remaining = (self.remaining - cost).max(0.0);
    }
}

/// Phase 2 stub: First council query interface.
/// Currently returns None (no real bridge yet). Will be expanded in subsequent commits.
impl EmergenceOrchestrator {
    /// Attempts to query the PATSAGi Councils / Ra-Thor lattice for guidance on an emergence seed.
    /// Returns structured guidance when available.
    pub fn query_council_for_guidance(&self, seed: &EmergenceSeed) -> Option<CouncilGuidance> {
        if !self.council_query_enabled {
            return None;
        }

        // TODO (Phase 2+): Replace with actual async/batched query to Ra-Thor lattice
        // For now, this is the integration point for future council-driven emergence.
        // Example future behavior:
        // - Send valence, biome, player history summary, group composition
        // - Receive flavor, suggested effects, mercy constraints from specific councils

        // Placeholder: Return basic guidance based on intensity for development continuity
        if seed.intensity > 0.6 {
            Some(CouncilGuidance {
                flavor: "abundance".to_string(),
                suggested_intensity: seed.intensity * 0.85,
                mercy_note: "Favor generative outcomes".to_string(),
            })
        } else {
            None
        }
    }
}

/// Structured response from council query (Phase 2 foundation).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouncilGuidance {
    pub flavor: String,           // e.g. "mercy", "abundance", "truth", "joy"
    pub suggested_intensity: f32,
    pub mercy_note: String,
}

/// Initial valence aggregation system (Phase 1).
pub fn valence_aggregation_system(
    mut commands: Commands,
    player_query: Query<(Entity, &PlayerSaveData), Changed<PlayerSaveData>>,
    mut orchestrator: ResMut<EmergenceOrchestrator>,
) {
    for (entity, save_data) in player_query.iter() {
        let valence = (save_data.resonance_score * 0.6
            + (save_data.muscle_memory_level / 5.0) * 0.4)
            .clamp(0.0, 1.0);

        if save_data.dirty && save_data.total_epiphanies > 0 {
            let seed = EmergenceSeed {
                source: EmergenceSource::Epiphany,
                location: None,
                valence_delta: valence - 0.5,
                intensity: (save_data.resonance_score * 0.8).clamp(0.1, 1.0),
                biome: save_data
                    .biome_affinity
                    .iter()
                    .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                    .map(|(b, _)| b.clone())
                    .unwrap_or_else(|| "neutral".to_string()),
                group_size: 1,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };

            if orchestrator.mercy_budget.can_afford(seed.intensity) {
                orchestrator.mercy_budget.spend(seed.intensity);
                commands.entity(entity).insert(seed);
            }
        }
    }
}

/// Event proposal system with Phase 2 council query integration point.
pub fn emergence_event_proposal_system(
    mut commands: Commands,
    seed_query: Query<(Entity, &EmergenceSeed), Without<DynamicEmergenceEvent>>,
    mut orchestrator: ResMut<EmergenceOrchestrator>,
) {
    for (entity, seed) in seed_query.iter() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Phase 2: Attempt council query (currently returns None until bridge is live)
        let council_guidance = orchestrator.query_council_for_guidance(seed);

        let mut effects = vec![];

        match seed.source {
            EmergenceSource::Epiphany => {
                effects.push(EmergenceEffect::EpiphanyTrigger {
                    scenario_id: "emergent_reflection".to_string(),
                    intensity_boost: seed.intensity * 0.3,
                });
                effects.push(EmergenceEffect::MuscleMemoryConsolidation {
                    boost: seed.intensity * 0.15,
                });
            }
            EmergenceSource::Harvest => {
                effects.push(EmergenceEffect::ResourceDelta {
                    resource: "harmonic_resonance".to_string(),
                    amount: seed.intensity * 2.0,
                    is_abundance: true,
                });
            }
            _ => {
                effects.push(EmergenceEffect::BiomeResonance {
                    intensity: seed.intensity * 0.5,
                });
            }
        }

        // If council guidance was returned, we can adjust effects here in future iterations
        if let Some(guidance) = &council_guidance {
            // Placeholder for future effect modulation based on council flavor
            // e.g. if guidance.flavor == "abundance" { ... }
        }

        let mercy_score = (0.7 + seed.intensity * 0.25).clamp(0.5, 0.98);

        let mut event = DynamicEmergenceEvent {
            id: orchestrator.event_counter,
            phase: if council_guidance.is_some() {
                DynamicEmergenceEventPhase::CouncilReview {
                    guidance: Some("pending_lattice_response".to_string()),
                }
            } else {
                DynamicEmergenceEventPhase::Proposal
            },
            seed: seed.clone(),
            proposed_effects: effects,
            mercy_score,
            created_at: now,
        };

        orchestrator.event_counter += 1;

        commands.entity(entity).insert(event);
        commands.entity(entity).remove::<EmergenceSeed>();
    }
}

/// Basic resolution system (Phase 1).
pub fn emergence_event_resolution_system(
    mut commands: Commands,
    mut event_query: Query<(Entity, &mut DynamicEmergenceEvent, Option<&mut PlayerSaveData>)>,
    mut persistence_events: EventWriter<PersistenceUpdated>,
) {
    for (entity, mut event, maybe_save_data) in event_query.iter_mut() {
        if matches!(event.phase, DynamicEmergenceEventPhase::Proposal) {
            for effect in &event.proposed_effects {
                if let Some(save_data) = maybe_save_data {
                    match effect {
                        EmergenceEffect::MuscleMemoryConsolidation { boost } => {
                            save_data.muscle_memory_level =
                                (save_data.muscle_memory_level + boost).min(5.0);
                            save_data.dirty = true;
                        }
                        EmergenceEffect::ResourceDelta { amount, .. } => {
                            save_data.resonance_score =
                                (save_data.resonance_score + amount * 0.01).min(1.0);
                            save_data.dirty = true;
                        }
                        _ => {}
                    }
                }
            }

            event.phase = DynamicEmergenceEventPhase::Resolution {
                effects_applied: true,
            };

            persistence_events.send(PersistenceUpdated {
                reason: format!("emergence_event_{}", event.id),
            });

            if let Some(save_data) = maybe_save_data {
                save_data.dirty = true;
            }
        }
    }
}

/// Plugin to register all emergence systems.
pub struct EmergencePlugin;

impl Plugin for EmergencePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EmergenceOrchestrator>()
            .add_systems(
                Update,
                (
                    valence_aggregation_system,
                    emergence_event_proposal_system,
                    emergence_event_resolution_system,
                )
                    .chain(),
            );
    }
}

/*
 * Integration Notes:
 *
 * 1. Already wired in simulation/src/lib.rs
 *
 * 2. In the main App (client or server), add:
 *    .add_plugins(EmergencePlugin)
 *
 * 3. Future: Replace the stub in query_council_for_guidance() with real
 *    async call to Ra-Thor lattice / PATSAGi Councils.
 *
 * 4. The CouncilReview phase is now partially active when guidance is returned.
 */
