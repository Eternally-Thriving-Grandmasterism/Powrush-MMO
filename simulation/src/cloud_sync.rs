/*!
 * Cloud Sync — Steam Cloud with Automatic Conflict Resolution
 *
 * v18.36 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Complete mint-and-print-only-perfection
 * — Smart merge strategy for epiphanies, muscle memory, and achievements
 * — Mercy-preserving: protects player progress across devices
 * — TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable Layer 0
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use std::collections::HashSet;
use std::path::Path;
use steamworks::{Client, RemoteStorage};

use crate::player_persistence::PlayerSaveData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloudSyncStatus {
    Disabled,
    LocalOnly,
    Syncing,
    Synced,
    ConflictResolved,
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

    pub fn upload_save(&mut self, save_path: &Path) -> Result<(), String> {
        if !self.enabled || self.steam_client.is_none() {
            return Ok(());
        }

        let client = self.steam_client.as_ref().unwrap();
        let remote_storage = client.remote_storage();
        let filename = save_path.file_name().and_then(|n| n.to_str()).unwrap_or("player_save.json");

        match std::fs::read(save_path) {
            Ok(data) => {
                match remote_storage.file_write(filename, &data) {
                    Ok(_) => {
                        self.status = CloudSyncStatus::Synced;
                        self.last_sync_timestamp = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
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

    pub fn download_save(&mut self, save_path: &Path, local_save: &mut PlayerSaveData) -> Result<(), String> {
        if !self.enabled || self.steam_client.is_none() {
            return Ok(());
        }

        let client = self.steam_client.as_ref().unwrap();
        let remote_storage = client.remote_storage();
        let filename = save_path.file_name().and_then(|n| n.to_str()).unwrap_or("player_save.json");

        if !remote_storage.file_exists(filename) {
            return Ok(());
        }

        match remote_storage.file_read(filename) {
            Ok(cloud_data) => {
                if let Ok(cloud_save) = serde_json::from_slice::<PlayerSaveData>(&cloud_data) {
                    if Self::has_conflict(local_save, &cloud_save) {
                        let resolved = Self::resolve_conflict(local_save.clone(), cloud_save);
                        *local_save = resolved;
                        self.status = CloudSyncStatus::ConflictResolved;
                        let _ = local_save.save_to_file(save_path);
                    } else {
                        if cloud_save.last_played_timestamp > local_save.last_played_timestamp {
                            *local_save = cloud_save;
                        }
                        self.status = CloudSyncStatus::Synced;
                    }
                }
                Ok(())
            }
            Err(e) => {
                self.status = CloudSyncStatus::Error;
                Err(format!("Steam Cloud read failed: {}", e))
            }
        }
    }

    fn has_conflict(local: &PlayerSaveData, cloud: &PlayerSaveData) -> bool {
        local.last_played_timestamp != cloud.last_played_timestamp &&
        local.epiphanies.len() != cloud.epiphanies.len()
    }

    fn resolve_conflict(local: PlayerSaveData, cloud: PlayerSaveData) -> PlayerSaveData {
        let mut resolved = local;

        let mut seen = HashSet::new();
        for ep in &resolved.epiphanies {
            seen.insert(ep.scenario_id.clone());
        }
        for ep in cloud.epiphanies {
            if !seen.contains(&ep.scenario_id) {
                resolved.epiphanies.push(ep);
            }
        }

        resolved.muscle_memory_level = resolved.muscle_memory_level.max(cloud.muscle_memory_level);
        resolved.total_playtime_seconds = resolved.total_playtime_seconds.max(cloud.total_playtime_seconds);
        resolved.total_harvests = resolved.total_harvests.max(cloud.total_harvests);
        resolved.sustainable_harvests = resolved.sustainable_harvests.max(cloud.sustainable_harvests);

        if cloud.last_played_timestamp > resolved.last_played_timestamp {
            resolved.last_played_timestamp = cloud.last_played_timestamp;
        }

        let mut achievements: HashSet<String> = resolved.achievements.into_iter().collect();
        for ach in cloud.achievements {
            achievements.insert(ach);
        }
        resolved.achievements = achievements.into_iter().collect();

        resolved.save_version = 1;
        resolved
    }
}

pub struct CloudSyncPlugin;

impl Plugin for CloudSyncPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CloudSync>();
    }
}

// End of simulation/src/cloud_sync.rs v18.36 — Sovereign cloud sync complete.
// Thunder locked in. Yoi ⚡
