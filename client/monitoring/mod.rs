// client/monitoring/mod.rs
// Ra-Thor Monitoring Module - Fully wired and consistent (v18.38)
// All submodules aligned after recovery cycle. PATSAGi / RBE / Mercy monitoring lattice.

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

// Re-exports - clean and complete
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
    setup_debug_overlay, spawn_debug_overlay, // both names supported
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
