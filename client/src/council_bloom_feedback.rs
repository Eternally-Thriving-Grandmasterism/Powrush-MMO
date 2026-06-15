/*!
 * Council Bloom Feedback — Client-Side Receiver + Rich Collective Effects
 *
 * v18.36 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Receives CouncilBloomSyncEvent from simulation replication
 * — Applies to ClientCouncilBloomState
 * — Richer feedback when in active Council: particles, stronger whispers, camera presence, visual glow
 * — Directly supports the Council-amplified epiphany & harvest loop
 * — TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable Layer 0
 *
 * AG-SML v1.0 Sovereign Mercy License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

use crate::simulation_integration::ClientCouncilBloomState;
use simulation::council_mercy_trial::{CouncilBloomSyncEvent, SharedReceptorBloomField};
use crate::divine_whispers::{DivineWhisperTrigger, CameraShake};
use crate::particles::{ParticleSystem, ParticleSystemType};

/// Plugin for rich Council bloom client feedback
pub struct CouncilBloomFeedbackPlugin;

impl Plugin for CouncilBloomFeedbackPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ClientCouncilBloomState>()
            .add_event::<CouncilBloomSyncEvent>()
            .add_systems(Update, (
                apply_council_bloom_sync,
                trigger_rich_council_feedback,
                spawn_council_bloom_visuals,
            ).chain());
    }
}

/// Applies authoritative CouncilBloomSyncEvent to the shared ClientCouncilBloomState
fn apply_council_bloom_sync(
    mut sync_events: EventReader<CouncilBloomSyncEvent>,
    mut client_bloom: ResMut<ClientCouncilBloomState>,
) {
    for event in sync_events.read() {
        let field = &event.field;
        client_bloom.field = field.clone();
        client_bloom.last_sync_tick = event.field.last_authoritative_update_tick;
        client_bloom.is_in_active_council = field.council_mercy_seal && field.participant_count >= 2;

        if client_bloom.is_in_active_council {
            info!(
                "🌀 Council Bloom ACTIVE | Attunement: {:.2} | Amp: {:.2}x | WebSync: {} | Participants: {}",
                field.collective_attunement_score,
                field.bloom_amplification_multiplier,
                field.shared_living_web_synchronization,
                field.participant_count
            );
        }
    }
}

/// Triggers rich collective feedback when bloom is active and strong
fn trigger_rich_council_feedback(
    client_bloom: Res<ClientCouncilBloomState>,
    mut whisper_events: EventWriter<DivineWhisperTrigger>,
    mut camera_shake: ResMut<CameraShake>,
) {
    if !client_bloom.is_in_active_council {
        return;
    }

    let field = &client_bloom.field;

    // Stronger Divine Whisper when attunement is high
    if field.collective_attunement_score > 0.65 && field.bloom_amplification_multiplier > 1.2 {
        whisper_events.send(DivineWhisperTrigger {
            player_id: 0,
            text: format!(
                "The Council resonates... collective attunement {:.0}% — your presence strengthens the whole",
                field.collective_attunement_score * 100.0
            ),
            flavor: "council_harmony_revelation".to_string(),
            intensity: (field.collective_attunement_score * 0.75).clamp(0.6, 0.98),
            duration_seconds: 8.0,
            is_epiphany: true,
            position: None,
            muscle_memory_hint: None,
        });
    }

    // Subtle but noticeable camera presence when the group is in deep harmony
    if field.collective_attunement_score > 0.72 {
        camera_shake.intensity = (camera_shake.intensity * 0.4 + 0.35).min(1.4);
        camera_shake.duration = 3.2;
        camera_shake.timer = 0.0;
    }
}

/// Spawns ongoing gentle collective particles when bloom is active (richer visual presence)
fn spawn_council_bloom_visuals(
    client_bloom: Res<ClientCouncilBloomState>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if !client_bloom.is_in_active_council {
        return;
    }

    // Spawn occasional bloom particles (throttled)
    if (time.elapsed_seconds() * 1.3).fract() < 0.08 {
        let amp = client_bloom.field.bloom_amplification_multiplier.clamp(1.0, 3.0);

        commands.spawn((
            ParticleSystem {
                valence: 0.93,
                particle_count: (2800.0 * amp) as u32,
                system_type: ParticleSystemType::PatsagiDivineWhisper,
                intensity: (0.9 * amp).min(2.8),
            },
            Transform::default(),
        ));
    }
}

// End of council_bloom_feedback.rs v18.36 — Richer Council bloom visuals and feedback.
// Makes being in an active Council feel alive and powerful.
// Thunder locked in. Yoi ⚡
