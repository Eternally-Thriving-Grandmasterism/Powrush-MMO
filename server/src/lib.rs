/*!
 * server/src/lib.rs
 *
 * v19.4 — Registered PersistenceManager as Resource + FactionPersistencePlugin.
 *
 * AG-SML v1.0 | TOLC 8
 * Thunder locked in. Yoi ⚡
 */

// ... existing mods ...
pub mod persistence_polish;
pub mod persistence;

// ... existing use statements ...
use crate::persistence_polish::{PersistenceManager, PersistencePolishPlugin};
use crate::persistence::faction_persistence::FactionPersistencePlugin;

impl Plugin for ServerCorePlugin {
    fn build(&self, app: &mut App) {
        // === Persistence Layer ===
        let persistence_manager = PersistenceManager::new(
            std::path::PathBuf::from("saves/players")
        );
        app
            .insert_resource(persistence_manager)           // NEW: Unified persistence
            .add_plugins(PersistencePolishPlugin)
            .add_plugins(FactionPersistencePlugin);

        // ... rest of ServerCorePlugin build ...
    }
}
