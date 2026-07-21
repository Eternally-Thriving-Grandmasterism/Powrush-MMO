//! Optional co-host auto-drain: pure mirror → CouncilRttInbox
//! v21.79.0
//!
//! When simulation and server share one Bevy App, the host (or a thin adapter)
//! pushes pure scalars into `CohostExportMirror`. This system drains them into
//! `CouncilRttInbox` every frame — no simulation crate dependency on server.
//!
//! Host adapter example (binary that depends on both crates):
//! ```ignore
//! for s in sim_export_queue.drain() {
//!     mirror.push(CohostMirrorSignal {
//!         decision_id: s.decision_id,
//!         mercy_factor: s.mercy_factor,
//!         strength: s.strength,
//!         realm_id: s.realm_id,
//!         abundance_velocity_hint: s.abundance_velocity_hint,
//!     });
//! }
//! ```
//! Contact: info@Rathor.ai

use bevy::prelude::*;
use tracing::info;

use super::{CouncilRttInbox, CouncilRttSignal};

/// Pure scalar — field-compatible with sim `CouncilRttExportSignal`.
#[derive(Debug, Clone)]
pub struct CohostMirrorSignal {
    pub decision_id: u64,
    pub mercy_factor: f32,
    pub strength: f32,
    pub realm_id: u8,
    pub abundance_velocity_hint: Option<f64>,
}

/// Host-filled mirror queue (optional co-host path).
#[derive(Resource, Debug, Default)]
pub struct CohostExportMirror {
    pub pending: Vec<CohostMirrorSignal>,
    pub enabled: bool,
    pub total_drained: u64,
}

impl CohostExportMirror {
    pub fn enabled() -> Self {
        Self {
            pending: Vec::new(),
            enabled: true,
            total_drained: 0,
        }
    }

    pub fn push(&mut self, signal: CohostMirrorSignal) {
        self.pending.push(signal);
    }

    pub fn push_passed(
        &mut self,
        decision_id: u64,
        mercy_factor: f32,
        strength: f32,
        realm_id: u8,
        abundance_hint: Option<f64>,
    ) {
        self.push(CohostMirrorSignal {
            decision_id,
            mercy_factor,
            strength,
            realm_id,
            abundance_velocity_hint: abundance_hint,
        });
    }

    pub fn drain(&mut self) -> Vec<CohostMirrorSignal> {
        std::mem::take(&mut self.pending)
    }
}

/// Auto-drain mirror → CouncilRttInbox (then existing bridge → transfer session).
pub fn cohost_auto_drain_system(
    mut mirror: ResMut<CohostExportMirror>,
    mut inbox: ResMut<CouncilRttInbox>,
) {
    if !mirror.enabled && mirror.pending.is_empty() {
        return;
    }
    if mirror.pending.is_empty() {
        return;
    }

    let pending = mirror.drain();
    let mut drained = 0u64;
    for s in pending {
        if inbox.ingested_ids.contains_key(&s.decision_id) {
            continue;
        }
        let mut signal = CouncilRttSignal::new(
            s.decision_id,
            s.mercy_factor,
            s.strength,
            s.realm_id,
        );
        if let Some(v) = s.abundance_velocity_hint {
            signal = signal.with_abundance(v);
        }
        inbox.push(signal);
        drained = drained.saturating_add(1);
    }

    if drained > 0 {
        mirror.total_drained = mirror.total_drained.saturating_add(drained);
        info!(
            target: "ra_thor::cohost",
            drained = drained,
            total_drained = mirror.total_drained,
            "Cohost mirror auto-drained into CouncilRttInbox"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mirror_push_and_drain() {
        let mut m = CohostExportMirror::enabled();
        m.push_passed(1, 0.9, 1.2, 0, Some(1.1));
        m.push_passed(2, 0.8, 1.0, 1, None);
        assert_eq!(m.pending.len(), 2);
        let d = m.drain();
        assert_eq!(d.len(), 2);
        assert!(m.pending.is_empty());
    }
}
