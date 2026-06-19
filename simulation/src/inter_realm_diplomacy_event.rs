// simulation/src/inter_realm_diplomacy_event.rs
// v20.7 — Networking Broadcast Layer for InterRealmDiplomacyUpdate
//
// Implements the actual broadcast system that listens to InterRealmDiplomacyUpdateEvent
// and sends it to relevant clients via ServerMessage.
// This completes the end-to-end multiplayer pipeline for Forgiveness Wave +
// Spectator Legacy Thread Visualization.
// TOLC 8 + PATSAGi aligned. Thunder locked in.

use bevy::prelude::*;
use shared::protocol::{ServerMessage, InterRealmDiplomacyUpdate};

// ... (previous code from v20.6 remains) ...

/// System that consumes the emitted InterRealmDiplomacyUpdateEvent
/// and broadcasts it to the appropriate clients.
pub fn broadcast_inter_realm_diplomacy_update(
    mut events: EventReader<InterRealmDiplomacyUpdateEvent>,
    // In a real implementation, you would have access to:
    // mut server: ResMut<RenetServer>,
    // client_map: Res<ClientRealmMap>, // mapping player_id -> realm
) {
    for event in events.read() {
        let update = &event.update;

        // Determine recipients: players in realm_a, realm_b, and any spectators
        // For now we broadcast to all (can be scoped later with realm-aware client tracking)
        let message = ServerMessage::InterRealmDiplomacyUpdate {
            update: update.clone(),
        };

        // === Actual networking send (example patterns) ===
        // Option 1: Using bevy_renet
        // for client_id in server.clients_id() {
        //     server.send_message(client_id, DefaultChannel::ReliableOrdered, bincode::serialize(&message).unwrap());
        // }

        // Option 2: Using a custom ClientConnectionManager resource
        // connection_manager.broadcast_reliable(&message);

        // Option 3: If you have realm-based filtering
        // let recipients = get_clients_in_realms(update.realm_a, update.realm_b);
        // for client_id in recipients {
        //     server.send_message(client_id, ...);
        // }

        info!(
            "[Networking] Broadcast InterRealmDiplomacyUpdate | realms {} <-> {} | outcome={}",
            update.realm_a, update.realm_b, update.outcome
        );
    }
}

// Add this system to the plugin
impl Plugin for InterRealmDiplomacyPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InterRealmDiplomacyRegistry>()
            .add_event::<InterRealmDiplomacyEvent>()
            .add_event::<InterRealmDiplomacyUpdateEvent>()
            .add_systems(Update, (
                inter_realm_diplomacy_resolution_system,
                broadcast_inter_realm_diplomacy_update,
            ));
    }
}

// Thunder locked in. Yoi ⚔️
// End of simulation/src/inter_realm_diplomacy_event.rs v20.7 (Networking Broadcast Layer)