//! ENEMY-DROID-1 — one hostile droid at Threshold (COMBAT_AND_PVP.md:479): a 0.5 s telegraphed shot for 25.
//! Client-local, never saved. Struck by the existing fire path via `CombatState::strikes`; pays 0 XP.

use {bevy::prelude::*, shared::hex_travel::PlaceId};
use crate::combat_core::{combat_fire, predator_present, respawn_step, set_pulse, CombatState, HEALTH, HIT, REACH};
use crate::combat_core::{LUNGE_TELL_DOTS, LUNGE_TELL_DOT_HEIGHT, LUNGE_TELL_DOT_RADIUS, LUNGE_TELL_LIFT};
use crate::{first_harvest_epiphany::FirstHarvestEpiphany, hex_travel::HexTravelState};
use crate::{human_presence::SoftPresence, title_screen::LaunchDoor};

/// North of the Threshold shelf (0,6): clear of the node, lamp, pond, Wards, both enemies and the wake point.
pub const DROID_HOME: [f32; 2] = [0.0, 13.0];
pub const DROID_REACH: f32 = REACH; // R1
/// Shot cycle: 0.75 s quiet, then a 0.5 s tell, then the shot. Playtest-tunable. Sherif may veto.
pub const DROID_SHOT_SECS: f32 = 1.25;
/// COMBAT_AND_PVP.md:479 "Shot, 0.5 s"; meets the 0.35 s telegraph rule (:433).
pub const DROID_SHOT_TELL_SECS: f32 = 0.5;
pub const DROID_HIT: u32 = 25;
pub const DROID_NAME: &str = "[DROID NAME TBD]"; // unnamed until NAME-PASS-1
const DROID_CUE_SECS: f64 = 4.2; // same hold as the private `CUE_SECS` at combat_core.rs:44

/// The droid's pool and clocks. Client-local, never saved.
#[derive(Resource, Debug, Default, Clone, PartialEq)]
pub struct DroidState {
    pub taken: u32,
    pub shot_timer: f32,
    pub down_secs: f32,
}

#[derive(Component)]
pub struct Droid;

#[derive(Component)] // one ground dot of the droid's shot telegraph
pub struct DroidTell;

/// The one reach formula for shot, tell and strike: Heartwood and xz ≤ `DROID_REACH` of `DROID_HOME`, inclusive.
/// Alive is checked by the callers, so a press on a down droid still reads "Missed".
pub fn droid_in_reach(place: PlaceId, _state: &DroidState, x: f32, z: f32) -> bool {
    predator_present(place) && Vec2::new(x, z).distance(Vec2::from_array(DROID_HOME)) <= DROID_REACH
}

pub fn droid_alive(state: &DroidState) -> bool { state.taken < HEALTH }

/// The tell is up for the last `DROID_SHOT_TELL_SECS` of each cycle, only while armed.
pub fn shot_tell_visible(shot_timer: f32, armed: bool) -> bool {
    armed && shot_timer >= DROID_SHOT_SECS - DROID_SHOT_TELL_SECS
}

/// Armed: gate on, player in reach, droid alive, player not yet defeated.
pub fn shot_armed(state: &DroidState, player_taken: u32, gate: bool, reach: bool) -> bool {
    gate && reach && droid_alive(state) && player_taken < HEALTH
}

/// One frame of the shot clock. Returns whether armed after the step (for the tell).
/// Not armed resets the timer to 0. Defeat is left to `predator_lunge`.
pub fn shot_step(state: &mut DroidState, player_taken: &mut u32, dt: f32, gate: bool, reach: bool) -> bool {
    let true = shot_armed(state, *player_taken, gate, reach) else { state.shot_timer = 0.0; return false };
    state.shot_timer += dt.max(0.0);
    if state.shot_timer >= DROID_SHOT_SECS {
        state.shot_timer -= DROID_SHOT_SECS;
        *player_taken = player_taken.saturating_add(DROID_HIT).min(HEALTH);
    }
    shot_armed(state, *player_taken, gate, reach)
}

/// One frame of the strike rise. The baseline always refreshes; a line comes back only on a
/// rise with the gate on and the player in reach. Never touches `SkillXp` or `predator_taken`.
pub fn strike_step(state: &mut DroidState, last: &mut u32, strikes: u32, gate: bool, reach: bool) -> Option<String> {
    let rise = std::mem::replace(last, strikes) != strikes;
    let true = (rise && gate && reach) else { return None };
    let true = droid_alive(state) else { return Some(format!("Missed {DROID_NAME}")) };
    state.taken = state.taken.saturating_add(HIT).min(HEALTH);
    let left = HEALTH - state.taken;
    Some(if left > 0 { format!("{DROID_NAME} {left} / {HEALTH}") } else { format!("{DROID_NAME} is down") })
}

/// One frame of the down clock via `respawn_step`; a respawn restores the full pool.
pub fn down_step(state: &mut DroidState, dt: f32) {
    let (secs, fire) = respawn_step(state.down_secs, dt, !droid_alive(state));
    state.down_secs = secs;
    if fire { (state.taken, state.shot_timer) = (0, 0.0) }
}

/// Dot i sits at angle TAU*i/N, radius `DROID_REACH`, around `home`, at the ground lift.
pub fn droid_tell_ring(home: [f32; 2]) -> [Vec3; LUNGE_TELL_DOTS] {
    std::array::from_fn(|i| {
        let a = std::f32::consts::TAU * i as f32 / LUNGE_TELL_DOTS as f32;
        Vec3::new(home[0] + DROID_REACH * a.cos(), LUNGE_TELL_LIFT, home[1] + DROID_REACH * a.sin())
    })
}

pub struct DroidPlugin;

impl Plugin for DroidPlugin {
    fn build(&self, app: &mut App) {
        let app = app.init_resource::<DroidState>().add_systems(Startup, spawn_droid);
        app.add_systems(Update, droid_tick.after(combat_fire));
    }
}

fn spawn_droid(mut cmd: Commands, mut meshes: ResMut<Assets<Mesh>>, mut mats: ResMut<Assets<StandardMaterial>>) {
    cmd.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.7, 0.9, 0.7))),
        MeshMaterial3d(mats.add(Color::srgb(0.42, 0.46, 0.52))),
        Transform::from_xyz(DROID_HOME[0], 0.8, DROID_HOME[1]),
        (Visibility::Hidden, Droid, Name::new(DROID_NAME)),
    ));
    let mesh = meshes.add(Cylinder::new(LUNGE_TELL_DOT_RADIUS, LUNGE_TELL_DOT_HEIGHT));
    let mat = mats.add(StandardMaterial { base_color: Color::srgb(0.86, 0.85, 0.82), unlit: true, ..default() });
    for at in droid_tell_ring(DROID_HOME) {
        cmd.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(mat.clone()), Transform::from_translation(at)))
            .insert((Visibility::Hidden, DroidTell));
    }
}

fn show(on: bool) -> Visibility { if on { Visibility::Visible } else { Visibility::Hidden } }

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn droid_tick(
    time: Res<Time>,
    mut last: Local<u32>,
    (door, travel, presence): (Option<Res<LaunchDoor>>, Option<Res<HexTravelState>>, Option<Res<SoftPresence>>),
    mut combat: ResMut<CombatState>,
    mut state: ResMut<DroidState>,
    mut epi: Option<ResMut<FirstHarvestEpiphany>>,
    mut body: Query<&mut Visibility, (With<Droid>, Without<DroidTell>)>,
    mut dots: Query<&mut Visibility, (With<DroidTell>, Without<Droid>)>,
) {
    let place = travel.as_deref().map(|t| t.current);
    let gate = place.is_some_and(predator_present) && door.as_deref().copied() == Some(LaunchDoor::InYard);
    let at = presence.as_deref().map(|p| p.position);
    let reach = gate && place.zip(at).is_some_and(|(pl, p)| droid_in_reach(pl, &state, p.x, p.z));
    let line = strike_step(&mut state, &mut last, combat.strikes, gate, reach);
    if let (Some(line), Some(epi)) = (line, epi.as_mut()) {
        set_pulse(epi, line, time.elapsed_secs_f64() + DROID_CUE_SECS);
    }
    let dt = time.delta_secs();
    down_step(&mut state, dt);
    let armed = shot_step(&mut state, &mut combat.player_taken, dt, gate, reach);
    let alive = gate && droid_alive(&state);
    body.iter_mut().for_each(|mut v| _ = v.set_if_neq(show(alive)));
    let tell = alive && shot_tell_visible(state.shot_timer, armed);
    dots.iter_mut().for_each(|mut v| _ = v.set_if_neq(show(tell)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat_core::{HEARTWOOD_ENTRANCE, PREDATOR_HOME, PREDATOR_RESPAWN_SECS};
    use crate::grazer::{FLEE_RADIUS, GRAZER_HOME};
    use crate::{skill_xp::SkillXp, terrain::PLAY_HALF};
    use shared::heartwood_lamp::{LAMP_DISK_CENTER, LAMP_DISK_RADIUS, WATER_POND_CENTER, WATER_POND_RADIUS};
    use shared::heartwood_wards::{WARD_POST_CENTERS, WARD_USE_RADIUS};
    use shared::threshold_shelf::{THRESHOLD_NODE_CENTER, THRESHOLD_SHELF_USE_RADIUS};

    fn run_for(s: &mut DroidState, t: &mut u32, secs: f32, reach: bool) -> bool { shot_step(s, t, secs, true, reach) }

    #[test]
    fn tell_shows_from_three_quarters() {
        let (mut s, mut t) = (DroidState::default(), 0);
        let armed = run_for(&mut s, &mut t, 0.75 - 1e-3, true);
        assert!(armed && !shot_tell_visible(s.shot_timer, armed));
        let armed = run_for(&mut s, &mut t, 2e-3, true);
        assert!(shot_tell_visible(s.shot_timer, armed) && t == 0);
    }

    #[test]
    fn shot_lands_at_period() {
        let (mut s, mut t) = (DroidState::default(), 0);
        assert!(run_for(&mut s, &mut t, 1.25 - 1e-3, true) && t == 0);
        let armed = run_for(&mut s, &mut t, 2e-3, true);
        assert_eq!(t, DROID_HIT);
        assert!(s.shot_timer < 1e-2 && !shot_tell_visible(s.shot_timer, armed));
    }

    #[test]
    fn reach_is_inclusive_at_edge() {
        let s = DroidState::default();
        let [hx, hz] = DROID_HOME;
        assert!(droid_in_reach(PlaceId::Heartwood, &s, hx + DROID_REACH, hz));
        assert!(!droid_in_reach(PlaceId::Heartwood, &s, hx + DROID_REACH + 1e-3, hz));
        let (mut st, mut t) = (DroidState::default(), 0);
        let edge = droid_in_reach(PlaceId::Heartwood, &s, hx - DROID_REACH, hz);
        run_for(&mut st, &mut t, DROID_SHOT_SECS, edge);
        let out = droid_in_reach(PlaceId::Heartwood, &s, hx - DROID_REACH - 1e-3, hz);
        run_for(&mut st, &mut t, DROID_SHOT_SECS, out);
        assert_eq!(t, DROID_HIT, "hit at the edge, none just outside");
    }

    #[test]
    fn timer_resets_on_leaving_reach() {
        let (mut s, mut t) = (DroidState::default(), 0);
        run_for(&mut s, &mut t, 1.0, true);
        run_for(&mut s, &mut t, 0.016, false);
        assert_eq!(s.shot_timer, 0.0);
        assert!(run_for(&mut s, &mut t, 1.0, true) && t == 0);
        assert!(!shot_step(&mut s, &mut t, 0.1, false, true) && s.shot_timer == 0.0, "gate off resets");
    }

    #[test]
    fn no_shot_while_down_or_defeated() {
        let (mut s, mut t) = (DroidState { taken: HEALTH, ..default() }, 0);
        assert!(!run_for(&mut s, &mut t, 5.0, true) && t == 0 && s.shot_timer == 0.0);
        let (mut s, mut t) = (DroidState::default(), HEALTH);
        assert!(!run_for(&mut s, &mut t, 5.0, true) && s.shot_timer == 0.0);
    }

    #[test]
    fn respawns_after_predator_delay() {
        let mut s = DroidState { taken: HEALTH, ..default() };
        down_step(&mut s, PREDATOR_RESPAWN_SECS - 0.1);
        assert!(!droid_alive(&s) && !run_for(&mut s.clone(), &mut 0, 1.0, true));
        down_step(&mut s, 0.1);
        assert!(droid_alive(&s) && s.taken == 0 && s.down_secs == 0.0);
    }

    #[test]
    fn strike_rise_lands_hit_with_zero_xp() {
        let (combat, skills) = (CombatState::default(), SkillXp::default());
        let (mut s, mut last) = (DroidState::default(), 0);
        let line = strike_step(&mut s, &mut last, 1, true, true);
        assert_eq!((s.taken, line), (HIT, Some(format!("{DROID_NAME} {} / {HEALTH}", HEALTH - HIT))));
        assert_eq!((skills.xp, skills.combat_day_hits, combat.predator_taken), (SkillXp::default().xp, 0, 0));
        assert_eq!(strike_step(&mut s, &mut last, 1, true, true), None, "no rise, no line");
        let mut k = DroidState { taken: HEALTH - HIT, ..default() };
        assert_eq!(strike_step(&mut k, &mut last, 2, true, true), Some(format!("{DROID_NAME} is down")));
        assert_eq!(strike_step(&mut k, &mut last, 3, true, true), Some(format!("Missed {DROID_NAME}")));
    }

    #[test]
    fn stale_rise_never_lands_later() {
        let (mut s, mut last) = (DroidState::default(), 0);
        assert_eq!(strike_step(&mut s, &mut last, 1, true, false), None);
        assert_eq!(strike_step(&mut s, &mut last, 1, false, true), None);
        assert_eq!(strike_step(&mut s, &mut last, 1, true, true), None);
        assert_eq!((s.taken, last), (0, 1));
        assert!(strike_step(&mut s, &mut last, 1u32.wrapping_add(1), true, true).is_some());
    }

    #[test]
    fn home_clear_of_teaching_ground_enemies_and_wake() {
        let home = Vec2::from_array(DROID_HOME);
        let clear = |c: Vec2, r: f32| home.distance(c) > r + DROID_REACH;
        assert!(clear(Vec2::from_array(LAMP_DISK_CENTER), LAMP_DISK_RADIUS));
        assert!(clear(Vec2::from_array(WATER_POND_CENTER), WATER_POND_RADIUS));
        assert!(WARD_POST_CENTERS.iter().all(|w| clear(Vec2::new(w[0], w[2]), WARD_USE_RADIUS)));
        let node = Vec2::new(THRESHOLD_NODE_CENTER[0], THRESHOLD_NODE_CENTER[2]);
        assert!(clear(node, THRESHOLD_SHELF_USE_RADIUS));
        assert!(home.x.abs() + DROID_REACH < PLAY_HALF && home.y.abs() + DROID_REACH < PLAY_HALF);
        assert!(clear(Vec2::from_array(PREDATOR_HOME), REACH));
        assert!(home.distance(Vec2::from_array(GRAZER_HOME)) > REACH + FLEE_RADIUS);
        assert!(clear(Vec2::from_array(HEARTWOOD_ENTRANCE), THRESHOLD_SHELF_USE_RADIUS), "wake point");
        assert!(droid_tell_ring(DROID_HOME).iter().all(|d| (d.xz().distance(home) - DROID_REACH).abs() < 1e-4));
    }
}
