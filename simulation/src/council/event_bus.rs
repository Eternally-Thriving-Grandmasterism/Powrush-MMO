/*!
 * CouncilEventBus with Append-Only Log + Replay support.
 *
 * Lightweight persistence + ability to reload historical events on startup.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
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
    persistence: Option<std::sync::Arc<std::sync::Mutex<BufWriter<File>>>>,
}

impl CouncilEventBus {
    pub fn new_bounded(capacity: usize) -> Self {
        let (tx, rx) = flume::bounded(capacity);
        Self { tx, rx, persistence: None }
    }

    pub fn new_bounded_with_persistence<P: AsRef<Path>>(capacity: usize, path: P) -> std::io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        let writer = BufWriter::new(file);
        let persistence = Some(std::sync::Arc::new(std::sync::Mutex::new(writer)));

        let (tx, rx) = flume::bounded(capacity);
        Ok(Self { tx, rx, persistence })
    }

    pub fn send(&self, event: CouncilEvent) -> Result<(), flume::SendError<CouncilEvent>> {
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

/// Loads all historical CouncilEvents from a JSON Lines persistence file.
/// Useful for replay, analytics, or restoring state on startup.
pub fn load_events_from_log<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<CouncilEvent>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut events = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str::<CouncilEvent>(&line) {
            Ok(event) => events.push(event),
            Err(e) => {
                eprintln!("Warning: Failed to parse council event line: {} - {}", line, e);
            }
        }
    }

    Ok(events)
}
