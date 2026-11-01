//! DIFFICULTY-TIERS-1. Area difficulty tiers, the unlock gate and the save shape. Sheltered is the
//! default and plays as before. Only the gate is live; Unbroken damage is inactive data nothing reads.
use bevy::prelude::*;
use shared::hex_travel::PlaceId;
use std::sync::atomic::{AtomicU8, Ordering};
use crate::combat_core::predator_present;
use crate::local_session_persist::LocalSessionPersist;
use crate::skill_xp::{total_level, SkillXp};
use crate::wellsworn::{WellswornRank, WellswornState};

pub const SWORN_TOTAL_LEVEL: u32 = 15;
pub const UNBROKEN_TOTAL_LEVEL: u32 = 30;
/// Unbroken enemy damage over base, in percent, and its hard max. Read by `enemy_hit`.
pub const UNBROKEN_DAMAGE_PCT: u32 = 15;
pub const DAMAGE_HARD_MAX_PCT: u32 = 25;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum DifficultyTier { #[default] Sheltered, Steady, Sworn, Unbroken }
pub const TIERS: [DifficultyTier; 4] = [DifficultyTier::Sheltered, DifficultyTier::Steady, DifficultyTier::Sworn, DifficultyTier::Unbroken];
impl DifficultyTier {
    pub const fn name(self) -> &'static str { ["Sheltered", "Steady", "Sworn", "Unbroken"][self as usize] }
    /// What picking this tier changes. In this card that is only who may pick it.
    pub const fn note(self) -> &'static str {
        match self {
            Self::Sheltered => "the usual game, always open",
            Self::Steady => "always open, plays like Sheltered for now",
            Self::Sworn => "opens at total level 15 after a Sheltered or Steady clear here",
            Self::Unbroken => "opens at total level 30 after a Sworn clear here, once you swear at the well",
        }
    }
    const fn bit(self) -> u8 { 1 << self as u8 }
    pub fn damage_scale(self) -> f32 {
        1.0 + if self == Self::Unbroken { clamp_damage_pct(UNBROKEN_DAMAGE_PCT) as f32 / 100.0 } else { 0.0 }
    }
}
pub fn clamp_damage_pct(pct: u32) -> u32 { pct.min(DAMAGE_HARD_MAX_PCT) }
/// DIFFICULTY-CUES-1. Sworn and Unbroken hide the ground ring and the enemy HP number.
pub const fn hides_cues(tier: DifficultyTier) -> bool { matches!(tier, DifficultyTier::Sworn | DifficultyTier::Unbroken) }
/// Enemy hit on this tier, integer percent rounded half up, never over round(base x 1.25).
pub const fn enemy_hit(tier: DifficultyTier, base: u32) -> u32 {
    let DifficultyTier::Unbroken = tier else { return base };
    let pct = if UNBROKEN_DAMAGE_PCT < DAMAGE_HARD_MAX_PCT { UNBROKEN_DAMAGE_PCT } else { DAMAGE_HARD_MAX_PCT };
    let (hit, cap) = ((base * (100 + pct) + 50) / 100, (base * 125 + 50) / 100);
    if hit < cap { hit } else { cap }
}
/// The tell sound and text fire once, on the frame the tell window opens.
pub fn tell_onset(was: &mut bool, now: bool) -> bool { !std::mem::replace(was, now) && now }
/// Tiers 3-4 drop only the " {left} / {health}" number from a pulse line.
pub fn cue_line(tier: DifficultyTier, line: String, left: u32, health: u32) -> String {
    if hides_cues(tier) { line.replacen(&format!(" {left} / {health}"), "", 1) } else { line }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PickError { Locked, StepUpOnlyAtEntry }
/// What the gate reads: total level and whether the Wellsworn oath is taken.
#[derive(Debug, Clone, Copy, Default)]
pub struct GateFacts { pub total_level: u32, pub wellsworn: bool }
pub fn wellsworn_ok(rank: Option<WellswornRank>) -> bool { matches!(rank, Some(WellswornRank::Sworn | WellswornRank::Stave | WellswornRank::Wellkeeper)) }
pub const fn slot(place: PlaceId) -> usize { match place { PlaceId::Sanctuary => 0, PlaceId::Heartwood => 1, PlaceId::Depths => 2 } }
pub fn gate(tier: DifficultyTier, place: PlaceId, clears: u8, f: GateFacts) -> Result<(), PickError> {
    use DifficultyTier::*;
    let open = match tier {
        Sheltered => true,
        _ if place == PlaceId::Sanctuary => false,
        Steady => true,
        Sworn => f.total_level >= SWORN_TOTAL_LEVEL && clears & (Sheltered.bit() | Steady.bit()) != 0,
        Unbroken => f.total_level >= UNBROKEN_TOTAL_LEVEL && clears & Sworn.bit() != 0 && f.wellsworn,
    };
    open.then_some(()).ok_or(PickError::Locked)
}
/// Per save: default tier, per-area override, tiers each area was cleared on, and a door pick for entry.
#[derive(Resource, Debug, Default, Clone, PartialEq)]
pub struct AreaDifficulty { pub default: DifficultyTier, pub area: [Option<DifficultyTier>; 3], pub clears: [u8; 3], pub pending: [Option<DifficultyTier>; 3],
    /// DIFFICULTY-REWARDS-1. This life stepped down from Unbroken here, so its in-progress Unbroken mark is gone.
    pub stepped_down: [bool; 3] }
impl AreaDifficulty {
    pub fn tier(&self, place: PlaceId) -> DifficultyTier {
        if place == PlaceId::Sanctuary { DifficultyTier::Sheltered } else { self.area[slot(place)].unwrap_or(self.default) }
    }
    pub fn shown(&self, place: PlaceId) -> DifficultyTier { self.pending[slot(place)].unwrap_or(self.tier(place)) }
    /// Lower is always allowed. Higher only at area entry, and only once unlocked.
    pub fn pick(&mut self, place: PlaceId, tier: DifficultyTier, at_entry: bool, f: GateFacts) -> Result<(), PickError> {
        if tier > self.tier(place) {
            if !at_entry { return Err(PickError::StepUpOnlyAtEntry); }
            gate(tier, place, self.clears[slot(place)], f)?;
        }
        let now = self.tier(place);
        if tier < now && now == DifficultyTier::Unbroken { self.stepped_down[slot(place)] = true; } // never earned marks or XP
        self.area[slot(place)] = Some(tier);
        Ok(())
    }
    pub fn record_clear(&mut self, place: PlaceId) {
        let t = self.tier(place);
        if !(t == DifficultyTier::Unbroken && self.stepped_down[slot(place)]) { self.clears[slot(place)] |= t.bit(); }
    }
    /// A life runs from spawn or respawn to the next defeat; the defeat starts a fresh one.
    pub fn new_life(&mut self) { self.stepped_down = [false; 3]; }
    /// Earned area difficulty marks per area, read from `clears` (the saved earned state).
    pub fn marks(&self) -> [Option<TierMark>; 3] { self.clears.map(TierMark::earned) }
    /// Door picker: the next tier that is lower (lands now) or unlocked (lands on entry).
    pub fn cycle_at_door(&mut self, place: PlaceId, f: GateFacts) {
        for t in (1..=4).map(|n| TIERS[(self.shown(place) as usize + n) % 4]) {
            if t <= self.tier(place) { self.pending[slot(place)] = None; let _ = self.pick(place, t, false, f); return; }
            if gate(t, place, self.clears[slot(place)], f).is_ok() { self.pending[slot(place)] = Some(t); return; }
        }
    }
    /// Area entry (apply_place): areas with no hostile clear on visit, then the door pick lands.
    pub fn enter(&mut self, place: PlaceId, f: GateFacts) {
        if !predator_present(place) { self.record_clear(place); }
        if let Some(t) = self.pending[slot(place)].take() { let _ = self.pick(place, t, true, f); }
    }
    pub fn to_save(&self) -> (u8, [u8; 3], [u8; 3]) { (self.default as u8, self.area.map(|t| t.map_or(0, |t| t as u8 + 1)), self.clears) }
    pub fn from_save(default: Option<u8>, area: Option<[u8; 3]>, clears: Option<[u8; 3]>) -> Self {
        let tier = |n: u8| TIERS.get(n as usize).copied();
        let area = area.unwrap_or_default().map(|n| n.checked_sub(1).and_then(tier));
        Self { default: default.and_then(tier).unwrap_or_default(), area, clears: clears.unwrap_or_default(), pending: [None; 3], stepped_down: [false; 3] }
    }
}
/// DIFFICULTY-REWARDS-1. Combat XP per tier in integer percent, and its hard max.
pub const COMBAT_XP_PCT: [u32; 4] = [100, 115, 130, 150];
pub const XP_HARD_MAX_PCT: u32 = 175;
const _: () = {
    let mut i = 0;
    while i < 4 { assert!(COMBAT_XP_PCT[i] >= 100 && COMBAT_XP_PCT[i] <= XP_HARD_MAX_PCT); i += 1; }
    assert!(COMBAT_XP_PCT[0] == 100);
};
/// Never below x1.00 (Sheltered is never reduced), never above the hard max.
pub const fn clamp_xp_pct(pct: u32) -> u32 { if pct > XP_HARD_MAX_PCT { XP_HARD_MAX_PCT } else if pct < 100 { 100 } else { pct } }
/// One combat gain on this tier, rounded half up: 10 -> 10/12/13/15.
pub const fn combat_xp(tier: DifficultyTier, base: u32) -> u32 { (base * clamp_xp_pct(COMBAT_XP_PCT[tier as usize]) + 50) / 100 }
/// Inactive drop data until LOOT-DROPS-1. Nothing reads these outside tests.
pub const DROP_CHANCE_PCT: [u32; 4] = [100, 115, 130, 150];
pub const DROP_CHANCE_MAX: f32 = 0.95;
pub const DROP_QTY_PCT: [u32; 4] = [100, 100, 125, 150];
pub const DROP_QTY_HARD_MAX_PCT: u32 = 200;
pub const DROP_QUALITY_STEPS: [u8; 4] = [0, 1, 2, 3];
pub const UNBROKEN_COSMETIC_ONLY_POOL: bool = true;
const _: () = {
    let mut i = 0;
    while i < 4 { assert!(DROP_QTY_PCT[i] >= 100 && DROP_QTY_PCT[i] <= DROP_QTY_HARD_MAX_PCT && DROP_CHANCE_PCT[i] <= XP_HARD_MAX_PCT); i += 1; }
};
/// Run modifiers (#738 CUT 6555509394). Inactive opt-in run modifiers: more threat for more combat XP, never less reward.
/// Integer data and pure fns only; nothing reads these outside tests and no system is wired.
/// Ids are internal with no on-screen names; earning a modifier waits for a later earn card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunModifier { pub id: u8, pub xp_bonus_pct: u32, pub foe_factor: u32, pub damage_pct: u32 }
pub const RUN_MODIFIERS: [RunModifier; 2] = [
    RunModifier { id: 0, xp_bonus_pct: 25, foe_factor: 1, damage_pct: 10 },
    RunModifier { id: 1, xp_bonus_pct: 30, foe_factor: 2, damage_pct: 0 },
];
/// Design ceiling on foes in one fight (forward constraint), and today's fixed spawns: one predator, one droid.
pub const MAX_FOES_PER_FIGHT: u32 = 4;
pub const BASE_FOES_PER_FIGHT: u32 = 2;
const _: () = {
    let (mut i, mut foes, mut dmg) = (0, BASE_FOES_PER_FIGHT, UNBROKEN_DAMAGE_PCT);
    while i < RUN_MODIFIERS.len() {
        let m = RUN_MODIFIERS[i];
        assert!(m.xp_bonus_pct > 0 && m.foe_factor >= 1);
        assert!(m.foe_factor > 1 || m.damage_pct > 0);
        assert!(UNBROKEN_DAMAGE_PCT + m.damage_pct <= DAMAGE_HARD_MAX_PCT && BASE_FOES_PER_FIGHT * m.foe_factor <= MAX_FOES_PER_FIGHT);
        (foes, dmg) = (foes * m.foe_factor, dmg + m.damage_pct);
        i += 1;
    }
    assert!(foes <= MAX_FOES_PER_FIGHT && dmg <= DAMAGE_HARD_MAX_PCT);
};
/// Steady or higher only; Sheltered rejects every non-empty set. The set must stay under the foe and damage ceilings.
pub const fn run_modifiers_allowed(tier: DifficultyTier, mods: &[RunModifier]) -> bool {
    if mods.is_empty() { return true; }
    if (tier as u8) < (DifficultyTier::Steady as u8) { return false; }
    let (mut i, mut foes, mut dmg) = (0, BASE_FOES_PER_FIGHT, UNBROKEN_DAMAGE_PCT);
    while i < mods.len() {
        if mods[i].foe_factor == 0 { return false; }
        (foes, dmg) = (foes.saturating_mul(mods[i].foe_factor), dmg.saturating_add(mods[i].damage_pct));
        i += 1;
    }
    foes <= MAX_FOES_PER_FIGHT && dmg <= DAMAGE_HARD_MAX_PCT
}
/// Tier XP plus every modifier bonus, saturating, through `clamp_xp_pct`. A rejected set earns the tier base only.
pub const fn stacked_xp_pct(tier: DifficultyTier, mods: &[RunModifier]) -> u32 {
    let mut pct = COMBAT_XP_PCT[tier as usize];
    if run_modifiers_allowed(tier, mods) {
        let mut i = 0;
        while i < mods.len() { pct = pct.saturating_add(mods[i].xp_bonus_pct); i += 1; }
    }
    clamp_xp_pct(pct)
}
pub fn drop_chance(tier: DifficultyTier, base: f32) -> f32 { (base.max(0.0) * DROP_CHANCE_PCT[tier as usize] as f32 / 100.0).min(DROP_CHANCE_MAX) }
pub const fn clamp_qty_pct(pct: u32) -> u32 { if pct > DROP_QTY_HARD_MAX_PCT { DROP_QTY_HARD_MAX_PCT } else if pct < 100 { 100 } else { pct } }
pub const fn drop_qty_at(pct: u32, base: u32) -> u32 { let q = (base * clamp_qty_pct(pct) + 50) / 100; if q < 1 { 1 } else { q } }
pub const fn drop_qty(tier: DifficultyTier, base: u32) -> u32 { drop_qty_at(DROP_QTY_PCT[tier as usize], base) }
/// Earned by clearing an area on Sworn or Unbroken. Player copy: "area difficulty mark".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TierMark { Sworn, Unbroken }
impl TierMark {
    pub const COPY: &'static str = "area difficulty mark";
    pub const fn earned(clears: u8) -> Option<Self> {
        if clears & DifficultyTier::Unbroken.bit() != 0 { Some(Self::Unbroken) } else if clears & DifficultyTier::Sworn.bit() != 0 { Some(Self::Sworn) } else { None }
    }
    pub fn line(self) -> String { format!("{}: {}", Self::COPY, if self == Self::Sworn { "Sworn" } else { "Unbroken" }) }
}
pub fn row_line(tier: DifficultyTier, at_door: bool) -> String {
    let hint = if at_door { "press to choose for entry" } else { "press to lower" };
    format!("area difficulty: {} · {} · {hint}", tier.name(), tier.note())
}
static ENTERED: AtomicU8 = AtomicU8::new(0);
/// Called from hex_travel::apply_place, the one area-entry path.
pub fn mark_entry(place: PlaceId) { ENTERED.store(slot(place) as u8 + 1, Ordering::SeqCst); }
pub fn gate_facts(skills: Option<&SkillXp>, ww: Option<&WellswornState>) -> GateFacts {
    GateFacts { total_level: skills.map_or(0, |s| total_level(&s.xp)), wellsworn: ww.is_some_and(|w| wellsworn_ok(w.rank)) }
}
fn apply_entry(mut diff: ResMut<AreaDifficulty>, skills: Option<Res<SkillXp>>, ww: Option<Res<WellswornState>>) {
    let Some(&place) = shared::hex_travel::LOCAL_HEXES.get((ENTERED.swap(0, Ordering::SeqCst) as usize).wrapping_sub(1)) else { return };
    diff.enter(place, gate_facts(skills.as_deref(), ww.as_deref()));
}
fn mark_save(diff: Res<AreaDifficulty>, persist: Option<ResMut<LocalSessionPersist>>) {
    if let (true, Some(mut p)) = (diff.is_changed(), persist) { p.dirty = true; }
}
pub struct DifficultyPlugin;
impl Plugin for DifficultyPlugin {
    fn build(&self, app: &mut App) { app.init_resource::<AreaDifficulty>().add_systems(Update, (apply_entry, mark_save).chain()); }
}

#[cfg(test)]
mod tests {
    use super::*;
    use DifficultyTier::*;
    const H: PlaceId = PlaceId::Heartwood;
    const HIGH: GateFacts = GateFacts { total_level: 40, wellsworn: true };
    #[test]
    fn default_is_sheltered_and_locked_tier_cannot_be_picked() {
        let mut d = AreaDifficulty::from_save(None, None, None);
        assert!(shared::hex_travel::LOCAL_HEXES.iter().all(|&p| d.tier(p) == Sheltered));
        d.clears[slot(H)] = Sheltered.bit();
        assert_eq!(d.pick(H, Sworn, true, GateFacts { total_level: 14, wellsworn: true }), Err(PickError::Locked));
        assert_eq!(d.pick(H, Unbroken, true, HIGH), Err(PickError::Locked), "needs a Sworn clear");
        assert_eq!(d.pick(PlaceId::Sanctuary, Steady, true, HIGH), Err(PickError::Locked));
        assert_eq!(d, AreaDifficulty { clears: [0, 1, 0], ..Default::default() });
        assert_eq!(d.pick(H, Sworn, true, HIGH), Ok(()));
    }
    #[test]
    fn step_down_mid_run_step_up_only_at_entry() {
        let mut d = AreaDifficulty { area: [None, Some(Sworn), None], ..Default::default() };
        assert_eq!(d.pick(H, Steady, false, HIGH).and(d.pick(H, Sheltered, false, GateFacts::default())), Ok(()));
        assert_eq!(d.pick(H, Steady, false, HIGH), Err(PickError::StepUpOnlyAtEntry));
        d.cycle_at_door(H, HIGH);
        assert_eq!((d.tier(H), d.shown(H)), (Sheltered, Steady), "door pick waits for entry");
        d.enter(H, HIGH);
        assert_eq!((d.tier(H), d.pending), (Steady, [None; 3]));
    }
    #[test]
    fn override_beats_default_for_that_area_only() {
        let mut d = AreaDifficulty { default: Steady, ..Default::default() };
        d.pick(PlaceId::Depths, Sheltered, false, HIGH).unwrap();
        assert_eq!((d.tier(PlaceId::Depths), d.tier(H), d.tier(PlaceId::Sanctuary)), (Sheltered, Steady, Sheltered));
    }
    #[test]
    fn visit_clears_only_where_no_hostile_and_save_round_trips() {
        let mut d = AreaDifficulty::default();
        (d.enter(PlaceId::Depths, HIGH), d.enter(H, HIGH));
        assert_eq!(d.clears, [0, 0, Sheltered.bit()]);
        (d.area[slot(H)] = Some(Steady), d.record_clear(H));
        let (t, a, c) = d.to_save();
        assert_eq!(AreaDifficulty::from_save(Some(t), Some(a), Some(c)), d);
    }
    #[test]
    fn damage_data_clamps_and_lower_tiers_stay_base() {
        assert_eq!((clamp_damage_pct(UNBROKEN_DAMAGE_PCT), clamp_damage_pct(90)), (15, 25));
        assert!(Unbroken.damage_scale() <= 1.25 && Sheltered.damage_scale() == 1.0 && Steady.damage_scale() == 1.0);
    }
    #[test]
    fn unbroken_wellsworn_gate_needs_sworn_or_higher() {
        assert!(!wellsworn_ok(None) && !wellsworn_ok(Some(WellswornRank::Drawn)));
        assert!([WellswornRank::Sworn, WellswornRank::Stave, WellswornRank::Wellkeeper].into_iter().all(|r| wellsworn_ok(Some(r))));
    }
    #[test]
    fn player_copy_says_area_difficulty_and_avoids_banned_words() {
        for line in TIERS.iter().flat_map(|&t| [row_line(t, true), row_line(t, false)]) {
            let low = line.to_lowercase();
            assert!(line.starts_with("area difficulty: ") && ["instance", "heroic", "mythic"].iter().all(|w| !low.contains(w)), "{line}");
        }
    }
    #[test]
    fn cues_damage_per_tier() {
        for (t, a, b) in [(Sheltered, 15, 25), (Steady, 15, 25), (Sworn, 15, 25), (Unbroken, 17, 29)] {
            assert_eq!((enemy_hit(t, 15), enemy_hit(t, 25)), (a, b), "{t:?}");
            assert!(enemy_hit(t, 15) <= 19 && enemy_hit(t, 25) <= 31);
        }
        assert!(clamp_damage_pct(UNBROKEN_DAMAGE_PCT) <= DAMAGE_HARD_MAX_PCT && enemy_hit(Unbroken, 100) <= 125);
    }

    #[test]
    fn cues_tell_onset_once_per_wind_up() {
        let mut was = false;
        let fired: Vec<bool> = [false, true, true, true, false, true].iter().map(|&v| tell_onset(&mut was, v)).collect();
        assert_eq!(fired, [false, true, false, false, false, true]);
    }

    #[test]
    fn cues_hp_number_hidden_on_tiers_three_four_only() {
        for t in TIERS {
            let line = cue_line(t, "Foe 185 / 200 · +10 combat XP".into(), 185, 200);
            assert_eq!(line, if hides_cues(t) { "Foe · +10 combat XP" } else { "Foe 185 / 200 · +10 combat XP" });
            assert_eq!(cue_line(t, "Missed Foe".into(), 200, 200), "Missed Foe");
        }
        assert!(!hides_cues(Sheltered) && !hides_cues(Steady) && hides_cues(Sworn) && hides_cues(Unbroken));
    }

    #[test]
    fn cues_leave_accessibility_flags_alone() {
        let s = shared::local_settings::LocalSettings::peace_defaults();
        let (before, ui) = (s.clone(), crate::local_settings::LocalUiFeel::from_settings(&s));
        for t in TIERS {
            let _ = (hides_cues(t), enemy_hit(t, 15), cue_line(t, String::new(), 0, 0));
            assert_eq!(s, before);
            assert_eq!((s.reduced_motion, &s.colorblind_wells, s.key_use.display_label()), (before.reduced_motion, &before.colorblind_wells, before.key_use.display_label()));
            assert_eq!(crate::local_settings::LocalUiFeel::from_settings(&s).text_scale, ui.text_scale);
        }
    }

    #[test]
    fn rewards_combat_xp_per_tier_and_hard_max() {
        assert_eq!(TIERS.map(|t| combat_xp(t, 10)), [10, 12, 13, 15]);
        assert_eq!(combat_xp(Sheltered, 10), 10);
        assert_eq!((clamp_xp_pct(176), clamp_xp_pct(500), clamp_xp_pct(175), clamp_xp_pct(50)), (175, 175, 175, 100));
        assert!(COMBAT_XP_PCT.iter().all(|&p| (100..=XP_HARD_MAX_PCT).contains(&p)));
    }
    #[test]
    fn rewards_drop_data_clamps() {
        for t in TIERS {
            assert!((0..=20).all(|n| drop_chance(t, n as f32 / 10.0) <= 0.95));
            assert!((0..=50).all(|b| drop_qty(t, b) >= 1 && drop_qty(t, b) <= (2 * b).max(1)));
        }
        assert_eq!((drop_qty_at(400, 10), drop_qty_at(0, 0), drop_qty(Unbroken, 2), drop_qty(Sworn, 4)), (20, 1, 3, 5));
        assert_eq!((DROP_QUALITY_STEPS, UNBROKEN_COSMETIC_ONLY_POOL), ([0, 1, 2, 3], true));
    }
    #[test]
    fn rewards_step_down_clears_only_this_lifes_unbroken_mark() {
        let mut d = AreaDifficulty { area: [None, Some(Unbroken), None], clears: [0, Sworn.bit(), 0], ..Default::default() };
        assert_eq!(d.marks()[slot(H)], Some(TierMark::Sworn));
        d.pick(H, Sworn, false, HIGH).unwrap();
        d.pick(H, Unbroken, true, HIGH).unwrap();
        d.record_clear(H);
        assert_eq!(d.marks()[slot(H)], Some(TierMark::Sworn), "stepped down this life: no Unbroken mark, earned Sworn mark kept");
        d.new_life();
        d.record_clear(H);
        assert_eq!(d.marks()[slot(H)], Some(TierMark::Unbroken));
        d.pick(H, Sheltered, false, HIGH).unwrap();
        assert_eq!(d.marks()[slot(H)], Some(TierMark::Unbroken), "stepping down never removes an earned mark");
        assert!(TierMark::Unbroken.line() == "area difficulty mark: Unbroken");
    }
    #[test]
    fn run_modifiers_stack_through_clamp_per_tier() {
        let [a, b] = RUN_MODIFIERS;
        assert_eq!(TIERS.map(|t| stacked_xp_pct(t, &[])), COMBAT_XP_PCT);
        assert_eq!(TIERS.map(|t| stacked_xp_pct(t, &[a])), [100, 140, 155, 175]);
        assert_eq!(TIERS.map(|t| stacked_xp_pct(t, &[b])), [100, 145, 160, 175]);
        assert_eq!(TIERS.map(|t| stacked_xp_pct(t, &[a, b])), [100, 170, 175, 175]);
        assert!(RUN_MODIFIERS.iter().all(|m| m.xp_bonus_pct >= 25 && stacked_xp_pct(Unbroken, &[*m]) == XP_HARD_MAX_PCT));
        let huge = RunModifier { id: 9, xp_bonus_pct: u32::MAX, foe_factor: 1, damage_pct: 1 };
        assert_eq!((stacked_xp_pct(Steady, &[huge, huge]), XP_HARD_MAX_PCT), (175, 175));
        assert_eq!(TIERS.map(|t| combat_xp(t, 10)), [10, 12, 13, 15]);
    }
    #[test]
    fn run_modifiers_rejected_on_sheltered_allowed_steady_up() {
        for m in RUN_MODIFIERS {
            assert_eq!(TIERS.map(|t| run_modifiers_allowed(t, &[m])), [false, true, true, true]);
        }
        assert!(TIERS.iter().all(|&t| run_modifiers_allowed(t, &[])));
        assert!(!run_modifiers_allowed(Sheltered, &RUN_MODIFIERS) && run_modifiers_allowed(Steady, &RUN_MODIFIERS));
    }
    #[test]
    fn run_modifiers_trade_threat_for_reward_under_ceilings() {
        for m in RUN_MODIFIERS {
            assert!(m.xp_bonus_pct > 0 && m.foe_factor >= 1 && (m.foe_factor > 1 || m.damage_pct > 0));
            assert!(UNBROKEN_DAMAGE_PCT + m.damage_pct <= DAMAGE_HARD_MAX_PCT);
            assert!(BASE_FOES_PER_FIGHT * m.foe_factor <= MAX_FOES_PER_FIGHT);
        }
        assert_eq!((BASE_FOES_PER_FIGHT, MAX_FOES_PER_FIGHT), (2, 4));
        let [a, b] = RUN_MODIFIERS;
        assert!(!run_modifiers_allowed(Steady, &[b, b]) && !run_modifiers_allowed(Steady, &[a, a]));
        assert_eq!(enemy_hit(Unbroken, 100), 115);
    }
}
