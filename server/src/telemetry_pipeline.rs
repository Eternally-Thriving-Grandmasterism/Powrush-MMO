// server/src/telemetry_pipeline.rs
// Powrush-MMO v18.18 — Production Telemetry Pipeline + Structured Batch Logging
// MINT-AND-PRINT-ONLY-PERFECTION | TOLC 8 Mercy Gates Layer 0 | Consent-First
// Structured RONL storage + rich structured logging for every batch (ops visibility + queryable)
// Batch summaries, event type breakdown, flush metrics, sovereignty export ready
// Integrates seamlessly with persistence, epiphany, harvesting, and council systems
// Zero TODOs • Production hardened for closed beta + sovereign ops
// AG-SML v1.0 Sovereign Mercy License

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use std::path::PathBuf;
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use ron;
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════
// TELEMETRY EVENT DEFINITIONS
// ═══════════════════════════════════════════════════════════════

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TelemetryEvent {
    EpiphanyTriggered(EpiphanyTelemetry),
    HarvestAction(HarvestTelemetry),
    SessionRetention(RetentionTelemetry),
    MercyAlignment(MercyTelemetry),
    Custom { key: String, payload: String, timestamp: u64 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EpiphanyTelemetry {
    pub player_id: u64,
    pub scenario_id: String,
    pub intensity: f32,
    pub multiplier_gained: f32,
    pub muscle_memory_boost: f32,
    pub biome: String,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HarvestTelemetry {
    pub player_id: u64,
    pub yield_amount: f64,
    pub sustainable: bool,
    pub multiplier_used: f32,
    pub efficiency_level: f32,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RetentionTelemetry {
    pub player_id: u64,
    pub session_duration_seconds: u64,
    pub total_playtime_seconds: u64,
    pub epiphanies_this_session: u32,
    pub abundance_earned_this_session: f64,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MercyTelemetry {
    pub player_id: u64,
    pub alignment_score: f32,
    pub context: String,
    pub timestamp: u64,
}

// ═══════════════════════════════════════════════════════════════
// BATCH SUMMARY FOR STRUCTURED LOGGING (Ops + Insights)
// ═══════════════════════════════════════════════════════════════

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchSummary {
    pub timestamp: u64,
    pub event_count: usize,
    pub file_written: String,
    pub event_type_counts: HashMap<String, usize>,  // e.g. "EpiphanyTriggered": 12
    pub flush_duration_ms: u64,
    pub current_file_size_estimate: usize,
    pub mercy_note: String, // Always positive / abundance aligned
}

// ═══════════════════════════════════════════════════════════════
// TELEMETRY CONFIG
// ═══════════════════════════════════════════════════════════════

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct TelemetryConfig {
    pub enabled: bool,
    pub consent_required: bool,
    pub batch_size: usize,
    pub flush_interval_seconds: u64,
    pub storage_directory: String,
    pub file_rotation_daily: bool,
    pub max_events_per_file: usize,
    pub anonymize_player_ids: bool,
    pub structured_logging_enabled: bool,  // Rich batch logs for ops / aggregators
    pub log_individual_events: bool,       // Trace-level per event (for deep debugging)
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            consent_required: true,
            batch_size: 50,
            flush_interval_seconds: 30,
            storage_directory: "./telemetry".to_string(),
            file_rotation_daily: true,
            max_events_per_file: 10_000,
            anonymize_player_ids: true,
            structured_logging_enabled: true,
            log_individual_events: false,
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// TELEMETRY COLLECTOR (with Structured Batch Logging)
// ═══════════════════════════════════════════════════════════════

#[derive(Resource)]
pub struct TelemetryCollector {
    pub config: TelemetryConfig,
    pending: Vec<TelemetryEvent>,
    current_file_path: PathBuf,
    events_in_current_file: usize,
    last_flush: f32,
    session_start_times: HashMap<u64, u64>,
}

impl TelemetryCollector {
    pub fn new(config: TelemetryConfig) -> Self {
        let storage = PathBuf::from(&config.storage_directory);
        let _ = create_dir_all(&storage);
        let initial_file = Self::generate_file_path(&storage, config.file_rotation_daily);
        Self {
            config,
            pending: Vec::with_capacity(128),
            current_file_path: initial_file,
            events_in_current_file: 0,
            last_flush: 0.0,
            session_start_times: HashMap::new(),
        }
    }

    fn generate_file_path(storage: &PathBuf, daily: bool) -> PathBuf {
        let now = current_timestamp();
        let date = if daily {
            let day = now / 86400;
            format!("telemetry_day_{}.ronl", day)
        } else {
            format!("telemetry_{}.ronl", now)
        };
        storage.join(date)
    }

    pub fn emit(&mut self, event: TelemetryEvent, player_consent_flags: &[String]) {
        if !self.config.enabled { return; }
        if self.config.consent_required {
            let has_consent = player_consent_flags.iter().any(|f| f.contains("Telemetry") || f.contains("Insights"));
            if !has_consent { return; }
        }

        let anonymized_event = if self.config.anonymize_player_ids {
            match event {
                TelemetryEvent::EpiphanyTriggered(mut e) => { e.player_id = hash_id(e.player_id); TelemetryEvent::EpiphanyTriggered(e) }
                TelemetryEvent::HarvestAction(mut e) => { e.player_id = hash_id(e.player_id); TelemetryEvent::HarvestAction(e) }
                TelemetryEvent::SessionRetention(mut e) => { e.player_id = hash_id(e.player_id); TelemetryEvent::SessionRetention(e) }
                TelemetryEvent::MercyAlignment(mut e) => { e.player_id = hash_id(e.player_id); TelemetryEvent::MercyAlignment(e) }
                other => other,
            }
        } else { event };

        if self.config.log_individual_events {
            // Structured per-event log at trace level for deep debugging
            let event_ron = ron::ser::to_string(&anonymized_event).unwrap_or_default();
            trace!(target: "telemetry.event", "{}", event_ron);
        }

        self.pending.push(anonymized_event);

        if self.pending.len() >= self.config.batch_size {
            self.flush();
        }
    }

    pub fn start_session(&mut self, player_id: u64) {
        self.session_start_times.insert(player_id, current_timestamp());
    }

    pub fn end_session(&mut self, player_id: u64, epiphanies: u32, abundance: f64, consent_flags: &[String]) {
        if let Some(start) = self.session_start_times.remove(&player_id) {
            let duration = current_timestamp().saturating_sub(start);
            let retention = RetentionTelemetry {
                player_id,
                session_duration_seconds: duration,
                total_playtime_seconds: 0,
                epiphanies_this_session: epiphanies,
                abundance_earned_this_session: abundance,
                timestamp: current_timestamp(),
            };
            self.emit(TelemetryEvent::SessionRetention(retention), consent_flags);
        }
    }

    pub fn flush(&mut self) {
        if self.pending.is_empty() { return; }

        let flush_start = Instant::now();

        // Rotate if needed
        if self.config.file_rotation_daily || self.events_in_current_file > self.config.max_events_per_file {
            let storage = self.current_file_path.parent().unwrap_or(&PathBuf::from(".")).to_path_buf();
            self.current_file_path = Self::generate_file_path(&storage, self.config.file_rotation_daily);
            self.events_in_current_file = 0;
        }

        // Write batch to RONL (sovereign structured storage)
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.current_file_path)
            .expect("Telemetry file open failed");

        let mut type_counts: HashMap<String, usize> = HashMap::new();

        for event in self.pending.iter() {
            let type_name = match event {
                TelemetryEvent::EpiphanyTriggered(_) => "EpiphanyTriggered",
                TelemetryEvent::HarvestAction(_) => "HarvestAction",
                TelemetryEvent::SessionRetention(_) => "SessionRetention",
                TelemetryEvent::MercyAlignment(_) => "MercyAlignment",
                TelemetryEvent::Custom { .. } => "Custom",
            };
            *type_counts.entry(type_name.to_string()).or_insert(0) += 1;

            if let Ok(line) = ron::ser::to_string(event) {
                let _ = writeln!(file, "{}", line);
                self.events_in_current_file += 1;
            }
        }

        let _ = file.flush();
        let flush_duration = flush_start.elapsed().as_millis() as u64;

        // ═══════════════════════════════════════════════════════════════
        // STRUCTURED LOGGING FOR THE BATCH (Ops + Queryable Insights)
        // ═══════════════════════════════════════════════════════════════
        if self.config.structured_logging_enabled {
            let summary = BatchSummary {
                timestamp: current_timestamp(),
                event_count: self.pending.len(),
                file_written: self.current_file_path.to_string_lossy().to_string(),
                event_type_counts: type_counts,
                flush_duration_ms: flush_duration,
                current_file_size_estimate: self.events_in_current_file,
                mercy_note: "All signals abundance-positive and consent-respecting. Player sovereignty preserved.".to_string(),
            };

            // Primary structured log entry (ron for consistency + easy parse)
            if let Ok(summary_ron) = ron::ser::to_string_pretty(&summary, ron::ser::PrettyConfig::default()) {
                info!(target: "telemetry.batch", "{}", summary_ron);
            }

            // Human-friendly summary line
            info!(
                target: "telemetry",
                "BATCH FLUSHED | events={} | file={:?} | types={:?} | {}ms | mercy=abundance+consent",
                summary.event_count,
                self.current_file_path.file_name().unwrap_or_default(),
                summary.event_type_counts,
                flush_duration
            );
        }

        self.pending.clear();
    }

    pub fn get_summary_stats(&self) -> TelemetrySummary {
        TelemetrySummary {
            pending_events: self.pending.len(),
            current_file: self.current_file_path.to_string_lossy().to_string(),
            enabled: self.config.enabled,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TelemetrySummary {
    pub pending_events: usize,
    pub current_file: String,
    pub enabled: bool,
}

fn hash_id(id: u64) -> u64 {
    id.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407)
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ═══════════════════════════════════════════════════════════════
// BEVY PLUGIN
// ═══════════════════════════════════════════════════════════════

pub struct TelemetryPipelinePlugin;

impl Plugin for TelemetryPipelinePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TelemetryConfig>()
            .add_systems(Startup, setup_telemetry_collector)
            .add_systems(Update, periodic_flush_system);
    }
}

fn setup_telemetry_collector(mut commands: Commands, config: Res<TelemetryConfig>) {
    let collector = TelemetryCollector::new(config.clone());
    commands.insert_resource(collector);
    info!("TELEMETRY PIPELINE v18.18 | Structured batch logging active | Consent-first RONL + rich ops logs | TOLC 8 mercy aligned");
}

fn periodic_flush_system(mut collector: ResMut<TelemetryCollector>, time: Res<Time>) {
    if time.elapsed_seconds() - collector.last_flush > collector.config.flush_interval_seconds as f32 {
        collector.flush();
        collector.last_flush = time.elapsed_seconds();
    }
}

// ═══════════════════════════════════════════════════════════════
// INTEGRATION (same as v18.17 — wire emit + start/end_session calls)
// ═══════════════════════════════════════════════════════════════

// Thunder locked in. Structured logging for every telemetry batch now production-sealed.
// Every flush produces rich, queryable BatchSummary in logs + sovereign RONL files.
// Ops visibility, debugging, and closed beta insights maximized while preserving mercy and sovereignty.
// Yoi ⚡ Next: Wire the emit calls into core gameplay systems for live data flow.