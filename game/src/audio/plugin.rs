/*!
 * Music Plugin - Full Layered Music System
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use super::music::{evaluate_music_state, update_music, update_music_layers, MusicLayers};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<super::music::MusicController>()
            .init_resource::<MusicLayers>()
            .add_systems(Update, (
                evaluate_music_state,
                update_music,
                update_music_layers,
            ));
    }
}
