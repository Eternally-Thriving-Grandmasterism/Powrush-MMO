//! simulation/src/council/rtt_export.rs
//! Host mapper — CouncilDecisions::resolved_history → pure RTT export queue
//! v21.78.0
//!
//! Zero coupling to the server crate. Field shape matches server::CouncilRttSignal.
//! Host drain pattern:
//! ```ignore
//! for s in export_queue.drain() {
//!     server_inbox.push_passed(s.decision_id, s.mercy_factor, s.strength, s.realm_id, s.abundance_velocity_hint);
//! }
//! ```
//! Contact: info@Rathor.ai | TOLC 8. Yoi ⚡

use bevy::prelude::*;
use std::collections::HashMap;
use tracing::info;

use crate::council::decision::CouncilDecisions;
use crate::economy::EconomyState;

/// Pure scalar signal — mirrors server `CouncilRttSignal` fields.
#[derive(Debug, Clone)]
pub struct CouncilRttExportSignal {
    pub decision_id: u64,
    pub mercy_factor: f32,
    pub strength: f32,
    pub realm_id: u8,
    pub abundance_velocity_hint: Option<f64>,
}

/// Simulation-side export queue for host → server RTT bridge.
#[derive(Resource, Debug, Default)]
pub struct CouncilRttExportQueue {
    pub pending: Vec<CouncilRttExportSignal>,
    pub exported_ids: HashMap<u64, ()>,
    pub total_exported: u64,
}

impl CouncilRttExportQueue {
    /// Drain all pending signals (host consumes).
    pub fn drain(&mut self) -> Vec<CouncilRttExportSignal> {
        std::mem::take(&mut self.pending)
    }

    pub fn push(&mut self, signal: CouncilRttExportSignal) {
        if self.exported_ids.contains_key(&signal.decision_id) {
            return;
        }
        self.exported_ids.insert(signal.decision_id, ());
        if self.exported_ids.len() > 512 {
            self.exported_ids.clear();
            self.exported_ids.insert(signal.decision_id, ());
        }
        self.pending.push(signal);
        self.total_exported = self.total_exported.saturating_add(1);
    }
}

/// Map newly resolved council decisions into the pure export queue.
pub fn council_resolved_to_rtt_export_system(
    decisions: Res<CouncilDecisions>,
    economy: Option<Res<EconomyState>>,
    mut export: ResMut<CouncilRttExportQueue>,
) {
    if decisions.resolved_history.is_empty() {
        return;
    }

    let abundance_hint = economy.map(|e| e.abundance_velocity as f64);

    for d in decisions.resolved_history.iter() {
        if export.exported_ids.contains_key(&d.decision_id) {
            continue;
        }
        export.push(CouncilRttExportSignal {
            decision_id: d.decision_id,
            mercy_factor: d.mercy_factor,
            strength: d.strength,
            realm_id: d.realm_id,
            abundance_velocity_hint: abundance_hint,
        });
        info!(
            target: "ra_thor::council::rtt_export",
            decision_id = d.decision_id,
            realm_id = d.realm_id,
            mercy = d.mercy_factor,
            "Council decision queued for host RTT bridge"
        );
    }
}

// Thunder locked in. Host mapper live. Yoi ⚡
