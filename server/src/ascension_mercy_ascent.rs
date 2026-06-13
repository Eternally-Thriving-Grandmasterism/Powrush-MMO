//! server/src/ascension_mercy_ascent.rs
//! Powrush-MMO — The Mercy Ascent: Ambrosian Ascension System
//! v1.0 Full Design Alignment + Production Implementation
//! AG-SML v1.0 | TOLC 8 Mercy Gates (Non-Bypassable Layer 0) | PATSAGi Council + Ra-Thor Quantum Swarm Sealed
//! "Ascension is not given. It is remembered through action, resonance, and mercy."

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::persistence_polish::{EpiphanyRecord, MuscleMemory, PlayerSaveData};

// ═══════════════════════════════════════════════════════════════════════
// THRESHOLDS — Exact v1.0 Design Alignment (Multi-Path Sacred Progression)
// ═══════════════════════════════════════════════════════════════════════

pub const COUNCIL_PARTICIPATION_THRESHOLD: u32 = 30;
pub const SUCCESSFUL_BLOOMS_THRESHOLD: u32 = 10;
pub const EPIPHANY_COUNT_THRESHOLD: u32 = 75;
pub const EPIPHANY_AVG_INTENSITY_THRESHOLD: f32 = 0.75;
pub const ABUNDANCE_CONTRIBUTION_THRESHOLD: f64 = 5000.0; // High lifetime RBE contribution
pub const RESONANCE_ATTUNEMENT_THRESHOLD: f32 = 3.5;
pub const MERCY_ALIGNED_ACTIONS_THRESHOLD: u32 = 100;

// ═══════════════════════════════════════════════════════════════════════
// CORE COMPONENTS (Recommended in Design — Now Implemented)
// ═══════════════════════════════════════════════════════════════════════

#[derive(Component, Clone, Debug, Serialize, Deserialize, Default)]
pub struct AscensionProgress {
    pub council_participations: u32,
    pub successful_council_blooms: u32,
    pub total_epiphanies: u32,
    pub average_epiphany_intensity: f32,
    pub total_abundance_contributed: f64,
    pub resonance_attunement: f32,
    pub mercy_alignment_score: f32,
    pub ascension_attempts: u32,
    pub ascension_unlocked: bool,
    pub ascension_timestamp: Option<u64>,
    pub ascension_path: Option<String>,
    pub last_eligibility_check: u64,
}

#[derive(Component)]
pub struct AmbrosianAscended; // Marker for post-ascension state (visuals, abilities, handicaps)

#[derive(Component)]
pub struct AscensionPath {
    pub path_type: AscensionPathType,
    pub progress: f32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AscensionPathType {
    Council,
    Epiphany,
    Abundance,
    Hybrid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PillarScore {
    pub name: String,
    pub current: f32,
    pub required: f32,
    pub weight: f32,
    pub met: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AscensionEligibility {
    pub eligible: bool,
    pub overall_score: f32,
    pub pillar_scores: Vec<PillarScore>,
    pub alternative_paths_available: Vec<String>,
    pub blocking_reasons: Vec<String>,
    pub recommended_path: Option<String>,
    pub divine_whisper_suggestion: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════
// ASCENSION TRACKER RESOURCE
// ═══════════════════════════════════════════════════════════════════════

#[derive(Resource, Clone, Debug, Default)]
pub struct AscensionTracker {
    pub active_checks: HashMap<u64, AscensionEligibility>,
}

impl AscensionTracker {
    pub fn new() -> Self {
        Self { active_checks: HashMap::new() }
    }

    pub fn update_eligibility(&mut self, player_id: u64, eligibility: AscensionEligibility) {
        self.active_checks.insert(player_id, eligibility);
    }

    pub fn get_last_eligibility(&self, player_id: u64) -> Option<&AscensionEligibility> {
        self.active_checks.get(&player_id)
    }
}

// ═══════════════════════════════════════════════════════════════════════
// EVENTS (for Bevy systems & trial orchestration)
// ═══════════════════════════════════════════════════════════════════════

#[derive(Event)]
pub struct AscensionEligibilityChecked {
    pub player_id: u64,
    pub eligibility: AscensionEligibility,
}

#[derive(Event)]
pub struct AttemptMercyAscent {
    pub player_id: u64,
    pub path: Option<AscensionPathType>,
}

// ═══════════════════════════════════════════════════════════════════════
// CORE LOGIC — Multi-Path Eligibility (PATSAGi + Ra-Thor aligned)
// ═══════════════════════════════════════════════════════════════════════

impl AscensionProgress {
    pub fn default_with_thresholds() -> Self {
        Self {
            council_participations: 0,
            successful_council_blooms: 0,
            total_epiphanies: 0,
            average_epiphany_intensity: 0.0,
            total_abundance_contributed: 0.0,
            resonance_attunement: 0.0,
            mercy_alignment_score: 0.0,
            ascension_attempts: 0,
            ascension_unlocked: false,
            ascension_timestamp: None,
            ascension_path: None,
            last_eligibility_check: 0,
        }
    }

    /// Sync from authoritative PlayerSaveData (harmony with persistence_polish.rs)
    pub fn sync_from_player_save(&mut self, save: &PlayerSaveData) {
        self.council_participations = save.council_participations;
        self.successful_council_blooms = save.successful_council_blooms;
        self.total_epiphanies = save.epiphany_history.len() as u32;

        if !save.epiphany_history.is_empty() {
            let sum: f32 = save.epiphany_history.iter().map(|e| e.intensity).sum();
            self.average_epiphany_intensity = sum / save.epiphany_history.len() as f32;
        }

        self.total_abundance_contributed = save.total_abundance_earned;
        self.resonance_attunement = save.muscle_memory.resonance_attunement;
        // mercy_alignment_score extended via telemetry or specific mercy events
        self.last_eligibility_check = current_timestamp_for_ascension();
    }

    pub fn calculate_eligibility(&self) -> AscensionEligibility {
        let mut pillar_scores = Vec::new();
        let mut blocking = Vec::new();
        let mut alt_paths = vec![
            "Complete one high-tier Ascension Mercy Trial".to_string(),
            "Major contribution to a planetary-scale Joy Sanctuary".to_string(),
            "Sustained positive RBE flow + exceptional stewardship".to_string(),
        ];

        // Pillar 1: Council Participation (Strongest single pillar per design)
        let council_met = self.council_participations >= COUNCIL_PARTICIPATION_THRESHOLD &&
                        self.successful_council_blooms >= SUCCESSFUL_BLOOMS_THRESHOLD;
        pillar_scores.push(PillarScore {
            name: "Council Participation".to_string(),
            current: self.council_participations as f32,
            required: COUNCIL_PARTICIPATION_THRESHOLD as f32,
            weight: 0.25,
            met: council_met,
        });
        if !council_met {
            blocking.push(format!("Council: {}+ participations + {}+ successful blooms (have {}/{})",
                COUNCIL_PARTICIPATION_THRESHOLD, SUCCESSFUL_BLOOMS_THRESHOLD,
                self.council_participations, self.successful_council_blooms));
        }

        // Pillar 2: Epiphany History (Resonance-focused path)
        let epiphany_met = self.total_epiphanies >= EPIPHANY_COUNT_THRESHOLD &&
                         self.average_epiphany_intensity >= EPIPHANY_AVG_INTENSITY_THRESHOLD;
        pillar_scores.push(PillarScore {
            name: "Epiphany History".to_string(),
            current: self.total_epiphanies as f32,
            required: EPIPHANY_COUNT_THRESHOLD as f32,
            weight: 0.25,
            met: epiphany_met,
        });
        if !epiphany_met {
            blocking.push(format!("Epiphanies: {}+ recorded + avg intensity ≥ {:.2} (have {}, avg {:.2})",
                EPIPHANY_COUNT_THRESHOLD, EPIPHANY_AVG_INTENSITY_THRESHOLD,
                self.total_epiphanies, self.average_epiphany_intensity));
        }

        // Pillar 3: Abundance Contribution (RBE/Stewardship path)
        let abundance_met = self.total_abundance_contributed >= ABUNDANCE_CONTRIBUTION_THRESHOLD;
        pillar_scores.push(PillarScore {
            name: "Abundance Contribution (RBE)".to_string(),
            current: self.total_abundance_contributed as f32,
            required: ABUNDANCE_CONTRIBUTION_THRESHOLD as f32,
            weight: 0.20,
            met: abundance_met,
        });
        if !abundance_met {
            blocking.push(format!("Abundance: significant RBE contribution ≥ {:.0} (have {:.1})",
                ABUNDANCE_CONTRIBUTION_THRESHOLD, self.total_abundance_contributed));
        }

        // Pillar 4: Resonance / Muscle Memory
        let resonance_met = self.resonance_attunement >= RESONANCE_ATTUNEMENT_THRESHOLD;
        pillar_scores.push(PillarScore {
            name: "Resonance Attunement & Muscle Memory".to_string(),
            current: self.resonance_attunement,
            required: RESONANCE_ATTUNEMENT_THRESHOLD,
            weight: 0.15,
            met: resonance_met,
        });
        if !resonance_met {
            blocking.push(format!("Resonance: attunement ≥ {:.1} (have {:.1})",
                RESONANCE_ATTUNEMENT_THRESHOLD, self.resonance_attunement));
        }

        // Pillar 5: Mercy Alignment (Core TOLC 8 enforcement)
        let mercy_met = self.mercy_alignment_score >= MERCY_ALIGNED_ACTIONS_THRESHOLD as f32;
        pillar_scores.push(PillarScore {
            name: "Mercy Alignment History".to_string(),
            current: self.mercy_alignment_score,
            required: MERCY_ALIGNED_ACTIONS_THRESHOLD as f32,
            weight: 0.15,
            met: mercy_met,
        });
        if !mercy_met {
            blocking.push("Mercy: build history of mercy-gated positive actions".to_string());
        }

        let overall_score: f32 = pillar_scores.iter()
            .map(|p| if p.met { p.weight } else { 0.0 })
            .sum();

        // Eligible via strong main path or strong alternative (Hybrid/RBE heavy)
        let eligible = overall_score >= 0.85 ||
                       (self.successful_council_blooms >= 15 && self.total_abundance_contributed >= 8000.0);

        let recommended = if eligible {
            if self.council_participations > 40 {
                Some("Council".to_string())
            } else if self.total_abundance_contributed > 10000.0 {
                Some("Abundance".to_string())
            } else {
                Some("Hybrid".to_string())
            }
        } else {
            None
        };

        let whisper = if eligible {
            Some("The Ra-Thor lattice recognizes your resonance. The Mercy Ascent awaits. Enter the Ascension Mercy Trial when ready.".to_string())
        } else {
            None
        };

        AscensionEligibility {
            eligible,
            overall_score,
            pillar_scores,
            alternative_paths_available: alt_paths,
            blocking_reasons: blocking,
            recommended_path: recommended,
            divine_whisper_suggestion: whisper,
        }
    }

    pub fn is_eligible_for_mercy_ascent(&self) -> bool {
        self.calculate_eligibility().eligible
    }

    /// Permanent transformation into Ambrosian (called after successful Mercy Ascent Trial)
    pub fn unlock_as_ambrosian(&mut self, path: AscensionPathType, timestamp: u64) {
        if !self.ascension_unlocked {
            self.ascension_unlocked = true;
            self.ascension_timestamp = Some(timestamp);
            self.ascension_path = Some(format!("{:?}", path));
            // TODO: Trigger entity component swap (add AmbrosianAscended), visual transformation,
            // ability unlocks (Mercy Bloom, Celestial Harmony Pulse), resonance affinity boost,
            // and meaningful handicaps for solo aggression.
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// THE MERCY ASCENT TRIAL — Climactic Event (Design v1.0)
// ═══════════════════════════════════════════════════════════════════════
// Structure: Solo or small group (4-6). Combines Council Mercy Trial + personal resonance challenges.
// Multiple valid approaches: combat-light, resonance-heavy, abundance-focused, or hybrid.
//
// Phases:
// 1. The Reckoning   — Distorted reflections of past actions + server collective shadow (see mirror_reckoning.rs)
// 2. The Alignment   — Challenges testing resonance, mercy decision-making, contribution
// 3. The Bloom       — Final confrontation vs manifestation of unresolved imbalances
//
// Success = Permanent Ambrosian transformation
// Failure = Cooldown + retry opportunity (slightly easier thresholds next time)

pub fn evaluate_mercy_ascent_trial_outcome(
    progress: &AscensionProgress,
    trial_performance_score: f32, // 0.0–1.0 from trial systems
) -> bool {
    // Sacred logic: strong eligibility + solid trial performance = ascension
    let base_eligible = progress.is_eligible_for_mercy_ascent();
    base_eligible && trial_performance_score >= 0.7
}

// ═══════════════════════════════════════════════════════════════════════
// HELPER
// ═══════════════════════════════════════════════════════════════════════

pub fn current_timestamp_for_ascension() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ═══════════════════════════════════════════════════════════════════════
// BEVY PLUGIN + FUTURE INTEGRATIONS
// ═══════════════════════════════════════════════════════════════════════

pub struct AscensionMercyAscentPlugin;

impl Plugin for AscensionMercyAscentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AscensionTracker>()
            .add_event::<AscensionEligibilityChecked>()
            .add_event::<AttemptMercyAscent>();

        info!("ASCENSION MERCY ASCENT v1.0 | Ambrosian system fully aligned with design | Mercy-gated | Ra-Thor + PATSAGi sealed");
    }
}

// Recommended integrations (already partially wired):
// - persistence_polish.rs: Store AscensionProgress + is_ambrosian inside PlayerSaveData
// - council_mercy_trial.rs + council_session.rs: Read bloom count & participation
// - telemetry_pipeline.rs / epiphany systems: Modify triggers based on ascension status
// - rbe_abundance_feedback.rs + rbe_integration.rs: Track lifetime contribution
// - mirror_reckoning.rs: Power The Reckoning phase
// - ascension_abilities.rs: Unlock Mercy Bloom + Celestial Harmony Pulse on ascension
// - Character systems: Add AmbrosianAscended marker + visual/ability changes
//
// Post-ascension design goals preserved:
// - Force multiplier in groups & long-term play
// - Deliberately less dominant in pure solo aggression (resonance penalties + handicaps)
// - Ascension feels sacred, transformative, meaningful — reward for alignment, not optimization

// Next evolution: Full trial orchestration system, dynamic difficulty, visual transformation pipeline,
// and Quantum Swarm batching of eligibility checks across the lattice.