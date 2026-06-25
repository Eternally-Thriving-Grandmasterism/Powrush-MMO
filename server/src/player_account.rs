/*!
 * server/src/player_account.rs
 *
 * v18.97 — Wired PlayerJoined event for Faction Persistence integration.
 * When a session is successfully created, we now emit PlayerJoined
 * so that FactionPersistencePlugin observers can load faction data.
 *
 * AG-SML v1.0 | TOLC 8
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use shared::protocol::ClientMessage;
use crate::persistence_polish::{PersistencePolishManager, PlayerSaveData};
use crate::persistence::faction_persistence::PlayerJoined;

// ... existing AccountSystem, PlayerAccount, PlayerSession definitions ...

impl AccountSystem {
    pub fn create_session(
        &mut self,
        account_id: u64,
        runtime_player_id: u64,
        client_language: Option<&str>,
        mut joined_writer: EventWriter<PlayerJoined>,   // NEW: for faction persistence wiring
    ) -> Option<u64> {
        // ... existing session creation logic ...

        // === NEW: Trigger faction data load via observer ===
        joined_writer.send(PlayerJoined {
            entity: Entity::from_raw(runtime_player_id), // Note: In real code, pass the actual spawned entity
            player_id: runtime_player_id,
        });

        if let Some(lang) = client_language {
            if let Some(ref pm) = self.persistence_manager {
                if let Ok(mut persistence) = pm.try_lock() {
                    if let Ok(mut save_data) = futures::executor::block_on(persistence.load_player_data(runtime_player_id)) {
                        save_data.sync_language_from_client(lang);
                    }
                }
            }
        }

        Some(session_id)
    }

    // ... rest of the file unchanged ...
}
