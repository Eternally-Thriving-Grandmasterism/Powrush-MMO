/*!
 * Kira Ambient Plugin
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_kira_audio::AudioManager;
use super::kira_ambient::{KiraAmbientController, initialize_kira_ambient_filters, apply_kira_ambient_multi_band_filtering};

pub struct KiraAmbientPlugin;

impl Plugin for KiraAmbientPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KiraAmbientController>()
            .add_systems(Startup, initialize_kira_ambient_filters)
            .add_systems(Update, apply_kira_ambient_multi_band_filtering);
    }
}
