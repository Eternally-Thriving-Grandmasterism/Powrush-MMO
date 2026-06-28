/*!
 * Audio Plugin - With Latency Monitoring + Adaptive Layering System
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy::asset::AssetApp;
use super::music::{evaluate_music_state, update_music, update_music_layers, MusicLayers};
use super::procedural_reverb_estimation::{
    update_procedural_reverb_estimation,
    ReverbEstimationConfig,
    ProceduralReverbEstimate,
    AudioListener,
};
use super::ir_manager::{IrLibrary, CurrentImpulseResponse};
use super::ir_asset::{load_ir_library_from_ron, IrAssetLoader};
use super::ir_metrics::IrTruncationMetrics;
use super::spatial_metrics::SpatialAudioMetrics;
use super::latency_metrics::AudioLatencyMetrics;
use crate::settings::audio_mixing::ReverbState;
use crate::settings::biome_acoustic::{load_biome_acoustic_profile, update_biome_acoustic_transition, CurrentBiomeAcoustics};
use crate::settings::audio_quality::AudioQualitySettings;
use shared::spatial::HierarchicalGrid;

// Adaptive Layering modules (declared here for sibling visibility; integrates with music + kira crossfades)
mod adaptive_layering;
mod events;

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
            // === Adaptive Layering System resources & events ===
            .init_resource::<adaptive_layering::AdaptiveLayeringState>()
            .init_resource::<adaptive_layering::AdaptiveAudioConfig>()
            .add_event::<events::PaletteTransitionEvent>()
            .add_event::<events::CombatStateChangedEvent>()
            .add_event::<events::RegionTransitionEvent>()
            .register_asset_loader(IrAssetLoader)
            .add_systems(Startup, (load_biome_acoustic_profile, load_ir_library_from_ron))
            .add_systems(Update, (
                evaluate_music_state,
                update_music,
                update_music_layers,
                update_procedural_reverb_estimation,
                update_biome_acoustic_transition,
                super::ir_asset::process_loaded_ir_assets,
                // Adaptive layering drives unified palette/intensity/ramp transitions
                adaptive_layering::adaptive_layering_system,
            ));
    }
}
