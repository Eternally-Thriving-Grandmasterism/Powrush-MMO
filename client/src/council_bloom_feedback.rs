/*!
 * Council Bloom Feedback — Client-Side (Optimized + Rich Experience)
 */

use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::replication::{CouncilBloomPayload, CouncilBloomReceived};

// Optional rich feedback (if these modules exist in your project)
// use crate::divine_whispers::DivineWhisperTrigger;
// use crate::camera_effects::CameraShake;
// use crate::spatial_audio::GameAudioEvent;

#[derive(Component)]
pub struct CouncilBloomEffect { /* ... */ }

#[derive(Resource)]
pub struct CouncilBloomParticleAssets { /* ... */ }

pub struct CouncilBloomFeedbackPlugin;

impl Plugin for CouncilBloomFeedbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CouncilBloomReceived>()
           .add_systems(Startup, setup_council_bloom_particles)
           .add_systems(Update, (
               process_council_bloom_received,
               despawn_old_bloom_effects,
               trigger_rich_bloom_feedback,
           ).chain());
    }
}

// ... existing setup, spawn, process, despawn functions ...

/// Triggers additional rich feedback (camera, whispers, sound) for strong blooms
fn trigger_rich_bloom_feedback(
    mut events: EventReader<CouncilBloomReceived>,
    // mut camera_shake: Option<ResMut<CameraShake>>,
    // mut whisper_writer: Option<EventWriter<DivineWhisperTrigger>>,
    // mut audio_events: Option<EventWriter<GameAudioEvent>>,
) {
    for event in events.read() {
        let p = &event.payload;

        if p.bloom_activated && p.collective_attunement_score > 0.75 {
            // Camera presence / shake for strong blooms
            // if let Some(shake) = &mut camera_shake {
            //     shake.intensity = (shake.intensity * 0.5 + 0.6).min(2.0);
            //     shake.duration = 2.8;
            // }

            // Divine Whisper / UI notification
            // if let Some(writer) = &mut whisper_writer {
            //     writer.send(DivineWhisperTrigger { ... });
            // }

            // Audio cue (spatial or kira)
            // if let Some(audio) = &mut audio_events {
            //     audio.send(GameAudioEvent::CouncilBloom { intensity: p.bloom_amplification_multiplier });
            // }

            info!("[Client] Rich Council Bloom feedback triggered (high attunement)");
        }
    }
}
