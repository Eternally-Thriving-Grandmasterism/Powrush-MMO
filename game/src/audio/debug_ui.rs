/*!
 * AudioTrigger handler + Debug UI
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy::diagnostic::Diagnostics;
use crate::settings::audio_mixing::{AudioCategory, Priority, DynamicAudio};
use crate::audio::events::AudioTrigger;
use crate::settings::audio_mixing::{
    AUDIO_MIXING_TIME, ACTIVE_DYNAMIC_AUDIO, CURRENT_DUCKING_LEVEL,
};

// ... existing debug UI code ...

/// Handles AudioTrigger events by spawning audio with the correct DynamicAudio component.
pub fn audio_trigger_handler(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut audio_triggers: EventReader<AudioTrigger>,
) {
    for trigger in audio_triggers.read() {
        let category = trigger.category.unwrap_or(AudioCategory::Sfx);
        let volume = trigger.intensity.unwrap_or(1.0);

        if let Some(path) = &trigger.sound_path {
            let sound = asset_server.load(path);

            commands.spawn((
                AudioBundle {
                    source: sound,
                    settings: PlaybackSettings::ONCE.with_volume(volume),
                },
                DynamicAudio {
                    category,
                    priority: trigger.priority,
                },
            ));

            #[cfg(debug_assertions)]
            if let Some(label) = &trigger.label {
                info!("[Audio] Triggered '{}' with priority {:?}", label, trigger.priority);
            }
        } else {
            // No sound path provided — could be used for priority-only signals in the future
            #[cfg(debug_assertions)]
            if let Some(label) = &trigger.label {
                debug!("[Audio] Received AudioTrigger '{}' (no sound) with priority {:?}", label, trigger.priority);
            }
        }
    }
}
