/*!
 * Biome Acoustic Profiles
 *
 * Data-driven acoustic properties per biome for procedural reverb and spatial audio.
 * Supports RON serialization for easy content authoring.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Acoustic properties for a single biome.
#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct BiomeAcousticProfile {
    pub name: String,
    pub base_absorption_low: f32,   // Low frequency absorption (0.0 - 1.0)
    pub base_absorption_high: f32,  // High frequency absorption
    pub reverb_wetness: f32,        // Default wetness/scattering
    pub early_reflection_factor: f32,
    pub material_tags: Vec<String>, // e.g. ["stone", "wood", "water", "forest"]
}

impl Default for BiomeAcousticProfile {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            base_absorption_low: 0.25,
            base_absorption_high: 0.35,
            reverb_wetness: 0.5,
            early_reflection_factor: 1.0,
            material_tags: vec!["generic".to_string()],
        }
    }
}

/// Resource that holds the current active biome profile(s).
/// Can be extended to support multiple overlapping biomes or transitions.
#[derive(Resource, Default, Clone)]
pub struct CurrentBiomeAcoustics {
    pub active_profile: BiomeAcousticProfile,
    pub transition_amount: f32, // 0.0 = old, 1.0 = fully new
}
