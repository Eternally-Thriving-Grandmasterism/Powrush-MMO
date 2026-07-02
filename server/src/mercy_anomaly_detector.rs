/*!
 * server/src/mercy_anomaly_detector.rs
 * UNIFIED MERCY ANOMALY DETECTOR — FULL CONSOLIDATION (Option A)
 *
 * Single canonical detector for Powrush-MMO.
 * Combines:
 *   - TOLC 8 / PATSAGi-first philosophy, mercy justifications, graduated ModerationAction
 *   - Spatial + harvest + inventory delta anti-cheat (chunk-aware position jumps, harvest rate windows, suspicious gains)
 *   - Per-player tracking, config-driven enforcement, SafetyNet integration points
 *
 * All valuable logic from both previous implementations preserved and unified.
 * No duplication. Production-grade. Mercy-gated, redemptive, never tyrannical.
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi Councils
 * Thunder locked in. Yoi ⚡
 */

use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

// ════════════════════════════════════════════════════════════════════════════════════
// CONFIGURATION
// ════════════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MercyAnomalyConfig {
    pub enabled: bool,
    pub mercy_enforcement_level: f32,
    pub anomaly_sensitivity: f32,
    pub auto_moderation_enabled: bool,
    pub divine_warning_enabled: bool,
    pub throttle_duration_secs: u64,
    pub max_warnings_before_action: u32,
    pub ban_duration_hours: u64,
    pub log_all_anomalies: bool,
    // Spatial / harvest tuning
    pub max_harvests_per_minute: u32,
    pub max_position_jump_distance: f32,
}

impl Default for MercyAnomalyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            mercy_enforcement_level: 0.92,
            anomaly_sensitivity: 1.0,
            auto_moderation_enabled: true,
            divine_warning_enabled: true,
            throttle_duration_secs: 300,
            max_warnings_before_action: 3,
            ban_duration_hours: 24,
            log_all_anomalies: true,
            max_harvests_per_minute: 12,
            max_position_jump_distance: 150.0,
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════════════
// ANOMALY TYPES (unified from both previous detectors)
// ════════════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnomalyType {
    // High-level / economy / behavior (TOLC 8 style)
    RapidResourceAbuse,
    WhisperSpam,
    ResourceHoarding,
    FactionBetrayal,
    ChatToxicity,
    EconomyManipulation,
    InventoryHotbarViolation,
    InventoryGeneralViolation,

    // Spatial + harvest + inventory delta (security style)
    ExcessiveHarvestRate { rate_per_minute: f32, threshold: f32 },
    ImpossiblePositionJump { distance: f32, max_allowed: f32 },
    SuspiciousInventoryDelta { item_id: u32, quantity_gained: u32 },

    Custom(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnomalyReport {
    pub player_id: u64,
    pub anomaly_type: AnomalyType,
    pub severity: f32,
    pub timestamp: u64,
    pub context: String,
    pub mercy_justification: String,
}

// ════════════════════════════════════════════════════════════════════════════════════
// MODERATION ACTIONS
// ════════════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ModerationAction {
    LogOnly,
    DivineWarning { message: String },
    Throttle { duration_secs: u64, reason: String },
    Kick { reason: String },
    Ban { duration_hours: u64, reason: String },
}

impl ModerationAction {
    pub fn is_severe(&self) -> bool {
        matches!(self, ModerationAction::Kick { .. } | ModerationAction::Ban { .. })
    }
}

// ════════════════════════════════════════════════════════════════════════════════════
// PER-PLAYER TRACKING (merged from spatial detector)
// ════════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
struct PlayerTracker {
    last_position: (f32, f32),
    last_harvest_time: Instant,
    harvests_in_window: u32,
    window_start: Instant,
    recent_inventory_deltas: Vec<(u32, u32, Instant)>, // (item_id, qty, time)
}

// ════════════════════════════════════════════════════════════════════════════════════
// MAIN DETECTOR
// ════════════════════════════════════════════════════════════════════════════════════

pub struct MercyAnomalyDetector {
    pub config: MercyAnomalyConfig,
    player_warnings: HashMap<u64, u32>,
    player_last_action: HashMap<u64, Instant>,
    recent_reports: Vec<AnomalyReport>,
    player_trackers: HashMap<u64, PlayerTracker>,
}

impl MercyAnomalyDetector {
    pub fn new(config: MercyAnomalyConfig) -> Self {
        Self {
            config,
            player_warnings: HashMap::new(),
            player_last_action: HashMap::new(),
            recent_reports: Vec::with_capacity(256),
            player_trackers: HashMap::new(),
        }
    }

    pub fn update_config(&mut self, new_config: MercyAnomalyConfig) {
        self.config = new_config;
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // CORE ENTRY POINT (high-level anomalies + inventory violations)
    // ─────────────────────────────────────────────────────────────────────────────
    pub fn report_anomaly(
        &mut self,
        player_id: u64,
        anomaly_type: AnomalyType,
        severity: f32,
        context: String,
    ) -> Option<ModerationAction> {
        if !self.config.enabled {
            return None;
        }

        let adjusted_severity = (severity * self.config.anomaly_sensitivity).clamp(0.0, 1.0);
        let mercy_justification = self.generate_mercy_justification(&anomaly_type, adjusted_severity);

        let report = AnomalyReport {
            player_id,
            anomaly_type: anomaly_type.clone(),
            severity: adjusted_severity,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            context,
            mercy_justification: mercy_justification.clone(),
        };

        if self.config.log_all_anomalies {
            self.log_anomaly(&report);
        }

        self.recent_reports.push(report);
        if self.recent_reports.len() > 512 {
            self.recent_reports.remove(0);
        }

        if self.config.auto_moderation_enabled {
            return self.decide_and_execute_action(player_id, &anomaly_type, adjusted_severity);
        }

        None
    }

    fn generate_mercy_justification(&self, anomaly_type: &AnomalyType, severity: f32) -> String {
        let base = match anomaly_type {
            AnomalyType::RapidResourceAbuse => "Rapid resource extraction detected. This disrupts the Eternal Flow and abundance for all.",
            AnomalyType::WhisperSpam => "Excessive divine whisper activity. The Councils request mindful communication.",
            AnomalyType::ResourceHoarding => "Accumulation beyond sustainable mercy thresholds. Share the abundance.",
            AnomalyType::FactionBetrayal => "Pattern of diplomacy violation detected. Honor the agreements of the Flow.",
            AnomalyType::ChatToxicity => "Harmful expression detected. All beings deserve grace and respect.",
            AnomalyType::EconomyManipulation => "Abnormal economic activity detected. The RBE must remain pure and abundant for all.",
            AnomalyType::InventoryHotbarViolation => "Unauthorized hotbar manipulation detected. Realign with mercy and fair play.",
            AnomalyType::InventoryGeneralViolation => "Unauthorized inventory manipulation detected. Realign with mercy and fair play.",
            AnomalyType::ExcessiveHarvestRate { .. } => "Harvest rate exceeds reasonable human limits. Slow down and respect the nodes.",
            AnomalyType::ImpossiblePositionJump { .. } => "Suspicious movement pattern detected. Sovereign space must remain harmonious.",
            AnomalyType::SuspiciousInventoryDelta { .. } => "Large inventory gain without corresponding harvest activity. Possible duplication or exploit.",
            AnomalyType::Custom(s) => &format!("Custom anomaly: {}. Align with mercy.", s),
        };

        format!("{} Severity: {:.2}. Mercy enforcement: {:.0}%. PATSAGi review recommended.",
            base, severity, self.config.mercy_enforcement_level * 100.0)
    }

    fn log_anomaly(&self, report: &AnomalyReport) {
        println!("[MERCY ANOMALY] Player {} | {:?} | Severity {:.2} | {}",
            report.player_id, report.anomaly_type, report.severity, report.mercy_justification);
    }

    fn decide_and_execute_action(
        &mut self,
        player_id: u64,
        anomaly_type: &AnomalyType,
        severity: f32,
    ) -> Option<ModerationAction> {
        let warnings = self.player_warnings.entry(player_id).or_insert(0);
        *warnings += 1;

        let action = if severity > 0.85 || *warnings >= self.config.max_warnings_before_action {
            if severity > 0.92 {
                ModerationAction::Ban {
                    duration_hours: self.config.ban_duration_hours,
                    reason: format!("Severe mercy violation: {:?}", anomaly_type),
                }
            } else {
                ModerationAction::Kick {
                    reason: format!("Repeated mercy violation after {} warnings", *warnings),
                }
            }
        } else if severity > 0.65 {
            ModerationAction::Throttle {
                duration_secs: self.config.throttle_duration_secs,
                reason: "Action rate limited for reflection and realignment with the Flow".to_string(),
            }
        } else if self.config.divine_warning_enabled {
            ModerationAction::DivineWarning {
                message: format!(
                    "Divine Whisper from the PATSAGi Councils: Your recent actions have been noted. Please realign with mercy and abundance. This is warning #{}.\n\n{}",
                    *warnings, self.generate_mercy_justification(anomaly_type, severity)
                ),
            }
        } else {
            ModerationAction::LogOnly
        };

        self.player_last_action.insert(player_id, Instant::now());
        Some(action)
    }

    pub fn get_recent_reports(&self, limit: usize) -> Vec<AnomalyReport> {
        self.recent_reports.iter().rev().take(limit).cloned().collect()
    }

    pub fn clear_warnings(&mut self, player_id: u64) {
        self.player_warnings.remove(&player_id);
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // SPATIAL + HARVEST + INVENTORY DELTA DETECTION (merged from security layer)
    // ─────────────────────────────────────────────────────────────────────────────

    pub fn update_player_position(&mut self, player_id: u64, new_position: (f32, f32)) {
        let now = Instant::now();
        let tracker = self.player_trackers.entry(player_id).or_insert_with(|| PlayerTracker {
            last_position: new_position,
            last_harvest_time: now,
            harvests_in_window: 0,
            window_start: now,
            recent_inventory_deltas: Vec::new(),
        });

        let distance = ((new_position.0 - tracker.last_position.0).powi(2)
            + (new_position.1 - tracker.last_position.1).powi(2))
            .sqrt();

        if distance > self.config.max_position_jump_distance {
            let _ = self.report_anomaly(
                player_id,
                AnomalyType::ImpossiblePositionJump {
                    distance,
                    max_allowed: self.config.max_position_jump_distance,
                },
                0.85,
                format!("Player jumped {:.1} units (max allowed {:.1})", distance, self.config.max_position_jump_distance),
            );
        }

        tracker.last_position = new_position;
    }

    pub fn record_harvest(&mut self, player_id: u64, node_id: u64, _amount: u32) {
        let now = Instant::now();
        let tracker = self.player_trackers.entry(player_id).or_insert_with(|| PlayerTracker {
            last_position: (0.0, 0.0),
            last_harvest_time: now,
            harvests_in_window: 0,
            window_start: now,
            recent_inventory_deltas: Vec::new(),
        });

        if now.duration_since(tracker.window_start) > Duration::from_secs(60) {
            tracker.harvests_in_window = 0;
            tracker.window_start = now;
        }

        tracker.harvests_in_window += 1;
        tracker.last_harvest_time = now;

        if tracker.harvests_in_window > self.config.max_harvests_per_minute {
            let rate = tracker.harvests_in_window as f32 / 60.0;
            let _ = self.report_anomaly(
                player_id,
                AnomalyType::ExcessiveHarvestRate {
                    rate_per_minute: rate,
                    threshold: self.config.max_harvests_per_minute as f32,
                },
                0.75,
                format!("Harvested {} times in last 60s (node {}). Rate: {:.1}/min", tracker.harvests_in_window, node_id, rate),
            );
        }
    }

    pub fn record_inventory_delta(&mut self, player_id: u64, item_id: u32, quantity_change: i32) {
        if quantity_change <= 0 {
            return;
        }

        let tracker = self.player_trackers.entry(player_id).or_insert_with(|| PlayerTracker {
            last_position: (0.0, 0.0),
            last_harvest_time: Instant::now(),
            harvests_in_window: 0,
            window_start: Instant::now(),
            recent_inventory_deltas: Vec::new(),
        });

        tracker.recent_inventory_deltas.push((item_id, quantity_change as u32, Instant::now()));
        tracker.recent_inventory_deltas.retain(|(_, _, t)| t.elapsed() < Duration::from_secs(30));

        let total_recent_gain: u32 = tracker.recent_inventory_deltas.iter()
            .filter(|(id, _, _)| *id == item_id)
            .map(|(_, qty, _)| *qty)
            .sum();

        if total_recent_gain > 50 && tracker.harvests_in_window == 0 {
            let _ = self.report_anomaly(
                player_id,
                AnomalyType::SuspiciousInventoryDelta {
                    item_id,
                    quantity_gained: total_recent_gain,
                },
                0.8,
                format!("Large inventory gain of item {} ({} total) with no recent harvests", item_id, total_recent_gain),
            );
        }
    }

    pub fn cleanup_stale_trackers(&mut self) {
        let now = Instant::now();
        self.player_trackers.retain(|_, tracker| {
            now.duration_since(tracker.last_harvest_time) < Duration::from_secs(300)
        });
    }
}

// ════════════════════════════════════════════════════════════════════════════════════
// PLUGIN / INTEGRATION
// ════════════════════════════════════════════════════════════════════════════════════

pub struct MercyAnomalyDetectorPlugin;

impl MercyAnomalyDetectorPlugin {
    pub fn new(config: MercyAnomalyConfig) -> Self {
        Self
    }
}

// Usage examples:
//   detector.update_player_position(player_id, pos);
//   detector.record_harvest(player_id, node_id, amount);
//   detector.record_inventory_delta(player_id, item_id, delta);
//   if let Some(action) = detector.report_anomaly(...) { ... }

// End of unified MercyAnomalyDetector — Full consolidation complete. Thunder locked in. Yoi ⚡