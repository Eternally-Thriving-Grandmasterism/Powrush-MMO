/*!
 * Kira Music Plugin - Multi-Band Filtering
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_kira_audio::AudioManager;
use super::kira_music::{KiraMusicController, update_kira_music, apply_kira_multi_band_filtering, initialize_kira_multi_band_filters};

pub struct KiraMusicPlugin;

impl Plugin for KiraMusicPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KiraMusicController>()
            .add_systems(Startup, initialize_kira_multi_band_filters)
            .add_systems(Update, (
                update_kira_music,
                apply_kira_multi_band_filtering,
            ));
    }
}
