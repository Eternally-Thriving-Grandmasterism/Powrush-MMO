/*!
 * Audio Debug UI Overlay with F3 Toggle
 *
 * Provides a toggleable debug window showing real-time audio mixing diagnostics.
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy::diagnostic::Diagnostics;
use crate::settings::audio_mixing::{
    AUDIO_MIXING_TIME,
    ACTIVE_DYNAMIC_AUDIO,
    CURRENT_DUCKING_LEVEL,
};

/// Resource controlling the visibility of the Audio Mixing Debug UI.
///
/// Default is `false` (hidden). Press F3 to toggle.
#[derive(Resource, Default, Debug)]
pub struct AudioDebugUiVisible(pub bool);

impl AudioDebugUiVisible {
    /// Toggles the current visibility state.
    pub fn toggle(&mut self) {
        self.0 = !self.0;
    }
}

/// System that toggles the audio debug UI when the F3 key is pressed.
///
/// This system is intentionally lightweight and only reacts to key presses.
pub fn toggle_audio_debug_ui(
    mut visible: ResMut<AudioDebugUiVisible>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        visible.toggle();
    }
}

/// Renders the Audio Mixing Debug window using egui.
/// Only active when `AudioDebugUiVisible` is true.
pub fn audio_debug_ui(
    mut contexts: EguiContexts,
    diagnostics: Res<Diagnostics>,
    visible: Res<AudioDebugUiVisible>,
) {
    if !visible.0 {
        return;
    }

    egui::Window::new("Audio Mixing Debug")
        .default_pos([10.0, 100.0])
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Dynamic Audio Mixing");

            if let Some(mixing_time) = diagnostics.get_measurement(&AUDIO_MIXING_TIME) {
                ui.label(format!("Mixing Time: {:.3} ms", mixing_time.value));
            }

            if let Some(active_count) = diagnostics.get_measurement(&ACTIVE_DYNAMIC_AUDIO) {
                ui.label(format!("Active DynamicAudio: {:.0}", active_count.value));
            }

            if let Some(ducking_level) = diagnostics.get_measurement(&CURRENT_DUCKING_LEVEL) {
                ui.label(format!("Current Ducking Level: {:.2}", ducking_level.value));

                let progress = (1.0 - ducking_level.value).clamp(0.0, 1.0) as f32;
                ui.add(egui::ProgressBar::new(progress).text("Ducking Intensity"));
            }

            ui.separator();
            ui.label("Press F3 to toggle this window.");
            ui.label("Tune values in config/adaptive_audio.ron (hot-reloadable).");
        });
}
