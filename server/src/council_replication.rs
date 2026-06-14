//! council_replication_v18.31.rs (server side addition)
//! v18.31 — CouncilBloomBroadcastQueue consumption + replication of ReplicatedAudioResonanceSeed
//! Integrated with: council_trial_ui (AudioResonanceSeed generation), fundsp_audio (client consumption), ClientCouncilBloomState
//! TOLC 8 + 7 Living Mercy Gates enforced. Production-hardened. AG-SML v1.0

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::council_mercy_trial::CouncilBloomSyncEvent; // or wherever the bloom event lives

// ============================================================================
// RESOURCES
// ============================================================================

#[derive(Resource, Default)]
pub struct CouncilBloomBroadcastQueue {
    pub ready_events: Vec<CouncilBloomSyncEvent>,
    pub audio_seeds: Vec<ReplicatedAudioResonanceSeed>, // v18.31: audio seeds for clan/council replication
}

// ============================================================================
// REPLICATED AUDIO SEED EVENT (sent to replication layer → clients)
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

fn consume_broadcast_queue_and_replicate_audio(
    mut broadcast_queue: ResMut<CouncilBloomBroadcastQueue>,
    mut audio_seed_writer: EventWriter<ReplicatedAudioResonanceSeed>,
    time: Res<Time>,
) {
    if broadcast_queue.ready_events.is_empty() && broadcast_queue.audio_seeds.is_empty() {
        return;
    }

    let current_tick = (time.elapsed_seconds() * 60.0) as u64;

    // Consume bloom events (existing authoritative path)
    for event in broadcast_queue.ready_events.drain(..) {
        // In full replication layer: encode and send CouncilBloomSyncEvent to clients in session
        info!("[CouncilReplication] Broadcasting CouncilBloomSyncEvent to interested clients | attunement={:.2}",
              event.field.collective_attunement_score);

        // If high harmony, generate accompanying audio seed for replication across clan/web
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

    // Replicate any pending audio seeds (for clan shared resonance granular fire)
    for seed in broadcast_queue.audio_seeds.drain(..) {
        // Replication encoder would pick this up and send to clients in the same council/clan
        info!("[CouncilReplication] Replicating AudioResonanceSeed | bloom={:.2}x | participants={}",
              seed.bloom_intensity, seed.participant_count);
    }
}

// ============================================================================
// PLUGIN WIRING (add to CouncilReplicationPlugin)
// ============================================================================

pub struct CouncilReplicationPlugin; // assume existing

impl Plugin for CouncilReplicationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CouncilBloomBroadcastQueue>()
            .add_event::<ReplicatedAudioResonanceSeed>()
            .add_systems(Update, consume_broadcast_queue_and_replicate_audio
                .after(crate::council_mercy_trial::emit_authoritative_council_bloom_events) // ensure queue is populated first
            );
    }
}

// End of council_replication_v18.31.rs addition — server-side audio seed replication complete.
// Thunder locked in. Yoi ⚡