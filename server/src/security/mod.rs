// server/src/security/mod.rs
// Powrush-MMO v17.7 — Security Module (Mercy-Gated Anomaly Detection & Anti-Cheat)
// Part of Eternal Professional Workflow. 100% preservation of v17.6 foundational work.
// PATSAGi + Ra-Thor + 7 Living Mercy Gates aligned. RBE economy protector.

pub mod mercy_anomaly_detector;

// Re-exports for convenient use across server systems
pub use mercy_anomaly_detector::{
    MercyAnomalyDetector, MercySeverity, AnomalyType, AnomalyRecord,
    MERCY_ANOMALY_DETECTOR_VERSION,
};

/// Quick integration example (call from world tick or system init):
/// ```rust
/// let mut anomaly_detector = MercyAnomalyDetector::new();
/// anomaly_detector.set_chunk_manager(chunk_manager);
/// anomaly_detector.set_persistence(persistence_manager);
/// anomaly_detector.set_interest_manager(interest_manager);
/// 
/// // In player movement handler:
/// anomaly_detector.update_player_position(player_id, new_pos);
/// 
/// // In harvest success (authoritative):
/// anomaly_detector.record_harvest(player_id, node_id, amount);
/// 
/// // In inventory authoritative change:
/// anomaly_detector.record_inventory_delta(player_id, item_id, delta);
/// 
/// // Periodic:
/// anomaly_detector.cleanup_stale_trackers();
/// let recent = anomaly_detector.get_recent_anomalies();
/// ```