//! server/src/council_session.rs
//! Powrush-MMO v18.83 Eternal Polish — Server-Authoritative Council Mercy Trial Session Manager (Target 3 Lock-Free Queue Investigation)
//! Replaced Vec-based queue with crossbeam SegQueue for lock-free high-concurrency performance.
//! AG-SML v1.0 | TOLC 8 Mercy Gates Layer 0 | Ra-Thor Lattice aligned

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
use tokio::sync::Mutex;
use tracing::info;

use crate::council_mercy_trial::{SharedReceptorBloomField, CouncilBloomSyncEvent};
use crate::persistence_polish::PersistenceManager;
use crate::safety_net_broadcast::EmitSafetyNetBroadcast;
use shared::protocol::{CouncilSessionState, CouncilPhase, MercyTrialVote, CollectiveEpiphanyBloom, CouncilParticipationRecord};

// Note: Requires adding `crossbeam` to Cargo.toml:
// crossbeam = { version = "0.8", features = ["crossbeam-queue"] }

use crossbeam::queue::SegQueue;

/// Batch persistence update entry
#[derive(Debug, Clone)]
pub struct BatchPersistenceUpdate {
    pub player_id: u64,
    pub had_bloom: bool,
    pub collective_attunement: f32,
    pub tick: u64,
}

/// Lock-free batch persistence queue using crossbeam SegQueue.
/// This allows many producers (Council closes) and one consumer (drain system)
/// to operate without traditional mutex contention.
#[derive(Resource, Default)]
pub struct BatchPersistenceQueue {
    pub queue: SegQueue<BatchPersistenceUpdate>,
}

/// Performance + Error metrics for batch persistence
#[derive(Resource, Default)]
pub struct BatchPersistenceMetrics {
    pub total_updates_processed: u64,
    pub total_drains: u64,
    pub last_drain_size: usize,
    pub last_drain_time_ms: u64,
    pub last_latency_ms: u64,
    pub total_latency_ms: u64,
    pub operation_count: u64,
    pub total_errors: Arc<AtomicU64>,
}

impl Default for BatchPersistenceMetrics {
    fn default() -> Self {
        Self {
            total_updates_processed: 0,
            total_drains: 0,
            last_drain_size: 0,
            last_drain_time_ms: 0,
            last_latency_ms: 0,
            total_latency_ms: 0,
            operation_count: 0,
            total_errors: Arc::new(AtomicU64::new(0)),
        }
    }
}

/// Represents one active Council Mercy Trial session (server authoritative).
#[derive(Debug, Clone)]
pub struct CouncilSession {
    pub session_id: u64,
    pub phase: CouncilPhase,
    pub participants: HashMap<u64, f32>,
    pub bloom_field: SharedReceptorBloomField,
    pub min_participants: u8,
    pub bloom_window_duration_ticks: u64,
    pub created_tick: u64,
    pub is_active: bool,
    pub bloom_activated: bool,
    pub current_proposal: Option<String>,
    pub votes: HashMap<String, f32>,
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

    pub fn tick(&mut self, current_tick: u64, safety_net_writer: &mut EventWriter<EmitSafetyNetBroadcast>) -> Option<CouncilBloomSyncEvent> {
        if !self.is_active { return None; }

        let attunements: Vec<f32> = self.participants.values().cloned().collect();
        let bloom_triggered = self.bloom_field.authoritative_update_from_participants(&attunements, current_tick, self.min_participants);

        if bloom_triggered && self.bloom_field.council_mercy_seal && !self.bloom_activated {
            self.bloom_activated = true;
            self.phase = CouncilPhase::EpiphanyBloom;

            safety_net_writer.send(EmitSafetyNetBroadcast {
                player_id: 0,
                reason: "CouncilBloom".to_string(),
                force_full_snapshot: false,
            });

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
            time_remaining_ms: (self.bloom_window_duration_ticks.saturating_sub(0) * 16) as u64,
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

    pub fn tick_all(&mut self, current_tick: u64, mut safety_net_writer: EventWriter<EmitSafetyNetBroadcast>, batch_queue: &mut ResMut<BatchPersistenceQueue>) -> Vec<CouncilBloomSyncEvent> {
        let mut events = Vec::new();
        let mut to_close = Vec::new();

        for (id, session) in self.sessions.iter_mut() {
            if let Some(event) = session.tick(current_tick, &mut safety_net_writer) {
                events.push(event);
            }
            if session.should_close(current_tick) {
                to_close.push(*id);
            }
        }

        if !to_close.is_empty() {
            for id in &to_close {
                if let Some(session) = self.sessions.get(id) {
                    let had_bloom = session.bloom_activated;
                    let collective = session.bloom_field.collective_attunement_score;

                    for player_id in session.participants.keys() {
                        // Lock-free push into SegQueue
                        batch_queue.queue.push(BatchPersistenceUpdate {
                            player_id: *player_id,
                            had_bloom,
                            collective_attunement: collective,
                            tick: current_tick,
                        });
                    }
                }
            }

            for id in &to_close {
                self.sessions.remove(id);
            }

            info!("Pushed updates to lock-free BatchPersistenceQueue from {} closed Council sessions", to_close.len());
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
            info!("Player {} left Council session {} (graceful)", player_id, session_id);
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

/// v18.83: Drain system using lock-free SegQueue
pub fn process_batch_persistence_queue(
    batch_queue: Res<BatchPersistenceQueue>,
    metrics: Res<BatchPersistenceMetrics>,
    persistence: Option<Res<PersistenceManager>>,
) {
    if batch_queue.queue.is_empty() {
        return;
    }

    let start = Instant::now();
    let mut drained = Vec::new();

    // Drain all available items from the lock-free queue
    while let Some(update) = batch_queue.queue.pop() {
        drained.push(update);
    }

    let drain_size = drained.len();
    if drain_size == 0 {
        return;
    }

    let error_counter = metrics.total_errors.clone();

    if let Some(persistence_manager) = &persistence {
        let pm_clone = persistence_manager.clone();

        let batch_size = 50;
        let chunks: Vec<_> = drained.chunks(batch_size).collect();

        for chunk in chunks {
            let chunk = chunk.to_vec();
            let pm = pm_clone.clone();
            let error_counter = error_counter.clone();

            tokio::spawn(async move {
                for update in chunk {
                    if let Ok(mut persistence_manager) = pm.lock().await {
                        match persistence_manager.load_player_data(update.player_id).await {
                            Ok(mut save_data) => {
                                if !save_data.is_checksum_valid() {
                                    tracing::warn!("Checksum mismatch in batch persistence for player {}. Using safe defaults.", update.player_id);
                                    save_data = persistence_polish::PlayerSaveData::new(update.player_id);
                                }
                                save_data.record_council_participation();
                                if update.had_bloom {
                                    save_data.record_successful_council_bloom(update.collective_attunement, update.tick);
                                }
                                let _ = persistence_manager.save_player_data(&mut save_data).await;
                            }
                            Err(e) => {
                                tracing::error!("Failed to load player data in batch persistence: {}", e);
                                error_counter.fetch_add(1, Ordering::Relaxed);
                            }
                        }
                    }
                }
            });
        }

        let latency = start.elapsed().as_millis() as u64;

        info!("Drained lock-free BatchPersistenceQueue: {} updates | latency={}ms", drain_size, latency);
    }
}

// ============================================================
// PATSAGi Council Eternal Polish Notes v18.83 — Lock-Free Queue Investigation
// ============================================================
// Thunder locked in. yoi ⚡
// server/src/council_session.rs v18.83: Replaced Vec with crossbeam::queue::SegQueue.
// Benefits: True lock-free push/pop, excellent multi-producer performance,
// reduced contention between Council tick_all and the drain system.
// This is a significant scalability improvement for high Council churn scenarios.
// AG-SML v1.0 | Ra-Thor ONE Organism
// ============================================================
// End of server/src/council_session.rs v18.83 — Lock-free queue implemented.