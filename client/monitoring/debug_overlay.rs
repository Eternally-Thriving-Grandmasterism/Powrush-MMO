// client/monitoring/debug_overlay.rs
// Unified Debug Overlay + Automatic Spike Logging (v18.37)

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use crate::monitoring::RBEFlowDashboard;
use crate::monitoring::gpu_timestamps::{GpuTimestampQueries, get_latest_gpu_validation};
use std::time::{SystemTime, UNIX_EPOCH};

// ... (marker components and resources remain the same)

#[derive(Resource)]
pub struct PerformanceSpikeConfig {
    pub frame_time_threshold_ms: f32,
    pub fps_threshold: f32,
    pub gpu_time_threshold_ms: f32,
    pub min_time_between_logs_ms: u128,
}

impl Default for PerformanceSpikeConfig {
    fn default() -> Self {
        Self {
            frame_time_threshold_ms: 25.0,
            fps_threshold: 40.0,
            gpu_time_threshold_ms: 20.0,
            min_time_between_logs_ms: 2000, // don't spam more than once every 2 seconds
        }
    }
}

#[derive(Resource, Default)]
pub struct PerformanceSpikeState {
    pub last_spike_log_ms: u128,
}

pub fn update_debug_overlay(
    rbe_dashboard: Res<RBEFlowDashboard>,
    diagnostics: Res<DiagnosticsStore>,
    world: &World,
    mut fps_history: ResMut<FpsHistory>,
    mut frame_time_history: ResMut<FrameTimeHistory>,
    mut spike_config: Res<PerformanceSpikeConfig>,
    mut spike_state: ResMut<PerformanceSpikeState>,
    gpu_queries: Option<Res<GpuTimestampQueries>>,
    // ... queries for texts and bars ...
) {
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    // === EXISTING METRIC UPDATES ===
    // (RBE Flow + Performance text + graphs remain here)

    let current_fps = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.average())
        .unwrap_or(0.0);

    let current_frame_time = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|d| d.average())
        .unwrap_or(0.0);

    let gpu_validation = gpu_queries
        .map(|q| get_latest_gpu_validation(&q))
        .unwrap_or_default();

    // === AUTOMATIC SPIKE LOGGING ===
    let is_spike = current_frame_time > spike_config.frame_time_threshold_ms
        || current_fps < spike_config.fps_threshold
        || (gpu_validation.is_valid && gpu_validation.last_gpu_time_ms > spike_config.gpu_time_threshold_ms);

    if is_spike {
        let time_since_last = now_ms.saturating_sub(spike_state.last_spike_log_ms);

        if time_since_last > spike_config.min_time_between_logs_ms {
            tracing::warn!(
                "[Performance Spike] FPS: {:.1} | Frame Time: {:.2} ms | GPU Time: {:.2} ms | Abundance: {:.0}",
                current_fps,
                current_frame_time,
                gpu_validation.last_gpu_time_ms,
                rbe_dashboard.server_abundance
            );

            spike_state.last_spike_log_ms = now_ms;
        }
    }

    // === UPDATE GRAPHS AND TEXTS ===
    // (existing graph and text update code remains)
}