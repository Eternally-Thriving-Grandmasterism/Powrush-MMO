/*!
 * Audio Plugin - Hot Reload for Region Scripts with live re-application
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
use crate::settings::audio_mixing::ReverbState;
use crate::settings::biome_acoustic::{load_biome_acoustic_profile, update_biome_acoustic_transition, CurrentBiomeAcoustics};
use crate::settings::audio_quality::AudioQualitySettings;
use shared::spatial::HierarchicalGrid;

mod adaptive_layering;
mod events;
mod kira_ambient;
mod kira_music;

pub use adaptive_layering::{
    AdaptiveLayeringState, AdaptiveAudioConfig, calculate_dynamic_ramp_time,
    AudioContext, EmotionalWeight, adaptive_layering_system, request_combat_palette,
    region_audio_transition_system, palette_to_music_mapping_system,
    feed_combat_intensity, combat_intensity_system,
    RegionPaletteConfig, RegionPaletteConfigHandle, load_region_palette_config,
    RegionPaletteLoader, hot_reload_region_palette_system, CurrentRegion,
};
pub use events::{PaletteTransitionEvent, PaletteType, TransitionPriority, RegionTransitionEvent, RegionType, CombatStateChangedEvent};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MusicLayers>()
            .init_resource::<super::music::MusicController>()
            .init_resource::<ReverbEstimationConfig>()
            .init_resource::<ProceduralReverbEstimate>()
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
            .init_resource::<AdaptiveLayeringState>()
            .init_resource::<AdaptiveAudioConfig>()
            .init_resource::<CurrentRegion>()
            .init_asset::<adaptive_layering::RegionPaletteConfig>()
            .init_resource::<adaptive_layering::RegionPaletteConfigHandle>()
            .register_asset_loader(adaptive_layering::RegionPaletteLoader)
            .add_event::<PaletteTransitionEvent>()
            .add_event::<CombatStateChangedEvent>()
            .add_event::<RegionTransitionEvent>()
            .register_asset_loader(IrAssetLoader)
            .add_systems(Startup, (
                load_biome_acoustic_profile,
                load_ir_library_from_ron,
                adaptive_layering::load_region_palette_config,
            ))
            .add_systems(Update, (
                evaluate_music_state, update_music, update_music_layers,
                update_procedural_reverb_estimation, update_biome_acoustic_transition,
                super::ir_asset::process_loaded_ir_assets,
                adaptive_layering_system,
                region_audio_transition_system,
                palette_to_music_mapping_system,
                combat_intensity_system,
                adaptive_layering::hot_reload_region_palette_system,
            ));
    }
}
