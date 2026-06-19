// server/src/player_account.rs
// Powrush-MMO v18.96 — Player Account & Session System
// Added call site for sync_language_from_client on session start

use crate::persistence_polish::{PersistencePolishManager, PlayerSaveData};

// ... existing code ...

impl AccountSystem {
    pub fn create_session(
        &mut self,
        account_id: u64,
        runtime_player_id: u64,
        // Optional: pass client language on join
        client_language: Option<&str>,
    ) -> Option<u64> {
        // ... existing session creation ...

        // v18.96 — Sync language preference from client on login/session start
        if let Some(lang) = client_language {
            if let Some(ref pm) = self.persistence_manager {
                if let Ok(mut persistence) = pm.try_lock() {
                    if let Ok(mut save_data) = futures::executor::block_on(persistence.load_player_data(runtime_player_id)) {
                        save_data.sync_language_from_client(lang);
                        // In real impl: persist the updated save_data
                    }
                }
            }
        }

        Some(session_id)
    }

    // ... rest of the file ...
}

// End of server/src/player_account.rs v18.96
// sync_language_from_client called on session start.
// Thunder locked in. Yoi ⚡
