/*!
 * Environmental Audio - Biome Acoustic Profile Serialization
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct AcousticParameters {
    pub wetness: f32,
    pub decay_time: f32,
    pub damping: f32,
    pub early_reflections: f32,
    pub influence_radius: f32,
}

impl Default for AcousticParameters {
    fn default() -> Self {
        Self {
            wetness: 0.4,
            decay_time: 2.0,
            damping: 0.4,
            early_reflections: 0.5,
            influence_radius: 35.0,
        }
    }
}

#[derive(Resource, Serialize, Deserialize, Default, Clone)]
pub struct BiomeAcousticProfile {
    pub forest: AcousticParameters,
    pub desert: AcousticParameters,
    pub stone_dungeon: AcousticParameters,
    pub cave: AcousticParameters,
    pub snow: AcousticParameters,
    pub swamp: AcousticParameters,
    pub open_field: AcousticParameters,
}

/// Loads BiomeAcousticProfile from RON file (with sensible defaults on failure)
pub fn load_biome_acoustic_profile(mut commands: Commands, asset_server: Res<AssetServer>) {
    let path = "assets/audio/biome_acoustics.ron";

    let profile = match fs::read_to_string(path) {
        Ok(content) => {
            match ron::from_str::<BiomeAcousticProfile>(&content) {
                Ok(p) => {
                    info!("Loaded biome acoustic profile from {}", path);
                    p
                }
                Err(e) => {
                    warn!("Failed to parse biome_acoustics.ron: {}. Using defaults.", e);
                    BiomeAcousticProfile::default()
                }
            }
        }
        Err(_) => {
            info!("No biome_acoustics.ron found. Using default acoustic profiles.");
            BiomeAcousticProfile::default()
        }
    };

    commands.insert_resource(profile);
}

// Example RON file content (assets/audio/biome_acoustics.ron):
/*
(
    forest: (
        wetness: 0.55,
        decay_time: 2.8,
        damping: 0.65,
        early_reflections: 0.6,
        influence_radius: 40.0,
    ),
    stone_dungeon: (
        wetness: 0.75,
        decay_time: 4.2,
        damping: 0.25,
        early_reflections: 0.85,
        influence_radius: 50.0,
    ),
    // ... other biomes
)
*/
