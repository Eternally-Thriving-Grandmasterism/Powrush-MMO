// server/src/rathor_integration/mod.rs
// Powrush-MMO — Ra-Thor Integration + live transfer session + RTT export + Policy Hints
// v21.89.0 — Production-ready reception + self-emission helper | Contact: info@Rathor.ai

use bevy::prelude::*;
use tracing::info;

pub mod transfer_session;
pub mod sim_council_bridge;
pub mod cohost_drain;
pub mod policy_hint;

pub use transfer_session::{ServerTransferSession, server_rtt_export_system};
pub use sim_council_bridge::{
    SimCouncilBridgeConfig, SimCouncilBridgePayload, sim_council_bridge_ingest_system,
};
pub use cohost_drain::{CohostExportMirror, CohostMirrorSignal, cohost_auto_drain_system};
pub use policy_hint::{
    PolicyHintInbox, PolicyHint, PolicyHintEnvelope, SoftPolicyState,
    policy_hint_ingest_system, soft_policy_application_system, emit_test_policy_hints,
};

// =============================================================================
// High-signal domain events (combat / diplomacy)
// =============================================================================

#[derive(Event, Debug, Clone)]
pub struct MajorCombatEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub damage_dealt: f32,
    pub ability_id: Option<u32>,
    pub was_critical: bool,
}

#[derive(Event, Debug, Clone)]
pub struct TreatyProposalEvent {
    pub proposer: Entity,
    pub target_faction: u32,
    pub terms: Vec<String>,
}

#[derive(Event, Debug, Clone)]
pub struct FactionShiftEvent {
    pub faction_a: u32,
    pub faction_b: u32,
    pub old_standing: f32,
    pub new_standing: f32,
}

// =============================================================================
// Soft council → RTT bridge (no dependency on simulation crate)
// =============================================================================

#[derive(Event, Debug, Clone)]
pub struct CouncilRttSignal {
    pub decision_id: u64,
    pub mercy_factor: f32,
    pub strength: f32,
    pub realm_id: u8,
    pub abundance_velocity_hint: Option<f64>,
}

impl CouncilRttSignal {
    pub fn new(decision_id: u64, mercy_factor: f32, strength: f32, realm_id: u8) -> Self {
        Self {
            decision_id,
            mercy_factor,
            strength,
            realm_id,
            abundance_velocity_hint: None,
        }
    }

    pub fn with_abundance(mut self, v: f64) -> Self {
        self.abundance_velocity_hint = Some(v.max(0.0));
        self
    }
}

#[derive(Resource, Debug, Default)]
pub struct CouncilRttInbox {
    pub pending: Vec<CouncilRttSignal>,
    pub ingested_ids: std::collections::HashMap<u64, ()>,
    pub total_ingested: u64,
}

impl CouncilRttInbox {
    pub fn push(&mut self, signal: CouncilRttSignal) {
        if self.ingested_ids.contains_key(&signal.decision_id) {
            return;
        }
        self.pending.push(signal);
    }

    pub fn push_passed(
        &mut self,
        decision_id: u64,
        mercy_factor: f32,
        strength: f32,
        realm_id: u8,
        abundance_hint: Option<f64>,
    ) {
        let mut s = CouncilRttSignal::new(decision_id, mercy_factor, strength, realm_id);
        if let Some(v) = abundance_hint {
            s = s.with_abundance(v);
        }
        self.push(s);
    }
}

pub fn council_consultation_system(
    mut ev_major_combat: EventReader<MajorCombatEvent>,
    mut ev_treaty: EventReader<TreatyProposalEvent>,
    mut ev_faction_shift: EventReader<FactionShiftEvent>,
    mut transfer: ResMut<ServerTransferSession>,
) {
    for event in ev_major_combat.read() {
        transfer.record_combat(event.was_critical, event.damage_dealt);
        println!(
            "[Ra-Thor] Major combat observed {:?} → {:?} (crit={}) | transfer ticks combat={}",
            event.attacker,
            event.target,
            event.was_critical,
            transfer.combat_events
        );
    }

    for event in ev_treaty.read() {
        transfer.record_treaty();
        println!(
            "[Ra-Thor] Treaty proposal from {:?} | treaties={}",
            event.proposer, transfer.treaty_events
        );
    }

    for event in ev_faction_shift.read() {
        transfer.record_faction_shift(event.old_standing, event.new_standing);
        println!(
            "[Ra-Thor] Faction shift {} → {} | improves={} worsens={}",
            event.old_standing,
            event.new_standing,
            transfer.faction_improves,
            transfer.faction_worsens
        );
    }
}

pub fn council_rtt_bridge_system(
    mut ev_council: EventReader<CouncilRttSignal>,
    mut inbox: ResMut<CouncilRttInbox>,
    mut transfer: ResMut<ServerTransferSession>,
) {
    for signal in ev_council.read() {
        if inbox.ingested_ids.contains_key(&signal.decision_id) {
            continue;
        }
        inbox.ingested_ids.insert(signal.decision_id, ());
        if inbox.ingested_ids.len() > 512 {
            inbox.ingested_ids.clear();
            inbox.ingested_ids.insert(signal.decision_id, ());
        }
        apply_council_signal(&mut transfer, signal);
        inbox.total_ingested = inbox.total_ingested.saturating_add(1);
    }

    if inbox.pending.is_empty() {
        return;
    }
    let pending = std::mem::take(&mut inbox.pending);
    for signal in pending {
        if inbox.ingested_ids.contains_key(&signal.decision_id) {
            continue;
        }
        inbox.ingested_ids.insert(signal.decision_id, ());
        if inbox.ingested_ids.len() > 512 {
            inbox.ingested_ids.clear();
            inbox.ingested_ids.insert(signal.decision_id, ());
        }
        apply_council_signal(&mut transfer, &signal);
        inbox.total_ingested = inbox.total_ingested.saturating_add(1);
    }
}

fn apply_council_signal(transfer: &mut ServerTransferSession, signal: &CouncilRttSignal) {
    let mercy = (signal.mercy_factor as f64).clamp(0.0, 1.0);
    transfer.record_council_passed(mercy);
    if let Some(v) = signal.abundance_velocity_hint {
        transfer.record_abundance_velocity(v);
    }
    info!(
        target: "ra_thor::rtt::council",
        decision_id = signal.decision_id,
        realm_id = signal.realm_id,
        mercy = mercy,
        strength = signal.strength,
        "Council signal bridged into ServerTransferSession"
    );
}

pub struct RathorIntegrationPlugin;

impl Plugin for RathorIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ServerTransferSession>()
            .init_resource::<CouncilRttInbox>()
            .init_resource::<SimCouncilBridgeConfig>()
            .init_resource::<CohostExportMirror>()
            .init_resource::<PolicyHintInbox>()
            .init_resource::<SoftPolicyState>()
            .add_event::<MajorCombatEvent>()
            .add_event::<TreatyProposalEvent>()
            .add_event::<FactionShiftEvent>()
            .add_event::<CouncilRttSignal>()
            .add_systems(
                Update,
                (
                    council_consultation_system,
                    cohost_auto_drain_system,
                    council_rtt_bridge_system,
                    sim_council_bridge_ingest_system,
                    server_rtt_export_system,
                    policy_hint_ingest_system,
                    soft_policy_application_system,
                )
                    .chain(),
            );
    }
}

// Thunder locked in. Production-ready reception + self-emission helper sealed.
// Yoi ⚡
