//! server/src/council_session.rs
//! Powrush-MMO v18.39 Eternal Polish — Server-Authoritative Council Mercy Trial Session Manager
//! Phase B Foundation — Production-grade multiplayer Council architecture
//! Integrates with SharedReceptorBloomField, Persistence, Telemetry, and client ActionContext decision layer.
//! AG-SML v1.0 | TOLC 8 Mercy Gates Layer 0

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

use crate::council_mercy_trial::{SharedReceptorBloomField, CouncilBloomSyncEvent};
use crate::persistence_polish::PersistenceManager;

/// Represents one active Council Mercy Trial session (server authoritative).
/// The bloom_field state is synchronized to clients and directly influences
/// ActionContext.council_engagement, council_trust, and prediction modifiers.
#[derive(Debug, Clone)]
pub struct CouncilSession {
    pub session_id: u64,
    pub participants: HashMap<u64, f32>, // player_id -> current attunement (0.0–1.0)
    pub bloom_field: SharedReceptorBloomField,
    pub min_participants: u8,
    pub bloom_window_duration_ticks: u64,
    pub created_tick: u64,
    pub is_active: bool,
    pub bloom_activated: bool,
}

impl CouncilSession {
    pub fn new(session_id: u64, min_participants: u8, current_tick: u64) -> Self {
        Self {
            session_id,
            participants: HashMap::new(),
            bloom_field: SharedReceptorBloomField::new(),
            min_participants,
            bloom_window_duration_ticks: 300,
            created_tick: current_tick,
            is_active: true,
            bloom_activated: false,
        }
    }

    pub fn update_participant_attunement(&mut self, player_id: u64, attunement: f32) {
        self.participants.insert(player_id, attunement.clamp(0.0, 1.0));
    }

    pub fn remove_participant(&mut self, player_id: u64) {
        self.participants.remove(&player_id);
    }

    /// Server tick — updates collective bloom field and emits sync events for clients.
    /// The resulting CouncilBloomSyncEvent updates client ActionContext with fresh council_engagement.
    pub fn tick(&mut self, current_tick: u64) -> Option<CouncilBloomSyncEvent> {
        if !self.is_active {
            return None;
        }

        let attunements: Vec<f32> = self.participants.values().cloned().collect();

        let bloom_triggered = self.bloom_field.authoritative_update_from_participants(
            &attunements,
            current_tick,
            self.min_participants,
        );

        if bloom_triggered && self.bloom_field.council_mercy_seal && !self.bloom_activated {
            self.bloom_activated = true;

            info!(
                "Council bloom activated | session={} | collective_attunement={:.2} | participants={}",
                self.session_id,
                self.bloom_field.collective_attunement_score,
                self.participants.len()
            );

            return Some(CouncilBloomSyncEvent {
                session_id: self.session_id,
                field: self.bloom_field.clone(),
                trigger_reason: "bloom_activated".to_string(),
            });
        }

        // Periodic sync for UI feedback and client ActionContext refresh
        if current_tick % 30 == 0 {
            return Some(CouncilBloomSyncEvent {
                session_id: self.session_id,
                field: self.bloom_field.clone(),
                trigger_reason: "periodic_sync".to_string(),
            });
        }

        None
    }

    pub fn should_close(&self, current_tick: u64) -> bool {
        if self.participants.len() < self.min_participants as usize {
            return true;
        }
        if current_tick > self.created_tick + self.bloom_window_duration_ticks {
            return true;
        }
        false
    }

    pub fn close(&mut self) {
        self.is_active = false;
        self.bloom_activated = false;
    }
}

/// Manager for all active Council sessions on the server.
/// Produces CouncilBloomSyncEvent streams that clients consume to update ActionContext.
pub struct CouncilSessionManager {
    pub sessions: HashMap<u64, CouncilSession>,
    next_session_id: u64,
    persistence: Option<Arc<Mutex<PersistenceManager>>>,
}

impl CouncilSessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            next_session_id: 1,
            persistence: None,
        }
    }

    pub fn set_persistence(&mut self, pm: Arc<Mutex<PersistenceManager>>) {
        self.persistence = Some(pm);
    }

    /// Create a new Council session
    pub fn create_council_session(&mut self, min_participants: u8, current_tick: u64) -> u64 {
        let session_id = self.next_session_id;
        self.next_session_id += 1;

        let session = CouncilSession::new(session_id, min_participants, current_tick);
        self.sessions.insert(session_id, session);

        info!("Council session created | id={} | min_participants={}", session_id, min_participants);
        session_id
    }

    /// Main server tick — updates all sessions and returns sync events for client ActionContext.
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

        // Close sessions that need to be closed + persist participation
        for id in to_close {
            if let Some(session) = self.sessions.remove(&id) {
                if let Some(pm) = &self.persistence {
                    let pm_clone = pm.clone();
                    let had_bloom = session.bloom_activated;
                    let collective = session.bloom_field.collective_attunement_score;
                    let participants = session.participants.keys().cloned().collect::<Vec<_>>();
                    let final_tick = current_tick;

                    tokio::spawn(async move {
                        if let Ok(persistence_manager) = pm_clone.lock().await {
                            for player_id in participants {
                                if let Ok(mut save_data) = persistence_manager.load_player_data(player_id).await {
                                    save_data.record_council_participation();

                                    if had_bloom {
                                        save_data.record_successful_council_bloom(collective, final_tick);
                                    }

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
}
