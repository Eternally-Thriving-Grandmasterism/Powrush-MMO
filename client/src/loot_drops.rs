//! LOOT-DROPS-1. Inactive named-find roll with caller-supplied rolls; nothing outside tests calls it.
//! Weights are council defaults (#738 CUT 6581050171), not Sherif's words; Sherif may veto or retune.
//! No odds are shown to players.
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
}
