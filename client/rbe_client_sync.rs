// client/rbe_client_sync.rs
// Powrush-MMO — Added handling for DivineWhisperReceived from local Ra-Thor
// Client now receives sovereign Divine Whispers and forwards to UI + log + audio
// AG-SML v1.0

use bevy::prelude::*;
use shared::protocol::{ClientMessage, ServerMessage, Vec3Ser, DivineWhisper};
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

#[derive(Resource)]
pub struct RbeClientSync {
    pub local_inventory: Arc<RwLock<LocalInventory>>,
    pub trade_state: Arc<RwLock<TradeUIState>>,
    pub gpu_state: Arc<RwLock<GpuSimulationState>>,
}

impl RbeClientSync {
    pub fn new() -> Self {
        Self {
            local_inventory: Arc::new(RwLock::new(LocalInventory::default())),
            trade_state: Arc::new(RwLock::new(TradeUIState::default())),
            gpu_state: Arc::new(RwLock::new(GpuSimulationState::default())),
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

            // === NEW: Divine Whispers from local Ra-Thor ===
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
        }
    }

    // ... other methods remain the same ...
}