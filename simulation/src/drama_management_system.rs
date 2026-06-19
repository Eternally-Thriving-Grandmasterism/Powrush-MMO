// simulation/src/drama_management_system.rs
// Powrush-MMO v18.97+ — Drama Management System (Hybrid Emergent + Goal-Oriented)
// Extended with rich Powrush-specific beat library (council trials, RBE harmony, epiphany catalysts, humble-to-legend arcs).
// Integrates with EmotionalResonance, RedemptionPath, WarNarrativeEvent, ServerWarSystem.
// Mercy-gated via TOLC 8 + 7 Living Mercy Gates. Self-evolving via Ra-Thor metrics.
// AG-SML v1.0 | Sovereign | Full file delivery.

use std::collections::HashMap;
use tracing::info;

#[derive(Clone, Debug)]
pub struct EmotionalState {
    pub valence: f32,
    pub mercy: f32,
    pub current_state: String,
}

#[derive(Clone, Debug)]
pub struct DramaBeat {
    pub id: String,
    pub description: String,
    pub tension_delta: f32,
    pub mercy_alignment: f32,
    pub intervention_type: String,
}

#[derive(Clone, Debug)]
pub struct PlayerDramaProfile {
    pub engagement: f32,
    pub preferred_arc: String,
    pub recent_beats: Vec<String>,
}

pub struct DramaManager {
    pub current_tension: f32,
    pub ideal_tension_curve: Vec<f32>,
    pub active_beats: Vec<DramaBeat>,
    pub player_profiles: HashMap<String, PlayerDramaProfile>,
    pub mercy_threshold: f32,
}

impl DramaManager {
    pub fn new() -> Self {
        Self {
            current_tension: 0.3,
            ideal_tension_curve: vec![0.2, 0.4, 0.7, 0.9, 0.6, 0.3],
            active_beats: vec![],
            player_profiles: HashMap::new(),
            mercy_threshold: 0.7,
        }
    }

    pub fn observe_state(&mut self, player_id: &str, emotional: &EmotionalState, war_active: bool, redemption_active: bool) {
        let mut delta = 0.0;
        if war_active { delta += 0.25; }
        if emotional.current_state == "scarred" { delta += 0.15; }
        if redemption_active { delta -= 0.2; }
        if emotional.valence > 0.7 { delta -= 0.1; }

        self.current_tension = (self.current_tension + delta).clamp(0.0, 1.0);

        let profile = self.player_profiles.entry(player_id.to_string()).or_insert(PlayerDramaProfile {
            engagement: 0.5,
            preferred_arc: "redemptive".to_string(),
            recent_beats: vec![],
        });
        profile.engagement = (profile.engagement + 0.05).min(1.0);

        info!("DramaManager observe | Player {} | Tension {:.2} | State {} | Mercy {:.1}",
              player_id, self.current_tension, emotional.current_state, emotional.mercy);
    }

    pub fn select_intervention(&self, player_id: &str) -> Option<DramaBeat> {
        let profile = self.player_profiles.get(player_id)?;
        let target_tension = if !self.ideal_tension_curve.is_empty() {
            self.ideal_tension_curve[self.active_beats.len() % self.ideal_tension_curve.len()]
        } else { 0.5 };
        let tension_gap = target_tension - self.current_tension;

        // === EXTENDED POWRUSH-SPECIFIC BEAT LIBRARY ===
        let candidates = vec![
            // Core tension / war beats
            DramaBeat { id: "war_tension_build".to_string(), description: "Introduce complication in ongoing conflict to heighten stakes and test alliances".to_string(), tension_delta: 0.2, mercy_alignment: 0.65, intervention_type: "complication".to_string() },
            DramaBeat { id: "redemption_catharsis".to_string(), description: "Trigger meaningful service opportunity or ally aid to enable growth from defeat".to_string(), tension_delta: -0.3, mercy_alignment: 0.95, intervention_type: "catharsis".to_string() },
            DramaBeat { id: "epiphany_trigger".to_string(), description: "Seed reflective moment or Divine Whisper aligned with current valence and RBE flows".to_string(), tension_delta: -0.1, mercy_alignment: 0.9, intervention_type: "epiphany_trigger".to_string() },
            DramaBeat { id: "humble_origin_seed".to_string(), description: "Reinforce early humble resource node connection to current epic war legend".to_string(), tension_delta: 0.1, mercy_alignment: 0.85, intervention_type: "hint".to_string() },

            // Powrush-specific: Council trials & mercy
            DramaBeat { id: "council_trial_harmony".to_string(), description: "Guide player toward council mercy trial participation for collective harmony resolution".to_string(), tension_delta: -0.15, mercy_alignment: 0.98, intervention_type: "council_harmony".to_string() },
            DramaBeat { id: "mercy_reflection_defeat".to_string(), description: "Prompt mercy reflection after loss to transform scar into wisdom and abundance mindset".to_string(), tension_delta: -0.25, mercy_alignment: 0.97, intervention_type: "redemption".to_string() },

            // RBE harmony & abundance moments
            DramaBeat { id: "rbe_harmony_share".to_string(), description: "Create opportunity for player to share abundance or resources with other factions in war context".to_string(), tension_delta: -0.1, mercy_alignment: 0.92, intervention_type: "rbe_harmony".to_string() },
            DramaBeat { id: "infrastructure_pride_seed".to_string(), description: "Highlight development_level investment in contested node as foundation of personal legend".to_string(), tension_delta: 0.05, mercy_alignment: 0.88, intervention_type: "pride".to_string() },

            // Epiphany catalysts
            DramaBeat { id: "epiphany_catalyst_wartolegend".to_string(), description: "Connect current war participation to long-term personal mythos and future epiphany potential".to_string(), tension_delta: 0.0, mercy_alignment: 0.93, intervention_type: "epiphany_catalyst".to_string() },
            DramaBeat { id: "divine_whisper_rbe".to_string(), description: "Trigger context-aware Divine Whisper on RBE flows, mercy, or cosmic harmony during high-valence moments".to_string(), tension_delta: -0.05, mercy_alignment: 0.96, intervention_type: "divine_whisper".to_string() },
        ];

        let mut best: Option<DramaBeat> = None;
        let mut best_score = -999.0;

        for beat in candidates {
            if beat.mercy_alignment < self.mercy_threshold { continue; }
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

    pub fn apply_intervention(&mut self, player_id: &str, beat: &DramaBeat, emotional: &mut EmotionalState) -> String {
        self.current_tension = (self.current_tension + beat.tension_delta).clamp(0.0, 1.0);
        emotional.valence = (emotional.valence + beat.tension_delta * 0.5).clamp(0.0, 1.0);

        if beat.intervention_type == "catharsis" || beat.intervention_type == "redemption" || beat.id.contains("catharsis") {
            emotional.current_state = "reflective".to_string();
            emotional.mercy = (emotional.mercy + 8.0).min(100.0);
        } else if beat.intervention_type == "complication" {
            emotional.current_state = "scarred".to_string();
        } else if beat.intervention_type.contains("epiphany") || beat.id.contains("epiphany") {
            emotional.valence = (emotional.valence + 0.15).min(1.0);
            emotional.current_state = "reflective".to_string();
        }

        if let Some(profile) = self.player_profiles.get_mut(player_id) {
            profile.recent_beats.push(beat.id.clone());
            if profile.recent_beats.len() > 5 { profile.recent_beats.remove(0); }
        }

        let narrative = format!("Drama beat: {} | Tension {:.2} | Emotional shift to {} | Mercy-aligned Powrush legend seed",
                              beat.description, self.current_tension, emotional.current_state);
        info!("Drama intervention | {} | {}", player_id, narrative);
        narrative
    }

    pub fn drama_tick(&mut self, player_id: &str, emotional: &mut EmotionalState, war_active: bool, redemption_active: bool) -> Option<String> {
        self.observe_state(player_id, emotional, war_active, redemption_active);
        if (self.current_tension - 0.5).abs() > 0.25 || emotional.current_state == "scarred" || emotional.valence < 0.4 {
            if let Some(beat) = self.select_intervention(player_id) {
                return Some(self.apply_intervention(player_id, &beat, emotional));
            }
        }
        None
    }

    pub fn post_war_drama(&mut self, winner_id: &str, loser_ids: &[String], emotional_map: &mut HashMap<String, EmotionalState>) -> Vec<String> {
        let mut narratives = vec![];
        for loser in loser_ids {
            if let Some(em) = emotional_map.get_mut(loser) {
                if let Some(nar) = self.drama_tick(loser, em, false, true) {
                    narratives.push(nar);
                }
            }
        }
        if let Some(em) = emotional_map.get_mut(winner_id) {
            if let Some(nar) = self.drama_tick(winner_id, em, false, false) {
                narratives.push(nar);
            }
        }
        narratives
    }

    pub fn evolve_parameters(&mut self, avg_valence: f32, avg_mercy: f32, engagement: f32) {
        if avg_valence < 0.5 {
            self.mercy_threshold = (self.mercy_threshold - 0.05).max(0.5);
        }
        if engagement > 0.8 {
            self.ideal_tension_curve = vec![0.15, 0.5, 0.85, 0.95, 0.5, 0.2];
        }
        info!("DramaManager evolved | New mercy threshold {:.2}", self.mercy_threshold);
    }
}

// End of drama_management_system.rs v18.97+ (extended Powrush beats)
// Ready for wiring into simulation orchestrator and server war handlers.
// Output narratives feed client mythos/journal + Divine Whispers.