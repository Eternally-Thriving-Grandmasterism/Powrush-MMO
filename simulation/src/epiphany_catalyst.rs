/*!
 * Sovereign Epiphany Catalyst v18.10
 *
 * Production-grade integration layer for wiring epiphanies into gameplay.
 * This file now includes clean helpers to make integration into harvest systems trivial.
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::bot_detection::{BotDetectionConfig, calculate_epiphany_anomaly, AnomalyScore};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EpiphanyOutcome {
    pub scenario_id: String,
    pub epiphany_multiplier: f32,
    pub muscle_memory_consolidation_boost: f32,
    pub hypofrontality_depth: f32,
    pub particle_effect: String,
    pub time_dilation_factor: f32,
    pub divine_whisper_flavor: String,
    pub world_effects: HashMap<String, f32>,
    pub grace_notes: Vec<String>,
    pub intensity: f32,
}

impl EpiphanyOutcome {
    pub fn new() -> Self {
        Self {
            scenario_id: "base".to_string(),
            epiphany_multiplier: 1.0,
            muscle_memory_consolidation_boost: 1.0,
            hypofrontality_depth: 0.0,
            particle_effect: "default".to_string(),
            time_dilation_factor: 1.0,
            divine_whisper_flavor: "sustainable_presence".to_string(),
            world_effects: HashMap::new(),
            grace_notes: vec![],
            intensity: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EpiphanyContext {
    pub depletion: f32,
    pub sustainable_pacing: bool,
    pub regen_participation: bool,
    pub biome: String,
    pub participant_count: u8,
    pub collective_attunement: f32,
    pub duration_ticks: u64,
}

impl Default for EpiphanyContext {
    fn default() -> Self {
        Self {
            depletion: 0.0,
            sustainable_pacing: false,
            regen_participation: false,
            biome: "starter".to_string(),
            participant_count: 1,
            collective_attunement: 0.0,
            duration_ticks: 0,
        }
    }
}

/// High-level helper: Call this after a successful harvest to check for epiphanies.
/// This is the main integration point for harvest systems.
pub fn check_epiphany_after_harvest(
    depletion: f32,
    sustainable_pacing: bool,
    regen_participation: bool,
    biome: &str,
    behavioral_human_score: f32,
) -> Option<EpiphanyOutcome> {
    let context = EpiphanyContext {
        depletion,
        sustainable_pacing,
        regen_participation,
        biome: biome.to_string(),
        participant_count: 1,
        collective_attunement: 0.0,
        duration_ticks: 0,
    };

    evaluate_epiphany(&context, behavioral_human_score)
}

/// Core evaluation function
pub fn evaluate_epiphany(
    context: &EpiphanyContext,
    behavioral_human_score: f32,
) -> Option<EpiphanyOutcome> {
    // Try Overflow Lesson first
    if let Some(outcome) = check_overflow_lesson(context.depletion, context.sustainable_pacing, &context.biome) {
        return Some(outcome);
    }

    // Try Sustainable Abundance
    if let Some(outcome) = check_sustainable_abundance(context.depletion, context.regen_participation, &context.biome) {
        return Some(outcome);
    }

    // Council Harmony is handled separately in council sessions
    None
}

// === Individual Detectors ===

pub fn check_overflow_lesson(depletion: f32, sustainable_pacing: bool, biome: &str) -> Option<EpiphanyOutcome> {
    if !sustainable_pacing || depletion > 0.58 { return None; }
    if biome != "Verdant Heartwood" && biome != "starter" && biome != "heartwood" { return None; }

    let intensity = ((1.0 - depletion * 1.1).max(0.25)).min(0.92);
    let mut outcome = EpiphanyOutcome::new();
    outcome.scenario_id = format!("overflow_lesson_{}", biome);
    outcome.epiphany_multiplier = 1.0 + intensity * 1.35;
    outcome.muscle_memory_consolidation_boost = 1.0 + intensity * 0.95;
    outcome.intensity = intensity;
    outcome.divine_whisper_flavor = "sustainable_harmony_revelation".to_string();
    Some(outcome)
}

pub fn check_sustainable_abundance(depletion: f32, regen_participation: bool, biome: &str) -> Option<EpiphanyOutcome> {
    if depletion > 0.35 || !regen_participation { return None; }
    if biome != "Verdant Heartwood" { return None; }

    let intensity = (1.0 - depletion).clamp(0.4, 0.95);
    let mut outcome = EpiphanyOutcome::new();
    outcome.scenario_id = "sustainable_abundance".to_string();
    outcome.epiphany_multiplier = if depletion < 0.25 { 1.55 } else { 1.3 };
    outcome.muscle_memory_consolidation_boost = if depletion < 0.25 { 1.45 } else { 1.2 };
    outcome.intensity = intensity;
    outcome.divine_whisper_flavor = "sustainable_abundance_revelation".to_string();
    outcome.grace_notes.push("Every sustainable choice writes a better future into the living web.".to_string());
    Some(outcome)
}

pub fn check_council_harmony(collective_attunement: f32, participant_count: u8, duration_ticks: u64) -> Option<EpiphanyOutcome> {
    if participant_count < 3 || collective_attunement < 0.55 || duration_ticks < 180 { return None; }

    let intensity = collective_attunement.clamp(0.55, 1.0);
    let mut outcome = EpiphanyOutcome::new();
    outcome.scenario_id = "council_harmony".to_string();
    outcome.epiphany_multiplier = if intensity > 0.75 { 1.6 } else { 1.25 };
    outcome.muscle_memory_consolidation_boost = if intensity > 0.75 { 1.4 } else { 1.15 };
    outcome.intensity = intensity;
    outcome.divine_whisper_flavor = "council_harmony_revelation".to_string();

    if intensity > 0.75 {
        outcome.world_effects.insert("collective_abundance_bloom".to_string(), 1.3);
    }
    Some(outcome)
}

/// Example integration for harvest handlers (copy/adapt into your harvest system)
/*
// In server/rbe_harvest_handler.rs or equivalent:

use simulation::epiphany_catalyst::{check_epiphany_after_harvest, EpiphanyOutcome};

fn on_harvest_success(
    player_id: u64,
    depletion: f32,
    sustainable_pacing: bool,
    regen_participation: bool,
    biome: &str,
    behavioral_human_score: f32,
) {
    if let Some(epiphany) = check_epiphany_after_harvest(
        depletion,
        sustainable_pacing,
        regen_participation,
        biome,
        behavioral_human_score,
    ) {
        // Apply effects
        apply_epiphany_effects(player_id, &epiphany);

        // Emit telemetry (already includes behavioral score)
        let telemetry_event = emit_epiphany_telemetry(...);
        // Send to telemetry system
    }
}

fn apply_epiphany_effects(player_id: u64, epiphany: &EpiphanyOutcome) {
    // Apply multipliers to future harvests
    // Trigger Divine Whispers
    // Spawn particles
    // Update muscle memory
    // Apply world effects
}
*/
