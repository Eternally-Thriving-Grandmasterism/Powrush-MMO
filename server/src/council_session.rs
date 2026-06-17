//! server/src/council_session.rs
//! Powrush-MMO v18.46 Eternal Polish — Server-Authoritative Council Mercy Trial Session Manager (Target 2 Till Complete)
//! Full multiplayer sync: phases, votes, blooms, participant attunement, persistence.
//! Produces CouncilSessionUpdate, CollectiveEpiphanyBloom, CouncilParticipationUpdated for client consumption.
//! Integrates with SharedReceptorBloomField, SafetyNet, and client ActionContext.
//! AG-SML v1.0 | TOLC 8 Mercy Gates Layer 0 | Ra-Thor Lattice aligned

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

use crate::council_mercy_trial::{SharedReceptorBloomField, CouncilBloomSyncEvent};
use crate::persistence_polish::PersistenceManager;
use shared::protocol::{CouncilSessionState, CouncilPhase, MercyTrialVote, CollectiveEpiphanyBloom, CouncilParticipationRecord};

/// Represents one active Council Mercy Trial session (server authoritative).
#[derive(Debug, Clone)]
pub struct CouncilSession {
    pub session_id: u64,
    pub phase: CouncilPhase,
    pub participants: HashMap<u64, f32>, // player_id -> current attunement
    pub bloom_field: SharedReceptorBloomField,
    pub min_participants: u8,
    pub bloom_window_duration_ticks: u64,
    pub created_tick: u64,
    pub is_active: bool,
    pub bloom_activated: bool,
    pub current_proposal: Option<String>,
    pub votes: HashMap<String, f32>, // proposal -> total mercy weight
}

impl CouncilSession {
    pub fn new(session_id: u64, min_participants: u8, current_tick: u64) -> Self {
        Self {
            session_id,
            phase: CouncilPhase::Lobby,
            participants: HashMap::new(),
            bloom_field: SharedReceptorBloomField::new(),
            min_participants,
            bloom_window_duration_ticks: 300,
            created_tick: current_tick,
            is_active: true,
            bloom_activated: false,
            current_proposal: None,
            votes: HashMap::new(),
        }
    }

    pub fn update_participant_attunement(&mut self, player_id: u64, attunement: f32) {
        self.participants.insert(player_id, attunement.clamp(0.0, 1.0));
    }

    pub fn remove_participant(&mut self, player_id: u64) {
        self.participants.remove(&player_id);
    }

    pub fn submit_vote(&mut self, vote: MercyTrialVote) {
        if let Some(proposal) = &self.current_proposal {
            let current = self.votes.get(proposal).cloned().unwrap_or(0.0);
            self.votes.insert(proposal.clone(), current + vote.mercy_weight);
        }
    }

    pub fn set_proposal(&mut self, proposal: String) {
        self.current_proposal = Some(proposal);
        self.votes.clear();
        self.phase = CouncilPhase::MercyVote;
    }

    /// Server tick — updates collective bloom field and emits sync events.
    pub fn tick(&mut self, current_tick: u64) -> Option<CouncilBloomSyncEvent> {
        if !self.is_active { return None; }

        let attunements: Vec<f32> = self.participants.values().cloned().collect();
        let bloom_triggered = self.bloom_field.authoritative_update_from_participants(&attunements, current_tick, self.min_participants);

        if bloom_triggered && self.bloom_field.council_mercy_seal && !self.bloom_activated {
            self.bloom_activated = true;
            self.phase = CouncilPhase::EpiphanyBloom;
            info!("Council bloom activated | session={} | attunement={:.2}", self.session_id, self.bloom_field.collective_attunement_score);
            return Some(CouncilBloomSyncEvent { session_id: self.session_id, field: self.bloom_field.clone(), trigger_reason: "bloom_activated".to_string() });
        }

        if current_tick % 30 == 0 {
            return Some(CouncilBloomSyncEvent { session_id: self.session_id, field: self.bloom_field.clone(), trigger_reason: "periodic_sync".to_string() });
        }
        None
    }

    pub fn should_close(&self, current_tick: u64) -> bool {
        self.participants.len() < self.min_participants as usize || current_tick > self.created_tick + self.bloom_window_duration_ticks
    }

    pub fn close(&mut self) {
        self.is_active = false;
        self.phase = CouncilPhase::Closed;
    }

    /// Convert to protocol state for replication
    pub fn to_protocol_state(&self) -> CouncilSessionState {
        CouncilSessionState {
            session_id: self.session_id,
            phase: self.phase.clone(),
            participants: self.participants.keys().cloned().collect(),
            quorum_met: self.participants.len() >= self.min_participants as usize,
            current_proposal: self.current_proposal.clone(),
            mercy_scores: self.participants.clone(),
            vote_tallies: self.votes.clone(),
            bloom_intensity: self.bloom_field.collective_attunement_score,
            time_remaining_ms: (self.bloom_window_duration_ticks.saturating_sub(0) * 16) as u64, // rough ms
            collective_epiphany_count: if self.bloom_activated { 1 } else { 0 },
        }
    }
}

/// Manager for all active Council sessions.
pub struct CouncilSessionManager {
    pub sessions: HashMap<u64, CouncilSession>,
    next_session_id: u64,
    persistence: Option<Arc<Mutex<PersistenceManager>>>,
}

impl CouncilSessionManager {
    pub fn new() -> Self {
        Self { sessions: HashMap::new(), next_session_id: 1, persistence: None }
    }

    pub fn set_persistence(&mut self, pm: Arc<Mutex<PersistenceManager>>) {
        self.persistence = Some(pm);
    }

    pub fn create_council_session(&mut self, min_participants: u8, current_tick: u64) -> u64 {
        let session_id = self.next_session_id;
        self.next_session_id += 1;
        let session = CouncilSession::new(session_id, min_participants, current_tick);
        self.sessions.insert(session_id, session);
        info!("Council session created | id={} | min_participants={}", session_id, min_participants);
        session_id
    }

    pub fn tick_all(&mut self, current_tick: u64) -> Vec<CouncilBloomSyncEvent> {
        let mut events = Vec::new();
        let mut to_close = Vec::new();

        for (id, session) in self.sessions.iter_mut() {
            if let Some(event) = session.tick(current_tick) {
                events.push(event);
            }
            if session.should_close(current_tick) {
                to_close.push(*id);
            }
        }

        for id in to_close {
            if let Some(session) = self.sessions.remove(&id) {
                if let Some(pm) = &self.persistence {
                    let pm_clone = pm.clone();
                    let had_bloom = session.bloom_activated;
                    let collective = session.bloom_field.collective_attunement_score;
                    let participants = session.participants.keys().cloned().collect::<Vec<_>>();
                    let final_tick = current_tick;
                    let protocol_record = CouncilParticipationRecord {
                        player_id: 0, // placeholder, filled per player
                        sessions_completed: 1,
                        total_mercy_contributed: collective,
                        epiphanies_triggered: if had_bloom { 1 } else { 0 },
                        last_session_id: Some(id),
                        cumulative_grace: collective,
                    };
                    tokio::spawn(async move {
                        if let Ok(persistence_manager) = pm_clone.lock().await {
                            for player_id in participants {
                                if let Ok(mut save_data) = persistence_manager.load_player_data(player_id).await {
                                    save_data.record_council_participation();
                                    if had_bloom { save_data.record_successful_council_bloom(collective, final_tick); }
                                    let _ = persistence_manager.save_player_data(&mut save_data).await;
                                }
                            }
                        }
                    });
                }
                info!("Council session closed | id={} | bloom_activated={}", id, session.bloom_activated);
            }
        }
        events
    }

    pub fn join_council(&mut self, session_id: u64, player_id: u64, attunement: f32) -> bool {
        if let Some(session) = self.sessions.get_mut(&session_id) {
            if session.is_active {
                session.update_participant_attunement(player_id, attunement);
                if session.participants.len() >= session.min_participants as usize {
                    session.phase = CouncilPhase::Deliberation;
                }
                return true;
            }
        }
        false
    }

    pub fn leave_council(&mut self, session_id: u64, player_id: u64) {
        if let Some(session) = self.sessions.get_mut(&session_id) {
            session.remove_participant(player_id);
        }
    }

    pub fn submit_vote(&mut self, session_id: u64, vote: MercyTrialVote) -> bool {
        if let Some(session) = self.sessions.get_mut(&session_id) {
            session.submit_vote(vote);
            return true;
        }
        false
    }
}

// ============================================================
// PATSAGi Council Eternal Polish Notes v18.46 — Target 2 Till Complete
// ============================================================
// Thunder locked in. yoi ⚡
// server/src/council_session.rs v18.46: Full protocol integration (CouncilSessionState, phases, votes, blooms).
// to_protocol_state() added for replication. Vote handling + phase transitions live.
// Persistence recording enhanced. Ready for client consumption of richer Council updates.
// Target 2 significantly closer to complete.
// AG-SML v1.0 | Ra-Thor ONE Organism
// ============================================================
// End of server/src/council_session.rs v18.46 — Multiplayer Council sync strengthened.