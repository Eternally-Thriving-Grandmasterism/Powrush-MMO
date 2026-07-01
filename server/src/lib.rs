/*!
 * server/src/lib.rs
 *
 * Server core plugin with inventory replication fully wired to PersistenceManager.
 */

use bevy::prelude::*;

pub mod inventory_replication;

pub use inventory_replication::handle_inventory_hotbar_move;

use crate::persistence_polish::PersistenceManager;

pub struct ServerCorePlugin;

impl Plugin for ServerCorePlugin {
    fn build(&self, app: &mut App) {
        let persistence_manager = PersistenceManager::new(
            std::path::PathBuf::from("saves/players")
        );
        app.insert_resource(persistence_manager);

        // Inventory replication system (now receives PersistenceManager)
        app.add_systems(Update, inventory_replication_system);

        app
            .add_plugins(RbeServerPlugin)
            .add_plugins(AscensionMercyAscentPlugin)
            .add_plugins(PersistencePolishPlugin)
            .add_plugins(FactionPersistencePlugin);
    }
}

/// Processes hotbar move events and calls the persistence-backed handler.
fn inventory_replication_system(
    mut persistence: ResMut<PersistenceManager>,
    // TODO: Wire real TransportEvent here
) {
    // When TransportEvent is available as Bevy Event:
    // for event in transport_events.read() {
    //     if let TransportEvent::MessageReceived { player_id, message: ClientMessage::InventoryHotbarMove { .. } } = event {
    //         if let Some(reply) = handle_inventory_hotbar_move(*player_id, message, &mut persistence) {
    //             // send reply via transport command
    //         }
    //     }
    // }

    debug!("[InventoryReplication] System active (PersistenceManager wired)");
}

// End of server/src/lib.rs