// simulation/src/drama_management_system.rs
// Powrush-MMO v18.97+ — Drama Management System (Hybrid Emergent + Goal-Oriented)
// Explores and implements drama management techniques for procedural narrative.
// Integrates with EmotionalResonance, RedemptionPath, WarNarrativeEvent, ServerWarSystem.
// Mercy-gated via TOLC 8 + 7 Living Mercy Gates. Self-evolving via Ra-Thor metrics.
// Builds directly on WarNarrativeEvent seeds and server war simulation.
// AG-SML v1.0 | Sovereign | Full file delivery.

use std::collections::HashMap;
use tracing::info;

// Re-use / compatible types from existing systems (server_war_system, simulation emotional layers)
// In full integration: use crate::server::server_war_system::{EmotionalResonance, RedemptionPath, WarNarrativeEvent};
// For standalone skeleton: local mirrors for clarity.

#[derive(Clone, Debug)]
pub struct EmotionalState {
    pub valence: f32,      // 0.0–1.0 (epiphany/joy)
    pub mercy: f32,
    pub current_state: String, // "neutral", "scarred", "triumphant", "reflective"
}

#[derive(Clone, Debug)]
pub struct DramaBeat {
    pub id: String,
    pub description: String,
    pub tension_delta: f32,      // positive = raise tension, negative = release
    pub mercy_alignment: f32,    // how well it honors 7 Gates (higher = better)
    pub intervention_type: String, // "hint", "complication", "catharsis", "epiphany_trigger", "alliance_seed"
}

#[derive(Clone, Debug)]
pub struct PlayerDramaProfile {
    pub engagement: f32,
    pub preferred_arc: String, // "heroic", "redemptive", "harmonious", "exploratory"
    pub recent_beats: Vec<String>,
}

/// Core Drama Manager
/// Observes world/player state, computes dramatic tension, selects & applies interventions
/// to shape coherent, meaningful arcs in procedural systems (wars, epiphanies, redemption).
pub struct DramaManager {
    pub current_tension: f32,           // 0.0 (calm) – 1.0 (peak crisis)
    pub ideal_tension_curve: Vec<f32>,  // target arc over time (rising action → climax → resolution)
    pub active_beats: Vec<DramaBeat>,
    pub player_profiles: HashMap<String, PlayerDramaProfile>,
    pub mercy_threshold: f32,           // minimum alignment for any intervention
}

impl DramaManager {
    pub fn new() -> Self {
        Self {
            current_tension: 0.3,
            ideal_tension_curve: vec![0.2, 0.4, 0.7, 0.9, 0.6, 0.3], // simple 6-phase arc
            active_beats: vec![],
            player_profiles: HashMap::new(),
            mercy_threshold: 0.7,
        }
    }

    /// Observe current emotional/world state (called every tick or on key events)
    /// Integrates directly with EmotionalResonance from server_war_system
    pub fn observe_state(&mut self, player_id: &str, emotional: &EmotionalState, war_active: bool, redemption_active: bool) {
        // Update tension based on state
        let mut delta = 0.0;
        if war_active {
            delta += 0.25; // war raises tension
        }
        if emotional.current_state == "scarred" {
            delta += 0.15; // emotional weight
        }
        if redemption_active {
            delta -= 0.2; // redemption releases tension toward catharsis
        }
        if emotional.valence > 0.7 {
            delta -= 0.1; // high epiphany calms
        }

        self.current_tension = (self.current_tension + delta).clamp(0.0, 1.0);

        // Update or init player profile
        let profile = self.player_profiles.entry(player_id.to_string()).or_insert(PlayerDramaProfile {
            engagement: 0.5,
            preferred_arc: "redemptive".to_string(),
            recent_beats: vec![],
        });
        profile.engagement = (profile.engagement + 0.05).min(1.0);

        info!("DramaManager observe | Player {} | Tension {:.2} | State {} | Mercy {:.1}",
              player_id, self.current_tension, emotional.current_state, emotional.mercy);
    }

    /// Select next drama beat / intervention
    /// Hybrid: Considers current tension vs ideal curve + player profile + mercy alignment
    pub fn select_intervention(&self, player_id: &str) -> Option<DramaBeat> {
        let profile = self.player_profiles.get(player_id)?;

        // Simple tension targeting
        let target_tension = if self.ideal_tension_curve.len() > 0 {
            self.ideal_tension_curve[self.active_beats.len() % self.ideal_tension_curve.len()]
        } else { 0.5 };

        let tension_gap = target_tension - self.current_tension;

        // Candidate beats (expand in full impl with more data-driven or planned beats)
        let candidates = vec![
            DramaBeat {
                id: "war_tension_build".to_string(),
                description: "Introduce complication in ongoing conflict to heighten stakes".to_string(),
                tension_delta: 0.2,
                mercy_alignment: 0.65,
                intervention_type: "complication".to_string(),
            },
            DramaBeat {
                id: "redemption_catharsis".to_string(),
                description: "Trigger meaningful service opportunity or ally aid to enable growth".to_string(),
                tension_delta: -0.3,
                mercy_alignment: 0.95,
                intervention_type: "catharsis".to_string(),
            },
            DramaBeat {
                id: "epiphany_trigger".to_string(),
                description: "Seed reflective moment or Divine Whisper aligned with current valence".to_string(),
                tension_delta: -0.1,
                mercy_alignment: 0.9,
                intervention_type: "epiphany_trigger".to_string(),
            },
            DramaBeat {
                id: "humble_origin_seed".to_string(),
                description: "Reinforce early humble beginnings connection to current epic events".to_string(),
                tension_delta: 0.1,
                mercy_alignment: 0.85,
                intervention_type: "hint".to_string(),
            },
        ];

        // Score candidates: tension fit + mercy + player preference
        let mut best: Option<DramaBeat> = None;
        let mut best_score = -999.0;

        for beat in candidates {
            if beat.mercy_alignment < self.mercy_threshold {
                continue;
            }
            let tension_fit = 1.0 - (beat.tension_delta - tension_gap).abs();
            let mercy_fit = beat.mercy_alignment;
            let preference_fit = if profile.preferred_arc.contains(&beat.intervention_type) { 0.3 } else { 0.0 };
            let score = tension_fit * 0.5 + mercy_fit * 0.4 + preference_fit;

            if score > best_score {
                best_score = score;
                best = Some(beat);
            }
        }

        best
    }

    /// Apply chosen intervention (example hooks — integrate with real systems)
    pub fn apply_intervention(&mut self, player_id: &str, beat: &DramaBeat, emotional: &mut EmotionalState) -> String {
        // Update tension
        self.current_tension = (self.current_tension + beat.tension_delta).clamp(0.0, 1.0);

        // Update emotional state (ties to existing EmotionalResonance)
        emotional.valence = (emotional.valence + beat.tension_delta * 0.5).clamp(0.0, 1.0);
        if beat.intervention_type == "catharsis" || beat.intervention_type == "redemption_catharsis" {
            emotional.current_state = "reflective".to_string();
            emotional.mercy = (emotional.mercy + 8.0).min(100.0);
        } else if beat.intervention_type == "complication" {
            if emotional.current_state != "scarred" {
                emotional.current_state = "scarred".to_string();
            }
        }

        // Record in profile
        if let Some(profile) = self.player_profiles.get_mut(player_id) {
            profile.recent_beats.push(beat.id.clone());
            if profile.recent_beats.len() > 5 {
                profile.recent_beats.remove(0);
            }
        }

        // Generate narrative seed (compatible with WarNarrativeEvent)
        let narrative = format!("Drama beat applied: {} | Tension now {:.2} | Emotional shift toward {}",
                              beat.description, self.current_tension, emotional.current_state);

        info!("Drama intervention | {} | {}", player_id, narrative);
        narrative
    }

    /// High-level tick: observe + decide + apply if needed
    /// Call from simulation tick or server war handler
    pub fn drama_tick(&mut self, player_id: &str, emotional: &mut EmotionalState, war_active: bool, redemption_active: bool) -> Option<String> {
        self.observe_state(player_id, emotional, war_active, redemption_active);

        // Decide if intervention is warranted (tension gap or emotional need)
        if (self.current_tension - 0.5).abs() > 0.25 || emotional.current_state == "scarred" {
            if let Some(beat) = self.select_intervention(player_id) {
                return Some(self.apply_intervention(player_id, &beat, emotional));
            }
        }
        None
    }

    /// Example: Integrate with server war resolution for post-war drama
    pub fn post_war_drama(&mut self, winner_id: &str, loser_ids: &[String], emotional_map: &mut HashMap<String, EmotionalState>) -> Vec<String> {
        let mut narratives = vec![];
        for loser in loser_ids {
            if let Some(em) = emotional_map.get_mut(loser) {
                if let Some(nar) = self.drama_tick(loser, em, false, true) {  // trigger redemption focus
                    narratives.push(nar);
                }
            }
        }
        // Winner gets triumphant / epiphany beat
        if let Some(em) = emotional_map.get_mut(winner_id) {
            if let Some(nar) = self.drama_tick(winner_id, em, false, false) {
                narratives.push(nar);
            }
        }
        narratives
    }

    /// Self-evolution hook: Adjust mercy_threshold or curve based on aggregate player metrics
    /// (call from Ra-Thor orchestrator with simulation feedback)
    pub fn evolve_parameters(&mut self, avg_valence: f32, avg_mercy: f32, engagement: f32) {
        if avg_valence < 0.5 {
            self.mercy_threshold = (self.mercy_threshold - 0.05).max(0.5); // lower bar for more redemptive interventions
        }
        if engagement > 0.8 {
            // tighten curve for more dynamic arcs
            self.ideal_tension_curve = vec![0.15, 0.5, 0.85, 0.95, 0.5, 0.2];
        }
        info!("DramaManager evolved | New mercy threshold {:.2}", self.mercy_threshold);
    }
}

// Usage example in simulation or server tick:
// let mut dm = DramaManager::new();
// let narrative = dm.drama_tick("player_123", &mut emotional_state, war_active, redemption_active);
// if let Some(n) = narrative { war_narrative_log.push(...); }

// End of drama_management_system.rs
// Fully compatible with existing Powrush emotional/narrative/war systems.
// Extends humble-beginnings-to-epic arcs with guided yet emergent drama.
// Mercy-first, self-evolving, ready for client UI feedback and PATSAGi oversight.