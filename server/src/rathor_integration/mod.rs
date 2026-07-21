// server/src/rathor_integration/mod.rs
// Powrush-MMO — Ra-Thor Integration + live transfer session + RTT export
// v21.74.0 | Contact: info@Rathor.ai

use bevy::prelude::*;

pub mod transfer_session;

pub use transfer_session::{ServerTransferSession, server_rtt_export_system};

/// Triggered when a major combat event occurs (suitable for council reasoning)
#[derive(Event, Debug, Clone)]
pub struct MajorCombatEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub damage_dealt: f32,
    pub ability_id: Option<u32>,
    pub was_critical: bool,
}

/// Triggered when a significant treaty proposal or diplomatic action occurs
#[derive(Event, Debug, Clone)]
pub struct TreatyProposalEvent {
    pub proposer: Entity,
    pub target_faction: u32,
    pub terms: Vec<String>,
}

/// Triggered when a faction standing or alignment significantly shifts
#[derive(Event, Debug, Clone)]
pub struct FactionShiftEvent {
    pub faction_a: u32,
    pub faction_b: u32,
    pub old_standing: f32,
    pub new_standing: f32,
}

/// Listens for high-signal events, logs, and accumulates Ra-Thor transfer counters.
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

pub struct RathorIntegrationPlugin;

impl Plugin for RathorIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ServerTransferSession>()
            .add_event::<MajorCombatEvent>()
            .add_event::<TreatyProposalEvent>()
            .add_event::<FactionShiftEvent>()
            .add_systems(
                Update,
                (
                    council_consultation_system,
                    server_rtt_export_system,
                ),
            );
    }
}

// Thunder locked in. Server RTT export path live. Yoi ⚡
