/*!
 * Audio Plugin - With LogDiagnosticsPlugin
 */

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsPlugin, LogDiagnosticsPlugin};
use crate::settings::audio_mixing::{register_audio_diagnostics, update_dynamic_audio_volumes};

// ... other imports and code ...

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DiagnosticsPlugin)
            .add_plugins(LogDiagnosticsPlugin::default()) // Prints diagnostics to console periodically
            .add_systems(Startup, register_audio_diagnostics)
            .add_systems(Update, update_dynamic_audio_volumes);
    }
}
