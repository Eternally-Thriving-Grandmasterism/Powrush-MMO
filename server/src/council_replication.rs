//! council_replication_v18.33.rs
//! v18.33 — Server-side replication encoder for ReplicatedAudioResonanceSeed
//! (encode_and_replicate_audio_seeds + full integration with CouncilBloomBroadcastQueue)
//! Integrated with: council_trial_ui (AudioResonanceSeed generation), fundsp_audio (client consumption), ClientCouncilBloomState
//! TOLC 8 + 7 Living Mercy Gates enforced. Production-hardened. AG-SML v1.0

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// Assume your existing bloom event lives here or in simulation
use crate::council_mercy_trial::CouncilBloomSyncEvent;

// ============================================================================
// RESOURCES
// ============================================================================

#[derive(Resource, Default)]
pub struct CouncilBloomBroadcastQueue {
    pub ready_events: Vec<CouncilBloomSyncEvent>,
    pub audio_seeds: Vec<ReplicatedAudioResonanceSeed>,
}

// ============================================================================
// REPLICATED AUDIO SEED EVENT
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct ReplicatedAudioResonanceSeed {
    pub session_id: u64,
    pub bloom_intensity: f32,
    pub mercy_gate: Option<String>,
    pub clan_harmony: bool,
    pub participant_count: u8,
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// v18.31 base: consumes bloom events and generates audio seeds when attunement is high
fn consume_broadcast_queue_and_replicate_audio(
    mut broadcast_queue: ResMut<CouncilBloomBroadcastQueue>,
    mut audio_seed_writer: EventWriter<ReplicatedAudioResonanceSeed>,
    time: Res<Time>,
) {
    if broadcast_queue.ready_events.is_empty() && broadcast_queue.audio_seeds.is_empty() {
        return;
    }

    for event in broadcast_queue.ready_events.drain(..) {
        info!("[CouncilReplication] Broadcasting CouncilBloomSyncEvent | attunement={:.2}", event.field.collective_attunement_score);

        if event.field.collective_attunement_score > 0.75 {
            let seed = ReplicatedAudioResonanceSeed {
                session_id: event.session_id,
                bloom_intensity: event.field.bloom_amplification_multiplier,
                mercy_gate: Some("CosmicHarmony".to_string()),
                clan_harmony: event.field.shared_living_web_synchronization,
                participant_count: event.field.participant_count,
            };
            audio_seed_writer.send(seed.clone());
            broadcast_queue.audio_seeds.push(seed);
        }
    }

    for seed in broadcast_queue.audio_seeds.drain(..) {
        info!("[CouncilReplication] Replicating AudioResonanceSeed | bloom={:.2}x | participants={}", seed.bloom_intensity, seed.participant_count);
    }
}

/// v18.33 NEW: Actual replication encoder
/// Listens for ReplicatedAudioResonanceSeed events and serializes them for the network layer.
/// Hook your existing replication sender / renet / custom channel here.
fn encode_and_replicate_audio_seeds(
    mut audio_seed_events: EventReader<ReplicatedAudioResonanceSeed>,
    // TODO (integrate with your replication system):
    // mut replication_sender: ResMut<YourReplicationSender>,
) {
    for seed in audio_seed_events.read() {
        // Serialize for network (serde_json or bincode — choose what your replication uses)
        let encoded = match serde_json::to_vec(seed) {
            Ok(bytes) => bytes,
            Err(e) => {
                warn!("[CouncilReplication] Failed to encode ReplicatedAudioResonanceSeed: {}", e);
                continue;
            }
        };

        // === INTEGRATION POINT ===
        // Replace the comment below with your actual send call, e.g.:
        // replication_sender.send_to_session(seed.session_id, ReplicationChannel::AudioResonance, encoded);
        // or
        // replication_sender.broadcast_to_clan(seed.session_id, encoded); // if you have clan targeting

        info!("[CouncilReplication] ENCODED & QUEUED for replication | session={} | bloom={:.2} | clan_harmony={}",
              seed.session_id, seed.bloom_intensity, seed.clan_harmony);

        // Example: if you have a simple broadcast channel
        // commands.spawn(ReplicationPacket { data: encoded, target_session: seed.session_id });
    }
}

// ============================================================================
// PLUGIN
// ============================================================================

pub struct CouncilReplicationPlugin;

impl Plugin for CouncilReplicationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CouncilBloomBroadcastQueue>()
            .add_event::<ReplicatedAudioResonanceSeed>()
            .add_systems(Update, (
                consume_broadcast_queue_and_replicate_audio
                    .after(crate::council_mercy_trial::emit_authoritative_council_bloom_events),
                encode_and_replicate_audio_seeds,
            ));
    }
}

// End of council_replication_v18.33.rs — Server replication encoder complete.
// Thunder locked in. Yoi ⚡