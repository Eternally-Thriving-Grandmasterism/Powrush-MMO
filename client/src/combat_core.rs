//! COMBAT-CORE-1 — offline PvE: one predator in the Heartwood Teeth band.
//! Health is client-local. No PvP, no kill XP, no loot. Online stays grey.

use bevy::input::gamepad::GamepadRumbleRequest;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use shared::heartwood_lamp::{heartwood_bath_return, HEARTWOOD_BATH_RETURN, WATER_POND_CENTER};
use shared::hex_travel::PlaceId;

use crate::coop_voice::VoiceYard;
use crate::feel_move::{buffer_alive, USE_BUFFER_SECS};
use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::first_session_guidance::FirstSessionGuidance;
use crate::harvest_feel::{rumble_harvest, SoftRbePool};
use crate::hex_travel::{places_plate_open, HexTravelState, PlacesPlate};
use crate::hud_anchor_registry::action_bar_prompts_showing;
use crate::hud_edit_mode::HudEditMode;
use crate::human_inventory::HumanInventory;
use crate::human_presence::SoftPresence;
use crate::human_soft_panels::HumanSoftPanels;
use crate::input::{InputMapSet, PlayerInput};
use crate::ledger_bind::LedgerYard;
use crate::lived_hour_bind::LivedHourBind;
use crate::mercy_harvest_nodes::{CareCycleOffer, NearbyMercyNode, HARVEST_REACH};
use crate::rbe_allocate_choice::RbeAllocateChoice;
use crate::skill_xp::{level_up_line, SkillXp, COMBAT};
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
/// PREDATOR-RESPAWN-1 — council-set, playtest-tunable delay after a kill. Sherif may veto.
pub const PREDATOR_RESPAWN_SECS: f32 = 30.0;

/// Damage taken so far; health is `HEALTH - taken`. Client-local.
#[derive(Resource, Debug, Default)]
pub struct CombatState {
    pub player_taken: u32,
    pub predator_taken: u32,
    pub lunge_timer: f32,
    /// Set by `predator_respawn`; pays like a first life now that the COMBAT-XP-PACE-1 daily cap governs XP.
    pub respawned: bool,
    /// ENEMY-DROID-1 — allowed strikes run so far (wrapping); droid.rs reads a rise.
    pub strikes: u32,
    /// Seconds the predator has been down; reset to 0 while alive. Only writer: `predator_respawn`.
    pub down_secs: f32,
}

#[derive(Component)]
pub struct Predator;

/// COMBAT_AND_PVP.md:433: every damaging ability telegraphs for at least 0.35 s.
pub const LUNGE_TELL_SECS: f32 = 0.35;
/// LUNGE-TELL-1 — ground ring of dots at the lunge reach (stripes or dots, colour is an extra).
pub const LUNGE_TELL_DOTS: usize = 20;
/// Radius and height of one ground disc.
pub const LUNGE_TELL_DOT_RADIUS: f32 = 0.12;
pub const LUNGE_TELL_DOT_HEIGHT: f32 = 0.02;
/// Ground y: the capsule foot is y 0 (Capsule3d::new(0.35, 0.9) at y 0.8, :215/:217),
/// plus the heartwood_lip.rs:199 lift.
pub const LUNGE_TELL_LIFT: f32 = 0.01;

/// LUNGE-TELL-1 — one ground dot of the lunge telegraph.
#[derive(Component)]
pub struct LungeTell;

/// The tell is up for the last `LUNGE_TELL_SECS` before each lunge, only while in reach.
pub fn tell_visible(lunge_timer: f32, reach: bool) -> bool {
    reach && lunge_timer >= LUNGE_SECS - LUNGE_TELL_SECS
}

/// Dot i sits at angle TAU*i/N, radius `REACH`, around `PREDATOR_HOME`, at the ground lift.
pub fn lunge_tell_ring() -> [Vec3; LUNGE_TELL_DOTS] {
    std::array::from_fn(|i| {
        let a = std::f32::consts::TAU * i as f32 / LUNGE_TELL_DOTS as f32;
        let [hx, hz] = PREDATOR_HOME;
        Vec3::new(hx + REACH * a.cos(), LUNGE_TELL_LIFT, hz + REACH * a.sin())
    })
}

/// RESPAWN-TELL-1 — the last stretch of the down clock, when the creature rises from the Place.
pub const PREDATOR_TELL_SECS: f32 = 1.5;
/// A ground pillar at `PREDATOR_HOME`: full height when the predator returns.
pub const RISE_TELL_RADIUS: f32 = 0.45;
pub const RISE_TELL_HEIGHT: f32 = 1.7;

/// The rising pillar at `PREDATOR_HOME`.
#[derive(Component)]
pub struct RiseTell;

/// Rise fraction (0..=1) while the tell shows: Heartwood only, predator down, last `PREDATOR_TELL_SECS`.
pub fn rise_tell(place: PlaceId, predator_taken: u32, down_secs: f32) -> Option<f32> {
    let onset = PREDATOR_RESPAWN_SECS - PREDATOR_TELL_SECS;
    (predator_present(place) && predator_taken >= HEALTH && down_secs >= onset)
        .then(|| ((down_secs - onset) / PREDATOR_TELL_SECS).clamp(0.0, 1.0))
}

/// The pillar grows up out of the ground: base pinned at y 0, height scales with the rise.
pub fn rise_tell_transform(rise: f32) -> Transform {
    let h = rise.clamp(0.01, 1.0);
    let [x, z] = PREDATOR_HOME;
    Transform::from_xyz(x, RISE_TELL_HEIGHT * h * 0.5, z).with_scale(Vec3::new(1.0, h, 1.0))
}

/// HIT-JUICE-1 — the Predator's base colour (spawn_predator); a flash always restores to it exactly.
pub const PREDATOR_COLOR: Color = Color::srgb(0.36, 0.22, 0.16);
/// HIT-JUICE-1 — neutral warm white the Predator tints toward on a landed hit.
pub const FLASH_COLOR: Color = Color::srgb(1.0, 0.96, 0.9);
/// The flash is over by this many seconds after the hit.
pub const FLASH_SECS: f32 = 0.10;
/// Camera kick on a landed hit and on the kill. Both stay under the contest win's 1.0.
pub const HIT_KICK: f32 = 0.35;
pub const KILL_KICK: f32 = 0.6;

/// 1.0 at the hit, easing out to 0.0 at `FLASH_SECS`; never rises with `t`.
pub fn flash_strength(t: f32) -> f32 {
    let left = (1.0 - t.max(0.0) / FLASH_SECS).max(0.0);
    left * left
}

/// The kill kicks harder than a plain hit.
pub fn hit_kick(kill: bool) -> f32 {
    [HIT_KICK, KILL_KICK][usize::from(kill)]
}

/// The Predator colour at flash `strength`; 0 is `PREDATOR_COLOR` exactly.
pub fn flash_color(strength: f32) -> Color {
    if strength <= 0.0 {
        PREDATOR_COLOR
    } else {
        PREDATOR_COLOR.mix(&FLASH_COLOR, strength.min(1.0))
    }
}

/// COMBAT-FEEL-1 — one held strike, the feel_move.rs:38-101 shape. Times are `Time<Fixed>` elapsed.
#[derive(Resource, Debug, Default, Clone, PartialEq)]
pub struct StrikeBuffer {
    /// Fixed-clock seconds the held strike stays live until (inclusive, `buffer_alive`).
    pub until: Option<f64>,
    /// Sticky: set on a fixed tick when the held strike comes into reach; consumed once in Update.
    pub fire: bool,
}

/// `mouse_look_grab_eligible` terms (human_presence.rs:1478) without `mouse_look`.
pub fn fire_grab_terms(in_yard: bool, settings_open: bool, hud_edit: bool, focused: bool) -> bool {
    in_yard && !settings_open && !hud_edit && focused
}

/// `prompts` = CareStrip or CarePrompt showing (COMBAT_AND_PVP.md:522).
pub fn fire_allowed(grab: bool, plate_open: bool, ui_busy: bool, prompts: bool) -> bool {
    grab && !plate_open && !ui_busy && !prompts
}

/// SKILL-KEYS-1 — the digit path only: true when no digit-reading panel is open (INPUT_CANON K1).
pub fn skill_gate_allows(allocate: bool, voice: bool, ledger: bool, satchel: bool, realm: bool) -> bool {
    !(allocate || voice || ledger || satchel || realm)
}

/// SKILL-KEYS-1 — only slot 0 (key 1) is the strike. Slots 1-3 have no kit yet and do nothing.
pub fn slot_strikes(slot: Option<u32>) -> bool {
    slot == Some(0)
}

/// One strike decision per frame: LMB/pad as before, or key 1 while no panel is open.
pub fn strike_pressed(fire: bool, slot: Option<u32>, gate: bool) -> bool {
    fire || (slot_strikes(slot) && gate)
}

/// The predator lives only in Heartwood. Never Sanctuary, never Depths.
pub fn predator_present(place: PlaceId) -> bool {
    place == PlaceId::Heartwood
}

pub fn in_reach(place: PlaceId, combat: &CombatState, x: f32, z: f32) -> bool {
    let d = Vec2::new(x, z).distance(Vec2::from_array(PREDATOR_HOME));
    predator_present(place) && combat.predator_taken < HEALTH && d <= REACH
}

/// COMBAT-FEEL-1 — fixed tick: only expires the held strike or, in reach, sets the sticky pulse.
pub fn strike_buffer_tick(buf: &mut StrikeBuffer, now: f64, reach: bool) {
    match buf.until.map(|until| buffer_alive(until, now)) {
        Some(false) => buf.until = None,
        Some(true) if reach => (buf.fire, buf.until) = (true, None),
        _ => {}
    }
}

/// COMBAT-FEEL-1 — one frame: the fixed `ticks` (time, reach) that ran, then the Update step at fixed `now`.
/// True means run the existing hit path once; an edge and a pulse on one frame are one strike.
pub fn strike_buffer_step(
    buf: &mut StrikeBuffer,
    ticks: &[(f64, bool)],
    (press, allowed, gate, reach): (bool, bool, bool, bool),
    now: f64,
) -> bool {
    for &(at, near) in ticks {
        strike_buffer_tick(buf, at, near);
    }
    let pulse = std::mem::take(&mut buf.fire);
    let run = allowed && (press || (pulse && gate && reach));
    if pulse || (press && allowed && reach) {
        buf.until = None; // feel_move.rs:69: an in-reach edge (or a consume) leaves nothing held
    }
    if press && allowed && gate && !reach {
        buf.until = Some(now + USE_BUFFER_SECS);
    }
    run
}

/// A landed hit pays XP. The kill itself pays nothing extra.
pub fn land_hit(combat: &mut CombatState, skills: &mut SkillXp, reach: bool) -> bool {
    land_hit_crossing(combat, skills, reach).0
}

/// TOTAL-LEVEL-2 — `land_hit` plus the COMBAT level crossing from `gain`.
pub fn land_hit_crossing(combat: &mut CombatState, skills: &mut SkillXp, reach: bool) -> (bool, Option<u32>) {
    if !reach || combat.predator_taken >= HEALTH {
        return (false, None);
    }
    combat.predator_taken = combat.predator_taken.saturating_add(HIT).min(HEALTH);
    if skills.combat_day_hits < crate::skill_xp::COMBAT_PAYING_HITS_PER_DAY {
        skills.combat_day_hits += 1;
        (true, skills.gain(COMBAT, XP_PER_LANDED_HIT))
    } else {
        (true, None) // COMBAT-XP-PACE-1: past the daily cap `gain` never runs
    }
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

/// PREDATOR-RESPAWN-1 — one step of the down timer: `(elapsed, fire)`.
/// Fires when `elapsed + dt >= PREDATOR_RESPAWN_SECS`; zero while alive, inert at `dt <= 0`.
pub fn respawn_step(elapsed: f32, dt: f32, down: bool) -> (f32, bool) {
    if !down {
        return (0.0, false);
    }
    if dt <= 0.0 {
        return (elapsed, false);
    }
    let elapsed = elapsed + dt;
    if elapsed >= PREDATOR_RESPAWN_SECS {
        (0.0, true)
    } else {
        (elapsed, false)
    }
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

/// TOTAL-LEVEL-2 — a crossing hit shows the level line; a crossing kill joins it to "is down".
pub fn fire_line(place: PlaceId, combat: &CombatState, hit: bool, gained: u32, level: Option<u32>) -> Option<String> {
    match (hit, level) {
        (true, Some(lvl)) if combat.predator_taken >= HEALTH => {
            Some(format!("{PREDATOR_NAME} is down · {}", level_up_line(COMBAT, lvl)))
        }
        (true, Some(lvl)) => Some(level_up_line(COMBAT, lvl)),
        _ => fire_cue(place, combat, hit, gained),
    }
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
            .init_resource::<StrikeBuffer>()
            .add_systems(Startup, spawn_predator)
            .add_systems(Startup, spawn_lunge_tell)
            .add_systems(Startup, spawn_rise_tell)
            .add_systems(Update, rise_tell_show.after(predator_respawn))
            .add_systems(Update, (combat_fire, predator_lunge, lunge_tell).chain().after(InputMapSet))
            .add_systems(Update, hit_flash.after(combat_fire))
            .add_systems(Update, predator_respawn.after(combat_fire).before(predator_lunge).before(hit_flash))
            .add_systems(FixedUpdate, hold_strike_buffer);
    }
}

fn spawn_predator(
    mut commands: Commands,
    (mut meshes, mut materials): (ResMut<Assets<Mesh>>, ResMut<Assets<StandardMaterial>>),
) {
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.35, 0.9))),
        MeshMaterial3d(materials.add(PREDATOR_COLOR)),
        Transform::from_xyz(PREDATOR_HOME[0], 0.8, PREDATOR_HOME[1]),
        Visibility::Hidden,
        Predator,
        Name::new(PREDATOR_NAME),
    ));
}

/// LUNGE-TELL-1 — 20 hidden discs sharing one mesh and one neutral material.
fn spawn_lunge_tell(
    mut commands: Commands,
    (mut meshes, mut materials): (ResMut<Assets<Mesh>>, ResMut<Assets<StandardMaterial>>),
) {
    let mesh = meshes.add(Cylinder::new(LUNGE_TELL_DOT_RADIUS, LUNGE_TELL_DOT_HEIGHT));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.86, 0.85, 0.82),
        unlit: true,
        ..default()
    });
    for at in lunge_tell_ring() {
        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(at),
            Visibility::Hidden,
            LungeTell,
        ));
    }
}

/// RESPAWN-TELL-1 — one hidden pillar at `PREDATOR_HOME`, primitive mesh and colour only.
fn spawn_rise_tell(
    mut commands: Commands,
    (mut meshes, mut materials): (ResMut<Assets<Mesh>>, ResMut<Assets<StandardMaterial>>),
) {
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(RISE_TELL_RADIUS, RISE_TELL_HEIGHT))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: PREDATOR_COLOR,
            unlit: true,
            ..default()
        })),
        rise_tell_transform(0.0),
        Visibility::Hidden,
        RiseTell,
    ));
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn combat_fire(
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
    input: Option<Res<PlayerInput>>,
    (allocate, voice, ledger, satchel, realm): (
        Option<Res<RbeAllocateChoice>>,
        Option<Res<VoiceYard>>,
        Option<Res<LedgerYard>>,
        Option<Res<HumanInventory>>,
        Option<Res<HumanSoftPanels>>,
    ),
    (buffer, fixed, diff): (Option<ResMut<StrikeBuffer>>, Option<Res<Time<Fixed>>>, Option<ResMut<crate::difficulty::AreaDifficulty>>),
) {
    let fire = buttons.is_some_and(|b| b.just_pressed(FIRE))
        || gamepads.iter().any(|g| g.just_pressed(FIRE_PAD));
    let gate = skill_gate_allows(
        allocate.is_some_and(|a| a.panel_open),
        voice.is_some_and(|v| v.sash_open),
        ledger.is_some_and(|l| l.sash_open),
        satchel.is_some_and(|i| i.open),
        realm.is_some_and(|r| r.realm_open),
    );
    let pressed = strike_pressed(fire, input.and_then(|i| i.ability_slot), gate);
    let pulse = buffer.as_ref().is_some_and(|b| b.fire);
    let (true, Some(travel), Some(presence), Some(mut skills)) = (pressed || pulse, travel, presence, skills) else {
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
    let allowed = fire_allowed(grab, plate_open, ui.iter().any(|i| *i != Interaction::None), prompts);
    let reach = in_reach(travel.current, &combat, presence.position.x, presence.position.z);
    // COMBAT-FEEL-1: the hit lands here in Update; only expiry and reach run on the fixed tick.
    let now_fixed = fixed.map_or(0.0, |t| t.elapsed_secs_f64());
    let run = match buffer {
        Some(mut buf) => strike_buffer_step(&mut buf, &[], (pressed, allowed, gate, reach), now_fixed),
        None => pressed && allowed,
    };
    if run {
        combat.strikes = combat.strikes.wrapping_add(1);
        let before = skills.xp[COMBAT];
        let (hit, level) = land_hit_crossing(&mut combat, &mut skills, reach);
        if let (true, Some(mut diff)) = (hit && combat.predator_taken >= HEALTH, diff) {
            diff.record_clear(travel.current); // DIFFICULTY-TIERS-1: the predator defeat clears this area
        }
        let gained = skills.xp[COMBAT].saturating_sub(before);
        if let (Some(line), Some(epi)) = (fire_line(travel.current, &combat, hit, gained, level), epi.as_mut()) {
            set_pulse(epi, line, now + CUE_SECS);
        }
    }
}

/// COMBAT-FEEL-1 — FixedUpdate hold: expiry and reach only. Never writes CombatState, SkillXp or the pulse line.
fn hold_strike_buffer(
    time: Option<Res<Time>>,
    buffer: Option<ResMut<StrikeBuffer>>,
    (travel, presence, combat): (Option<Res<HexTravelState>>, Option<Res<SoftPresence>>, Option<Res<CombatState>>),
) {
    let (Some(time), Some(mut buf)) = (time, buffer.filter(|b| b.until.is_some())) else {
        return;
    };
    let reach = match (travel, presence, combat) {
        (Some(t), Some(p), Some(c)) => in_reach(t.current, &c, p.position.x, p.position.z),
        _ => false,
    };
    strike_buffer_tick(&mut buf, time.elapsed_secs_f64(), reach);
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

/// LUNGE-TELL-1 — chained after `predator_lunge`, so the timer and reach are post-tick.
fn lunge_tell(
    door: Option<Res<LaunchDoor>>,
    travel: Option<Res<HexTravelState>>,
    presence: Option<Res<SoftPresence>>,
    combat: Option<Res<CombatState>>,
    mut dots: Query<&mut Visibility, With<LungeTell>>,
) {
    let (Some(travel), Some(presence), Some(combat)) = (travel, presence, combat) else {
        for mut vis in &mut dots {
            vis.set_if_neq(Visibility::Hidden);
        }
        return;
    };
    // The exact reach term of predator_lunge, combat_core.rs:326-327. No second formula.
    let (in_yard, p) = (door.as_deref().copied() == Some(LaunchDoor::InYard), presence.position);
    let reach = in_yard && in_reach(travel.current, &combat, p.x, p.z);
    for mut vis in &mut dots {
        vis.set_if_neq(if tell_visible(combat.lunge_timer, reach) {
            Visibility::Visible
        } else {
            Visibility::Hidden
        });
    }
}

/// HIT-JUICE-1 — after `combat_fire`: a rise in `predator_taken` is one landed hit (Update clock).
/// Writes the material only while a flash is live, plus one restore. A kill never tints.
#[allow(clippy::too_many_arguments)]
fn hit_flash(
    time: Res<Time>,
    (mut last, mut since): (Local<u32>, Local<Option<f64>>),
    combat: Option<Res<CombatState>>,
    pool: Option<ResMut<SoftRbePool>>,
    materials: Option<ResMut<Assets<StandardMaterial>>>,
    predator: Query<&MeshMaterial3d<StandardMaterial>, With<Predator>>,
    gamepads: Query<Entity, With<Gamepad>>,
    mut rumble: MessageWriter<GamepadRumbleRequest>,
) {
    let (Some(combat), Some(mut pool), Some(mut materials)) = (combat, pool, materials) else {
        return;
    };
    let (taken, now, live) = (combat.predator_taken, time.elapsed_secs_f64(), since.is_some());
    let (rise, kill) = (taken > std::mem::replace(&mut *last, taken), taken >= HEALTH);
    if rise {
        pool.kick = pool.kick.max(hit_kick(kill));
        rumble_harvest(&mut rumble, gamepads.iter(), false);
        *since = (!kill).then_some(now);
    }
    let strength = since.map_or(0.0, |at| flash_strength((now - at) as f32));
    if strength <= 0.0 {
        *since = None;
        if !live {
            return;
        }
    }
    for handle in &predator {
        if let Some(mut mat) = materials.get_mut(&handle.0) {
            mat.base_color = flash_color(strength);
        }
    }
}

/// PREDATOR-RESPAWN-1 — counts on `Res<Time>` everywhere; writes only `predator_taken`, `respawned`
/// and `down_secs`, the down clock's only writer.
fn predator_respawn(time: Res<Time>, combat: Option<ResMut<CombatState>>) {
    let Some(mut combat) = combat else {
        return;
    };
    let (down_secs, fire) = respawn_step(combat.down_secs, time.delta_secs(), combat.predator_taken >= HEALTH);
    if combat.down_secs != down_secs {
        combat.down_secs = down_secs;
    }
    if fire {
        (combat.predator_taken, combat.respawned) = (0, true);
    }
}

/// RESPAWN-TELL-1 — after `predator_respawn`, so the frame it fires already reads hidden.
fn rise_tell_show(
    travel: Option<Res<HexTravelState>>,
    combat: Option<Res<CombatState>>,
    mut pillar: Query<(&mut Visibility, &mut Transform), With<RiseTell>>,
) {
    let rise = match (travel, combat) {
        (Some(t), Some(c)) => rise_tell(t.current, c.predator_taken, c.down_secs),
        _ => None,
    };
    for (mut vis, mut at) in &mut pillar {
        vis.set_if_neq(if rise.is_some() { Visibility::Visible } else { Visibility::Hidden });
        if let Some(r) = rise {
            at.set_if_neq(rise_tell_transform(r));
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
    fn slot_zero_inert_under_each_panel_and_guard() {
        assert!(skill_gate_allows(false, false, false, false, false));
        for i in 0..5 {
            let open: [bool; 5] = std::array::from_fn(|j| j == i);
            let gate = skill_gate_allows(open[0], open[1], open[2], open[3], open[4]);
            assert!(!gate && !strike_pressed(false, Some(0), gate), "panel {i} open");
            assert!(strike_pressed(true, None, gate), "panels never gate LMB/pad");
        }
        assert!(strike_pressed(false, Some(0), true));
        let guards = [
            fire_grab_terms(false, false, false, true),
            fire_grab_terms(true, true, false, true),
            fire_grab_terms(true, false, true, true),
            fire_grab_terms(true, false, false, false),
        ];
        for grab in guards {
            assert!(!(strike_pressed(false, Some(0), true) && fire_allowed(grab, false, false, false)));
        }
        let ok = fire_grab_terms(true, false, false, true);
        for (plate, ui, prompts) in [(true, false, false), (false, true, false), (false, false, true)] {
            assert!(!(strike_pressed(false, Some(0), true) && fire_allowed(ok, plate, ui, prompts)));
        }
    }

    #[test]
    fn slots_one_to_three_never_write() {
        for slot in [Some(1), Some(2), Some(3)] {
            assert!(!slot_strikes(slot) && !strike_pressed(false, slot, true));
        }
        assert!(!slot_strikes(None) && slot_strikes(Some(0)));
        let (c, s, epi) = (CombatState::default(), SkillXp::default(), FirstHarvestEpiphany::default());
        let (mut c2, mut s2, epi2) = (CombatState::default(), SkillXp::default(), FirstHarvestEpiphany::default());
        if strike_pressed(false, Some(2), true) {
            land_hit_crossing(&mut c2, &mut s2, true);
        }
        assert_eq!((c2.predator_taken, c2.player_taken), (c.predator_taken, c.player_taken));
        assert_eq!(s2.xp, s.xp);
        assert_eq!((epi2.pulse_line, epi2.pulse_until), (epi.pulse_line, epi.pulse_until));
    }

    #[test]
    fn lmb_and_key_one_same_frame_is_one_hit() {
        let (mut c, mut s) = (CombatState::default(), SkillXp::default());
        if strike_pressed(true, Some(0), true) {
            assert!(land_hit_crossing(&mut c, &mut s, true).0);
        }
        assert_eq!((c.predator_taken, s.xp[COMBAT]), (HIT, XP_PER_LANDED_HIT));
    }

    #[test]
    fn slot_zero_off_heartwood_hits_nothing() {
        let (mut c, mut s, [x, z]) = (CombatState::default(), SkillXp::default(), PREDATOR_HOME);
        for place in [PlaceId::Sanctuary, PlaceId::Depths] {
            assert!(strike_pressed(false, Some(0), true) && !predator_present(place));
            let reach = in_reach(place, &c, x, z);
            assert_eq!(land_hit_crossing(&mut c, &mut s, reach), (false, None));
        }
        assert!(c.predator_taken == 0 && s.xp[COMBAT] == 0);
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

    #[test]
    fn crossing_hit_prints_level_line_and_kill_joins_it() {
        let step = crate::skill_xp::XP_CURVE_STEP;
        for (taken, want) in [(0, String::new()), (HEALTH - HIT, format!("{PREDATOR_NAME} is down · "))] {
            let (mut c, mut s) = (CombatState { predator_taken: taken, ..Default::default() }, SkillXp::default());
            s.xp[COMBAT] = step - XP_PER_LANDED_HIT;
            let (hit, level) = land_hit_crossing(&mut c, &mut s, true);
            let line = fire_line(PlaceId::Heartwood, &c, hit, XP_PER_LANDED_HIT, level).unwrap();
            assert_eq!(line, format!("{want}[NAME TBD] · level {}", level.expect("crossing")));
        }
    }

    #[test]
    fn no_crossing_keeps_feel_pass_cues() {
        let (mut c, mut s) = (CombatState::default(), SkillXp::default());
        let (hit, level) = land_hit_crossing(&mut c, &mut s, true);
        assert_eq!(level, None);
        assert_eq!(fire_line(PlaceId::Heartwood, &c, hit, 10, level), fire_cue(PlaceId::Heartwood, &c, hit, 10));
        assert_eq!(fire_line(PlaceId::Heartwood, &c, false, 0, Some(2)), Some(format!("Missed {PREDATOR_NAME}")));
        let down = CombatState { predator_taken: HEALTH, ..Default::default() };
        assert_eq!(fire_line(PlaceId::Heartwood, &down, true, 0, None), Some(format!("{PREDATOR_NAME} is down")));
    }

    #[test]
    fn restored_combat_xp_prints_nothing() {
        use bevy::ecs::system::RunSystemOnce;
        let mut world = World::new();
        world.insert_resource(Time::<()>::default());
        world.insert_resource(HexTravelState::default());
        world.insert_resource(SoftPresence::default());
        world.insert_resource(CombatState::default());
        let mut s = SkillXp::default();
        s.xp = crate::skill_xp::skills_with_combat_on_load(None, Some(5 * crate::skill_xp::XP_CURVE_STEP), 0, 0);
        assert!(s.xp[COMBAT] > 0);
        world.insert_resource(s);
        world.insert_resource(FirstHarvestEpiphany::default());
        world.clear_trackers();
        world.run_system_once(combat_fire).unwrap();
        assert!(!world.is_resource_changed::<FirstHarvestEpiphany>());
        assert!(world.resource::<FirstHarvestEpiphany>().pulse_line.is_empty());
    }

    /// Helper-level replay (not an App test): press out of reach, reach arrives at `reach_at`.
    fn replay(fps: f64, reach_at: f64) -> (u32, u32, Option<String>) {
        let (mut buf, mut c, mut s) = (StrikeBuffer::default(), CombatState::default(), SkillXp::default());
        let (step, tick, mut hits, mut line) = (1.0 / fps, 1.0 / crate::feel_move::SIM_HZ, 0, None);
        let (mut vt, mut ft) = (0.0, 0.0);
        for frame in 0..(fps as u32) {
            let mut ticks = Vec::new();
            while ft + tick <= vt {
                ft += tick;
                ticks.push((ft, ft >= reach_at));
            }
            let (before, reach) = (s.xp[COMBAT], vt >= reach_at);
            if strike_buffer_step(&mut buf, &ticks, (frame == 0, true, true, reach), ft) {
                let (hit, level) = land_hit_crossing(&mut c, &mut s, reach);
                hits += u32::from(hit);
                line = fire_line(PlaceId::Heartwood, &c, hit, s.xp[COMBAT] - before, level).or(line);
            }
            vt += step;
        }
        (hits, s.xp[COMBAT], line)
    }

    #[test]
    fn buffered_strike_same_at_30_and_144_fps() {
        let slow = replay(30.0, 0.06);
        assert_eq!(slow, replay(144.0, 0.06));
        assert_eq!((slow.0, slow.1), (1, XP_PER_LANDED_HIT));
        let want = CombatState { predator_taken: HIT, ..Default::default() };
        assert_eq!(slow.2, fire_cue(PlaceId::Heartwood, &want, true, XP_PER_LANDED_HIT));
    }

    #[test]
    fn buffered_strike_expires_past_window() {
        assert!([30.0, 144.0].into_iter().all(|fps| replay(fps, USE_BUFFER_SECS + 0.05).0 == 0));
    }

    #[test]
    fn zero_or_many_ticks_in_a_frame_is_one_hit() {
        let open = (false, true, true, true);
        let mut buf = StrikeBuffer { until: None, fire: true };
        assert!(strike_buffer_step(&mut buf, &[], open, 0.0) && !strike_buffer_step(&mut buf, &[], open, 0.0));
        let mut buf = StrikeBuffer { until: Some(USE_BUFFER_SECS), fire: false };
        let many = [(0.01, true), (0.02, true), (0.03, true)];
        assert!(strike_buffer_step(&mut buf, &many, open, 0.03) && !strike_buffer_step(&mut buf, &many, open, 0.04));
        let mut buf = StrikeBuffer { until: None, fire: true };
        assert!(strike_buffer_step(&mut buf, &[], (true, true, true, true), 0.0), "edge + pulse is one strike");
        assert_eq!(buf, StrikeBuffer::default());
    }

    #[test]
    fn closed_gate_on_consume_writes_nothing() {
        for terms in [(false, false, true, true), (false, true, false, true), (false, true, true, false)] {
            let mut buf = StrikeBuffer { until: Some(USE_BUFFER_SECS), fire: true };
            assert!(!strike_buffer_step(&mut buf, &[], terms, 0.0));
            assert_eq!(buf, StrikeBuffer::default());
        }
    }

    #[test]
    fn slots_one_to_three_never_arm() {
        for slot in [Some(1), Some(2), Some(3)] {
            let mut buf = StrikeBuffer::default();
            let press = strike_pressed(false, slot, true);
            assert!(!strike_buffer_step(&mut buf, &[], (press, true, true, false), 0.0) && buf.until.is_none());
        }
    }

    #[test]
    fn level_lines_never_read_as_tend_breathe() {
        let down = CombatState { predator_taken: HEALTH, ..Default::default() };
        let joined = fire_line(PlaceId::Heartwood, &down, true, XP_PER_LANDED_HIT, Some(2)).unwrap();
        for line in [level_up_line(COMBAT, 2), joined] {
            assert!(!crate::mercy_harvest_nodes::pulse_line_is_tend_breathe(&line), "{line}");
        }
    }

    #[test]
    fn lunge_tell_opens_at_the_window_edge() {
        assert!(tell_visible(LUNGE_SECS - LUNGE_TELL_SECS, true));
    }

    #[test]
    fn lunge_tell_closed_just_below_the_window() {
        assert!(!tell_visible(LUNGE_SECS - LUNGE_TELL_SECS - 1e-3, true));
    }

    #[test]
    fn lunge_tell_closed_after_the_wrap() {
        let mut c = CombatState { lunge_timer: 0.59, ..Default::default() };
        predator_tick(&mut c, 0.02, true);
        assert!(c.lunge_timer < LUNGE_SECS - LUNGE_TELL_SECS, "{}", c.lunge_timer);
        assert!(!tell_visible(c.lunge_timer, true));
    }

    #[test]
    fn lunge_tell_closed_when_reach_is_lost() {
        assert!(!tell_visible(LUNGE_SECS - 1e-3, false));
    }

    #[test]
    fn lunge_tell_ring_matches_reach() {
        let ring = lunge_tell_ring();
        assert_eq!(ring.len(), 20);
        let home = Vec2::from_array(PREDATOR_HOME);
        for at in ring {
            assert!((Vec2::new(at.x, at.z).distance(home) - REACH).abs() < 1e-4, "{at}");
            assert_eq!(at.y, LUNGE_TELL_LIFT);
        }
    }

    #[test]
    fn flash_strength_edges_and_monotone() {
        assert_eq!((flash_strength(0.0), flash_strength(FLASH_SECS), flash_strength(1.0)), (1.0, 0.0, 0.0));
        let steps: Vec<f32> = (0..=150).map(|ms| flash_strength(ms as f32 * 0.001)).collect();
        assert!(steps.windows(2).all(|w| w[1] <= w[0]), "{steps:?}");
        assert_eq!(flash_color(0.0), PREDATOR_COLOR);
    }

    #[test]
    fn hit_kick_order() {
        assert_eq!((hit_kick(false), hit_kick(true)), (HIT_KICK, KILL_KICK));
        assert!(0.0 < hit_kick(false) && hit_kick(false) < hit_kick(true) && hit_kick(true) < 1.0);
    }

    /// Bare App: no plugins, one Predator material, one pad. `step` returns (kick, rumbles) per update.
    fn flash_app() -> (App, Handle<StandardMaterial>) {
        let mut app = App::new();
        app.add_message::<GamepadRumbleRequest>()
            .insert_resource(Time::<()>::default())
            .init_resource::<CombatState>()
            .init_resource::<SoftRbePool>()
            .init_resource::<Assets<StandardMaterial>>()
            .add_systems(Update, hit_flash);
        let handle = app.world_mut().resource_mut::<Assets<StandardMaterial>>().add(PREDATOR_COLOR);
        app.world_mut().spawn((MeshMaterial3d(handle.clone()), Predator));
        app.world_mut().spawn(Gamepad::default());
        (app, handle)
    }

    fn step(app: &mut App, secs: f32) -> (f32, usize) {
        app.world_mut().resource_mut::<SoftRbePool>().kick = 0.0;
        app.world_mut().resource_mut::<Time>().advance_by(std::time::Duration::from_secs_f32(secs));
        app.update();
        let rumbles = app.world_mut().resource_mut::<Messages<GamepadRumbleRequest>>().drain().count();
        (app.world().resource::<SoftRbePool>().kick, rumbles)
    }

    fn colour(app: &App, handle: &Handle<StandardMaterial>) -> Color {
        app.world().resource::<Assets<StandardMaterial>>().get(handle).unwrap().base_color
    }

    #[test]
    fn one_rise_is_one_hit_across_zero_one_or_many_updates() {
        let (mut app, handle) = flash_app();
        assert_eq!(step(&mut app, 0.0), (0.0, 0), "no land, no feel");
        app.world_mut().resource_mut::<CombatState>().predator_taken = HIT;
        assert_eq!(step(&mut app, 0.0), (HIT_KICK, 1));
        assert_eq!(colour(&app, &handle), flash_color(1.0), "flash is live");
        for updates in [0, 1, 6] {
            (0..updates).for_each(|_| assert_eq!(step(&mut app, 0.03), (0.0, 0), "no new land"));
        }
        assert_eq!(colour(&app, &handle), PREDATOR_COLOR, "restored exactly");
    }

    #[test]
    fn kill_restores_and_never_tints() {
        let (mut app, handle) = flash_app();
        app.world_mut().resource_mut::<CombatState>().predator_taken = HEALTH - HIT;
        assert_eq!(step(&mut app, 0.0), (HIT_KICK, 1));
        assert_ne!(colour(&app, &handle), PREDATOR_COLOR, "the hit flash is live");
        app.world_mut().resource_mut::<CombatState>().predator_taken = HEALTH;
        assert_eq!(step(&mut app, 0.02), (KILL_KICK, 1));
        assert_eq!(colour(&app, &handle), PREDATOR_COLOR, "kill restores a live flash");
        assert_eq!(step(&mut app, 0.02), (0.0, 0));
        assert_eq!(colour(&app, &handle), PREDATOR_COLOR, "kill never tints");
    }

    #[test]
    fn respawn_step_edges() {
        let down = |at: f32| respawn_step(0.0, at, true);
        assert_eq!(down(PREDATOR_RESPAWN_SECS - 0.01), (PREDATOR_RESPAWN_SECS - 0.01, false));
        assert_eq!(down(PREDATOR_RESPAWN_SECS), (0.0, true), "fires at 30 s");
        assert_eq!(respawn_step(29.0, 1.0, true), (0.0, true));
        assert_eq!(respawn_step(12.0, 5.0, false), (0.0, false), "no tick while alive");
        assert_eq!(respawn_step(12.0, 0.0, true), (12.0, false), "dt 0 inert");
        assert_eq!(respawn_step(12.0, -1.0, true), (12.0, false));
    }

    #[test]
    fn one_reset_per_kill_and_re_down_starts_fresh() {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default())
            .init_resource::<CombatState>()
            .add_systems(Update, predator_respawn);
        let mut tick = |app: &mut App, secs: f32| {
            app.world_mut().resource_mut::<Time>().advance_by(std::time::Duration::from_secs_f32(secs));
            app.update();
            app.world().resource::<CombatState>().predator_taken
        };
        assert_eq!(tick(&mut app, 40.0), 0, "alive: nothing to reset");
        assert!(!app.world().resource::<CombatState>().respawned);
        app.world_mut().resource_mut::<CombatState>().predator_taken = HEALTH;
        assert_eq!(tick(&mut app, 20.0), HEALTH);
        assert_eq!(tick(&mut app, 10.0), 0, "one reset at 30 s");
        assert!(app.world().resource::<CombatState>().respawned);
        assert_eq!(tick(&mut app, 25.0), 0, "alive again: timer zeroed");
        app.world_mut().resource_mut::<CombatState>().predator_taken = HEALTH;
        assert_eq!(tick(&mut app, 20.0), HEALTH, "re-down starts fresh, not at 25 s");
        assert_eq!(tick(&mut app, 10.0), 0);
    }

    #[test]
    fn paying_hit_edges_19_20_21() {
        let cap = crate::skill_xp::COMBAT_PAYING_HITS_PER_DAY;
        for (count, pays) in [(cap - 1, true), (cap, false), (cap + 1, false)] {
            let (mut c, mut s) = (CombatState::default(), SkillXp::default());
            s.combat_day_hits = count;
            let hit = land_hit_crossing(&mut c, &mut s, true);
            assert_eq!(c.predator_taken, HIT, "damage unchanged at {count}");
            assert_eq!(s.xp[COMBAT], if pays { XP_PER_LANDED_HIT } else { 0 }, "at {count}");
            assert_eq!(s.combat_day_hits, if pays { count + 1 } else { count });
            assert_eq!(hit, (true, None));
        }
    }

    #[test]
    fn respawned_life_pays_until_the_cap() {
        let (mut c, mut s) = (CombatState { respawned: true, ..Default::default() }, SkillXp::default());
        let xp = s.xp[COMBAT];
        let place = PlaceId::Heartwood;
        let (mut paid, mut lives, cap) = (0, 3, crate::skill_xp::COMBAT_PAYING_HITS_PER_DAY); // 14 hits a life: 3 lives cross the cap
        while c.predator_taken < HEALTH {
            let before = c.predator_taken;
            let paid_before = s.xp[COMBAT];
            assert!(land_hit_crossing(&mut c, &mut s, true).0);
            let gained = s.xp[COMBAT] - paid_before;
            assert_eq!(c.predator_taken, (before + HIT).min(HEALTH), "HIT damage lands");
            assert_eq!(gained, if paid < cap { XP_PER_LANDED_HIT } else { 0 }, "pays until the cap");
            paid += u32::from(gained > 0);
            let line = fire_line(place, &c, true, gained, None).unwrap();
            assert_eq!(line.contains('+'), gained > 0 && c.predator_taken < HEALTH, "{line}");
            if c.predator_taken >= HEALTH && lives > 1 {
                (c.predator_taken, lives) = (0, lives - 1);
            }
        }
        assert_eq!((paid, s.xp[COMBAT], s.combat_day_hits), (cap, xp + cap * XP_PER_LANDED_HIT, cap));
        assert_eq!(fire_cue(place, &c, true, 0).unwrap(), format!("{PREDATOR_NAME} is down"));
    }

    #[test]
    fn defeat_leaves_respawned_set() {
        let mut c = CombatState { respawned: true, player_taken: HEALTH, ..Default::default() };
        reseat_on_defeat(&mut c, &mut SoftPresence::default());
        assert!(c.respawned && c.player_taken == 0);
    }

    /// Stand-in for `combat_fire`: one strike lands `HIT` past the same kill guard.
    #[derive(Resource, Default)]
    struct Strike(bool);

    fn stand_in_fire(mut strike: ResMut<Strike>, mut combat: ResMut<CombatState>) {
        if std::mem::take(&mut strike.0) && combat.predator_taken < HEALTH {
            combat.predator_taken = combat.predator_taken.saturating_add(HIT).min(HEALTH);
        }
    }

    #[test]
    fn reset_then_hit_on_adjacent_frames_is_one_flash_rise() {
        let (mut app, _) = flash_app();
        app.init_resource::<Strike>()
            .add_systems(Update, stand_in_fire.before(predator_respawn).before(hit_flash))
            .add_systems(Update, predator_respawn.after(combat_fire).before(predator_lunge).before(hit_flash));
        app.world_mut().resource_mut::<CombatState>().predator_taken = HEALTH;
        assert_eq!(step(&mut app, 0.0), (KILL_KICK, 1), "the kill");
        assert_eq!(step(&mut app, PREDATOR_RESPAWN_SECS - 0.5), (0.0, 0));
        assert_eq!(app.world().resource::<CombatState>().predator_taken, HEALTH);
        app.world_mut().resource_mut::<Strike>().0 = true;
        assert_eq!(step(&mut app, 0.5), (0.0, 0), "reset frame: blocked strike, no rise");
        assert_eq!(app.world().resource::<CombatState>().predator_taken, 0);
        app.world_mut().resource_mut::<Strike>().0 = true;
        assert_eq!(step(&mut app, 0.02), (HIT_KICK, 1), "next frame: exactly one rise");
        assert_eq!(step(&mut app, 0.02), (0.0, 0));
    }

    #[test]
    fn rise_tell_onset_and_gates() {
        let (hw, onset) = (PlaceId::Heartwood, PREDATOR_RESPAWN_SECS - PREDATOR_TELL_SECS);
        assert_eq!(rise_tell(hw, HEALTH, onset), Some(0.0), "onset at 28.5 s");
        assert_eq!(rise_tell(hw, HEALTH, onset - 0.1), None, "hidden at 28.4 s");
        assert_eq!(rise_tell(hw, HEALTH, PREDATOR_RESPAWN_SECS), Some(1.0));
        assert_eq!(rise_tell(hw, HEALTH - HIT, onset + 1.0), None, "hidden while alive");
        assert_eq!(rise_tell(PlaceId::Sanctuary, HEALTH, onset + 1.0), None, "never in Sanctuary");
        assert_eq!(rise_tell(PlaceId::Depths, HEALTH, onset + 1.0), None, "never in Depths");
        let (low, high) = (rise_tell_transform(0.2), rise_tell_transform(0.8));
        assert!(high.scale.y > low.scale.y && high.translation.y > low.translation.y, "it rises");
        let base = |t: Transform| t.translation.y - RISE_TELL_HEIGHT * t.scale.y * 0.5;
        assert!(base(low).abs() < 1e-5 && base(high).abs() < 1e-5, "grows from the ground");
        assert_eq!((high.translation.x, high.translation.z), (PREDATOR_HOME[0], PREDATOR_HOME[1]));
    }

    #[test]
    fn rise_tell_shows_then_clears_on_respawn() {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default())
            .insert_resource(HexTravelState { current: PlaceId::Heartwood })
            .init_resource::<CombatState>()
            .add_systems(Update, (predator_respawn, rise_tell_show).chain());
        let pillar = app.world_mut().spawn((Transform::default(), Visibility::Hidden, RiseTell)).id();
        let mut tick = |app: &mut App, secs: f32| {
            app.world_mut().resource_mut::<Time>().advance_by(std::time::Duration::from_secs_f32(secs));
            app.update();
            *app.world().get::<Visibility>(pillar).unwrap()
        };
        assert_eq!(tick(&mut app, 40.0), Visibility::Hidden, "hidden while alive");
        app.world_mut().resource_mut::<CombatState>().predator_taken = HEALTH;
        assert_eq!(tick(&mut app, PREDATOR_RESPAWN_SECS - PREDATOR_TELL_SECS - 0.1), Visibility::Hidden);
        assert_eq!(tick(&mut app, 0.5), Visibility::Visible, "the creature rises");
        app.world_mut().resource_mut::<HexTravelState>().current = PlaceId::Sanctuary;
        assert_eq!(tick(&mut app, 0.1), Visibility::Hidden, "hidden in Sanctuary");
        app.world_mut().resource_mut::<HexTravelState>().current = PlaceId::Heartwood;
        assert_eq!(tick(&mut app, 0.1), Visibility::Visible);
        assert_eq!(tick(&mut app, 1.0), Visibility::Hidden, "hidden after respawn");
        let c = app.world().resource::<CombatState>();
        assert!(c.respawned && c.predator_taken == 0 && c.down_secs == 0.0);
    }
}
