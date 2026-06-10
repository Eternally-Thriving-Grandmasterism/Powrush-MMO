/*!
 * Cloud Sync v18.10
 *
 * Foundation for optional cloud synchronization of player progress.
 * Designed to be sovereignty-friendly (opt-in) and Steam-first.
 */

use bevy::prelude::*;
use std::path::Path;

/// Cloud sync status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloudSyncStatus {
    Disabled,
    LocalOnly,
    Syncing,
    Synced,
    Error,
}

/// Main Cloud Sync Resource
#[derive(Resource, Debug)]
pub struct CloudSync {
    pub enabled: bool,
    pub status: CloudSyncStatus,
    pub last_sync_timestamp: u64,
}

impl Default for CloudSync {
    fn default() -> Self {
        Self {
            enabled: false, // Opt-in by default (sovereignty)
            status: CloudSyncStatus::LocalOnly,
            last_sync_timestamp: 0,
        }
    }
}

impl CloudSync {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            status: if enabled { CloudSyncStatus::LocalOnly } else { CloudSyncStatus::Disabled },
            last_sync_timestamp: 0,
        }
    }

    /// Placeholder for uploading save to cloud (Steam Cloud or custom backend)
    pub fn upload_save(&mut self, _save_path: &Path) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        // TODO: Implement actual upload
        // - Steam Cloud: steamworks::RemoteStorage::file_write(...)
        // - Custom backend: HTTP upload

        self.status = CloudSyncStatus::Synced;
        self.last_sync_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Ok(())
    }

    /// Placeholder for downloading save from cloud
    pub fn download_save(&mut self, _save_path: &Path) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        // TODO: Implement actual download + conflict resolution
        self.status = CloudSyncStatus::Synced;
        Ok(())
    }
}

// === Cloud Sync Plugin ===

pub struct CloudSyncPlugin;

impl Plugin for CloudSyncPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CloudSync>()
            // Future: add systems for auto-sync on save, periodic sync, etc.
            ;
    }
}
