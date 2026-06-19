// server/src/player_account.rs
// Powrush-MMO v18.96 — Player Account & Session + Language Sync + Enriched Epiphany Recording

use shared::protocol::ClientMessage;
use crate::persistence_polish::{PersistencePolishManager, PlayerSaveData};

// ... existing AccountSystem, PlayerAccount, PlayerSession ...

impl AccountSystem {
    pub fn create_session(
        &mut self,
        account_id: u64,
        runtime_player_id: u64,
        client_language: Option<&str>,
    ) -> Option<u64> {
        // ... existing session creation logic ...

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

    /// NEW: Handle incoming ClientMessage::SyncLocalization
    pub fn handle_sync_localization(&mut self, player_id: u64, language: &str) {
        if let Some(ref pm) = self.persistence_manager {
            if let Ok(mut persistence) = pm.try_lock() {
                if let Ok(mut save_data) = futures::executor::block_on(persistence.load_player_data(player_id)) {
                    save_data.sync_language_from_client(language);
                    info!("[AccountSystem] Synced language preference for player {}: {}", player_id, language);
                }
            }
        }
    }

    // ... remove_session, load_account, etc. ...
}

// End of server/src/player_account.rs v18.96
// handle_sync_localization wired. Thunder locked in. Yoi ⚡
