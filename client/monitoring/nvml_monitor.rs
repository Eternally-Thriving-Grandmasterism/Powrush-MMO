// client/monitoring/nvml_monitor.rs
// NVIDIA GPU Monitoring via NVML (v18.38 PATSAGi Complete)
// Full sovereign monitoring for Powrush MMOARPG player hardware transparency.
// Feature "nvidia" gated. Thread-safe Arc<Mutex>. Integrated with Debug Overlay and RBE Flow.
// AG-SML v1.0 | Ra-Thor Lattice aligned | Mercy for accurate non-intrusive metrics.

use bevy::prelude::*;
use std::sync::{Arc, Mutex};

#[cfg(feature = "nvidia")]
use nvml_wrapper::{Nvml, enum_wrappers::device::Clock, error::NvmlError};

#[derive(Clone, Debug, Default)]
pub struct NvmlGpuInfo {
    pub utilization_gpu: u32,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub temperature_c: u32,
    pub power_watts: f32,
    pub fan_speed_percent: u32,
    pub graphics_clock_mhz: u32,
    pub memory_clock_mhz: u32,
    pub is_available: bool,
    pub last_error: Option<String>,
}

#[derive(Resource, Clone)]
pub struct NvmlMonitorResource(pub Arc<Mutex<NvmlMonitor>>);

#[derive(Debug)]
pub struct NvmlMonitor {
    #[cfg(feature = "nvidia")]
    nvml: Option<Nvml>,
    pub state: Arc<Mutex<NvmlGpuInfo>>,
}

impl Default for NvmlMonitor {
    fn default() -> Self {
        Self {
            #[cfg(feature = "nvidia")]
            nvml: None,
            state: Arc::new(Mutex::new(NvmlGpuInfo::default())),
        }
    }
}

impl NvmlMonitor {
    pub fn new() -> Self {
        let mut monitor = Self::default();

        #[cfg(feature = "nvidia")]
        {
            match Nvml::init() {
                Ok(nvml) => {
                    monitor.nvml = Some(nvml);
                    info!("[NVML] NVIDIA Management Library initialized successfully. Real GPU metrics active.");
                }
                Err(e) => {
                    warn!("[NVML] Failed to initialize NVML: {:?}. GPU metrics will use fallback.", e);
                }
            }
        }

        #[cfg(not(feature = "nvidia"))]
        {
            warn!("[NVML] 'nvidia' feature not enabled. Compile with --features nvidia for real metrics.");
        }

        monitor
    }

    pub fn get_info(&self) -> NvmlGpuInfo {
        if let Ok(guard) = self.state.lock() {
            guard.clone()
        } else {
            NvmlGpuInfo {
                is_available: false,
                last_error: Some("Lock poisoned".to_string()),
                ..Default::default()
            }
        }
    }

    #[cfg(feature = "nvidia")]
    pub fn update(&self) {
        if let Some(ref nvml) = self.nvml {
            match nvml.device_by_index(0) {
                Ok(device) => {
                    let mut info = NvmlGpuInfo {
                        is_available: true,
                        ..Default::default()
                    };

                    if let Ok(util) = device.utilization_rates() {
                        info.utilization_gpu = util.gpu;
                    }
                    if let Ok(mem) = device.memory_info() {
                        info.memory_used_mb = mem.used / 1_000_000;
                        info.memory_total_mb = mem.total / 1_000_000;
                    }
                    if let Ok(temp) = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu) {
                        info.temperature_c = temp;
                    }
                    if let Ok(power) = device.power_usage() {
                        info.power_watts = power as f32 / 1000.0;
                    }
                    if let Ok(fan) = device.fan_speed(0) {
                        info.fan_speed_percent = fan;
                    }

                    // Clock Speeds - Graphics + Memory (from recent iteration recovery)
                    if let Ok(clock) = device.clock_info(Clock::Graphics) {
                        info.graphics_clock_mhz = clock;
                    }
                    if let Ok(mem_clock) = device.clock_info(Clock::Memory) {
                        info.memory_clock_mhz = mem_clock;
                    }

                    if let Ok(mut guard) = self.state.lock() {
                        *guard = info;
                    }
                }
                Err(e) => {
                    if let Ok(mut guard) = self.state.lock() {
                        guard.last_error = Some(format!("{:?}", e));
                        guard.is_available = false;
                    }
                }
            }
        }
    }
}

// === Bevy Plugin ===
pub struct NvmlMonitorPlugin;

impl Plugin for NvmlMonitorPlugin {
    fn build(&self, app: &mut App) {
        let monitor = NvmlMonitor::new();
        let resource = NvmlMonitorResource(Arc::new(Mutex::new(monitor)));

        app.insert_resource(resource.clone());

        // Update system - runs every frame or at fixed interval
        app.add_systems(Update, update_nvml_monitor);
    }
}

fn update_nvml_monitor(
    nvml_res: Option<Res<NvmlMonitorResource>>,
) {
    if let Some(res) = nvml_res {
        // Call inner update (feature gated inside)
        if let Ok(guard) = res.0.lock() {
            #[cfg(feature = "nvidia")]
            {
                guard.update();
            }
        }
    }
}

// === Helper for Debug Overlay etc. ===
pub fn get_nvml_info(nvml: &NvmlMonitorResource) -> NvmlGpuInfo {
    if let Ok(guard) = nvml.0.lock() {
        guard.get_info()
    } else {
        NvmlGpuInfo { is_available: false, last_error: Some("Mutex lock failed".to_string()), ..Default::default() }
    }
}
