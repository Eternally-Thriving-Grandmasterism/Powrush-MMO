//! LOOT-DROPS-1. Named-find roll with caller-supplied rolls; LOOT-DROPS-WIRE-1 calls it once per defeat (combat_core.rs).
//! Weights from #738 CUT 6581050171, kept by Sherif ("3 yes keep", 7:28 PM ET); Sherif may veto or retune.
//! No odds are shown to players.
use bevy::prelude::*;
use crate::difficulty::{drop_chance, DifficultyTier};
use shared::temper::{NamedFind, Standing, NAMED_FINDS};
/// Basis points in one whole: `chance_roll` lives in 0..10000.
pub const LOOT_BPS: u32 = 10_000;
/// A find whose `role` matches the caller's role weighs this many times its standing weight.
pub const ROLE_SKEW: u32 = 2;
/// Unbroken's extra over Sworn yields no named find, so its power reads as Sworn.
pub const fn power_tier(tier: DifficultyTier) -> DifficultyTier {
    if matches!(tier, DifficultyTier::Unbroken) { DifficultyTier::Sworn } else { tier }
}
/// Kept 8, Proven 4, Storied 2. Worn is never a named find.
pub const fn standing_weight(s: Standing) -> u32 {
    match s { Standing::Worn => 0, Standing::Kept => 8, Standing::Proven => 4, Standing::Storied => 2 }
}
pub fn find_weight(f: &NamedFind, role: Option<&str>) -> u32 {
    let w = standing_weight(f.standing());
    if role == Some(f.role) { w * ROLE_SKEW } else { w }
}
/// One named-find roll. `chance_roll` is a caller roll in 0..LOOT_BPS; `pick_roll` is any u32.
pub fn roll_named_find(tier: DifficultyTier, base_chance: f32, role: Option<&str>, chance_roll: u16, pick_roll: u32) -> Option<&'static NamedFind> {
    let bar = (drop_chance(power_tier(tier), base_chance) * LOOT_BPS as f32) as u32;
    if u32::from(chance_roll) >= bar { return None; }
    let total: u32 = NAMED_FINDS.iter().map(|f| find_weight(f, role)).sum();
    let mut pick = pick_roll % total;
    for f in NAMED_FINDS.iter() {
        let w = find_weight(f, role);
        if pick < w { return Some(f); } else { pick -= w; }
    }
    None
}
/// LOOT-DROPS-WIRE-1: council base chance (#738 RULINGS 6595183027); Sherif may veto.
pub const LOOT_BASE_CHANCE: f32 = 0.25;
pub const FINDS_SAVE: &str = "powrush_finds.json";
/// Pure mix of (strikes, place, finds owned): the same inputs give the same rolls.
pub const fn loot_hash(strikes: u32, place: u32, owned: u32) -> u32 {
    let h = (strikes.wrapping_mul(0x9E37_79B9) ^ place.wrapping_mul(0x85EB_CA6B) ^ owned.wrapping_mul(0xC2B2_AE35)).wrapping_mul(0x7FEB_352D);
    (h ^ (h >> 15)).wrapping_mul(0x846C_A68B) ^ (h >> 16)
}
/// Named finds earned on defeats: names only, a record with no stats. `pending` is the line waiting for `show_find`.
#[derive(Resource, Debug, Default, Clone, PartialEq)]
pub struct FoundFinds { pub names: Vec<&'static str>, pub pending: Option<String>, pub equipped: Option<&'static str> }
impl FoundFinds {
    /// GEAR-SLOT-1 — one slot: equip an owned named find by name; a second equip replaces the first. True only when it set the slot.
    pub fn equip_named(&mut self, name: &str) -> bool {
        let Some(f) = NAMED_FINDS.iter().find(|f| f.name == name).filter(|f| self.names.contains(&f.name)) else { return false };
        self.equipped = Some(f.name); true
    }
    /// GEAR-SLOT-1 — empty the one slot.
    pub fn clear_equipped(&mut self) { self.equipped = None; }
    /// GEAR-SLOT-1 — the equipped owned find, or None when the slot is empty, unowned or unknown. Reads `temper`/`seated` only.
    pub fn equipped_find(&self) -> Option<&'static NamedFind> {
        let name = self.equipped?;
        NAMED_FINDS.iter().find(|f| f.name == name && self.names.contains(&f.name))
    }
    /// One roll per defeat, role None. An owned pick is silent and a waiting line is never replaced. True when a new find lands.
    pub fn drop_once(&mut self, tier: DifficultyTier, strikes: u32, place: u32) -> bool {
        if self.pending.is_some() { return false; }
        let h = loot_hash(strikes, place, self.names.len() as u32);
        let Some(f) = roll_named_find(tier, LOOT_BASE_CHANCE, None, (h % LOOT_BPS) as u16, loot_hash(h, place, 1)) else { return false };
        if self.names.contains(&f.name) { return false; }
        self.names.push(f.name); self.pending = Some(find_line(f)); true
    }
    pub fn encode(&self) -> String {
        let names = serde_json::to_string(&self.names).unwrap_or_default();
        match self.equipped_find() {
            Some(f) => format!(r#"{{"schema":"{FINDS_SCHEMA}","names":{names},"equipped":{}}}"#, serde_json::to_string(f.name).unwrap_or_default()),
            None => format!(r#"{{"schema":"{FINDS_SCHEMA}","names":{names}}}"#),
        }
    }
    /// The v1 JSON object, or a pre-JSON save of one name per line. Known names only, in table order.
    pub fn decode(s: Option<&str>) -> Self {
        let s = s.unwrap_or_default();
        let names = match serde_json::from_str::<FindsDoc>(s) {
            Ok(FindsDoc {
                schema: Some(k), ..
            }) if !k.starts_with("powrush_finds") => Vec::new(),
            Ok(doc) => doc.names,
            Err(_) => s.lines().map(str::to_string).collect(),
        };
        let has = |n: &str| names.iter().any(|l| l.trim() == n);
        // GEAR-SLOT-1 — restore the slot only when its name is known and owned; unknown, unowned or foreign-schema bytes give None.
        let equipped = serde_json::from_str::<FindsDoc>(s).ok().and_then(|d| d.equipped).and_then(|name| NAMED_FINDS.iter().map(|f| f.name).find(|n| *n == name && has(n)));
        FoundFinds { names: NAMED_FINDS.iter().map(|f| f.name).filter(|n| has(n)).collect(), pending: None, equipped }
    }
}
pub const FINDS_SCHEMA: &str = "powrush_finds_v1";
/// `powrush_finds.json` body. Unknown fields and unknown names are ignored on load.
#[derive(serde::Deserialize)]
struct FindsDoc {
    #[serde(default)]
    schema: Option<String>,
    #[serde(default)]
    names: Vec<String>,
    #[serde(default)]
    equipped: Option<String>,
}
pub fn find_line(f: &NamedFind) -> String { format!("{} · {}", f.name, f.sentence) }
/// The waiting line shows only once the calm queue is done and the current pulse line has expired.
pub fn find_ready(calm_done: bool, pulse_until: f64, now: f64) -> bool { calm_done && pulse_until <= now }
/// Live, then `.bak`, read raw like wood: `read_named` rejects the pre-JSON newline save, so it cannot be the only path.
pub fn load_finds(mut finds: ResMut<FoundFinds>) {
    use shared::user_persist::{read_backup_bytes, read_live_bytes, RawPersist};
    let read = |raw: std::io::Result<RawPersist>| match raw {
        Ok(RawPersist::Bytes(b)) => Some(FoundFinds::decode(std::str::from_utf8(&b).ok())),
        _ => None,
    };
    *finds = read(read_live_bytes(FINDS_SAVE))
        .filter(|f| !f.names.is_empty())
        .or_else(|| read(read_backup_bytes(FINDS_SAVE)))
        .unwrap_or_default();
}
pub fn save_finds(finds: &FoundFinds) { let _ = shared::user_persist::write_named(FINDS_SAVE, finds.encode()); }
#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difficulty::TIERS;
    use shared::temper::VENDOR_STANDING_CEILING;
    fn sweep(mut f: impl FnMut(u16, u32)) {
        let mut s: u32 = 0x2545_F491;
        for _ in 0..512 {
            s = s.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            f(((s >> 16) % LOOT_BPS) as u16, s);
        }
    }

    #[test]
    fn loot_drops_weights_floor_total_and_role_skew() {
        assert!(NAMED_FINDS.iter().all(|f| find_weight(f, None) >= 2));
        assert_eq!(NAMED_FINDS.iter().map(|f| find_weight(f, None)).sum::<u32>(), 28);
        for f in NAMED_FINDS.iter() {
            assert_eq!(find_weight(f, Some(f.role)), find_weight(f, None) * ROLE_SKEW);
            assert_eq!(find_weight(f, Some("no-such-role")), find_weight(f, None));
        }
    }

    #[test]
    fn loot_drops_unbroken_power_never_above_sworn() {
        assert_eq!(power_tier(DifficultyTier::Unbroken), DifficultyTier::Sworn);
        sweep(|c, p| {
            let name = |t| roll_named_find(t, 0.5, Some("tend"), c, p).map(|f| f.name);
            assert_eq!(name(DifficultyTier::Unbroken), name(DifficultyTier::Sworn));
        });
    }

    #[test]
    fn loot_drops_every_find_above_vendor_ceiling() {
        for t in TIERS {
            sweep(|c, p| if let Some(f) = roll_named_find(t, 1.0, None, c, p) {
                assert!(f.standing() > VENDOR_STANDING_CEILING, "{}", f.name);
            });
        }
        assert!(roll_named_find(DifficultyTier::Sworn, 1.0, None, 9_999, 0).is_none());
        let src = include_str!("loot_drops.rs").split("#[cfg(test)]").next().expect("prefix");
        for banned in ["fn convert", "fn exchange", "fn trade", "fn buy", "fn sell"] {
            assert!(!src.contains(banned), "no conversion fn: {banned}");
        }
    }

    #[test]
    fn loot_drops_wire_seeded_rolls_repeat_owned_silent_all_owned_never() {
        let (mut found, all) = (0, FoundFinds { names: NAMED_FINDS.iter().map(|f| f.name).collect(), pending: None, equipped: None });
        for s in 0..2048 {
            let (mut a, mut b, mut c) = (FoundFinds::default(), FoundFinds::default(), all.clone());
            assert!(a.drop_once(DifficultyTier::Sworn, s, 1) == b.drop_once(DifficultyTier::Sworn, s, 1) && a == b && loot_hash(s, 1, 0) == loot_hash(s, 1, 0));
            assert!(!c.drop_once(DifficultyTier::Sworn, s, 1) && c == all, "all owned never drops");
            let Some(line) = a.pending.take() else { assert_eq!(a, FoundFinds::default()); continue };
            found += 1;
            assert!(NAMED_FINDS.iter().any(|n| n.name == a.names[0] && line == format!("{} · {}", n.name, n.sentence)));
            let mut w = FoundFinds { names: vec![], pending: Some(line), equipped: None };
            assert!(!w.drop_once(DifficultyTier::Sworn, s, 1) && w.names.is_empty(), "no re-roll while a line waits");
            let mut o = FoundFinds { names: a.names.clone(), pending: None, equipped: None };
            if o.drop_once(DifficultyTier::Sworn, s, 0) { assert!(o.names[1] != o.names[0]) } else { assert!(o.names.len() == 1 && o.pending.is_none()) }
        }
        assert!(found > 0);
    }
    #[test]
    fn loot_drops_wire_save_round_trip_and_ready_gate() {
        let f = FoundFinds { names: vec![NAMED_FINDS[1].name, NAMED_FINDS[4].name], pending: Some("x".into()), equipped: None };
        assert_eq!(FoundFinds::decode(Some(&f.encode())), FoundFinds { pending: None, ..f.clone() });
        assert!(FoundFinds::decode(Some("no such find\n")) == FoundFinds::default() && FoundFinds::decode(None) == FoundFinds::default());
        assert!(!find_ready(false, 0.0, 10.0) && !find_ready(true, 12.0, 10.0) && find_ready(true, 10.0, 10.0) && FINDS_SAVE == "powrush_finds.json");
    }

    /// Exact bytes the pre-JSON `encode` wrote for NAMED_FINDS[1] then [4].
    pub(crate) const OLD_FINDS: &[u8] = b"Steady Climate Pick\nQuiet Harmony Loom";
    /// Runs `body` with `POWRUSH_USER_DIR` on a fresh temp dir, under the crate env lock.
    pub(crate) fn in_user_dir<R>(tag: &str, body: impl FnOnce(&std::path::Path) -> R) -> R {
        let dir = std::env::temp_dir().join(format!("powrush-finds-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("powrush_seal.json"), b"{}").unwrap();
        let _guard = crate::test_env::lock();
        let (env, prev) = (shared::user_persist::USER_DIR_OVERRIDE_ENV, std::env::var(shared::user_persist::USER_DIR_OVERRIDE_ENV).ok());
        std::env::set_var(env, &dir);
        let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| body(&dir)));
        match prev { Some(v) => std::env::set_var(env, v), None => std::env::remove_var(env) }
        let _ = std::fs::remove_dir_all(&dir);
        out.unwrap_or_else(|p| std::panic::resume_unwind(p))
    }
    /// What the Startup `load_finds` system leaves in `FoundFinds`.
    fn loaded() -> FoundFinds {
        let mut app = App::new();
        app.init_resource::<FoundFinds>().add_systems(Startup, load_finds);
        app.update();
        app.world().resource::<FoundFinds>().clone()
    }
    fn two() -> FoundFinds { FoundFinds { names: vec![NAMED_FINDS[1].name, NAMED_FINDS[4].name], pending: None, equipped: None } }

    /// LTS2 (H3) — the exact old newline bytes load from the live file, and from `.bak` when live is gone or unreadable.
    #[test]
    fn lts2_old_newline_bytes_load_live_then_bak() {
        in_user_dir("lts2", |dir| {
            let (live, bak) = (dir.join(FINDS_SAVE), dir.join(format!("{FINDS_SAVE}.bak")));
            std::fs::write(&live, OLD_FINDS).unwrap();
            assert!(shared::user_persist::read_named(FINDS_SAVE).is_err(), "global JSON rule stays");
            assert_eq!(loaded(), two());
            std::fs::rename(&live, &bak).unwrap();
            assert_eq!(loaded(), two(), "old bytes in .bak");
            std::fs::write(&live, b"").unwrap();
            assert_eq!(loaded(), two(), "empty live falls back to .bak");
        });
    }

    /// LTS3 (H3) — new saves are one JSON value with the v1 schema and round-trip through user_persist; `pending` is not saved.
    #[test]
    fn lts3_finds_save_is_json_and_round_trips() {
        let f = FoundFinds { pending: Some("x".into()), ..two() };
        assert_eq!(f.encode(), r#"{"schema":"powrush_finds_v1","names":["Steady Climate Pick","Quiet Harmony Loom"]}"#);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&f.encode()).unwrap()["schema"], "powrush_finds_v1");
        assert_eq!(FoundFinds::decode(Some(&f.encode())), two());
        in_user_dir("lts3", |dir| {
            save_finds(&FoundFinds { names: vec![NAMED_FINDS[1].name], pending: None, equipped: None });
            save_finds(&f);
            assert_eq!(FoundFinds::decode(shared::user_persist::read_named(FINDS_SAVE).ok().as_deref()), two());
            let bak = std::fs::read_to_string(dir.join(format!("{FINDS_SAVE}.bak"))).unwrap();
            assert_eq!(FoundFinds::decode(Some(&bak)).names, vec![NAMED_FINDS[1].name], "prior JSON save kept as .bak");
            assert_eq!(loaded(), two());
        });
    }

    /// LTS4 (H3) — the SessionBlob v1 literal stays at local_session_persist.rs:278 and finds stay out of that file.
    #[test]
    fn lts4_session_v1_literal_pinned() {
        const V1: &str = r#"{"schema":"powrush_local_session_v1","vitality":1.0,"harmony":0.0,"joy":0.0,"harvests":0,"tends":0,"realm":0,"first_harvest_lived":false,"whisper_lived":false}"#;
        let src = include_str!("local_session_persist.rs");
        assert!(src.lines().nth(277).is_some_and(|l| l.contains(&format!("fs::write(&temp.path, r#\"{V1}\"#)"))), "v1 literal moved");
        assert!(src.lines().nth(278).is_some_and(|l| l.contains("run_best, None")) && !src.contains("finds"));
        let v: serde_json::Value = serde_json::from_str(V1).unwrap();
        assert_eq!((v["vitality"].as_f64(), v["harmony"].as_f64(), v["joy"].as_f64()), (Some(1.0), Some(0.0), Some(0.0)));
    }

    /// LTS5 (H3) — an extra field and an unknown name keep the known names; a foreign schema loads empty.
    #[test]
    fn lts5_finds_forward_compat_extra_field() {
        let s = r#"{"schema":"powrush_finds_v1","names":["Quiet Harmony Loom","no such find","Steady Climate Pick"],"later":{"x":1}}"#;
        assert_eq!(FoundFinds::decode(Some(s)), two());
        in_user_dir("lts5", |dir| {
            std::fs::write(dir.join(FINDS_SAVE), s).unwrap();
            assert_eq!(loaded(), two());
        });
        assert_eq!(FoundFinds::decode(Some(r#"{"schema":"powrush_house_v1","names":["Steady Climate Pick"]}"#)), FoundFinds::default());
    }

    /// GS1 (H3) — an owned `equipped` name in v1 bytes loads live, then from `.bak`, and survives a save and relaunch.
    /// No key, legacy newline bytes, an unowned, unknown or null name and a foreign schema all load `equipped: None`.
    #[test]
    fn gs1_equipped_saves_in_v1_and_loads_fail_closed() {
        let v1 = |eq: &str| format!(r#"{{"schema":"powrush_finds_v1","names":["Steady Climate Pick","Quiet Harmony Loom"],"equipped":{eq}}}"#);
        let (loom, loom_v1) = (FoundFinds { equipped: Some(NAMED_FINDS[4].name), ..two() }, v1(r#""Quiet Harmony Loom""#));
        assert_eq!((FoundFinds::decode(Some(&loom_v1)), loom.encode()), (loom.clone(), loom_v1.clone()));
        for eq in [r#""Yard-Worn Tend Hook""#, r#""no such find""#, "null"] {
            assert_eq!(FoundFinds::decode(Some(&v1(eq))), two(), "{eq}");
        }
        assert_eq!(FoundFinds::decode(Some(&two().encode())), two(), "v1 with no key");
        assert_eq!(FoundFinds::decode(std::str::from_utf8(OLD_FINDS).ok()), two(), "legacy newline");
        let foreign = r#"{"schema":"powrush_house_v1","names":["Quiet Harmony Loom"],"equipped":"Quiet Harmony Loom"}"#;
        assert_eq!(FoundFinds::decode(Some(foreign)), FoundFinds::default());
        in_user_dir("gs1", |dir| {
            let (live, bak) = (dir.join(FINDS_SAVE), dir.join(format!("{FINDS_SAVE}.bak")));
            std::fs::write(&live, &loom_v1).unwrap();
            assert_eq!(loaded(), loom);
            std::fs::rename(&live, &bak).unwrap();
            assert_eq!(loaded(), loom, "owned slot from .bak");
            std::fs::write(&live, v1(r#""no such find""#)).unwrap();
            assert_eq!(loaded(), two(), "unknown name: names kept, slot empty");
            save_finds(&loom);
            assert_eq!(loaded(), loom, "save then relaunch keeps the slot");
            std::fs::write(&live, OLD_FINDS).unwrap();
            assert_eq!(loaded(), two(), "legacy newline bytes");
        });
    }

    /// GS3 — one slot: a second owned equip replaces the first, an unowned or unknown name is a no-op, and clear empties it.
    /// `drop_once` rolls the same with or without a slot and never moves it.
    #[test]
    fn gs3_one_slot_replaces_and_clears() {
        let mut f = two();
        assert!(f.equip_named(NAMED_FINDS[1].name) && f.equipped == Some(NAMED_FINDS[1].name));
        assert!(f.equip_named(NAMED_FINDS[4].name) && f.equipped == Some(NAMED_FINDS[4].name), "the second equip replaces the first");
        for name in [NAMED_FINDS[0].name, "no such find", ""] {
            assert!(!f.equip_named(name) && f.equipped == Some(NAMED_FINDS[4].name), "{name:?} is a no-op");
        }
        assert_eq!(f.encode().matches(r#""equipped":"#).count(), 1);
        f.clear_equipped();
        assert!(f == two() && f.equipped_find().is_none() && !FoundFinds::default().equip_named(NAMED_FINDS[4].name));
        for s in 0..512 {
            let (mut a, mut b) = (FoundFinds { equipped: Some(NAMED_FINDS[4].name), ..two() }, two());
            assert_eq!(a.drop_once(DifficultyTier::Sworn, s, 1), b.drop_once(DifficultyTier::Sworn, s, 1));
            assert_eq!((a.names, a.pending, a.equipped), (b.names, b.pending, Some(NAMED_FINDS[4].name)));
        }
    }
}
