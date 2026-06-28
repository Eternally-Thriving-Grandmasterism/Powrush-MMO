/*!
 * Audio Plugin - With F3 Toggleable Debug UI
 */

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_egui::EguiPlugin;
use crate::settings::audio_mixing::{register_audio_diagnostics, update_dynamic_audio_volumes};
use crate::audio::debug_ui::{AudioDebugUiVisible, toggle_audio_debug_ui, audio_debug_ui};

// ... other imports ...

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DiagnosticsPlugin)
            .add_plugins(LogDiagnosticsPlugin::default())
            .add_plugins(EguiPlugin)
            .init_resource::<AudioDebugUiVisible>()
            .add_systems(Startup, register_audio_diagnostics)
            .add_systems(Update, (
                update_dynamic_audio_volumes,
                toggle_audio_debug_ui,
                audio_debug_ui,
            ));
    }
}
