/*!
 * Audio Latency Monitoring
 *
 * Tracks timing between acoustic estimation and actual audio output changes.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

#[derive(Resource, Default)]
pub struct AudioLatencyMetrics {
    /// Timestamp of last estimation update (in seconds since startup)
    pub last_estimation_time: AtomicU64,

    /// Timestamp when last estimation was applied to Kira effects
    pub last_application_time: AtomicU64,

    /// Timestamp when last IR crossfade started
    pub last_crossfade_start: AtomicU64,

    /// Timestamp when last IR crossfade completed
    pub last_crossfade_complete: AtomicU64,

    /// Number of estimation-to-application latency samples
    pub latency_samples: AtomicU64,

    /// Sum of latency samples (in milliseconds, fixed point)
    pub latency_sum_ms: AtomicU64,

    /// Maximum observed latency (ms)
    pub max_latency_ms: AtomicU64,
}

impl AudioLatencyMetrics {
    pub fn record_estimation(&self, time: f32) {
        self.last_estimation_time.store((time * 1000.0) as u64, Ordering::Relaxed);
    }

    pub fn record_application(&self, time: f32) {
        let est_time = self.last_estimation_time.load(Ordering::Relaxed);
        let app_time = (time * 1000.0) as u64;

        self.last_application_time.store(app_time, Ordering::Relaxed);

        if est_time > 0 {
            let latency_ms = app_time.saturating_sub(est_time);
            self.latency_sum_ms.fetch_add(latency_ms, Ordering::Relaxed);
            self.latency_samples.fetch_add(1, Ordering::Relaxed);

            // Update max
            let current_max = self.max_latency_ms.load(Ordering::Relaxed);
            if latency_ms > current_max {
                self.max_latency_ms.store(latency_ms, Ordering::Relaxed);
            }
        }
    }

    pub fn record_crossfade_start(&self, time: f32) {
        self.last_crossfade_start.store((time * 1000.0) as u64, Ordering::Relaxed);
    }

    pub fn record_crossfade_complete(&self, time: f32) {
        self.last_crossfade_complete.store((time * 1000.0) as u64, Ordering::Relaxed);
    }

    pub fn average_latency_ms(&self) -> f32 {
        let samples = self.latency_samples.load(Ordering::Relaxed);
        if samples == 0 { return 0.0; }
        let sum = self.latency_sum_ms.load(Ordering::Relaxed);
        (sum as f32) / (samples as f32)
    }

    pub fn max_latency_ms(&self) -> u64 {
        self.max_latency_ms.load(Ordering::Relaxed)
    }

    pub fn snapshot(&self) -> AudioLatencySnapshot {
        AudioLatencySnapshot {
            average_latency_ms: self.average_latency_ms(),
            max_latency_ms: self.max_latency_ms(),
            latency_samples: self.latency_samples.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AudioLatencySnapshot {
    pub average_latency_ms: f32,
    pub max_latency_ms: u64,
    pub latency_samples: u64,
}
