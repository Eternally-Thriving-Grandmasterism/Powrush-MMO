/*!
 * Sovereign Epiphany Catalyst v18.9
 *
 * Professional integration layer with automatic telemetry emission.
 *
 * Usage from harvest systems:
 *   let context = EpiphanyContext { ... };
 *   if let Some(outcome) = evaluate_epiphany(&context) {
 *       // outcome already triggered telemetry if enabled
 *   }
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// === Telemetry Configuration (sovereign-friendly) ===

#[derive(Resource, Debug, Clone)]
pub struct TelemetryConfig {
    pub epiphany_telemetry_enabled: bool,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            epiphany_telemetry_enabled: true, // Can be disabled for sovereign self-host
        }
    }
}

// === Telemetry Event ===

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
    pub mercy_alignment_score: f32,
    pub timestamp_ms: u64,
}

/// Emits a rich telemetry event for closed beta analysis.
/// Call this directly if you need more control.
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
    let mut score = 0.6;
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

// === Core Types ===

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

// === Main Evaluation Function with Auto-Telemetry ===

/// Evaluates context and returns an epiphany if one triggers.
/// Automatically emits telemetry when `TelemetryConfig::epiphany_telemetry_enabled` is true.
pub fn evaluate_epiphany(
    context: &EpiphanyContext,
    telemetry_config: Option<&TelemetryConfig>,
    player_id: Option<u64>,
    timestamp_ms: u64,
) -> Option<EpiphanyOutcome> {
    let outcome = if let Some(o) = check_overflow_lesson(context.depletion, context.sustainable_pacing, &context.biome) {
        Some(o)
    } else if let Some(o) = check_sustainable_abundance(context.depletion, context.regen_participation, &context.biome) {
        Some(o)
    } else if context.participant_count >= 3 {
        check_council_harmony(context.collective_attunement, context.participant_count, context.duration_ticks)
    } else {
        None
    };

    if let Some(ref o) = outcome {
        if telemetry_config.map_or(true, |cfg| cfg.epiphany_telemetry_enabled) {
            let _event = emit_epiphany_telemetry(o, context, player_id, timestamp_ms);
            // In real implementation: send _event to your main telemetry system / analytics
            // e.g. telemetry.send(TelemetryEvent::Epiphany(event));
        }
    }

    outcome
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
    Some(outcome)
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

pub fn get_available_scenarios() -> Vec<&'static str> {
    vec!["overflow_lesson", "council_harmony", "sustainable_abundance"]
}

// === Integration Examples (copy into your harvest / Council code) ===

/*
// Example in rbe_harvest_handler.rs or client harvesting system:

use crate::epiphany_catalyst::{EpiphanyContext, evaluate_epiphany, TelemetryConfig};

fn on_harvest_complete(...) {
    let context = EpiphanyContext {
        depletion: current_depletion,
        sustainable_pacing: was_sustainable,
        regen_participation: player_participated_in_regen,
        biome: current_biome,
        participant_count: 1,
        collective_attunement: 0.0,
        duration_ticks: harvest_duration,
    };

    let telemetry_config = world.get_resource::<TelemetryConfig>().cloned().unwrap_or_default();

    if let Some(outcome) = evaluate_epiphany(&context, Some(&telemetry_config), Some(player_id), current_time_ms) {
        // Apply world effects, trigger Divine Whispers, particles, etc.
        apply_epiphany_effects(outcome);
    }
}

// Example in Council session system:

fn on_council_tick(...) {
    let context = EpiphanyContext {
        depletion: 0.0,
        sustainable_pacing: false,
        regen_participation: false,
        biome: current_biome,
        participant_count: current_participants.len() as u8,
        collective_attunement: calculate_collective_attunement(),
        duration_ticks: session_duration,
    };

    if let Some(outcome) = evaluate_epiphany(&context, Some(&telemetry_config), None, current_time_ms) {
        // Broadcast Council Harmony bloom to all participants
        trigger_council_harmony_effects(outcome);
    }
}
*/
