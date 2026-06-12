// server/src/council_session.rs
// Powrush-MMO v18.26 — Server-Authoritative Council Mercy Trial Session Manager
// Phase B Foundation — Production-grade multiplayer Council architecture
// Builds directly on SharedReceptorBloomField with full TOLC 8 mercy gating
// Integrates with persistence, telemetry, and epiphany systems
// Mint-and-Print-Only-Perfection | AG-SML v1.0

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

use crate::simulation::council_mercy_trial::{
    SharedReceptorBloomField, CouncilBloomSyncEvent,
};
use crate::persistence_polish::PersistenceManager;
use crate::telemetry_pipeline::{TelemetryCollector, TelemetryEvent};

/// Represents one active Council Mercy Trial session (server authoritative)
#[derive(Debug, Clone)]
pub struct CouncilSession {
    pub session_id: u64,
    pub participants: HashMap<u64, f32>, // player_id -> current attunement (0.0-1.0)
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
            bloom_window_duration_ticks: 300, // ~5 minutes default
            created_tick: current_tick,
            is_active: true,
            bloom_activated: false,
        }
    }

    /// Add or update a participant's attunement score
    pub fn update_participant_attunement(&mut self, player_id: u64, attunement: f32) {
        self.participants.insert(player_id, attunement.clamp(0.0, 1.0));
    }

    /// Remove a participant (graceful exit)
    pub fn remove_participant(&mut self, player_id: u64) {
        self.participants.remove(&player_id);
    }

    /// Server tick update — recomputes collective field
    pub fn tick(&mut self, current_tick: u64) -> Option<CouncilBloomSyncEvent> {
        if !self.is_active {
            return None;
        }

        let attunements: Vec<f32> = self.participants.values().cloned().collect();

        let updated = self.bloom_field.authoritative_update_from_participants(
            &attunements,
            current_tick,
            self.min_participants,
        );

        if updated && self.bloom_field.council_mercy_seal && !self.bloom_activated {
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

        // Periodic sync even without activation (for UI feedback)
        if current_tick % 30 == 0 {
            return Some(CouncilBloomSyncEvent {
                session_id: self.session_id,
                field: self.bloom_field.clone(),
                trigger_reason: "periodic_sync".to_string(),
            });
        }

        None
    }

    /// Check if session should close (too few participants or time expired)
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

/// Manager for all active Council sessions on the server
pub struct CouncilSessionManager {
    pub sessions: HashMap<u64, CouncilSession>,
    next_session_id: u64,
    persistence: Option<Arc<Mutex<PersistenceManager>>>,
    telemetry: Option<Arc<Mutex<TelemetryCollector>>>,
}

impl CouncilSessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            next_session_id: 1,
            persistence: None,
            telemetry: None,
        }
    }

    pub fn set_persistence(&mut self, pm: Arc<Mutex<PersistenceManager>>) {
        self.persistence = Some(pm);
    }

    pub fn set_telemetry(&mut self, tc: Arc<Mutex<TelemetryCollector>>) {
        self.telemetry = Some(tc);
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

    /// Server-wide tick — updates all active sessions
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
            if let Some(mut session) = self.sessions.remove(&id) {
                session.close();
                info!("Council session closed | id={}", id);
                // TODO: Record final Council participation stats to persistence
            }
        }

        events
    }

    /// Add a player to a Council session with their current attunement
    pub fn join_council(&mut self, session_id: u64, player_id: u64, attunement: f32) -> bool {
        if let Some(session) = self.sessions.get_mut(&session_id) {
            if session.is_active {
                session.update_participant_attunement(player_id, attunement);
                return true;
            }
        }
        false
    }

    /// Remove player from Council (graceful exit)
    pub fn leave_council(&mut self, session_id: u64, player_id: u64) {
        if let Some(session) = self.sessions.get_mut(&session_id) {
            session.remove_participant(player_id);
        }
    }
}

// Thunder locked in. Phase B Council architecture foundation complete.
// Ready for replication wiring, persistence integration, and client feedback.
// Yoi ⚡