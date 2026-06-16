// client/monitoring/nvml_monitor.rs
// NVIDIA GPU Monitoring via NVML (v18.37)
// Provides accurate GPU utilization and memory usage on NVIDIA hardware

use bevy::prelude::*;
use std::sync::{Arc, Mutex};

#[cfg(feature = "nvidia")]
use nvml_wrapper::{Nvml, error::NvmlError};

#[derive(Clone, Debug, Default)]
pub struct NvmlGpuInfo {
    pub utilization_gpu: u32,      // 0-100
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub temperature_c: u32,
    pub power_watts: f32,
    pub is_available: bool,
    pub last_error: Option<String>,
}

#[derive(Resource, Clone, Default)]
pub struct NvmlGpuState {
    pub info: Arc<Mutex<NvmlGpuInfo>>,
}

#[cfg(feature = "nvidia")]
pub struct NvmlMonitor {
    nvml: Option<Nvml>,
    state: NvmlGpuState,
}

#[cfg(feature = "nvidia")]
impl NvmlMonitor {
    pub fn new() -> Self {
        let nvml = match Nvml::init() {
            Ok(nvml) => {
                info!("[NVML] Successfully initialized NVIDIA Management Library");
                Some(nvml)
            }
            Err(e) => {
                warn!("[NVML] Failed to initialize: {:?}. NVIDIA metrics will be unavailable.", e);
                None
            }
        };

        Self {
            nvml,
            state: NvmlGpuState::default(),
        }
    }

    pub fn update(&self) {
        if let Some(ref nvml) = self.nvml {
            match nvml.device_by_index(0) {
                Ok(device) => {
                    let mut info = NvmlGpuInfo {
                        is_available: true,
                        ..Default::default()
                    };

                    // GPU Utilization
                    if let Ok(util) = device.utilization_rates() {
                        info.utilization_gpu = util.gpu;
                    }

                    // Memory Info
                    if let Ok(mem) = device.memory_info() {
                        info.memory_used_mb = mem.used / 1_000_000;
                        info.memory_total_mb = mem.total / 1_000_000;
                    }

                    // Temperature
                    if let Ok(temp) = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu) {
                        info.temperature_c = temp;
                    }

                    // Power Usage
                    if let Ok(power) = device.power_usage() {
                        info.power_watts = power as f32 / 1000.0;
                    }

                    if let Ok(mut guard) = self.state.info.lock() {
                        *guard = info;
                    }
                }
                Err(e) => {
                    if let Ok(mut guard) = self.state.info.lock() {
                        guard.last_error = Some(format!("{:?}", e));
                        guard.is_available = false;
                    }
                }
            }
        }
    }

    pub fn get_info(&self) -> NvmlGpuInfo {
        if let Ok(guard) = self.state.info.lock() {
            guard.clone()
        } else {
            NvmlGpuInfo::default()
        }
    }
}

// Stub implementation when NVML feature is disabled
#[cfg(not(feature = "nvidia"))]
pub struct NvmlMonitor;

#[cfg(not(feature = "nvidia"))]
impl NvmlMonitor {
    pub fn new() -> Self { Self }
    pub fn update(&self) {}
    pub fn get_info(&self) -> NvmlGpuInfo { NvmlGpuInfo { is_available: false, ..Default::default() } }
}

#[derive(Resource)]
pub struct NvmlMonitorResource(pub NvmlMonitor);

pub fn update_nvml_metrics(nvml: Res<NvmlMonitorResource>) {
    nvml.0.update();
}