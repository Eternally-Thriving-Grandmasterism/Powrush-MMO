/*!
 * server/src/audio_moment_catalog.rs
 * Powrush-MMO — Server-side Audio Moment Catalog
 *
 * Stores **recipes + metadata** (not bulk PCM) per player so moments can be:
 *   - synced from clients after real-time synthesis
 *   - listed / recalled across devices
 *   - re-rendered client-side from the deterministic recipe
 *
 * Full PCM blobs stay local (or future CDN). Server remains light and mercy-aligned.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Permanent PATSAGi Councils | Contact: info@Rathor.ai
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

// Mirror of shared audio moment types (keep in sync with shared/src/audio_moments.rs)

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AudioMomentSource {
    RealtimeSynthesis,
    PremadeAsset,
    RecipeRecall,
    ExternalImport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AudioMomentFlavor {
    DivineWhisper,
    CouncilBloom,
    EpiphanyChime,
    MercyResonance,
    AmbientPad,
    TransitionStinger,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WaveformKind {
    Sine,
    Triangle,
    SoftSquare,
    NoiseBurst,
    HarmonicStack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSynthesisRecipe {
    pub waveform: WaveformKind,
    pub frequency_hz: f32,
    pub duration_secs: f32,
    pub sample_rate: u32,
    pub amplitude: f32,
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub partial_hz: f32,
    pub partial_amp: f32,
    pub brightness: f32,
    pub valence: f32,
    pub seed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioMoment {
    pub id: u64,
    pub owner_player_id: u64,
    pub title: String,
    pub flavor: AudioMomentFlavor,
    pub source: AudioMomentSource,
    pub created_at_unix: u64,
    pub context: String,
    pub recipe: AudioSynthesisRecipe,
    pub rendered_path: Option<String>,
    pub favorite: bool,
    pub play_count: u32,
    pub mercy_seal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerAudioCatalog {
    pub schema: String,
    pub owner_player_id: u64,
    pub moments: HashMap<u64, AudioMoment>,
    pub next_id: u64,
    pub last_synced_unix: u64,
}

impl PlayerAudioCatalog {
    pub fn new(owner_player_id: u64) -> Self {
        Self {
            schema: "audio_moment_catalog_v1".into(),
            owner_player_id,
            moments: HashMap::new(),
            next_id: 1,
            last_synced_unix: 0,
        }
    }

    pub fn upsert(&mut self, mut moment: AudioMoment) -> u64 {
        moment.owner_player_id = self.owner_player_id;
        if moment.id == 0 {
            moment.id = self.next_id;
            self.next_id += 1;
        } else if moment.id >= self.next_id {
            self.next_id = moment.id + 1;
        }
        let id = moment.id;
        self.moments.insert(id, moment);
        self.last_synced_unix = unix_now();
        id
    }
}

#[derive(Resource, Default)]
pub struct ServerAudioMomentStore {
    pub by_player: HashMap<u64, PlayerAudioCatalog>,
    pub root: PathBuf,
}

impl ServerAudioMomentStore {
    pub fn with_root(root: impl Into<PathBuf>) -> Self {
        Self {
            by_player: HashMap::new(),
            root: root.into(),
        }
    }

    fn player_path(&self, player_id: u64) -> PathBuf {
        self.root.join(format!("player_{}/catalog.json", player_id))
    }

    pub fn load_player(&mut self, player_id: u64) -> &PlayerAudioCatalog {
        if !self.by_player.contains_key(&player_id) {
            let path = self.player_path(player_id);
            let catalog = if path.exists() {
                fs::read_to_string(&path)
                    .ok()
                    .and_then(|s| serde_json::from_str(&s).ok())
                    .unwrap_or_else(|| PlayerAudioCatalog::new(player_id))
            } else {
                PlayerAudioCatalog::new(player_id)
            };
            self.by_player.insert(player_id, catalog);
        }
        self.by_player.get(&player_id).unwrap()
    }

    pub fn save_player(&self, player_id: u64) {
        if let Some(catalog) = self.by_player.get(&player_id) {
            let path = self.player_path(player_id);
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(json) = serde_json::to_string_pretty(catalog) {
                let tmp = path.with_extension("json.tmp");
                if fs::write(&tmp, json).is_ok() {
                    let _ = fs::rename(&tmp, &path);
                }
            }
        }
    }

    pub fn upsert_moment(&mut self, player_id: u64, moment: AudioMoment) -> u64 {
        if !self.by_player.contains_key(&player_id) {
            self.load_player(player_id);
        }
        let id = self
            .by_player
            .get_mut(&player_id)
            .map(|c| c.upsert(moment))
            .unwrap_or(0);
        self.save_player(player_id);
        info!(
            target: "powrush::audio",
            player_id,
            moment_id = id,
            "Server audio moment catalog upserted"
        );
        id
    }

    pub fn get_catalog(&mut self, player_id: u64) -> PlayerAudioCatalog {
        self.load_player(player_id).clone()
    }
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Events the network layer can forward into this store
#[derive(Event, Clone, Debug)]
pub struct ServerSaveAudioMoment {
    pub player_id: u64,
    pub moment: AudioMoment,
}

#[derive(Event, Clone, Debug)]
pub struct ServerRequestAudioCatalog {
    pub player_id: u64,
}

#[derive(Event, Clone, Debug)]
pub struct ServerAudioCatalogReady {
    pub player_id: u64,
    pub catalog: PlayerAudioCatalog,
}

pub struct AudioMomentCatalogPlugin;

impl Plugin for AudioMomentCatalogPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ServerAudioMomentStore::with_root("server_data/audio_moments"))
            .add_event::<ServerSaveAudioMoment>()
            .add_event::<ServerRequestAudioCatalog>()
            .add_event::<ServerAudioCatalogReady>()
            .add_systems(Update, (handle_save_moments, handle_catalog_requests));
    }
}

fn handle_save_moments(
    mut events: EventReader<ServerSaveAudioMoment>,
    mut store: ResMut<ServerAudioMomentStore>,
) {
    for ev in events.read() {
        // Only accept mercy-sealed moments by default
        if !ev.moment.mercy_seal {
            warn!(
                target: "powrush::audio",
                player_id = ev.player_id,
                "Rejected non-sealed audio moment"
            );
            continue;
        }
        store.upsert_moment(ev.player_id, ev.moment.clone());
    }
}

fn handle_catalog_requests(
    mut events: EventReader<ServerRequestAudioCatalog>,
    mut store: ResMut<ServerAudioMomentStore>,
    mut ready: EventWriter<ServerAudioCatalogReady>,
) {
    for ev in events.read() {
        let catalog = store.get_catalog(ev.player_id);
        ready.send(ServerAudioCatalogReady {
            player_id: ev.player_id,
            catalog,
        });
    }
}

// Thunder locked in. Server holds recipes; clients render. Yoi ⚡
