/*!
 * Audio Plugin - Central wiring including IR Management for convolution reverb
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use super::music::{evaluate_music_state, update_music, update_music_layers, MusicLayers};
use super::procedural_reverb_estimation::{
    update_procedural_reverb_estimation,
    ReverbEstimationConfig,
    ProceduralReverbEstimate,
    AudioListener,
};
use super::ir_manager::{IrLibrary, CurrentImpulseResponse};
use crate::settings::audio_mixing::ReverbState;
use crate::settings::biome_acoustic::{load_biome_acoustic_profile, update_biome_acoustic_transition, CurrentBiomeAcoustics};
use shared::spatial::HierarchicalGrid;

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
            .add_systems(Startup, load_biome_acoustic_profile)
            .add_systems(Update, (
                evaluate_music_state,
                update_music,
                update_music_layers,
                update_procedural_reverb_estimation,
                update_biome_acoustic_transition,
            ));
    }
}
