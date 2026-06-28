/*!
 * Audio Debug UI Overlay with F3 Toggle
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy::diagnostic::Diagnostics;
use crate::settings::audio_mixing::{
    AUDIO_MIXING_TIME,
    ACTIVE_DYNAMIC_AUDIO,
    CURRENT_DUCKING_LEVEL,
};

/// Resource to control visibility of the audio debug UI.
#[derive(Resource, Default)]
pub struct AudioDebugUiVisible(pub bool);

/// System that toggles the debug UI visibility when F3 is pressed.
pub fn toggle_audio_debug_ui(
    mut visible: ResMut<AudioDebugUiVisible>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::F3) {
        visible.0 = !visible.0;
    }
}

/// Main debug UI system. Only renders when `AudioDebugUiVisible` is true.
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
            ui.label("Press F3 to hide this window.");
            ui.label("Edit config/adaptive_audio.ron to tune ducking curves live.");
        });
}
