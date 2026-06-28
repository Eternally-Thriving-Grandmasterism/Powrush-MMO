/*!
 * IR Truncation Metrics Resource
 *
 * Thread-safe counters for monitoring IR truncation performance.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Resource, Default)]
pub struct IrTruncationMetrics {
    /// Number of times truncation succeeded inside the IrAssetLoader
    pub truncations_in_loader: AtomicU64,
    /// Number of times truncation was deferred to post-processor (async fallback)
    pub async_fallbacks: AtomicU64,
    /// Number of times truncation succeeded in the post-processor
    pub truncations_in_post_processor: AtomicU64,
    /// Number of times truncation was skipped (IR already short enough)
    pub truncations_skipped: AtomicU64,
    /// Total truncation attempts
    pub total_truncation_attempts: AtomicU64,
}

impl IrTruncationMetrics {
    pub fn record_loader_success(&self) {
        self.truncations_in_loader.fetch_add(1, Ordering::Relaxed);
        self.total_truncation_attempts.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_async_fallback(&self) {
        self.async_fallbacks.fetch_add(1, Ordering::Relaxed);
        self.total_truncation_attempts.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_post_processor_success(&self) {
        self.truncations_in_post_processor.fetch_add(1, Ordering::Relaxed);
        self.total_truncation_attempts.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_skipped(&self) {
        self.truncations_skipped.fetch_add(1, Ordering::Relaxed);
    }

    /// Returns a snapshot of current metrics (for debugging/UI)
    pub fn snapshot(&self) -> IrTruncationSnapshot {
        IrTruncationSnapshot {
            truncations_in_loader: self.truncations_in_loader.load(Ordering::Relaxed),
            async_fallbacks: self.async_fallbacks.load(Ordering::Relaxed),
            truncations_in_post_processor: self.truncations_in_post_processor.load(Ordering::Relaxed),
            truncations_skipped: self.truncations_skipped.load(Ordering::Relaxed),
            total_truncation_attempts: self.total_truncation_attempts.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IrTruncationSnapshot {
    pub truncations_in_loader: u64,
    pub async_fallbacks: u64,
    pub truncations_in_post_processor: u64,
    pub truncations_skipped: u64,
    pub total_truncation_attempts: u64,
}
