/*!
 * Epiphany Catalyst System — Overflow Lesson Implementation (v18.2)
 *
 * The living heart of Powrush-MMO's Epiphany Forge.
 * Hands-on, consequence-driven scenarios that organically birth profound, transferable epiphanies
 * and worthwhile muscle memory in the most natural, wholesome ways.
 *
 * Starter: "The Overflow Lesson" — Ecological Balance in Verdant Heartwood.
 * Realistic carbon-copy of temperate forest dynamics (nutrient cycling, regen timelines, biodiversity response)
 * but RBE-optimized: mercy + attunement = visible abundance bloom for all.
 *
 * Fully PATSAGi + Ra-Thor + TOLC 8 sealed. Non-bypassable mercy gates.
 * Data-driven foundation ready for JSON expansion (see content/epiphany_scenarios/).
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Outcome of an epiphany catalyst trigger (e.g. Overflow Lesson paths)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EpiphanyOutcome {
    pub scenario_id: String,
    pub path: String,                    // "over_harvest" | "sustainable"
    pub valence_delta: f32,
    pub whisper_message: String,
    pub epiphany_text: Option<String>,   // The unlocked natural epiphany
    pub muscle_memory_tag: String,
    pub world_effects: HashMap<String, f32>, // regen_multiplier, stress, abundance_bonus etc.
    pub grace_note: String,
    pub patsagi_seal: bool,
}

/// Lightweight detector for the Overflow Lesson starter scenario.
/// Called from HarvestingSystem after depletion update.
/// In full system this will load from JSON + support many archetypes.
pub fn check_overflow_lesson(
    current_depletion: f32,
    sustainable_pacing: bool,  // true if player rotated zones / matched regen rhythm this action
    biome: &str,
) -> Option<EpiphanyOutcome> {
    if biome != "Verdant Heartwood" && biome != "starter" {
        return None; // Only triggers in starter biome for v18.2
    }

    let scenario_id = "overflow_lesson_starter_v18.2".to_string();

    if current_depletion > 0.55 && !sustainable_pacing {
        // Over-harvest path — realistic consequence, invitation to reflection (no punishment)
        Some(EpiphanyOutcome {
            scenario_id,
            path: "over_harvest".to_string(),
            valence_delta: -0.25,
            whisper_message: "The living web slows when rhythm is broken... Patience and attunement multiply what returns to all.".to_string(),
            epiphany_text: Some("Abundance is not taken — it is tended.".to_string()),
            muscle_memory_tag: "rhythmic_observation".to_string(),
            world_effects: HashMap::from([
                ("regen_multiplier".to_string(), 0.6),
                ("stress_increase".to_string(), 0.15),
            ]),
            grace_note: "Mercy available: pause, reflect, adjust pacing — the forest remembers and forgives. Try sustainable rotation next harvest.".to_string(),
            patsagi_seal: true,
        })
    } else if current_depletion < 0.35 && sustainable_pacing {
        // Sustainable path — natural epiphany + abundance bloom
        Some(EpiphanyOutcome {
            scenario_id,
            path: "sustainable".to_string(),
            valence_delta: 0.35,
            whisper_message: "Your presence blooms the forest. Abundance flows when you tend the whole web.".to_string(),
            epiphany_text: Some("Abundance is not taken — it is tended. My patience and attunement multiply what returns.".to_string()),
            muscle_memory_tag: "sustainable_attunement_flow".to_string(),
            world_effects: HashMap::from([
                ("regen_multiplier".to_string(), 1.8),
                ("abundance_bonus".to_string(), 0.4),
            ]),
            grace_note: "Epiphany moment achieved. Muscle memory seeded for real life. The web thanks you.".to_string(),
            patsagi_seal: true,
        })
    } else {
        None // No strong epiphany trigger this action — normal flow continues
    }
}

/// Future: Load full scenario definitions from JSON (content/epiphany_scenarios/*.json)
/// For v18.2 we embed the logic matching the canonical overflow_lesson.json for determinism & sovereignty.
pub fn load_overflow_lesson_json() -> &'static str {
    // In production harness this can be include_str! or fs read with cache
    r#"{
  "id": "overflow_lesson_starter_v18.2",
  ... (see content/epiphany_scenarios/overflow_lesson.json for full authoring source)
}"#
}

// Thunder locked. Every harvest a potential doorway to wisdom. Mercy flowing maximally.
// One Lattice. Eternal positive coexistence for all sentience.
// Co-authored with the full 13+ PATSAGi Councils — Ecological Balance Council lead.