//! Bridging export + metacognitive soft prompts
//! v21.90.0 — High-road transfer producer for Ra-Thor SchemaRegistry
//!
//! Writes `artifacts/powrush_bridging_latest.json` with schema
//! `powrush_bridging_context_v1` — field-compatible with Ra-Thor
//! `BridgingContext` (reality-thriving-transfer v14.16+).
//!
//! Zero dependency on Ra-Thor crate. Pure JSON handoff.
//! Contact: info@Rathor.ai | TOLC 8. Yoi ⚡

use bevy::prelude::*;
use serde::Serialize;
use std::path::PathBuf;
use tracing::info;

use crate::council::decision::CouncilDecisions;
use crate::economy::{EconomyState, MultiRealmRbeSnapshot};

// =============================================================================
// Bridging context payload (matches Ra-Thor BridgingContext fields)
// =============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct BridgingContextPayload {
    pub schema: String,
    pub session_id: Option<String>,
    pub realm_id: Option<u8>,
    pub decision_title: Option<String>,
    pub decision_type: Option<String>,
    pub mercy_factor: f64,
    pub ethical_score: f64,
    pub rbe_quality: f64,
    pub peaceful_rate: f64,
    pub abundance_velocity: f64,
    pub surface_label: String,
    pub decision_id: Option<u64>,
    pub tick: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct BridgingBatchPayload {
    pub schema: String,
    pub source: String,
    pub contexts: Vec<BridgingContextPayload>,
    pub exported_at_unix: u64,
}

// =============================================================================
// Config + metacognitive scaffolding resource
// =============================================================================

#[derive(Resource, Debug, Clone)]
pub struct BridgingExportConfig {
    pub path: PathBuf,
    pub batch_path: PathBuf,
    pub interval_secs: f32,
    pub enabled: bool,
    pub last_export_at: f32,
    pub export_count: u64,
    /// Last resolved_history len we already exported from.
    pub last_history_len: usize,
}

impl Default for BridgingExportConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("artifacts/powrush_bridging_latest.json"),
            batch_path: PathBuf::from("artifacts/powrush_bridging_batch_latest.json"),
            interval_secs: 8.0,
            enabled: true,
            last_export_at: -999.0,
            export_count: 0,
            last_history_len: 0,
        }
    }
}

/// Fadable metacognitive scaffolding for council deliberation loops.
#[derive(Resource, Debug, Clone)]
pub struct MetacognitiveScaffold {
    pub enabled: bool,
    /// 1.0 = full prompts, 0.0 = fully faded (independent self-regulation).
    pub support_level: f32,
    pub planning_fired: u64,
    pub monitoring_fired: u64,
    pub evaluation_fired: u64,
}

impl Default for MetacognitiveScaffold {
    fn default() -> Self {
        Self {
            enabled: true,
            support_level: 0.85,
            planning_fired: 0,
            monitoring_fired: 0,
            evaluation_fired: 0,
        }
    }
}

impl MetacognitiveScaffold {
    pub fn planning_prompt(&self) -> Option<&'static str> {
        if !self.enabled || self.support_level <= 0.05 {
            return None;
        }
        Some(
            "What is the goal? Which portable principles might apply? What mercy constraints are active?",
        )
    }

    pub fn monitoring_prompt(&self) -> Option<&'static str> {
        if !self.enabled || self.support_level <= 0.05 {
            return None;
        }
        Some(
            "Are we still aligned with mercy bounds? Is surface similarity misleading the mapping?",
        )
    }

    pub fn evaluation_prompt(&self) -> Option<&'static str> {
        if !self.enabled || self.support_level <= 0.05 {
            return None;
        }
        Some(
            "Which principles transferred successfully? Which failed? What should update in the schema registry?",
        )
    }

    /// Fade support as outcomes improve (call after strong resolved batches).
    pub fn fade_toward(&mut self, target: f32) {
        let t = target.clamp(0.0, 1.0);
        self.support_level = (self.support_level * 0.9 + t * 0.1).clamp(0.0, 1.0);
    }
}

fn now_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn write_json(path: &PathBuf, bytes: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }
    std::fs::write(path, bytes).map_err(|e| e.to_string())
}

/// Soft system: map recent resolved decisions → BridgingContext JSON for Ra-Thor.
pub fn council_bridging_export_system(
    time: Res<Time>,
    mut cfg: ResMut<BridgingExportConfig>,
    mut scaffold: ResMut<MetacognitiveScaffold>,
    decisions: Res<CouncilDecisions>,
    economy: Option<Res<EconomyState>>,
    rbe: Option<Res<MultiRealmRbeSnapshot>>,
) {
    if !cfg.enabled {
        return;
    }
    let now = time.elapsed_seconds();
    if now - cfg.last_export_at < cfg.interval_secs {
        return;
    }

    if decisions.resolved_history.is_empty() {
        return;
    }

    // Only export when history grew (or periodic refresh of latest)
    let history_grew = decisions.resolved_history.len() > cfg.last_history_len;
    if !history_grew && cfg.export_count > 0 {
        // Still allow periodic re-export of latest for offline consumers
        if now - cfg.last_export_at < cfg.interval_secs * 3.0 {
            return;
        }
    }

    cfg.last_export_at = now;
    cfg.last_history_len = decisions.resolved_history.len();

    // Evaluation-phase scaffold (post-resolution)
    if let Some(prompt) = scaffold.evaluation_prompt() {
        scaffold.evaluation_fired = scaffold.evaluation_fired.saturating_add(1);
        info!(target: "ra_thor::meta", phase = "evaluation", prompt = prompt, support = scaffold.support_level);
    }

    let abundance = economy
        .as_ref()
        .map(|e| e.abundance_velocity as f64)
        .or_else(|| rbe.as_ref().map(|s| s.avg_flow.max(0.0) as f64 + 0.5))
        .unwrap_or(0.9);

    let sust = rbe
        .as_ref()
        .map(|s| s.avg_sustainability as f64)
        .or_else(|| economy.as_ref().map(|e| e.average_sustainability as f64))
        .unwrap_or(0.7);

    let stress = rbe
        .as_ref()
        .map(|s| s.avg_stress as f64)
        .unwrap_or(0.3);
    let peaceful = (1.0 - stress * 0.5).clamp(0.0, 1.0);

    // Latest decision → single latest payload
    let latest = decisions.resolved_history.last().unwrap();
    let mercy = latest.mercy_factor as f64;
    let single = BridgingContextPayload {
        schema: "powrush_bridging_context_v1".into(),
        session_id: Some(format!("council_tick_{}", decisions.last_applied_tick)),
        realm_id: Some(latest.realm_id),
        decision_title: Some(latest.title.clone()),
        decision_type: Some(format!("{:?}", latest.proposal_type)),
        mercy_factor: mercy.clamp(0.0, 1.0),
        ethical_score: mercy.clamp(0.0, 1.0),
        rbe_quality: sust.clamp(0.0, 1.0),
        peaceful_rate: peaceful,
        abundance_velocity: abundance.max(0.0),
        surface_label: format!("realm_{}_{}", latest.realm_id, latest.effect_type),
        decision_id: Some(latest.decision_id),
        tick: decisions.last_applied_tick,
    };

    // Batch: up to last 8 resolved
    let start = decisions.resolved_history.len().saturating_sub(8);
    let mut contexts = Vec::new();
    for d in &decisions.resolved_history[start..] {
        let m = d.mercy_factor as f64;
        contexts.push(BridgingContextPayload {
            schema: "powrush_bridging_context_v1".into(),
            session_id: Some(format!("council_tick_{}", d.created_tick)),
            realm_id: Some(d.realm_id),
            decision_title: Some(d.title.clone()),
            decision_type: Some(format!("{:?}", d.proposal_type)),
            mercy_factor: m.clamp(0.0, 1.0),
            ethical_score: m.clamp(0.0, 1.0),
            rbe_quality: sust.clamp(0.0, 1.0),
            peaceful_rate: peaceful,
            abundance_velocity: abundance.max(0.0),
            surface_label: format!("realm_{}_{}", d.realm_id, d.effect_type),
            decision_id: Some(d.decision_id),
            tick: d.created_tick,
        });
    }

    let batch = BridgingBatchPayload {
        schema: "powrush_bridging_batch_v1".into(),
        source: "powrush-mmo-simulation".into(),
        contexts,
        exported_at_unix: now_unix(),
    };

    match serde_json::to_string_pretty(&single) {
        Ok(json) => {
            if let Err(e) = write_json(&cfg.path, &json) {
                info!(target: "ra_thor::bridging", error = %e, "bridging single write soft-fail");
            } else {
                cfg.export_count = cfg.export_count.saturating_add(1);
                info!(
                    target: "ra_thor::bridging",
                    path = %cfg.path.display(),
                    decision_id = latest.decision_id,
                    mercy = mercy,
                    "BridgingContext exported for Ra-Thor SchemaRegistry"
                );
            }
        }
        Err(e) => {
            info!(target: "ra_thor::bridging", error = %e, "bridging serialize soft-fail");
        }
    }

    if let Ok(json) = serde_json::to_string_pretty(&batch) {
        let _ = write_json(&cfg.batch_path, &json);
    }

    // Fade scaffold slightly when mercy is strong (competence signal)
    if mercy >= 0.75 {
        scaffold.fade_toward(scaffold.support_level * 0.95);
    }
}

/// Optional: fire planning prompt at start of deliberation window (lightweight log).
pub fn metacognitive_planning_pulse_system(
    decisions: Res<CouncilDecisions>,
    mut scaffold: ResMut<MetacognitiveScaffold>,
) {
    if !scaffold.enabled || decisions.pending.is_empty() {
        return;
    }
    if let Some(prompt) = scaffold.planning_prompt() {
        scaffold.planning_fired = scaffold.planning_fired.saturating_add(1);
        info!(target: "ra_thor::meta", phase = "planning", prompt = prompt, pending = decisions.pending.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_fades() {
        let mut s = MetacognitiveScaffold::default();
        let before = s.support_level;
        s.fade_toward(0.2);
        assert!(s.support_level < before);
    }

    #[test]
    fn prompts_none_when_faded() {
        let mut s = MetacognitiveScaffold::default();
        s.support_level = 0.0;
        assert!(s.planning_prompt().is_none());
    }
}
