//! COMBAT-CORE-1 — offline PvE: one predator in the Heartwood Teeth band.
//! Health is client-local. No PvP, no kill XP, no loot. Online stays grey.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use shared::heartwood_lamp::{heartwood_bath_return, HEARTWOOD_BATH_RETURN, WATER_POND_CENTER};
use shared::hex_travel::PlaceId;

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::first_session_guidance::FirstSessionGuidance;
use crate::hex_travel::{places_plate_open, HexTravelState, PlacesPlate};
use crate::hud_anchor_registry::action_bar_prompts_showing;
use crate::hud_edit_mode::HudEditMode;
use crate::human_presence::SoftPresence;
use crate::lived_hour_bind::LivedHourBind;
use crate::mercy_harvest_nodes::{CareCycleOffer, NearbyMercyNode, HARVEST_REACH};
use crate::skill_xp::{SkillXp, COMBAT};
use crate::soft_play_bindings::{FIRE, FIRE_PAD};
use crate::title_screen::{HouseLabel, LaunchDoor};

pub const PREDATOR_NAME: &str = "[NAME TBD]";
pub const HEALTH: u32 = 200; // player and predator pools, per CUT A1
/// COMBAT_AND_PVP.md:474 — Predator "Lunge, 0.6 s" hits 15. Fire lands the same 15.
pub const HIT: u32 = 15;
pub const LUNGE_SECS: f32 = 0.6;
pub const REACH: f32 = HARVEST_REACH; // both sides reuse the harvest reach
pub const XP_PER_LANDED_HIT: u32 = 10; // tuning value, set in playtest
/// Teeth band: past the lamp, pond, Wards and threshold node, inside PLAY_HALF.
pub const PREDATOR_HOME: [f32; 2] = [-18.0, -18.0];
/// Defeat wakes here: the dry-Lip return of heartwood_lip.rs:209, rebuilt locally.
pub const HEARTWOOD_ENTRANCE: [f32; 2] = HEARTWOOD_BATH_RETURN;
const STAND_HEIGHT: f32 = 0.90; // copy of private heartwood_lip.rs:30
/// FEEL-PASS-1 — how long a combat cue holds the pulse strip (skill_xp level-up uses 4.2 s).
const CUE_SECS: f64 = 4.2;

/// Damage taken so far; health is `HEALTH - taken`. Client-local.
#[derive(Resource, Debug, Default)]
pub struct CombatState {
    pub player_taken: u32,
    pub predator_taken: u32,
    pub lunge_timer: f32,
}

#[derive(Component)]
pub struct Predator;

/// `mouse_look_grab_eligible` terms (human_presence.rs:1478) without `mouse_look`.
pub fn fire_grab_terms(in_yard: bool, settings_open: bool, hud_edit: bool, focused: bool) -> bool {
    in_yard && !settings_open && !hud_edit && focused
}

/// `prompts` = CareStrip or CarePrompt showing (COMBAT_AND_PVP.md:522).
pub fn fire_allowed(grab: bool, plate_open: bool, ui_busy: bool, prompts: bool) -> bool {
    grab && !plate_open && !ui_busy && !prompts
}

/// The predator lives only in Heartwood. Never Sanctuary, never Depths.
pub fn predator_present(place: PlaceId) -> bool {
    place == PlaceId::Heartwood
}

pub fn in_reach(place: PlaceId, combat: &CombatState, x: f32, z: f32) -> bool {
    let d = Vec2::new(x, z).distance(Vec2::from_array(PREDATOR_HOME));
    predator_present(place) && combat.predator_taken < HEALTH && d <= REACH
}

/// A landed hit pays XP. The kill itself pays nothing extra.
pub fn land_hit(combat: &mut CombatState, skills: &mut SkillXp, reach: bool) -> bool {
    if !reach || combat.predator_taken >= HEALTH {
        return false;
    }
    combat.predator_taken = combat.predator_taken.saturating_add(HIT).min(HEALTH);
    skills.gain(COMBAT, XP_PER_LANDED_HIT);
    true
}

/// One lunge per 0.6 s in reach. True when the player is defeated.
pub fn predator_tick(combat: &mut CombatState, dt: f32, reach: bool) -> bool {
    combat.lunge_timer = if reach { combat.lunge_timer + dt.max(0.0) } else { 0.0 };
    while combat.lunge_timer >= LUNGE_SECS {
        combat.lunge_timer -= LUNGE_SECS;
        combat.player_taken = combat.player_taken.saturating_add(HIT).min(HEALTH);
    }
    combat.player_taken >= HEALTH
}

/// Wake at the entrance at full health. Only presence and health move.
pub fn reseat_on_defeat(combat: &mut CombatState, presence: &mut SoftPresence) {
    let [px, pz] = WATER_POND_CENTER;
    let [x, z] = heartwood_bath_return(PlaceId::Heartwood, px, pz).unwrap_or(HEARTWOOD_ENTRANCE);
    (presence.position, presence.velocity) = (Vec3::new(x, STAND_HEIGHT, z), Vec3::ZERO);
    presence.grounded = true;
    (combat.player_taken, combat.lunge_timer) = (0, 0.0);
}

/// FEEL-PASS-1 — the line for one FIRE that passed `fire_allowed`. `gained` is the real XP change.
pub fn fire_cue(place: PlaceId, combat: &CombatState, hit: bool, gained: u32) -> Option<String> {
    let remaining = HEALTH.saturating_sub(combat.predator_taken);
    if hit && remaining == 0 {
        return Some(format!("{PREDATOR_NAME} is down"));
    }
    if hit {
        let xp = if gained > 0 { format!(" · +{gained} combat XP") } else { String::new() };
        return Some(format!("{PREDATOR_NAME} {remaining} / {HEALTH}{xp}"));
    }
    (predator_present(place) && remaining > 0).then(|| format!("Missed {PREDATOR_NAME}"))
}

pub fn wake_cue() -> String {
    "You wake at the Heartwood entrance".to_string()
}

/// Compare before write: an unchanged cue leaves the resource untouched.
pub fn set_pulse(epi: &mut impl std::ops::DerefMut<Target = FirstHarvestEpiphany>, line: String, until: f64) {
    if epi.pulse_line != line || epi.pulse_until != until {
        (epi.pulse_line, epi.pulse_until) = (line, until);
    }
}

pub struct CombatCorePlugin;

impl Plugin for CombatCorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CombatState>()
            .add_systems(Startup, spawn_predator)
            .add_systems(Update, (combat_fire, predator_lunge).chain());
    }
}

fn spawn_predator(
    mut commands: Commands,
    (mut meshes, mut materials): (ResMut<Assets<Mesh>>, ResMut<Assets<StandardMaterial>>),
) {
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.35, 0.9))),
        MeshMaterial3d(materials.add(Color::srgb(0.36, 0.22, 0.16))),
        Transform::from_xyz(PREDATOR_HOME[0], 0.8, PREDATOR_HOME[1]),
        Visibility::Hidden,
        Predator,
        Name::new(PREDATOR_NAME),
    ));
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn combat_fire(
    time: Res<Time>,
    buttons: Option<Res<ButtonInput<MouseButton>>>,
    gamepads: Query<&Gamepad>,
    (door, house): (Option<Res<LaunchDoor>>, Option<Res<HouseLabel>>),
    (edit, plate): (Option<Res<HudEditMode>>, Option<Res<PlacesPlate>>),
    windows: Query<&Window, With<PrimaryWindow>>,
    ui: Query<&Interaction>,
    (care, mut epi): (Option<Res<CareCycleOffer>>, Option<ResMut<FirstHarvestEpiphany>>),
    (nearby, guidance): (Option<Res<NearbyMercyNode>>, Option<Res<FirstSessionGuidance>>),
    bind: Option<Res<LivedHourBind>>,
    (travel, presence): (Option<Res<HexTravelState>>, Option<Res<SoftPresence>>),
    mut combat: ResMut<CombatState>,
    skills: Option<ResMut<SkillXp>>,
) {
    let pressed = buttons.is_some_and(|b| b.just_pressed(FIRE))
        || gamepads.iter().any(|g| g.just_pressed(FIRE_PAD));
    let (true, Some(travel), Some(presence), Some(mut skills)) = (pressed, travel, presence, skills) else {
        return;
    };
    let grab = fire_grab_terms(
        door.as_deref().copied() == Some(LaunchDoor::InYard),
        house.as_deref().is_some_and(|label| label.settings_open),
        edit.as_deref().is_some_and(|mode| mode.active),
        windows.iter().any(|w| w.focused),
    );
    let (now, hidden) = (time.elapsed_secs_f64(), bind.is_some_and(|b| b.guidance_hidden));
    let prompts = guidance.as_deref().is_some_and(|g| {
        let (care, epi, nearby) = (care.as_deref(), epi.as_deref(), nearby.as_deref());
        let (strip, prompt) = action_bar_prompts_showing(care, epi, nearby, g, now, hidden);
        strip || prompt
    });
    let plate_open = plate.as_deref().is_some_and(places_plate_open);
    if fire_allowed(grab, plate_open, ui.iter().any(|i| *i != Interaction::None), prompts) {
        let reach = in_reach(travel.current, &combat, presence.position.x, presence.position.z);
        let before = skills.xp[COMBAT];
        let hit = land_hit(&mut combat, &mut skills, reach);
        let gained = skills.xp[COMBAT].saturating_sub(before);
        if let (Some(line), Some(epi)) = (fire_cue(travel.current, &combat, hit, gained), epi.as_mut()) {
            set_pulse(epi, line, now + CUE_SECS);
        }
    }
}

fn predator_lunge(
    time: Res<Time>,
    door: Option<Res<LaunchDoor>>,
    travel: Option<Res<HexTravelState>>,
    presence: Option<ResMut<SoftPresence>>,
    mut combat: ResMut<CombatState>,
    mut epi: Option<ResMut<FirstHarvestEpiphany>>,
    mut shown: Query<&mut Visibility, With<Predator>>,
) {
    let (Some(travel), Some(mut presence)) = (travel, presence) else {
        return;
    };
    let show = combat.predator_taken < HEALTH && predator_present(travel.current);
    for mut vis in &mut shown {
        vis.set_if_neq(if show { Visibility::Visible } else { Visibility::Hidden });
    }
    let (in_yard, p) = (door.as_deref().copied() == Some(LaunchDoor::InYard), presence.position);
    let reach = in_yard && in_reach(travel.current, &combat, p.x, p.z);
    if predator_tick(&mut combat, time.delta_secs(), reach) {
        reseat_on_defeat(&mut combat, &mut presence);
        if let Some(epi) = epi.as_mut() {
            set_pulse(epi, wake_cue(), time.elapsed_secs_f64() + CUE_SECS);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::human_inventory::HumanInventory;
    use crate::terrain::PLAY_HALF;
    use shared::heartwood_lamp::{LAMP_DISK_CENTER, LAMP_DISK_RADIUS, WATER_POND_RADIUS};
    use shared::heartwood_wards::{WARD_POST_CENTERS, WARD_USE_RADIUS};
    use shared::threshold_shelf::{THRESHOLD_NODE_CENTER, THRESHOLD_SHELF_USE_RADIUS};

    #[test]
    fn fire_inert_under_each_guard() {
        assert!(fire_allowed(fire_grab_terms(true, false, false, true), false, false, false));
        assert!(!fire_grab_terms(false, false, false, true), "not in yard");
        assert!(!fire_grab_terms(true, true, false, true), "settings open");
        assert!(!fire_grab_terms(true, false, true, true), "hud edit");
        assert!(!fire_grab_terms(true, false, false, false), "unfocused");
        assert!(!fire_allowed(true, true, false, false), "places plate");
        assert!(!fire_allowed(true, false, true, false), "ui hover or press");
        assert!(!fire_allowed(true, false, false, true), "care prompts own ACTION_BAR");
    }

    #[test]
    fn hit_grants_xp_kill_does_not() {
        let (mut c, mut s) = (CombatState::default(), SkillXp::default());
        assert!(!land_hit(&mut c, &mut s, false) && s.xp[COMBAT] == 0, "a miss pays nothing");
        assert!(land_hit(&mut c, &mut s, true) && s.xp[COMBAT] == XP_PER_LANDED_HIT);
        c.predator_taken = HEALTH - HIT;
        assert!(land_hit(&mut c, &mut s, true) && c.predator_taken >= HEALTH);
        assert_eq!(s.xp[COMBAT], 2 * XP_PER_LANDED_HIT, "the kill adds no bonus");
        assert!(!land_hit(&mut c, &mut s, true) && s.xp[COMBAT] == 2 * XP_PER_LANDED_HIT);
    }

    #[test]
    fn predator_never_in_sanctuary_or_depths_and_clear_of_teaching_ground() {
        let (c, [x, z]) = (CombatState::default(), PREDATOR_HOME);
        assert!(in_reach(PlaceId::Heartwood, &c, x, z));
        for place in [PlaceId::Sanctuary, PlaceId::Depths] {
            assert!(!predator_present(place) && !in_reach(place, &c, x, z));
        }
        let home = Vec2::new(x, z);
        let clear = |c: Vec2, r: f32| home.distance(c) > r + REACH;
        assert!(clear(Vec2::from_array(LAMP_DISK_CENTER), LAMP_DISK_RADIUS));
        assert!(clear(Vec2::from_array(WATER_POND_CENTER), WATER_POND_RADIUS));
        assert!(WARD_POST_CENTERS.iter().all(|w| clear(Vec2::new(w[0], w[2]), WARD_USE_RADIUS)));
        let node = Vec2::new(THRESHOLD_NODE_CENTER[0], THRESHOLD_NODE_CENTER[2]);
        assert!(clear(node, THRESHOLD_SHELF_USE_RADIUS));
        assert!(x.abs() + REACH < PLAY_HALF && z.abs() + REACH < PLAY_HALF);
    }

    #[test]
    fn defeat_reseats_with_inventory_unchanged() {
        let inv = HumanInventory::default();
        let (slot, seen) = (inv.selected, inv.last_seen_harvests);
        let mut c = CombatState::default();
        assert!(!predator_tick(&mut c, LUNGE_SECS, true) && c.player_taken == HIT);
        assert!(predator_tick(&mut c, LUNGE_SECS * 14.0, true), "200 health, 15 per lunge");
        let (position, velocity) = (Vec3::new(-18.0, STAND_HEIGHT, -18.0), Vec3::X);
        let (prev_tick, curr_tick) = (None, None);
        let mut p = SoftPresence { position, velocity, grounded: false, prev_tick, curr_tick };
        reseat_on_defeat(&mut c, &mut p);
        assert_eq!((p.position.x, p.position.z), (HEARTWOOD_ENTRANCE[0], HEARTWOOD_ENTRANCE[1]));
        assert!(c.player_taken == 0 && p.grounded && p.velocity == Vec3::ZERO);
        assert!(inv.selected == slot && inv.last_seen_harvests == seen);
    }

    #[test]
    fn hit_prints_remaining_and_real_xp() {
        let (mut c, mut s) = (CombatState::default(), SkillXp::default());
        assert!(land_hit(&mut c, &mut s, true));
        let line = fire_cue(PlaceId::Heartwood, &c, true, s.xp[COMBAT]).unwrap();
        let want = format!("{PREDATOR_NAME} {} / {HEALTH} · +{XP_PER_LANDED_HIT} combat XP", HEALTH - HIT);
        assert_eq!(line, want);
        c.predator_taken = HEALTH;
        assert_eq!(fire_cue(PlaceId::Heartwood, &c, true, 0).unwrap(), format!("{PREDATOR_NAME} is down"));
    }

    #[test]
    fn at_xp_cap_no_unpaid_xp_printed() {
        let (mut c, mut s) = (CombatState::default(), SkillXp::default());
        s.xp[COMBAT] = crate::skill_xp::XP_CAP;
        let before = s.xp[COMBAT];
        assert!(land_hit(&mut c, &mut s, true));
        let line = fire_cue(PlaceId::Heartwood, &c, true, s.xp[COMBAT] - before).unwrap();
        assert_eq!(line, format!("{PREDATOR_NAME} {} / {HEALTH}", HEALTH - HIT));
        assert!(!line.contains("XP"));
    }

    #[test]
    fn miss_only_in_heartwood_with_predator_up() {
        let mut c = CombatState::default();
        assert_eq!(fire_cue(PlaceId::Heartwood, &c, false, 0), Some(format!("Missed {PREDATOR_NAME}")));
        assert_eq!(fire_cue(PlaceId::Sanctuary, &c, false, 0), None);
        c.predator_taken = HEALTH;
        assert_eq!(fire_cue(PlaceId::Heartwood, &c, false, 0), None, "predator down");
    }

    #[test]
    fn defeat_writes_wake_line() {
        use bevy::ecs::system::RunSystemOnce;
        let mut world = World::new();
        let mut time = Time::<()>::default();
        time.advance_by(std::time::Duration::from_secs_f32(LUNGE_SECS));
        world.insert_resource(time);
        let mut travel = HexTravelState::default();
        travel.current = PlaceId::Heartwood;
        world.insert_resource(travel);
        world.insert_resource(LaunchDoor::InYard);
        let [x, z] = PREDATOR_HOME;
        world.insert_resource(SoftPresence { position: Vec3::new(x, STAND_HEIGHT, z), ..Default::default() });
        world.insert_resource(CombatState { player_taken: HEALTH - HIT, ..Default::default() });
        world.insert_resource(FirstHarvestEpiphany::default());
        world.run_system_once(predator_lunge).unwrap();
        assert_eq!(world.resource::<FirstHarvestEpiphany>().pulse_line, wake_cue());
        assert_eq!(world.resource::<CombatState>().player_taken, 0);
    }

    #[test]
    fn no_write_on_a_frame_without_an_event() {
        use bevy::ecs::system::RunSystemOnce;
        let mut world = World::new();
        world.insert_resource(Time::<()>::default());
        world.insert_resource(HexTravelState::default());
        world.insert_resource(SoftPresence::default());
        world.insert_resource(CombatState::default());
        world.insert_resource(SkillXp::default());
        world.insert_resource(FirstHarvestEpiphany::default());
        world.clear_trackers();
        world.run_system_once(combat_fire).unwrap();
        world.run_system_once(predator_lunge).unwrap();
        assert!(!world.is_resource_changed::<FirstHarvestEpiphany>());
    }

    #[test]
    fn combat_lines_never_read_as_tend_breathe() {
        let (up, down) = (CombatState::default(), CombatState { predator_taken: HEALTH, ..Default::default() });
        let hits = [(&up, true, XP_PER_LANDED_HIT), (&up, true, 0), (&up, false, 0), (&down, true, 0)];
        let cues = hits.map(|(c, hit, xp)| fire_cue(PlaceId::Heartwood, c, hit, xp).unwrap());
        for line in cues.into_iter().chain([wake_cue()]) {
            assert!(!crate::mercy_harvest_nodes::pulse_line_is_tend_breathe(&line), "{line}");
        }
    }
}
