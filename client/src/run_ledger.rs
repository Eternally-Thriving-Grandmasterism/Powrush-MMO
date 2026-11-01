//! RUN-LEDGER-1. Your best Heartwood clear per area and tier: clear time, damage taken, tells dodged.
//! Read-only over combat, droid, travel and difficulty state. Kept in the local session save only.
use bevy::prelude::*;
use serde::{Deserialize, Deserializer};
use shared::hex_travel::PlaceId;
use crate::combat_core::{in_reach, predator_present, tell_visible, CombatState, HEALTH};
use crate::difficulty::{slot, AreaDifficulty, DifficultyTier};
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
    fn build(&self, app: &mut App) { app.init_resource::<RunLedger>().add_systems(Update, watch_runs); }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
