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
/// Unbroken enemy damage over base, in percent, and its hard max. Inactive: nothing reads it yet.
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
pub struct AreaDifficulty { pub default: DifficultyTier, pub area: [Option<DifficultyTier>; 3], pub clears: [u8; 3], pub pending: [Option<DifficultyTier>; 3] }
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
        self.area[slot(place)] = Some(tier);
        Ok(())
    }
    pub fn record_clear(&mut self, place: PlaceId) { self.clears[slot(place)] |= self.tier(place).bit(); }
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
        Self { default: default.and_then(tier).unwrap_or_default(), area, clears: clears.unwrap_or_default(), pending: [None; 3] }
    }
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
}
