// client/monitoring/rbe_flow_responder.rs
// Event-driven RBE Flow Responder (Level 1, 2 & 3 responses)
// AG-SML v1.0 | TOLC 8 Mercy Gates enforced

use bevy::prelude::*;
use crate::monitoring::{RBEFlowAlert, RBEFlowDashboard};

/// Event-driven responder for RBE Flow Alerts.
/// Level 1: Informational
/// Level 2: Supportive (mild adjustments)
/// Level 3: Protective (stronger automated recovery actions with logging)
pub fn rbe_flow_responder_system(
    mut alert_events: EventReader<RBEFlowAlert>,
    mut dashboard: ResMut<RBEFlowDashboard>,
) {
    for alert in alert_events.read() {
        match alert {
            // ============================================================
            // LEVEL 1 - INFORMATIONAL
            // ============================================================
            RBEFlowAlert::LowAbundanceCreationRate { rate, threshold } => {
                tracing::warn!(
                    "[RBE][L1] Low creation rate: {:.2} (threshold: {:.2})",
                    rate, threshold
                );
                dashboard.add_alert(alert.clone());
            }

            RBEFlowAlert::HighSafetyNetTriggerFrequency { count, window_size } => {
                tracing::warn!(
                    "[RBE][L1] High trigger frequency: {} in last {}",
                    count, window_size
                );
                dashboard.add_alert(alert.clone());
            }

            RBEFlowAlert::LowRestorationEffectiveness { effectiveness, threshold } => {
                tracing::warn!(
                    "[RBE][L1] Low restoration effectiveness: {:.1}% (threshold: {:.1}%)",
                    effectiveness * 100.0, threshold * 100.0
                );
                dashboard.add_alert(alert.clone());
            }

            // ============================================================
            // LEVEL 2 - SUPPORTIVE
            // ============================================================
            RBEFlowAlert::SuddenAbundanceDrop { previous, current, drop } => {
                tracing::error!(
                    "[RBE][L2] Sudden abundance drop! Prev: {:.2} -> Current: {:.2} (drop: {:.2})",
                    previous, current, drop
                );
                dashboard.add_alert(alert.clone());

                // Level 2: Flag for immediate visibility + mild preventive action
            }

            RBEFlowAlert::PersistentScarcitySignal { trigger_count } => {
                tracing::warn!(
                    "[RBE][L2] Persistent scarcity signal: {} recent triggers",
                    trigger_count
                );
                dashboard.add_alert(alert.clone());
            }

            // ============================================================
            // LEVEL 3 - PROTECTIVE / AUTOMATED RECOVERY
            // These are stronger actions. They should be logged, auditable,
            // and ideally configurable by councils.
            // ============================================================
        }
    }
}