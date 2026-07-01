/*!
 * server/src/lib.rs
 *
 * Server core plugin + module declarations.
 * Now includes inventory_replication for authoritative hotbar drag & drop handling.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Mercy Gates
 */

use bevy::prelude::*;

pub mod inventory_replication;

// Re-export for convenience
pub use inventory_replication::handle_inventory_hotbar_move;

pub struct ServerCorePlugin;

impl Plugin for ServerCorePlugin {
    fn build(&self, app: &mut App) {
        // === Persistence Layer ===
        let persistence_manager = PersistenceManager::new(
            std::path::PathBuf::from("saves/players")
        );
        app.insert_resource(persistence_manager);

        // === Inventory Replication (NEW) ===
        // Registers the system that processes InventoryHotbarMove events
        app.add_systems(Update, inventory_replication_system);

        // === Existing plugins ===
        app
            .add_plugins(RbeServerPlugin)
            .add_plugins(AscensionMercyAscentPlugin)
            .add_plugins(PersistencePolishPlugin)
            .add_plugins(FactionPersistencePlugin)
            // ... rest of plugins ...
            ;
    }
}

/// Bevy system that drains TransportEvent and routes InventoryHotbarMove to the handler.
/// In production this should be connected to your actual TransportEvent source
/// (either as Bevy Event or via channel draining).
fn inventory_replication_system(
    // TODO: Replace with real event source
    // mut transport_events: EventReader<TransportEvent>,
) {
    // Example integration (uncomment and wire when TransportEvent is a Bevy Event):
    //
    // for event in transport_events.read() {
    //     if let TransportEvent::MessageReceived { player_id, message } = event {
    //         if let Some(reply) = handle_inventory_hotbar_move(*player_id, message) {
    //             // Send reply via your TransportCommand channel
    //         }
    //     }
    // }

    // For now this is a no-op placeholder system.
    // Once you expose TransportEvent as a Bevy Event, the handler will activate automatically.
    debug!("[InventoryReplication] System tick (waiting for TransportEvent wiring)");
}

// End of server/src/lib.rs
// Thunder locked in. Yoi ⚡