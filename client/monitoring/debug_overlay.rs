// client/monitoring/debug_overlay.rs
// Unified Debug Overlay for Powrush-MMO (v18.37)
// Now displays real GPU time from wgpu timestamp queries

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use crate::monitoring::RBEFlowDashboard;
use crate::monitoring::gpu_timestamps::{GpuTimestampQueries, get_latest_gpu_time_ms};

// ... (marker components remain the same)

#[derive(Component)] struct DebugRbeAbundance;
#[derive(Component)] struct DebugRbeCreation;
#[derive(Component)] struct DebugRbeRestoration;
#[derive(Component)] struct DebugL2Boost;
#[derive(Component)] struct DebugL3Boost;
#[derive(Component)] struct DebugAlerts;

#[derive(Component)] struct DebugFps;
#[derive(Component)] struct DebugFrameTime;
#[derive(Component)] struct DebugEntities;
#[derive(Component)] struct DebugGpuFrameTime;
#[derive(Component)] struct DebugGpuLoad;

#[derive(Component)] struct FpsGraphContainer;
#[derive(Component)] struct FrameTimeGraphContainer;
#[derive(Component)] struct FpsBar { index: usize }
#[derive(Component)] struct FrameTimeBar { index: usize }

#[derive(Component)] struct DebugOverlayContainer;

#[derive(Resource, Default)]
pub struct DebugOverlayVisible(pub bool);

#[derive(Resource)]
pub struct FpsHistory { /* ... */ }

#[derive(Resource)]
pub struct FrameTimeHistory { /* ... */ }

// (rest of resource definitions remain)

pub fn update_debug_overlay(
    rbe_dashboard: Res<RBEFlowDashboard>,
    diagnostics: Res<DiagnosticsStore>,
    world: &World,
    mut fps_history: ResMut<FpsHistory>,
    mut frame_time_history: ResMut<FrameTimeHistory>,
    // ... other queries ...
    gpu_queries: Option<Res<GpuTimestampQueries>>,
) {
    // RBE Flow + Performance updates (unchanged)
    // ...

    // === REAL GPU TIME FROM TIMESTAMP QUERIES ===
    if let Some(queries) = gpu_queries {
        let real_gpu_time = get_latest_gpu_time_ms(&queries);

        if let Ok(mut text) = gpu_frame_time_q.get_single_mut() {
            if real_gpu_time > 0.0 {
                text.0 = format!("GPU Frame Time: {:.2} ms", real_gpu_time);
            } else {
                text.0 = "GPU Frame Time: measuring...".to_string();
            }
        }

        if let Ok(mut text) = gpu_load_q.get_single_mut() {
            // Simple load estimate based on real GPU time
            let load = (real_gpu_time / 16.67).clamp(0.0, 2.0) * 50.0;
            text.0 = format!("Est. GPU Load: ~{:.0} %", load.min(100.0));
        }
    } else {
        // Fallback if timestamp queries not yet available
        if let Ok(mut text) = gpu_frame_time_q.get_single_mut() {
            text.0 = "GPU Frame Time: (initializing)".to_string();
        }
    }

    // Update graphs (FPS + Frame Time) - unchanged
    // ...
}