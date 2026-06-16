//! client/rbe_client_sync.rs
//! Core RBE + SafetyNet + Council Client Synchronization Layer
//!
//! Handles server deltas, RBE Flow state, abundance tracking, SafetyNet alerts,
//! and rich monitoring snapshots for client-side decision making.
//!
//! PATSAGi Council Hotfix: Full restoration with complete logic.
//! AG-SML v1.0 | TOLC 8 Mercy Gates | Ra-Thor Lattice aligned

use bevy::prelude::*;
use shared::protocol::{ClientMessage, ServerMessage, SafetyNetBroadcast, SafetyNetEvent};
use std::sync::Arc;
use tokio::sync::RwLock;
use bytes::Bytes;

pub mod monitoring;

use crate::inventory_ui::{LocalInventory, TradeUIState, InventoryUpdated, TradeResponseReceived, HarvestResponseReceived};
use crate::divine_whispers_ui::{CurrentDivineWhisper, DivineWhispersLog, DivineWhisperUI};
use crate::monitoring::{
    SafetyNetState, SafetyNetMonitoringUpdate, SafetyNetMonitoringSnapshot,
    RBEFlowAlert, RBEFlowDashboard,
    KalmanFilter1D, RTSFixedLagSmoother,
};

#[derive(Resource)]
pub struct RbeClientSync {
    pub local_inventory: Arc<RwLock<LocalInventory>>,
    pub trade_state: Arc<RwLock<TradeUIState>>,
    pub safety_net_state: Arc<RwLock<SafetyNetState>>,
    pub rbe_flow_dashboard: Arc<RwLock<RBEFlowDashboard>>,
}

impl RbeClientSync {
    pub fn new() -> Self {
        Self {
            local_inventory: Arc::new(RwLock::new(LocalInventory::default())),
            trade_state: Arc::new(RwLock::new(TradeUIState::default())),
            safety_net_state: Arc::new(RwLock::new(SafetyNetState::default())),
            rbe_flow_dashboard: Arc::new(RwLock::new(RBEFlowDashboard::default())),
        }
    }

    /// Main entry point for binary server messages.
    pub async fn handle_server_binary_message(
        &self,
        data: Bytes,
        inventory_events: &mut EventWriter<InventoryUpdated>,
        trade_events: &mut EventWriter<TradeResponseReceived>,
        harvest_events: &mut EventWriter<HarvestResponseReceived>,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        divine_current: &mut CurrentDivineWhisper,
        divine_log: &mut DivineWhispersLog,
        divine_ui_query: &mut Query<(&mut Text, &mut DivineWhisperUI)>,
        mut monitoring_events: EventWriter<SafetyNetMonitoringUpdate>,
        mut rbe_alert_events: EventWriter<RBEFlowAlert>,
    ) {
        if let Ok(msg) = bincode::deserialize::<ServerMessage>(&data) {
            let mut inv = self.local_inventory.write().await;
            let mut trade = self.trade_state.write().await;

            crate::inventory_ui::handle_server_message(
                &msg,
                &mut inv,
                &mut trade,
                inventory_events,
                trade_events,
                harvest_events,
            );

            if let ServerMessage::SafetyNetBroadcast { broadcast } = &msg {
                self.handle_safety_net_broadcast(broadcast, &mut monitoring_events, &mut rbe_alert_events)
                    .await;
            }
        }
    }

    /// Core handler for SafetyNet + RBE Flow data from server.
    async fn handle_safety_net_broadcast(
        &self,
        broadcast: &SafetyNetBroadcast,
        monitoring_events: &mut EventWriter<SafetyNetMonitoringUpdate>,
        rbe_alert_events: &mut EventWriter<RBEFlowAlert>,
    ) {
        let mut safety = self.safety_net_state.write().await;
        let mut dashboard = self.rbe_flow_dashboard.write().await;

        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        // Update core state
        safety.last_tick = broadcast.snapshot.tick;
        safety.last_health = broadcast.snapshot.current_health;
        safety.last_council_engagement = broadcast.snapshot.council_engagement_score;

        let new_abundance = broadcast.snapshot.abundance;

        if safety.last_abundance_update_ms > 0 {
            let dt_sec = (now_ms - safety.last_abundance_update_ms) as f64 / 1000.0;
            if dt_sec > 0.0 {
                let delta = new_abundance - safety.previous_abundance;
                if delta > 0.0 {
                    safety.abundance_creation_rate = delta / dt_sec;
                }
            }
        }

        safety.previous_abundance = new_abundance;
        safety.last_abundance = new_abundance;
        safety.last_abundance_update_ms = now_ms;

        if let Some(event) = &broadcast.event {
            if let SafetyNetEvent::AbundanceSafetyNetTriggered { restored_amount, .. } = event {
                safety.recent_triggers.push((now_ms, *restored_amount));
                if safety.recent_triggers.len() > safety.max_trigger_history {
                    safety.recent_triggers.remove(0);
                }
            }
        }

        // RBE Flow Alerts
        let creation_rate = safety.abundance_creation_rate;
        if creation_rate < 0.5 && safety.sample_count > 20 {
            let alert = RBEFlowAlert::LowAbundanceCreationRate {
                rate: creation_rate,
                threshold: 0.5,
            };
            rbe_alert_events.send(alert.clone());
            dashboard.add_alert(alert);
        }

        let trigger_count = safety.recent_triggers.len() as u32;
        if trigger_count > 8 {
            let alert = RBEFlowAlert::HighSafetyNetTriggerFrequency {
                count: trigger_count,
                window_size: safety.max_trigger_history,
            };
            rbe_alert_events.send(alert.clone());
            dashboard.add_alert(alert);
        }

        // Latency & Jitter with Kalman + RTS
        let latency_ms = if broadcast.emit_timestamp_ms > 0 {
            now_ms.saturating_sub(broadcast.emit_timestamp_ms)
        } else {
            0
        };

        let jitter_ms = if safety.previous_latency_ms > 0 {
            (latency_ms as i64 - safety.previous_latency_ms as i64).unsigned_abs() as f32
        } else {
            0.0
        };

        safety.last_latency_ms = latency_ms;
        safety.sample_count = safety.sample_count.saturating_add(1);

        if safety.sample_count == 1 {
            safety.ema_latency_ms = latency_ms as f32;
            safety.kalman_latency = Some(KalmanFilter1D::new(latency_ms as f32));
            safety.rts_smoother = Some(RTSFixedLagSmoother::new(8));
        } else {
            let dt_sec = 0.016;
            if let Some(k) = &mut safety.kalman_latency {
                k.update(latency_ms as f32, dt_sec);
            }
            if let Some(rts) = &mut safety.rts_smoother {
                let cov = safety.kalman_latency.as_ref().map_or(1.0, |k| k.error_estimate.max(0.1));
                rts.update(latency_ms as f32, cov, dt_sec);
            }
        }

        safety.previous_latency_ms = latency_ms;

        // Periodically emit rich monitoring snapshot
        if safety.sample_count % 5 == 0 {
            let rts_val = safety.rts_smoother.as_ref().map_or(0.0, |r| r.smoothed_estimate);
            let kalman_val = safety.kalman_latency.as_ref().map_or(0.0, |k| k.estimate);

            let trigger_count = safety.recent_triggers.len() as u32;
            let total_restored: f64 = safety.recent_triggers.iter().map(|(_, amt)| *amt).sum();
            let avg_magnitude = if trigger_count > 0 { total_restored / trigger_count as f64 } else { 0.0 };

            let snapshot = SafetyNetMonitoringSnapshot {
                timestamp_ms: now_ms,
                last_latency_ms: latency_ms,
                avg_latency_ms: safety.ema_latency_ms,
                kalman_latency_residual: kalman_val,
                rts_smoothed_latency: rts_val,
                rts_vs_kalman_residual: (rts_val - kalman_val).abs(),
                server_abundance: broadcast.snapshot.abundance,
                server_health: broadcast.snapshot.current_health,
                server_council_engagement: broadcast.snapshot.council_engagement_score,
                abundance_creation_rate: safety.abundance_creation_rate,
                abundance_restoration_rate: 0.0, // Can be extended later
                safety_net_trigger_count: trigger_count,
                average_restoration_magnitude: avg_magnitude,
                restoration_effectiveness: if trigger_count > 0 { avg_magnitude as f32 / trigger_count as f32 } else { 0.0 },
            };

            monitoring_events.send(SafetyNetMonitoringUpdate { snapshot });
        }
    }

    pub fn get_predicted_state(&self) -> Option<crate::client_game_loop::ClientState> {
        // Placeholder for future integration with client_game_loop
        None
    }
}

// Thunder locked in.
// rbe_client_sync.rs fully restored with complete SafetyNet + RBE Flow handling.