/*!
 * client/steamworks_remote_storage.rs
 * Powrush-MMO — Steamworks SDK RemoteStorage backend
 *
 * Feature-gated (`steam`). AppID from SteamPartnerRuntimeConfig resolution.
 * Updates SteamPartnerChecklistState after init (app_cloud / account_cloud).
 *
 * v21.89.6 | AG-SML v1.0 | TOLC 8 | Permanent PATSAGi
 * Contact: info@Rathor.ai
 */

use bevy::prelude::*;
use crate::steam_cloud_audio_mirror::{
    SteamCloudBackend, SteamCloudBackendHandle, NullSteamCloudBackend,
    STEAM_CLOUD_CATALOG_FILE,
};
use crate::steam_partner_config::SteamPartnerChecklistState;
use std::sync::Arc;

#[cfg(feature = "steam")]
mod steam_impl {
    use super::*;
    use crate::steam_partner_config::SteamPartnerRuntimeConfig;
    use steamworks::{Client, SingleClient};

    #[derive(Resource)]
    pub struct SteamworksRuntime {
        pub client: Client,
        pub single: SingleClient,
        pub app_id: u32,
    }

    pub struct SteamRemoteStorageBackend {
        client: Client,
    }

    impl SteamRemoteStorageBackend {
        pub fn new(client: Client) -> Self {
            Self { client }
        }

        pub fn cloud_enabled(&self) -> bool {
            let rs = self.client.remote_storage();
            rs.is_cloud_enabled_for_account() && rs.is_cloud_enabled_for_app()
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
                return Err(
                    "Steam Cloud disabled for this app — enable + Publish in Partner checklist step 1"
                        .into(),
                );
            }

            let (total, available) = rs.quota();
            if available > 0 && (bytes.len() as u64) > available {
                return Err(format!(
                    "Steam Cloud quota exceeded: need {} bytes, available {} / total {}",
                    bytes.len(),
                    available,
                    total
                ));
            }

            if rs.file_write(remote_name, bytes) {
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
            let data = rs.file_read(remote_name);
            if data.is_empty() {
                let size = rs.file_size(remote_name);
                if size > 0 {
                    return Err(format!(
                        "RemoteStorage FileRead empty for '{}' (size {})",
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

    pub fn try_init_steamworks(
        commands: &mut Commands,
        partner: &SteamPartnerRuntimeConfig,
        checklist: &mut SteamPartnerChecklistState,
    ) -> Result<u32, String> {
        let app_id = partner.app_id;
        info!(
            target: "powrush::steam",
            app_id,
            source = partner.app_id_source.as_str(),
            "Initializing Steamworks with resolved AppID"
        );

        let (client, single) = Client::init_app(steamworks::AppId(app_id))
            .map_err(|e| format!("Steam Client::init_app({app_id}) failed: {e:?}"))?;

        let resolved = client.utils().app_id().0;
        let rs = client.remote_storage();
        let account_cloud = rs.is_cloud_enabled_for_account();
        let app_cloud = rs.is_cloud_enabled_for_app();
        let (total, available) = rs.quota();

        checklist.steam_init_ok = true;
        checklist.cloud_enabled_account = account_cloud;
        checklist.cloud_enabled_app = app_cloud;
        checklist.recompute_summary();

        info!(
            target: "powrush::steam",
            app_id = resolved,
            account_cloud,
            app_cloud,
            file_count = rs.file_count(),
            total_bytes = total,
            available_bytes = available,
            checklist = %checklist.status_summary,
            "Steamworks initialized"
        );

        if !app_cloud {
            warn!(
                target: "powrush::steam",
                "app_cloud=false — complete Partner checklist step 1 (Enable Steam Cloud + Publish). See publishing/steam/PARTNER_CHECKLIST.md"
            );
        }

        let backend = SteamRemoteStorageBackend::new(client.clone());
        commands.insert_resource(SteamCloudBackendHandle {
            backend: Arc::new(backend),
        });
        commands.insert_resource(SteamworksRuntime {
            client,
            single,
            app_id: resolved,
        });

        Ok(resolved)
    }

    pub fn pump_steam_callbacks(runtime: Option<ResMut<SteamworksRuntime>>) {
        if let Some(mut rt) = runtime {
            rt.single.run_callbacks();
        }
    }

    pub fn list_remote_files(runtime: &SteamworksRuntime) -> Vec<(String, i32)> {
        let rs = runtime.client.remote_storage();
        let n = rs.file_count();
        let mut out = Vec::with_capacity(n as usize);
        for i in 0..n {
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

pub struct SteamworksRemoteStoragePlugin;

impl Plugin for SteamworksRemoteStoragePlugin {
    fn build(&self, app: &mut App) {
        // Partner config must load first (same Startup; registration order matters)
        app.add_systems(Startup, init_steam_backend_system);

        #[cfg(feature = "steam")]
        {
            app.add_systems(Update, steam_impl::pump_steam_callbacks);
        }
    }
}

fn init_steam_backend_system(
    mut commands: Commands,
    partner: Res<crate::steam_partner_config::SteamPartnerRuntimeConfig>,
    mut checklist: ResMut<SteamPartnerChecklistState>,
) {
    #[cfg(feature = "steam")]
    {
        match steam_impl::try_init_steamworks(&mut commands, &partner, &mut checklist) {
            Ok(app_id) => {
                info!(
                    target: "powrush::steam",
                    app_id,
                    "Steam RemoteStorage backend active"
                );
            }
            Err(e) => {
                checklist.steam_init_ok = false;
                checklist.recompute_summary();
                warn!(
                    target: "powrush::steam",
                    error = %e,
                    checklist = %checklist.status_summary,
                    "Steam init failed — NullSteamCloudBackend (local stage / Auto-Cloud only)"
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
        let _ = partner;
        checklist.steam_init_ok = false;
        checklist.recompute_summary();
        info!(
            target: "powrush::steam",
            checklist = %checklist.status_summary,
            "Built without `steam` feature — Null backend"
        );
        commands.insert_resource(SteamCloudBackendHandle {
            backend: Arc::new(NullSteamCloudBackend),
        });
    }
}

// Thunder locked in. Yoi ⚡
