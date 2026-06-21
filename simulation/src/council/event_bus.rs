/*!
 * CouncilEventBus
 *
 * High-performance, mercy-aligned event streaming for the Council Proposal System.
 * Built on flume for excellent sync + async ergonomics and low latency.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use flume::{Receiver, Sender};
use serde::{Deserialize, Serialize};

use crate::world::AgentId;

/// Events emitted by the Council governance system.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CouncilEvent {
    /// A new proposal was submitted
    ProposalSubmitted {
        proposal_id: u64,
        proposer: AgentId,
        proposal_type: crate::council::ProposalType,
        title: String,
    },

    /// A proposal passed deliberation
    ProposalPassed {
        proposal_id: u64,
        votes_for: u32,
        votes_against: u32,
        mercy_factor: f32,
    },

    /// A decision was applied to the world state
    DecisionApplied {
        proposal_id: u64,
        effect_type: String,
        magnitude: f32,
        applied_tick: u64,
    },

    /// New entry appended to the persistent audit log
    AuditLogAppended {
        decision_id: u64,
        proposer: AgentId,
        mercy_factor: f32,
    },
}

/// High-performance event bus for Council governance.
pub struct CouncilEventBus {
    tx: Sender<CouncilEvent>,
    rx: Receiver<CouncilEvent>,
}

impl CouncilEventBus {
    /// Creates a new bounded CouncilEventBus (recommended for backpressure safety).
    pub fn new_bounded(capacity: usize) -> Self {
        let (tx, rx) = flume::bounded(capacity);
        Self { tx, rx }
    }

    /// Creates an unbounded CouncilEventBus (use with care).
    pub fn new_unbounded() -> Self {
        let (tx, rx) = flume::unbounded();
        Self { tx, rx }
    }

    /// Send an event (non-blocking if bounded and full).
    pub fn send(&self, event: CouncilEvent) -> Result<(), flume::SendError<CouncilEvent>> {
        self.tx.send(event)
    }

    /// Try to receive an event without blocking.
    pub fn try_recv(&self) -> Result<CouncilEvent, flume::TryRecvError> {
        self.rx.try_recv()
    }

    /// Blocking receive.
    pub fn recv(&self) -> Result<CouncilEvent, flume::RecvError> {
        self.rx.recv()
    }

    /// Returns a clone of the sender (for multiple producers).
    pub fn sender(&self) -> Sender<CouncilEvent> {
        self.tx.clone()
    }

    /// Returns a clone of the receiver (for multiple consumers / fan-out).
    pub fn receiver(&self) -> Receiver<CouncilEvent> {
        self.rx.clone()
    }
}

impl Default for CouncilEventBus {
    fn default() -> Self {
        Self::new_bounded(1024)
    }
}
