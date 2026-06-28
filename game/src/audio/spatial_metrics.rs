/*!
 * Spatial Audio Metrics
 *
 * Thread-safe counters for monitoring the spatial/procedural audio system.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Resource, Default)]
pub struct SpatialAudioMetrics {
    /// Total number of times the reverb estimation system has run
    pub estimation_runs: AtomicU64,

    /// Number of times real HierarchicalGrid raycasts were used
    pub grid_raycast_uses: AtomicU64,

    /// Number of times the high-quality heuristic fallback was used
    pub heuristic_fallbacks: AtomicU64,

    /// Number of listener region changes (cache effectiveness)
    pub listener_region_changes: AtomicU64,

    /// Total number of ray samples taken across all updates
    pub total_ray_samples: AtomicU64,

    /// Sum of all estimated room sizes (for average calculation)
    pub room_size_sum: AtomicU64,

    /// Sum of all estimated wetness values
    pub wetness_sum: AtomicU64,

    /// Number of times early reflection delay was updated
    pub early_reflection_updates: AtomicU64,
}

impl SpatialAudioMetrics {
    pub fn record_estimation_run(&self) {
        self.estimation_runs.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_grid_raycast_use(&self, ray_count: u32) {
        self.grid_raycast_uses.fetch_add(1, Ordering::Relaxed);
        self.total_ray_samples.fetch_add(ray_count as u64, Ordering::Relaxed);
    }

    pub fn record_heuristic_fallback(&self, ray_count: u32) {
        self.heuristic_fallbacks.fetch_add(1, Ordering::Relaxed);
        self.total_ray_samples.fetch_add(ray_count as u64, Ordering::Relaxed);
    }

    pub fn record_listener_region_change(&self) {
        self.listener_region_changes.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_room_estimate(&self, room_size: f32, wetness: f32) {
        // Store as fixed-point to avoid floats in atomics
        let room_fixed = (room_size * 1000.0) as u64;
        let wet_fixed = (wetness * 1000.0) as u64;
        self.room_size_sum.fetch_add(room_fixed, Ordering::Relaxed);
        self.wetness_sum.fetch_add(wet_fixed, Ordering::Relaxed);
    }

    pub fn record_early_reflection_update(&self) {
        self.early_reflection_updates.fetch_add(1, Ordering::Relaxed);
    }

    /// Returns average room size over all recorded estimates
    pub fn average_room_size(&self) -> f32 {
        let count = self.estimation_runs.load(Ordering::Relaxed);
        if count == 0 { return 0.0; }
        let sum = self.room_size_sum.load(Ordering::Relaxed);
        (sum as f32 / 1000.0) / count as f32
    }

    /// Returns average wetness over all recorded estimates
    pub fn average_wetness(&self) -> f32 {
        let count = self.estimation_runs.load(Ordering::Relaxed);
        if count == 0 { return 0.0; }
        let sum = self.wetness_sum.load(Ordering::Relaxed);
        (sum as f32 / 1000.0) / count as f32
    }

    pub fn snapshot(&self) -> SpatialAudioSnapshot {
        SpatialAudioSnapshot {
            estimation_runs: self.estimation_runs.load(Ordering::Relaxed),
            grid_raycast_uses: self.grid_raycast_uses.load(Ordering::Relaxed),
            heuristic_fallbacks: self.heuristic_fallbacks.load(Ordering::Relaxed),
            listener_region_changes: self.listener_region_changes.load(Ordering::Relaxed),
            total_ray_samples: self.total_ray_samples.load(Ordering::Relaxed),
            average_room_size: self.average_room_size(),
            average_wetness: self.average_wetness(),
            early_reflection_updates: self.early_reflection_updates.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SpatialAudioSnapshot {
    pub estimation_runs: u64,
    pub grid_raycast_uses: u64,
    pub heuristic_fallbacks: u64,
    pub listener_region_changes: u64,
    pub total_ray_samples: u64,
    pub average_room_size: f32,
    pub average_wetness: f32,
    pub early_reflection_updates: u64,
}
