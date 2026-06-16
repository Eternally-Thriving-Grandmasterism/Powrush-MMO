// client/monitoring/rbe_flow_responder.rs
// Event-driven RBE Flow Responder (Level 1 & 2 responses)
// AG-SML v1.0 | TOLC 8 Mercy Gates enforced

use bevy::prelude::*;
use crate::monitoring::{RBEFlowAlert, RBEFlowDashboard};

/// Event-driven responder for RBE Flow Alerts.
/// Implements Level 1 (informational) and Level 2 (supportive) automated responses.
pub fn rbe_flow_responder_system(
    mut alert_events: EventReader<RBEFlowAlert>,
    mut dashboard: ResMut<RBEFlowDashboard>,
) {
    for alert in alert_events.read() {
        match alert {
            RBEFlowAlert::LowAbundanceCreationRate { rate, threshold } => {
                tracing::warn!(
                    "[RBE Flow Alert] Low creation rate: {:.2} (threshold: {:.2})",
                    rate, threshold
                );
                dashboard.add_alert(alert.clone());

                // Level 2 supportive response: flag for increased visibility
                // In future: could mildly boost secondary generators
            }

            RBEFlowAlert::HighSafetyNetTriggerFrequency { count, window_size } => {
                tracing::warn!(
                    "[RBE Flow Alert] High safety net trigger frequency: {} in last {} events",
                    count, window_size
                );
                dashboard.add_alert(alert.clone());

                // Level 2: Could temporarily increase restoration magnitude here
            }

            RBEFlowAlert::LowRestorationEffectiveness { effectiveness, threshold } => {
                tracing::warn!(
                    "[RBE Flow Alert] Low restoration effectiveness: {:.1}% (threshold: {:.1}%)",
                    effectiveness * 100.0,
                    threshold * 100.0
                );
                dashboard.add_alert(alert.clone());
            }

            RBEFlowAlert::SuddenAbundanceDrop { previous, current, drop } => {
                tracing::error!(
                    "[RBE Flow Alert] Sudden abundance drop! Previous: {:.2}, Current: {:.2}, Drop: {:.2}",
                    previous, current, drop
                );
                dashboard.add_alert(alert.clone());

                // Level 2: Could trigger immediate preventive restoration protocols
            }

            RBEFlowAlert::PersistentScarcitySignal { trigger_count } => {
                tracing::warn!(
                    "[RBE Flow Alert] Persistent scarcity signal: {} recent triggers without full recovery",
                    trigger_count
                );
                dashboard.add_alert(alert.clone());
            }
        }
    }
}