/*!
 * Audio Plugin - Central wiring for music, ambient, and procedural reverb estimation
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

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MusicLayers>()
            .init_resource::<super::music::MusicController>()
            .init_resource::<ReverbEstimationConfig>()
            .init_resource::<ProceduralReverbEstimate>()
            .init_resource::<ReverbState>()
            .add_systems(Update, (
                evaluate_music_state,
                update_music,
                update_music_layers,
                update_procedural_reverb_estimation,
            ));
    }
}

pub struct MusicPlugin; // Legacy alias if needed

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        // Delegates to AudioPlugin or kept for compatibility
    }
}
