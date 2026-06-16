// client/monitoring/rbe_flow_responder.rs
// Event-driven RBE Flow Responder with concrete Level 3 recovery (v18.37)
// AG-SML v1.0 | TOLC 8 Mercy Gates enforced

use bevy::prelude::*;
use crate::monitoring::{RBEFlowAlert, RBEFlowDashboard};

/// Event-driven responder for RBE Flow Alerts.
/// Includes concrete Level 3 automated recovery actions.
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
            // Level 1
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

            // Level 2
            RBEFlowAlert::SuddenAbundanceDrop { previous, current, drop } => {
                tracing::error!("[RBE][L2] Sudden drop: {:.2} -> {:.2}", previous, current);
                dashboard.add_alert(alert.clone());
            }

            RBEFlowAlert::PersistentScarcitySignal { trigger_count } => {
                tracing::warn!("[RBE][L2] Persistent scarcity: {} triggers", trigger_count);
                dashboard.add_alert(alert.clone());

                // === CONCRETE LEVEL 3 RECOVERY ===
                if trigger_count > 12 {
                    tracing::error!("[RBE][L3] ACTIVATING AUTOMATED RECOVERY - Persistent scarcity detected");
                    dashboard.activate_l3_recovery(now_ms);
                }
            }
        }
    }

    // Decay Level 3 recovery over time
    dashboard.decay_l3_recovery();
}