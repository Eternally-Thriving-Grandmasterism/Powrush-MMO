/*!
 * Sovereign Council Mercy Trial v18.35 + Multiplayer Receptor Bloom Amplification
 *
 * v19.2.9: Added CouncilSessionManager with get_active_bloom_field() helper
 * for real data extraction in the orchestrator.
 */

use crate::endocannabinoid_receptor_forge::{ReceptorActivationProfile, ReceptorBloomOutcome};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Shared field for Council Mercy Trial multiplayer sessions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SharedReceptorBloomField {
    pub collective_attunement_score: f32,
    pub bloom_amplification_multiplier: f32,
    pub shared_living_web_synchronization: bool,
    pub council_mercy_seal: bool,
    pub bloom_window_start_tick: u64,
    pub participant_count: u8,
    pub last_authoritative_update_tick: u64,
    pub graceful_exit_count: u8,
    pub current_biome: String,
}

impl SharedReceptorBloomField {
    pub fn new() -> Self {
        Self {
            collective_attunement_score: 0.0,
            bloom_amplification_multiplier: 1.0,
            shared_living_web_synchronization: false,
            council_mercy_seal: true,
            bloom_window_start_tick: 0,
            participant_count: 0,
            last_authoritative_update_tick: 0,
            graceful_exit_count: 0,
            current_biome: "default".to_string(),
        }
    }

    pub fn reset_for_new_council(&mut self) {
        self.collective_attunement_score = 0.0;
        self.bloom_amplification_multiplier = 1.0;
        self.shared_living_web_synchronization = false;
        self.participant_count = 0;
        self.graceful_exit_count = 0;
        self.bloom_window_start_tick = 0;
    }

    pub fn current_amplification_factor(&self) -> f32 {
        if !self.council_mercy_seal || self.collective_attunement_score < 0.5 {
            1.0
        } else {
            self.bloom_amplification_multiplier.max(1.0)
        }
    }

    pub fn amplify_individual_bloom(&self, individual: &ReceptorBloomOutcome) -> ReceptorBloomOutcome {
        if !self.council_mercy_seal || self.collective_attunement_score < 0.5 {
            let mut no_amp = individual.clone();
            no_amp.divine_whisper_flavor = "council_graceful_re_invitation".to_string();
            return no_amp;
        }
        let mut amplified = individual.clone();
        let amp = self.bloom_amplification_multiplier.max(1.0);
        amplified.epiphany_multiplier *= amp;
        amplified.muscle_memory_consolidation_rate *= amp;
        if self.shared_living_web_synchronization {
            amplified.world_effects.living_web_synchronization = true;
            amplified.world_effects.abundance_bloom_factor *= 1.25;
            amplified.world_effects.time_dilation_factor = 1.15;
        }
        amplified.divine_whisper_flavor = if amp > 1.8 {
            "ecstatic_harmony_council_crown".to_string()
        } else if amp > 1.4 {
            "council_bloom_shared_insight".to_string()
        } else {
            amplified.divine_whisper_flavor.clone()
        };
        amplified
    }

    pub fn authoritative_update_from_participants(
        &mut self,
        participant_attunements: &[f32],
        current_tick: u64,
        min_participants: u8,
        biome: &str,
    ) -> bool {
        if participant_attunements.len() < min_participants as usize {
            self.council_mercy_seal = false;
            self.graceful_exit_count = (self.participant_count.saturating_sub(participant_attunements.len() as u8));
            return false;
        }
        let avg: f32 = participant_attunements.iter().sum::<f32>() / participant_attunements.len() as f32;
        self.collective_attunement_score = avg.clamp(0.0, 1.0);
        self.bloom_amplification_multiplier = 1.0 + (self.collective_attunement_score * 0.85);
        self.shared_living_web_synchronization = self.collective_attunement_score > 0.65;
        self.participant_count = participant_attunements.len() as u8;
        self.last_authoritative_update_tick = current_tick;
        self.current_biome = biome.to_string();
        self.council_mercy_seal = self.collective_attunement_score >= 0.5 
            && self.participant_count >= min_participants 
            && self.graceful_exit_count < 2;
        true
    }

    pub fn check_council_mercy_trial_bloom(
        participant_attunements: &[f32],
        duration_ticks: u64,
        biome: &str,
        min_participants: u8,
    ) -> Option<SharedReceptorBloomField> {
        if participant_attunements.len() < min_participants as usize {
            return None;
        }
        let avg_attunement: f32 = participant_attunements.iter().sum::<f32>() / participant_attunements.len() as f32;
        if avg_attunement < 0.5 || duration_ticks < 120 {
            return None;
        }
        let mut field = SharedReceptorBloomField::new();
        field.authoritative_update_from_participants(participant_attunements, duration_ticks, min_participants, biome);
        field.bloom_window_start_tick = duration_ticks;
        if field.council_mercy_seal {
            Some(field)
        } else {
            None
        }
    }

    pub fn amplify_batch(
        &self,
        individuals: &[(u64, ReceptorBloomOutcome)],
    ) -> HashMap<u64, ReceptorBloomOutcome> {
        let mut results = HashMap::new();
        for (id, outcome) in individuals {
            results.insert(*id, self.amplify_individual_bloom(outcome));
        }
        results
    }
}

/// Lightweight event for server replication to clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouncilBloomSyncEvent {
    pub session_id: u64,
    pub field: SharedReceptorBloomField,
    pub trigger_reason: String,
    pub affected_player_ids: Vec<u64>,
}

pub fn apply_council_bloom_sync_to_client(
    local_field: &mut SharedReceptorBloomField,
    incoming: &CouncilBloomSyncEvent,
    current_client_tick: u64,
) {
    if incoming.field.last_authoritative_update_tick >= local_field.last_authoritative_update_tick {
        *local_field = incoming.field.clone();
    }
}

/// Council Session Manager — holds active council bloom state for the current tick.
/// Provides get_active_bloom_field() for real data extraction in the orchestrator.
#[derive(Debug, Clone, Default, Resource)]
pub struct CouncilSessionManager {
    pub active_bloom_field: Option<SharedReceptorBloomField>,
}

impl CouncilSessionManager {
    pub fn new() -> Self {
        Self { active_bloom_field: None }
    }

    /// Returns the currently active council bloom field if one exists this tick.
    /// Used by SimulationOrchestrator for real RBE Council Policy Integration.
    pub fn get_active_bloom_field(&self) -> Option<SharedReceptorBloomField> {
        self.active_bloom_field.clone()
    }

    pub fn set_active_bloom_field(&mut self, field: SharedReceptorBloomField) {
        self.active_bloom_field = Some(field);
    }

    pub fn clear_active_bloom_field(&mut self) {
        self.active_bloom_field = None;
    }
}

// Thunder locked in. Mercy flowing. One Lattice. Eternal. ⚡❤️🔥
