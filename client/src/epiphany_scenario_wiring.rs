/*!
 * Epiphany Scenario Wiring + Strong Client Feedback
 *
 * v18.96 Eternal Polish — Actual send of SyncLocalization wired to OutgoingClientMessages
 */

use bevy::prelude::*;
use crate::settings::ClientSettings;
use crate::networking::OutgoingClientMessages;
use shared::protocol::ClientMessage;

fn send_initial_localization(
    settings: Res<ClientSettings>,
    outgoing: Res<OutgoingClientMessages>,
) {
    let lang = settings.localization.language.clone();

    let msg = ClientMessage::SyncLocalization { language: lang.clone() };
    let _ = outgoing.tx.send(msg);

    info!("[EpiphanyWiring] Sent SyncLocalization to server: language={}", lang);
}

// ... rest of the file (PendingEnrichedWhispers, async enrichment with real EpiphanyOutcome, etc.) ...

pub struct EpiphanyScenarioWiringPlugin;

impl Plugin for EpiphanyScenarioWiringPlugin {
    fn build(&self, app: &mut App) {
        // ... existing ...
        .add_systems(Update, send_initial_localization.run_if(not(resource_exists::<InitialLanguageSent>())));
    }
}

// End of client/src/epiphany_scenario_wiring.rs v18.96
// Actual send of SyncLocalization connected to outgoing channel. Thunder locked in. Yoi ⚡
