/*!
 * Sovereign Epiphany Catalyst v18.2 + Overflow Lesson Forge
 *
 * Core of the autotelic epiphany pipeline for Powrush-MMO.
 * Every sustainable harvest that respects rhythm, mercy, and the living web can trigger an Overflow Lesson — a revelatory moment of insight, muscle memory consolidation, and deeper attunement with the RBE-aligned ecology.
 *
 * This module produces the base EpiphanyOutcome that is then richly layered by Receptor Bloom (v18.8) and Flow State Forge (v18.14).
 * Non-intrusive, carbon-copy feel. 100% mercy-gated. TOLC 8 Layer 0 enforced.
 * Co-authored with Sherif / Autonomicity Games Inc. + Flow State + Receptor + PATSAGi Councils.
 * Mint-and-Print-Only-Perfection.
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The living outcome of an Overflow Lesson epiphany.
/// Rich data for simulation, client visuals (particles, time dilation), Divine Whispers, wisdom journal, and muscle memory transfer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EpiphanyOutcome {
    pub scenario_id: String,
    pub epiphany_multiplier: f32,                 // 1.0+ compounds insight probability
    pub muscle_memory_consolidation_boost: f32,   // transferable godlike intuition in future similar contexts
    pub hypofrontality_depth: f32,                // depth of ego-dissolution / presence window (0.0–1.0+)
    pub particle_effect: String,                  // client hook: "overflow_lesson_bloom", etc.
    pub time_dilation_factor: f32,                // >1.0 = seamless time distortion during revelation
    pub divine_whisper_flavor: String,            // "sustainable_harmony_revelation", "ecstatic_overflow", etc.
    pub world_effects: HashMap<String, f32>,      // abundance_bloom, stress_increase, regen_multiplier, etc.
    pub grace_notes: Vec<String>,                 // loving, non-coercive messages
    pub overflow_lesson_type: String,             // "sustainable_harvest", "rhythmic_attunement", etc.
    pub intensity: f32,
}

impl EpiphanyOutcome {
    pub fn new() -> Self {
        Self {
            scenario_id: "base_overflow_lesson".to_string(),
            epiphany_multiplier: 1.0,
            muscle_memory_consolidation_boost: 1.0,
            hypofrontality_depth: 0.0,
            particle_effect: "default".to_string(),
            time_dilation_factor: 1.0,
            divine_whisper_flavor: "sustainable_presence".to_string(),
            world_effects: HashMap::new(),
            grace_notes: vec![],
            overflow_lesson_type: "none".to_string(),
            intensity: 0.0,
        }
    }
}

/// Core detector for Overflow Lessons.
/// Triggers only on sustainable pacing (mercy > threshold) + low-to-moderate depletion in supported biomes.
/// Produces base epiphany that Receptor Bloom and Flow State Forge will enrich.
pub fn check_overflow_lesson(
    depletion: f32,
    sustainable_pacing: bool,
    biome: &str,
) -> Option<EpiphanyOutcome> {
    if !sustainable_pacing {
        return None;
    }
    if depletion > 0.58 {
        return None; // too depleted — realistic friction, invitation to rest/regenerate
    }
    if biome != "Verdant Heartwood" && biome != "starter" && biome != "heartwood" {
        return None;
    }

    let intensity = ((1.0 - depletion * 1.1).max(0.25)).min(0.92);
    let epiphany_mult = 1.0 + intensity * 1.35;
    let muscle_boost = 1.0 + intensity * 0.95;
    let hypo_depth = intensity * 0.78;

    let mut world_effects = HashMap::new();
    world_effects.insert("abundance_bloom".to_string(), intensity * 0.42);
    if depletion > 0.35 {
        world_effects.insert("stress_increase".to_string(), 0.04);
    } else {
        world_effects.insert("regen_multiplier".to_string(), 1.0 + intensity * 0.35);
    }

    let grace = if intensity > 0.7 {
        "Overflow Lesson received: The living web rewards sustained, merciful presence with deeper revelation and transferable mastery."
    } else {
        "Sustainable rhythm opens a gentle window of insight. The web remembers your harmony."
    };

    Some(EpiphanyOutcome {
        scenario_id: format!("overflow_lesson_{}_{:.2}", biome, depletion),
        epiphany_multiplier: epiphany_mult,
        muscle_memory_consolidation_boost: muscle_boost,
        hypofrontality_depth: hypo_depth,
        particle_effect: "overflow_lesson_bloom".to_string(),
        time_dilation_factor: 1.0 + intensity * 0.55,
        divine_whisper_flavor: "sustainable_harmony_revelation".to_string(),
        world_effects,
        grace_notes: vec![grace.to_string()],
        overflow_lesson_type: "sustainable_harvest".to_string(),
        intensity,
    })
}
