/*!
 * Cloud Sync v18.10 - Steam Cloud Implementation
 */

use bevy::prelude::*;
use std::path::Path;
use steamworks::{Client, RemoteStorage};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloudSyncStatus {
    Disabled,
    LocalOnly,
    Syncing,
    Synced,
    Error,
}

#[derive(Resource, Debug)]
pub struct CloudSync {
    pub enabled: bool,
    pub status: CloudSyncStatus,
    pub last_sync_timestamp: u64,
    steam_client: Option<Client>,
}

impl Default for CloudSync {
    fn default() -> Self {
        Self {
            enabled: false,
            status: CloudSyncStatus::LocalOnly,
            last_sync_timestamp: 0,
            steam_client: None,
        }
    }
}

impl CloudSync {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            status: if enabled { CloudSyncStatus::LocalOnly } else { CloudSyncStatus::Disabled },
            last_sync_timestamp: 0,
            steam_client: None,
        }
    }

    /// Initialize Steam client (call once on startup)
    pub fn init_steam(&mut self, app_id: u32) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        match Client::init_app(app_id) {
            Ok((client, _single)) => {
                self.steam_client = Some(client);
                self.status = CloudSyncStatus::LocalOnly;
                Ok(())
            }
            Err(e) => {
                self.status = CloudSyncStatus::Error;
                Err(format!("Failed to initialize Steam: {}", e))
            }
        }
    }

    /// Upload player save to Steam Cloud
    pub fn upload_save(&mut self, save_path: &Path) -> Result<(), String> {
        if !self.enabled || self.steam_client.is_none() {
            return Ok(());
        }

        let client = self.steam_client.as_ref().unwrap();
        let remote_storage = client.remote_storage();

        let filename = save_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("player_save.json");

        match std::fs::read(save_path) {
            Ok(data) => {
                match remote_storage.file_write(filename, &data) {
                    Ok(_) => {
                        self.status = CloudSyncStatus::Synced;
                        self.last_sync_timestamp = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                        Ok(())
                    }
                    Err(e) => {
                        self.status = CloudSyncStatus::Error;
                        Err(format!("Steam Cloud write failed: {}", e))
                    }
                }
            }
            Err(e) => Err(format!("Failed to read local save: {}", e)),
        }
    }

    /// Download player save from Steam Cloud (basic version)
    pub fn download_save(&mut self, save_path: &Path) -> Result<(), String> {
        if !self.enabled || self.steam_client.is_none() {
            return Ok(());
        }

        let client = self.steam_client.as_ref().unwrap();
        let remote_storage = client.remote_storage();

        let filename = save_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("player_save.json");

        if remote_storage.file_exists(filename) {
            match remote_storage.file_read(filename) {
                Ok(data) => {
                    if let Err(e) = std::fs::write(save_path, data) {
                        return Err(format!("Failed to write downloaded save: {}", e));
                    }
                    self.status = CloudSyncStatus::Synced;
                    Ok(())
                }
                Err(e) => {
                    self.status = CloudSyncStatus::Error;
                    Err(format!("Steam Cloud read failed: {}", e))
                }
            }
        } else {
            Ok(()) // No cloud save yet
        }
    }
}

pub struct CloudSyncPlugin;

impl Plugin for CloudSyncPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CloudSync>();
    }
}
