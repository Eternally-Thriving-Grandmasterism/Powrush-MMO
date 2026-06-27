/*!
 * Music Plugin - Dynamic Music with Gameplay Integration
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use super::music::{evaluate_music_state, update_music, handle_victory_timeout};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<super::music::MusicController>()
            .add_systems(Update, (
                evaluate_music_state,
                update_music,
                handle_victory_timeout,
            ));
    }
}
