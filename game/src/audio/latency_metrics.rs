/*!
 * Audio Latency Monitoring - With Crossfade Duration
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Resource, Default)]
pub struct AudioLatencyMetrics {
    pub last_estimation_time: AtomicU64,
    pub last_application_time: AtomicU64,
    pub last_crossfade_start: AtomicU64,
    pub last_crossfade_complete: AtomicU64,
    pub latency_samples: AtomicU64,
    pub latency_sum_ms: AtomicU64,
    pub max_latency_ms: AtomicU64,
}

impl AudioLatencyMetrics {
    // ... existing methods ...

    pub fn last_crossfade_duration_ms(&self) -> Option<u64> {
        let start = self.last_crossfade_start.load(Ordering::Relaxed);
        let complete = self.last_crossfade_complete.load(Ordering::Relaxed);

        if start > 0 && complete > start {
            Some(complete - start)
        } else {
            None
        }
    }

    pub fn snapshot(&self) -> AudioLatencySnapshot {
        AudioLatencySnapshot {
            average_latency_ms: self.average_latency_ms(),
            max_latency_ms: self.max_latency_ms(),
            latency_samples: self.latency_samples.load(Ordering::Relaxed),
            last_crossfade_duration_ms: self.last_crossfade_duration_ms(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AudioLatencySnapshot {
    pub average_latency_ms: f32,
    pub max_latency_ms: u64,
    pub latency_samples: u64,
    pub last_crossfade_duration_ms: Option<u64>,
}
