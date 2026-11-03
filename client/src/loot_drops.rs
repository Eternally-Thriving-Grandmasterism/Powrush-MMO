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
pub struct FoundFinds { pub names: Vec<&'static str>, pub pending: Option<String> }
impl FoundFinds {
    /// One roll per defeat, role None. An owned pick is silent and a waiting line is never replaced. True when a new find lands.
    pub fn drop_once(&mut self, tier: DifficultyTier, strikes: u32, place: u32) -> bool {
        if self.pending.is_some() { return false; }
        let h = loot_hash(strikes, place, self.names.len() as u32);
        let Some(f) = roll_named_find(tier, LOOT_BASE_CHANCE, None, (h % LOOT_BPS) as u16, loot_hash(h, place, 1)) else { return false };
        if self.names.contains(&f.name) { return false; }
        self.names.push(f.name); self.pending = Some(find_line(f)); true
    }
    pub fn encode(&self) -> String { self.names.join("\n") }
    pub fn decode(s: Option<&str>) -> Self {
        let has = |n: &str| s.is_some_and(|s| s.lines().any(|l| l.trim() == n));
        FoundFinds { names: NAMED_FINDS.iter().map(|f| f.name).filter(|n| has(n)).collect(), pending: None }
    }
}
pub fn find_line(f: &NamedFind) -> String { format!("{} · {}", f.name, f.sentence) }
/// The waiting line shows only once the calm queue is done and the current pulse line has expired.
pub fn find_ready(calm_done: bool, pulse_until: f64, now: f64) -> bool { calm_done && pulse_until <= now }
pub fn load_finds(mut finds: ResMut<FoundFinds>) { *finds = FoundFinds::decode(shared::user_persist::read_named(FINDS_SAVE).ok().as_deref()); }
pub fn save_finds(finds: &FoundFinds) { let _ = shared::user_persist::write_named(FINDS_SAVE, finds.encode()); }
#[cfg(test)]
mod tests {
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
        let (mut found, all) = (0, FoundFinds { names: NAMED_FINDS.iter().map(|f| f.name).collect(), pending: None });
        for s in 0..2048 {
            let (mut a, mut b, mut c) = (FoundFinds::default(), FoundFinds::default(), all.clone());
            assert!(a.drop_once(DifficultyTier::Sworn, s, 1) == b.drop_once(DifficultyTier::Sworn, s, 1) && a == b && loot_hash(s, 1, 0) == loot_hash(s, 1, 0));
            assert!(!c.drop_once(DifficultyTier::Sworn, s, 1) && c == all, "all owned never drops");
            let Some(line) = a.pending.take() else { assert_eq!(a, FoundFinds::default()); continue };
            found += 1;
            assert!(NAMED_FINDS.iter().any(|n| n.name == a.names[0] && line == format!("{} · {}", n.name, n.sentence)));
            let mut w = FoundFinds { names: vec![], pending: Some(line) };
            assert!(!w.drop_once(DifficultyTier::Sworn, s, 1) && w.names.is_empty(), "no re-roll while a line waits");
            let mut o = FoundFinds { names: a.names.clone(), pending: None };
            if o.drop_once(DifficultyTier::Sworn, s, 0) { assert!(o.names[1] != o.names[0]) } else { assert!(o.names.len() == 1 && o.pending.is_none()) }
        }
        assert!(found > 0);
    }
    #[test]
    fn loot_drops_wire_save_round_trip_and_ready_gate() {
        let f = FoundFinds { names: vec![NAMED_FINDS[1].name, NAMED_FINDS[4].name], pending: Some("x".into()) };
        assert_eq!(FoundFinds::decode(Some(&f.encode())), FoundFinds { pending: None, ..f.clone() });
        assert!(FoundFinds::decode(Some("no such find\n")) == FoundFinds::default() && FoundFinds::decode(None) == FoundFinds::default());
        assert!(!find_ready(false, 0.0, 10.0) && !find_ready(true, 12.0, 10.0) && find_ready(true, 10.0, 10.0) && FINDS_SAVE == "powrush_finds.json");
    }
}
