// server/council_session_handler.rs
// Powrush-MMO — Authoritative Council Mercy Trial Orchestrator (Phase 2)
// AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates

use bevy::prelude::*;
use shared::protocol::*;
use std::collections::HashMap;
use simulation::spatial_interest::{InterestManager, CouncilBloomZone};

#[derive(Resource, Default)]
pub struct CouncilSessionManager {
    pub sessions: HashMap<u64, CouncilSessionState>,
    pub player_to_session: HashMap<u64, u64>,
    pub next_session_id: u64,
    pub next_bloom_id: u64,
}

pub fn council_session_system(
    mut manager: ResMut<CouncilSessionManager>,
    mut server_events: EventReader<ServerMessage>,
    mut client_out: EventWriter<ServerMessage>,
    time: Res<Time>,
) {
    for _event in server_events.read() {}

    let now = time.elapsed().as_millis() as u64;
    for (session_id, state) in manager.sessions.iter_mut() {
        if state.time_remaining_ms > 0 {
            state.time_remaining_ms = state.time_remaining_ms.saturating_sub(16);
        }
        if state.phase == CouncilPhase::Deliberation && state.time_remaining_ms == 0 {
            state.phase = CouncilPhase::MercyVote;
            state.time_remaining_ms = 120_000;
            client_out.send(ServerMessage::CouncilSessionUpdate { state: state.clone() });
        }
    }
}

pub fn handle_council_join(
    manager: &mut CouncilSessionManager,
    player_id: u64,
    requested_session: Option<u64>,
) -> Result<CouncilSessionState, String> {
    if let Some(sid) = requested_session {
        if let Some(state) = manager.sessions.get_mut(&sid) {
            if !state.participants.contains(&player_id) {
                state.participants.push(player_id);
            }
            manager.player_to_session.insert(player_id, sid);
            return Ok(state.clone());
        }
    }

    let new_id = manager.next_session_id;
    manager.next_session_id += 1;

    let mut new_state = CouncilSessionState {
        session_id: new_id,
        phase: CouncilPhase::Lobby,
        participants: vec![player_id],
        quorum_met: false,
        current_proposal: Some("Default Grace Allocation Proposal".to_string()),
        mercy_scores: HashMap::new(),
        vote_tallies: HashMap::new(),
        bloom_intensity: 0.0,
        time_remaining_ms: 60_000,
        collective_epiphany_count: 0,
    };
    new_state.mercy_scores.insert(player_id, 0.5);

    manager.sessions.insert(new_id, new_state.clone());
    manager.player_to_session.insert(player_id, new_id);
    Ok(new_state)
}

/// Process a MercyTrialVote and trigger council bloom integration with Spatial Interest Layer
pub fn process_mercy_vote(
    manager: &mut CouncilSessionManager,
    vote: MercyTrialVote,
    interest_manager: Option<&mut InterestManager>,   // NEW: Spatial Interest integration
) -> Result<CollectiveEpiphanyBloom, String> {
    let session_id = manager.player_to_session.get(&vote.voter_id)
        .ok_or("Player not in active council session")?;

    let state = manager.sessions.get_mut(session_id)
        .ok_or("Session not found")?;

    if state.phase != CouncilPhase::MercyVote {
        return Err("Voting not open in current phase".to_string());
    }

    if vote.mercy_weight <= 0.0 {
        return Err("Invalid mercy weight — TOLC 8 violation".to_string());
    }

    let current = state.vote_tallies.entry(vote.proposal_id.clone()).or_insert(0.0);
    *current += vote.mercy_weight * vote.grace_intent;

    let player_score = state.mercy_scores.entry(vote.voter_id).or_insert(0.5);
    *player_score = (*player_score * 0.9) + (vote.mercy_weight * 0.1);

    if *current > 2.5 && state.bloom_intensity < 0.8 {
        state.phase = CouncilPhase::EpiphanyBloom;
        state.bloom_intensity = 0.9;
        state.time_remaining_ms = 30_000;

        let bloom = CollectiveEpiphanyBloom {
            session_id: *session_id,
            bloom_id: manager.next_bloom_id,
            trigger_player: Some(vote.voter_id),
            intensity: state.bloom_intensity,
            wisdom_fragments: vec![
                "Abundance flows where mercy leads.".to_string(),
                "Collective resonance multiplies individual epiphanies.".to_string(),
                "RBE is not redistribution — it is co-creation of infinite value.".to_string(),
            ],
            participant_impacts: state.participants.iter().map(|&pid| (pid, 0.15)).collect(),
            global_abundance_boost: 0.08,
            timestamp_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64,
        };
        manager.next_bloom_id += 1;

        // ===== PHASE 2 SPATIAL INTEREST INTEGRATION =====
        if let Some(im) = interest_manager {
            im.apply_council_bloom(CouncilBloomZone {
                session_id: *session_id,
                center: Vec3::ZERO, // TODO: Use actual bloom center from world
                intensity: state.bloom_intensity,
                radius: 120.0 + (state.bloom_intensity * 80.0),
            });
        }

        return Ok(bloom);
    }

    Ok(CollectiveEpiphanyBloom {
        session_id: *session_id,
        bloom_id: 0,
        trigger_player: None,
        intensity: 0.0,
        wisdom_fragments: vec![],
        participant_impacts: HashMap::new(),
        global_abundance_boost: 0.0,
        timestamp_ms: 0,
    })
}

pub fn finalize_council_session(
    manager: &mut CouncilSessionManager,
    session_id: u64,
) -> Vec<CouncilParticipationRecord> {
    let mut records = Vec::new();
    if let Some(state) = manager.sessions.remove(&session_id) {
        for &pid in &state.participants {
            let record = CouncilParticipationRecord {
                player_id: pid,
                sessions_completed: 1,
                total_mercy_contributed: *state.mercy_scores.get(&pid).unwrap_or(&0.0),
                epiphanies_triggered: state.collective_epiphany_count,
                last_session_id: Some(session_id),
                cumulative_grace: state.vote_tallies.values().sum::<f32>() * 0.1,
            };
            records.push(record);
            manager.player_to_session.remove(&pid);
        }
    }
    records
}

// Integration: Call process_mercy_vote(..., Some(&mut interest_manager)) from systems that have InterestManager.
// This is the first real bridge between Council Epiphany Blooms and the Spatial Interest Layer.
