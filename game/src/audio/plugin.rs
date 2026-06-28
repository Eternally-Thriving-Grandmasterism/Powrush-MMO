/*!
 * Audio Plugin - Central wiring for music, ambient, procedural reverb, and spatial audio
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use super::music::{evaluate_music_state, update_music, update_music_layers, MusicLayers};
use super::procedural_reverb_estimation::{
    update_procedural_reverb_estimation,
    ReverbEstimationConfig,
    ProceduralReverbEstimate,
};
use crate::settings::audio_mixing::ReverbState;
use shared::spatial::HierarchicalGrid;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MusicLayers>()
            .init_resource::<super::music::MusicController>()
            .init_resource::<ReverbEstimationConfig>()
            .init_resource::<ProceduralReverbEstimate>()
            .init_resource::<ReverbState>()
            // Initialize HierarchicalGrid for procedural reverb ray queries
            // In production this can be populated from server replication or client spatial system
            .init_resource::<HierarchicalGrid>()
            .add_systems(Update, (
                evaluate_music_state,
                update_music,
                update_music_layers,
                update_procedural_reverb_estimation,
            ));
    }
}
