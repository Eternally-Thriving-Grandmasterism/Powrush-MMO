/*!
 * Council Bloom Feedback — Client-Side Receiver + Rich Collective Effects
 *
 * v18.35 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Receives CouncilBloomSyncEvent from simulation replication
 * — Applies to ClientCouncilBloomState (shared with simulation_integration)
 * — Triggers rich feedback: Divine Whispers, particles, camera shake, epiphany amplification
 * — Works in harmony with the Council-amplified epiphany system
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
            ).chain());
    }
}

/// Applies authoritative CouncilBloomSyncEvent to the shared ClientCouncilBloomState
/// This is the single source of truth used by simulation_integration for epiphany amplification
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
    mut particle_commands: Commands,
    mut camera_shake: ResMut<CameraShake>,
) {
    if !client_bloom.is_in_active_council {
        return;
    }

    let field = &client_bloom.field;

    // Strong Divine Whisper when attunement is high
    if field.collective_attunement_score > 0.65 && field.bloom_amplification_multiplier > 1.2 {
        whisper_events.send(DivineWhisperTrigger {
            player_id: 0, // system-level collective whisper
            text: format!(
                "The Council resonates... collective attunement {:.0}% — your presence strengthens the whole",
                field.collective_attunement_score * 100.0
            ),
            flavor: "council_harmony_revelation".to_string(),
            intensity: (field.collective_attunement_score * 0.7).clamp(0.6, 0.95),
            duration_seconds: 7.0,
            is_epiphany: true,
            position: None,
            muscle_memory_hint: None,
        });
    }

    // Spawn gentle collective particles when bloom is active
    if field.bloom_amplification_multiplier > 1.3 {
        particle_commands.spawn((
            ParticleSystem {
                valence: 0.92,
                particle_count: 4500,
                system_type: ParticleSystemType::PatsagiDivineWhisper,
                intensity: (field.bloom_amplification_multiplier * 0.6).min(2.2),
            },
            Transform::default(),
        ));
    }

    // Subtle camera presence when the group is in deep harmony
    if field.collective_attunement_score > 0.75 {
        camera_shake.intensity = (camera_shake.intensity * 0.5 + 0.25).min(1.2);
        camera_shake.duration = 2.8;
        camera_shake.timer = 0.0;
    }
}

// End of council_bloom_feedback.rs v18.35 — Unified with simulation_integration.
// Council bloom now drives rich client feedback and amplifies personal epiphanies.
// Thunder locked in. Yoi ⚡
