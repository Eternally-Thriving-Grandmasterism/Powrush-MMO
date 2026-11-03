//! RUN-LEDGER-1. Your best Heartwood clear per area and tier: clear time, damage taken, tells dodged.
//! Read-only over combat, droid, travel and difficulty state. Kept in the local session save only.
use bevy::prelude::*;
use serde::{Deserialize, Deserializer};
use shared::hex_travel::PlaceId;
use crate::combat_core::{in_reach, predator_present, tell_visible, CombatState, HEALTH};
use crate::difficulty::{clamp_xp_pct, combat_xp, slot, AreaDifficulty, DifficultyTier, RunModifier, COMBAT_XP_PCT};
use crate::droid::{droid_in_reach, shot_armed, shot_tell_visible, DroidState};
use crate::{hex_travel::HexTravelState, human_presence::SoftPresence, title_screen::LaunchDoor};
/// One best row: `[clear_ms, damage_taken, tells_dodged]`. `clear_ms == 0` means no row yet.
pub type RunRow = [u32; 3];
/// Saved shape, indexed `[difficulty::slot(place)][tier as usize]`.
pub type RunBestSave = [[RunRow; 4]; 3];
#[derive(Resource, Debug, Default, Clone, PartialEq)]
pub struct RunLedger { pub best: RunBestSave }
impl RunLedger {
    pub fn row(&self, place: PlaceId, tier: DifficultyTier) -> Option<RunRow> { Some(self.best[slot(place)][tier as usize]).filter(|r| r[0] > 0) }
    /// Fastest clear wins; a tie goes to less damage taken. Dodges never rank.
    pub fn offer(&mut self, place: PlaceId, tier: DifficultyTier, row: RunRow) -> bool {
        let better = self.row(place, tier).is_none_or(|b| (row[0], row[1]) < (b[0], b[1]));
        if better { self.best[slot(place)][tier as usize] = row; }
        better
    }
}
/// SessionBlob reader: any bad value loads as no ledger and the rest of the session stays.
pub fn lenient<'de, D: Deserializer<'de>>(d: D) -> Result<Option<RunBestSave>, D::Error> { Ok(serde_json::Value::deserialize(d).ok().and_then(|v| serde_json::from_value(v).ok())) }
/// One frame as the ledger sees it. `tier` is `None` while an entry door pick has not landed yet.
#[derive(Debug, Clone, Copy)]
pub struct Seen { pub place: PlaceId, pub tier: Option<DifficultyTier>, pub player_taken: u32, pub predator_taken: u32, pub tell: bool, pub shot_tell: bool, pub dt: f32 }
#[derive(Debug, Clone, Copy)]
struct Run { tier: Option<DifficultyTier>, secs: f32, damage: u32, dodged: u32 }
/// The ledger's own last-seen place and frame, plus the open run of this visit.
#[derive(Debug, Default)]
pub struct RunTrack { place: Option<PlaceId>, last: Option<Seen>, open: Option<Run> }
impl RunTrack {
    /// One frame. Returns `(tier, row)` on the frame of the first predator clear of this visit.
    pub fn step(&mut self, s: Seen) -> Option<(DifficultyTier, RunRow)> {
        let (entered, prev) = (self.place.replace(s.place) != Some(s.place), self.last.replace(s));
        if entered || !predator_present(s.place) {
            self.open = predator_present(s.place).then_some(Run { tier: s.tier, secs: 0.0, damage: 0, dodged: 0 });
            return None;
        }
        let (Some(run), Some(p)) = (self.open.as_mut(), prev) else { return None };
        let tier_moved = run.tier.zip(s.tier).is_some_and(|(a, b)| a != b);
        if tier_moved || s.player_taken >= HEALTH || s.player_taken < p.player_taken {
            return self.open.take().and(None); // tier change, defeat or new life: void, nothing written
        }
        run.tier = run.tier.or(s.tier);
        run.secs += s.dt.max(0.0);
        let rose = s.player_taken - p.player_taken;
        run.damage += rose; // a frame where a hit lands never counts a dodge
        if rose == 0 { run.dodged += (p.tell && !s.tell) as u32 + (p.shot_tell && !s.shot_tell) as u32; }
        let Some(tier) = run.tier.filter(|_| s.predator_taken >= HEALTH && p.predator_taken < HEALTH) else { return None };
        let row = [((run.secs * 1000.0).round() as u32).max(1), run.damage, run.dodged];
        self.open.take().map(|_| (tier, row)) // closed: the respawn in this visit opens no second run
    }
}
fn watch_runs(time: Res<Time>, door: Option<Res<LaunchDoor>>, (travel, presence): (Option<Res<HexTravelState>>, Option<Res<SoftPresence>>),
    (combat, droid, diff): (Option<Res<CombatState>>, Option<Res<DroidState>>, Option<Res<AreaDifficulty>>), mut track: Local<RunTrack>, mut ledger: ResMut<RunLedger>) {
    let (Some(travel), Some(combat)) = (travel, combat) else { return };
    let place = travel.current;
    let gate = predator_present(place) && door.as_deref().copied() == Some(LaunchDoor::InYard);
    let p = presence.map(|p| p.position);
    let reach = gate && p.is_some_and(|p| in_reach(place, &combat, p.x, p.z));
    let shot_tell = droid.as_deref().is_some_and(|d| {
        let near = gate && p.is_some_and(|p| droid_in_reach(place, d, p.x, p.z));
        shot_tell_visible(d.shot_timer, shot_armed(d, combat.player_taken, gate, near))
    });
    let tier = diff.as_deref().map_or(Some(DifficultyTier::Sheltered), |d| d.pending[slot(place)].is_none().then(|| d.tier(place)));
    let s = Seen { place, tier, player_taken: combat.player_taken, predator_taken: combat.predator_taken,
        tell: tell_visible(combat.lunge_timer, reach), shot_tell, dt: time.delta_secs() };
    if let Some((tier, row)) = track.step(s) { ledger.offer(place, tier, row); }
}
pub struct RunLedgerPlugin;
impl Plugin for RunLedgerPlugin {
    fn build(&self, app: &mut App) { app.init_resource::<RunLedger>().add_systems(Update, watch_runs.after(crate::combat_core::combat_fire)); }
}
/// RUN-LEDGER-REWARD-1. Skill curve steepness `a`; proposed, card-gated.
pub const SKILL_CURVE_A: f32 = 3.0;
/// Dodge saturation `k` in `1 - exp(-dodged / k)`; proposed, card-gated.
pub const DODGE_K: f32 = 3.0;
/// Weight of the time term; proposed, card-gated.
pub const W_TIME: f32 = 0.40;
/// Weight of the damage term; proposed, card-gated.
pub const W_DAMAGE: f32 = 0.40;
/// Weight of the dodge term; proposed, card-gated.
pub const W_DODGE: f32 = 0.20;
/// Par clear time in ms; a clear at or under par scores time 1. Proposed, card-gated.
pub const RUN_PAR_MS: u32 = 60_000;
/// Skill multiplier ceiling per `DifficultyTier as usize`; proposed, card-gated.
pub const SKILL_CAP: [f32; 4] = [1.20, 1.45, 1.85, 3.00];
pub fn skill_cap(tier: DifficultyTier) -> f32 {
    SKILL_CAP[tier as usize]
}
/// Skill score in [0, 1]: weighted time, damage and saturating dodge terms. No row scores 0.
pub fn skill_score(row: RunRow) -> f32 {
    if row[0] == 0 {
        return 0.0;
    }
    let time = (RUN_PAR_MS as f64 / row[0] as f64).min(1.0);
    let damage = 1.0 - row[1].min(HEALTH) as f64 / HEALTH as f64;
    let dodge = 1.0 - (-(row[2] as f64) / DODGE_K as f64).exp();
    (W_TIME as f64 * time + W_DAMAGE as f64 * damage + W_DODGE as f64 * dodge).clamp(0.0, 1.0)
        as f32
}
fn skill_curve(cap: f32, s: f32) -> f32 {
    let a = SKILL_CURVE_A as f64;
    let m = 1.0 + (cap as f64 - 1.0) * ((a * s as f64).exp() - 1.0) / (a.exp() - 1.0);
    (m as f32).clamp(1.0, cap)
}
/// Convex skill multiplier in [1.0, skill_cap(tier)]; a `clear_ms == 0` row is exactly 1.0.
/// Live through `premium_paid` (SKILL-PREMIUM-ON-1). Tier is a ceiling only, never a flat bonus.
/// RUN-LEDGER-WIRE-1 gives it its own bucket after clamp_xp_pct / XP_HARD_MAX_PCT 175, never
/// inside it: see `skill_premium` and `premium_xp_pct` below. Caps are tier-weighted so only
/// Unbroken reaches 3.0x. Live on Sherif's 2026-11-02 yes; the bucket can lift XP past 175
/// (525% max).
pub fn skill_mult(tier: DifficultyTier, row: RunRow) -> f32 {
    if row[0] == 0 {
        return 1.0;
    }
    skill_curve(skill_cap(tier), skill_score(row))
}
/// RUN-LEDGER-WIRE-1. Ceiling of the skill premium itself; its only cap.
pub const SKILL_PREMIUM_MAX: f32 = 3.0;
/// The skill premium: exactly 1.0 while any run modifier is set (a set Sheltered rejects too), else `skill_mult`.
pub fn skill_premium(tier: DifficultyTier, row: RunRow, mods: &[RunModifier]) -> f32 {
    if mods.is_empty() { skill_mult(tier, row) } else { 1.0 }
}
/// Its own bucket, applied after `clamp_xp_pct` 175 and never re-clamped: the result may pass 175 (525% max).
/// Earned XP and materials only; never loot rarity, the vendor ceiling, the upgrade meter or Lumen. Wired via `premium_paid`.
pub fn premium_xp_pct(clamped_pct: u32, premium: f32) -> f32 {
    clamped_pct as f32 * premium.clamp(1.0, SKILL_PREMIUM_MAX)
}
/// RUN-LEDGER-LIVE-1 / SKILL-PREMIUM-ON-1: live on Sherif's 2026-11-02 yes, 3.0x top. Sherif may veto.
pub const SKILL_PREMIUM_LIVE: bool = true;
/// One paid combat gain. Off or no stored best: exactly `combat_xp`. On: premium on the clamped pct, floored, never below `combat_xp`.
pub fn premium_paid(tier: DifficultyTier, base: u32, row: Option<RunRow>, live: bool) -> u32 {
    let today = combat_xp(tier, base);
    let Some(r) = row.filter(|_| live) else { return today };
    ((base as f32 * premium_xp_pct(clamp_xp_pct(COMBAT_XP_PCT[tier as usize]), skill_premium(tier, r, &[])) / 100.0) as u32).max(today)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difficulty::{stacked_xp_pct, RUN_MODIFIERS, XP_HARD_MAX_PCT};
    use DifficultyTier::*;
    const H: PlaceId = PlaceId::Heartwood;
    const O: Seen = Seen { place: H, tier: Some(Sheltered), player_taken: 0, predator_taken: 0, tell: false, shot_tell: false, dt: 1.0 };
    fn f(player_taken: u32, predator_taken: u32, tell: bool, shot_tell: bool) -> Seen { Seen { player_taken, predator_taken, tell, shot_tell, ..O } }
    fn run(frames: &[Seen]) -> Option<(DifficultyTier, RunRow)> {
        let mut t = RunTrack::default();
        [Seen { place: PlaceId::Sanctuary, ..O }].iter().chain(frames).fold(None, |_, s| t.step(*s))
    }
    #[test]
    fn dodge_counts_and_hit_does_not() {
        let dodge = [O, f(0, 0, true, false), O, f(0, HEALTH, false, false)];
        assert_eq!(run(&dodge), Some((Sheltered, [3000, 0, 1])));
        let hit = [O, f(0, 0, true, false), f(15, 0, false, false), f(15, HEALTH, false, false)];
        assert_eq!(run(&hit), Some((Sheltered, [3000, 15, 0])));
    }
    #[test]
    fn walking_away_from_both_tells_is_a_dodge() {
        let walk = [O, f(0, 0, true, true), O, f(0, HEALTH, false, false)];
        assert_eq!(run(&walk).map(|r| r.1[2]), Some(2));
    }
    #[test]
    fn tier_change_defeat_and_leaving_write_nothing() {
        let tier = [O, Seen { tier: Some(Steady), ..O }, f(0, HEALTH, false, false)];
        assert_eq!(run(&tier), None);
        assert_eq!(run(&[O, f(HEALTH, 0, false, false), f(0, HEALTH, false, false)]), None);
        assert_eq!(run(&[O, f(185, 0, false, false), O, f(0, HEALTH, false, false)]), None);
        assert_eq!(run(&[O, Seen { place: PlaceId::Depths, ..O }, f(0, HEALTH, false, false)]), None);
    }
    #[test]
    fn entry_pick_lands_then_tier_is_fixed_and_respawn_opens_no_second_run() {
        let door = [Seen { tier: None, ..O }, O, Seen { tier: Some(Steady), ..f(0, HEALTH, false, false) }];
        assert_eq!(run(&door), None, "tier read at run start, then a change voids");
        let again = [O, f(0, HEALTH, false, false), O, f(0, HEALTH, false, false)];
        assert_eq!(run(&again), None);
    }
    #[test]
    fn slower_never_replaces_and_tie_goes_to_less_damage() {
        let mut l = RunLedger::default();
        assert!(l.offer(H, Sworn, [5000, 30, 0]));
        assert!(!l.offer(H, Sworn, [6000, 0, 9]), "more dodges but slower");
        assert!(!l.offer(H, Sworn, [5000, 45, 0]));
        assert!(l.offer(H, Sworn, [5000, 15, 0]));
        assert_eq!((l.row(H, Sworn), l.row(H, Sheltered)), (Some([5000, 15, 0]), None));
    }
    const TIERS4: [DifficultyTier; 4] = [Sheltered, Steady, Sworn, Unbroken];
    #[test]
    fn skill_mult_floor_cap_and_safe_inputs() {
        let (worst, best) = ([u32::MAX, HEALTH * 2, 0], [1, 0, 1000]);
        for t in TIERS4 {
            let cap = skill_cap(t);
            assert_eq!((skill_curve(cap, 0.0), skill_mult(t, [0, 0, 0]), skill_mult(t, [0, 9, 9])), (1.0, 1.0, 1.0));
            assert!(skill_mult(t, worst) >= 1.0 && skill_mult(t, worst) < 1.0 + 1e-4, "{t:?}");
            assert!(skill_mult(t, best) <= cap && cap - skill_mult(t, best) < 1e-4, "{t:?}");
            assert!(skill_mult(t, [u32::MAX; 3]).is_finite() && skill_score([u32::MAX; 3]).is_finite());
            assert!(skill_curve(cap, 0.5) < (1.0 + cap) / 2.0, "convex: midpoint below chord");
        }
    }
    #[test]
    fn skill_score_monotonic_saturating_and_tier_is_ceiling_only() {
        let s = skill_score;
        assert!(s([120_000, 50, 2]) < s([90_000, 50, 2]) && s([90_000, 50, 2]) < s([60_000, 50, 2]));
        assert!(s([90_000, 100, 2]) < s([90_000, 50, 2]) && s([90_000, 50, 2]) < s([90_000, 50, 5]));
        assert!(s([1, 0, 40]) - s([1, 0, 20]) < 0.001 && (W_TIME + W_DAMAGE + W_DODGE - 1.0).abs() < 1e-6);
        assert!(SKILL_CAP[0] >= 1.0 && SKILL_CAP.windows(2).all(|w| w[0] < w[1]));
        assert!([Steady, Sworn, Unbroken].iter().all(|&t| skill_mult(Sheltered, [1, 0, 1000]) < skill_cap(t)));
    }
    #[test]
    fn premium_table_floor_ceiling_and_convex_bands() {
        let best = [1, 0, 1000];
        assert_eq!(SKILL_CAP, [1.20, 1.45, 1.85, 3.00], "council default table");
        let steps: Vec<f32> = SKILL_CAP.windows(2).map(|w| w[1] - w[0]).collect();
        assert!(steps.windows(2).all(|d| d[0] < d[1]) && SKILL_CAP[3] == SKILL_PREMIUM_MAX, "convex bands, only Unbroken at 3.0");
        assert!(SKILL_CAP[3] == SKILL_PREMIUM_MAX && SKILL_PREMIUM_MAX == 3.0, "top premium pinned at 3.0");
        for t in TIERS4 {
            assert_eq!(skill_premium(t, [0, 0, 0], &[]), 1.0, "no row is exactly 1.0");
            let (mid, top) = (skill_premium(t, [90_000, 50, 2], &[]), skill_premium(t, best, &[]));
            assert!(1.0 < mid && mid < top && (top - skill_cap(t)).abs() < 1e-4, "{t:?} monotonic to its own cap");
            let today = stacked_xp_pct(t, &[]);
            assert_eq!(premium_xp_pct(today, skill_premium(t, [0, 0, 0], &[])), today as f32, "default identical to today");
        }
        assert_eq!((premium_xp_pct(100, 0.0), premium_xp_pct(175, 9.0)), (100.0, 525.0), "never cuts; 3.0x is the only ceiling");
    }
    #[test]
    fn premium_never_stacks_with_mods_and_sits_after_the_175_clamp() {
        for t in TIERS4 {
            for mods in [&RUN_MODIFIERS[..1], &RUN_MODIFIERS[..]] {
                assert_eq!(skill_premium(t, [1, 0, 1000], mods), 1.0, "{t:?}: any non-empty set is 1.0, Sheltered-rejected too");
            }
        }
        assert_eq!(stacked_xp_pct(Unbroken, &[]), 150);
        assert!(premium_xp_pct(stacked_xp_pct(Unbroken, &[]), skill_premium(Unbroken, [1, 0, 1000], &[])) > XP_HARD_MAX_PCT as f32, "own bucket");
    }
    #[test]
    fn premium_paid_off_is_today_and_on_floors_the_premium() {
        const _: () = assert!(SKILL_PREMIUM_LIVE, "live on Sherif's 2026-11-02 yes");
        for t in TIERS4 {
            for row in [None, Some([0, 0, 0]), Some([90_000, 50, 2]), Some([1, 0, 1000])] {
                assert_eq!(premium_paid(t, 10, row, false), combat_xp(t, 10), "{t:?}: off is byte-identical");
            }
            assert_eq!(premium_paid(t, 10, None, true), combat_xp(t, 10), "{t:?}: no stored best pays today");
            for r in [[0, 0, 0], [90_000, 50, 2], [1, 0, 1000]] {
                let on = (10.0 * premium_xp_pct(clamp_xp_pct(COMBAT_XP_PCT[t as usize]), skill_premium(t, r, &[])) / 100.0) as u32;
                assert_eq!(premium_paid(t, 10, Some(r), true), on.max(combat_xp(t, 10)), "{t:?} {r:?}: parity");
                assert!(premium_paid(t, 10, Some(r), true) >= premium_paid(t, 10, Some(r), false), "{t:?} {r:?}: CT7 on never below off");
            }
            assert_eq!(premium_paid(t, 10, Some([0, 0, 0]), true), combat_xp(t, 10), "{t:?}: CT7 a 1.0 premium pays exactly today");
        }
        let raw = 3.0 * premium_xp_pct(100, skill_premium(Sheltered, [1, 0, 1000], &[])) / 100.0;
        assert!(raw > 3.5 && raw < 4.0 && premium_paid(Sheltered, 3, Some([1, 0, 1000]), true) == 3, "floor, never round up");
        assert_eq!(premium_paid(Unbroken, 10, Some([1, 0, 1000]), true), 45, "10 x 450% = 45 (skill_mult(Unbroken, best) is exactly 3.0 in f32)");
    }
}
