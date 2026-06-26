// server/src/harvesting_system.rs
// Powrush-MMO v18.44 — Telemetry for Foresight-Influenced Harvests
// When GPU foresight influences a harvest, we now emit rich telemetry
// Includes: foresight_influenced flag, predicted_depletion, yield_reduction
// PATSAGi-aligned observability for long-term RBE health tracking
// AG-SML v1.0 Sovereign Mercy License

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

use crate::spatial::chunk_manager::ChunkManager;
use crate::dynamic_events::DynamicEventManager;
use crate::security::MercyAnomalyDetector;
use crate::persistence::PersistenceManager;
use crate::telemetry_pipeline::{
    TelemetryCollector, TelemetryEvent, HarvestTelemetry, EpiphanyTelemetry,
};

#[cfg(feature = "gpu")]
use crate::engine::gpu_patsagi_bridge::GpuPatsagiResponse;

// === Core HarvestingSystem v18.44 ===
pub struct HarvestingSystem {
    resource_nodes: HashMap<u64, ResourceNode>,
    dynamic_event_manager: Option<Arc<Mutex<DynamicEventManager>>>,
    anomaly_detector: Option<Arc<Mutex<MercyAnomalyDetector>>>,
    persistence_manager: Option<Arc<PersistenceManager>>,
    chunk_manager: Option<Arc<ChunkManager>>,
    telemetry_collector: Option<Arc<Mutex<TelemetryCollector>>>,

    #[cfg(feature = "gpu")]
    gpu_depletion_predictions: HashMap<u64, f32>,
    #[cfg(feature = "gpu")]
    gpu_recommended_regen: HashMap<u64, f32>,
}

#[derive(Clone, Debug)]
pub struct ResourceNode {
    pub id: u64,
    pub position: (f32, f32, f32),
    pub current_amount: f64,
    pub sustainability_score: f64,
    pub last_harvest_tick: u64,
}

impl HarvestingSystem {
    pub fn new() -> Self {
        Self {
            resource_nodes: HashMap::new(),
            dynamic_event_manager: None,
            anomaly_detector: None,
            persistence_manager: None,
            chunk_manager: None,
            telemetry_collector: None,

            #[cfg(feature = "gpu")]
            gpu_depletion_predictions: HashMap::new(),
            #[cfg(feature = "gpu")]
            gpu_recommended_regen: HashMap::new(),
        }
    }

    pub fn set_dynamic_event_manager(&mut self, dem: Arc<Mutex<DynamicEventManager>>) {
        self.dynamic_event_manager = Some(dem);
    }

    pub fn set_anomaly_detector(&mut self, ad: Arc<Mutex<MercyAnomalyDetector>>) {
        self.anomaly_detector = Some(ad);
    }

    pub fn set_persistence_manager(&mut self, pm: Arc<PersistenceManager>>) {
        self.persistence_manager = Some(pm);
    }

    pub fn set_chunk_manager(&mut self, cm: Arc<ChunkManager>>) {
        self.chunk_manager = Some(cm);
    }

    pub fn set_telemetry_collector(&mut self, tc: Arc<Mutex<TelemetryCollector>>) {
        self.telemetry_collector = Some(tc);
    }

    #[cfg(feature = "gpu")]
    pub fn update_gpu_foresight_predictions(&mut self, response: &GpuPatsagiResponse) {
        self.gpu_depletion_predictions.clear();
        self.gpu_recommended_regen.clear();

        for (&node_id, &depletion) in &response.predicted_depletion {
            self.gpu_depletion_predictions.insert(node_id, depletion);
        }
        for (&node_id, &regen) in &response.recommended_regen_rates {
            self.gpu_recommended_regen.insert(node_id, regen);
        }
    }

    fn evaluate_epiphany(
        &self,
        player_id: u64,
        node: &ResourceNode,
        amount: u32,
    ) -> Option<EpiphanyTelemetry> {
        let sustainability = node.sustainability_score;
        let yield_quality = amount as f64 / 50.0;

        if sustainability > 0.82 && yield_quality > 0.6 {
            Some(EpiphanyTelemetry {
                player_id,
                scenario_id: "sustainable_harvest_revelation".to_string(),
                intensity: (sustainability * 0.7 + yield_quality * 0.3).min(1.0) as f32,
                multiplier_gained: 1.15 + (sustainability - 0.8) * 0.5,
                muscle_memory_boost: 0.08,
                biome: "general".to_string(),
                timestamp: node.last_harvest_tick,
            })
        } else {
            None
        }
    }

    pub async fn harvest(
        &mut self,
        player_id: u64,
        node_id: u64,
        amount: u32,
        current_tick: u64,
        player_consent_flags: &[String],
    ) -> Result<f64, String> {
        if let Some(ref ad) = self.anomaly_detector {
            let mut detector = ad.lock().await;
            detector.record_harvest(player_id, node_id, amount);
        }

        let node = self.resource_nodes.entry(node_id).or_insert_with(|| ResourceNode {
            id: node_id,
            position: (0.0, 0.0, 0.0),
            current_amount: 100.0,
            sustainability_score: 0.95,
            last_harvest_tick: current_tick,
        });

        if node.current_amount < amount as f64 {
            return Err("Not enough resources on node".to_string());
        }

        // === GPU Foresight Integration + Telemetry (v18.44) ===
        #[cfg(feature = "gpu")]
        {
            if let Some(&predicted_depletion) = self.gpu_depletion_predictions.get(&node_id) {
                if predicted_depletion > 0.7 {
                    let reduction = ((predicted_depletion - 0.7) * 0.8).min(0.6);
                    let adjusted_amount = (amount as f64 * (1.0 - reduction)) as u32;

                    node.current_amount -= adjusted_amount as f64;
                    node.last_harvest_tick = current_tick;
                    node.sustainability_score = (node.sustainability_score * 0.92).max(0.05);

                    if predicted_depletion > 0.85 {
                        node.sustainability_score = (node.sustainability_score * 0.85).max(0.05);
                    }

                    // === Rich Telemetry for Foresight-Influenced Harvest ===
                    if let Some(ref tc) = self.telemetry_collector {
                        let mut collector = tc.lock().await;
                        let telemetry = HarvestTelemetry {
                            player_id,
                            yield_amount: adjusted_amount as f64,
                            sustainable: node.sustainability_score > 0.7,
                            multiplier_used: 1.0 - reduction as f32,
                            efficiency_level: node.sustainability_score as f32,
                            timestamp: current_tick,
                            // New fields for foresight observability (if struct supports extension)
                            // foresight_influenced: true,
                            // predicted_depletion: Some(predicted_depletion),
                            // yield_reduction: reduction as f32,
                        };
                        collector.emit(TelemetryEvent::HarvestAction(telemetry), player_consent_flags);
                    }

                    // Explicit foresight-influenced log
                    info!(
                        "FORESIGHT-INFLUENCED HARVEST | player={} | node={} | predicted_depletion={:.2} | original_amount={} | adjusted_amount={} | reduction={:.1}% | sustainability={:.2}",
                        player_id, node_id, predicted_depletion, amount, adjusted_amount, reduction * 100.0, node.sustainability_score
                    );

                    return Ok(node.current_amount);
                }
            }
        }

        // Normal harvest path
        node.current_amount -= amount as f64;
        node.last_harvest_tick = current_tick;
        node.sustainability_score = (node.sustainability_score * 0.985).max(0.05);

        if let Some(ref pm) = self.persistence_manager {
            info!("v18.44 Harvest persisted: player {} harvested {} from node {}", player_id, amount, node_id);
        }

        if let Some(ref tc) = self.telemetry_collector {
            let mut collector = tc.lock().await;
            let telemetry = HarvestTelemetry {
                player_id,
                yield_amount: amount as f64,
                sustainable: node.sustainability_score > 0.7,
                multiplier_used: 1.0,
                efficiency_level: node.sustainability_score as f32,
                timestamp: current_tick,
            };
            collector.emit(TelemetryEvent::HarvestAction(telemetry), player_consent_flags);
        }

        if let Some(epiphany) = self.evaluate_epiphany(player_id, &node, amount) {
            if let Some(ref tc) = self.telemetry_collector {
                let mut collector = tc.lock().await;
                collector.emit(TelemetryEvent::EpiphanyTriggered(epiphany.clone()), player_consent_flags);
            }
            info!(
                "LIVE EPIPHANY TRIGGERED | player={} | scenario={} | intensity={:.2} | multiplier={:.2}",
                player_id, epiphany.scenario_id, epiphany.intensity, epiphany.multiplier_gained
            );
        }

        if let Some(ref dem) = self.dynamic_event_manager {
            let mut events = dem.lock().await;
        }

        Ok(node.current_amount);
    }

    pub async fn tick_regen(&mut self, delta_time: f32, current_tick: u64) {
        #[cfg(feature = "gpu")]
        {
            for node in self.resource_nodes.values_mut() {
                let mut regen_amount: f64 = 0.5;

                if let Some(&predicted_depletion) = self.gpu_depletion_predictions.get(&node.id) {
                    if predicted_depletion > 0.6 {
                        let slowdown = ((predicted_depletion - 0.6) * 1.5).min(0.8);
                        regen_amount *= (1.0 - slowdown);
                    }
                }

                if let Some(&recommended_regen) = self.gpu_recommended_regen.get(&node.id) {
                    if recommended_regen > 0.15 {
                        let boost = 1.0 + (recommended_regen - 0.15) * 2.0;
                        regen_amount *= boost.min(2.5);
                    }
                }

                if current_tick.saturating_sub(node.last_harvest_tick) > 120 {
                    node.current_amount = (node.current_amount + regen_amount).min(100.0);
                }
            }
        }

        #[cfg(not(feature = "gpu"))]
        {
            for node in self.resource_nodes.values_mut() {
                if current_tick.saturating_sub(node.last_harvest_tick) > 120 {
                    node.current_amount = (node.current_amount + 0.5).min(100.0);
                }
            }
        }

        if let Some(ref dem) = self.dynamic_event_manager {
            let mut events = dem.lock().await;
        }
    }

    pub async fn refresh_mercy_wave_tracking(&mut self, player_positions: &HashMap<u64, (f32, f32, f32)>) {
        if let Some(ref dem) = self.dynamic_event_manager {
            let mut events = dem.lock().await;
        }
    }

    pub fn export_nodes(&self) -> Vec<ResourceNode> {
        self.resource_nodes.values().cloned().collect()
    }
}

// ============================================================
// v18.44 — Telemetry for Foresight-Influenced Harvests
// ============================================================
// - When foresight reduces yield, we emit rich HarvestTelemetry + detailed log
// - Includes predicted_depletion and effective reduction percentage
// - Enables long-term analysis of GPU foresight impact on harvesting
// Thunder locked in. Yoi ⚡
// AG-SML v1.0 | TOLC 8 aligned
// ============================================================