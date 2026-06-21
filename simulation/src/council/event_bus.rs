/*!
 * CouncilEventBus with Append-Only Log + Optional fsync Durability.
 *
 * Supports both best-effort and strong durability modes.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

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
/// Supports both normal and durable (fsync) modes.
#[cfg_attr(feature = "bevy", derive(Resource))]
#[derive(Clone)]
pub struct CouncilEventBus {
    tx: Sender<CouncilEvent>,
    rx: Receiver<CouncilEvent>,
    persistence: Option<Arc<Mutex<BufWriter<File>>>>,
    durable: bool,
}

impl CouncilEventBus {
    /// Creates a new bounded event bus (in-memory only).
    pub fn new_bounded(capacity: usize) -> Self {
        let (tx, rx) = flume::bounded(capacity);
        Self {
            tx,
            rx,
            persistence: None,
            durable: false,
        }
    }

    /// Creates a bounded event bus with append-only JSON Lines persistence.
    /// Durability is best-effort (flush only).
    pub fn new_bounded_with_persistence<P: AsRef<Path>>(capacity: usize, path: P) -> std::io::Result<Self> {
        Self::new_bounded_with_persistence_internal(capacity, path, false)
    }

    /// Creates a bounded event bus with append-only persistence **and fsync** after every write.
    /// This provides stronger durability guarantees at the cost of performance.
    pub fn new_bounded_with_durable_persistence<P: AsRef<Path>>(capacity: usize, path: P) -> std::io::Result<Self> {
        Self::new_bounded_with_persistence_internal(capacity, path, true)
    }

    fn new_bounded_with_persistence_internal<P: AsRef<Path>>(capacity: usize, path: P, durable: bool) -> std::io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        let writer = BufWriter::new(file);
        let persistence = Some(Arc::new(Mutex::new(writer)));

        let (tx, rx) = flume::bounded(capacity);

        Ok(Self {
            tx,
            rx,
            persistence,
            durable,
        })
    }

    /// Send an event. If persistence is enabled, the event is appended.
    /// If durable mode is enabled, fsync is called after every write.
    pub fn send(&self, event: CouncilEvent) -> Result<(), flume::SendError<CouncilEvent>> {
        if let Some(writer) = &self.persistence {
            if let Ok(mut guard) = writer.lock() {
                if let Ok(json) = serde_json::to_string(&event) {
                    let _ = writeln!(guard, "{}", json);
                    let _ = guard.flush();

                    if self.durable {
                        // Stronger durability: force data to disk
                        if let Ok(file) = guard.get_ref().try_clone() {
                            let _ = file.sync_all();
                        }
                    }
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

/// Loads historical events from a JSON Lines persistence file.
pub fn load_events_from_log<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<CouncilEvent>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut events = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() { continue; }
        if let Ok(event) = serde_json::from_str::<CouncilEvent>(&line) {
            events.push(event);
        }
    }

    Ok(events)
}
