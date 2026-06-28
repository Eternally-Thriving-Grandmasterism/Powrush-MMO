/*!
 * Council Bloom Feedback — Rich Client Experience
 * Fully connected to camera, whispers, audio, and UI notification.
 */

use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::replication::{CouncilBloomPayload, CouncilBloomReceived};

// Rich feedback systems from the project
use crate::divine_whispers::DivineWhisperTrigger;
use crate::spatial_audio::GameAudioEvent;

// Camera shake resource (adjust path if needed)
#[derive(Resource, Default)]
pub struct CameraShake {
    pub intensity: f32,
    pub duration: f32,
    pub timer: f32,
}

#[derive(Component)]
pub struct CouncilBloomEffect { /* ... */ }

#[derive(Resource)]
pub struct CouncilBloomParticleAssets { /* ... */ }

#[derive(Event, Clone, Debug)]
pub struct CouncilBloomNotification {
    pub message: String,
    pub attunement: f32,
}

pub struct CouncilBloomFeedbackPlugin;

impl Plugin for CouncilBloomFeedbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CouncilBloomReceived>()
           .add_event::<CouncilBloomNotification>()
           .init_resource::<CameraShake>()
           .add_systems(Startup, setup_council_bloom_particles)
           .add_systems(Update, (
               process_council_bloom_received,
               despawn_old_bloom_effects,
               trigger_rich_bloom_feedback,
               show_bloom_notification_ui,
           ).chain());
    }
}

// ... (keep existing setup, spawn, process, despawn functions) ...

/// Rich multi-sensory feedback for strong Council Blooms
fn trigger_rich_bloom_feedback(
    mut events: EventReader<CouncilBloomReceived>,
    mut camera_shake: ResMut<CameraShake>,
    mut whisper_writer: EventWriter<DivineWhisperTrigger>,
    mut audio_writer: EventWriter<GameAudioEvent>,
) {
    for event in events.read() {
        let p = &event.payload;

        if p.bloom_activated && p.collective_attunement_score > 0.72 {
            // Camera shake / presence
            camera_shake.intensity = (camera_shake.intensity * 0.4 + 0.65).min(2.2);
            camera_shake.duration = 3.0;
            camera_shake.timer = 0.0;

            // Divine Whisper
            whisper_writer.send(DivineWhisperTrigger {
                text: format!(
                    "The Council resonates... collective attunement {:.0}%",
                    p.collective_attunement_score * 100.0
                ),
                intensity: 0.85,
                duration_seconds: 7.0,
                is_epiphany: true,
                ..default()
            });

            // Audio cue
            audio_writer.send(GameAudioEvent::CouncilBloom {
                intensity: p.bloom_amplification_multiplier,
            });

            info!("[Client] Rich Council Bloom feedback triggered");
        }
    }
}

/// Simple egui notification popup for blooms
fn show_bloom_notification_ui(
    mut contexts: EguiContexts,
    mut events: EventReader<CouncilBloomNotification>,
) {
    for event in events.read() {
        // This will show in the next frame - for a real toast you would use a state + timer
        egui::Window::new("Council Bloom")
            .collapsible(false)
            .resizable(false)
            .show(contexts.ctx_mut(), |ui| {
                ui.label(&event.message);
                ui.label(format!("Attunement: {:.1}%", event.attunement * 100.0));
            });
    }
}
