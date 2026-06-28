/*!
 * Procedural Reverb Estimation with Latency Monitoring
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
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

    // ... (ray casting + estimation logic)

    // After computing new values, record application will happen in Kira systems
    // For now we can record a basic application here as approximation
    latency_metrics.record_application(now);

    // Apply to estimate and reverb state (existing code)
    // ...
}
