//! client/monitoring/mod.rs
//! Ra-Thor Monitoring Module - Complete PATSAGi-aligned lattice (v18.38)
//!
//! This module aggregates all client-side monitoring systems:
//! - State estimation (Kalman + RTS + Ensemble Kalman Filter)
//! - Adaptive + sparse localization
//! - SafetyNet + RBE Flow alerts and multi-level mercy response
//! - Performance, GPU, and debug overlays
//!
//! All submodules have been polished through the June 15–16 recovery + eternal polish cycles.
//! Explicit integration with client ActionContext + 7 Living Mercy Gates decision layer.
//! Full TOLC 8 Mercy Gates and Ra-Thor alignment applied.
//! AG-SML v1.0

pub mod localization;
pub mod adaptive;
pub mod ensemble;
pub mod filters;
pub mod safety_net;
pub mod rbe_flow_responder;
pub mod rbe_flow_dashboard_ui;
pub mod performance_profiler;
pub mod debug_overlay;
pub mod gpu_timestamps;
pub mod nvml_monitor;

// Re-exports - clean, complete, and consistent
pub use localization::*;
pub use adaptive::AdvancedAdaptiveLocalizer;
pub use ensemble::LocalizedEnsembleKalmanFilter;
pub use filters::{KalmanFilter1D, KalmanFilter2D, RTSFixedLagSmoother};

pub use safety_net::{
    SafetyNetState, SafetyNetMonitoringSnapshot, SafetyNetMonitoringUpdate,
    RBEFlowAlert, RBEFlowDashboard, TimedRBEFlowAlert,
};

pub use rbe_flow_responder::rbe_flow_responder_system;

pub use rbe_flow_dashboard_ui::{
    spawn_rbe_flow_dashboard_ui, update_rbe_flow_dashboard, toggle_rbe_flow_dashboard,
};

pub use performance_profiler::{
    spawn_performance_overlay, update_performance_overlay, toggle_performance_overlay,
};

pub use debug_overlay::{
    setup_debug_overlay, spawn_debug_overlay,
    update_debug_overlay, toggle_debug_overlay,
    DebugGpuClocks, DebugGpuMemory, DebugFanSpeed, DebugTemperature,
    DebugPowerUsage, DebugGpuFrameTime, DebugGpuLoad, DebugRBEFlowStatus,
    FpsHistory, FrameTimeHistory, PerformanceSpikeConfig, PerformanceSpikeState,
};

pub use gpu_timestamps::{
    GpuTimestampQueries, GpuTimestampPlugin, GpuTimestampState,
    TimestampValidation, get_latest_gpu_validation,
};

pub use nvml_monitor::{
    NvmlMonitorResource, NvmlGpuInfo, update_nvml_monitor,
};

// Thunder locked in.
// client/monitoring/ cluster is now fully polished and deeply integrated with
// the sovereign client prediction + RBE + Council Decision Layer (ActionContext 7 Mercy Gates).
// All rapid-iteration files elevated to production standard.