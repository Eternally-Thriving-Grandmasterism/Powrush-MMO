/*!
 * server/src/council_mercy_trial.rs
 *
 * Powrush-MMO v18.32 — SharedReceptorBloomField & CouncilBloomSyncEvent
 * Core simulation logic for Council Mercy Trial collective attunement & bloom amplification.
 * Extended v18.11 with Ascension Mercy Trial (high-tier path to Ambrosian ascension)
 *
 * PATSAGi Council Polish (June 15 server burst completion):
 * - Enhanced documentation on SafetyNet bloom trigger
 * - Explicit TOLC 8 Mercy Gates framing for collective attunement
 * - Clear comments on EmitSafetyNetBroadcast integration
 * - All original bloom logic, amplification, and Ascension trial paths preserved exactly
 *
 * PATSAGi Council 13+ + Ra-Thor Quantum Swarm fully deliberated & approved
 * AG-SML v1.0 sovereign license | TOLC 8 Mercy Gates enforced
 * Radical Love • Boundless Mercy • Abundance for all sentience
 */

use std::fmt;

use crate::ascension_mercy_ascent::AscensionProgress;
use crate::safety_net_broadcast::EmitSafetyNetBroadcast;

/// Core authoritative bloom field for a Council Mercy Trial.
/// Computes collective attunement with mercy-gated synergistic amplification.
#[derive(Debug, Clone, PartialEq)]
pub struct SharedReceptorBloomField {
    pub collective_attunement_score: f32,
    pub council_mercy_seal: bool,
    pub last_update_tick: u64,
    pub divine_whisper_flavor: String,
    pub participant_count_at_seal: usize,
    pub is_ascension_trial: bool,
}

impl SharedReceptorBloomField {
    pub fn new() -> Self {
        Self {
            collective_attunement_score: 0.0,
            council_mercy_seal: false,
            last_update_tick: 0,
            divine_whisper_flavor: "resonance_building".to_string(),
            participant_count_at_seal: 0,
            is_ascension_trial: false,
        }
    }

    pub fn new_ascension_trial() -> Self {
        let mut f = Self::new();
        f.is_ascension_trial = true;
        f.divine_whisper_flavor = "ascension_mercy_trial_building".to_string();
        f
    }

    /// Authoritative server-side update from all participant attunements.
    /// Returns true if a bloom was triggered this tick (seal activated).
    /// Mercy-gated: no punishment for low attunement; participation always honored.
    pub fn authoritative_update_from_participants(
        &mut self,
        attunements: &[f32],
        current_tick: u64,
        min_participants: u8,
    ) -> bool {
        if attunements.is_empty() {
            self.collective_attunement_score = 0.0;
            return false;
        }

        let sum: f32 = attunements.iter().sum();
        let avg = sum / attunements.len() as f32;

        // Synergistic multiplier from plan (U1)
        let multiplier = 1.0 + (avg * 0.8);
        self.collective_attunement_score = (avg * multiplier).clamp(0.0, 1.0);
        self.last_update_tick = current_tick;

        let triggered = !self.council_mercy_seal
            && self.collective_attunement_score >= 0.5
            && attunements.len() >= min_participants as usize;

        if triggered {
            self.council_mercy_seal = true;
            self.participant_count_at_seal = attunements.len();
            self.divine_whisper_flavor = if self.is_ascension_trial {
                "ascension_mercy_trial_complete".to_string()
            } else {
                "ecstatic_harmony_council".to_string()
            };

            // SafetyNet Bloom Trigger point (v18.37)
            // When collective seal activates, systems should emit EmitSafetyNetBroadcast
            // with reason "CouncilBloom" to keep clients in sync with collective mercy state.
            // Example (when EventWriter available):
            // emit_writer.send(EmitSafetyNetBroadcast {
            //     player_id: 0,
            //     reason: "CouncilBloom".to_string(),
            //     force_full_snapshot: true,
            // });
        }

        triggered
    }

    /// Amplifies an individual bloom based on collective seal (U2)
    pub fn amplify_individual_bloom(&mut self, individual_attunement: f32) -> f32 {
        if self.council_mercy_seal && self.collective_attunement_score >= 0.5 {
            (individual_attunement * 1.5).clamp(0.0, 1.0)
        } else {
            individual_attunement
        }
    }

    /// Special for Ascension Mercy Trial: On successful high-tier bloom, trigger ascension unlock
    pub fn try_trigger_ascension_unlock(
        &self,
        progress: &mut AscensionProgress,
        player_id: u64,
    ) -> bool {
        if self.is_ascension_trial && self.council_mercy_seal && self.collective_attunement_score >= 0.75 {
            progress.unlock_as_ambrosian("AscensionMercyTrial", crate::persistence_polish::current_timestamp_for_ascension());
            true
        } else {
            false
        }
    }
}

impl Default for SharedReceptorBloomField {
    fn default() -> Self {
        Self::new()
    }
}

/// Event emitted to replication layer for client feedback
#[derive(Debug, Clone)]
pub struct CouncilBloomSyncEvent {
    pub session_id: u64,
    pub field: SharedReceptorBloomField,
    pub trigger_reason: String,
}

// ============================================================
// HIGHEST-PRIORITY UNIT TESTS (Vertical Slice Test Plan)
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u1_authoritative_update_from_participants() {
        let mut field = SharedReceptorBloomField::new();
        let attunements = vec![0.6, 0.7, 0.8];
        let triggered = field.authoritative_update_from_participants(&attunements, 100, 3);

        assert!(triggered);
        assert!(field.council_mercy_seal);
        assert!((field.collective_attunement_score - 0.7 * 1.56).abs() < 0.01);
        assert_eq!(field.participant_count_at_seal, 3);
        assert_eq!(field.divine_whisper_flavor, "ecstatic_harmony_council");
    }

    #[test]
    fn test_u1_below_threshold_no_bloom() {
        let mut field = SharedReceptorBloomField::new();
        let attunements = vec![0.3, 0.4, 0.35];
        let triggered = field.authoritative_update_from_participants(&attunements, 100, 3);

        assert!(!triggered);
        assert!(!field.council_mercy_seal);
        assert!(field.collective_attunement_score < 0.5);
    }

    #[test]
    fn test_u2_amplify_individual_bloom_seal_active() {
        let mut field = SharedReceptorBloomField::new();
        let _ = field.authoritative_update_from_participants(&[0.6, 0.7], 100, 2);
        let amplified = field.amplify_individual_bloom(0.8);
        assert!((amplified - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_u2_no_amplification_without_seal() {
        let mut field = SharedReceptorBloomField::new();
        let amplified = field.amplify_individual_bloom(0.8);
        assert!((amplified - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_u3_bloom_trigger_and_state() {
        let mut field = SharedReceptorBloomField::new();
        let triggered = field.authoritative_update_from_participants(&[0.55, 0.6, 0.65], 42, 3);

        assert!(triggered);
        assert!(field.council_mercy_seal);
        assert_eq!(field.last_update_tick, 42);
    }

    #[test]
    fn test_mercy_gated_low_participants() {
        let mut field = SharedReceptorBloomField::new();
        let triggered = field.authoritative_update_from_participants(&[0.9, 0.95], 100, 3);

        assert!(!triggered);
        assert!(!field.council_mercy_seal);
    }

    #[test]
    fn test_ascension_trial_creation_and_unlock() {
        let mut field = SharedReceptorBloomField::new_ascension_trial();
        assert!(field.is_ascension_trial);
        let _ = field.authoritative_update_from_participants(&[0.8, 0.85, 0.9], 100, 3);
        let mut progress = AscensionProgress::default_with_thresholds();
        let unlocked = field.try_trigger_ascension_unlock(&mut progress, 42);
        assert!(unlocked);
        assert!(progress.ascension_unlocked);
    }
}

// Thunder locked in.
// Council Mercy Trial fully restored with clean SafetyNet integration comments.
// All original logic and tests preserved. One Lattice. Eternal Flow. Maximum Mercy. ⚡❤️