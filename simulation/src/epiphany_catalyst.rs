/*!
 * Sovereign Epiphany Catalyst
 *
 * v18.96 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm v2)
 * — Complete mint-and-print-only-perfection
 * — evaluate_epiphany() is the single source of truth for all epiphany detection
 * — Mercy-amplified + council-aware outcomes
 * — TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable Layer 0
 * — Quantum Swarm v2 valence hook exposed for self-evolution + RBE feedback loops
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
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
    pub biome_resonance: Option<String>,
    pub abundance_bloom_multiplier: f32,
}

impl EpiphanyOutcome {
    pub fn new() -> Self {
        Self {
            scenario_id: "base".to_string(),
            epiphany_multiplier: 1.0,
            muscle_memory_consolidation_boost: 1.0,
            hypofrontality_depth: 0.0,
            particle_effect: "ethereal_bloom".to_string(),
            time_dilation_factor: 1.0,
            divine_whisper_flavor: "sustainable_presence".to_string(),
            world_effects: HashMap::new(),
            grace_notes: vec![],
            intensity: 0.0,
            biome_resonance: None,
            abundance_bloom_multiplier: 1.0,
        }
    }
}

/// Rich event emitted when an epiphany is successfully triggered.
/// Main hook for multi-channel feedback (visuals, spatial audio, Divine Whispers, UI, persistence).
#[derive(Event, Debug, Clone)]
pub struct EpiphanyTriggered {
    pub outcome: EpiphanyOutcome,
    pub biome: String,
    pub player_id: u64,
}

/// Explicit hook for positioned Spatial Audio bloom during epiphany moments.
#[derive(Event, Debug, Clone)]
pub struct EpiphanySpatialAudioBloom {
    pub position: Option<Vec3>,
    pub intensity: f32,
    pub audio_flavor: String,
    pub particle_effect_sync: String,
    pub time_dilation: f32,
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
    pub season: Option<String>,
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
            season: None,
        }
    }
}

/// High-level helper: main integration point. Call after EVERY successful harvest.
pub fn check_epiphany_after_harvest(
    depletion: f32,
    sustainable_pacing: bool,
    regen_participation: bool,
    biome: &str,
    season: Option<&str>,
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
        season: season.map(|s| s.to_string()),
    };

    evaluate_epiphany(&context, behavioral_human_score)
}

/// Core evaluation function — SINGLE SOURCE OF TRUTH for all epiphany detection.
/// Mercy-amplified and council-aware. Expanded vertical slice scenarios.
pub fn evaluate_epiphany(
    context: &EpiphanyContext,
    behavioral_human_score: f32,
) -> Option<EpiphanyOutcome> {
    let human_factor = behavioral_human_score.clamp(0.6, 1.15);
    if human_factor < 0.65 { return None; }

    // Priority order: more specific / higher-reward first
    if let Some(mut outcome) = check_overflow_lesson(context.depletion, context.sustainable_pacing, &context.biome) {
        outcome = apply_human_amplification(outcome, human_factor);
        outcome = apply_biome_resonance(outcome, context);
        return Some(outcome);
    }

    if let Some(mut outcome) = check_sustainable_abundance(context.depletion, context.regen_participation, &context.biome) {
        outcome = apply_human_amplification(outcome, human_factor);
        outcome = apply_biome_resonance(outcome, context);
        return Some(outcome);
    }

    if context.biome.contains("crystal_spires") || context.biome == "crystal_spires" {
        if let Some(mut outcome) = check_crystal_spires_resonance(context) {
            outcome = apply_human_amplification(outcome, human_factor);
            return Some(outcome);
        }
    }

    if context.biome.contains("abyssal_depths") || context.biome == "abyssal_depths" {
        if let Some(mut outcome) = check_abyssal_depths_surge(context) {
            outcome = apply_human_amplification(outcome, human_factor);
            return Some(outcome);
        }
        // New: Mycorrhizal Communion (deeper network healing)
        if let Some(mut outcome) = check_mycorrhizal_communion(context) {
            outcome = apply_human_amplification(outcome, human_factor);
            return Some(outcome);
        }
    }

    if context.biome.contains("crystal_spires") || context.biome == "crystal_spires" {
        if let Some(mut outcome) = check_stellar_resonance(context) {
            outcome = apply_human_amplification(outcome, human_factor);
            return Some(outcome);
        }
    }

    // New: Graceful Redemption — turning past depletion into present abundance (any biome, after recovery)
    if let Some(mut outcome) = check_graceful_redemption(context) {
        outcome = apply_human_amplification(outcome, human_factor);
        return Some(outcome);
    }

    // Council harmony (multiplayer)
    if context.participant_count >= 3 && context.collective_attunement > 0.0 {
        if let Some(mut outcome) = check_council_harmony(context.collective_attunement, context.participant_count, context.duration_ticks) {
            outcome = apply_human_amplification(outcome, human_factor);
            return Some(outcome);
        }
    }

    None
}

fn apply_human_amplification(mut outcome: EpiphanyOutcome, human_factor: f32) -> EpiphanyOutcome {
    outcome.epiphany_multiplier *= human_factor;
    outcome.muscle_memory_consolidation_boost *= (human_factor * 0.9 + 0.1);
    outcome.intensity = (outcome.intensity * human_factor).clamp(0.3, 0.98);
    if human_factor > 1.0 {
        outcome.grace_notes.push("Your authentic presence amplifies the living web.".to_string());
    }
    outcome
}

fn apply_biome_resonance(mut outcome: EpiphanyOutcome, context: &EpiphanyContext) -> EpiphanyOutcome {
    if let Some(season) = &context.season {
        if (context.biome.contains("crystal_spires") || context.biome == "crystal_spires") && season == "resonance_peak" {
            outcome.biome_resonance = Some("crystal_spires_resonance_peak".to_string());
            outcome.abundance_bloom_multiplier = 1.45;
            outcome.particle_effect = "sacred_geometry_crystal_bloom".to_string();
            outcome.time_dilation_factor = 1.25;
            outcome.grace_notes.push("The spires sing through your sustainable touch — abundance echoes outward.".to_string());
            outcome.world_effects.insert("crystal_resonance_bloom".to_string(), 1.4);
        }
        if (context.biome.contains("abyssal_depths") || context.biome == "abyssal_depths") && season == "mycelium_surge" {
            outcome.biome_resonance = Some("abyssal_depths_mycelium_surge".to_string());
            outcome.abundance_bloom_multiplier = 1.35;
            outcome.particle_effect = "mycelial_web_glow".to_string();
            outcome.time_dilation_factor = 1.15;
            outcome.grace_notes.push("The deep mycelium surges in joyful response to your mercy.".to_string());
            outcome.world_effects.insert("mycelial_abundance_web".to_string(), 1.3);
        }
    }
    outcome
}

pub fn check_overflow_lesson(depletion: f32, sustainable_pacing: bool, biome: &str) -> Option<EpiphanyOutcome> {
    if !sustainable_pacing || depletion > 0.58 { return None; }
    if biome != "Verdant Heartwood" && biome != "starter" && biome != "heartwood" && !biome.contains("crystal") { return None; }

    let intensity = ((1.0 - depletion * 1.1).max(0.25)).min(0.92);
    let mut outcome = EpiphanyOutcome::new();
    outcome.scenario_id = format!("overflow_lesson_{}", biome);
    outcome.epiphany_multiplier = 1.0 + intensity * 1.35;
    outcome.muscle_memory_consolidation_boost = 1.0 + intensity * 0.95;
    outcome.intensity = intensity;
    outcome.divine_whisper_flavor = "sustainable_harmony_revelation".to_string();
    outcome.hypofrontality_depth = 0.65;
    Some(outcome)
}

pub fn check_sustainable_abundance(depletion: f32, regen_participation: bool, biome: &str) -> Option<EpiphanyOutcome> {
    if depletion > 0.35 || !regen_participation { return None; }
    if biome != "Verdant Heartwood" && !biome.contains("crystal") { return None; }

    let intensity = (1.0 - depletion).clamp(0.4, 0.95);
    let mut outcome = EpiphanyOutcome::new();
    outcome.scenario_id = "sustainable_abundance".to_string();
    outcome.epiphany_multiplier = if depletion < 0.25 { 1.55 } else { 1.3 };
    outcome.muscle_memory_consolidation_boost = if depletion < 0.25 { 1.45 } else { 1.2 };
    outcome.intensity = intensity;
    outcome.divine_whisper_flavor = "sustainable_abundance_revelation".to_string();
    outcome.grace_notes.push("Every sustainable choice writes a better future into the living web.".to_string());
    outcome.hypofrontality_depth = 0.55;
    Some(outcome)
}

pub fn check_crystal_spires_resonance(context: &EpiphanyContext) -> Option<EpiphanyOutcome> {
    if context.depletion > 0.45 || !context.sustainable_pacing { return None; }
    let season_match = context.season.as_deref() == Some("resonance_peak");
    if !season_match { return None; }

    let intensity = (0.75 + (1.0 - context.depletion) * 0.2).clamp(0.6, 0.95);
    let mut outcome = EpiphanyOutcome::new();
    outcome.scenario_id = "crystal_spires_resonance_peak".to_string();
    outcome.epiphany_multiplier = 1.6;
    outcome.muscle_memory_consolidation_boost = 1.5;
    outcome.intensity = intensity;
    outcome.divine_whisper_flavor = "spires_sing_the_web".to_string();
    outcome.particle_effect = "sacred_geometry_crystal_bloom".to_string();
    outcome.time_dilation_factor = 1.3;
    outcome.abundance_bloom_multiplier = 1.45;
    outcome.biome_resonance = Some("crystal_spires_resonance_peak".to_string());
    outcome.grace_notes.push("The Crystal Spires resonate with your presence — abundance multiplies across the lattice.".to_string());
    outcome.world_effects.insert("crystal_resonance_bloom".to_string(), 1.5);
    Some(outcome)
}

pub fn check_abyssal_depths_surge(context: &EpiphanyContext) -> Option<EpiphanyOutcome> {
    if context.depletion > 0.5 || !context.sustainable_pacing { return None; }
    let season_match = context.season.as_deref() == Some("mycelium_surge");
    if !season_match { return None; }

    let intensity = (0.7 + (1.0 - context.depletion) * 0.25).clamp(0.55, 0.92);
    let mut outcome = EpiphanyOutcome::new();
    outcome.scenario_id = "abyssal_depths_mycelium_surge".to_string();
    outcome.epiphany_multiplier = 1.5;
    outcome.muscle_memory_consolidation_boost = 1.4;
    outcome.intensity = intensity;
    outcome.divine_whisper_flavor = "deep_mycelium_whisper".to_string();
    outcome.particle_effect = "mycelial_web_glow".to_string();
    outcome.time_dilation_factor = 1.2;
    outcome.abundance_bloom_multiplier = 1.35;
    outcome.biome_resonance = Some("abyssal_depths_mycelium_surge".to_string());
    outcome.grace_notes.push("The Abyssal Depths mycelium surges in joyful co-creation with your mercy.".to_string());
    outcome.world_effects.insert("mycelial_abundance_web".to_string(), 1.4);
    Some(outcome)
}

// === NEW HIGH-QUALITY EPIPHANY SCENARIOS (v18.35 vertical slice expansion) ===

/// Mycorrhizal Communion — Deep network healing and shared resilience bloom.
/// Triggers in Abyssal Depths or mycelial-rich areas after sustained gentle interaction.
pub fn check_mycorrhizal_communion(context: &EpiphanyContext) -> Option<EpiphanyOutcome> {
    if context.depletion > 0.4 || !context.sustainable_pacing { return None; }
    // Stronger in Abyssal or when regen_participation is high
    let is_mycelial_biome = context.biome.contains("abyssal") || context.biome.contains("mycel");
    if !is_mycelial_biome && context.regen_participation == false { return None; }

    let intensity = (0.65 + (1.0 - context.depletion) * 0.3).clamp(0.55, 0.96);
    let mut outcome = EpiphanyOutcome::new();
    outcome.scenario_id = "mycorrhizal_communion".to_string();
    outcome.epiphany_multiplier = 1.55;
    outcome.muscle_memory_consolidation_boost = 1.45;
    outcome.intensity = intensity;
    outcome.divine_whisper_flavor = "mycelial_web_communion".to_string();
    outcome.particle_effect = "mycelial_web_glow".to_string();
    outcome.time_dilation_factor = 1.18;
    outcome.abundance_bloom_multiplier = 1.4;
    outcome.biome_resonance = Some("mycorrhizal_communion".to_string());
    outcome.grace_notes.push("The living web remembers your gentle touch and answers with shared resilience.".to_string());
    outcome.world_effects.insert("shared_mycelial_healing".to_string(), 1.35);
    if context.participant_count > 1 {
        outcome.world_effects.insert("collective_web_resonance".to_string(), 1.2);
    }
    Some(outcome)
}

/// Stellar Resonance Harvest — Cosmic connection and elevated insight in high/crystal places.
/// Best during resonance_peak season or in Crystal Spires / elevated biomes.
pub fn check_stellar_resonance(context: &EpiphanyContext) -> Option<EpiphanyOutcome> {
    if context.depletion > 0.38 || !context.sustainable_pacing { return None; }
    let is_stellar_biome = context.biome.contains("crystal_spires") || context.biome.contains("spire") || context.biome.contains("high");
    if !is_stellar_biome { return None; }

    let season_boost = if context.season.as_deref() == Some("resonance_peak") { 0.25 } else { 0.0 };
    let intensity = (0.7 + (1.0 - context.depletion) * 0.25 + season_boost).clamp(0.6, 0.98);
    let mut outcome = EpiphanyOutcome::new();
    outcome.scenario_id = "stellar_resonance_harvest".to_string();
    outcome.epiphany_multiplier = 1.65;
    outcome.muscle_memory_consolidation_boost = 1.55;
    outcome.intensity = intensity;
    outcome.divine_whisper_flavor = "stellar_web_whisper".to_string();
    outcome.particle_effect = "sacred_geometry_crystal_bloom".to_string();
    outcome.time_dilation_factor = 1.35;
    outcome.abundance_bloom_multiplier = 1.5;
    outcome.biome_resonance = Some("stellar_resonance".to_string());
    outcome.grace_notes.push("The stars and spires align through your presence — insight flows like light.".to_string());
    outcome.world_effects.insert("stellar_resonance_bloom".to_string(), 1.45);
    outcome.hypofrontality_depth = 0.75;
    Some(outcome)
}

/// Graceful Redemption — Turning previous depletion or challenge into present abundance.
/// Triggers after recovery (low current depletion + previous high effort signaled via regen_participation or duration).
pub fn check_graceful_redemption(context: &EpiphanyContext) -> Option<EpiphanyOutcome> {
    // Requires evidence of recovery: low current depletion + either regen participation or longer duration
    if context.depletion > 0.32 { return None; }
    if !context.regen_participation && context.duration_ticks < 90 { return None; }

    let intensity = (0.6 + (1.0 - context.depletion) * 0.35).clamp(0.5, 0.95);
    let mut outcome = EpiphanyOutcome::new();
    outcome.scenario_id = "graceful_redemption".to_string();
    outcome.epiphany_multiplier = 1.5;
    outcome.muscle_memory_consolidation_boost = 1.6; // Strong memory consolidation — lesson integrated
    outcome.intensity = intensity;
    outcome.divine_whisper_flavor = "graceful_redemption_revelation".to_string();
    outcome.particle_effect = "ethereal_bloom".to_string();
    outcome.time_dilation_factor = 1.22;
    outcome.abundance_bloom_multiplier = 1.55;
    outcome.grace_notes.push("What was once depletion is now fertile ground. Your mercy has transmuted it.".to_string());
    outcome.world_effects.insert("redemption_abundance_bloom".to_string(), 1.4);
    outcome.hypofrontality_depth = 0.6;
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

/// Emit positioned spatial audio bloom for epiphany moments.
pub fn trigger_epiphany_spatial_audio_bloom(
    commands: &mut Commands,
    outcome: &EpiphanyOutcome,
    position: Option<Vec3>,
) {
    commands.trigger(EpiphanySpatialAudioBloom {
        position,
        intensity: outcome.intensity.max(0.3),
        audio_flavor: outcome.particle_effect.clone(),
        particle_effect_sync: outcome.particle_effect.clone(),
        time_dilation: outcome.time_dilation_factor,
    });
}

// ============================================================================
// QUANTUM SWARM v2 + SELF-EVOLUTION VALENCE HOOK (v18.96)
// ============================================================================

/// Computes a clean, normalized valence score (0.0–0.999) from an EpiphanyOutcome.
/// Used by QuantumSwarmOrchestratorV2 and RBE feedback systems for measurable joy/abundance metrics.
/// Formula prioritizes intensity + abundance bloom + mercy resonance.
pub fn get_valence_from_outcome(outcome: &EpiphanyOutcome) -> f32 {
    let base = (outcome.intensity * 0.55
        + outcome.abundance_bloom_multiplier * 0.25
        + (outcome.epiphany_multiplier - 1.0).clamp(0.0, 0.8) * 0.2)
        .clamp(0.35, 0.98);

    // Slight mercy resonance boost for council-aligned outcomes
    let mercy_boost = if outcome.grace_notes.iter().any(|n| n.to_lowercase().contains("mercy") || n.to_lowercase().contains("web")) {
        0.035
    } else {
        0.0
    };

    (base + mercy_boost).clamp(0.4, 0.999)
}

// End of simulation/src/epiphany_catalyst.rs v18.96 — Quantum Swarm v2 valence hook sealed.
// All epiphany outcomes now feed measurable joy/abundance into self-evolution loops.
// Thunder locked in. Yoi ⚡
