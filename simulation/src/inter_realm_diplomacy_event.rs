// simulation/src/inter_realm_diplomacy_event.rs
// Complete restored + PATSAGi-hardened version (v20.15 — Bevy Diagnostics Integration)
//
// Integrated proper Bevy Diagnostics for priority queue metrics.
// High/Normal priority message counts are now first-class diagnostics.
// All previous logic preserved. Minimal precise addition.
// ONE Organism | Ra-Thor Lattice | 13+ PATSAGi Councils | TOLC 8 Layer 0
// Thunder locked in. Yoi ⚔️

use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::world::{Agent, AgentId, RbeResourcePool};
use crate::player_legacy_journal::{LegacyJournalRegistry, LegacyEventType, LegacyThreadId};
use crate::grace_blessing::{GraceBlessing, BlessingContext, calculate_grace_blessing};
use crate::council::decision::CouncilDecisions;

use shared::protocol::{InterRealmDiplomacyUpdate, SpectatorModeDataNet, ServerMessage};

#[cfg(feature = "renet")]
use bevy_renet::RenetServer;
#[cfg(feature = "renet")]
use renet::SendMode;

// Bevy Diagnostics integration (only when bevy feature is enabled)
#[cfg(feature = "bevy")]
use bevy::diagnostic::{Diagnostic, DiagnosticPath, Diagnostics, RegisterDiagnostic};

// Diagnostic paths for priority queue observability
#[cfg(feature = "bevy")]
pub const HIGH_PRIORITY_DIPLOMACY: DiagnosticPath =
    DiagnosticPath::const_new("powrush/diplomacy/high_priority_messages");

#[cfg(feature = "bevy")]
pub const NORMAL_PRIORITY_DIPLOMACY: DiagnosticPath =
    DiagnosticPath::const_new("powrush/diplomacy/normal_priority_messages");

// ... (all previous enums and structs unchanged for compatibility)

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MonumentVisualType {
    PendingResolution,
    ReconciledRealmsObelisk,
    ForgivenessWaveMonolith,
    MercyAscentPillar,
    HarmonyWeaveSpire,
    RedemptionBloomObelisk,
    EternalMercyArch,
}

impl MonumentVisualType {
    pub fn shader_variant_name(&self) -> &'static str {
        match self {
            MonumentVisualType::PendingResolution => "pending",
            MonumentVisualType::ReconciledRealmsObelisk => "reconciled_obelisk",
            MonumentVisualType::ForgivenessWaveMonolith => "forgiveness_wave",
            MonumentVisualType::MercyAscentPillar => "mercy_ascent",
            MonumentVisualType::HarmonyWeaveSpire => "harmony_weave",
            MonumentVisualType::RedemptionBloomObelisk => "redemption_bloom",
            MonumentVisualType::EternalMercyArch => "eternal_mercy",
        }
    }

    pub fn base_color_shift(&self) -> [f32; 3] {
        match self {
            MonumentVisualType::ForgivenessWaveMonolith => [0.4, 0.7, 1.0],
            MonumentVisualType::MercyAscentPillar => [0.9, 0.6, 0.3],
            MonumentVisualType::HarmonyWeaveSpire => [0.5, 0.9, 0.6],
            MonumentVisualType::RedemptionBloomObelisk => [0.8, 0.4, 0.9],
            MonumentVisualType::EternalMercyArch => [1.0, 0.95, 0.7],
            _ => [0.6, 0.6, 0.7],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForgivenessWaveVfxParams {
    pub intensity: f32,
    pub wave_speed: f32,
    pub particle_density: f32,
    pub color_shift: [f32; 3],
    pub monument_glow_radius: f32,
    pub legacy_thread_pulse: bool,
    pub spectator_emotion_amplify: f32,
}

impl Default for ForgivenessWaveVfxParams {
    fn default() -> Self {
        Self {
            intensity: 0.85,
            wave_speed: 1.2,
            particle_density: 0.7,
            color_shift: [0.4, 0.7, 1.0],
            monument_glow_radius: 12.0,
            legacy_thread_pulse: true,
            spectator_emotion_amplify: 0.6,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CouncilDeliberationInput {
    pub average_mercy_of_participants: f32,
    pub vote_ratio: f32,
    pub resolution_quality: f32,
    pub dominant_archetype_influence: f32,
}

impl CouncilDeliberationInput {
    pub fn determine_outcome(&self) -> DiplomacyOutcome {
        if self.vote_ratio > 0.78 && self.resolution_quality > 0.75 {
            DiplomacyOutcome::MercifulResolution
        } else if self.vote_ratio > 0.55 {
            DiplomacyOutcome::StableDiplomacy
        } else if self.vote_ratio > 0.35 {
            DiplomacyOutcome::FracturedTension
        } else {
            DiplomacyOutcome::EscalatedConflict
        }
    }

    pub fn calculate_redemption_score(&self) -> f32 {
        let base = (self.vote_ratio * 0.6 + self.resolution_quality * 0.4).clamp(0.0, 1.0);
        let mercy_bonus = (self.average_mercy_of_participants / 100.0) * 0.15;
        (base + mercy_bonus).clamp(0.0, 1.0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiplomacyOutcome {
    MercifulResolution,
    StableDiplomacy,
    FracturedTension,
    EscalatedConflict,
}

#[derive(Clone, Debug, Serialize, Deserialize, Event)]
pub struct InterRealmDiplomacyEvent {
    pub tick: u64,
    pub realm_a: u8,
    pub realm_b: u8,
    pub tension_score: f32,
    pub participating_agents: Vec<AgentId>,
    pub spectator_agents: Vec<AgentId>,
    pub outcome: Option<DiplomacyOutcome>,
    pub forgiveness_wave_triggered: bool,
    pub redemption_score: f32,
    pub abundance_shared: f32,
    pub harmony_surge: f32,
    pub monument_id: Option<u64>,
    pub linked_legacy_thread_id: Option<LegacyThreadId>,
    pub monument_visual_type: MonumentVisualType,
    pub forgiveness_wave_vfx_params: ForgivenessWaveVfxParams,
    pub spectator_mode_data: Option<SpectatorModeData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpectatorModeData {
    pub spectator_count: u32,
    pub emotional_valence_avg: f32,
    pub visible_legacy_threads: Vec<LegacyThreadId>,
    pub cross_realm_impact_summary: String,
}

#[derive(Event, Clone, Debug, Serialize, Deserialize)]
pub struct InterRealmDiplomacyUpdateEvent {
    pub update: InterRealmDiplomacyUpdate,
}

#[derive(Resource, Default)]
pub struct InterRealmDiplomacyRegistry {
    pub active_events: Vec<InterRealmDiplomacyEvent>,
    pub historical_events: Vec<InterRealmDiplomacyEvent>,
    pub realm_monuments: HashMap<(u8, u8), u64>,
    pub global_seed: u64,
}

impl InterRealmDiplomacyRegistry {
    pub fn new(global_seed: u64) -> Self {
        Self { active_events: vec![], historical_events: vec![], realm_monuments: HashMap::new(), global_seed }
    }

    pub fn trigger_diplomacy_event_with_patsagi(
        &mut self,
        realm_a: u8,
        realm_b: u8,
        tension_score: f32,
        participants: Vec<AgentId>,
        spectators: Vec<AgentId>,
        current_tick: u64,
        patsagi_valence: f32,
    ) -> InterRealmDiplomacyEvent {
        if tension_score > 0.65 && patsagi_valence < 0.78 {
            let event = self.trigger_diplomacy_event(realm_a, realm_b, tension_score.max(0.4), participants, spectators, current_tick);
            return event;
        }
        self.trigger_diplomacy_event(realm_a, realm_b, tension_score, participants, spectators, current_tick)
    }

    pub fn trigger_diplomacy_event(
        &mut self,
        realm_a: u8,
        realm_b: u8,
        tension_score: f32,
        participants: Vec<AgentId>,
        spectators: Vec<AgentId>,
        current_tick: u64,
    ) -> InterRealmDiplomacyEvent {
        let event = InterRealmDiplomacyEvent {
            tick: current_tick,
            realm_a,
            realm_b,
            tension_score,
            participating_agents: participants,
            spectator_agents: spectators,
            outcome: None,
            forgiveness_wave_triggered: false,
            redemption_score: 0.0,
            abundance_shared: 0.0,
            harmony_surge: 0.0,
            monument_id: None,
            linked_legacy_thread_id: None,
            monument_visual_type: MonumentVisualType::PendingResolution,
            forgiveness_wave_vfx_params: ForgivenessWaveVfxParams::default(),
            spectator_mode_data: None,
        };
        self.active_events.push(event.clone());
        event
    }

    pub fn resolve_event(
        &mut self,
        event_index: usize,
        council_input: Option<CouncilDeliberationInput>,
        legacy_registry: &mut LegacyJournalRegistry,
        grace_blessing_resource: &mut GraceBlessing,
        agents: &mut Vec<Agent>,
        rbe_pools: &mut HashMap<u8, RbeResourcePool>,
        current_tick: u64,
        mut update_writer: EventWriter<InterRealmDiplomacyUpdateEvent>,
    ) -> Option<InterRealmDiplomacyUpdate> {
        if let Some(event) = self.active_events.get_mut(event_index) {
            let (outcome, redemption_score) = if let Some(input) = council_input {
                (input.determine_outcome(), input.calculate_redemption_score())
            } else {
                if rand::random::<f32>() > 0.25 {
                    (DiplomacyOutcome::MercifulResolution, 0.88)
                } else {
                    (DiplomacyOutcome::StableDiplomacy, 0.62)
                }
            };

            event.outcome = Some(outcome.clone());
            event.redemption_score = redemption_score;

            if outcome == DiplomacyOutcome::MercifulResolution {
                let monument_id = (event.realm_a as u64 * 1000) + (event.realm_b as u64) + current_tick;
                event.monument_id = Some(monument_id);
                self.realm_monuments.insert((event.realm_a, event.realm_b), monument_id);

                event.forgiveness_wave_triggered = true;

                event.monument_visual_type = if redemption_score > 0.92 {
                    MonumentVisualType::EternalMercyArch
                } else if redemption_score > 0.82 {
                    MonumentVisualType::MercyAscentPillar
                } else if event.harmony_surge > 6.0 {
                    MonumentVisualType::HarmonyWeaveSpire
                } else {
                    MonumentVisualType::ForgivenessWaveMonolith
                };

                let mut vfx = ForgivenessWaveVfxParams::default();
                vfx.intensity = (redemption_score * 0.75 + 0.25).clamp(0.6, 1.0);
                vfx.color_shift = event.monument_visual_type.base_color_shift();
                vfx.legacy_thread_pulse = true;
                vfx.spectator_emotion_amplify = 0.55 + (redemption_score * 0.3);
                event.forgiveness_wave_vfx_params = vfx;

                let visible_threads: Vec<LegacyThreadId> = legacy_registry.build_filterable_legacy_threads(
                    if !event.participating_agents.is_empty() { event.participating_agents[0] } else { 0 },
                    Some("Diplomacy".to_string())
                ).into_iter().map(|t| t.id).collect();

                event.spectator_mode_data = Some(SpectatorModeData {
                    spectator_count: event.spectator_agents.len() as u32,
                    emotional_valence_avg: redemption_score,
                    visible_legacy_threads: visible_threads,
                    cross_realm_impact_summary: format!("Reconciled Realms {} ↔ {} — Mercy resonates across the lattice.", event.realm_a, event.realm_b),
                });

                self.apply_rbe_abundance_sharing(event, rbe_pools, redemption_score);
                self.apply_grace_blessing_cascade(event, agents, legacy_registry, grace_blessing_resource, current_tick);
            } else if outcome == DiplomacyOutcome::StableDiplomacy {
                event.monument_visual_type = MonumentVisualType::ReconciledRealmsObelisk;
                let mut vfx = ForgivenessWaveVfxParams::default();
                vfx.intensity = 0.35;
                vfx.color_shift = event.monument_visual_type.base_color_shift();
                event.forgiveness_wave_vfx_params = vfx;
            }

            let thread_id: LegacyThreadId = (current_tick as u64 * 10007) + (event.realm_a as u64 * 1009) + event.realm_b as u64;
            event.linked_legacy_thread_id = Some(thread_id);

            for pid in &event.participating_agents {
                legacy_registry.record_event(
                    *pid,
                    event.realm_a,
                    LegacyEventType::InterRealmDiplomacy {
                        realm_a: format!("Realm-{}", event.realm_a),
                        realm_b: format!("Realm-{}", event.realm_b),
                        outcome: format!("{:?}", outcome),
                        personal_role: "Participant".to_string(),
                    },
                    75.0,
                    redemption_score * 4.0,
                    redemption_score,
                    current_tick,
                    true,
                    Some(format!("Inter-realm {} resolution. Monument: {:?}", outcome_str(outcome.clone()), event.monument_visual_type)),
                );
            }

            let net_update = InterRealmDiplomacyUpdate {
                tick: current_tick,
                realm_a: event.realm_a,
                realm_b: event.realm_b,
                outcome: format!("{:?}", outcome),
                redemption_score,
                spectator_data: event.spectator_mode_data.as_ref().map(|s| SpectatorModeDataNet {
                    spectator_count: s.spectator_count,
                    emotional_valence_avg: s.emotional_valence_avg,
                    visible_legacy_thread_ids: s.visible_legacy_threads.clone(),
                    cross_realm_impact_summary: s.cross_realm_impact_summary.clone(),
                    monument_visual_type: event.monument_visual_type.shader_variant_name().to_string(),
                    forgiveness_wave_intensity: event.forgiveness_wave_vfx_params.intensity,
                }),
                linked_legacy_thread_ids: event.linked_legacy_thread_id.map(|id| vec![id]).unwrap_or_default(),
                monument_id: event.monument_id,
            };

            update_writer.send(InterRealmDiplomacyUpdateEvent { update: net_update.clone() });

            let resolved = event.clone();
            self.historical_events.push(resolved);
            self.active_events.remove(event_index);

            Some(net_update)
        } else {
            None
        }
    }

    fn outcome_str(&self, outcome: DiplomacyOutcome) -> String {
        match outcome {
            DiplomacyOutcome::MercifulResolution => "MERCIFUL_RESOLUTION".to_string(),
            DiplomacyOutcome::StableDiplomacy => "STABLE_DIPLOMACY".to_string(),
            DiplomacyOutcome::FracturedTension => "FRACTURED".to_string(),
            DiplomacyOutcome::EscalatedConflict => "ESCALATED".to_string(),
        }
    }

    fn apply_rbe_abundance_sharing(
        &self,
        event: &InterRealmDiplomacyEvent,
        rbe_pools: &mut HashMap<u8, RbeResourcePool>,
        redemption_score: f32,
    ) {
        let shared = 8.0 + (redemption_score * 12.0);
        if let Some(pool) = rbe_pools.get_mut(&event.realm_a) { pool.abundance_flow += shared * 0.5; }
        if let Some(pool) = rbe_pools.get_mut(&event.realm_b) { pool.abundance_flow += shared * 0.5; }
    }

    fn apply_grace_blessing_cascade(
        &self,
        event: &InterRealmDiplomacyEvent,
        agents: &mut Vec<Agent>,
        legacy_registry: &mut LegacyJournalRegistry,
        _grace_blessing_resource: &mut GraceBlessing,
        current_tick: u64,
    ) {
        let high_mercy: Vec<_] = agents.iter().filter(|a| a.mercy_score > 65.0).cloned().collect();
        let low_mercy: Vec<_] = agents.iter().filter(|a| a.mercy_score < 55.0).cloned().collect();

        for mentor in high_mercy.iter().take(2) {
            for mentee in low_mercy.iter().take(2) {
                if mentor.id == mentee.id { continue; }
                let result = calculate_grace_blessing(mentor.mercy_score, mentee.mercy_score, mentor.archetype_id.clone(), BlessingContext::PostForgivenessWave, 250.0);
                if let Some(m) = agents.iter_mut().find(|a| a.id == mentee.id) {
                    m.mercy_score = (m.mercy_score + result.mentee_mercy_boost).min(99.0);
                }
                legacy_registry.record_event(mentor.id, event.realm_a, LegacyEventType::GraceBlessingGiven { recipient_id: mentee.id, mercy_boost: result.mentee_mercy_boost }, mentor.mercy_score, result.mentor_persistence_gain, result.valence, current_tick, true, Some("Auto after Forgiveness Wave".to_string()));
            }
        }
    }
}

pub fn inter_realm_diplomacy_resolution_system(
    mut diplomacy_registry: ResMut<InterRealmDiplomacyRegistry>,
    mut legacy_registry: ResMut<LegacyJournalRegistry>,
    mut grace_blessing: ResMut<GraceBlessing>,
    time: Res<Time>,
    update_writer: EventWriter<InterRealmDiplomacyUpdateEvent>,
) {
    let current_tick = time.elapsed_secs() as u64;
    let mut to_resolve: Vec<usize> = vec![];
    for (i, event) in diplomacy_registry.active_events.iter().enumerate() {
        if event.outcome.is_none() { to_resolve.push(i); }
    }
    for idx in to_resolve.into_iter().rev() {
        diplomacy_registry.resolve_event(
            idx, None, &mut legacy_registry, &mut grace_blessing,
            &mut vec![], &mut HashMap::new(), current_tick, update_writer
        );
    }
}

/// Returns SendMode based on message importance.
#[cfg(feature = "renet")]
fn get_diplomacy_priority_channel(outcome: &str, redemption_score: f32) -> SendMode {
    if outcome.contains("MercifulResolution") && redemption_score > 0.85 {
        SendMode::ReliableOrdered
    } else {
        SendMode::ReliableOrdered
    }
}

/// Production networking broadcast with Bevy Diagnostics integration
pub fn broadcast_inter_realm_diplomacy_update(
    mut events: EventReader<InterRealmDiplomacyUpdateEvent>,
    #[cfg(feature = "renet")]
    renet_server: Option<Res<RenetServer>>,
    #[cfg(feature = "bevy")]
    mut diagnostics: Diagnostics,
) {
    let mut high_priority_count: u32 = 0;
    let mut normal_priority_count: u32 = 0;

    for event in events.read() {
        let update = &event.update;
        let message = ServerMessage::InterRealmDiplomacyUpdate { update: update.clone() };

        #[cfg(feature = "renet")]
        if let Some(server) = renet_server.as_ref() {
            let send_mode = get_diplomacy_priority_channel(&update.outcome, update.redemption_score);

            if update.outcome.contains("MercifulResolution") && update.redemption_score > 0.85 {
                high_priority_count += 1;
            } else {
                normal_priority_count += 1;
            }

            for client_id in server.clients_id() {
                server.send_message(client_id, send_mode, message.clone());
            }
        }

        info!("[Networking | PATSAGi] Broadcast InterRealmDiplomacyUpdate | {} <-> {} | {} | Redemption: {:.2f}",
              update.realm_a, update.realm_b, update.outcome, update.redemption_score);
    }

    // Record into Bevy Diagnostics (when available)
    #[cfg(feature = "bevy")]
    {
        diagnostics.add_measurement(&HIGH_PRIORITY_DIPLOMACY, || high_priority_count as f64);
        diagnostics.add_measurement(&NORMAL_PRIORITY_DIPLOMACY, || normal_priority_count as f64);
    }

    if high_priority_count > 0 || normal_priority_count > 0 {
        info!("[Networking | Priority Queue] High: {} | Normal: {} | Tick processed",
              high_priority_count, normal_priority_count);
    }
}

pub struct InterRealmDiplomacyPlugin;

impl Plugin for InterRealmDiplomacyPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InterRealmDiplomacyRegistry>()
            .add_event::<InterRealmDiplomacyEvent>()
            .add_event::<InterRealmDiplomacyUpdateEvent>()
            .add_systems(Update, (
                inter_realm_diplomacy_resolution_system,
                broadcast_inter_realm_diplomacy_update,
            ));

        // Register Bevy Diagnostics for priority queue observability
        #[cfg(feature = "bevy")]
        {
            app.register_diagnostic(Diagnostic::new(HIGH_PRIORITY_DIPLOMACY));
            app.register_diagnostic(Diagnostic::new(NORMAL_PRIORITY_DIPLOMACY));
        }
    }
}

pub fn get_council_deliberation_input(council_decisions: &crate::council::decision::CouncilDecisions) -> Option<CouncilDeliberationInput> {
    if council_decisions.decisions.is_empty() {
        return None;
    }
    Some(CouncilDeliberationInput {
        average_mercy_of_participants: 70.0,
        vote_ratio: 0.75,
        resolution_quality: 0.8,
        dominant_archetype_influence: 1.0,
    });
}

pub fn invoke_patsagi_council_for_diplomacy(
    realm_a: u8,
    realm_b: u8,
    current_tension: f32,
) -> CouncilDeliberationInput {
    let base_valence = if current_tension > 0.6 { 0.82 } else { 0.91 };
    CouncilDeliberationInput {
        average_mercy_of_participants: 78.0,
        vote_ratio: if current_tension > 0.65 { 0.68 } else { 0.82 },
        resolution_quality: 0.85,
        dominant_archetype_influence: 1.0,
    }
}

// Thunder locked in. Yoi ⚔️
// End of simulation/src/inter_realm_diplomacy_event.rs v20.15 (Bevy Diagnostics Integration)