// client/rbe_client_sync.rs
// Powrush-MMO — RBE + Council + Safety Net client sync layer
// Handles authoritative ServerMessage consumption including SafetyNetBroadcast (v18.37)
// Includes latency monitoring + histograms + jitter analysis for SafetyNet broadcasts
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

/// Simple but effective fixed-bucket latency histogram for SafetyNet monitoring.
/// Buckets chosen for typical MMO/game network latency distribution.
#[derive(Clone, Debug, Default)]
pub struct LatencyHistogram {
    /// Buckets: [0-10, 10-25, 25-50, 50-100, 100-200, 200-500, 500-1000, >1000] ms
    pub buckets: [u32; 8],
    pub total_samples: u32,
}

impl LatencyHistogram {
    pub fn new() -> Self {
        Self {
            buckets: [0; 8],
            total_samples: 0,
        }
    }

    /// Record a latency sample (in milliseconds)
    pub fn record(&mut self, latency_ms: u64) {
        self.total_samples = self.total_samples.saturating_add(1);

        let bucket_index = match latency_ms {
            0..=10 => 0,
            11..=25 => 1,
            26..=50 => 2,
            51..=100 => 3,
            101..=200 => 4,
            201..=500 => 5,
            501..=1000 => 6,
            _ => 7,
        };

        self.buckets[bucket_index] = self.buckets[bucket_index].saturating_add(1);
    }

    /// Approximate percentile (0.0 - 1.0). Returns latency value for that percentile.
    pub fn percentile(&self, p: f32) -> u64 {
        if self.total_samples == 0 {
            return 0;
        }

        let target = (self.total_samples as f32 * p.clamp(0.0, 1.0)) as u32;
        let mut cumulative = 0u32;

        let bucket_edges = [10u64, 25, 50, 100, 200, 500, 1000, u64::MAX];

        for (i, &count) in self.buckets.iter().enumerate() {
            cumulative += count;
            if cumulative >= target {
                return bucket_edges[i];
            }
        }

        bucket_edges.last().copied().unwrap_or(0)
    }

    pub fn p50(&self) -> u64 { self.percentile(0.5) }
    pub fn p95(&self) -> u64 { self.percentile(0.95) }
    pub fn p99(&self) -> u64 { self.percentile(0.99) }
}

/// Safety Net state resource for client-side sovereignty tracking + advanced latency + jitter monitoring
#[derive(Resource, Clone)]
pub struct SafetyNetState {
    pub last_tick: u64,
    pub last_abundance: f64,
    pub last_health: f32,
    pub last_council_engagement: f32,
    pub pending_events: Vec<String>,

    // Basic latency stats
    pub last_latency_ms: u64,
    pub avg_latency_ms: f32,
    pub max_latency_ms: u64,
    pub min_latency_ms: u64,
    pub sample_count: u32,

    // Histogram for distribution analysis (p50/p95/p99)
    pub latency_histogram: LatencyHistogram,

    // Jitter analysis (variation in latency)
    pub last_jitter_ms: u64,
    pub avg_jitter_ms: f32,
    pub max_jitter_ms: u64,
    previous_latency_ms: u64,   // internal tracking only
}

impl Default for SafetyNetState {
    fn default() -> Self {
        Self {
            last_tick: 0,
            last_abundance: 0.0,
            last_health: 100.0,
            last_council_engagement: 0.0,
            pending_events: Vec::new(),
            last_latency_ms: 0,
            avg_latency_ms: 0.0,
            max_latency_ms: 0,
            min_latency_ms: u64::MAX,
            sample_count: 0,
            latency_histogram: LatencyHistogram::new(),
            last_jitter_ms: 0,
            avg_jitter_ms: 0.0,
            max_jitter_ms: 0,
            previous_latency_ms: 0,
        }
    }
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

            // ===== SAFETY NET BROADCAST CONSUMPTION + LATENCY + JITTER (v18.37) =====
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

        // ===== LATENCY + JITTER ANALYSIS =====
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let latency_ms = if broadcast.emit_timestamp_ms > 0 {
            now_ms.saturating_sub(broadcast.emit_timestamp_ms)
        } else {
            0
        };

        // Calculate jitter (variation from previous sample)
        let jitter_ms = if safety.previous_latency_ms > 0 {
            (latency_ms as i64 - safety.previous_latency_ms as i64).unsigned_abs()
        } else {
            0
        };

        // Update basic latency stats
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

        // Update jitter stats
        safety.last_jitter_ms = jitter_ms;
        if safety.sample_count > 1 {
            if safety.sample_count == 2 {
                safety.avg_jitter_ms = jitter_ms as f32;
                safety.max_jitter_ms = jitter_ms;
            } else {
                safety.avg_jitter_ms = (safety.avg_jitter_ms * (safety.sample_count - 2) as f32 + jitter_ms as f32) / (safety.sample_count - 1) as f32;
                if jitter_ms > safety.max_jitter_ms {
                    safety.max_jitter_ms = jitter_ms;
                }
            }
        }

        // Store for next jitter calculation
        safety.previous_latency_ms = latency_ms;

        // Record into histogram for distribution analysis
        safety.latency_histogram.record(latency_ms);

        // Log latency + jitter + key percentiles
        tracing::info!(
            "[SafetyNet][Latency] {}ms | jitter={}ms | p50={} p95={} p99={} | reason={} | tick={}",
            latency_ms,
            jitter_ms,
            safety.latency_histogram.p50(),
            safety.latency_histogram.p95(),
            safety.latency_histogram.p99(),
            broadcast.broadcast_reason,
            broadcast.snapshot.tick
        );

        if latency_ms > 150 || jitter_ms > 50 {
            tracing::warn!(
                "[SafetyNet] High latency/jitter detected: {}ms latency, {}ms jitter (p95={})",
                latency_ms, jitter_ms, safety.latency_histogram.p95()
            );
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
        // Future: periodically export histogram + jitter snapshot to telemetry_pipeline
    }

    // ... other methods remain the same ...
}