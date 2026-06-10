/*!
 * Sovereign Epiphany Catalyst v18.9 (with Bot Detection integration)
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::bot_detection::{BotDetectionConfig, calculate_epiphany_anomaly, AnomalyScore, BotSuspicion, update_suspicion};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EpiphanyOutcome { /* ... existing fields ... */ }

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
    pub behavioral_human_score: f32,      // NEW
    pub anomaly_score: f32,               // NEW
    pub timestamp_ms: u64,
}

pub fn emit_epiphany_telemetry(
    outcome: &EpiphanyOutcome,
    context: &EpiphanyContext,
    player_id: Option<u64>,
    timestamp_ms: u64,
    behavioral_human_score: f32,          // NEW
) -> EpiphanyTelemetryEvent {
    let was_council = context.participant_count >= 3;
    let mercy_score = calculate_mercy_alignment(outcome, context);

    // Calculate anomaly (requires historical data in real impl)
    let dummy_historical = 0.55;
    let anomaly = calculate_epiphany_anomaly(
        &EpiphanyTelemetryEvent {
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
            behavioral_human_score,
            anomaly_score: 0.0,
            timestamp_ms,
        },
        dummy_historical,
        behavioral_human_score,
    );

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
        behavioral_human_score,
        anomaly_score: anomaly.overall_score,
        timestamp_ms,
    }
}

// ... rest of the file (evaluate_epiphany, detectors, etc.) remains largely the same
// with minor updates to pass behavioral_human_score when emitting telemetry.
