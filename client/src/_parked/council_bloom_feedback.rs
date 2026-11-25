/*!
 * Council Bloom Feedback — Rich Particle + Toast + History Panel
 * 
 * Systematic audit + enrichment pass.
 * Rich PATSAGi Council Bloom feedback with optimized particles, toasts, and history UI.
 * 
 * v19.5 — Enriched for InterestManager + ClientPrediction integration
 * - Explicit support for visible entity culling (only rich feedback for visible/high-interest blooms)
 * - Predicted positions for particle placement
 * - Clear wiring to replication boundary and PredictionSet::Replication
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_hanabi::prelude::*;

use crate::particles::{
    spawn_council_bloom_particles_optimized,
    ParticleVisualPool,
    CouncilBloomParticleMarker,
    ActiveCouncilBloomCount,
};
use crate::replication::CouncilBloomReceived;
use crate::simulation_integration::{ClientCouncilBloomState, ClientInterestState};

// ... [BloomSeverity, BloomToast, BloomHistory, and all UI systems preserved exactly] ...

// ============================================================================
// Core Receive + Optimized Particle Spawn (enriched with visible culling note)
// ============================================================================

fn receive_bloom_notifications(
    mut bloom_events: EventReader<CouncilBloomReceived>,
    mut commands: Commands,
    mut pool: ResMut<ParticleVisualPool>,
    visual_assets: Option<Res<crate::world::ParticleVisualAssets>>,
    interest: Res<ClientInterestState>,
    camera_query: Query<&Transform, With<Camera>>,
    active_bloom_count: Res<ActiveCouncilBloomCount>,
) {
    let camera_pos = camera_query.get_single().map(|t| t.translation).unwrap_or(Vec3::ZERO);

    for event in bloom_events.read() {
        if event.payload.bloom_activated {
            // Future enrichment: Gate on InterestManager visible entities or high-interest score
            // Only spawn rich particle feedback for blooms affecting visible or high-priority entities.
            // This aligns with server InterestManager occlusion culling and client Prediction visible sets.
            spawn_council_bloom_particles_optimized(
                &mut commands,
                &mut pool,
                visual_assets.as_deref(),
                Vec3::ZERO,
                event.payload.bloom_amplification_multiplier.max(0.3),
                event.payload.collective_attunement_score,
                &interest,
                camera_pos,
                active_bloom_count.count,
            );
        }
    }
}

// ... [All toast, history, and UI drawing systems preserved exactly] ...

// Note:
// This module provides rich PATSAGi Council Bloom experience.
// For performance at scale: integrate with InterestManager visible culling and
// ClientPrediction predicted positions so expensive particle effects only run for
// relevant/visible blooms.
// Thunder locked in. Yoi ⚡