/*!
 * Procedural Reverb Estimation with Latency Monitoring
 *
 * Now wired to HierarchicalGrid::raycast_distance for spatial occupancy / occlusion-aware reverb.
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
 */

use bevy::prelude::*;
use shared::spatial::HierarchicalGrid;
use crate::settings::audio_mixing::ReverbState;
use crate::settings::biome_acoustic::CurrentBiomeAcoustics;
use crate::audio::ir_manager::CurrentImpulseResponse;
use crate::audio::spatial_metrics::SpatialAudioMetrics;
use crate::audio::latency_metrics::AudioLatencyMetrics;

// ... (config and resource definitions unchanged)

pub fn update_procedural_reverb_estimation(
    time: Res<Time>,
    config: Res<ReverbEstimationConfig>,
    mut estimate: ResMut<ProceduralReverbEstimate>,
    mut reverb_state: ResMut<ReverbState>,
    grid: Option<Res<HierarchicalGrid>>,
    biome: Option<Res<CurrentBiomeAcoustics>>,
    listener: Option<Res<AudioListener>>,
    ir_library: Option<Res<crate::audio::ir_manager::IrLibrary>>,
    mut current_ir: ResMut<CurrentImpulseResponse>,
    spatial_metrics: Res<SpatialAudioMetrics>,
    latency_metrics: Res<AudioLatencyMetrics>,
) {
    let now = time.elapsed_secs();

    // Record estimation timestamp
    latency_metrics.record_estimation(now);

    // ... (existing region + update logic)

    let listener_pos = listener.as_ref().map(|l| l.position).unwrap_or(Vec3::ZERO);
    let region_key = ((listener_pos.x / 32.0).floor() as i32 * 73856093)
        ^ ((listener_pos.y / 32.0).floor() as i32 * 19349663)
        ^ ((listener_pos.z / 32.0).floor() as i32 * 83492791);

    if estimate.last_listener_region != region_key as u64 {
        spatial_metrics.record_listener_region_change();
    }

    if estimate.last_listener_region == region_key as u64
        && now - estimate.last_update < config.update_interval
    {
        return;
    }

    estimate.last_listener_region = region_key as u64;
    estimate.last_update = now;
    spatial_metrics.record_estimation_run();

    // === Wired raycast_distance integration (enriched 2026-06-30) ===
    // Uses the production HierarchicalGrid::raycast_distance (DDA-style, multi-level Z-order)
    // to estimate occlusion / first-hit distance for more accurate procedural reverb.
    // This replaces/augments previous placeholder ray casting logic.
    // Direction can be derived from listener orientation or a default "forward" for room estimation.
    if let Some(grid_res) = grid {
        let grid = grid_res.as_ref();
        // Example: cast forward from listener for primary occlusion distance
        let forward = Vec3 { x: 0.0, y: 0.0, z: 1.0 }; // TODO: use actual listener orientation
        if let Some(first_hit_dist) = grid.raycast_distance(listener_pos, forward, config.max_ray_distance) {
            // Use first_hit_dist to modulate reverb decay / early reflections
            // Example influence (non-destructive):
            estimate.occlusion_factor = (first_hit_dist / config.max_ray_distance).clamp(0.0, 1.0);
            spatial_metrics.record_raycast_hit();
        } else {
            estimate.occlusion_factor = 0.0;
        }
    }

    // ... (remaining estimation + IR selection logic)

    // After computing new values, record application will happen in Kira systems
    // For now we can record a basic application here as approximation
    latency_metrics.record_application(now);

    // Apply to estimate and reverb state (existing code)
    // ...
}
