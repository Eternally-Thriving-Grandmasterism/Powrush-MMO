// client/monitoring/debug_overlay.rs
// Debug Overlay with real NVML GPU Memory integration (v18.37)

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::render::renderer::RenderDevice;
use crate::monitoring::RBEFlowDashboard;
use crate::monitoring::gpu_timestamps::{GpuTimestampQueries, get_latest_gpu_validation};
use crate::monitoring::nvml_monitor::{NvmlMonitorResource, NvmlGpuInfo};

// ... existing markers ...

pub fn update_debug_overlay(
    rbe_dashboard: Res<RBEFlowDashboard>,
    diagnostics: Res<DiagnosticsStore>,
    world: &World,
    render_device: Option<Res<RenderDevice>>,
    gpu_queries: Option<Res<GpuTimestampQueries>>,
    nvml: Option<Res<NvmlMonitorResource>>,
    // ... other queries for texts ...
) {
    // ... existing RBE Flow + Performance + Graphs updates ...

    // === GPU MEMORY (with real NVML data when available) ===
    let mut used_real_nvml = false;

    if let Some(nvml_res) = nvml {
        let info = nvml_res.0.get_info();

        if info.is_available {
            used_real_nvml = true;

            if let Ok(mut text) = gpu_memory_limit_q.get_single_mut() {
                text.0 = format!("GPU Memory: {} / {} MB", info.memory_used_mb, info.memory_total_mb);
            }

            if let Ok(mut text) = gpu_memory_usage_q.get_single_mut() {
                let usage_percent = if info.memory_total_mb > 0 {
                    (info.memory_used_mb as f32 / info.memory_total_mb as f32) * 100.0
                } else { 0.0 };
                text.0 = format!("GPU Mem Usage: {:.1}% (real)", usage_percent);
            }
        }
    }

    if !used_real_nvml {
        // Fallback to wgpu limits + heuristic
        if let Some(device) = render_device {
            let limits = device.limits();
            if let Ok(mut text) = gpu_memory_limit_q.get_single_mut() {
                let max_gb = limits.max_buffer_size as f64 / 1_000_000_000.0;
                text.0 = format!("Max Buffer: {:.1} GB", max_gb);
            }
            if let Ok(mut text) = gpu_memory_usage_q.get_single_mut() {
                text.0 = "GPU Mem: (NVML not active)".to_string();
            }
        }
    }
}