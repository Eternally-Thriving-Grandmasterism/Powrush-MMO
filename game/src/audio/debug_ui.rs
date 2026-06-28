/*!
 * Audio Debug UI Overlay
 *
 * Uses bevy_egui to display real-time audio mixing diagnostics.
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy::diagnostic::Diagnostics;
use crate::settings::audio_mixing::{
    AUDIO_MIXING_TIME,
    ACTIVE_DYNAMIC_AUDIO,
    CURRENT_DUCKING_LEVEL,
};

pub fn audio_debug_ui(
    mut contexts: EguiContexts,
    diagnostics: Res<Diagnostics>,
) {
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

                // Visual bar for ducking intensity
                let progress = (1.0 - ducking_level.value).clamp(0.0, 1.0);
                ui.add(egui::ProgressBar::new(progress as f32)
                    .text("Ducking Intensity"));
            }

            ui.separator();
            ui.label("Edit config/adaptive_audio.ron to tune ducking curves live.");
        });
}
