/*!
 * Audio Plugin - Music + Environmental Systems
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use super::music::{evaluate_music_state, update_music};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<super::music::MusicController>()
            .add_systems(Update, (
                evaluate_music_state,
                update_music,
            ));
    }
}
