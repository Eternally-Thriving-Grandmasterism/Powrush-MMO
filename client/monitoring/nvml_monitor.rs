// client/monitoring/nvml_monitor.rs
// NVIDIA GPU Monitoring via NVML (v18.37)

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
    pub graphics_clock_mhz: u32,   // NEW
    pub memory_clock_mhz: u32,     // NEW
    pub is_available: bool,
    pub last_error: Option<String>,
}

// ... (rest of struct and stub implementation remain) ...

#[cfg(feature = "nvidia")]
impl NvmlMonitor {
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

                    // Clock Speeds
                    if let Ok(clock) = device.clock_info(Clock::Graphics) {
                        info.graphics_clock_mhz = clock;
                    }
                    if let Ok(mem_clock) = device.clock_info(Clock::Memory) {
                        info.memory_clock_mhz = mem_clock;
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
}