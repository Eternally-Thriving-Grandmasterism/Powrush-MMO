/*!
 * CouncilEventBus with optional Append-Only Log persistence.
 *
 * Lightweight, mercy-aligned durability for Council governance events.
 * Uses JSON Lines for human-readable, append-only audit logs.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

use flume::{Receiver, Sender};
use serde::{Deserialize, Serialize};

use crate::world::AgentId;

#[cfg(feature = "bevy")]
use bevy::prelude::Resource;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CouncilEvent {
    ProposalSubmitted {
        proposal_id: u64,
        proposer: AgentId,
        proposal_type: crate::council::ProposalType,
        title: String,
    },
    ProposalPassed {
        proposal_id: u64,
        votes_for: u32,
        votes_against: u32,
        mercy_factor: f32,
    },
    DecisionApplied {
        proposal_id: u64,
        effect_type: String,
        magnitude: f32,
        applied_tick: u64,
    },
    AuditLogAppended {
        decision_id: u64,
        proposer: AgentId,
        mercy_factor: f32,
    },
}

/// High-performance Council Event Bus with optional append-only persistence.
#[cfg_attr(feature = "bevy", derive(Resource))]
#[derive(Clone)]
pub struct CouncilEventBus {
    tx: Sender<CouncilEvent>,
    rx: Receiver<CouncilEvent>,
    /// Optional persistent writer (JSON Lines)
    persistence: Option<std::sync::Arc<std::sync::Mutex<BufWriter<File>>>>,
}

impl CouncilEventBus {
    /// Creates a new bounded event bus (in-memory only).
    pub fn new_bounded(capacity: usize) -> Self {
        let (tx, rx) = flume::bounded(capacity);
        Self {
            tx,
            rx,
            persistence: None,
        }
    }

    /// Creates a bounded event bus that also appends every event to a JSON Lines file.
    /// This provides lightweight, human-readable persistence for audit and replay.
    pub fn new_bounded_with_persistence<P: AsRef<Path>>(capacity: usize, path: P) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        let writer = BufWriter::new(file);
        let persistence = Some(std::sync::Arc::new(std::sync::Mutex::new(writer)));

        let (tx, rx) = flume::bounded(capacity);

        Ok(Self {
            tx,
            rx,
            persistence,
        })
    }

    /// Send an event. If persistence is enabled, the event is also appended to the log file.
    pub fn send(&self, event: CouncilEvent) -> Result<(), flume::SendError<CouncilEvent>> {
        // Persist first (best effort)
        if let Some(writer) = &self.persistence {
            if let Ok(mut guard) = writer.lock() {
                if let Ok(json) = serde_json::to_string(&event) {
                    let _ = writeln!(guard, "{}", json);
                    let _ = guard.flush();
                }
            }
        }

        self.tx.send(event)
    }

    pub fn try_recv(&self) -> Result<CouncilEvent, flume::TryRecvError> {
        self.rx.try_recv()
    }

    pub fn recv(&self) -> Result<CouncilEvent, flume::RecvError> {
        self.rx.recv()
    }

    pub fn sender(&self) -> Sender<CouncilEvent> {
        self.tx.clone()
    }

    pub fn receiver(&self) -> Receiver<CouncilEvent> {
        self.rx.clone()
    }
}

impl Default for CouncilEventBus {
    fn default() -> Self {
        Self::new_bounded(1024)
    }
}
