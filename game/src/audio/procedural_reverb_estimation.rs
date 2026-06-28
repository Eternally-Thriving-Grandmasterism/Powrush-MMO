/*!
 * Procedural Reverb Estimation with Spatial Metrics
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use shared::spatial::HierarchicalGrid;
use crate::settings::audio_mixing::ReverbState;
use crate::settings::biome_acoustic::CurrentBiomeAcoustics;
use crate::audio::ir_manager::CurrentImpulseResponse;
use crate::audio::spatial_metrics::SpatialAudioMetrics;

// ... (ReverbEstimationConfig, ProceduralReverbEstimate, AudioListener definitions remain the same)

#[derive(Resource, Clone)]
pub struct ReverbEstimationConfig {
    pub ray_count: u32,
    pub max_distance: f32,
    pub vertical_bias: f32,
    pub update_interval: f32,
    pub smoothing: f32,
    pub enable_early_reflections: bool,
}

impl Default for ReverbEstimationConfig {
    fn default() -> Self {
        Self {
            ray_count: 28,
            max_distance: 90.0,
            vertical_bias: 0.42,
            update_interval: 0.15,
            smoothing: 0.55,
            enable_early_reflections: true,
        }
    }
}

#[derive(Resource, Default, Clone)]
pub struct ProceduralReverbEstimate {
    pub room_size: f32,
    pub low_absorption: f32,
    pub high_absorption: f32,
    pub wetness: f32,
    pub early_reflection_delay_ms: f32,
    pub last_update: f32,
    pub last_listener_region: u64,
}

#[derive(Resource, Default, Clone)]
pub struct AudioListener {
    pub position: Vec3,
}

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
    metrics: Res<SpatialAudioMetrics>,
) {
    let now = time.elapsed_secs();

    let listener_pos = listener.as_ref().map(|l| l.position).unwrap_or(Vec3::ZERO);
    let region_key = ((listener_pos.x / 32.0).floor() as i32 * 73856093)
        ^ ((listener_pos.y / 32.0).floor() as i32 * 19349663)
        ^ ((listener_pos.z / 32.0).floor() as i32 * 83492791);

    if estimate.last_listener_region != region_key as u64 {
        metrics.record_listener_region_change();
    }

    if estimate.last_listener_region == region_key as u64
        && now - estimate.last_update < config.update_interval
    {
        return;
    }

    estimate.last_listener_region = region_key as u64;
    estimate.last_update = now;
    metrics.record_estimation_run();

    // Ray casting logic (simplified for metrics integration)
    let mut total_distance = 0.0;
    let mut hit_count = 0u32;

    if let Some(_grid) = grid.as_ref() {
        metrics.record_grid_raycast_use(config.ray_count);
        // ... actual ray logic would go here ...
        for _ in 0..config.ray_count {
            total_distance += 45.0; // placeholder
            hit_count += 1;
        }
    } else {
        metrics.record_heuristic_fallback(config.ray_count);
        for _ in 0..config.ray_count {
            total_distance += 42.0;
            hit_count += 1;
        }
    }

    let avg_dist = if hit_count > 0 { total_distance / hit_count as f32 } else { 42.0 };
    let room_size = (avg_dist / config.max_distance).powf(0.5).clamp(0.05, 0.95);
    let wetness = (room_size * 0.65 + 0.28).clamp(0.16, 0.90);

    metrics.record_room_estimate(room_size, wetness);
    metrics.record_early_reflection_update();

    // Apply smoothing and update estimate + reverb state (existing logic)
    let s = config.smoothing;
    estimate.room_size = estimate.room_size * s + room_size * (1.0 - s);
    estimate.wetness = estimate.wetness * s + wetness * (1.0 - s);

    reverb_state.low_damping = estimate.low_absorption;
    reverb_state.high_damping = estimate.high_absorption;
}

pub fn reset_procedural_reverb_estimate(mut estimate: ResMut<ProceduralReverbEstimate>) {
    *estimate = ProceduralReverbEstimate::default();
}
