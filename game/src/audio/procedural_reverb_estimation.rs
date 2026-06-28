/*!
 * Procedural Reverb Ray Tracing Estimation (Production Enhanced)
 *
 * Full integration with HierarchicalGrid + BiomeAcousticProfile + listener caching.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use shared::spatial::HierarchicalGrid;
use crate::settings::audio_mixing::ReverbState;
use crate::settings::biome_acoustic::CurrentBiomeAcoustics;

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

/// Listener position resource (can be updated by player/camera systems)
#[derive(Resource, Default, Clone)]
pub struct AudioListener {
    pub position: Vec3,
}

/// Main estimation system with full feature set.
pub fn update_procedural_reverb_estimation(
    time: Res<Time>,
    config: Res<ReverbEstimationConfig>,
    mut estimate: ResMut<ProceduralReverbEstimate>,
    mut reverb_state: ResMut<ReverbState>,
    grid: Option<Res<HierarchicalGrid>>,
    biome: Option<Res<CurrentBiomeAcoustics>>,
    listener: Option<Res<AudioListener>>,
) {
    let now = time.elapsed_secs();

    // Simple region-based caching using listener position
    let listener_pos = listener.as_ref().map(|l| l.position).unwrap_or(Vec3::ZERO);
    let region_key = ((listener_pos.x / 32.0).floor() as i32 * 73856093)
        ^ ((listener_pos.y / 32.0).floor() as i32 * 19349663)
        ^ ((listener_pos.z / 32.0).floor() as i32 * 83492791);

    // Skip update if we are in the same region and within interval
    if estimate.last_listener_region == region_key as u64
        && now - estimate.last_update < config.update_interval
    {
        return;
    }
    estimate.last_listener_region = region_key as u64;
    estimate.last_update = now;

    let mut total_distance = 0.0;
    let mut hit_count = 0u32;
    let mut early_sum = 0.0;

    // === Real HierarchicalGrid ray queries when available ===
    if let Some(grid) = grid.as_ref() {
        for i in 0..config.ray_count {
            let t = i as f32 / config.ray_count as f32;
            let yaw = t * std::f32::consts::TAU;
            let pitch = if (t * 6.0).fract() < config.vertical_bias {
                if i % 2 == 0 { -0.62 } else { 0.52 }
            } else {
                (t - 0.5) * 0.82
            };

            let dir = Vec3::new(
                yaw.cos() * pitch.cos(),
                pitch.sin(),
                yaw.sin() * pitch.cos(),
            );

            if let Some(dist) = grid.raycast_distance(
                shared::spatial::Vec3 {
                    x: listener_pos.x,
                    y: listener_pos.y,
                    z: listener_pos.z,
                },
                shared::spatial::Vec3 { x: dir.x, y: dir.y, z: dir.z },
                config.max_distance,
            ) {
                total_distance += dist;
                hit_count += 1;
                if config.enable_early_reflections && dist < 32.0 {
                    early_sum += dist;
                }
            }
        }
    } else {
        // High-quality heuristic fallback
        for i in 0..config.ray_count {
            let t = i as f32 / config.ray_count as f32;
            let yaw = t * std::f32::consts::TAU;
            let pitch = if (t * 5.0).fract() < config.vertical_bias {
                if i % 2 == 0 { -0.68 } else { 0.58 }
            } else {
                (t - 0.5) * 0.88
            };
            let sim = config.max_distance * (0.30 + (yaw.sin() * 0.27 + pitch.cos() * 0.30).abs());
            total_distance += sim.min(config.max_distance);
            hit_count += 1;
        }
    }

    let avg_dist = if hit_count > 0 { total_distance / hit_count as f32 } else { 42.0 };
    let room_size = (avg_dist / config.max_distance).powf(0.50).clamp(0.05, 0.95);

    // === Material absorption from BiomeAcousticProfile when available ===
    let (mut low_abs, mut high_abs) = if let Some(biome) = biome.as_ref() {
        let p = &biome.active_profile;
        (
            p.base_absorption_low.clamp(0.02, 0.85),
            p.base_absorption_high.clamp(0.08, 0.92),
        )
    } else {
        let base = 0.12 + (1.0 - room_size) * 0.50;
        ((base * 0.78).clamp(0.03, 0.78), (base * 1.30).clamp(0.10, 0.94))
    };

    // Blend with geometry-based modulation
    let geo_mod = (1.0 - room_size) * 0.25;
    low_abs = (low_abs + geo_mod).clamp(0.03, 0.88);
    high_abs = (high_abs + geo_mod * 1.1).clamp(0.10, 0.95);

    let wetness = (room_size * 0.65 + 0.28).clamp(0.16, 0.90);

    let early_delay = if config.enable_early_reflections && early_sum > 0.0 {
        (early_sum / hit_count as f32 * 2.6).clamp(6.0, 78.0)
    } else {
        22.0
    };

    // Smooth + apply
    let s = config.smoothing;
    estimate.room_size = estimate.room_size * s + room_size * (1.0 - s);
    estimate.low_absorption = estimate.low_absorption * s + low_abs * (1.0 - s);
    estimate.high_absorption = estimate.high_absorption * s + high_abs * (1.0 - s);
    estimate.wetness = estimate.wetness * s + wetness * (1.0 - s);
    estimate.early_reflection_delay_ms = estimate.early_reflection_delay_ms * s + early_delay * (1.0 - s);

    reverb_state.low_damping = estimate.low_absorption;
    reverb_state.high_damping = estimate.high_absorption;
}

pub fn reset_procedural_reverb_estimate(mut estimate: ResMut<ProceduralReverbEstimate>) {
    *estimate = ProceduralReverbEstimate::default();
}
