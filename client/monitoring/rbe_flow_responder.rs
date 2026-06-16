// client/monitoring/rbe_flow_responder.rs
// Event-driven RBE Flow Responder with L2 alert decay (v18.37)
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
                dashboard.add_informational_alert(alert.clone());
            }

            RBEFlowAlert::HighSafetyNetTriggerFrequency { count, window_size } => {
                tracing::warn!("[RBE][L1] High trigger frequency: {}", count);
                dashboard.add_informational_alert(alert.clone());

                if *count > 5 {
                    dashboard.add_l2_alert(alert.clone());
                    dashboard.activate_l2_support(now_ms);
                }
            }

            RBEFlowAlert::LowRestorationEffectiveness { effectiveness, threshold } => {
                tracing::warn!("[RBE][L1] Low effectiveness: {:.1}%", effectiveness * 100.0);
                dashboard.add_informational_alert(alert.clone());
            }

            RBEFlowAlert::SuddenAbundanceDrop { previous, current, drop } => {
                tracing::error!("[RBE][L2] Sudden drop: {:.2} -> {:.2}", previous, current);
                dashboard.add_l2_alert(alert.clone());

                if *drop > 500.0 {
                    tracing::error!("[RBE][L3] ACTIVATING L3 RECOVERY");
                    dashboard.activate_l3_recovery(now_ms);
                } else {
                    dashboard.activate_l2_support(now_ms);
                }
            }

            RBEFlowAlert::PersistentScarcitySignal { trigger_count } => {
                tracing::warn!("[RBE][L2] Persistent scarcity: {} triggers", trigger_count);
                dashboard.add_l2_alert(alert.clone());

                if *trigger_count > 12 {
                    tracing::error!("[RBE][L3] ACTIVATING L3 RECOVERY");
                    dashboard.activate_l3_recovery(now_ms);
                } else if *trigger_count > 6 {
                    dashboard.activate_l2_support(now_ms);
                }
            }
        }
    }

    // Decay logic for all tiers
    dashboard.decay_l2_alerts();
    dashboard.decay_l2_support(now_ms);
    dashboard.decay_l3_recovery(now_ms);
}