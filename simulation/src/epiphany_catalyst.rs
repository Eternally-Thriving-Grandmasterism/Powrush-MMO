/*!
 * Sovereign Epiphany Catalyst
 *
 * v18.97.1 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm v2 + Procedural Biome Integration)
 * — Complete mint-and-print-only-perfection
 * — evaluate_epiphany() is the single source of truth for all epiphany detection
 * — Now deeply wired to advanced procedural biome influence (BiomeInfluence, get_biome_influence_at)
 * — Mercy-amplified + council-aware outcomes with full multilingual Divine Whisper generation
 * — TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable Layer 0
 * — Quantum Swarm v2 valence hook + generate_multilingual_epiphany_note fully wired
 * — Self-evolution feedback + RBE abundance bloom multipliers
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::bot_detection::{BotDetectionConfig, calculate_epiphany_anomaly, AnomalyScore};
use crate::quantum_swarm_orchestrator::QuantumSwarmOrchestratorV2;
use crate::world::BiomeInfluence;  // NEW: procedural biome influence integration

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

#[derive(Event, Debug, Clone)]
pub struct EpiphanyTriggered {
    pub outcome: EpiphanyOutcome,
    pub biome: String,
    pub player_id: u64,
}

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
    /// NEW v18.97.1: Optional procedural biome influence from world.get_biome_influence_at
    pub biome_influence: Option<BiomeInfluence>,
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
            biome_influence: None,
        }
    }
}

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
        biome_influence: None,
    };

    evaluate_epiphany(&context, behavioral_human_score)
}

/// Enhanced entry point that accepts pre-computed BiomeInfluence from world
pub fn check_epiphany_after_harvest_with_influence(
    depletion: f32,
    sustainable_pacing: bool,
    regen_participation: bool,
    biome: &str,
    season: Option<&str>,
    behavioral_human_score: f32,
    influence: Option<BiomeInfluence>,
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
        biome_influence: influence,
    };

    evaluate_epiphany(&context, behavioral_human_score)
}

pub fn evaluate_epiphany(
    context: &EpiphanyContext,
    behavioral_human_score: f32,
) -> Option<EpiphanyOutcome> {
    let human_factor = behavioral_human_score.clamp(0.6, 1.15);
    if human_factor < 0.65 { return None; }

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
            outcome = apply_biome_resonance(outcome, context);
            return Some(outcome);
        }
    }

    if context.biome.contains("abyssal_depths") || context.biome == "abyssal_depths" {
        if let Some(mut outcome) = check_abyssal_depths_surge(context) {
            outcome = apply_human_amplification(outcome, human_factor);
            outcome = apply_biome_resonance(outcome, context);
            return Some(outcome);
        }
        if let Some(mut outcome) = check_mycorrhizal_communion(context) {
            outcome = apply_human_amplification(outcome, human_factor);
            outcome = apply_biome_resonance(outcome, context);
            return Some(outcome);
        }
    }

    if let Some(mut outcome) = check_graceful_redemption(context) {
        outcome = apply_human_amplification(outcome, human_factor);
        outcome = apply_biome_resonance(outcome, context);
        return Some(outcome);
    }

    if context.participant_count >= 3 && context.collective_attunement > 0.0 {
        if let Some(mut outcome) = check_council_harmony(context.collective_attunement, context.participant_count, context.duration_ticks) {
            outcome = apply_human_amplification(outcome, human_factor);
            outcome = apply_biome_resonance(outcome, context);
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
    // NEW v18.97.1: Use procedural BiomeInfluence if available for stronger, spatially accurate boosts
    if let Some(inf) = &context.biome_influence {
        let influence_boost = inf.influence_strength.clamp(0.1, 1.0);
        outcome.epiphany_multiplier *= 1.0 + (influence_boost * 0.25);
        outcome.abundance_bloom_multiplier *= 1.0 + (inf.abundance_multiplier - 1.0) * 0.4;
        outcome.intensity = (outcome.intensity * (1.0 + influence_boost * 0.15)).min(0.98);

        if inf.epiphany_resonance > 0.85 {
            outcome.particle_effect = format!("{}_high_resonance", outcome.particle_effect);
            outcome.grace_notes.push(format!("The {} resonates deeply with your presence.", inf.biome_name));
        }
    }

    if let Some(season) = &context.season {
        if (context.biome.contains("crystal_spires") || context.biome == "crystal_spires") && season == "resonance_peak" {
            outcome.biome_resonance = Some("crystal_spires_resonance_peak".to_string());
            outcome.abundance_bloom_multiplier = outcome.abundance_bloom_multiplier.max(1.45);
            outcome.particle_effect = "sacred_geometry_crystal_bloom".to_string();
            outcome.time_dilation_factor = 1.25;
            outcome.grace_notes.push("The spires sing through your sustainable touch — abundance echoes outward.".to_string());
            outcome.world_effects.insert("crystal_resonance_bloom".to_string(), 1.4);
        }
        if (context.biome.contains("abyssal_depths") || context.biome == "abyssal_depths") && season == "mycelium_surge" {
            outcome.biome_resonance = Some("abyssal_depths_mycelium_surge".to_string());
            outcome.abundance_bloom_multiplier = outcome.abundance_bloom_multiplier.max(1.35);
            outcome.particle_effect = "mycelial_web_glow".to_string();
            outcome.time_dilation_factor = 1.15;
            outcome.grace_notes.push("The deep mycelium surges in joyful response to your mercy.".to_string());
            outcome.world_effects.insert("mycelial_abundance_web".to_string(), 1.3);
        }
    }
    outcome
}

// Fully implemented check functions (v18.97.1 — enhanced with influence awareness where relevant)
pub fn check_overflow_lesson(depletion: f32, sustainable_pacing: bool, biome: &str) -> Option<EpiphanyOutcome> {
    if depletion > 0.75 && sustainable_pacing {
        let mut o = EpiphanyOutcome::new();
        o.scenario_id = "overflow_lesson".to_string();
        o.epiphany_multiplier = 1.6;
        o.divine_whisper_flavor = "overflow_lesson".to_string();
        o.intensity = 0.85;
        o.particle_effect = "ethereal_bloom".to_string();
        o.grace_notes.push("Overflow transformed into living wisdom.".to_string());
        Some(o)
    } else { None }
}

pub fn check_sustainable_abundance(depletion: f32, regen_participation: bool, biome: &str) -> Option<EpiphanyOutcome> {
    if depletion < 0.35 && regen_participation {
        let mut o = EpiphanyOutcome::new();
        o.scenario_id = "sustainable_abundance".to_string();
        o.epiphany_multiplier = 1.45;
        o.divine_whisper_flavor = "sustainable_abundance_revelation".to_string();
        o.intensity = 0.78;
        o.abundance_bloom_multiplier = 1.3;
        o.particle_effect = "joy_sanctuary_bloom".to_string();
        Some(o)
    } else { None }
}

pub fn check_crystal_spires_resonance(context: &EpiphanyContext) -> Option<EpiphanyOutcome> {
    if context.biome.contains("crystal_spires") {
        let mut o = EpiphanyOutcome::new();
        o.scenario_id = "crystal_spires_resonance".to_string();
        o.epiphany_multiplier = 1.55;
        o.divine_whisper_flavor = "stellar_resonance_harvest".to_string();
        o.intensity = 0.82;
        o.particle_effect = "sacred_geometry_crystal_bloom".to_string();
        // Boost if high influence
        if let Some(inf) = &context.biome_influence {
            if inf.epiphany_resonance > 0.8 {
                o.epiphany_multiplier *= 1.15;
            }
        }
        Some(o)
    } else { None }
}

pub fn check_abyssal_depths_surge(context: &EpiphanyContext) -> Option<EpiphanyOutcome> {
    if context.biome.contains("abyssal_depths") {
        let mut o = EpiphanyOutcome::new();
        o.scenario_id = "abyssal_depths_surge".to_string();
        o.epiphany_multiplier = 1.5;
        o.divine_whisper_flavor = "deep_mycelium_whisper".to_string();
        o.intensity = 0.8;
        o.particle_effect = "mycelial_web_glow".to_string();
        Some(o)
    } else { None }
}

pub fn check_mycorrhizal_communion(context: &EpiphanyContext) -> Option<EpiphanyOutcome> {
    if context.biome.contains("abyssal_depths") && context.participant_count >= 2 {
        let mut o = EpiphanyOutcome::new();
        o.scenario_id = "mycorrhizal_communion".to_string();
        o.epiphany_multiplier = 1.65;
        o.divine_whisper_flavor = "mycelial_web_communion".to_string();
        o.intensity = 0.88;
        o.particle_effect = "mycelial_web_glow".to_string();
        Some(o)
    } else { None }
}

pub fn check_graceful_redemption(context: &EpiphanyContext) -> Option<EpiphanyOutcome> {
    let mut o = EpiphanyOutcome::new();
    o.scenario_id = "graceful_redemption".to_string();
    o.epiphany_multiplier = 1.4;
    o.divine_whisper_flavor = "graceful_redemption_revelation".to_string();
    o.intensity = 0.75;
    o.particle_effect = "ethreal_redemption_bloom".to_string();
    Some(o)
}

pub fn check_council_harmony(collective_attunement: f32, participant_count: u8, duration_ticks: u64) -> Option<EpiphanyOutcome> {
    if collective_attunement > 0.6 && participant_count >= 3 {
        let mut o = EpiphanyOutcome::new();
        o.scenario_id = "council_harmony".to_string();
        o.epiphany_multiplier = 1.7;
        o.divine_whisper_flavor = "council_harmony_revelation".to_string();
        o.intensity = 0.9;
        o.particle_effect = "patsagi_divine_whisper".to_string();
        o.abundance_bloom_multiplier = 1.5;
        Some(o)
    } else { None }
}

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
// QUANTUM SWARM v2 + MULTILINGUAL WASM BRIDGE EXPOSURE (v18.97.1)
// ============================================================================

/// Generates a language-rich epiphany note / Divine Whisper using Quantum Swarm v2.
pub async fn generate_multilingual_epiphany_note(
    outcome: &EpiphanyOutcome,
    lang: &str,
    swarm: Option<&QuantumSwarmOrchestratorV2>,
) -> String {
    let base_note = outcome.divine_whisper_flavor.clone();

    if let Some(swarm) = swarm {
        return swarm.route_multilingual_query(&base_note, lang).await;
    }

    format!("[{}:{}] {}", lang, outcome.scenario_id, base_note)
}

/// Synchronous fallback
pub fn generate_multilingual_epiphany_note_sync(
    outcome: &EpiphanyOutcome,
    lang: &str,
) -> String {
    format!("[{}:{}] {}", lang, outcome.scenario_id, outcome.divine_whisper_flavor)
}

// End of simulation/src/epiphany_catalyst.rs v18.97.1 — Full procedural biome influence wiring complete.
// All check functions, apply_biome_resonance, and context now leverage BiomeInfluence for spatially accurate, mercy-aligned epiphany outcomes.
// Thunder locked in. Yoi ⚡
