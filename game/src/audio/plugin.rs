/*!
 * Audio Plugin - With Dynamic Audio Mixing
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy::asset::AssetApp;
use super::music::{evaluate_music_state, update_music, update_music_layers, MusicLayers};
use super::procedural_reverb_estimation::{update_procedural_reverb_estimation, ReverbEstimationConfig, ProceduralReverbEstimate, AudioListener};
use super::ir_manager::{IrLibrary, CurrentImpulseResponse};
use super::ir_asset::{load_ir_library_from_ron, IrAssetLoader};
use super::ir_metrics::IrTruncationMetrics;
use super::spatial_metrics::SpatialAudioMetrics;
use super::latency_metrics::AudioLatencyMetrics;
use crate::settings::audio_mixing::{AudioMixer, update_dynamic_audio_volumes};
use crate::settings::audio_mixing::DynamicAudio;
use crate::settings::biome_acoustic::{load_biome_acoustic_profile, update_biome_acoustic_transition, CurrentBiomeAcoustics};
use crate::settings::audio_quality::AudioQualitySettings;
use shared::spatial::HierarchicalGrid;

mod adaptive_layering;
mod events;
mod kira_ambient;
mod kira_music;

pub use adaptive_layering::{ /* ... all previous exports ... */ };

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MusicLayers>()
            .init_resource::<super::music::MusicController>()
            .init_resource::<ReverbEstimationConfig>()
            .init_resource::<ProceduralReverbEstimate>()
            .init_resource::<AudioMixer>()
            .init_resource::<ReverbState>()
            .init_resource::<HierarchicalGrid>()
            .init_resource::<CurrentBiomeAcoustics>()
            .init_resource::<AudioListener>()
            .init_resource::<IrLibrary>()
            .init_resource::<CurrentImpulseResponse>()
            .init_resource::<AudioQualitySettings>()
            .init_resource::<IrTruncationMetrics>()
            .init_resource::<SpatialAudioMetrics>()
            .init_resource::<AudioLatencyMetrics>()
            .init_resource::<AudioEventMetrics>()
            .init_resource::<AdaptiveLayeringState>()
            .init_resource::<AdaptiveAudioConfig>()
            // ... asset registrations ...
            .add_systems(Update, (
                evaluate_music_state,
                update_music,
                update_music_layers,
                update_procedural_reverb_estimation,
                update_biome_acoustic_transition,
                super::ir_asset::process_loaded_ir_assets,
                adaptive_layering_system,
                region_audio_transition_system,
                palette_to_music_mapping_system,
                combat_intensity_system,
                hot_reload_region_palette_system,
                hot_reload_ai_config_system,
                on_region_palette_config_reloaded,
                on_ai_config_reloaded,
                // Core audio mixing
                update_dynamic_audio_volumes,
            ));
    }
}
