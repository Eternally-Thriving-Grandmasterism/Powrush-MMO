/*!
 * Audio Debug UI + Trigger Systems
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy::diagnostic::Diagnostics;
use crate::settings::audio_mixing::{AudioCategory, Priority};
use crate::audio::events::AudioTrigger;
use crate::settings::audio_mixing::{
    AUDIO_MIXING_TIME,
    ACTIVE_DYNAMIC_AUDIO,
    CURRENT_DUCKING_LEVEL,
};

// ... existing debug UI code (AudioDebugUiVisible, toggle_audio_debug_ui, audio_debug_ui) ...

// === Example Gameplay Integration Systems ===

/// Emits AudioTrigger events based on combat state changes.
pub fn combat_audio_trigger_system(
    mut combat_events: EventReader<crate::audio::events::CombatStateChangedEvent>,
    mut audio_triggers: EventWriter<AudioTrigger>,
) {
    for event in combat_events.read() {
        if event.entering_combat {
            audio_triggers.send(AudioTrigger {
                priority: Priority::High,
                category: Some(AudioCategory::Sfx),
                intensity: Some(event.intensity),
                label: Some("combat_start".to_string()),
            });
        } else {
            audio_triggers.send(AudioTrigger {
                priority: Priority::Normal,
                category: Some(AudioCategory::Music),
                intensity: Some(0.6),
                label: Some("combat_end".to_string()),
            });
        }
    }
}

/// Emits AudioTrigger events when the player changes regions.
pub fn region_audio_trigger_system(
    mut region_events: EventReader<crate::audio::events::RegionTransitionEvent>,
    mut audio_triggers: EventWriter<AudioTrigger>,
) {
    for event in region_events.read() {
        let priority = match event.to_region {
            crate::audio::events::RegionType::Industrial
            | crate::audio::events::RegionType::Council => Priority::High,
            crate::audio::events::RegionType::Forest
            | crate::audio::events::RegionType::Wilderness => Priority::Normal,
            _ => Priority::Normal,
        };

        audio_triggers.send(AudioTrigger {
            priority,
            category: Some(AudioCategory::Ambient),
            intensity: Some(0.8),
            label: Some(format!("region_{:?}", event.to_region)),
        });
    }
}
