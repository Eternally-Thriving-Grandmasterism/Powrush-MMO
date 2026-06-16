//! client/rbe_client_sync.rs
//! Core RBE + SafetyNet + Council Client Synchronization Layer
//!
//! Expanded with deeper ClientGameLoop coupling and advanced harvest logic.
//! AG-SML v1.0 | TOLC 8 Mercy Gates | Ra-Thor Lattice aligned

use bevy::prelude::*;
use shared::protocol::{ClientMessage, ServerMessage, SafetyNetBroadcast, SafetyNetEvent};
use std::sync::Arc;
use tokio::sync::RwLock;
use bytes::Bytes;

pub mod monitoring;

use crate::client_game_loop::ClientState;
use crate::inventory_ui::{LocalInventory, TradeUIState, InventoryUpdated, TradeResponseReceived, HarvestResponseReceived};
use crate::divine_whispers_ui::{CurrentDivineWhisper, DivineWhispersLog, DivineWhisperUI};
use crate::monitoring::{SafetyNetState, SafetyNetMonitoringUpdate, SafetyNetMonitoringSnapshot, RBEFlowAlert, RBEFlowDashboard};

#[derive(Resource)]
pub struct RbeClientSync {
    pub local_inventory: Arc<RwLock<LocalInventory>>,
    pub trade_state: Arc<RwLock<TradeUIState>>,
    pub safety_net_state: Arc<RwLock<SafetyNetState>>,
    pub rbe_flow_dashboard: Arc<RwLock<RBEFlowDashboard>>,
    last_harvest_ms: Arc<RwLock<u64>>,
}

impl RbeClientSync {
    pub fn new() -> Self {
        Self {
            local_inventory: Arc::new(RwLock::new(LocalInventory::default())),
            trade_state: Arc::new(RwLock::new(TradeUIState::default())),
            safety_net_state: Arc::new(RwLock::new(SafetyNetState::default())),
            rbe_flow_dashboard: Arc::new(RwLock::new(RBEFlowDashboard::default())),
            last_harvest_ms: Arc::new(RwLock::new(0)),
        }
    }

    // Server message handling (kept concise for this expansion)
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
                &msg, &mut inv, &mut trade,
                inventory_events, trade_events, harvest_events
            );

            if let ServerMessage::SafetyNetBroadcast { broadcast } = &msg {
                self.handle_safety_net_broadcast(broadcast, &mut monitoring_events, &mut rbe_alert_events).await;
            }
        }
    }

    async fn handle_safety_net_broadcast(
        &self,
        broadcast: &SafetyNetBroadcast,
        monitoring_events: &mut EventWriter<SafetyNetMonitoringUpdate>,
        rbe_alert_events: &mut EventWriter<RBEFlowAlert>,
    ) {
        // ... (core logic kept from previous version for brevity in this expansion)
        let mut safety = self.safety_net_state.write().await;
        let mut dashboard = self.rbe_flow_dashboard.write().await;

        let now_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64;

        safety.last_tick = broadcast.snapshot.tick;
        safety.last_health = broadcast.snapshot.current_health;
        safety.last_council_engagement = broadcast.snapshot.council_engagement_score;

        let new_abundance = broadcast.snapshot.abundance;

        if safety.last_abundance_update_ms > 0 {
            let dt_sec = (now_ms - safety.last_abundance_update_ms) as f64 / 1000.0;
            if dt_sec > 0.0 {
                let delta = new_abundance - safety.previous_abundance;
                if delta > 0.0 { safety.abundance_creation_rate = delta / dt_sec; }
            }
        }

        safety.previous_abundance = new_abundance;
        safety.last_abundance = new_abundance;
        safety.last_abundance_update_ms = now_ms;

        if let Some(event) = &broadcast.event {
            if let SafetyNetEvent::AbundanceSafetyNetTriggered { restored_amount, .. } = event {
                safety.recent_triggers.push((now_ms, *restored_amount));
                if safety.recent_triggers.len() > safety.max_trigger_history { safety.recent_triggers.remove(0); }
            }
        }

        // Basic alerts
        let creation_rate = safety.abundance_creation_rate;
        if creation_rate < 0.5 && safety.sample_count > 20 {
            let alert = RBEFlowAlert::LowAbundanceCreationRate { rate: creation_rate, threshold: 0.5 };
            rbe_alert_events.send(alert.clone());
            dashboard.add_alert(alert);
        }

        // Latency tracking (simplified for this expansion)
        let latency_ms = if broadcast.emit_timestamp_ms > 0 { now_ms.saturating_sub(broadcast.emit_timestamp_ms) } else { 0 };
        safety.last_latency_ms = latency_ms;
        safety.sample_count = safety.sample_count.saturating_add(1);

        if safety.sample_count == 1 {
            safety.ema_latency_ms = latency_ms as f32;
            safety.kalman_latency = Some(KalmanFilter1D::new(latency_ms as f32));
            safety.rts_smoother = Some(RTSFixedLagSmoother::new(8));
        } else {
            let dt_sec = 0.016;
            if let Some(k) = &mut safety.kalman_latency { k.update(latency_ms as f32, dt_sec); }
            if let Some(rts) = &mut safety.rts_smoother {
                let cov = safety.kalman_latency.as_ref().map_or(1.0, |k| k.error_estimate.max(0.1));
                rts.update(latency_ms as f32, cov, dt_sec);
            }
        }
        safety.previous_latency_ms = latency_ms;

        if safety.sample_count % 5 == 0 {
            let snapshot = SafetyNetMonitoringSnapshot {
                timestamp_ms: now_ms,
                last_latency_ms: latency_ms,
                avg_latency_ms: safety.ema_latency_ms,
                kalman_latency_residual: 0.0,
                rts_smoothed_latency: 0.0,
                rts_vs_kalman_residual: 0.0,
                server_abundance: broadcast.snapshot.abundance,
                server_health: broadcast.snapshot.current_health,
                server_council_engagement: broadcast.snapshot.council_engagement_score,
                abundance_creation_rate: safety.abundance_creation_rate,
                abundance_restoration_rate: 0.0,
                safety_net_trigger_count: safety.recent_triggers.len() as u32,
                average_restoration_magnitude: 0.0,
                restoration_effectiveness: 0.0,
            };
            monitoring_events.send(SafetyNetMonitoringUpdate { snapshot });
        }
    }

    // ============================================================
    // Advanced Harvest Logic
    // ============================================================

    /// Calculates current harvest effectiveness multiplier based on RBE + SafetyNet conditions.
    pub async fn calculate_harvest_effectiveness(&self) -> f32 {
        let dashboard = self.rbe_flow_dashboard.read().await;
        let safety = self.safety_net_state.read().await;

        let mut effectiveness = 1.0;

        // Boost from active abundance multiplier
        if dashboard.abundance_boost_active {
            effectiveness *= dashboard.restoration_multiplier.max(1.0);
        }

        // Penalty from high latency (mercy protection for unstable connections)
        if safety.ema_latency_ms > 300.0 {
            let latency_penalty = (1.0 - (safety.ema_latency_ms - 300.0) / 1000.0).max(0.6);
            effectiveness *= latency_penalty;
        }

        effectiveness.clamp(0.5, 2.0)
    }

    /// Validates and queues a harvest with full effectiveness calculation.
    pub async fn try_queue_harvest(
        &self,
        player_id: u64,
        node_id: u64,
        amount: f32,
    ) -> Option<ClientMessage> {
        let effectiveness = self.calculate_harvest_effectiveness().await;

        if effectiveness < 0.6 {
            return None; // Too unstable or poor conditions
        }

        // Apply effectiveness to amount (client-side prediction of result)
        let adjusted_amount = amount * effectiveness;

        Some(ClientMessage::Harvest { player_id, node_id, adjusted_amount })
    }

    pub async fn try_batch_harvest(
        &self,
        player_id: u64,
        harvests: Vec<(u64, f32)>,
    ) -> Vec<ClientMessage> {
        let mut messages = Vec::new();
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;

        let mut last = self.last_harvest_ms.write().await;
        if now - *last < 120 { return messages; }

        for (node_id, amount) in harvests {
            if let Some(msg) = self.try_queue_harvest(player_id, node_id, amount).await {
                messages.push(msg);
            }
        }
        *last = now;
        messages
    }

    // ============================================================
    // Deeper Prediction Layer Coupling
    // ============================================================

    /// Returns modifiers the ClientGameLoop can use to adjust prediction behavior.
    pub async fn get_prediction_modifiers(&self) -> (f32, f32) {
        let safety = self.safety_net_state.read().await;
        let dashboard = self.rbe_flow_dashboard.read().await;

        // Lower value = more conservative prediction
        let latency_factor = if safety.ema_latency_ms > 400.0 { 0.7 } else { 1.0 };
        let abundance_factor = if dashboard.abundance_creation_rate < 0.3 { 0.85 } else { 1.0 };

        (latency_factor, abundance_factor)
    }

    /// Called by ClientGameLoop when applying server corrections.
    /// Can influence local prediction model based on current RBE/SafetyNet state.
    pub async fn apply_server_correction(
        &self,
        corrected_state: &ClientState,
        server_abundance: f64,
    ) {
        let mut dashboard = self.rbe_flow_dashboard.write().await;
        dashboard.server_abundance = server_abundance;

        // Future: Use corrected_state + current modifiers to adjust prediction confidence
    }

    pub async fn get_prediction_context(&self) -> (f64, f32, bool) {
        let dashboard = self.rbe_flow_dashboard.read().await;
        let safety = self.safety_net_state.read().await;
        (dashboard.abundance_creation_rate, safety.ema_latency_ms, dashboard.abundance_boost_active)
    }

    pub async fn get_rbe_flow_health(&self) -> (f64, bool) {
        let dashboard = self.rbe_flow_dashboard.read().await;
        (dashboard.abundance_creation_rate, dashboard.abundance_boost_active)
    }

    pub async fn get_safety_net_summary(&self) -> (f32, u64) {
        let safety = self.safety_net_state.read().await;
        (safety.ema_latency_ms, safety.last_latency_ms)
    }

    pub async fn get_current_abundance_rate(&self) -> f64 {
        let dashboard = self.rbe_flow_dashboard.read().await;
        dashboard.abundance_creation_rate
    }
}

// Thunder locked in.
// rbe_client_sync.rs now has advanced harvest effectiveness and deep prediction coupling.