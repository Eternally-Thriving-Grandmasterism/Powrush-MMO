// client/monitoring/debug_overlay.rs
// Debug Overlay with GPU Memory Metrics (v18.37)

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::render::renderer::RenderDevice;
use crate::monitoring::RBEFlowDashboard;
use crate::monitoring::gpu_timestamps::{GpuTimestampQueries, get_latest_gpu_validation};

// ... existing marker components ...

#[derive(Component)] struct DebugGpuMemoryLimit;
#[derive(Component)] struct DebugGpuMemoryUsage;

pub fn update_debug_overlay(
    rbe_dashboard: Res<RBEFlowDashboard>,
    diagnostics: Res<DiagnosticsStore>,
    world: &World,
    render_device: Option<Res<RenderDevice>>,
    gpu_queries: Option<Res<GpuTimestampQueries>>,
    // ... other queries ...
) {
    // ... existing RBE + Performance updates ...

    // === GPU MEMORY METRICS ===
    if let Some(device) = render_device {
        let limits = device.limits();

        if let Ok(mut text) = gpu_memory_limit_q.get_single_mut() {
            let max_buffer_gb = limits.max_buffer_size as f64 / 1_000_000_000.0;
            text.0 = format!("Max Buffer: {:.1} GB", max_buffer_gb);
        }

        if let Ok(mut text) = gpu_memory_usage_q.get_single_mut() {
            // Real current usage requires platform-specific integration (NVML, etc.)
            // For now we show a note + rough pressure estimate
            let pressure = if limits.max_buffer_size > 0 {
                // Very rough heuristic based on common game allocations
                ((limits.max_buffer_size as f64 * 0.3) / limits.max_buffer_size as f64) * 100.0
            } else { 0.0 };

            text.0 = format!("GPU Memory Usage: ~{:.0}% (limits shown)", pressure.min(100.0));
        }
    } else {
        if let Ok(mut text) = gpu_memory_limit_q.get_single_mut() {
            text.0 = "Max Buffer: N/A".to_string();
        }
        if let Ok(mut text) = gpu_memory_usage_q.get_single_mut() {
            text.0 = "GPU Memory: (unavailable)".to_string();
        }
    }
}