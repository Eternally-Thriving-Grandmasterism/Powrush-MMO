// client/monitoring/mod.rs
// Ra-Thor Monitoring Module

pub mod localization;
pub mod adaptive;
pub mod ensemble;
pub mod filters;
pub mod safety_net;
pub mod rbe_flow_responder;
pub mod rbe_flow_dashboard_ui;
pub mod performance_profiler;
pub mod debug_overlay;

// Re-exports
pub use localization::*;
pub use adaptive::AdvancedAdaptiveLocalizer;
pub use ensemble::LocalizedEnsembleKalmanFilter;
pub use filters::{KalmanFilter1D, KalmanFilter2D, RTSFixedLagSmoother};
pub use safety_net::{SafetyNetState, SafetyNetMonitoringSnapshot, SafetyNetMonitoringUpdate, RBEFlowAlert, RBEFlowDashboard, TimedRBEFlowAlert};
pub use rbe_flow_responder::rbe_flow_responder_system;
pub use rbe_flow_dashboard_ui::{spawn_rbe_flow_dashboard_ui, update_rbe_flow_dashboard, toggle_rbe_flow_dashboard};
pub use performance_profiler::{spawn_performance_overlay, update_performance_overlay, toggle_performance_overlay};
pub use debug_overlay::{spawn_debug_overlay, update_debug_overlay, toggle_debug_overlay, FpsHistory};