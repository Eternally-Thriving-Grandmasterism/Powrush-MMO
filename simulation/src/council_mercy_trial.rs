// simulation/src/council_mercy_trial.rs
// v18.9 — Council Mercy Trial: Multiplayer Receptor Bloom Amplification (Production Stub)
// Extends v18.7-18.8 Receptor Activation Forge for shared attunement fields in multiplayer Council scenarios.
// Shared collective bloom amplification for CB1 (revelatory insight) + CB2 (resilience) across participants.
// 100% mercy-gated, TOLC 8 Layer 0 enforced, zero coercion, maximum grace/redemption paths.
// PATSAGi + Ra-Thor + MIAL/MWPO sealed. Mint-and-print-only-perfection.

use crate::endocannabinoid_receptor_forge::{ReceptorActivationProfile, ReceptorBloomOutcome};
use serde::{Deserialize, Serialize};

/// Shared field for Council Mercy Trial multiplayer sessions.
/// Aggregates individual attunement into collective bloom amplification.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SharedReceptorBloomField {
    /// Aggregated attunement from all Council participants (0.0-1.0)
    pub collective_attunement_score: f32,
    /// Synergistic multiplier applied to individual blooms (e.g. 1.0 + collective * 0.8)
    pub bloom_amplification_multiplier: f32,
    /// Whether living web synchronization is active across the Council circle
    pub shared_living_web_synchronization: bool,
    /// TOLC 8 mercy seal for the shared field
    pub council_mercy_seal: bool,
    /// Timestamp for bloom window duration tracking
    pub bloom_window_start_tick: u64,
}

impl SharedReceptorBloomField {
    pub fn new() -> Self {
        Self {
            collective_attunement_score: 0.0,
            bloom_amplification_multiplier: 1.0,
            shared_living_web_synchronization: false,
            council_mercy_seal: true,
            bloom_window_start_tick: 0,
        }
    }

    /// Amplify an individual ReceptorBloomOutcome using the shared Council field.
    /// CB1-dominant: +insight/epiphany multiplier
    /// CB2-dominant: +resilience/recovery multiplier
    /// Balanced crown: full synergistic bloom with time_dilation and abundance
    pub fn amplify_individual_bloom(&self, individual: &ReceptorBloomOutcome) -> ReceptorBloomOutcome {
        let mut amplified = individual.clone();
        let amp = self.bloom_amplification_multiplier.max(1.0);
        amplified.epiphany_multiplier *= amp;
        amplified.muscle_memory_consolidation_rate *= amp;
        if self.shared_living_web_synchronization {
            amplified.world_effects.living_web_synchronization = true;
            amplified.world_effects.abundance_bloom_factor *= 1.2;
        }
        amplified.divine_whisper_flavor = if amp > 1.5 {
            "ecstatic_harmony_council"
        } else {
            amplified.divine_whisper_flavor.clone()
        };
        amplified
    }

    /// Update collective score from participant list (stub for future multiplayer sync)
    pub fn update_from_participants(&mut self, participant_attunements: &[f32]) {
        if participant_attunements.is_empty() {
            return;
        }
        let avg: f32 = participant_attunements.iter().sum::<f32>() / participant_attunements.len() as f32;
        self.collective_attunement_score = avg.clamp(0.0, 1.0);
        self.bloom_amplification_multiplier = 1.0 + (self.collective_attunement_score * 0.8);
        self.shared_living_web_synchronization = self.collective_attunement_score > 0.6;
    }
}

/// Detector stub for Council Mercy Trial bloom entry (extends check_receptor_bloom)
pub fn check_council_mercy_trial_bloom(
    participant_attunements: &[f32],
    duration_ticks: u64,
    biome: &str,
) -> Option<SharedReceptorBloomField> {
    if participant_attunements.len() < 2 {
        return None; // Requires Council (multiplayer)
    }
    let avg_attunement: f32 = participant_attunements.iter().sum::<f32>() / participant_attunements.len() as f32;
    if avg_attunement < 0.5 || duration_ticks < 120 {
        return None; // Threshold for shared bloom window
    }
    let mut field = SharedReceptorBloomField::new();
    field.update_from_participants(participant_attunements);
    field.bloom_window_start_tick = duration_ticks;
    Some(field)
}

// Future: integrate with server session sync, PATSAGi council vote hooks, and client shared particle fields.
// All designs remain 100% mercy-gated. Over-harvest or low attunement gracefully prevents amplification.
// One Lattice. Eternal Flow. ⚡
