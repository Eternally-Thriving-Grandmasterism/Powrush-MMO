// client/rbe_client_sync.rs
// Powrush-MMO — RBE + Council + Safety Net client sync layer
// Thin integration layer after modularization (v18.37)
// AG-SML v1.0 | TOLC 8 Mercy Gates enforced

use bevy::prelude::*;
use shared::protocol::ServerMessage;
use std::sync::Arc;
use tokio::sync::RwLock;
use bytes::Bytes;

pub mod monitoring;

use crate::inventory_ui::{LocalInventory, TradeUIState, InventoryUpdated, TradeResponseReceived, HarvestResponseReceived, handle_server_message};
use crate::divine_whispers_ui::{CurrentDivineWhisper, DivineWhispersLog, DivineWhisperUI, receive_divine_whisper_from_server};
use crate::monitoring::{
    SafetyNetState, SafetyNetMonitoringUpdate, SafetyNetMonitoringSnapshot,
    KalmanFilter1D, RTSFixedLagSmoother,
};

#[derive(Resource)]
pub struct RbeClientSync {
    pub local_inventory: Arc<RwLock<LocalInventory>>,
    pub trade_state: Arc<RwLock<TradeUIState>>,
    pub safety_net_state: Arc<RwLock<SafetyNetState>>,
}

impl RbeClientSync {
    pub fn new() -> Self {
        Self {
            local_inventory: Arc::new(RwLock::new(LocalInventory::default())),
            trade_state: Arc::new(RwLock::new(TradeUIState::default())),
            safety_net_state: Arc::new(RwLock::new(SafetyNetState::default())),
        }
    }

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
    ) {
        if let Ok(msg) = bincode::deserialize::<ServerMessage>(&data) {
            let mut inv = self.local_inventory.write().await;
            let mut trade = self.trade_state.write().await;

            handle_server_message(&msg, &mut inv, &mut trade, inventory_events, trade_events, harvest_events);

            if let ServerMessage::SafetyNetBroadcast { broadcast } = &msg {
                self.handle_safety_net_broadcast(broadcast, &mut monitoring_events).await;
            }

            // TODO: Add handling for other message types (GpuPatsagiUpdate, DivineWhisper, etc.)
        }
    }

    async fn handle_safety_net_broadcast(
        &self,
        broadcast: &shared::protocol::SafetyNetBroadcast,
        monitoring_events: &mut EventWriter<SafetyNetMonitoringUpdate>,
    ) {
        let mut safety = self.safety_net_state.write().await;

        // Update basic server state
        safety.last_tick = broadcast.snapshot.tick;
        safety.last_abundance = broadcast.snapshot.abundance;
        safety.last_health = broadcast.snapshot.current_health;
        safety.last_council_engagement = broadcast.snapshot.council_engagement_score;

        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let latency_ms = if broadcast.emit_timestamp_ms > 0 {
            now_ms.saturating_sub(broadcast.emit_timestamp_ms)
        } else {
            0
        };

        let jitter_ms = if safety.previous_latency_ms > 0 {
            (latency_ms as i64 - safety.previous_latency_ms as i64).unsigned_abs()
        } else {
            0
        };

        safety.last_latency_ms = latency_ms;
        safety.sample_count = safety.sample_count.saturating_add(1);

        if safety.sample_count == 1 {
            safety.ema_latency_ms = latency_ms as f32;
            safety.kalman_latency = Some(KalmanFilter1D::new(latency_ms as f32));
            safety.rts_smoother = Some(RTSFixedLagSmoother::new(8));
        } else {
            // Update EMA and Kalman
            let dt_sec = 0.016; // approximate
            if let Some(k) = &mut safety.kalman_latency {
                k.update(latency_ms as f32, dt_sec);
            }
            if let Some(rts) = &mut safety.rts_smoother {
                let cov = safety.kalman_latency.as_ref().map_or(1.0, |k| k.error_estimate.max(0.1));
                rts.update(latency_ms as f32, cov, dt_sec);
            }
        }

        safety.previous_latency_ms = latency_ms;

        // Emit monitoring update periodically
        if safety.sample_count % 5 == 0 {
            let rts_val = safety.rts_smoother.as_ref().map_or(0.0, |r| r.smoothed_estimate);
            let kalman_val = safety.kalman_latency.as_ref().map_or(0.0, |k| k.estimate);

            let snapshot = SafetyNetMonitoringSnapshot {
                timestamp_ms: now_ms,
                last_latency_ms: latency_ms,
                avg_latency_ms: safety.ema_latency_ms,
                kalman_latency_residual: safety.kalman_latency.as_ref().map_or(0.0, |k| k.last_residual),
                rts_smoothed_latency: rts_val,
                rts_vs_kalman_residual: rts_val - kalman_val,
                server_abundance: safety.last_abundance,
                server_health: safety.last_health,
                server_council_engagement: safety.last_council_engagement,
            };

            monitoring_events.send(SafetyNetMonitoringUpdate { snapshot });
        }
    }
}