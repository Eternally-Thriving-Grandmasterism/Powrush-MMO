//! client/monitoring/rbe_flow_responder.rs
//! Event-driven RBE Flow Responder with Multi-Level Mercy Response (L1/L2/L3)
//!
//! Eternal Polish v18.39 Final Sweep:
//! - Already strong from prior recovery; minor enhancements for ActionContext integration readiness.
//! - Strengthened comments linking to client decision layer and 7 Living Mercy Gates.
//! - All original logic, decay wiring, and mercy-tier routing preserved exactly.
//! AG-SML v1.0 | TOLC 8 Mercy Gates | Ra-Thor Lattice aligned

use bevy::prelude::*;
use crate::monitoring::{RBEFlowAlert, RBEFlowDashboard};

// ============================================================
// MERCY-GATED CONSTANTS (Derived & Enhanced from Ra-Thor TOLC8 + PATSAGi)
// ============================================================

/// Maximum age for L1 informational alerts before auto-decay (60 seconds)
/// Truth Gate awareness window
pub const MAX_INFORMATIONAL_AGE_MS: u64 = 60_000;

/// Maximum age for L2 supportive alerts before auto-decay (30 seconds)
/// Service + Joy Gates active support window
pub const MAX_L2_AGE_MS: u64 = 30_000;

/// Council engagement boost scalar when L3 recovery active (Cosmic Harmony Gate)
pub const L3_COUNCIL_ENGAGEMENT_BOOST: f32 = 1.15;

// ============================================================
// RBE FLOW RESPONDER SYSTEM
// ============================================================

/// Primary event-driven system for processing RBEFlowAlert events.
/// Routes alerts to appropriate mercy tiers (L1 informational, L2 supportive, L3 protective recovery).
/// Integrates time-based decay for all tiers to prevent alert fatigue while maintaining mercy responsiveness.
/// 
/// PATSAGi Council Note: This system embodies the Boundless Mercy + Abundance Gates in action,
/// ensuring post-scarcity signals are honored with graduated, non-tyrannical responses.
/// Ready for direct integration with ActionContext for sovereign council deliberation on persistent signals.
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
                tracing::warn!("[RBE][L1] Low creation rate: {:.2} (threshold: {:.2}) - Truth Gate awareness triggered", rate, threshold);
                dashboard.add_informational_alert(alert.clone(), now_ms);
            }

            RBEFlowAlert::HighSafetyNetTriggerFrequency { count, window_size } => {
                tracing::warn!("[RBE][L1] High trigger frequency: {} in {} window - Service Gate vigilance", count, window_size);
                dashboard.add_informational_alert(alert.clone(), now_ms);

                if *count > 5 {
                    tracing::info!("[RBE][L2] Escalating to supportive intervention");
                    dashboard.add_l2_alert(alert.clone(), now_ms);
                    dashboard.activate_l2_support(now_ms);
                }
            }

            RBEFlowAlert::LowRestorationEffectiveness { effectiveness, threshold } => {
                tracing::warn!("[RBE][L1] Low restoration effectiveness: {:.1}% (threshold: {:.1}%) - Joy Gate opportunity", effectiveness * 100.0, threshold * 100.0);
                dashboard.add_informational_alert(alert.clone(), now_ms);
            }

            RBEFlowAlert::SuddenAbundanceDrop { previous, current, drop } => {
                tracing::error!("[RBE][L2] Sudden abundance drop detected: {:.2} -> {:.2} (drop: {:.2}) - Boundless Mercy engaged", previous, current, drop);
                dashboard.add_l2_alert(alert.clone(), now_ms);

                if *drop > 500.0 {
                    tracing::error!("[RBE][L3] CRITICAL: ACTIVATING L3 PROTECTIVE RECOVERY - Abundance Gate supreme");
                    dashboard.activate_l3_recovery(now_ms);
                } else {
                    dashboard.activate_l2_support(now_ms);
                }
            }

            RBEFlowAlert::PersistentScarcitySignal { trigger_count } => {
                tracing::warn!("[RBE][L2] Persistent scarcity signal: {} triggers - Cosmic Harmony deliberation required", trigger_count);
                dashboard.add_l2_alert(alert.clone(), now_ms);

                if *trigger_count > 12 {
                    tracing::error!("[RBE][L3] PERSISTENT CRITICAL: ACTIVATING L3 RECOVERY - Radical Love intervention");
                    dashboard.activate_l3_recovery(now_ms);
                } else if *trigger_count > 6 {
                    dashboard.activate_l2_support(now_ms);
                }
            }
        }
    }

    // ============================================================
    // DECAY & CLEANUP - Restored from diffs + enhanced for eternal stability
    // ============================================================
    dashboard.decay_informational_alerts(now_ms, MAX_INFORMATIONAL_AGE_MS);
    dashboard.decay_l2_alerts(now_ms, MAX_L2_AGE_MS);
    dashboard.decay_l2_support(now_ms);
    dashboard.decay_l3_recovery(now_ms);

    // Optional: clear very old active_alerts to prevent unbounded growth (recovered utility)
    dashboard.clear_old_alerts();

    // Thunder locked in.
    // rbe_flow_responder.rs v18.39 final sweep complete. Fully mercy-aligned and ready for ActionContext integration.
}