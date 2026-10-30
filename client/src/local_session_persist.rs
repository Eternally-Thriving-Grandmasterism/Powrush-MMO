/*!
 * Local Session Persist — v22.10.0
 *
 * Pool, climate, web, companion trust → data/powrush_local_session.json
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::companion_bond::CompanionBond;
use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::harvest_feel::SoftRbePool;
use crate::human_inventory::HumanInventory;
use crate::living_ecology::PersistentWeb;
use crate::living_practice_loop::SoftPlayerRealm;
use crate::skill_xp::SkillXp;
#[cfg(test)] use crate::skill_xp::skills_on_load;
use crate::skill_xp::{skills_with_combat_on_load, COMBAT};

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
    #[serde(default)]
    thread_strength: f32,
    #[serde(default)]
    companion_trust: f32,
    /// SKILL-XP-1. `None` = absent (old save), which seeds once on load.
    #[serde(default)]
    skills: Option<[u32; 4]>,
    /// COMBAT-CORE-1. `None` = absent (v1 save), which loads as 0.
    #[serde(default)]
    combat_xp: Option<u32>,
    /// COMBAT-XP-PACE-1. UTC day key of `combat_day_hits`; `None` (v1 save) loads as 0.
    #[serde(default)]
    combat_day: Option<u32>,
    /// COMBAT-XP-PACE-1. Paying combat hits that day; `None` (v1 save) loads as 0.
    #[serde(default)]
    combat_day_hits: Option<u32>,
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
            thread_strength: 0.28,
            companion_trust: 0.18,
            skills: None,
            combat_xp: None,
            combat_day: None,
            combat_day_hits: None,
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
    shared::user_persist::persist_path(PERSIST_PATH)
}

fn load_blob() -> Option<SessionBlob> {
    load_blob_at(&persist_path())
}

fn load_blob_at(path: &Path) -> Option<SessionBlob> {
    if let Some(blob) = read_session_blob(path) {
        return Some(blob);
    }
    read_session_blob(&path.with_extension("json.bak"))
}

fn read_session_blob(path: &Path) -> Option<SessionBlob> {
    let bytes = fs::read(path).ok()?;
    serde_json::from_slice(&bytes).ok()
}

fn save_blob(blob: &SessionBlob) {
    save_blob_at(&persist_path(), blob);
}

fn save_blob_at(path: &Path, blob: &SessionBlob) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let Ok(json) = serde_json::to_string_pretty(blob) else {
        return;
    };
    let tmp = path.with_extension("json.tmp");
    if let Err(e) = fs::write(&tmp, json) {
        warn!(target: "powrush::session", "local session write failed: {e}");
        return;
    }
    // A bad live file must not replace the last good `.bak`.
    if path.exists() && read_session_blob(path).is_some() {
        let bak = path.with_extension("json.bak");
        if let Err(e) = fs::copy(path, &bak) {
            warn!(target: "powrush::session", "local session backup failed: {e}");
        }
    }
    if let Err(e) = fs::rename(&tmp, path) {
        warn!(target: "powrush::session", "local session write failed: {e}");
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
    mut web: ResMut<PersistentWeb>,
    mut bond: ResMut<CompanionBond>,
    skills: Option<ResMut<SkillXp>>,
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
    web.thread_strength = blob.thread_strength.max(0.0);
    web.apply_decay_on_return();
    bond.trust = blob.companion_trust.clamp(0.0, 1.0);
    bond.mounted = false;
    if let Some(mut skills) = skills {
        skills.xp =
            skills_with_combat_on_load(blob.skills, blob.combat_xp, blob.harvests, blob.tends);
        (skills.combat_day, skills.combat_day_hits) = (blob.combat_day.unwrap_or(0), blob.combat_day_hits.unwrap_or(0));
    }
    info!(
        target: "powrush::session",
        v = pool.vitality,
        trust = bond.trust,
        thread = web.thread_strength,
        "local session restored"
    );
}

fn mark_dirty(
    pool: Res<SoftRbePool>,
    realm: Res<SoftPlayerRealm>,
    harvest: Res<FirstHarvestEpiphany>,
    web: Res<PersistentWeb>,
    bond: Res<CompanionBond>,
    mut persist: ResMut<LocalSessionPersist>,
) {
    if pool.is_changed()
        || realm.is_changed()
        || harvest.is_changed()
        || web.is_changed()
        || bond.is_changed()
    {
        persist.dirty = true;
    }
}

fn save_local_session(
    mut persist: ResMut<LocalSessionPersist>,
    pool: Res<SoftRbePool>,
    realm: Res<SoftPlayerRealm>,
    harvest: Res<FirstHarvestEpiphany>,
    web: Res<PersistentWeb>,
    bond: Res<CompanionBond>,
    skills: Option<Res<SkillXp>>,
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
        thread_strength: web.thread_strength,
        companion_trust: bond.trust,
        skills: skills.as_ref().map(|s| [s.xp[0], s.xp[1], s.xp[2], s.xp[3]]),
        combat_xp: skills.as_ref().map(|s| s.xp[COMBAT]),
        combat_day: skills.as_ref().map(|s| s.combat_day),
        combat_day_hits: skills.as_ref().map(|s| s.combat_day_hits),
    };
    save_blob(&blob);
    persist.dirty = false;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    #[test]
    fn v1_save_loads_with_skills_kept_and_combat_zero() {
        let temp = TempSession::new("v1-combat");
        let mut v1 = serde_json::to_value(SessionBlob { vitality: 4.0, ..Default::default() }).unwrap();
        v1.as_object_mut().unwrap().remove("combat_xp");
        v1["skills"] = serde_json::json!([5, 6, 7, 8]);
        fs::write(&temp.path, v1.to_string()).unwrap();
        let a = load_blob_at(&temp.path).expect("v1 save must not reset the session");
        assert!((a.vitality - 4.0).abs() < 0.001 && a.combat_xp.is_none());
        assert_eq!(skills_with_combat_on_load(a.skills, a.combat_xp, 3, 2), [5, 6, 7, 8, 0]);
    }

    #[test]
    fn combat_xp_round_trips_and_back_compat_both_ways() {
        let temp = TempSession::new("combat-rt");
        let b = SessionBlob { skills: Some([1, 2, 3, 4]), combat_xp: Some(42), ..Default::default() };
        save_blob_at(&temp.path, &b);
        let a = load_blob_at(&temp.path).unwrap();
        assert_eq!((a.skills, a.combat_xp), (Some([1, 2, 3, 4]), Some(42)));
        assert_eq!(skills_with_combat_on_load(a.skills, a.combat_xp, 0, 0)[COMBAT], 42);
        // `skills` stays 4 wide for older builds; unknown fields are ignored on load.
        let mut json = serde_json::to_value(&a).unwrap();
        assert_eq!(json["skills"].as_array().map(Vec::len), Some(4));
        json["later_field"] = 1.into();
        let later: SessionBlob = serde_json::from_value(json).unwrap();
        assert_eq!(later.combat_xp, Some(42));
    }

    #[test]
    fn blob_keeps_trust() {
        let blob = SessionBlob {
            companion_trust: 0.62,
            thread_strength: 0.4,
            ..Default::default()
        };
        let json = serde_json::to_string(&blob).unwrap();
        let back: SessionBlob = serde_json::from_str(&json).unwrap();
        assert!((back.companion_trust - 0.62).abs() < 0.01);
    }

    struct TempSession {
        dir: PathBuf,
        path: PathBuf,
    }

    impl TempSession {
        fn new(label: &str) -> Self {
            static SEQ: AtomicU64 = AtomicU64::new(0);
            let n = SEQ.fetch_add(1, Ordering::Relaxed);
            let dir = std::env::temp_dir().join(format!(
                "powrush-save-atomic-{label}-{}-{n}",
                std::process::id()
            ));
            fs::create_dir_all(&dir).expect("temp session dir");
            let path = dir.join("powrush_local_session.json");
            Self { dir, path }
        }
    }

    impl Drop for TempSession {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }

    fn blob_with_trust(trust: f32) -> SessionBlob {
        SessionBlob {
            companion_trust: trust,
            ..Default::default()
        }
    }

    fn trust_near(blob: &SessionBlob, trust: f32) -> bool {
        (blob.companion_trust - trust).abs() < 0.001
    }

    #[test]
    fn corrupt_main_loads_last_good_bak() {
        let temp = TempSession::new("recover");
        let blob = blob_with_trust(0.62);
        save_blob_at(&temp.path, &blob);
        let bak = temp.path.with_extension("json.bak");
        let tmp = temp.path.with_extension("json.tmp");
        assert!(!bak.exists(), "first save has no prior good file to snapshot");
        assert!(!tmp.exists());
        // Second save copies the live good file to `.bak` before replacing it.
        save_blob_at(&temp.path, &blob);
        assert!(bak.is_file());
        assert!(!tmp.exists());
        fs::write(&temp.path, b"not-a-session").unwrap();
        let loaded = load_blob_at(&temp.path).expect("last good bak");
        assert!(trust_near(&loaded, 0.62));
        let bak_blob: SessionBlob =
            serde_json::from_slice(&fs::read(&bak).unwrap()).unwrap();
        assert!(trust_near(&bak_blob, loaded.companion_trust));
    }

    #[test]
    fn corrupt_main_is_never_promoted_to_bak() {
        let fresh = TempSession::new("nopromote-fresh");
        fs::write(&fresh.path, b"CORRUPT-MAIN").unwrap();
        save_blob_at(&fresh.path, &blob_with_trust(0.41));
        assert!(
            !fresh.path.with_extension("json.bak").exists(),
            "corrupt main must not become .bak"
        );
        assert!(!fresh.path.with_extension("json.tmp").exists());
        let loaded = load_blob_at(&fresh.path).expect("new good main");
        assert!(trust_near(&loaded, 0.41));

        let kept = TempSession::new("nopromote-kept");
        let good = blob_with_trust(0.62);
        save_blob_at(&kept.path, &good);
        save_blob_at(&kept.path, &good);
        let bak = kept.path.with_extension("json.bak");
        let bak_before = fs::read(&bak).unwrap();
        fs::write(&kept.path, b"CORRUPT-MAIN").unwrap();
        save_blob_at(&kept.path, &blob_with_trust(0.41));
        let bak_after = fs::read(&bak).unwrap();
        assert_eq!(bak_before, bak_after);
        assert!(!bak_after.windows(b"CORRUPT-MAIN".len()).any(|w| w == b"CORRUPT-MAIN"));
        let bak_blob: SessionBlob = serde_json::from_slice(&bak_after).unwrap();
        assert!(trust_near(&bak_blob, 0.62));
        let main_blob = load_blob_at(&kept.path).unwrap();
        assert!(trust_near(&main_blob, 0.41));
        assert!(!kept.path.with_extension("json.tmp").exists());
    }

    #[test]
    fn good_save_leaves_no_json_tmp() {
        let temp = TempSession::new("notmp");
        save_blob_at(&temp.path, &blob_with_trust(0.5));
        assert!(temp.path.is_file());
        assert!(!temp.path.with_extension("json.tmp").exists());
        save_blob_at(&temp.path, &blob_with_trust(0.77));
        assert!(!temp.path.with_extension("json.tmp").exists());
        let loaded = load_blob_at(&temp.path).unwrap();
        assert!(trust_near(&loaded, 0.77));
    }

    #[test]
    fn old_blob_seeds_skills_once_and_round_trips() {
        use crate::skill_xp::{XP_PER_GATHER as G, XP_PER_TEND as T};
        let temp = TempSession::new("skills");
        let mut b = SessionBlob::default();
        (b.harvests, b.tends) = (3, 2);
        let mut old = serde_json::to_value(b).unwrap();
        old.as_object_mut().unwrap().remove("skills");
        fs::write(&temp.path, old.to_string()).unwrap();
        let loaded = load_blob_at(&temp.path).unwrap();
        assert!(loaded.skills.is_none() && loaded.schema == "powrush_local_session_v1");
        let seeded = skills_on_load(loaded.skills, loaded.harvests, loaded.tends);
        assert_eq!(seeded, [3 * G, 2 * T, 0, 0]);
        // Field now present: counters grow, a second load must not re-seed.
        let kept = [seeded[0], seeded[1], 7, 0];
        let mut b = loaded;
        (b.harvests, b.tends, b.skills) = (9, 9, Some(kept));
        save_blob_at(&temp.path, &b);
        let a = load_blob_at(&temp.path).unwrap();
        assert_eq!(skills_on_load(a.skills, a.harvests, a.tends), kept);
    }

    #[test]
    fn combat_day_and_hits_round_trip() {
        let temp = TempSession::new("combat-day");
        let b = SessionBlob { combat_day: Some(20_400), combat_day_hits: Some(17), ..Default::default() };
        save_blob_at(&temp.path, &b);
        let a = load_blob_at(&temp.path).unwrap();
        assert_eq!((a.combat_day, a.combat_day_hits), (Some(20_400), Some(17)));
        assert_eq!(a.schema, "powrush_local_session_v1");
    }

    #[test]
    fn v1_save_without_day_fields_loads_as_zero_hits() {
        let temp = TempSession::new("combat-day-v1");
        let mut v1 = serde_json::to_value(SessionBlob { combat_xp: Some(30), ..Default::default() }).unwrap();
        v1.as_object_mut().unwrap().remove("combat_day");
        v1.as_object_mut().unwrap().remove("combat_day_hits");
        fs::write(&temp.path, v1.to_string()).unwrap();
        let a = load_blob_at(&temp.path).expect("v1 save must not reset the session");
        assert_eq!((a.combat_day.unwrap_or(0), a.combat_day_hits.unwrap_or(0)), (0, 0));
        assert_eq!(a.combat_xp, Some(30));
    }

    /// X3 stand-in for `combat_fire`: the same land, `fire_line` and `set_pulse` as combat_core :397-400.
    #[test]
    fn paying_hit_marks_the_session_dirty() {
        use crate::combat_core::{fire_line, land_hit_crossing, set_pulse, CombatState};
        let mut app = App::new();
        app.init_resource::<SoftRbePool>()
            .init_resource::<SoftPlayerRealm>()
            .init_resource::<FirstHarvestEpiphany>()
            .init_resource::<PersistentWeb>()
            .init_resource::<CompanionBond>()
            .init_resource::<LocalSessionPersist>()
            .add_systems(Update, mark_dirty);
        app.update();
        app.update();
        app.world_mut().resource_mut::<LocalSessionPersist>().dirty = false;
        app.update();
        assert!(!app.world().resource::<LocalSessionPersist>().dirty, "quiet frame stays clean");
        let (mut c, mut s) = (CombatState::default(), SkillXp::default());
        let (hit, level) = land_hit_crossing(&mut c, &mut s, true);
        assert!(hit && s.combat_day_hits == 1);
        let line = fire_line(shared::hex_travel::PlaceId::Heartwood, &c, hit, s.xp[COMBAT], level).unwrap();
        set_pulse(&mut app.world_mut().resource_mut::<FirstHarvestEpiphany>(), line, 4.2);
        app.update();
        assert!(app.world().resource::<LocalSessionPersist>().dirty, "a paying hit is saved");
    }
}
