/*!
 * Sovereign Epiphany Catalyst v18.9
 *
 * Includes professional telemetry hooks for closed beta.
 * 
 * Use `emit_epiphany_telemetry()` after successful epiphany evaluation
 * to track player growth, scenario effectiveness, and mercy alignment.
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// === Telemetry Events for Closed Beta ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpiphanyTelemetryEvent {
    pub scenario_id: String,
    pub player_id: Option<u64>,
    pub intensity: f32,
    pub epiphany_multiplier: f32,
    pub muscle_memory_gain: f32,
    pub hypofrontality_depth: f32,
    pub biome: String,
    pub participant_count: u8,
    pub was_council_session: bool,
    pub mercy_alignment_score: f32, // 0.0 - 1.0 how mercy-aligned the trigger was
    pub timestamp_ms: u64,
}

/// Call this after a successful epiphany to record data for closed beta analysis
pub fn emit_epiphany_telemetry(
    outcome: &EpiphanyOutcome,
    context: &EpiphanyContext,
    player_id: Option<u64>,
    timestamp_ms: u64,
) -> EpiphanyTelemetryEvent {
    let was_council = context.participant_count >= 3;
    let mercy_score = calculate_mercy_alignment(outcome, context);

    EpiphanyTelemetryEvent {
        scenario_id: outcome.scenario_id.clone(),
        player_id,
        intensity: outcome.intensity,
        epiphany_multiplier: outcome.epiphany_multiplier,
        muscle_memory_gain: outcome.muscle_memory_consolidation_boost,
        hypofrontality_depth: outcome.hypofrontality_depth,
        biome: context.biome.clone(),
        participant_count: context.participant_count,
        was_council_session: was_council,
        mercy_alignment_score: mercy_score,
        timestamp_ms,
    }
}

fn calculate_mercy_alignment(outcome: &EpiphanyOutcome, context: &EpiphanyContext) -> f32 {
    let mut score = 0.6; // base mercy alignment

    if outcome.scenario_id.contains("council") && context.participant_count >= 3 {
        score += 0.25;
    }
    if context.sustainable_pacing || context.regen_participation {
        score += 0.15;
    }
    if outcome.intensity > 0.7 {
        score += 0.1;
    }
    score.min(1.0)
}

// === Existing EpiphanyOutcome and Context (abbreviated for clarity) ===

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

pub fn evaluate_epiphany(context: &EpiphanyContext) -> Option<EpiphanyOutcome> {
    if let Some(outcome) = check_overflow_lesson(context.depletion, context.sustainable_pacing, &context.biome) {
        return Some(outcome);
    }
    if let Some(outcome) = check_sustainable_abundance(context.depletion, context.regen_participation, &context.biome) {
        return Some(outcome);
    }
    if context.participant_count >= 3 {
        if let Some(outcome) = check_council_harmony(context.collective_attunement, context.participant_count, context.duration_ticks) {
            return Some(outcome);
        }
    }
    None
}

// Detector implementations (simplified for this commit)
pub fn check_overflow_lesson(depletion: f32, sustainable_pacing: bool, biome: &str) -> Option<EpiphanyOutcome> { None }
pub fn check_council_harmony(collective: f32, count: u8, ticks: u64) -> Option<EpiphanyOutcome> { None }
pub fn check_sustainable_abundance(depletion: f32, regen: bool, biome: &str) -> Option<EpiphanyOutcome> { None }

pub fn get_available_scenarios() -> Vec<&'static str> {
    vec!["overflow_lesson", "council_harmony", "sustainable_abundance"]
}
