/*!
 * server/src/player_account.rs
 *
 * v18.98 — Added PlayerLeft wiring for clean session removal + faction persistence save trigger.
 *
 * AG-SML v1.0 | TOLC 8
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use shared::protocol::ClientMessage;
use crate::persistence_polish::{PersistencePolishManager, PlayerSaveData};
use crate::persistence::faction_persistence::{PlayerJoined, PlayerLeft};

// ... existing AccountSystem, PlayerAccount, PlayerSession definitions ...

impl AccountSystem {
    pub fn create_session(
        &mut self,
        account_id: u64,
        runtime_player_id: u64,
        client_language: Option<&str>,
        mut joined_writer: EventWriter<PlayerJoined>,
    ) -> Option<u64> {
        // ... existing session creation logic ...

        joined_writer.send(PlayerJoined {
            entity: Entity::from_raw(runtime_player_id),
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

    /// NEW: Called when a player disconnects or session is explicitly removed.
    /// Triggers PlayerLeft so faction data is saved via observer.
    pub fn remove_session(
        &mut self,
        runtime_player_id: u64,
        mut left_writer: EventWriter<PlayerLeft>,
    ) {
        // Optional: perform any local cleanup here

        left_writer.send(PlayerLeft {
            entity: Entity::from_raw(runtime_player_id),
            player_id: runtime_player_id,
        });

        // Future: could also trigger immediate save here if needed
    }

    // ... rest of AccountSystem methods ...
}
