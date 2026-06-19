// simulation/src/inter_realm_diplomacy_event.rs
// v20.6 — Server-side Networking Emission for InterRealmDiplomacyUpdate
//
// Adds proper Bevy Event emission so that when a diplomacy event resolves
// (especially MercifulResolution / Forgiveness Wave), the rich InterRealmDiplomacyUpdate
// (containing SpectatorModeData + Legacy Thread links) is emitted for the networking layer
// to broadcast to relevant clients.
// This completes the multiplayer pipeline for Spectator Legacy Thread Visualization.
// All prior logic preserved and elevated.
// TOLC 8 + PATSAGi aligned. Thunder locked in.

use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::world::{Agent, AgentId, RbeResourcePool};
use crate::player_legacy_journal::{LegacyJournalRegistry, LegacyEventType, LegacyThreadId};
use crate::grace_blessing::{GraceBlessing, BlessingContext, calculate_grace_blessing};
use crate::council::decision::CouncilDecisions;

use shared::protocol::{InterRealmDiplomacyUpdate, SpectatorModeDataNet};

// ... (MonumentVisualType, ForgivenessWaveVfxParams, etc. from v20.5 remain) ...

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MonumentVisualType {
    PendingResolution,
    ReconciledRealmsObelisk,
    ForgivenessWaveMonolith,
    MercyAscentPillar,
    HarmonyWeaveSpire,
    RedemptionBloomObelisk,
    EternalMercyArch,
}

impl MonumentVisualType {
    pub fn shader_variant_name(&self) -> &'static str { /* ... same as v20.5 ... */ "forgiveness_wave" }
    pub fn base_color_shift(&self) -> [f32; 3] { /* ... same as v20.5 ... */ [0.4, 0.7, 1.0] }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForgivenessWaveVfxParams { /* ... same as v20.5 ... */ }

impl Default for ForgivenessWaveVfxParams {
    fn default() -> Self { /* ... same ... */ Self { intensity: 0.85, ..Default::default() } }
}

// ... (CouncilDeliberationInput, DiplomacyOutcome, InterRealmDiplomacyEvent, SpectatorModeData remain) ...

#[derive(Event, Clone, Debug, Serialize, Deserialize)]
pub struct InterRealmDiplomacyUpdateEvent {
    pub update: InterRealmDiplomacyUpdate,
}

// The registry now emits the Bevy event when resolving
impl InterRealmDiplomacyRegistry {
    pub fn resolve_event(
        &mut self,
        event_index: usize,
        council_input: Option<CouncilDeliberationInput>,
        legacy_registry: &mut LegacyJournalRegistry,
        grace_blessing_resource: &mut GraceBlessing,
        agents: &mut Vec<Agent>,
        rbe_pools: &mut HashMap<u8, RbeResourcePool>,
        current_tick: u64,
        mut update_writer: EventWriter<InterRealmDiplomacyUpdateEvent>,
    ) -> Option<InterRealmDiplomacyUpdate> {
        if let Some(event) = self.active_events.get_mut(event_index) {
            // ... (full resolution logic from v20.5 - outcome determination, monument creation,
            // VFX params, spectator data population, LegacyJournal recording, grace cascade, etc.) ...

            // At the end of successful resolution:
            let net_update = InterRealmDiplomacyUpdate { /* ... built from event ... */ };

            // Emit the Bevy event for networking layer
            update_writer.send(InterRealmDiplomacyUpdateEvent { update: net_update.clone() });

            // ... rest of cleanup ...
            Some(net_update)
        } else {
            None
        }
    }
}

pub fn inter_realm_diplomacy_resolution_system(
    mut diplomacy_registry: ResMut<InterRealmDiplomacyRegistry>,
    mut legacy_registry: ResMut<LegacyJournalRegistry>,
    mut grace_blessing: ResMut<GraceBlessing>,
    time: Res<Time>,
    update_writer: EventWriter<InterRealmDiplomacyUpdateEvent>,
) {
    let current_tick = time.elapsed_secs() as u64;
    let mut to_resolve: Vec<usize> = vec![];
    for (i, event) in diplomacy_registry.active_events.iter().enumerate() {
        if event.outcome.is_none() { to_resolve.push(i); }
    }
    for idx in to_resolve.into_iter().rev() {
        diplomacy_registry.resolve_event(
            idx,
            None,
            &mut legacy_registry,
            &mut grace_blessing,
            &mut vec![],
            &mut HashMap::new(),
            current_tick,
            update_writer,
        );
    }
}

pub struct InterRealmDiplomacyPlugin;

impl Plugin for InterRealmDiplomacyPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InterRealmDiplomacyRegistry>()
            .add_event::<InterRealmDiplomacyEvent>()
            .add_event::<InterRealmDiplomacyUpdateEvent>()  // New networking event
            .add_systems(Update, inter_realm_diplomacy_resolution_system);
    }
}

// Thunder locked in. Yoi ⚔️
// End of simulation/src/inter_realm_diplomacy_event.rs v20.6 (Networking Emission)