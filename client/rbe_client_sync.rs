// client/rbe_client_sync.rs
// Powrush-MMO — RBE + Council + Safety Net client sync layer
// Handles authoritative ServerMessage consumption including SafetyNetBroadcast (v18.37)
// Includes latency monitoring for SafetyNet broadcasts
// AG-SML v1.0 | TOLC 8 Mercy Gates enforced

use bevy::prelude::*;
use shared::protocol::{ClientMessage, ServerMessage, Vec3Ser, DivineWhisper, SafetyNetBroadcast, SafetyNetEvent};
use std::sync::Arc;
use tokio::sync::RwLock;
use bytes::Bytes;

use crate::inventory_ui::{LocalInventory, TradeUIState, InventoryUpdated, TradeResponseReceived, HarvestResponseReceived, handle_server_message};
use crate::divine_whispers_ui::{CurrentDivineWhisper, DivineWhispersLog, DivineWhisperUI, receive_divine_whisper_from_server};

#[derive(Resource, Default, Clone)]
pub struct GpuSimulationState {
    pub global_confidence: f32,
    pub node_predictions: std::collections::HashMap<u64, shared::protocol::NodeGpuPrediction>,
    pub last_update_notes: String,
}

/// Safety Net state resource for client-side sovereignty tracking + latency monitoring
#[derive(Resource, Default, Clone)]
pub struct SafetyNetState {
    pub last_tick: u64,
    pub last_abundance: f64,
    pub last_health: f32,
    pub last_council_engagement: f32,
    pub pending_events: Vec<String>,
    // Latency monitoring (ms)
    pub last_latency_ms: u64,
    pub avg_latency_ms: f32,
    pub max_latency_ms: u64,
    pub min_latency_ms: u64,
    pub sample_count: u32,
}

#[derive(Resource)]
pub struct RbeClientSync {
    pub local_inventory: Arc<RwLock<LocalInventory>>,
    pub trade_state: Arc<RwLock<TradeUIState>>,
    pub gpu_state: Arc<RwLock<GpuSimulationState>>,
    pub safety_net_state: Arc<RwLock<SafetyNetState>>,
}

impl RbeClientSync {
    pub fn new() -> Self {
        Self {
            local_inventory: Arc::new(RwLock::new(LocalInventory::default())),
            trade_state: Arc::new(RwLock::new(TradeUIState::default())),
            gpu_state: Arc::new(RwLock::new(GpuSimulationState::default())),
            safety_net_state: Arc::new(RwLock::new(SafetyNetState {
                min_latency_ms: u64::MAX,
                ..Default::default()
            })),
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
    ) {
        if let Ok(msg) = bincode::deserialize::<ServerMessage>(&data) {
            let mut inv = self.local_inventory.write().await;
            let mut trade = self.trade_state.write().await;

            handle_server_message(
                &msg,
                &mut inv,
                &mut trade,
                inventory_events,
                trade_events,
                harvest_events,
            );

            // Handle GPU simulation updates
            if let ServerMessage::GpuPatsagiUpdate { global_confidence, node_predictions, notes } = &msg {
                let mut gpu = self.gpu_state.write().await;
                gpu.global_confidence = *global_confidence;
                gpu.node_predictions = node_predictions.clone();
                gpu.last_update_notes = notes.clone();
                tracing::info!("[RbeClientSync] Received GPU PATSAGi update (confidence: {:.2})", global_confidence);
            }

            // Divine Whispers from local Ra-Thor
            if let ServerMessage::DivineWhisperReceived { whisper } = &msg {
                receive_divine_whisper_from_server(
                    whisper.clone(),
                    divine_current,
                    divine_log,
                    divine_ui_query,
                    commands,
                    asset_server,
                );
                tracing::info!("[Divine] Received whisper from server: {}", whisper.message);
            }

            // ===== SAFETY NET BROADCAST CONSUMPTION + LATENCY MONITORING (v18.37) =====
            if let ServerMessage::SafetyNetBroadcast { broadcast } = &msg {
                self.handle_safety_net_broadcast(broadcast).await;
            }
        }
    }

    async fn handle_safety_net_broadcast(&self, broadcast: &SafetyNetBroadcast) {
        let mut safety = self.safety_net_state.write().await;

        // Update authoritative snapshot
        safety.last_tick = broadcast.snapshot.tick;
        safety.last_abundance = broadcast.snapshot.abundance;
        safety.last_health = broadcast.snapshot.current_health;
        safety.last_council_engagement = broadcast.snapshot.council_engagement_score;

        // ===== LATENCY MONITORING =====
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let latency_ms = if broadcast.emit_timestamp_ms > 0 {
            now_ms.saturating_sub(broadcast.emit_timestamp_ms)
        } else {
            0
        };

        // Update running stats
        safety.last_latency_ms = latency_ms;
        safety.sample_count = safety.sample_count.saturating_add(1);

        if safety.sample_count == 1 {
            safety.avg_latency_ms = latency_ms as f32;
            safety.min_latency_ms = latency_ms;
            safety.max_latency_ms = latency_ms;
        } else {
            safety.avg_latency_ms = (safety.avg_latency_ms * (safety.sample_count - 1) as f32 + latency_ms as f32) / safety.sample_count as f32;
            if latency_ms < safety.min_latency_ms {
                safety.min_latency_ms = latency_ms;
            }
            if latency_ms > safety.max_latency_ms {
                safety.max_latency_ms = latency_ms;
            }
        }

        // Log latency (production: send to telemetry_pipeline)
        tracing::info!(
            "[SafetyNet][Latency] {}ms | reason={} | tick={} | player={}",
            latency_ms,
            broadcast.broadcast_reason,
            broadcast.snapshot.tick,
            broadcast.snapshot.player_id
        );

        if latency_ms > 150 {
            tracing::warn!("[SafetyNet] High latency detected: {}ms (threshold 150ms)", latency_ms);
        }

        // Process attached event if present
        if let Some(event) = &broadcast.event {
            match event {
                SafetyNetEvent::AbundanceSafetyNetTriggered { restored_amount, reason } => {
                    tracing::warn!("[SafetyNet] Abundance safety net triggered: +{:.2} ({}) for player {}", 
                        restored_amount, reason, broadcast.snapshot.player_id);
                    safety.pending_events.push(format!("Abundance restored: {:.2}", restored_amount));
                }
                SafetyNetEvent::CouncilStateSync { bloom_intensity, collective_attunement } => {
                    tracing::info!("[SafetyNet] Council state sync: bloom={:.2} attunement={:.2}", 
                        bloom_intensity, collective_attunement);
                }
                SafetyNetEvent::EpiphanyPersistenceConfirmed { epiphany_id, multiplier_applied } => {
                    tracing::info!("[SafetyNet] Epiphany persistence confirmed: id={} multiplier={:.2}", 
                        epiphany_id, multiplier_applied);
                }
                SafetyNetEvent::DesyncRecovery { corrected_abundance, corrected_health } => {
                    tracing::warn!("[SafetyNet] Desync recovery applied: abundance={:.2} health={:.1}", 
                        corrected_abundance, corrected_health);
                }
                SafetyNetEvent::SovereigntyHeartbeat => {
                    tracing::debug!("[SafetyNet] Sovereignty heartbeat received (tick={})", broadcast.snapshot.tick);
                }
            }
        }

        tracing::debug!(
            "[SafetyNet] Broadcast consumed | player={} | tick={} | abundance={:.2} | health={:.1} | council_score={:.2} | reason={}",
            broadcast.snapshot.player_id,
            broadcast.snapshot.tick,
            broadcast.snapshot.abundance,
            broadcast.snapshot.current_health,
            broadcast.snapshot.council_engagement_score,
            broadcast.broadcast_reason
        );

        // TODO (next cycle): Emit Bevy event for UI feedback, trigger local persistence safety write if needed,
        // update inventory abundance display, play mercy confirmation audio.
    }

    // ... other methods remain the same ...
}