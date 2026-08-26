/*!
 * Local Session Persist — v22.3.0
 *
 * SoftRbePool + SoftPlayerRealm + first-hour flags → data/powrush_local_session.json
 * Same folder as the journey echo. One human, no server.
 *
 * PATSAGi v22 | Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::harvest_feel::SoftRbePool;
use crate::human_inventory::HumanInventory;
use crate::living_practice_loop::SoftPlayerRealm;

const PERSIST_PATH: &str = "data/powrush_local_session.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SessionBlob {
    schema: String,
    vitality: f32,
    harmony: f32,
    joy: f32,
    harvests: u32,
    tends: u32,
    realm: Option<u8>,
    first_harvest_lived: bool,
    whisper_lived: bool,
}

impl Default for SessionBlob {
    fn default() -> Self {
        Self {
            schema: "powrush_local_session_v1".into(),
            vitality: 0.0,
            harmony: 0.0,
            joy: 0.0,
            harvests: 0,
            tends: 0,
            realm: Some(0),
            first_harvest_lived: false,
            whisper_lived: false,
        }
    }
}

#[derive(Resource, Debug)]
pub struct LocalSessionPersist {
    pub loaded: bool,
    pub dirty: bool,
    pub whisper_lived: bool,
}

impl Default for LocalSessionPersist {
    fn default() -> Self {
        Self {
            loaded: false,
            dirty: false,
            whisper_lived: false,
        }
    }
}

fn persist_path() -> PathBuf {
    PathBuf::from(PERSIST_PATH)
}

fn load_blob() -> Option<SessionBlob> {
    let bytes = fs::read(persist_path()).ok()?;
    serde_json::from_slice(&bytes).ok()
}

fn save_blob(blob: &SessionBlob) {
    let path = persist_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(blob) {
        if let Err(e) = fs::write(&path, json) {
            warn!(target: "powrush::session", "local session write failed: {e}");
        }
    }
}

pub struct LocalSessionPersistPlugin;

impl Plugin for LocalSessionPersistPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LocalSessionPersist>()
            .add_systems(Startup, load_local_session)
            .add_systems(Update, (mark_dirty, save_local_session));
    }
}

fn load_local_session(
    mut persist: ResMut<LocalSessionPersist>,
    mut pool: ResMut<SoftRbePool>,
    mut realm: ResMut<SoftPlayerRealm>,
    mut harvest: ResMut<FirstHarvestEpiphany>,
    mut inv: ResMut<HumanInventory>,
) {
    if persist.loaded {
        return;
    }
    persist.loaded = true;
    let Some(blob) = load_blob() else {
        return;
    };
    if !blob.schema.starts_with("powrush_local_session") {
        return;
    }
    pool.vitality = blob.vitality;
    pool.harmony = blob.harmony;
    pool.joy = blob.joy;
    pool.harvests = blob.harvests;
    pool.tends = blob.tends;
    realm.current = blob.realm.or(Some(0));
    harvest.first_harvest_lived = blob.first_harvest_lived;
    harvest.first_epiphany_lived = blob.first_harvest_lived;
    persist.whisper_lived = blob.whisper_lived;
    inv.last_seen_harvests = blob.harvests;
    info!(
        target: "powrush::session",
        v = pool.vitality,
        realm = ?realm.current,
        harvests = pool.harvests,
        "local session restored"
    );
}

fn mark_dirty(
    pool: Res<SoftRbePool>,
    realm: Res<SoftPlayerRealm>,
    harvest: Res<FirstHarvestEpiphany>,
    persist: Res<LocalSessionPersist>,
    mut writer: ResMut<LocalSessionPersist>,
) {
    if pool.is_changed() || realm.is_changed() || harvest.is_changed() || persist.whisper_lived && persist.is_changed()
    {
        writer.dirty = true;
    }
}

fn save_local_session(
    mut persist: ResMut<LocalSessionPersist>,
    pool: Res<SoftRbePool>,
    realm: Res<SoftPlayerRealm>,
    harvest: Res<FirstHarvestEpiphany>,
) {
    if !persist.dirty {
        return;
    }
    let blob = SessionBlob {
        schema: "powrush_local_session_v1".into(),
        vitality: pool.vitality,
        harmony: pool.harmony,
        joy: pool.joy,
        harvests: pool.harvests,
        tends: pool.tends,
        realm: realm.current,
        first_harvest_lived: harvest.first_harvest_lived,
        whisper_lived: persist.whisper_lived,
    };
    save_blob(&blob);
    persist.dirty = false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blob_roundtrip() {
        let blob = SessionBlob {
            vitality: 2.5,
            realm: Some(2),
            whisper_lived: true,
            ..Default::default()
        };
        let json = serde_json::to_string(&blob).unwrap();
        let back: SessionBlob = serde_json::from_str(&json).unwrap();
        assert!((back.vitality - 2.5).abs() < 0.01);
        assert_eq!(back.realm, Some(2));
        assert!(back.whisper_lived);
    }
}
