/*!
 * Audio Plugin - With Diagnostics
 */

use bevy::prelude::*;
use bevy::diagnostic::DiagnosticsPlugin;
use crate::settings::audio_mixing::{register_audio_diagnostics, update_dynamic_audio_volumes};

// ...

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DiagnosticsPlugin) // Usually already included via DefaultPlugins
            .add_systems(Startup, register_audio_diagnostics)
            .add_systems(Update, update_dynamic_audio_volumes);
    }
}
