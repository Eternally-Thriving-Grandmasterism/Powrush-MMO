//! Ra-Thor → Powrush soft Policy Hint surface
//! v21.85.0 — Ultramasterism Feedback Loop (structural)
//!
//! Soft, non-authoritative, mercy-gated recommendations only.
//! Never overrides local simulation sovereignty or player agency.
//! Contact: info@Rathor.ai | TOLC 8 | PATSAGi

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, warn};

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
// Inbox Resource
// =============================================================================

#[derive(Resource, Debug)]
pub struct PolicyHintInbox {
    pub path: PathBuf,
    pub hints: VecDeque<PolicyHint>,
    pub total_ingested: u64,
    pub total_rejected: u64,
    pub last_ingest_unix: u64,
    /// Only accept hints for this session (or "*")
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

    /// Strongest (strength × mercy_factor) active hint for a category.
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

    /// All active hints (non-expired).
    pub fn active(&self) -> impl Iterator<Item = &PolicyHint> {
        let now = now_unix();
        self.hints
            .iter()
            .filter(move |h| h.expires_at_unix.map_or(true, |e| e > now))
    }

    fn push(&mut self, hint: PolicyHint) {
        // Simple de-dupe by hint_id
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
    // Zero-harm: recommended_delta must be non-negative for all current categories
    if h.recommended_delta < 0.0 {
        return Err("negative recommended_delta forbidden".into());
    }
    Ok(())
}

// =============================================================================
// Ingest System
// =============================================================================

pub fn policy_hint_ingest_system(mut inbox: ResMut<PolicyHintInbox>) {
    let path = inbox.path.clone();
    if !path.exists() {
        return;
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

    // Session filter
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
