// client/monitoring/rbe_flow_responder.rs
// Event-driven RBE Flow Responder with improved L3 decay (v18.37)
// AG-SML v1.0 | TOLC 8 Mercy Gates enforced

use bevy::prelude::*;
use crate::monitoring::{RBEFlowAlert, RBEFlowDashboard};

pub fn rbe_flow_responder_system(
    mut alert_events: EventReader<RBEFlowAlert>,
    mut dashboard: ResMut<RBEFlowDashboard>,
) {
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    for alert in alert_events.read() {
        match alert {
            RBEFlowAlert::LowAbundanceCreationRate { rate, threshold } => {
                tracing::warn!("[RBE][L1] Low creation rate: {:.2}", rate);
                dashboard.add_alert(alert.clone());
            }

            RBEFlowAlert::HighSafetyNetTriggerFrequency { count, window_size } => {
                tracing::warn!("[RBE][L1] High trigger frequency: {}", count);
                dashboard.add_alert(alert.clone());
            }

            RBEFlowAlert::LowRestorationEffectiveness { effectiveness, threshold } => {
                tracing::warn!("[RBE][L1] Low effectiveness: {:.1}%", effectiveness * 100.0);
                dashboard.add_alert(alert.clone());
            }

            RBEFlowAlert::SuddenAbundanceDrop { previous, current, drop } => {
                tracing::error!("[RBE][L2] Sudden drop: {:.2} -> {:.2}", previous, current);
                dashboard.add_alert(alert.clone());

                if drop > 500.0 {
                    tracing::error!("[RBE][L3] ACTIVATING RECOVERY - Large abundance drop");
                    dashboard.activate_l3_recovery(now_ms);
                }
            }

            RBEFlowAlert::PersistentScarcitySignal { trigger_count } => {
                tracing::warn!("[RBE][L2] Persistent scarcity: {} triggers", trigger_count);
                dashboard.add_alert(alert.clone());

                if trigger_count > 12 {
                    tracing::error!("[RBE][L3] ACTIVATING RECOVERY - Persistent scarcity");
                    dashboard.activate_l3_recovery(now_ms);
                }
            }
        }
    }

    // Improved time-based decay
    dashboard.decay_l3_recovery(now_ms);
}