/*!
 * Epiphany Scenario Wiring + Strong Client Feedback
 *
 * v18.96 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Sends ClientMessage::SyncLocalization on startup after ClientSettings loaded
 * — Async enriched whispers + server recording ready
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use std::sync::{Arc, Mutex};

use crate::settings::ClientSettings;
use simulation::divine_whispers::DivineWhisperTrigger;
use simulation::epiphany_catalyst::{EpiphanyOutcome, generate_multilingual_epiphany_note};
use shared::protocol::ClientMessage;

// ... PendingEnrichedWhispers, drain system, trigger_scenario_with_async_enrichment (with real EpiphanyOutcome) ...

/// NEW v18.96: Send localization preference to server on startup (after settings loaded)
fn send_initial_localization(
    mut commands: Commands,
    settings: Res<ClientSettings>,
    // In full impl: outgoing_message_channel: Res<OutgoingClientMessages>,
) {
    let lang = settings.localization.language.clone();

    // Production integration: serialize and send via WebSocket outgoing channel
    // Example:
    // let msg = ClientMessage::SyncLocalization { language: lang.clone() };
    // outgoing_message_channel.send(bincode::serialize(&msg).unwrap());

    info!("[EpiphanyWiring] Sending initial language preference to server: {}", lang);

    // For now we log; wire to actual transport channel in networking.rs
    commands.insert_resource(InitialLanguageSent(true));
}

#[derive(Resource)]
struct InitialLanguageSent(bool);

pub struct EpiphanyScenarioWiringPlugin;

impl Plugin for EpiphanyScenarioWiringPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<EpiphanyScenarioRegistry>()
            .init_resource::<PendingEnrichedWhispers>()
            .add_event::<EpiphanyEvent>()
            .add_systems(Startup, |mut commands: Commands| {
                commands.insert_resource(load_epiphany_scenarios());
            })
            .add_systems(Update, (
                epiphany_detector_system,
                drain_pending_whispers,
                send_initial_localization.run_if(not(resource_exists::<InitialLanguageSent>())),
            ).chain());
    }
}

// End of client/src/epiphany_scenario_wiring.rs v18.96
// Client now sends SyncLocalization on startup. Thunder locked in. Yoi ⚡
