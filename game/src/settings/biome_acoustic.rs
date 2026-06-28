/*!
 * Biome Acoustic Profiles
 *
 * Data-driven acoustic properties per biome for procedural reverb and spatial audio.
 * RON serialization + loading system.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Acoustic properties for a single biome.
#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct BiomeAcousticProfile {
    pub name: String,
    pub base_absorption_low: f32,
    pub base_absorption_high: f32,
    pub reverb_wetness: f32,
    pub early_reflection_factor: f32,
    pub material_tags: Vec<String>,
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

/// Resource holding the currently active biome acoustics.
#[derive(Resource, Default, Clone)]
pub struct CurrentBiomeAcoustics {
    pub active_profile: BiomeAcousticProfile,
    pub transition_amount: f32,
}

/// System that loads a biome acoustic profile from RON.
pub fn load_biome_acoustic_profile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Example: load from assets/biomes/forest.acoustic.ron
    // In production this would be driven by world/biome system
    let profile: BiomeAcousticProfile = ron::from_str(
        r#"
        (
            name: "forest",
            base_absorption_low: 0.38,
            base_absorption_high: 0.52,
            reverb_wetness: 0.65,
            early_reflection_factor: 1.3,
            material_tags: ["wood", "leaves", "dirt"],
        )
        "#,
    ).unwrap_or_default();

    commands.insert_resource(CurrentBiomeAcoustics {
        active_profile: profile,
        transition_amount: 1.0,
    });
}

/// Simple system to transition between biome profiles (placeholder for now)
pub fn update_biome_acoustic_transition(
    mut current: ResMut<CurrentBiomeAcoustics>,
    time: Res<Time>,
) {
    // In real implementation this would blend between profiles based on player location
    current.transition_amount = (current.transition_amount + time.delta_secs() * 0.5).min(1.0);
}
