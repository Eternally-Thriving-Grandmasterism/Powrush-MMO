/*!
 * Settings Plugin - Includes Biome Transition Logic
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use super::{persistence, GameSettings, editor, audio_mixing, environmental_audio};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSettings>()
            .init_resource::<audio_mixing::AudioMixer>()
            .init_resource::<environmental_audio::BiomeTransition>()
            .add_systems(Startup, (
                persistence::load_settings,
                environmental_audio::load_biome_acoustic_profile,
            ))
            .add_systems(Update, (
                audio_mixing::apply_audio_settings,
                audio_mixing::update_dynamic_audio_volumes,
                environmental_audio::apply_unified_spatial_audio,
                environmental_audio::update_weather_ambients,
                environmental_audio::apply_deep_weather_effects,
                environmental_audio::apply_thunder_reverb_boost,
                environmental_audio::update_procedural_reverb,
                environmental_audio::blend_acoustic_probes,
                environmental_audio::apply_biome_transition,
            ));
    }
}
