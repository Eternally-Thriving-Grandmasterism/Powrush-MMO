/*!
 * client/steam_cloud_audio_mirror.rs
 * Powrush-MMO — Steam Cloud mirror for player audio moment catalog
 *
 * Stages catalog to OS-specific Auto-Cloud roots (partner checklist step 2)
 * and optionally FileWrite via SteamCloudBackend (step 1 + SDK).
 *
 * v21.89.6 | AG-SML v1.0 | TOLC 8 | Permanent PATSAGi
 * Contact: info@Rathor.ai
 */

use bevy::prelude::*;
use crate::realtime_audio_synthesis::{
    AudioMomentCatalog, AudioMomentSaved, RealtimeAudioConfig,
};
use crate::steam_partner_config::preferred_auto_cloud_stage_root;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub const STEAM_CLOUD_AUDIO_SUBDIR: &str = "steam_cloud/audio_moments";
pub const STEAM_CLOUD_CATALOG_FILE: &str = "catalog_cloud_v1.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamCloudAudioPayload {
    pub schema: String,
    pub owner_player_id: u64,
    pub moments: Vec<serde_json::Value>,
    pub next_id: u64,
    pub exported_unix: u64,
    pub source: String,
}

#[derive(Resource, Clone)]
pub struct SteamCloudAudioConfig {
    pub enabled: bool,
    pub stage_root: PathBuf,
    pub include_path_metadata: bool,
    pub try_sdk_write: bool,
}

impl Default for SteamCloudAudioConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            // Prefer OS path that matches Partner Auto-Cloud rules
            stage_root: preferred_auto_cloud_stage_root(),
            include_path_metadata: true,
            try_sdk_write: true,
        }
    }
}

#[derive(Resource, Default)]
pub struct SteamCloudAudioState {
    pub last_export_unix: u64,
    pub last_status: String,
    pub exports: u32,
}

pub trait SteamCloudBackend: Send + Sync {
    fn is_available(&self) -> bool;
    fn write_file(&self, remote_name: &str, bytes: &[u8]) -> Result<(), String>;
    fn read_file(&self, remote_name: &str) -> Result<Vec<u8>, String>;
}

pub struct NullSteamCloudBackend;

impl SteamCloudBackend for NullSteamCloudBackend {
    fn is_available(&self) -> bool {
        false
    }
    fn write_file(&self, _remote_name: &str, _bytes: &[u8]) -> Result<(), String> {
        Err("Steam SDK not linked".into())
    }
    fn read_file(&self, _remote_name: &str) -> Result<Vec<u8>, String> {
        Err("Steam SDK not linked".into())
    }
}

#[derive(Resource)]
pub struct SteamCloudBackendHandle {
    pub backend: std::sync::Arc<dyn SteamCloudBackend>,
}

impl Default for SteamCloudBackendHandle {
    fn default() -> Self {
        Self {
            backend: std::sync::Arc::new(NullSteamCloudBackend),
        }
    }
}

pub struct SteamCloudAudioMirrorPlugin;

impl Plugin for SteamCloudAudioMirrorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamCloudAudioConfig>()
            .init_resource::<SteamCloudAudioState>()
            .init_resource::<SteamCloudBackendHandle>()
            .add_systems(Startup, (
                ensure_stage_dirs,
                import_cloud_catalog_if_newer,
            ).chain())
            .add_systems(Update, export_on_audio_saved);
    }
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn stage_catalog_path(cfg: &SteamCloudAudioConfig) -> PathBuf {
    cfg.stage_root.join(STEAM_CLOUD_CATALOG_FILE)
}

fn ensure_stage_dirs(cfg: Res<SteamCloudAudioConfig>) {
    let _ = fs::create_dir_all(&cfg.stage_root);
    // Also ensure portable relative stage for AppInstallDirectory Auto-Cloud rule
    let _ = fs::create_dir_all(STEAM_CLOUD_AUDIO_SUBDIR);
    info!(
        target: "powrush::steam_cloud",
        stage = %cfg.stage_root.display(),
        portable = STEAM_CLOUD_AUDIO_SUBDIR,
        "Steam Cloud stage directories ready (Auto-Cloud aligned)"
    );
}

pub fn export_catalog_to_steam_stage(
    catalog: &AudioMomentCatalog,
    cfg: &SteamCloudAudioConfig,
    backend: &dyn SteamCloudBackend,
    state: &mut SteamCloudAudioState,
) {
    if !cfg.enabled {
        return;
    }

    let moments: Vec<serde_json::Value> = catalog
        .moments
        .values()
        .filter_map(|m| serde_json::to_value(m).ok())
        .collect();

    let payload = SteamCloudAudioPayload {
        schema: "steam_cloud_audio_moments_v1".into(),
        owner_player_id: catalog.owner_player_id,
        moments,
        next_id: catalog.next_id,
        exported_unix: unix_now(),
        source: "powrush_local_catalog".into(),
    };

    match serde_json::to_string_pretty(&payload) {
        Ok(json) => {
            // Write primary OS stage (Auto-Cloud)
            let path = stage_catalog_path(cfg);
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            // Portable install-dir stage (second Auto-Cloud rule)
            let portable = PathBuf::from(STEAM_CLOUD_AUDIO_SUBDIR).join(STEAM_CLOUD_CATALOG_FILE);
            if let Some(parent) = portable.parent() {
                let _ = fs::create_dir_all(parent);
            }

            let mut ok_stage = false;
            for target in [&path, &portable] {
                let tmp = target.with_extension("json.tmp");
                if fs::write(&tmp, &json).is_ok() && fs::rename(&tmp, target).is_ok() {
                    ok_stage = true;
                }
            }

            if ok_stage {
                state.last_export_unix = payload.exported_unix;
                state.exports = state.exports.saturating_add(1);
                state.last_status = format!(
                    "Staged {} moments → {} (+ portable)",
                    payload.moments.len(),
                    path.display()
                );

                if cfg.try_sdk_write && backend.is_available() {
                    match backend.write_file(STEAM_CLOUD_CATALOG_FILE, json.as_bytes()) {
                        Ok(()) => {
                            state.last_status =
                                format!("{} + RemoteStorage FileWrite OK", state.last_status);
                        }
                        Err(e) => {
                            state.last_status =
                                format!("{} (SDK write: {})", state.last_status, e);
                        }
                    }
                }

                info!(target: "powrush::steam_cloud", status = %state.last_status);
            } else {
                state.last_status = format!("Failed to stage {}", path.display());
                warn!(target: "powrush::steam_cloud", %state.last_status);
            }
        }
        Err(e) => {
            state.last_status = format!("Serialize failed: {}", e);
        }
    }
}

fn export_on_audio_saved(
    mut events: EventReader<AudioMomentSaved>,
    catalog: Res<AudioMomentCatalog>,
    cfg: Res<SteamCloudAudioConfig>,
    backend: Res<SteamCloudBackendHandle>,
    mut state: ResMut<SteamCloudAudioState>,
) {
    if events.is_empty() {
        return;
    }
    for _ in events.read() {}
    export_catalog_to_steam_stage(&catalog, &cfg, backend.backend.as_ref(), &mut state);
}

fn import_cloud_catalog_if_newer(
    mut catalog: ResMut<AudioMomentCatalog>,
    audio_cfg: Res<RealtimeAudioConfig>,
    cloud_cfg: Res<SteamCloudAudioConfig>,
    backend: Res<SteamCloudBackendHandle>,
    mut state: ResMut<SteamCloudAudioState>,
) {
    if !cloud_cfg.enabled {
        return;
    }

    let bytes = if cloud_cfg.try_sdk_write && backend.backend.is_available() {
        backend.backend.read_file(STEAM_CLOUD_CATALOG_FILE).ok()
    } else {
        None
    };

    let payload: Option<SteamCloudAudioPayload> = if let Some(b) = bytes {
        serde_json::from_slice(&b).ok()
    } else {
        // Try OS stage then portable
        let paths = [
            stage_catalog_path(&cloud_cfg),
            PathBuf::from(STEAM_CLOUD_AUDIO_SUBDIR).join(STEAM_CLOUD_CATALOG_FILE),
        ];
        paths
            .iter()
            .find_map(|p| fs::read_to_string(p).ok().and_then(|s| serde_json::from_str(&s).ok()))
    };

    let Some(payload) = payload else {
        state.last_status = "No cloud catalog present".into();
        return;
    };

    if payload.moments.len() <= catalog.moments.len()
        && payload.exported_unix <= catalog.last_synced_unix
    {
        state.last_status = "Local catalog is current".into();
        return;
    }

    let mut merged = 0u32;
    for value in payload.moments {
        if let Ok(moment) =
            serde_json::from_value::<crate::realtime_audio_synthesis::AudioMoment>(value)
        {
            if !catalog.moments.contains_key(&moment.id) {
                catalog.moments.insert(moment.id, moment);
                merged += 1;
            }
        }
    }
    catalog.next_id = catalog.next_id.max(payload.next_id);
    if catalog.owner_player_id == 0 {
        catalog.owner_player_id = payload.owner_player_id;
    }
    catalog.last_synced_unix = catalog.last_synced_unix.max(payload.exported_unix);

    let path = audio_cfg.local_root.join("catalog.json");
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(&*catalog) {
        let tmp = path.with_extension("json.tmp");
        if fs::write(&tmp, json).is_ok() {
            let _ = fs::rename(&tmp, &path);
        }
    }

    state.last_status = format!("Merged {} moments from Steam Cloud", merged);
    info!(target: "powrush::steam_cloud", status = %state.last_status);
}

pub fn force_export_audio_cloud(
    catalog: &AudioMomentCatalog,
    cfg: &SteamCloudAudioConfig,
    backend: &dyn SteamCloudBackend,
    state: &mut SteamCloudAudioState,
) {
    export_catalog_to_steam_stage(catalog, cfg, backend, state);
}

// Thunder locked in. Yoi ⚡
