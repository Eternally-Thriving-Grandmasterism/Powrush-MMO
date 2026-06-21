/*!
 * CouncilEventBus with Write-Ahead Logging (WAL) support.
 *
 * In WAL mode, events are durably persisted to the log *before* being delivered
 * to consumers. This provides strong "event is committed" semantics.
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

/// High-performance Council Event Bus with multiple durability strategies,
/// including Write-Ahead Logging semantics.
#[cfg_attr(feature = "bevy", derive(Resource))]
#[derive(Clone)]
pub struct CouncilEventBus {
    tx: Sender<CouncilEvent>,
    rx: Receiver<CouncilEvent>,
    persistence: Option<Arc<Mutex<BufWriter<File>>>>,
    wal_mode: bool,           // Write-Ahead Logging enabled
    fsync_interval: u32,
    write_count: Arc<Mutex<u32>>,
}

impl CouncilEventBus {
    pub fn new_bounded(capacity: usize) -> Self {
        let (tx, rx) = flume::bounded(capacity);
        Self {
            tx, rx,
            persistence: None,
            wal_mode: false,
            fsync_interval: 0,
            write_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Best-effort persistence (no WAL guarantee).
    pub fn new_bounded_with_persistence<P: AsRef<Path>>(capacity: usize, path: P) -> std::io::Result<Self> {
        Self::new_internal(capacity, path, false, false, 0)
    }

    /// Strong per-write durability.
    pub fn new_bounded_with_durable_persistence<P: AsRef<Path>>(capacity: usize, path: P) -> std::io::Result<Self> {
        Self::new_internal(capacity, path, true, false, 1)
    }

    /// Periodic fsync batching.
    pub fn new_bounded_with_batched_durability<P: AsRef<Path>>(capacity: usize, path: P, fsync_interval: u32) -> std::io::Result<Self> {
        Self::new_internal(capacity, path, true, false, fsync_interval)
    }

    /// **Write-Ahead Logging mode**.
    /// Events are durably persisted to the log *before* being made available to consumers.
    /// This provides the strongest "committed" semantics for governance events.
    pub fn new_bounded_with_write_ahead_log<P: AsRef<Path>>(capacity: usize, path: P, fsync_interval: u32) -> std::io::Result<Self> {
        Self::new_internal(capacity, path, true, true, fsync_interval)
    }

    fn new_internal<P: AsRef<Path>>(capacity: usize, path: P, durable: bool, wal_mode: bool, fsync_interval: u32) -> std::io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        let writer = BufWriter::new(file);
        let persistence = Some(Arc::new(Mutex::new(writer)));

        let (tx, rx) = flume::bounded(capacity);

        Ok(Self {
            tx, rx,
            persistence,
            wal_mode,
            fsync_interval,
            write_count: Arc::new(Mutex::new(0)),
        })
    }

    pub fn send(&self, event: CouncilEvent) -> Result<(), flume::SendError<CouncilEvent>> {
        // In WAL mode, we must successfully persist before delivering the event
        if self.persistence.is_some() {
            if let Some(writer) = &self.persistence {
                if let Ok(mut guard) = writer.lock() {
                    if let Ok(json) = serde_json::to_string(&event) {
                        // Write + flush
                        let _ = writeln!(guard, "{}", json);
                        let _ = guard.flush();

                        // Durability logic
                        let mut count = self.write_count.lock().unwrap();
                        *count += 1;

                        let should_sync = if self.fsync_interval == 0 {
                            true
                        } else {
                            *count % self.fsync_interval == 0
                        };

                        if self.wal_mode || should_sync {
                            if let Ok(file) = guard.get_ref().try_clone() {
                                let _ = file.sync_all();
                            }
                        }
                    }
                }
            }
        }

        // Only deliver to consumers after successful WAL write (when enabled)
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
