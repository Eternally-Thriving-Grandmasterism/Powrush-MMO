/*!
 * Environmental Audio - Biome Transition Logic
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum BiomeType {
    Forest,
    Desert,
    StoneDungeon,
    Cave,
    Snow,
    Swamp,
    OpenField,
}

#[derive(Resource)]
pub struct BiomeTransition {
    pub current_biome: BiomeType,
    pub previous_biome: BiomeType,
    pub transition_timer: f32,
    pub transition_duration: f32,
}

impl Default for BiomeTransition {
    fn default() -> Self {
        Self {
            current_biome: BiomeType::OpenField,
            previous_biome: BiomeType::OpenField,
            transition_timer: 0.0,
            transition_duration: 3.5, // seconds to cross biome boundary
        }
    }
}

/// Smoothly blends acoustic parameters during biome transitions
pub fn apply_biome_transition(
    mut reverb_state: ResMut<crate::settings::audio_mixing::ReverbState>,
    biome_profile: Res<BiomeAcousticProfile>,
    mut transition: ResMut<BiomeTransition>,
    time: Res<Time>,
) {
    if transition.current_biome == transition.previous_biome {
        // No transition happening - apply current biome directly
        let params = get_acoustic_params(&biome_profile, transition.current_biome);
        reverb_state.wetness = params.wetness;
        reverb_state.decay_time = params.decay_time;
        reverb_state.damping = params.damping;
        return;
    }

    transition.transition_timer += time.delta_seconds();
    let t = (transition.transition_timer / transition.transition_duration).clamp(0.0, 1.0);

    let from = get_acoustic_params(&biome_profile, transition.previous_biome);
    let to = get_acoustic_params(&biome_profile, transition.current_biome);

    // Smooth lerp between previous and current biome acoustics
    reverb_state.wetness = from.wetness.lerp(to.wetness, t);
    reverb_state.decay_time = from.decay_time.lerp(to.decay_time, t);
    reverb_state.damping = from.damping.lerp(to.damping, t);

    if t >= 1.0 {
        transition.previous_biome = transition.current_biome;
        transition.transition_timer = 0.0;
    }
}

fn get_acoustic_params(profile: &BiomeAcousticProfile, biome: BiomeType) -> AcousticParameters {
    match biome {
        BiomeType::Forest => profile.forest,
        BiomeType::Desert => profile.desert,
        BiomeType::StoneDungeon => profile.stone_dungeon,
        BiomeType::Cave => profile.cave,
        BiomeType::Snow => profile.snow,
        BiomeType::Swamp => profile.swamp,
        BiomeType::OpenField => profile.open_field,
    }
}

/// Call this when your biome detection system detects a change
pub fn trigger_biome_change(
    mut transition: ResMut<BiomeTransition>,
    new_biome: BiomeType,
) {
    if transition.current_biome != new_biome {
        transition.previous_biome = transition.current_biome;
        transition.current_biome = new_biome;
        transition.transition_timer = 0.0;
    }
}
