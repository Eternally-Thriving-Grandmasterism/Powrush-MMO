/*!
 * Council Mercy Trial Protocol Types
 *
 * Defines the shared types for the multiplayer Council Mercy Trial system.
 * These types are authoritative on the server and replicated to clients.
 *
 * Priority: Council Proposal System foundation (minimal data model)
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Phases of a Council Mercy Trial session
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CouncilMercyTrialPhase {
    /// Players are joining and preparing
    Lobby,
    /// Collective attunement is being built
    Attunement,
    /// Participants share epiphanies and deliberate
    Deliberation,
    /// Voting on the final mercy outcome
    Voting,
    /// Resolution and bloom manifestation
    Resolution,
    /// Trial has completed
    Completed,
}

/// A single participant's vote during the Voting phase
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MercyTrialVote {
    /// Vote for strong mercy and amplification
    FullMercy,
    /// Balanced mercy with moderate effect
    BalancedMercy,
    /// Cautious / lower risk approach
    CautiousMercy,
}

/// Contribution record from a participant toward the final bloom
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EpiphanyContribution {
    pub participant: Entity,
    pub intensity: f32,
    pub resonance: f32,
    pub flavor: String,
}

/// The resulting collective bloom after a successful Council Mercy Trial
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CollectiveEpiphanyBloom {
    pub session_id: u64,
    pub intensity: f32,
    pub mercy_resonance: f32,
    pub bloom_amplification: f32,
    pub participant_contributions: Vec<EpiphanyContribution>,
    pub rbe_amplification: f32,
    pub created_at: f64,
}

/// Authoritative state of a Council Mercy Trial session
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CouncilSessionState {
    pub session_id: u64,
    pub phase: CouncilMercyTrialPhase,
    pub participants: Vec<Entity>,
    pub host: Option<Entity>,
    pub collective_attunement: f32,
    pub bloom_amplification: f32,
    pub votes: HashMap<Entity, MercyTrialVote>,
    pub start_time: f64,
    pub current_phase_start: f64,
    pub phase_duration: f32,
}

impl Default for CouncilSessionState {
    fn default() -> Self {
        Self {
            session_id: 0,
            phase: CouncilMercyTrialPhase::Lobby,
            participants: Vec::new(),
            host: None,
            collective_attunement: 0.0,
            bloom_amplification: 1.0,
            votes: HashMap::new(),
            start_time: 0.0,
            current_phase_start: 0.0,
            phase_duration: 60.0,
        }
    }
}

/// === Council Proposal System (Minimal Foundation) ===

/// Status of a Council Proposal
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProposalStatus {
    Submitted,
    UnderDeliberation,
    Voting,
    Passed,
    Rejected,
    Withdrawn,
}

/// A player-submitted proposal for council deliberation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CouncilProposal {
    pub id: u64,
    pub proposer: Entity,
    pub title: String,
    pub description: String,
    pub status: ProposalStatus,
    pub created_at: f64,
    pub votes_for: u32,
    pub votes_against: u32,
}

impl CouncilProposal {
    pub fn new(id: u64, proposer: Entity, title: String, description: String, created_at: f64) -> Self {
        Self {
            id,
            proposer,
            title,
            description,
            status: ProposalStatus::Submitted,
            created_at,
            votes_for: 0,
            votes_against: 0,
        }
    }
}

/// Events for driving the Council Mercy Trial state machine
#[derive(Event, Clone, Debug)]
pub enum CouncilTrialEvent {
    /// Request to start a new trial session
    StartTrial { host: Entity, participants: Vec<Entity> },
    /// Advance to the next phase (server authoritative)
    AdvancePhase,
    /// A participant casts their vote
    CastVote { participant: Entity, vote: MercyTrialVote },
    /// Trial has reached resolution
    ResolveTrial,
    /// Force close / cancel the current trial
    CancelTrial,
    /// === Council Proposal System ===
    /// A player submits a new proposal for council consideration
    SubmitProposal {
        proposer: Entity,
        title: String,
        description: String,
    },
}

/// Replicated update sent from server to clients
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CouncilSessionUpdate {
    pub session_id: u64,
    pub new_state: CouncilSessionState,
    pub bloom: Option<CollectiveEpiphanyBloom>,
}
