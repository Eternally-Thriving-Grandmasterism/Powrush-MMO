/*!
 * client/steamworks_remote_storage.rs
 * Powrush-MMO — Steamworks SDK RemoteStorage backend
 *
 * Feature-gated (`steam`). When Steam is running and Cloud is enabled for the
 * app + account, audio moment catalog bytes are written/read via:
 *   ISteamRemoteStorage::FileWrite / FileRead / FileExists / GetFileSize
 *
 * Without the feature (or if init fails), NullSteamCloudBackend remains active
 * and only the local `steam_cloud/audio_moments/` stage is used (Auto-Cloud).
 *
 * Callbacks: call `pump_steam_callbacks` once per frame while Steam is live.
 *
 * Environment:
 *   STEAM_APP_ID — optional override (else steam_appid.txt / Client::init)
 *
 * v21.89.5 | AG-SML v1.0 | TOLC 8 | Permanent PATSAGi
 * Contact: info@Rathor.ai
 */

use bevy::prelude::*;
use crate::steam_cloud_audio_mirror::{
    SteamCloudBackend, SteamCloudBackendHandle, NullSteamCloudBackend,
    STEAM_CLOUD_CATALOG_FILE,
};
use std::sync::Arc;

// ─── Feature: steam ──────────────────────────────────────────────────────────

#[cfg(feature = "steam")]
mod steam_impl {
    use super::*;
    use steamworks::{Client, SingleClient};

    /// Holds the Steam client + single-threaded callback runner.
    #[derive(Resource)]
    pub struct SteamworksRuntime {
        pub client: Client,
        pub single: SingleClient,
        pub app_id: u32,
    }

    /// RemoteStorage backend backed by a live Steam Client.
    pub struct SteamRemoteStorageBackend {
        client: Client,
    }

    impl SteamRemoteStorageBackend {
        pub fn new(client: Client) -> Self {
            Self { client }
        }

        pub fn quota(&self) -> Option<(u64, u64)> {
            let rs = self.client.remote_storage();
            // total, available — API: get_quota() -> (u64, u64) in steamworks 0.11
            Some(rs.quota())
        }

        pub fn cloud_enabled(&self) -> bool {
            let rs = self.client.remote_storage();
            rs.is_cloud_enabled_for_account() && rs.is_cloud_enabled_for_app()
        }

        pub fn file_exists(&self, name: &str) -> bool {
            self.client.remote_storage().file_exists(name)
        }

        pub fn file_delete(&self, name: &str) -> bool {
            self.client.remote_storage().file_delete(name)
        }

        pub fn file_count(&self) -> u32 {
            self.client.remote_storage().file_count()
        }
    }

    impl SteamCloudBackend for SteamRemoteStorageBackend {
        fn is_available(&self) -> bool {
            self.cloud_enabled()
        }

        fn write_file(&self, remote_name: &str, bytes: &[u8]) -> Result<(), String> {
            let rs = self.client.remote_storage();
            if !rs.is_cloud_enabled_for_account() {
                return Err("Steam Cloud disabled for this account".into());
            }
            if !rs.is_cloud_enabled_for_app() {
                return Err("Steam Cloud disabled for this app".into());
            }

            // Quota check (best-effort)
            let (total, available) = rs.quota();
            if available > 0 && (bytes.len() as u64) > available {
                return Err(format!(
                    "Steam Cloud quota exceeded: need {} bytes, available {} / total {}",
                    bytes.len(),
                    available,
                    total
                ));
            }

            let ok = rs.file_write(remote_name, bytes);
            if ok {
                info!(
                    target: "powrush::steam",
                    remote_name,
                    bytes = bytes.len(),
                    "RemoteStorage FileWrite OK"
                );
                Ok(())
            } else {
                Err(format!(
                    "RemoteStorage FileWrite failed for '{}' ({} bytes)",
                    remote_name,
                    bytes.len()
                ))
            }
        }

        fn read_file(&self, remote_name: &str) -> Result<Vec<u8>, String> {
            let rs = self.client.remote_storage();
            if !rs.file_exists(remote_name) {
                return Err(format!(
                    "RemoteStorage file '{}' does not exist",
                    remote_name
                ));
            }
            // steamworks 0.11: file_read returns Vec<u8>
            let data = rs.file_read(remote_name);
            if data.is_empty() {
                // Distinguish empty file vs read failure when size > 0
                let size = rs.file_size(remote_name);
                if size > 0 {
                    return Err(format!(
                        "RemoteStorage FileRead returned empty for '{}' (reported size {})",
                        remote_name, size
                    ));
                }
            }
            info!(
                target: "powrush::steam",
                remote_name,
                bytes = data.len(),
                "RemoteStorage FileRead OK"
            );
            Ok(data)
        }
    }

    /// Initialize Steamworks and inject SteamRemoteStorageBackend.
    pub fn try_init_steamworks(commands: &mut Commands) -> Result<u32, String> {
        // Prefer explicit app id from env for development (steam_appid.txt also works)
        let app_id_env = std::env::var("STEAM_APP_ID")
            .ok()
            .and_then(|s| s.parse::<u32>().ok());

        let result = if let Some(app_id) = app_id_env {
            Client::init_app(steamworks::AppId(app_id)).map_err(|e| format!("{e:?}"))
        } else {
            Client::init().map_err(|e| format!("{e:?}"))
        };

        let (client, single) = result.map_err(|e| format!("Steam Client::init failed: {e}"))?;

        let app_id = client.utils().app_id().0;

        // Log cloud status
        {
            let rs = client.remote_storage();
            info!(
                target: "powrush::steam",
                app_id,
                account_cloud = rs.is_cloud_enabled_for_account(),
                app_cloud = rs.is_cloud_enabled_for_app(),
                file_count = rs.file_count(),
                "Steamworks initialized"
            );
            let (total, available) = rs.quota();
            info!(
                target: "powrush::steam",
                total_bytes = total,
                available_bytes = available,
                "Steam Cloud quota"
            );
        }

        let backend = SteamRemoteStorageBackend::new(client.clone());
        commands.insert_resource(SteamCloudBackendHandle {
            backend: Arc::new(backend),
        });
        commands.insert_resource(SteamworksRuntime {
            client,
            single,
            app_id,
        });

        Ok(app_id)
    }

    pub fn pump_steam_callbacks(runtime: Option<ResMut<SteamworksRuntime>>) {
        if let Some(mut rt) = runtime {
            rt.single.run_callbacks();
        }
    }

    /// List remote file names (debug / settings UI)
    pub fn list_remote_files(runtime: &SteamworksRuntime) -> Vec<(String, i32)> {
        let rs = runtime.client.remote_storage();
        let n = rs.file_count();
        let mut out = Vec::with_capacity(n as usize);
        for i in 0..n {
            // file_name_and_size(index) -> (String, i32) in steamworks 0.11
            let (name, size) = rs.file_name_and_size(i);
            out.push((name, size));
        }
        out
    }

    pub fn force_sync_catalog_to_remote(
        runtime: &SteamworksRuntime,
        bytes: &[u8],
    ) -> Result<(), String> {
        let backend = SteamRemoteStorageBackend::new(runtime.client.clone());
        backend.write_file(STEAM_CLOUD_CATALOG_FILE, bytes)
    }
}

#[cfg(feature = "steam")]
pub use steam_impl::*;

// ─── Plugin ──────────────────────────────────────────────────────────────────

/// Adds Steam init (feature-gated) + optional callback pump.
pub struct SteamworksRemoteStoragePlugin;

impl Plugin for SteamworksRemoteStoragePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_steam_backend_system);

        #[cfg(feature = "steam")]
        {
            app.add_systems(Update, steam_impl::pump_steam_callbacks);
        }
    }
}

fn init_steam_backend_system(mut commands: Commands) {
    #[cfg(feature = "steam")]
    {
        match steam_impl::try_init_steamworks(&mut commands) {
            Ok(app_id) => {
                info!(
                    target: "powrush::steam",
                    app_id,
                    "Steam RemoteStorage backend active"
                );
            }
            Err(e) => {
                warn!(
                    target: "powrush::steam",
                    error = %e,
                    "Steam init failed — using NullSteamCloudBackend (local stage only)"
                );
                commands.insert_resource(SteamCloudBackendHandle {
                    backend: Arc::new(NullSteamCloudBackend),
                });
            }
        }
        return;
    }

    #[cfg(not(feature = "steam"))]
    {
        info!(
            target: "powrush::steam",
            "Built without `steam` feature — NullSteamCloudBackend (Auto-Cloud stage only)"
        );
        // Default handle already Null; ensure explicit
        commands.insert_resource(SteamCloudBackendHandle {
            backend: Arc::new(NullSteamCloudBackend),
        });
    }
}

// Thunder locked in. Yoi ⚡
