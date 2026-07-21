//! Ra-Thor → Powrush soft Policy Hint surface
//! v21.89.0 — Ultramasterism Feedback Loop (production-ready reception + self-emission helper)
//!
//! Soft, non-authoritative, mercy-gated recommendations only.
//! Never overrides local simulation sovereignty or player agency.
//! Contact: info@Rathor.ai | TOLC 8 | PATSAGi

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, warn};

use super::ServerTransferSession;

const MAX_HINTS: usize = 32;
const DEFAULT_HINT_PATH: &str = "artifacts/ra_thor_policy_hints.json";

// =============================================================================
// Schema — ra_thor_policy_hint_v1
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PolicyHint {
    pub hint_id: String,
    pub category: String,
    pub strength: f32,
    pub mercy_factor: f32,
    pub recommended_delta: f32,
    #[serde(default)]
    pub rationale: Option<String>,
    #[serde(default)]
    pub expires_at_unix: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyHintEnvelope {
    pub schema: String,
    pub source: String,
    pub emitted_at_unix: u64,
    pub target_session_id: String,
    #[serde(default)]
    pub source_export_seq: Option<u64>,
    pub hints: Vec<PolicyHint>,
}

// =============================================================================
// Soft Application State (observable effects)
// =============================================================================

#[derive(Resource, Debug, Default)]
pub struct SoftPolicyState {
    pub abundance_bias_applied: f64,
    pub peaceful_weight_applied: f64,
    pub ethical_floor_applied: f64,
    pub council_nudge_applied: f64,
    pub innovation_applied: f64,
    pub mercy_presence_applied: f64,
    pub applications: u64,
    pub applied_hint_ids: HashSet<String>,
}

// =============================================================================
// Inbox Resource
// =============================================================================

#[derive(Resource, Debug)]
pub struct PolicyHintInbox {
    pub path: PathBuf,
    pub hints: VecDeque<PolicyHint>,
    pub total_ingested: u64,
    pub total_rejected: u64,
    pub last_ingest_unix: u64,
    pub last_file_mtime: Option<u64>,
    pub session_id: String,
}

impl Default for PolicyHintInbox {
    fn default() -> Self {
        Self {
            path: PathBuf::from(DEFAULT_HINT_PATH),
            hints: VecDeque::new(),
            total_ingested: 0,
            total_rejected: 0,
            last_ingest_unix: 0,
            last_file_mtime: None,
            session_id: "*".into(),
        }
    }
}

impl PolicyHintInbox {
    pub fn with_session_id(mut self, id: impl Into<String>) -> Self {
        self.session_id = id.into();
        self
    }

    pub fn with_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.path = path.into();
        self
    }

    pub fn strongest_for(&self, category: &str) -> Option<&PolicyHint> {
        let now = now_unix();
        self.hints
            .iter()
            .filter(|h| h.category == category)
            .filter(|h| h.expires_at_unix.map_or(true, |e| e > now))
            .max_by(|a, b| {
                let sa = a.strength * a.mercy_factor;
                let sb = b.strength * b.mercy_factor;
                sa.partial_cmp(&sb).unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    pub fn active(&self) -> impl Iterator<Item = &PolicyHint> {
        let now = now_unix();
        self.hints
            .iter()
            .filter(move |h| h.expires_at_unix.map_or(true, |e| e > now))
    }

    fn push(&mut self, hint: PolicyHint) {
        if self.hints.iter().any(|h| h.hint_id == hint.hint_id) {
            return;
        }
        self.hints.push_back(hint);
        while self.hints.len() > MAX_HINTS {
            self.hints.pop_front();
        }
        self.total_ingested = self.total_ingested.saturating_add(1);
    }
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn file_mtime_secs(path: &Path) -> Option<u64> {
    std::fs::metadata(path)
        .ok()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
}

// =============================================================================
// Validation (mercy + zero-harm)
// =============================================================================

fn is_allowed_category(cat: &str) -> bool {
    matches!(
        cat,
        "abundance_bias"
            | "peaceful_resolution_weight"
            | "ethical_floor"
            | "council_participation_nudge"
            | "innovation_encouragement"
            | "mercy_presence"
    )
}

fn validate_hint(h: &PolicyHint) -> Result<(), String> {
    if !is_allowed_category(&h.category) {
        return Err(format!("disallowed category: {}", h.category));
    }
    if !(0.0..=1.0).contains(&h.strength) {
        return Err("strength out of [0,1]".into());
    }
    if !(0.0..=1.0).contains(&h.mercy_factor) {
        return Err("mercy_factor out of [0,1]".into());
    }
    if h.recommended_delta < 0.0 {
        return Err("negative recommended_delta forbidden".into());
    }
    Ok(())
}

// =============================================================================
// Ingest System (only re-reads when file mtime changes)
// =============================================================================

pub fn policy_hint_ingest_system(mut inbox: ResMut<PolicyHintInbox>) {
    let path = inbox.path.clone();
    if !path.exists() {
        return;
    }

    // Skip if file has not changed since last successful ingest
    if let Some(mtime) = file_mtime_secs(&path) {
        if inbox.last_file_mtime == Some(mtime) {
            return;
        }
    }

    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            warn!(target: "ra_thor::policy", error = %e, "Failed to read policy hint file");
            return;
        }
    };

    let envelope: PolicyHintEnvelope = match serde_json::from_str(&content) {
        Ok(e) => e,
        Err(e) => {
            warn!(target: "ra_thor::policy", error = %e, "Malformed policy hint JSON — ignored");
            inbox.total_rejected = inbox.total_rejected.saturating_add(1);
            return;
        }
    };

    if envelope.schema != "ra_thor_policy_hint_v1" {
        warn!(target: "ra_thor::policy", schema = %envelope.schema, "Unknown policy hint schema — ignored");
        inbox.total_rejected = inbox.total_rejected.saturating_add(1);
        return;
    }

    if envelope.target_session_id != "*" && envelope.target_session_id != inbox.session_id {
        return;
    }

    let mut accepted = 0u32;
    for hint in envelope.hints {
        match validate_hint(&hint) {
            Ok(()) => {
                inbox.push(hint);
                accepted += 1;
            }
            Err(reason) => {
                warn!(target: "ra_thor::policy", reason = %reason, "Hint rejected");
                inbox.total_rejected = inbox.total_rejected.saturating_add(1);
            }
        }
    }

    if accepted > 0 {
        inbox.last_ingest_unix = now_unix();
        inbox.last_file_mtime = file_mtime_secs(&path);
        info!(
            target: "ra_thor::policy",
            accepted,
            total_ingested = inbox.total_ingested,
            active = inbox.hints.len(),
            session = %inbox.session_id,
            "Policy hints ingested (soft, mercy-gated)"
        );
    }
}

// =============================================================================
// Optional Self-Emission Helper (for full-loop testing without Ra-Thor)
// =============================================================================

/// Write a valid ra_thor_policy_hint_v1 envelope to the default path.
/// Useful for self-contained testing of the full feedback loop.
pub fn emit_test_policy_hints(
    target_session_id: &str,
    source_export_seq: Option<u64>,
) -> Result<(), String> {
    let envelope = PolicyHintEnvelope {
        schema: "ra_thor_policy_hint_v1".into(),
        source: "powrush-self-test".into(),
        emitted_at_unix: now_unix(),
        target_session_id: target_session_id.into(),
        source_export_seq,
        hints: vec![
            PolicyHint {
                hint_id: format!("self_abundance_{}", now_unix()),
                category: "abundance_bias".into(),
                strength: 0.72,
                mercy_factor: 0.90,
                recommended_delta: 0.05,
                rationale: Some("Self-test abundance bias".into()),
                expires_at_unix: None,
            },
            PolicyHint {
                hint_id: format!("self_peaceful_{}", now_unix()),
                category: "peaceful_resolution_weight".into(),
                strength: 0.68,
                mercy_factor: 0.88,
                recommended_delta: 0.04,
                rationale: Some("Self-test peaceful weight".into()),
                expires_at_unix: None,
            },
            PolicyHint {
                hint_id: format!("self_mercy_{}", now_unix()),
                category: "mercy_presence".into(),
                strength: 0.75,
                mercy_factor: 0.94,
                recommended_delta: 0.05,
                rationale: Some("Self-test mercy presence".into()),
                expires_at_unix: None,
            },
        ],
    };

    let path = Path::new(DEFAULT_HINT_PATH);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let json = serde_json::to_string_pretty(&envelope).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())?;

    info!(
        target: "ra_thor::policy",
        path = %path.display(),
        session = target_session_id,
        "Self-test policy hints emitted (full-loop testing helper)"
    );
    Ok(())
}

// =============================================================================
// Soft Application Systems (full closed category set)
// =============================================================================

fn apply_if_new(
    soft: &mut SoftPolicyState,
    hint: &PolicyHint,
    category: &str,
    apply_fn: impl FnOnce(f64),
) {
    if soft.applied_hint_ids.contains(&hint.hint_id) {
        return;
    }

    let scale = (hint.strength * hint.mercy_factor) as f64;
    let delta = (hint.recommended_delta as f64) * scale;

    apply_fn(delta);

    soft.applied_hint_ids.insert(hint.hint_id.clone());
    soft.applications = soft.applications.saturating_add(1);

    info!(
        target: "ra_thor::policy::soft",
        category = category,
        hint_id = %hint.hint_id,
        delta = delta,
        scale = scale,
        mercy = hint.mercy_factor,
        "Soft policy applied (non-authoritative)"
    );
}

pub fn soft_policy_application_system(
    inbox: Res<PolicyHintInbox>,
    mut soft: ResMut<SoftPolicyState>,
    mut transfer: ResMut<ServerTransferSession>,
) {
    if let Some(hint) = inbox.strongest_for("abundance_bias") {
        apply_if_new(&mut soft, hint, "abundance_bias", |delta| {
            transfer.record_abundance_velocity(1.0 + delta);
            soft.abundance_bias_applied += delta;
        });
    }

    if let Some(hint) = inbox.strongest_for("peaceful_resolution_weight") {
        apply_if_new(&mut soft, hint, "peaceful_resolution_weight", |delta| {
            transfer.record_treaty();
            soft.peaceful_weight_applied += delta;
        });
    }

    if let Some(hint) = inbox.strongest_for("ethical_floor") {
        apply_if_new(&mut soft, hint, "ethical_floor", |delta| {
            transfer.record_council_passed(0.85 + delta.min(0.14));
            soft.ethical_floor_applied += delta;
        });
    }

    if let Some(hint) = inbox.strongest_for("council_participation_nudge") {
        apply_if_new(&mut soft, hint, "council_participation_nudge", |delta| {
            transfer.record_council_passed(0.80);
            soft.council_nudge_applied += delta;
        });
    }

    if let Some(hint) = inbox.strongest_for("innovation_encouragement") {
        apply_if_new(&mut soft, hint, "innovation_encouragement", |delta| {
            transfer.record_faction_shift(0.4, 0.4 + delta as f32);
            soft.innovation_applied += delta;
        });
    }

    if let Some(hint) = inbox.strongest_for("mercy_presence") {
        apply_if_new(&mut soft, hint, "mercy_presence", |delta| {
            transfer.record_council_passed(0.90 + delta.min(0.09));
            soft.mercy_presence_applied += delta;
        });
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowed_categories_pass() {
        let h = PolicyHint {
            hint_id: "t1".into(),
            category: "abundance_bias".into(),
            strength: 0.7,
            mercy_factor: 0.9,
            recommended_delta: 0.05,
            rationale: None,
            expires_at_unix: None,
        };
        assert!(validate_hint(&h).is_ok());
    }

    #[test]
    fn negative_delta_rejected() {
        let h = PolicyHint {
            hint_id: "t2".into(),
            category: "abundance_bias".into(),
            strength: 0.7,
            mercy_factor: 0.9,
            recommended_delta: -0.1,
            rationale: None,
            expires_at_unix: None,
        };
        assert!(validate_hint(&h).is_err());
    }

    #[test]
    fn disallowed_category_rejected() {
        let h = PolicyHint {
            hint_id: "t3".into(),
            category: "increase_aggression".into(),
            strength: 0.5,
            mercy_factor: 0.5,
            recommended_delta: 0.1,
            rationale: None,
            expires_at_unix: None,
        };
        assert!(validate_hint(&h).is_err());
    }
}
