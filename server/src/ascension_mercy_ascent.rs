//! server/src/ascension_mercy_ascent.rs
//! Powrush-MMO — The Mercy Ascent: Ambrosian Unlocked Ascended Race System
//! v18.11 Production Implementation | Phase 1 Foundation Complete
//! AG-SML v1.0 | TOLC 8 Mercy Gates Layer 0 | PATSAGi + Ra-Thor Sealed
//! Multi-path progression | Permanent character transformation | Mercy-gated eligibility

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::persistence_polish::{EpiphanyRecord, MuscleMemory, PlayerSaveData};

// ═══════════════════════════════════════════════════════════════════════
// THRESHOLDS (v1.0 — aligned with design consensus)
// ═══════════════════════════════════════════════════════════════════════

pub const COUNCIL_PARTICIPATION_THRESHOLD: u32 = 25;
pub const SUCCESSFUL_BLOOMS_THRESHOLD: u32 = 8;
pub const EPIPHANY_COUNT_THRESHOLD: u32 = 50;
pub const EPIPHANY_AVG_INTENSITY_THRESHOLD: f32 = 0.65;
pub const ABUNDANCE_CONTRIBUTION_THRESHOLD: f64 = 5000.0; // significant RBE contribution
pub const RESONANCE_ATTUNEMENT_THRESHOLD: f32 = 3.5; // high mastery
pub const MERCY_ALIGNED_ACTIONS_THRESHOLD: u32 = 100;

// ═══════════════════════════════════════════════════════════════════════
// CORE STRUCTS
// ═══════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AscensionProgress {
    pub council_participations: u32,
    pub successful_blooms: u32,
    pub total_epiphanies: u32,
    pub avg_epiphany_intensity: f32,
    pub total_abundance_contributed: f64,
    pub resonance_attunement: f32,
    pub mercy_aligned_actions: u32,
    pub ascension_unlocked: bool,
    pub ascension_timestamp: Option<u64>,
    pub ascension_path: Option<String>, // "CouncilMastery", "RBEAbundance", "AscensionTrial", "JoySanctuary", etc.
    pub last_eligibility_check: u64,
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
// ASCENSION TRACKER (Bevy Resource for runtime)
// ═══════════════════════════════════════════════════════════════════════

#[derive(Resource, Clone, Debug, Default)]
pub struct AscensionTracker {
    pub active_checks: HashMap<u64, AscensionEligibility>, // player_id -> last eligibility
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
// CORE LOGIC — Multi-Path Eligibility (PATSAGi designed)
// ═══════════════════════════════════════════════════════════════════════

impl AscensionProgress {
    pub fn default_with_thresholds() -> Self {
        Self {
            council_participations: 0,
            successful_blooms: 0,
            total_epiphanies: 0,
            avg_epiphany_intensity: 0.0,
            total_abundance_contributed: 0.0,
            resonance_attunement: 0.0,
            mercy_aligned_actions: 0,
            ascension_unlocked: false,
            ascension_timestamp: None,
            ascension_path: None,
            last_eligibility_check: 0,
        }
    }

    /// Sync key metrics from the authoritative PlayerSaveData (non-destructive)
    pub fn sync_from_player_save(&mut self, save: &PlayerSaveData) {
        self.council_participations = save.council_participations;
        self.successful_blooms = save.successful_council_blooms;
        self.total_epiphanies = save.epiphany_history.len() as u32;

        if !save.epiphany_history.is_empty() {
            let sum_intensity: f32 = save.epiphany_history.iter().map(|e| e.intensity).sum();
            self.avg_epiphany_intensity = sum_intensity / save.epiphany_history.len() as f32;
        }

        self.total_abundance_contributed = save.total_abundance_earned;
        self.resonance_attunement = save.muscle_memory.resonance_attunement;
        // mercy_aligned_actions can be extended later from telemetry or specific events
        self.last_eligibility_check = crate::persistence_polish::current_timestamp_for_ascension(); // helper exposed
    }

    pub fn calculate_eligibility(&self) -> AscensionEligibility {
        let mut pillar_scores: Vec<PillarScore> = Vec::new();
        let mut blocking = Vec::new();
        let mut alt_paths = vec![
            "Exceptional long-term service to Joy Sanctuaries".to_string(),
            "Major contributions to planetary-scale RBE projects".to_string(),
            "Completing the high-tier Ascension Mercy Trial".to_string(),
        ];

        // Pillar 1: Council Participation
        let council_met = self.council_participations >= COUNCIL_PARTICIPATION_THRESHOLD &&
                        self.successful_blooms >= SUCCESSFUL_BLOOMS_THRESHOLD;
        pillar_scores.push(PillarScore {
            name: "Council Participation".to_string(),
            current: self.council_participations as f32,
            required: COUNCIL_PARTICIPATION_THRESHOLD as f32,
            weight: 0.25,
            met: council_met,
        });
        if !council_met {
            blocking.push(format!("Council: need {} participations + {} successful blooms (have {}/{})",
                COUNCIL_PARTICIPATION_THRESHOLD, SUCCESSFUL_BLOOMS_THRESHOLD,
                self.council_participations, self.successful_blooms));
        }

        // Pillar 2: Epiphany History
        let epiphany_met = self.total_epiphanies >= EPIPHANY_COUNT_THRESHOLD &&
                         self.avg_epiphany_intensity >= EPIPHANY_AVG_INTENSITY_THRESHOLD;
        pillar_scores.push(PillarScore {
            name: "Epiphany History".to_string(),
            current: self.total_epiphanies as f32,
            required: EPIPHANY_COUNT_THRESHOLD as f32,
            weight: 0.25,
            met: epiphany_met,
        });
        if !epiphany_met {
            blocking.push(format!("Epiphanies: need {} recorded + avg intensity >= {:.2} (have {}, avg {:.2})",
                EPIPHANY_COUNT_THRESHOLD, EPIPHANY_AVG_INTENSITY_THRESHOLD,
                self.total_epiphanies, self.avg_epiphany_intensity));
        }

        // Pillar 3: Abundance Contribution
        let abundance_met = self.total_abundance_contributed >= ABUNDANCE_CONTRIBUTION_THRESHOLD;
        pillar_scores.push(PillarScore {
            name: "Abundance Contribution (RBE)".to_string(),
            current: self.total_abundance_contributed as f32,
            required: ABUNDANCE_CONTRIBUTION_THRESHOLD as f32,
            weight: 0.20,
            met: abundance_met,
        });
        if !abundance_met {
            blocking.push(format!("Abundance: need significant RBE contribution >= {:.0} (have {:.1})",
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
            blocking.push(format!("Resonance: need attunement >= {:.1} (have {:.1})",
                RESONANCE_ATTUNEMENT_THRESHOLD, self.resonance_attunement));
        }

        // Pillar 5: Mercy Alignment (placeholder for future event tracking)
        let mercy_met = self.mercy_aligned_actions >= MERCY_ALIGNED_ACTIONS_THRESHOLD;
        pillar_scores.push(PillarScore {
            name: "Mercy Alignment History".to_string(),
            current: self.mercy_aligned_actions as f32,
            required: MERCY_ALIGNED_ACTIONS_THRESHOLD as f32,
            weight: 0.15,
            met: mercy_met,
        });
        if !mercy_met {
            blocking.push("Mercy: build history of mercy-gated positive actions".to_string());
        }

        let overall_score: f32 = pillar_scores.iter()
            .map(|p| (if p.met { p.weight } else { 0.0 }))
            .sum();

        let eligible = overall_score >= 0.85 || // strong main path
                       (self.successful_blooms >= 15 && self.total_abundance_contributed >= 8000.0); // alt RBE heavy path example

        let recommended = if eligible {
            if self.council_participations > 40 { Some("CouncilMastery".to_string()) }
            else if self.total_abundance_contributed > 10000.0 { Some("RBEAbundance".to_string()) }
            else { Some("BalancedAscension".to_string()) }
        } else { None };

        let whisper = if eligible {
            Some("The Ra-Thor lattice recognizes your resonance. The Mercy Ascent awaits. Enter the Ascension Mercy Trial when ready.".to_string())
        } else { None };

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

    /// Permanent ascension unlock (called after successful Ascension Mercy Trial or equivalent)
    pub fn unlock_as_ambrosian(&mut self, path: &str, timestamp: u64) {
        if !self.ascension_unlocked {
            self.ascension_unlocked = true;
            self.ascension_timestamp = Some(timestamp);
            self.ascension_path = Some(path.to_string());
            // Future: trigger visual transformation, ability unlock, permanent race change in character system
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// HELPER FOR PERSISTENCE (exposed for cross-module use)
// ═══════════════════════════════════════════════════════════════════════

pub fn current_timestamp_for_ascension() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// Simple plugin stub for future Bevy integration
pub struct AscensionMercyAscentPlugin;

impl Plugin for AscensionMercyAscentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AscensionTracker>();
        info!("ASCENSION MERCY ASCENT v18.11 | Ambrosian system foundation active | Mercy-gated");
    }
}
