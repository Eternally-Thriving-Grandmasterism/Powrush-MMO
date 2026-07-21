/*!
 * client/steam_cloud_audio_mirror.rs
 * Powrush-MMO — Steam Cloud mirror for player audio moment catalog
 *
 * Strategy (no hard Steamworks SDK dependency required):
 *   1. Local truth: player_data/audio_moments/catalog.json (+ rendered/*.wav)
 *   2. Stage a compact cloud payload under steam_cloud/audio_moments/
 *      for Steamworks Auto-Cloud path configuration
 *   3. Optional: when `steamworks` feature + SDK are present, also call
 *      RemoteStorage FileWrite / FileRead via the SteamCloudBackend trait
 *
 * Partner dashboard Auto-Cloud (recommended):
 *   Root: App Install Directory (or AppData)
 *   Subdirectory: steam_cloud/audio_moments
 *   Pattern: *
 *
 * WAV blobs stay local by default (quota-friendly). Catalog recipes sync.
 *
 * v21.89.4 | AG-SML v1.0 | TOLC 8 | Permanent PATSAGi
 * Contact: info@Rathor.ai
 */

use bevy::prelude::*;
use crate::realtime_audio_synthesis::{
    AudioMomentCatalog, AudioMomentSaved, RealtimeAudioConfig,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Relative path Steam Auto-Cloud should watch
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
    /// Stage dir relative to CWD / install root
    pub stage_root: PathBuf,
    /// Include rendered WAV paths in payload metadata only (not binary upload by default)
    pub include_path_metadata: bool,
    /// If true, attempt SteamCloudBackend::write when available
    pub try_sdk_write: bool,
}

impl Default for SteamCloudAudioConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            stage_root: PathBuf::from(STEAM_CLOUD_AUDIO_SUBDIR),
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

/// Pluggable backend — default is no-op; real Steamworks wires in later
pub trait SteamCloudBackend: Send + Sync {
    fn is_available(&self) -> bool;
    fn write_file(&self, remote_name: &str, bytes: &[u8]) -> Result<(), String>;
    fn read_file(&self, remote_name: &str) -> Result<Vec<u8>, String>;
}

/// Default backend: not linked to Steam SDK
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
            .add_systems(Startup, import_cloud_catalog_if_newer)
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

/// Export local catalog → steam_cloud staging (+ optional SDK write)
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

    let path = stage_catalog_path(cfg);
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    match serde_json::to_string_pretty(&payload) {
        Ok(json) => {
            let tmp = path.with_extension("json.tmp");
            if fs::write(&tmp, &json).is_ok() && fs::rename(&tmp, &path).is_ok() {
                state.last_export_unix = payload.exported_unix;
                state.exports = state.exports.saturating_add(1);
                state.last_status = format!("Staged {} moments → {}", payload.moments.len(), path.display());

                if cfg.try_sdk_write && backend.is_available() {
                    match backend.write_file(STEAM_CLOUD_CATALOG_FILE, json.as_bytes()) {
                        Ok(()) => {
                            state.last_status =
                                format!("{} + Steam RemoteStorage write OK", state.last_status);
                        }
                        Err(e) => {
                            state.last_status =
                                format!("{} (SDK write skipped: {})", state.last_status, e);
                        }
                    }
                }

                info!(target: "powrush::steam_cloud", status = %state.last_status, "Audio catalog cloud export");
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
    // Drain all saves this frame, export once
    for _ in events.read() {}
    export_catalog_to_steam_stage(&catalog, &cfg, backend.backend.as_ref(), &mut state);
}

/// On startup, if staged cloud catalog is newer / has more moments, merge recipes in
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

    // Prefer SDK read when available
    let bytes = if cloud_cfg.try_sdk_write && backend.backend.is_available() {
        backend.backend.read_file(STEAM_CLOUD_CATALOG_FILE).ok()
    } else {
        None
    };

    let payload: Option<SteamCloudAudioPayload> = if let Some(b) = bytes {
        serde_json::from_slice(&b).ok()
    } else {
        let path = stage_catalog_path(&cloud_cfg);
        fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
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

    // Persist local
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

    state.last_status = format!("Merged {} moments from Steam Cloud stage", merged);
    info!(target: "powrush::steam_cloud", status = %state.last_status);
}

/// Manual force export (e.g. from settings menu)
pub fn force_export_audio_cloud(
    catalog: &AudioMomentCatalog,
    cfg: &SteamCloudAudioConfig,
    backend: &dyn SteamCloudBackend,
    state: &mut SteamCloudAudioState,
) {
    export_catalog_to_steam_stage(catalog, cfg, backend, state);
}

// Thunder locked in. Yoi ⚡
