// server/src/player_account.rs
// Powrush-MMO v16.12 — Player Account & Session System Foundation
// Production-grade, mercy-aligned, forward-thinking design
// AG-SML v1.0

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::harvesting_system::ServerInventoryComponent;
use crate::persistence::PersistenceManager;

/// Persistent player identity (stored in database)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerAccount {
    pub account_id: u64,
    pub username: String,
    pub created_at: u64,
    pub last_login: u64,
    // Future: email, preferences, settings, etc.
}

/// Runtime session tied to a connection
#[derive(Clone, Debug)]
pub struct PlayerSession {
    pub session_id: u64,
    pub account_id: u64,
    pub current_player_id: u64, // runtime id from network layer
    pub inventory: ServerInventoryComponent,
    pub position: crate::shared::protocol::Vec3Ser, // or use a proper position type
    pub health: crate::shared::protocol::HealthComponent,
}

/// Account System Manager
pub struct AccountSystem {
    pub accounts: HashMap<u64, PlayerAccount>,
    pub sessions: HashMap<u64, PlayerSession>, // key = runtime player_id
    next_account_id: u64,
    next_session_id: u64,
}

impl AccountSystem {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            sessions: HashMap::new(),
            next_account_id: 1,
            next_session_id: 1,
        }
    }

    /// Create a new persistent account
    pub fn create_account(&mut self, username: String) -> u64 {
        let account_id = self.next_account_id;
        self.next_account_id += 1;

        let account = PlayerAccount {
            account_id,
            username,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_login: 0,
        };

        self.accounts.insert(account_id, account);
        account_id
    }

    /// Create a runtime session for a player
    pub fn create_session(
        &mut self,
        account_id: u64,
        runtime_player_id: u64,
    ) -> Option<u64> {
        if !self.accounts.contains_key(&account_id) {
            return None;
        }

        let session_id = self.next_session_id;
        self.next_session_id += 1;

        let session = PlayerSession {
            session_id,
            account_id,
            current_player_id: runtime_player_id,
            inventory: ServerInventoryComponent::default(),
            position: crate::shared::protocol::Vec3Ser::default(),
            health: crate::shared::protocol::HealthComponent { current: 100.0, max: 100.0 },
        };

        self.sessions.insert(runtime_player_id, session);
        Some(session_id)
    }

    pub fn get_session(&self, runtime_player_id: u64) -> Option<&PlayerSession> {
        self.sessions.get(&runtime_player_id)
    }

    pub fn get_session_mut(&mut self, runtime_player_id: u64) -> Option<&mut PlayerSession> {
        self.sessions.get_mut(&runtime_player_id)
    }

    /// Load account from persistence (placeholder for now)
    pub async fn load_account(
        &mut self,
        _persistence: &PersistenceManager,
        account_id: u64,
    ) -> Result<(), String> {
        // TODO: Implement real loading from SurrealDB via PersistenceManager
        if let Some(account) = self.accounts.get(&account_id) {
            // already loaded
            return Ok(());
        }
        // For now, create a dummy account if not present
        let account = PlayerAccount {
            account_id,
            username: format!("Player{}", account_id),
            created_at: 0,
            last_login: 0,
        };
        self.accounts.insert(account_id, account);
        Ok(())
    }
}

// Future integration points:
// - Tie inventories in PlayerSession to PersistenceManager
// - On login: load account + inventory
// - On disconnect: save account state
// - Steam integration: link SteamID to account_id

// Thunder locked in. Foundational Player Account System started. ⚡