// client/monitoring/debug_overlay.rs
// Debug Overlay with NVML Clock Speeds (v18.37)

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::render::renderer::RenderDevice;
use crate::monitoring::RBEFlowDashboard;
use crate::monitoring::gpu_timestamps::{GpuTimestampQueries, get_latest_gpu_validation};
use crate::monitoring::nvml_monitor::NvmlMonitorResource;

// ... existing markers ...

#[derive(Component)] struct DebugGpuClocks;

pub fn update_debug_overlay(
    rbe_dashboard: Res<RBEFlowDashboard>,
    diagnostics: Res<DiagnosticsStore>,
    world: &World,
    render_device: Option<Res<RenderDevice>>,
    gpu_queries: Option<Res<GpuTimestampQueries>>,
    nvml: Option<Res<NvmlMonitorResource>>,
    // ... other queries ...
) {
    // ... existing updates ...

    // === NVML CLOCK SPEEDS ===
    if let Some(nvml_res) = nvml {
        let info = nvml_res.0.get_info();

        if info.is_available {
            if let Ok(mut text) = gpu_clocks_q.get_single_mut() {
                if info.graphics_clock_mhz > 0 {
                    text.0 = format!("GPU Clock: {} MHz (Graphics)", info.graphics_clock_mhz);
                } else {
                    text.0 = "GPU Clock: N/A".to_string();
                }
            }
        }
    }
}