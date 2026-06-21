/*!
 * CouncilEventBus with Periodic Fsync Batching.
 *
 * Offers three durability modes:
 * - No persistence
 * - Best-effort (flush only)
 * - Periodic fsync (configurable batch size)
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

/// High-performance Council Event Bus with flexible durability options.
#[cfg_attr(feature = "bevy", derive(Resource))]
#[derive(Clone)]
pub struct CouncilEventBus {
    tx: Sender<CouncilEvent>,
    rx: Receiver<CouncilEvent>,
    persistence: Option<Arc<Mutex<BufWriter<File>>>>,
    durable: bool,
    fsync_interval: u32,      // 0 = every write, >0 = every N writes
    write_count: Arc<Mutex<u32>>,
}

impl CouncilEventBus {
    pub fn new_bounded(capacity: usize) -> Self {
        let (tx, rx) = flume::bounded(capacity);
        Self {
            tx,
            rx,
            persistence: None,
            durable: false,
            fsync_interval: 0,
            write_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Best-effort persistence (flush only, no fsync).
    pub fn new_bounded_with_persistence<P: AsRef<Path>>(capacity: usize, path: P) -> std::io::Result<Self> {
        Self::new_bounded_with_persistence_internal(capacity, path, false, 0)
    }

    /// Strong durability: fsync after **every** write.
    pub fn new_bounded_with_durable_persistence<P: AsRef<Path>>(capacity: usize, path: P) -> std::io::Result<Self> {
        Self::new_bounded_with_persistence_internal(capacity, path, true, 1)
    }

    /// Recommended balanced mode: fsync periodically (e.g. every 50 or 100 events).
    /// This gives good durability with much better performance than per-write fsync.
    pub fn new_bounded_with_batched_durability<P: AsRef<Path>>(capacity: usize, path: P, fsync_interval: u32) -> std::io::Result<Self> {
        Self::new_bounded_with_persistence_internal(capacity, path, true, fsync_interval)
    }

    fn new_bounded_with_persistence_internal<P: AsRef<Path>>(capacity: usize, path: P, durable: bool, fsync_interval: u32) -> std::io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        let writer = BufWriter::new(file);
        let persistence = Some(Arc::new(Mutex::new(writer)));

        let (tx, rx) = flume::bounded(capacity);

        Ok(Self {
            tx,
            rx,
            persistence,
            durable,
            fsync_interval,
            write_count: Arc::new(Mutex::new(0)),
        })
    }

    pub fn send(&self, event: CouncilEvent) -> Result<(), flume::SendError<CouncilEvent>> {
        if let Some(writer) = &self.persistence {
            if let Ok(mut guard) = writer.lock() {
                if let Ok(json) = serde_json::to_string(&event) {
                    let _ = writeln!(guard, "{}", json);
                    let _ = guard.flush();

                    if self.durable {
                        let mut count = self.write_count.lock().unwrap();
                        *count += 1;

                        let should_sync = if self.fsync_interval == 0 {
                            true
                        } else {
                            *count % self.fsync_interval == 0
                        };

                        if should_sync {
                            if let Ok(file) = guard.get_ref().try_clone() {
                                let _ = file.sync_all();
                            }
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
