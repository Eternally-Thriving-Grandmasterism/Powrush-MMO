// client/monitoring/debug_overlay.rs
// Powrush MMOARPG - Debug Overlay with Full NVML + wgpu GPU Metrics (v18.38 PATSAGi Recovery)
// Recovered and polished from rapid iteration commit chain. All features merged: Graphics/Memory Clocks, Fan Speed, Temperature, Power, Memory Metrics, GPU Frame Time from wgpu timestamps.
// Full Bevy Query safety, comprehensive comments, RBE Flow alignment, Mercy-gated transparent monitoring for player sovereignty.
// AG-SML Licensed. Part of Ra-Thor Lattice / Powrush Divine Module.

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::render::renderer::RenderDevice;
use crate::monitoring::RBEFlowDashboard;
use crate::monitoring::gpu_timestamps::{GpuTimestampQueries, get_latest_gpu_validation};
use crate::monitoring::nvml_monitor::{NvmlMonitorResource, NvmlGpuInfo};

// === DEBUG OVERLAY COMPONENTS ===
#[derive(Component)]
pub struct DebugGpuClocks;

#[derive(Component)]
pub struct DebugGpuMemory;

#[derive(Component)]
pub struct DebugFanSpeed;

#[derive(Component)]
pub struct DebugTemperature;

#[derive(Component)]
pub struct DebugPowerUsage;

#[derive(Component)]
pub struct DebugGpuFrameTime;

#[derive(Component)]
pub struct DebugGpuLoad;

#[derive(Component)]
pub struct DebugRBEFlowStatus;

// === SETUP: Spawn Debug UI Texts (call once in startup) ===
pub fn setup_debug_overlay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Example positioning - adjust for your HUD layout
    let font = asset_server.load("fonts/FiraSans-Bold.ttf"); // or your game font
    let text_style = TextFont {
        font: font.clone(),
        font_size: 14.0,
        ..default()
    };
    let text_color = TextColor(Color::srgb(0.0, 1.0, 0.8)); // Cyan mercy glow

    commands.spawn((
        Text::new("GPU Clocks: --"),
        text_style.clone(),
        text_color.clone(),
        DebugGpuClocks,
        // Node for UI positioning...
    ));

    commands.spawn((
        Text::new("GPU Memory: -- / -- MB"),
        text_style.clone(),
        text_color.clone(),
        DebugGpuMemory,
    ));

    commands.spawn((
        Text::new("Fan: --%"),
        text_style.clone(),
        text_color.clone(),
        DebugFanSpeed,
    ));

    commands.spawn((
        Text::new("Temp: -- C"),
        text_style.clone(),
        text_color.clone(),
        DebugTemperature,
    ));

    commands.spawn((
        Text::new("Power: -- W"),
        text_style.clone(),
        text_color.clone(),
        DebugPowerUsage,
    ));

    commands.spawn((
        Text::new("GPU Frame Time: measuring..."),
        text_style.clone(),
        text_color.clone(),
        DebugGpuFrameTime,
    ));

    commands.spawn((
        Text::new("Est. GPU Load: --%"),
        text_style.clone(),
        text_color.clone(),
        DebugGpuLoad,
    ));

    commands.spawn((
        Text::new("RBE Flow: Sovereign"),
        text_style,
        text_color,
        DebugRBEFlowStatus,
    ));
}

// === UPDATE: Polished full integration ===
pub fn update_debug_overlay(
    rbe_dashboard: Res<RBEFlowDashboard>,
    diagnostics: Res<DiagnosticsStore>,
    gpu_queries: Option<Res<GpuTimestampQueries>>,
    nvml: Option<Res<NvmlMonitorResource>>,
    mut gpu_clocks_q: Query<&mut Text, With<DebugGpuClocks>>,
    mut gpu_memory_q: Query<&mut Text, With<DebugGpuMemory>>,
    mut fan_q: Query<&mut Text, With<DebugFanSpeed>>,
    mut temp_q: Query<&mut Text, With<DebugTemperature>>,
    mut power_q: Query<&mut Text, With<DebugPowerUsage>>,
    mut frame_time_q: Query<&mut Text, With<DebugGpuFrameTime>>,
    mut load_q: Query<&mut Text, With<DebugGpuLoad>>,
    mut rbe_q: Query<&mut Text, With<DebugRBEFlowStatus>>,
) {
    // === RBE FLOW DASHBOARD STATUS (Mercy aligned player sovereignty) ===
    if let Ok(mut text) = rbe_q.get_single_mut() {
        let flow_state = if rbe_dashboard.is_flowing {
            "RBE Flow: Eternal | Sovereign"
        } else {
            "RBE Flow: Stabilizing..."
        };
        text.0 = flow_state.to_string();
    }

    // === NVML GPU METRICS (Full real hardware data - PATSAGi monitored) ===
    if let Some(nvml_res) = nvml {
        let info = nvml_res.0.get_info();

        if info.is_available {
            // GPU Clocks (Graphics + Memory) - Recovered from iteration chain
            if let Ok(mut text) = gpu_clocks_q.get_single_mut() {
                let mut parts = Vec::new();
                if info.graphics_clock_mhz > 0 {
                    parts.push(format!("Graphics: {} MHz", info.graphics_clock_mhz));
                }
                if info.memory_clock_mhz > 0 {
                    parts.push(format!("Memory: {} MHz", info.memory_clock_mhz));
                }
                text.0 = if !parts.is_empty() {
                    format!("GPU Clocks: {}", parts.join(" | "))
                } else {
                    "GPU Clocks: N/A".to_string()
                };
            }

            // GPU Memory
            if let Ok(mut text) = gpu_memory_q.get_single_mut() {
                if info.memory_total_mb > 0 {
                    text.0 = format!("GPU Memory: {} / {} MB ({}% used)", 
                        info.memory_used_mb, 
                        info.memory_total_mb,
                        if info.memory_total_mb > 0 { (info.memory_used_mb * 100 / info.memory_total_mb) } else { 0 });
                } else {
                    text.0 = "GPU Memory: N/A".to_string();
                }
            }

            // Fan Speed
            if let Ok(mut text) = fan_q.get_single_mut() {
                if info.fan_speed_percent > 0 {
                    text.0 = format!("Fan Speed: {}%", info.fan_speed_percent);
                } else {
                    text.0 = "Fan Speed: N/A".to_string();
                }
            }

            // Temperature
            if let Ok(mut text) = temp_q.get_single_mut() {
                if info.temperature_c > 0 {
                    text.0 = format!("GPU Temp: {} C", info.temperature_c);
                } else {
                    text.0 = "GPU Temp: N/A".to_string();
                }
            }

            // Power Usage
            if let Ok(mut text) = power_q.get_single_mut() {
                if info.power_watts > 0.0 {
                    text.0 = format!("GPU Power: {:.1} W", info.power_watts);
                } else {
                    text.0 = "GPU Power: N/A".to_string();
                }
            }
        } else {
            // NVML not available - graceful degradation
            for mut q in [&mut gpu_clocks_q, &mut gpu_memory_q, &mut fan_q, &mut temp_q, &mut power_q] {
                if let Ok(mut text) = q.get_single_mut() {
                    text.0 = "GPU Metrics: NVML Unavailable (feature flag or driver)".to_string();
                }
            }
        }
    }

    // === wgpu REAL GPU FRAME TIME + LOAD (from TimestampQueryNode) ===
    if let Some(queries) = gpu_queries {
        let validation = get_latest_gpu_validation(&queries);

        if let Ok(mut text) = frame_time_q.get_single_mut() {
            if validation.is_valid && validation.last_gpu_time_ms > 0.0 {
                text.0 = format!("GPU Frame Time: {:.2} ms", validation.last_gpu_time_ms);
            } else if validation.last_error.is_some() {
                text.0 = format!("GPU Frame Time: {} (fallback)", validation.last_error.as_deref().unwrap_or("measuring..."));
            } else {
                text.0 = "GPU Frame Time: measuring...".to_string();
            }
        }

        if let Ok(mut text) = load_q.get_single_mut() {
            if validation.is_valid && validation.last_gpu_time_ms > 0.0 {
                // Realistic load estimate based on real GPU time (16.67ms = 60fps target)
                let load = (validation.last_gpu_time_ms / 16.67).clamp(0.0, 2.0) * 50.0;
                text.0 = format!("Est. GPU Load: ~{:.0} %", load.min(100.0));
            } else {
                text.0 = "Est. GPU Load: (initializing)".to_string();
            }
        }
    } else {
        if let Ok(mut text) = frame_time_q.get_single_mut() {
            text.0 = "GPU Frame Time: (wgpu timestamps disabled)".to_string();
        }
    }

    // === DIAGNOSTICS FALLBACK (Frame time from Bevy if needed) ===
    if let Some(fps) = diagnostics.get_measurement(&FrameTimeDiagnosticsPlugin::FPS) {
        // Can extend for additional CPU frame metrics here if desired
        let _fps_avg = fps.average().unwrap_or(0.0);
    }
}

// === Optional: Clear or reset overlay texts ===
pub fn reset_debug_overlay(
    mut clocks_q: Query<&mut Text, With<DebugGpuClocks>>,
    // ... other queries
) {
    for mut text in &mut clocks_q {
        text.0 = "GPU Clocks: --".to_string();
    }
    // Extend for others as needed
}
