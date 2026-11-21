//! COMBAT-CORE-1 — offline PvE: `predator_present` gates the Heartwood Teeth run;
//! Sanctuary practice goes through `fight_present`. Health is client-local.
//! No PvP, no kill XP. Named finds are the loot. Online stays grey.

use bevy::input::gamepad::GamepadRumbleRequest;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use shared::heartwood_lamp::{heartwood_bath_return, HEARTWOOD_BATH_RETURN, WATER_POND_CENTER};
use shared::hex_travel::PlaceId;
use shared::temper::Standing;

use crate::coop_voice::VoiceYard;
use crate::feel_move::{buffer_alive, USE_BUFFER_SECS};
use crate::first_harvest_epiphany::{CalmQueue, FirstHarvestEpiphany};
use crate::loot_drops::{find_ready, load_finds, save_finds, FoundFinds};
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
use crate::difficulty::{cue_line, enemy_hit, hides_cues, tell_onset, AreaDifficulty, DifficultyTier};
use crate::run_ledger::{premium_paid, RunLedger, RunRow, SKILL_PREMIUM_LIVE};
use crate::wellsworn::STAVEHOLD_GATE_SPOT;
use shared::peace_audio::{audio_output_safe, STING_ASSET};
/// COMBAT-FEEL-SFX-1 — the landed-hit thud (Kenney Impact Sounds, CC0), distinct from the sting.
pub const HIT_ASSET: &str = "audio/combat_hit.ogg";
/// The hit file peaks at -4.0 dBFS; x0.05 (-26 dB) keeps it under the sting's -27.7 dBFS.
pub const HIT_GAIN: f32 = 0.05;
pub const FIND_ASSET: &str = "audio/find_pickup.ogg";
pub const FIND_GAIN: f32 = 0.05; // peak −14.6 dBFS; same ceiling as hit

pub const PREDATOR_NAME: &str = "Heartwood Prowler";
pub const HEALTH: u32 = 200; // player and predator pools, per CUT A1
/// COMBAT_AND_PVP.md:435/:676 — the basic player strike lands 8 (predator and droid alike). Gear adds on top; no tier scaling.
pub const HIT: u32 = 8;
/// COMBAT_AND_PVP.md:486/:678 — Predator "Lunge, 0.6 s" hits 15, before `enemy_hit` tier scaling and Plate.
pub const LUNGE_HIT: u32 = 15;
pub const LUNGE_SECS: f32 = 0.6;
pub const REACH: f32 = HARVEST_REACH; // both sides reuse the harvest reach
pub const XP_PER_LANDED_HIT: u32 = 10; // tuning value, set in playtest
/// Teeth band: past the lamp, pond, Wards and threshold node, inside PLAY_HALF.
pub const PREDATOR_HOME: [f32; 2] = [-18.0, -18.0];
/// COMBAT-REACH-1 — the Sanctuary practice fight, south of the Stavehold gate on the Contested side.
pub const STAVEHOLD_FIGHT_HOME: [f32; 2] = [0.0, -17.0];
// CL-R1: clear of the gate by REACH + SPOT_REACH (2.0, wellsworn.rs:27), and the ring plus margins fit the yard.
const _: () = assert!(STAVEHOLD_GATE_SPOT[1] - STAVEHOLD_FIGHT_HOME[1] > REACH + 2.0);
const _: () = assert!(-STAVEHOLD_FIGHT_HOME[1] + REACH + crate::grazer::RING_MARGIN + crate::grazer::EDGE_MARGIN < crate::terrain::PLAY_HALF);
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
    /// COMBAT-FEEL-LOOK-1 — landed lunges so far (wrapping); `hit_flash` kicks on a rise.
    pub hurt: u32,
    /// COMBAT-CORE-2a — seconds of Plate left; up while > 0. Armed by key 2, decays in `predator_lunge`.
    pub plate_secs: f32,
    /// COMBAT-CORE-2a — Plate already armed this lunge; clears on the timer wrap or reset and in `reseat_at`.
    pub plate_armed: bool,
}

/// COMBAT-CORE-2a — COMBAT_AND_PVP.md:456 "Plate, 3 s, incoming damage reduced by 30%".
pub const PLATE_SECS: f32 = 3.0;
/// COMBAT-CORE-2a — a lunge under Plate: ×0.7, integer half-up like `enemy_hit` (15→11, 17→12).
pub fn plate_hit(hit: u32) -> u32 { (hit * 70 + 50) / 100 }
/// COMBAT-CORE-2a — the equipped owned find carries the Shell ward.
pub fn shell_equipped(finds: Option<&FoundFinds>) -> bool { finds.and_then(FoundFinds::equipped_find).is_some_and(|f| f.ward == Some(shared::temper::WardKind::Shell)) }
/// COMBAT-CORE-2a — key 2 raises Plate only inside the visible tell, once per lunge, never while it is up. True when it armed.
pub fn plate_arm(combat: &mut CombatState, reach: bool, shell: bool) -> bool {
    let arm = shell && tell_visible(combat.lunge_timer, reach) && !combat.plate_armed && combat.plate_secs <= 0.0;
    if arm { (combat.plate_secs, combat.plate_armed) = (PLATE_SECS, true); }
    arm
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
/// RING-COLOR-CONST-1 — neutral colour of the lunge-tell ground ring (combat dots and droid).
pub const LUNGE_TELL_COLOR: Color = Color::srgb(0.86, 0.85, 0.82);

/// LUNGE-TELL-1 — one ground dot of the lunge telegraph.
#[derive(Component)]
pub struct LungeTell;

/// The tell is up for the last `LUNGE_TELL_SECS` before each lunge, only while in reach.
pub fn tell_visible(lunge_timer: f32, reach: bool) -> bool {
    reach && lunge_timer >= LUNGE_SECS - LUNGE_TELL_SECS
}

/// TELL-CADENCE-1 — armed from the first tell onset after entering reach until reach is lost; true once per entry.
pub fn tell_arm(armed: &mut bool, reach: bool, tell: bool) -> bool {
    tell_onset(armed, reach && (*armed || tell))
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

/// Rise fraction (0..=1) while the tell shows: where `fight_present` (Heartwood, Sanctuary), predator down, last `PREDATOR_TELL_SECS`.
pub fn rise_tell(place: PlaceId, predator_taken: u32, down_secs: f32) -> Option<f32> {
    let onset = PREDATOR_RESPAWN_SECS - PREDATOR_TELL_SECS;
    (fight_present(place) && predator_taken >= HEALTH && down_secs >= onset)
        .then(|| ((down_secs - onset) / PREDATOR_TELL_SECS).clamp(0.0, 1.0))
}
/// CLEARED-STATE-1 — Heartwood reads cleared while the predator is down, until the rise tell takes over.
pub fn cleared(place: PlaceId, predator_taken: u32, down_secs: f32) -> bool {
    fight_present(place) && predator_taken >= HEALTH && down_secs < PREDATOR_RESPAWN_SECS - PREDATOR_TELL_SECS
}
#[derive(Component)]
pub struct ClearedTell;

/// The pillar grows up out of the ground: base pinned at y 0, height scales with the rise.
pub fn rise_tell_transform(rise: f32) -> Transform {
    let h = rise.clamp(0.01, 1.0);
    let [x, z] = PREDATOR_HOME;
    Transform::from_xyz(x, RISE_TELL_HEIGHT * h * 0.5, z).with_scale(Vec3::new(1.0, h, 1.0))
}

/// HIT-JUICE-1 — the Predator's base colour (spawn_predator); a flash always restores to it exactly.
pub const PREDATOR_COLOR: Color = Color::srgb(0.81, 0.62, 0.55);
/// HIT-JUICE-1 — neutral warm white the Predator tints toward on a landed hit.
pub const FLASH_COLOR: Color = Color::srgb(1.0, 0.96, 0.9);
/// READ-FIX-SANCTUARY-TELLS-1 — Sanctuary-only body, flash and ring: same hue, lower value, to clear the 3.0 floor on the light Sanctuary fog and sky (council default; a veto reverts to the globals above).
pub const SANCTUARY_PREDATOR_COLOR: Color = Color::srgb(0.13, 0.10, 0.09);
pub const SANCTUARY_FLASH_COLOR: Color = Color::srgb(0.38, 0.36, 0.33);
pub const SANCTUARY_LUNGE_TELL_COLOR: Color = Color::srgb(0.36, 0.35, 0.32);
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

/// READ-FIX-SANCTUARY-TELLS-1 — (body, flash, ring) for `place`: the Sanctuary tints there, the globals elsewhere.
pub fn tell_colors(place: PlaceId) -> (Color, Color, Color) {
    if place == PlaceId::Sanctuary { (SANCTUARY_PREDATOR_COLOR, SANCTUARY_FLASH_COLOR, SANCTUARY_LUNGE_TELL_COLOR) } else { (PREDATOR_COLOR, FLASH_COLOR, LUNGE_TELL_COLOR) }
}

/// READ-FIX-SANCTUARY-TELLS-1 — `flash_color` on the place's base; identical to it off Sanctuary.
pub fn flash_color_at(place: PlaceId, strength: f32) -> Color {
    let (body, flash, _) = tell_colors(place);
    if strength <= 0.0 { body } else { body.mix(&flash, strength.min(1.0)) }
}

/// HIT-FLASH-SHAPE-1 — peak extra Predator scale at flash strength 1 (visual only).
pub const HIT_PULSE_PEAK: f32 = 0.12;
/// HIT-FLASH-SHAPE-1 — Predator scale at flash `strength`; exactly 1.0 at 0, below 0 or NaN.
pub fn hit_pulse_scale(strength: f32) -> f32 {
    if strength > 0.0 {
        1.0 + HIT_PULSE_PEAK * strength.min(1.0)
    } else {
        1.0
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

/// `prompts` = CareStrip or CarePrompt showing (COMBAT_AND_PVP.md:534).
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

/// ABILITY-TOOLTIP-1 — tooltip text for an ability slot. Slot 0 is council wording (Sherif may veto) and stays test-only.
/// ABILITY-TOOLTIP-SHOW-1 A1 — slot 1 (Plate) is COMBAT_AND_PVP.md:456 verbatim, shown on the AbilityTip slab. Slots 2-3 get none.
pub fn slot_tooltip(slot: u32) -> Option<String> {
    if slot == 1 { return Some("Plate, 3 s, incoming damage reduced by 30%.".into()); }
    slot_strikes(Some(slot)).then(|| format!(
        "Strike (left click, right trigger, or key 1 while no panel is open): hits the predator for {HIT} when it is within {REACH} of you."
    ))
}

/// One strike decision per frame: LMB/pad as before, or key 1 while no panel is open.
pub fn strike_pressed(fire: bool, slot: Option<u32>, gate: bool) -> bool {
    fire || (slot_strikes(slot) && gate)
}

/// Heartwood-run gate only; Sanctuary practice uses `fight_present`, not this.
pub fn predator_present(place: PlaceId) -> bool {
    place == PlaceId::Heartwood
}

/// COMBAT-REACH-1 — the fight lives in Heartwood plus the Sanctuary practice fight (no book); homes, shift and clears.
pub fn fight_present(place: PlaceId) -> bool { place == PlaceId::Heartwood || place == PlaceId::Sanctuary }
pub fn fight_home(place: PlaceId) -> [f32; 2] { if place == PlaceId::Sanctuary { STAVEHOLD_FIGHT_HOME } else { PREDATOR_HOME } }
pub fn home_shift(place: PlaceId) -> Vec3 { let ([hx, hz], [px, pz]) = (fight_home(place), PREDATOR_HOME); Vec3::new(hx - px, 0.0, hz - pz) }
pub fn clears_area(place: PlaceId) -> bool { place != PlaceId::Sanctuary } // Sanctuary clears on entry

pub fn in_reach(place: PlaceId, combat: &CombatState, x: f32, z: f32) -> bool {
    let d = Vec2::new(x, z).distance(Vec2::from_array(fight_home(place)));
    fight_present(place) && combat.predator_taken < HEALTH && d <= REACH
}
/// STRIKE-FACING-1 — "aimed at a body in front of you" (COMBAT_AND_PVP.md:435): the target sits inside ±60° of the body's +Z
/// (cos 60° = 0.5). No body yaw, or within 0.1 m of the target, fails open. Only the strike reads it: never in_reach, Plate or the lunge.
pub fn facing_ok(body_fwd: Option<Vec3>, player: Vec3, target: [f32; 2]) -> bool {
    let to = Vec2::new(target[0] - player.x, target[1] - player.z);
    body_fwd.is_none_or(|f| to.length() <= 0.1 || Vec2::new(f.x, f.z).normalize_or_zero().dot(to.normalize()) >= 0.5)
}
/// COMBAT-SIGNPOST-1 — arrival at the fight home, predator up or down (not `in_reach`).
pub fn at_fight_home(place: PlaceId, x: f32, z: f32) -> bool { fight_present(place) && Vec2::new(x, z).distance(Vec2::from_array(fight_home(place))) <= REACH }

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
    land_hit_on(combat, skills, reach, DifficultyTier::Sheltered)
}

/// DIFFICULTY-REWARDS-1 — `land_hit_crossing` on a tier: only the paid XP scales, the cap still counts hits.
pub fn land_hit_on(combat: &mut CombatState, skills: &mut SkillXp, reach: bool, tier: DifficultyTier) -> (bool, Option<u32>) {
    land_hit_paid(combat, skills, reach, tier, None, None)
}
/// GEAR-SLOT-1 — the equipped owned find's added strike from its standing: Worn +0, Kept +1, Proven +2, Storied +3 (ceiling +3).
/// No slot, no `FoundFinds`, an unowned or unknown name all add 0. Reads `temper`/`seated` through `standing()` only; Ward adds 0.
pub fn equipped_strike_bonus(finds: Option<&FoundFinds>) -> u32 {
    match finds.and_then(FoundFinds::equipped_find).map(|f| f.standing()) {
        Some(Standing::Kept) => 1,
        Some(Standing::Proven) => 2,
        Some(Standing::Storied) => 3,
        _ => 0,
    }
}
/// RUN-LEDGER-LIVE-1 — `land_hit_on` with the stored best row; pays through `premium_paid` (off: unchanged). GEAR-SLOT-1 adds the slot bonus.
pub fn land_hit_paid(combat: &mut CombatState, skills: &mut SkillXp, reach: bool, tier: DifficultyTier, row: Option<RunRow>, finds: Option<&FoundFinds>) -> (bool, Option<u32>) {
    if !reach || combat.predator_taken >= HEALTH {
        return (false, None);
    }
    combat.predator_taken = combat.predator_taken.saturating_add(HIT + equipped_strike_bonus(finds)).min(HEALTH);
    if skills.combat_day_hits < crate::skill_xp::COMBAT_PAYING_HITS_PER_DAY {
        skills.combat_day_hits += 1;
        (true, skills.gain(COMBAT, premium_paid(tier, XP_PER_LANDED_HIT, row, SKILL_PREMIUM_LIVE)))
    } else {
        (true, None) // COMBAT-XP-PACE-1: past the daily cap `gain` never runs
    }
}

/// COMBAT-JUMP-DODGE-1 — a due shot connects only when the player is grounded.
pub fn ground_shot_connects(grounded: bool) -> bool {
    grounded
}

/// One lunge per 0.6 s in reach. True when the player is defeated.
/// The clock, tell, and defeat return advance either way; damage is the due wrap only while grounded.
pub fn predator_tick(combat: &mut CombatState, dt: f32, reach: bool, grounded: bool, tier: DifficultyTier) -> bool {
    let plate = combat.plate_secs > 0.0; // COMBAT-CORE-2a: Plate-off (every existing caller) is unchanged
    predator_tick_plated(combat, dt, reach, grounded, tier, plate)
}
/// COMBAT-CORE-2a — `predator_tick` with Plate: a due wrap lands `plate_hit` while `plate`. A wrap or reset ends the lunge's arm.
pub fn predator_tick_plated(combat: &mut CombatState, dt: f32, reach: bool, grounded: bool, tier: DifficultyTier, plate: bool) -> bool {
    combat.lunge_timer = if reach { combat.lunge_timer + dt.max(0.0) } else { 0.0 };
    combat.plate_armed &= reach && combat.lunge_timer < LUNGE_SECS;
    while combat.lunge_timer >= LUNGE_SECS {
        combat.lunge_timer -= LUNGE_SECS;
        if ground_shot_connects(grounded) {
            let hit = enemy_hit(tier, LUNGE_HIT);
            combat.player_taken = combat.player_taken.saturating_add(if plate { plate_hit(hit) } else { hit }).min(HEALTH);
        }
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
    reseat_at(PlaceId::Heartwood, combat, presence);
}

/// COMBAT-REACH-1 — a Sanctuary defeat wakes at the Stavehold gate; Heartwood is unchanged.
pub fn reseat_at(place: PlaceId, combat: &mut CombatState, presence: &mut SoftPresence) {
    let [px, pz] = WATER_POND_CENTER;
    let [x, z] = if place == PlaceId::Sanctuary { STAVEHOLD_GATE_SPOT } else { heartwood_bath_return(PlaceId::Heartwood, px, pz).unwrap_or(HEARTWOOD_ENTRANCE) };
    (presence.position, presence.velocity) = (Vec3::new(x, STAND_HEIGHT, z), Vec3::ZERO);
    presence.grounded = true;
    (combat.player_taken, combat.lunge_timer, combat.plate_secs, combat.plate_armed) = (0, 0.0, 0.0, false);
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

/// COMBAT-REACH-1 — the wake line for the place of the defeat.
pub fn wake_cue_at(place: PlaceId) -> String { if place == PlaceId::Sanctuary { "You wake at the Stavehold gate".to_string() } else { wake_cue() } }
/// HURT-READOUT-1 — the player's health on the shared Pulse line, e.g. "You 185 / 200". Council default copy; Sherif may veto.
pub fn hurt_line(player_taken: u32) -> String { format!("You {} / {HEALTH}", HEALTH.saturating_sub(player_taken)) }
/// True only for `hurt_line` output; the wake line ("You wake at …") never matches.
pub fn is_hurt_line(line: &str) -> bool { line.strip_prefix("You ").and_then(|r| r.split_once(" / ")).and_then(|(n, _)| n.parse::<u32>().ok()).is_some_and(|n| n <= HEALTH && hurt_line(HEALTH - n) == line) }
/// The hurt line, only when the Pulse slot is free (idle, or already a hurt line) and never on `hides_cues` tiers.
pub fn hurt_pulse(tier: DifficultyTier, line: &str, until: f64, now: f64, player_taken: u32) -> Option<String> { (!hides_cues(tier) && (until <= now || is_hurt_line(line))).then(|| hurt_line(player_taken)) }

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
            .init_resource::<FoundFinds>()
            .add_systems(Startup, (load_finds, check_music))
            .add_systems(Update, (show_find, music_bed).after(predator_lunge))
            .add_systems(Startup, spawn_predator)
            .add_systems(Startup, spawn_lunge_tell)
            .add_systems(Startup, (spawn_rise_tell, spawn_cleared_tell, spawn_ability_tip))
            .add_systems(Update, ability_tip_show.after(combat_fire))
            .add_systems(Update, (rise_tell_show, cleared_show).after(predator_respawn))
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
        base_color: LUNGE_TELL_COLOR,
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

/// CLEARED-STATE-1 — one hidden steady disc at `PREDATOR_HOME`, unlit Heartwood amber, no animation.
fn spawn_cleared_tell(mut commands: Commands, (mut meshes, mut materials): (ResMut<Assets<Mesh>>, ResMut<Assets<StandardMaterial>>)) {
    commands.spawn((
        (Mesh3d(meshes.add(Cylinder::new(RISE_TELL_RADIUS, LUNGE_TELL_DOT_HEIGHT))), MeshMaterial3d(materials.add(StandardMaterial { base_color: crate::climate_plane::HEARTWOOD_AMBER_LAMP, unlit: true, ..default() }))),
        (Transform::from_xyz(PREDATOR_HOME[0], LUNGE_TELL_LIFT, PREDATOR_HOME[1]), Visibility::Hidden, ClearedTell),
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
    (buffer, fixed, diff, runs, mut cmd, assets, mute, mut finds, body): (
        Option<ResMut<StrikeBuffer>>,
        Option<Res<Time<Fixed>>>,
        Option<ResMut<crate::difficulty::AreaDifficulty>>,
        Option<Res<RunLedger>>,
        Commands,
        Option<Res<AssetServer>>,
        Option<Res<crate::local_settings::MasterMuteGain>>,
        Option<ResMut<FoundFinds>>,
        Query<&Transform, With<crate::human_presence::HumanPresence>>,
    ),
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
    let plate_key = gate && input.as_deref().and_then(|i| i.ability_slot) == Some(1); // COMBAT-CORE-2a: key 2, digit path only
    let pressed = strike_pressed(fire, input.and_then(|i| i.ability_slot), gate);
    let pulse = buffer.as_ref().is_some_and(|b| b.fire);
    let pulse = pulse || plate_key; // key 2 also wakes the fire path; the strike below re-reads the buffer
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
    if plate_key && allowed { plate_arm(&mut combat, reach, shell_equipped(finds.as_deref())); }
    let reach = reach && facing_ok(body.iter().next().map(|t| t.rotation * Vec3::Z), presence.position, fight_home(travel.current)); // STRIKE-FACING-1 A1
    if !(pressed || buffer.as_ref().is_some_and(|b| b.fire)) { return; }
    // COMBAT-FEEL-1: the hit lands here in Update; only expiry and reach run on the fixed tick.
    let now_fixed = fixed.map_or(0.0, |t| t.elapsed_secs_f64());
    let run = match buffer {
        Some(mut buf) => strike_buffer_step(&mut buf, &[], (pressed, allowed, gate, reach), now_fixed),
        None => pressed && allowed,
    };
    let tier = diff.as_deref().map_or(DifficultyTier::Sheltered, |d| d.tier(travel.current));
    if run {
        combat.strikes = combat.strikes.wrapping_add(1);
        let before = skills.xp[COMBAT];
        let row = runs.as_deref().and_then(|r| r.row(travel.current, tier));
        let pred_before = combat.predator_taken;
        let (hit, level) = land_hit_paid(&mut combat, &mut skills, reach, tier, row, finds.as_deref());
        crate::combat_text::hook_after_fire(&mut cmd, travel.current, pred_before, combat.predator_taken, hit, presence.position);
        play_hit(&mut cmd, assets.as_deref(), mute.as_deref(), hit); // COMBAT-FEEL-SFX-1: a landed hit only
        let down = hit && combat.predator_taken >= HEALTH;
        let defeat = down && clears_area(travel.current);
        if let (true, Some(mut diff)) = (defeat, diff) {
            diff.record_clear(travel.current); // DIFFICULTY-TIERS-1: the predator defeat clears this area
        }
        if let (true, Some(f)) = (defeat, finds.as_mut()) {
            if f.drop_once(tier, combat.strikes, travel.current as u32) { save_finds(f); } // LOOT-DROPS-WIRE-1: saved when a find lands
        }
        if let (true, false, Some(f)) = (down, clears_area(travel.current), finds.as_mut()) { if f.first_find(combat.strikes, travel.current as u32) { save_finds(f); } }
        let gained = skills.xp[COMBAT].saturating_sub(before);
        if let (Some(line), Some(epi)) = (fire_line(travel.current, &combat, hit, gained, level), epi.as_mut()) {
            let line = cue_line(tier, line, HEALTH.saturating_sub(combat.predator_taken), HEALTH);
            set_pulse(epi, line, now + CUE_SECS);
        }
    }
}

/// LOOT-DROPS-WIRE-1 — a waiting find line shows once the calm queue is done and the current pulse has expired.
fn show_find(time: Res<Time>, finds: Option<ResMut<FoundFinds>>, mut epi: Option<ResMut<FirstHarvestEpiphany>>, calm: Option<Res<CalmQueue>>, mut cmd: Commands, assets: Option<Res<AssetServer>>, mute: Option<Res<crate::local_settings::MasterMuteGain>>) {
    let (Some(mut finds), Some(epi), now) = (finds, epi.as_mut(), time.elapsed_secs_f64()) else { return };
    if finds.pending.is_some() && find_ready(calm.is_some_and(|c| c.done(now)), epi.pulse_until, now) {
        if let Some(line) = finds.pending.take() { set_pulse(epi, line, now + CUE_SECS); play_find(&mut cmd, assets.as_deref(), mute.as_deref()); }
    }
}

/// COMBAT-FEEL-1 — FixedUpdate hold: expiry and reach only. Never writes CombatState, SkillXp or the pulse line.
fn hold_strike_buffer(
    time: Option<Res<Time>>,
    buffer: Option<ResMut<StrikeBuffer>>,
    (travel, presence, combat): (Option<Res<HexTravelState>>, Option<Res<SoftPresence>>, Option<Res<CombatState>>),
    body: Query<&Transform, With<crate::human_presence::HumanPresence>>,
) {
    let (Some(time), Some(mut buf)) = (time, buffer.filter(|b| b.until.is_some())) else {
        return;
    };
    let reach = match (travel, presence, combat) {
        (Some(t), Some(p), Some(c)) => in_reach(t.current, &c, p.position.x, p.position.z) && facing_ok(body.iter().next().map(|b| b.rotation * Vec3::Z), p.position, fight_home(t.current)),
        _ => false,
    };
    strike_buffer_tick(&mut buf, time.elapsed_secs_f64(), reach);
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn predator_lunge(
    time: Res<Time>,
    door: Option<Res<LaunchDoor>>,
    travel: Option<Res<HexTravelState>>,
    presence: Option<ResMut<SoftPresence>>,
    mut combat: ResMut<CombatState>,
    mut epi: Option<ResMut<FirstHarvestEpiphany>>,
    mut shown: Query<&mut Visibility, With<Predator>>,
    mut diff: Option<ResMut<AreaDifficulty>>,
    (mut seated, mut homed): (Local<Vec3>, Query<(&mut Transform, Has<Predator>), Or<(With<Predator>, With<LungeTell>, With<ClearedTell>)>>),
    (mut cmd, assets, mute, mut mats, tinted): (Commands, Option<Res<AssetServer>>, Option<Res<crate::local_settings::MasterMuteGain>>, Option<ResMut<Assets<StandardMaterial>>>, Query<(&MeshMaterial3d<StandardMaterial>, Has<Predator>, Has<RiseTell>), Or<(With<Predator>, With<LungeTell>, With<RiseTell>)>>),
) {
    let (Some(travel), Some(mut presence)) = (travel, presence) else {
        return;
    };
    let tier = diff.as_deref().map_or(DifficultyTier::Sheltered, |d| d.tier(travel.current));
    let show = combat.predator_taken < HEALTH && fight_present(travel.current);
    for mut vis in &mut shown {
        vis.set_if_neq(if show { Visibility::Visible } else { Visibility::Hidden });
    }
    // COMBAT-REACH-1 D4: re-seat the predator, ring and disc once per home change (Heartwood: zero shift).
    let shift = home_shift(travel.current);
    if shift != *seated {
        let delta = shift - *seated;
        homed.iter_mut().for_each(|(mut t, _)| t.translation += delta);
        *seated = shift;
        let (body, _, ring) = tell_colors(travel.current); // READ-FIX-SANCTUARY-TELLS-1 (CL-R2): tint only; never gates the re-seat
        for (m, pred, rise) in &tinted { if let Some(mut mat) = mats.as_mut().and_then(|a| a.get_mut(&m.0)) { let want = if pred || rise { body } else { ring }; if mat.base_color != want { mat.base_color = want; } } }
    }
    let (in_yard, p) = (door.as_deref().copied() == Some(LaunchDoor::InYard), presence.position);
    let reach = in_yard && in_reach(travel.current, &combat, p.x, p.z);
    let grounded = presence.grounded;
    let before = combat.player_taken; // CL-C1: one edge per frame, read before the reseat
    let t0 = combat.lunge_timer;
    let defeated = predator_tick(&mut combat, time.delta_secs(), reach, grounded, tier);
    combat.plate_secs = (combat.plate_secs - time.delta_secs()).max(0.0);
    crate::combat_text::hook_after_lunge(&mut cmd, t0, before, time.delta_secs(), reach, grounded, combat.player_taken, presence.position);
    if hurt_edge(before, combat.player_taken) {
        combat.hurt = combat.hurt.wrapping_add(1);
        if !tell_visible(combat.lunge_timer, reach) { play_hurt(&mut cmd, assets.as_deref(), mute.as_deref()); } // P4: never on a tell frame
        if let (Some(epi), now) = (epi.as_mut(), time.elapsed_secs_f64()) {
            if let Some(line) = hurt_pulse(tier, &epi.pulse_line, epi.pulse_until, now, combat.player_taken) { set_pulse(epi, line, now + CUE_SECS); }
        }
    }
    let ([hx, hz], lean) = (fight_home(travel.current), lean_at(combat.lunge_timer, reach));
    let to = Vec2::new(p.x - hx, p.z - hz).normalize_or_zero() * lean + Vec2::new(hx, hz);
    for (mut t, _) in homed.iter_mut().filter(|(_, predator)| *predator) {
        if (t.translation.x, t.translation.z) != (to.x, to.y) { (t.translation.x, t.translation.z) = (to.x, to.y); } // CL-C3: x/z only, absolute
    }
    if defeated {
        reseat_at(travel.current, &mut combat, &mut presence);
        if let Some(d) = diff.as_mut() { d.new_life(); } // DIFFICULTY-REWARDS-1: the defeat ends this life
        if let Some(epi) = epi.as_mut() {
            set_pulse(epi, wake_cue_at(travel.current), time.elapsed_secs_f64() + CUE_SECS);
        }
    }
}

/// DIFFICULTY-CUES-1 — the wind-up sound, the same sting spawn as peace_audio.rs, behind `audio_output_safe`.
/// Silent while the shared Mute flag is on or the gain is 0, like peace_audio.rs:112.
pub fn play_tell(cmd: &mut Commands, assets: Option<&AssetServer>, mute: Option<&crate::local_settings::MasterMuteGain>) {
    let gain = mute.map_or(1.0, |m| if m.muted { 0.0 } else { m.gain });
    if let (true, true, Some(assets)) = (audio_output_safe(), gain > 0.0, assets) {
        let play = PlaybackSettings::DESPAWN.with_volume(bevy::audio::Volume::Linear(gain));
        cmd.spawn((AudioPlayer::<AudioSource>(assets.load(STING_ASSET)), play));
    }
}

/// COMBAT-FEEL-SFX-1 — the hit volume: None on a miss, muted or zero gain; else the tell's gain x `HIT_GAIN`.
pub fn hit_volume(hit: bool, mute: Option<&crate::local_settings::MasterMuteGain>) -> Option<f32> {
    let gain = mute.map_or(1.0, |m| if m.muted { 0.0 } else { m.gain }) * HIT_GAIN;
    (hit && gain > 0.0).then_some(gain)
}

/// COMBAT-FEEL-SFX-1 — the same spawn as `play_tell`, behind `audio_output_safe`, for a landed hit only.
pub fn play_hit(cmd: &mut Commands, assets: Option<&AssetServer>, mute: Option<&crate::local_settings::MasterMuteGain>, hit: bool) {
    if let (true, Some(gain), Some(assets)) = (audio_output_safe(), hit_volume(hit, mute), assets) {
        let play = PlaybackSettings::DESPAWN.with_volume(bevy::audio::Volume::Linear(gain));
        cmd.spawn((AudioPlayer::<AudioSource>(assets.load(HIT_ASSET)), play));
    }
}

pub fn find_volume(mute: Option<&crate::local_settings::MasterMuteGain>) -> Option<f32> {
    let gain = mute.map_or(1.0, |m| if m.muted { 0.0 } else { m.gain }) * FIND_GAIN;
    (gain > 0.0).then_some(gain)
}

pub fn play_find(cmd: &mut Commands, assets: Option<&AssetServer>, mute: Option<&crate::local_settings::MasterMuteGain>) {
    if let (true, Some(gain), Some(assets)) = (audio_output_safe(), find_volume(mute), assets) {
        cmd.spawn((AudioPlayer::<AudioSource>(assets.load(FIND_ASSET)), PlaybackSettings::DESPAWN.with_volume(bevy::audio::Volume::Linear(gain))));
    }
}

/// COMBAT-FEEL-LOOK-1 — the player's hurt thud: `combat_hit.ogg` played lower, so being hit and hitting differ.
pub const HURT_SPEED: f32 = 0.8;
const _: () = assert!(HURT_SPEED < 1.0);
/// One hurt per frame with a `player_taken` rise.
pub fn hurt_edge(before: u32, after: u32) -> bool { after > before }
/// The same guard and gain path as `play_hit` (gain from `hit_volume`, so at most `HIT_GAIN`), slower playback.
pub fn play_hurt(cmd: &mut Commands, assets: Option<&AssetServer>, mute: Option<&crate::local_settings::MasterMuteGain>) {
    if let (true, Some(gain), Some(assets)) = (audio_output_safe(), hit_volume(true, mute), assets) {
        let play = PlaybackSettings::DESPAWN.with_volume(bevy::audio::Volume::Linear(gain)).with_speed(HURT_SPEED);
        cmd.spawn((AudioPlayer::<AudioSource>(assets.load(HIT_ASSET)), play));
    }
}
/// The creature leans toward the player over the existing wind-up; 0 outside the tell, back to 0 on the wrap.
pub const LUNGE_LEAN: f32 = 0.4;
const _: () = assert!(LUNGE_LEAN <= 0.45);
pub fn lean_at(lunge_timer: f32, reach: bool) -> f32 {
    if !tell_visible(lunge_timer, reach) { return 0.0; }
    let s = ((lunge_timer - (LUNGE_SECS - LUNGE_TELL_SECS)) / LUNGE_TELL_SECS).clamp(0.0, 1.0);
    LUNGE_LEAN * s * s * (3.0 - 2.0 * s)
}

/// MUSIC-WIRE-1 — White Bat Audio "After The War" (MUSIC_CREDITS[12]); the file is gitignored and never committed.
pub const MUSIC_ASSET: &str = "audio/music/after-the-war.ogg";
/// Ducked bed: x MasterMuteGain like the tell, and below `HIT_GAIN` (CL-M2).
pub const MUSIC_GAIN: f32 = 0.04;
const _: () = assert!(MUSIC_GAIN < HIT_GAIN);
/// CL-S2 — the fade-out starts at 27.0 s, so the level is 0 when the rise tell starts at 28.5 s.
pub const MUSIC_FADE_AT: f32 = PREDATOR_RESPAWN_SECS - 2.0 * PREDATOR_TELL_SECS;
/// Set once at Startup from `music_file_present`; false means `load` is never called (CL-M1).
#[derive(Resource, Debug, Default, Clone, Copy, PartialEq)]
pub struct MusicPresent(pub bool);
#[derive(Component)]
pub struct MusicBed;
/// D3 — checked under Bevy's asset root, the same root the loader reads.
pub fn music_file_present() -> bool { bevy::asset::io::file::FileAssetReader::get_base_path().join("assets").join(MUSIC_ASSET).is_file() }
fn check_music(mut cmd: Commands) { cmd.insert_resource(MusicPresent(music_file_present())); }
/// CL-S1 — armed only on a Sanctuary defeat edge; a place change drops it and hushes for `HOLD_SECS` (D1, mirrors climate_visible.rs:361-364).
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct MusicLatch { pub armed: bool, pub was_down: bool, pub place: Option<PlaceId>, pub hush_until: f64, pub level: f32 }
pub fn music_latch(l: &mut MusicLatch, place: PlaceId, down: bool, now: f64) {
    if l.place != Some(place) { (l.place, l.armed, l.hush_until) = (Some(place), false, now + crate::first_whisper::HOLD_SECS); }
    l.armed = down && (l.armed || (!l.was_down && place == PlaceId::Sanctuary));
    l.was_down = down;
}
pub fn music_target(present: bool, l: &MusicLatch, combat: &CombatState, now: f64, gain: f32) -> f32 {
    let calm = l.armed && l.place == Some(PlaceId::Sanctuary) && cleared(PlaceId::Sanctuary, combat.predator_taken, combat.down_secs);
    if present && calm && now >= l.hush_until && combat.down_secs < MUSIC_FADE_AT { MUSIC_GAIN * gain } else { 0.0 }
}
/// D5 — fades only: the level moves toward the target by `MUSIC_GAIN` per `PREDATOR_TELL_SECS`.
pub fn music_ramp(level: f32, target: f32, dt: f32) -> f32 {
    let step = MUSIC_GAIN * dt / PREDATOR_TELL_SECS;
    if level < target { (level + step).min(target) } else { (level - step).max(target) }
}
/// CL-S3 — the same `audio_output_safe` and gain guards as `play_tell`; one looping bed, despawned at 0.
fn music_bed(
    (mut cmd, assets, time, mut latch): (Commands, Option<Res<AssetServer>>, Res<Time>, Local<MusicLatch>),
    (present, combat, travel, mute): (Option<Res<MusicPresent>>, Res<CombatState>, Option<Res<HexTravelState>>, Option<Res<crate::local_settings::MasterMuteGain>>),
    mut beds: Query<(Entity, Option<&mut bevy::audio::AudioSink>), With<MusicBed>>,
) {
    let (Some(travel), now) = (travel, time.elapsed_secs_f64()) else { return };
    music_latch(&mut latch, travel.current, combat.predator_taken >= HEALTH, now);
    let gain = mute.map_or(1.0, |m| if m.muted { 0.0 } else { m.gain });
    let present = present.is_some_and(|p| p.0) && audio_output_safe();
    latch.level = music_ramp(latch.level, music_target(present, &latch, &combat, now, gain), time.delta_secs());
    if latch.level <= 0.0 { return beds.iter().for_each(|(e, _)| { cmd.entity(e).despawn(); }); }
    let mut any = false;
    for (_, sink) in &mut beds { any = true; if let Some(mut s) = sink { s.set_volume(bevy::audio::Volume::Linear(latch.level)); } }
    if let (false, Some(assets)) = (any, assets) {
        let play = PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::Linear(latch.level));
        cmd.spawn((AudioPlayer::<AudioSource>(assets.load(MUSIC_ASSET)), play, MusicBed));
    }
}

pub const LUNGE_TELL_TEXT: &str = "Brace, it is about to lunge";

/// LUNGE-TELL-1 — chained after `predator_lunge`, so the timer and reach are post-tick.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn lunge_tell(
    (mut cmd, assets, time, mut armed): (Commands, Option<Res<AssetServer>>, Res<Time>, Local<bool>),
    (diff, mut epi, mute): (Option<Res<AreaDifficulty>>, Option<ResMut<FirstHarvestEpiphany>>, Option<Res<crate::local_settings::MasterMuteGain>>),
    door: Option<Res<LaunchDoor>>,
    travel: Option<Res<HexTravelState>>,
    presence: Option<Res<SoftPresence>>,
    combat: Option<Res<CombatState>>,
    mut dots: Query<&mut Visibility, With<LungeTell>>,
    mut settings: Option<ResMut<crate::local_settings::LocalSettingsState>>,
) {
    let (Some(travel), Some(presence), Some(combat)) = (travel, presence, combat) else {
        for mut vis in &mut dots {
            vis.set_if_neq(Visibility::Hidden);
        }
        *armed = false;
        return;
    };
    // The exact reach term of predator_lunge, combat_core.rs:634-635. No second formula.
    let (in_yard, p) = (door.as_deref().copied() == Some(LaunchDoor::InYard), presence.position);
    let reach = in_yard && in_reach(travel.current, &combat, p.x, p.z);
    let (tell, tier) = (tell_visible(combat.lunge_timer, reach), diff.map_or(DifficultyTier::Sheltered, |d| d.tier(travel.current)));
    let tip_edge = !*armed && reach && tell;
    if tell_arm(&mut armed, reach, tell) {
        play_tell(&mut cmd, assets.as_deref(), mute.as_deref());
        if let (true, Some(epi)) = (hides_cues(tier), epi.as_mut()) {
            set_pulse(epi, LUNGE_TELL_TEXT.to_string(), time.elapsed_secs_f64() + f64::from(LUNGE_TELL_SECS));
        }
    }
    if tip_edge { crate::combat_text::hook_tip_edge(&mut cmd, settings.as_deref_mut(), presence.position, PREDATOR_NAME); }
    for mut vis in &mut dots {
        vis.set_if_neq(if *armed && !hides_cues(tier) {
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
    (mut last, mut since, mut hurt_seen): (Local<u32>, Local<Option<f64>>, Local<u32>),
    combat: Option<Res<CombatState>>,
    pool: Option<ResMut<SoftRbePool>>,
    materials: Option<ResMut<Assets<StandardMaterial>>>,
    mut predator: Query<
        (&MeshMaterial3d<StandardMaterial>, Option<&mut Transform>),
        With<Predator>,
    >,
    gamepads: Query<Entity, With<Gamepad>>,
    mut rumble: MessageWriter<GamepadRumbleRequest>,
    travel: Option<Res<HexTravelState>>,
) {
    let (Some(combat), Some(mut pool), Some(mut materials)) = (combat, pool, materials) else {
        return;
    };
    if combat.hurt != std::mem::replace(&mut *hurt_seen, combat.hurt) { // COMBAT-FEEL-LOOK-1 (CL-C2): the player was hit
        pool.kick = pool.kick.max(HIT_KICK);
        rumble_harvest(&mut rumble, gamepads.iter(), false);
    }
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
    for (handle, transform) in &mut predator {
        if let Some(mut mat) = materials.get_mut(&handle.0) {
            mat.base_color = flash_color_at(travel.as_deref().map_or(PlaceId::Heartwood, |t| t.current), strength);
        }
        if let Some(mut transform) = transform {
            transform.scale = Vec3::splat(hit_pulse_scale(strength));
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
    let shift = travel.as_deref().map_or(Vec3::ZERO, |t| home_shift(t.current));
    let rise = match (travel, combat) {
        (Some(t), Some(c)) => rise_tell(t.current, c.predator_taken, c.down_secs),
        _ => None,
    };
    for (mut vis, mut at) in &mut pillar {
        vis.set_if_neq(if rise.is_some() { Visibility::Visible } else { Visibility::Hidden });
        if let Some(r) = rise {
            let mut t = rise_tell_transform(r);
            t.translation += shift;
            at.set_if_neq(t);
        }
    }
}

/// CLEARED-STATE-1 — after `predator_respawn`, like `rise_tell_show`; toggles Visibility only.
fn cleared_show(travel: Option<Res<HexTravelState>>, combat: Option<Res<CombatState>>, mut disc: Query<&mut Visibility, With<ClearedTell>>) {
    let on = matches!((travel, combat), (Some(t), Some(c)) if cleared(t.current, c.predator_taken, c.down_secs));
    disc.iter_mut().for_each(|mut vis| { vis.set_if_neq(if on { Visibility::Visible } else { Visibility::Hidden }); });
}

/// ABILITY-TOOLTIP-SHOW-1 — the AbilityTip slab (registry ACTION_BAR occupant, rank 5, class 3, 240 px) holding slot 1's text,
/// in the Practice strip's existing plate style. Spawns Hidden; the active preset places it and applies R5/R2/§6.3.
fn spawn_ability_tip(mut commands: Commands) {
    use crate::{hud_anchor_registry::{HudSlab, ACTION_BAR, ID_ABILITY_TIP}, ui_kit::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY}};
    let node = Node { position_type: PositionType::Absolute, bottom: ACTION_BAR.bottom(), right: ACTION_BAR.right(), width: Val::Px(240.0), padding: UiRect::axes(Val::Px(18.0), Val::Px(12.0)), justify_content: JustifyContent::Center, align_items: AlignItems::Center, border: UiRect::all(Val::Px(1.5)), ..default() };
    commands.spawn((node, BackgroundColor(TITLE_PLATE_BG), crate::ui_frame::plate_frame(), BorderColor::all(TITLE_BORDER), Visibility::Hidden, HudSlab(ID_ABILITY_TIP))).with_children(|tip| {
        tip.spawn((Text::new(slot_tooltip(1).unwrap_or_default()), TextFont { font_size: FontSize::Px(15.5 / 1.2), ..default() }, TextColor(TITLE_TEXT_PRIMARY)));
    });
}
/// ABILITY-TOOLTIP-SHOW-1 A2 (T1 + Shell gate) — the tip's own Visibility, written outright each frame after `combat_fire`:
/// Visible only with the Shell ward equipped and the fight in reach at this place, so it never offers Plate when Plate can't arm.
fn ability_tip_show(combat: Res<CombatState>, finds: Option<Res<FoundFinds>>, (travel, presence): (Option<Res<HexTravelState>>, Option<Res<SoftPresence>>), mut slabs: Query<(&crate::hud_anchor_registry::HudSlab, &mut Visibility)>) {
    let show = shell_equipped(finds.as_deref()) && travel.zip(presence).is_some_and(|(t, p)| in_reach(t.current, &combat, p.position.x, p.position.z));
    for (_, mut vis) in slabs.iter_mut().filter(|(s, _)| s.0 == crate::hud_anchor_registry::ID_ABILITY_TIP) { *vis = if show { Visibility::Visible } else { Visibility::Hidden }; }
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
    fn slot_tooltip_tracks_hit_reach_and_strike_slots() {
        let text = slot_tooltip(0).expect("slot 0 strikes");
        assert!(text.contains(&HIT.to_string()) && text.contains(&REACH.to_string()), "{text}");
        assert!(text.contains("key 1 while no panel is open"), "{text}");
        for s in 0..=4 {
            assert_eq!(slot_tooltip(s).is_some(), s <= 1, "slot {s}"); // ABILITY-TOOLTIP-SHOW-1 A3: slot 1 carries the Plate line
        }
        let lower = text.to_lowercase();
        for banned in ["class", "soon", "coming", "unlock", "cooldown"] {
            assert!(!lower.contains(banned), "{banned}");
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
    fn loot_drops_wire_drop_seam_once_per_defeat() {
        let (mut c, mut s) = (CombatState::default(), SkillXp::default());
        let defeat = |c: &mut CombatState, s: &mut SkillXp| land_hit(c, s, true) && c.predator_taken >= HEALTH && clears_area(PlaceId::Heartwood);
        assert!(!defeat(&mut c, &mut s), "a plain hit is no defeat");
        c.predator_taken = HEALTH - HIT;
        assert!(defeat(&mut c, &mut s) && !defeat(&mut c, &mut s), "once per kill, never after");
        assert!(!clears_area(PlaceId::Sanctuary));
    }

    #[test]
    fn music_wire_slot_latch_hush_and_fade() {
        let (mut l, mut c) = (MusicLatch::default(), CombatState { predator_taken: HEALTH, ..default() });
        (music_latch(&mut l, PlaceId::Heartwood, true, 0.0), music_latch(&mut l, PlaceId::Sanctuary, true, 1.0));
        assert!(!l.armed && music_target(true, &l, &c, 10.0, 1.0) == 0.0, "a Heartwood defeat never arms Sanctuary");
        let mut l = MusicLatch::default();
        (music_latch(&mut l, PlaceId::Sanctuary, false, 0.0), music_latch(&mut l, PlaceId::Sanctuary, true, 1.0));
        assert!(l.armed && music_target(true, &l, &c, 3.0, 1.0) == 0.0, "hush for HOLD_SECS after entry");
        assert!(music_target(true, &l, &c, 6.0, 1.0) == MUSIC_GAIN && music_target(true, &l, &c, 6.0, 0.5) == MUSIC_GAIN * 0.5);
        assert!(music_target(false, &l, &c, 6.0, 1.0) == 0.0 && music_target(true, &l, &c, 6.0, 0.0) == 0.0, "absent or muted");
        c.down_secs = MUSIC_FADE_AT;
        assert!(music_target(true, &l, &c, 6.0, 1.0) == 0.0 && rise_tell(PlaceId::Sanctuary, HEALTH, MUSIC_FADE_AT).is_none());
        let mut level = MUSIC_GAIN;
        for _ in 0..90 { level = music_ramp(level, 0.0, PREDATOR_TELL_SECS / 90.0); }
        assert!(level <= 1e-6 && MUSIC_FADE_AT + PREDATOR_TELL_SECS <= PREDATOR_RESPAWN_SECS - PREDATOR_TELL_SECS, "0 at rise onset");
        (c.down_secs, c.predator_taken) = (1.0, 0);
        assert_eq!(music_target(true, &l, &c, 6.0, 1.0), 0.0, "never while the predator stands");
        (music_latch(&mut l, PlaceId::Heartwood, true, 7.0), music_latch(&mut l, PlaceId::Sanctuary, true, 8.0));
        assert!(!l.armed, "a place change drops the latch");
    }

    #[test]
    fn music_wire_absent_file_spawns_nothing_and_stings_stay_apart() {
        use bevy::ecs::system::RunSystemOnce;
        let mut world = World::new();
        world.insert_resource(Time::<()>::default());
        world.insert_resource(HexTravelState { current: PlaceId::Sanctuary });
        world.insert_resource(CombatState { predator_taken: HEALTH, ..default() });
        world.insert_resource(MusicPresent(false));
        for _ in 0..3 { world.run_system_once(music_bed).unwrap(); }
        assert_eq!(world.query::<&AudioPlayer>().iter(&world).count() + world.query::<&MusicBed>().iter(&world).count(), 0);
        let body = include_str!("combat_core.rs").split("fn music_bed(").nth(1).and_then(|s| s.split("\n}\n").next()).expect("music_bed");
        assert!(body.contains(concat!("assets.load(", "MUSIC_ASSET)")) && !body.contains(concat!("STING", "_ASSET")) && !body.contains(concat!("HIT", "_ASSET")));
        assert_eq!(MUSIC_ASSET, "audio/music/after-the-war.ogg");
    }

    #[test]
    fn combat_feel_look_kick_decays_to_zero() {
        let mut pool = SoftRbePool::default();
        pool.kick = pool.kick.max(HIT_KICK);
        let k: Vec<f32> = (0..4).map(|_| { pool.tick_juice(0.125); pool.kick }).collect();
        assert!(k.windows(2).all(|w| w[1] <= w[0]) && pool.kick == 0.0, "the kick decays to 0");
    }

    #[test]
    fn combat_feel_look_damage_and_timing_unchanged() {
        assert!(HEALTH == 200 && HIT == 8 && LUNGE_HIT == 15 && LUNGE_SECS == 0.6 && LUNGE_TELL_SECS == 0.35 && REACH == HARVEST_REACH && HIT_KICK == 0.35);
        assert!(HURT_SPEED != 1.0 && hit_volume(true, None) == Some(HIT_GAIN));
    }

    /// TELL-CADENCE-1 — one Sheltered frame at an exact dt: (sting this frame, lean).
    fn cadence(c: &mut CombatState, armed: &mut bool, reach: bool, dt: f32) -> (bool, f32) {
        predator_tick(c, dt, reach, true, DifficultyTier::Sheltered);
        (tell_arm(armed, reach, tell_visible(c.lunge_timer, reach)), lean_at(c.lunge_timer, reach))
    }
    #[test]
    fn tell_cadence_sting_once_per_entry() {
        let (mut c, mut armed) = (CombatState::default(), false);
        assert!((0..15).filter(|_| cadence(&mut c, &mut armed, true, 0.125).0).count() == 1 && !cadence(&mut c, &mut armed, true, 0.0).0 && c.player_taken == 3 * LUNGE_HIT, "3 wraps, one sting");
    }
    #[test]
    fn tell_cadence_rearms_on_exit() {
        let (mut c, mut armed) = (CombatState::default(), false);
        (0..15).for_each(|_| { cadence(&mut c, &mut armed, true, 0.125); });
        assert!(!cadence(&mut c, &mut armed, false, 0.125).0 && !armed, "leaving reach hides the ring and re-arms");
        assert_eq!((0..8).filter(|_| cadence(&mut c, &mut armed, true, 0.125).0).count(), 1, "one sting on re-entry");
    }
    #[test]
    fn tell_cadence_lean_every_lunge() {
        let (mut c, mut armed, mut per) = (CombatState::default(), false, vec![0.0f32]);
        for _ in 0..15 {
            let (before, lean) = (c.lunge_timer, cadence(&mut c, &mut armed, true, 0.125).1);
            if c.lunge_timer < before { per.push(0.0); }
            if let Some(last) = per.last_mut() { *last = last.max(lean); }
        }
        assert!(per[..3].iter().all(|l| *l > 0.0 && *l <= LUNGE_LEAN), "{per:?}");
    }
    #[test]
    fn tell_cadence_ring_steady_no_blink() {
        let (mut c, mut armed) = (CombatState::default(), false);
        while !cadence(&mut c, &mut armed, true, 0.125).0 {}
        assert!((0..12).all(|_| { cadence(&mut c, &mut armed, true, 0.125); armed }), "steady across the wraps");
        assert!(!(armed && !hides_cues(DifficultyTier::Sworn)) && tell_visible(0.375, true) && !tell_visible(0.125, true));
    }

    #[test]
    fn combat_feel_look_lean_only_inside_the_wind_up() {
        assert!(lean_at(0.125, true) == 0.0 && lean_at(0.25, true) == 0.0 && lean_at(0.5, false) == 0.0 && lean_at(0.0, true) == 0.0);
        let (a, b) = (lean_at(0.375, true), lean_at(0.5, true));
        assert!(0.0 < a && a < b && b <= LUNGE_LEAN && lean_at(0.5625, true) <= LUNGE_LEAN);
        assert!(tell_visible(0.375, true) && !tell_visible(0.125, true), "the tell window is unchanged");
    }

    #[test]
    fn combat_feel_look_hurt_edge_once_per_rise() {
        let mut c = CombatState::default();
        let step = |c: &mut CombatState, dt: f32| { let before = c.player_taken; predator_tick(c, dt, true, true, DifficultyTier::Sheltered); hurt_edge(before, c.player_taken) };
        assert!((0..4).all(|_| !step(&mut c, 0.125)), "no edge before the strike");
        assert!(step(&mut c, 0.125) && !step(&mut c, 0.0), "once on the wrap, not on an unchanged frame");
        assert!(step(&mut c, 1.25) && c.player_taken == 3 * enemy_hit(DifficultyTier::Sheltered, LUNGE_HIT), "two wraps, one edge");
    }

    fn clock_after(start: f32, dt: f32, reach: bool) -> (f32, u32) {
        let mut timer = if reach { start + dt.max(0.0) } else { 0.0 };
        let mut wraps = 0u32;
        while timer >= LUNGE_SECS {
            timer -= LUNGE_SECS;
            wraps += 1;
        }
        (timer, wraps)
    }
    fn due_taken(start: u32, wraps: u32, tier: DifficultyTier) -> u32 {
        let mut taken = start;
        for _ in 0..wraps {
            taken = taken.saturating_add(enemy_hit(tier, LUNGE_HIT)).min(HEALTH);
        }
        taken
    }

    #[test]
    fn jump_dodge_ground_shot_connects_true_and_false() {
        assert!(ground_shot_connects(true));
        assert!(!ground_shot_connects(false));
    }

    #[test]
    fn jump_dodge_airborne_lunge_wraps_clock_tells_and_lean_without_health() {
        let tier = DifficultyTier::Sheltered;
        let mut air = CombatState::default();
        let mut ground = CombatState::default();
        let (mut was_air, mut was_ground) = (false, false);
        for dt in [LUNGE_SECS - LUNGE_TELL_SECS - 1e-3, 2e-3] {
            predator_tick(&mut air, dt, true, false, tier);
            predator_tick(&mut ground, dt, true, true, tier);
            assert_eq!(air.lunge_timer, ground.lunge_timer);
            let tell = tell_visible(air.lunge_timer, true);
            assert_eq!(tell, tell_visible(ground.lunge_timer, true));
            assert_eq!(lean_at(air.lunge_timer, true), lean_at(ground.lunge_timer, true));
            assert_eq!(tell_onset(&mut was_air, tell), tell_onset(&mut was_ground, tell_visible(ground.lunge_timer, true)));
            assert_eq!((air.player_taken, ground.player_taken, air.hurt, ground.hurt), (0, 0, 0, 0));
        }
        assert!(tell_visible(air.lunge_timer, true) && lean_at(air.lunge_timer, true) > 0.0 && lean_at(air.lunge_timer, true) <= LUNGE_LEAN);
        predator_tick(&mut air, LUNGE_TELL_SECS, true, false, tier);
        predator_tick(&mut ground, LUNGE_TELL_SECS, true, true, tier);
        assert_eq!(air.lunge_timer, ground.lunge_timer);
        assert_eq!(lean_at(air.lunge_timer, true), lean_at(ground.lunge_timer, true));
        assert_eq!(tell_visible(air.lunge_timer, true), tell_visible(ground.lunge_timer, true));
        assert_eq!(air.player_taken, 0);
        assert_eq!(ground.player_taken, LUNGE_HIT);
        let mut one = CombatState { player_taken: 11, hurt: 2, ..Default::default() };
        assert!(!predator_tick(&mut one, LUNGE_SECS, true, false, tier));
        assert_eq!((one.player_taken, one.lunge_timer, one.hurt), (11, 0.0, 2));
        assert!(!tell_visible(one.lunge_timer, true) && lean_at(one.lunge_timer, true) == 0.0);
        assert!(!hurt_edge(11, one.player_taken));
        let mut multi = CombatState::default();
        for _ in 0..5 {
            assert!(!predator_tick(&mut multi, LUNGE_SECS, true, false, tier));
        }
        assert_eq!((multi.player_taken, multi.lunge_timer), (0, 0.0));
        let (start, dt) = (0.0f32, LUNGE_SECS * 4.0);
        let (timer, wraps) = clock_after(start, dt, true);
        assert_eq!((timer, wraps), (0.0, 4));
        let mut mega = CombatState::default();
        assert!(!predator_tick(&mut mega, dt, true, false, tier));
        assert_eq!((mega.player_taken, mega.lunge_timer), (0, timer));
        let (start, dt) = (0.05f32, 3.7f32);
        let (timer, wraps) = clock_after(start, dt, true);
        assert_eq!(wraps, 6);
        let mut messy = CombatState { lunge_timer: start, ..Default::default() };
        let before = messy.player_taken;
        predator_tick(&mut messy, dt, true, false, tier);
        assert_eq!(messy.lunge_timer, timer);
        assert_eq!(messy.player_taken, before);
        assert!(!hurt_edge(before, messy.player_taken));
        assert_eq!(lean_at(messy.lunge_timer, true), lean_at(timer, true));
        assert_eq!(tell_visible(messy.lunge_timer, true), tell_visible(timer, true));
    }

    #[test]
    fn jump_dodge_grounded_lunge_keeps_tier_saturation_and_clock() {
        assert_eq!((enemy_hit(DifficultyTier::Sheltered, LUNGE_HIT), enemy_hit(DifficultyTier::Steady, LUNGE_HIT), enemy_hit(DifficultyTier::Sworn, LUNGE_HIT), enemy_hit(DifficultyTier::Unbroken, LUNGE_HIT)), (LUNGE_HIT, LUNGE_HIT, LUNGE_HIT, 17));
        let mut shel = CombatState::default();
        assert!(!predator_tick(&mut shel, LUNGE_SECS, true, true, DifficultyTier::Sheltered) && shel.player_taken == LUNGE_HIT && shel.lunge_timer == 0.0);
        let mut steady = CombatState::default();
        predator_tick(&mut steady, LUNGE_SECS, true, true, DifficultyTier::Steady);
        assert_eq!((steady.player_taken, steady.lunge_timer), (LUNGE_HIT, 0.0));
        let mut unbroken = CombatState::default();
        predator_tick(&mut unbroken, LUNGE_SECS, true, true, DifficultyTier::Unbroken);
        assert_eq!(unbroken.player_taken, 17);
        let mut sat = CombatState::default();
        for _ in 0..13 {
            assert!(!predator_tick(&mut sat, LUNGE_SECS, true, true, DifficultyTier::Sheltered));
        }
        assert_eq!(sat.player_taken, 13 * LUNGE_HIT);
        assert!(predator_tick(&mut sat, LUNGE_SECS, true, true, DifficultyTier::Sheltered));
        assert_eq!(sat.player_taken, HEALTH);
        let (start, dt) = (0.05f32, 3.7f32);
        let (timer, wraps) = clock_after(start, dt, true);
        let mut air = CombatState { lunge_timer: start, ..Default::default() };
        let mut ground = CombatState { lunge_timer: start, ..Default::default() };
        predator_tick(&mut air, dt, true, false, DifficultyTier::Sheltered);
        predator_tick(&mut ground, dt, true, true, DifficultyTier::Sheltered);
        assert_eq!(air.lunge_timer, timer);
        assert_eq!(ground.lunge_timer, timer);
        assert_eq!(air.player_taken, 0);
        assert_eq!(ground.player_taken, due_taken(0, wraps, DifficultyTier::Sheltered));
        let mut neg = CombatState { lunge_timer: 0.2, player_taken: 5, ..Default::default() };
        predator_tick(&mut neg, -0.5, true, true, DifficultyTier::Sheltered);
        assert_eq!((neg.lunge_timer, neg.player_taken), (0.2, 5), "dt.max(0.0)");
    }

    #[test]
    fn jump_dodge_out_of_reach_resets_either_ground_bool() {
        for grounded in [false, true] {
            let mut c = CombatState { lunge_timer: 0.7, player_taken: 40, hurt: 3, ..Default::default() };
            assert!(!predator_tick(&mut c, 3.0, false, grounded, DifficultyTier::Sheltered));
            assert_eq!((c.lunge_timer, c.player_taken, c.hurt), (0.0, 40, 3));
            assert_eq!(clock_after(0.7, 3.0, false), (0.0, 0));
            let mut held = CombatState { lunge_timer: 0.2, ..Default::default() };
            predator_tick(&mut held, -1.0, true, grounded, DifficultyTier::Sheltered);
            assert_eq!((held.lunge_timer, held.player_taken), (0.2, 0));
        }
    }

    #[test]
    fn jump_dodge_crossing_pays_only_the_grounded_due_not_stored_air() {
        let mut c = CombatState::default();
        for _ in 0..3 {
            assert!(!predator_tick(&mut c, LUNGE_SECS, true, false, DifficultyTier::Sheltered));
        }
        assert_eq!((c.player_taken, c.lunge_timer), (0, 0.0));
        assert!(!predator_tick(&mut c, 0.0, true, true, DifficultyTier::Sheltered));
        assert_eq!(c.player_taken, 0);
        let half = LUNGE_SECS * 0.5;
        assert!(!predator_tick(&mut c, half, true, true, DifficultyTier::Sheltered));
        assert_eq!((c.player_taken, c.lunge_timer), (0, half));
        assert!(!predator_tick(&mut c, LUNGE_SECS - half, true, true, DifficultyTier::Sheltered));
        assert_eq!((c.player_taken, c.lunge_timer), (LUNGE_HIT, 0.0), "jump avoids due damage but no future immune");
        let mut mid = CombatState { lunge_timer: 0.4, player_taken: 10, ..Default::default() };
        predator_tick(&mut mid, 0.1, true, false, DifficultyTier::Sheltered);
        assert_eq!((mid.player_taken, mid.lunge_timer), (10, 0.5));
        assert!(!hurt_edge(10, mid.player_taken));
        predator_tick(&mut mid, 0.1, true, true, DifficultyTier::Sheltered);
        assert_eq!((mid.player_taken, mid.lunge_timer), (10 + LUNGE_HIT, 0.0));
        assert!(hurt_edge(10, mid.player_taken));
    }

    #[test]
    fn jump_dodge_already_dead_stays_defeated_and_near_dead_airborne_does_not() {
        let mut near = CombatState { player_taken: HEALTH - LUNGE_HIT, hurt: 4, ..Default::default() };
        assert!(!predator_tick(&mut near, LUNGE_SECS * 4.0, true, false, DifficultyTier::Sheltered));
        assert_eq!((near.player_taken, near.lunge_timer, near.hurt), (HEALTH - LUNGE_HIT, 0.0, 4));
        let mut grounded_near = CombatState { player_taken: HEALTH - LUNGE_HIT, ..Default::default() };
        assert!(predator_tick(&mut grounded_near, LUNGE_SECS, true, true, DifficultyTier::Sheltered));
        assert_eq!(grounded_near.player_taken, HEALTH);
        let mut dead = CombatState { player_taken: HEALTH, hurt: 1, lunge_timer: 0.2, ..Default::default() };
        let (timer, _) = clock_after(0.2, LUNGE_SECS * 3.0, true);
        assert!(predator_tick(&mut dead, LUNGE_SECS * 3.0, true, false, DifficultyTier::Unbroken));
        assert_eq!((dead.player_taken, dead.hurt, dead.lunge_timer), (HEALTH, 1, timer));
        let mut dead_ground = CombatState { player_taken: HEALTH, ..Default::default() };
        assert!(predator_tick(&mut dead_ground, 0.0, true, true, DifficultyTier::Sheltered));
        assert_eq!(dead_ground.player_taken, HEALTH);
    }

    #[test]
    fn jump_dodge_hurt_edge_only_on_real_damage() {
        let mut c = CombatState::default();
        let before = c.player_taken;
        predator_tick(&mut c, LUNGE_SECS, true, false, DifficultyTier::Sheltered);
        assert!(!hurt_edge(before, c.player_taken));
        let before = c.player_taken;
        predator_tick(&mut c, LUNGE_SECS, true, true, DifficultyTier::Sheltered);
        assert!(hurt_edge(before, c.player_taken) && c.player_taken == LUNGE_HIT);
        let before = c.player_taken;
        predator_tick(&mut c, 0.0, true, true, DifficultyTier::Sheltered);
        assert!(!hurt_edge(before, c.player_taken));
    }

    fn run_predator_lunge(y: f32, grounded: bool, taken: u32, timer: f32, secs: f32, hurt: u32) -> World {
        use bevy::ecs::system::RunSystemOnce;
        let mut world = World::new();
        let mut time = Time::<()>::default();
        time.advance_by(std::time::Duration::from_secs_f32(secs));
        world.insert_resource(time);
        world.insert_resource(HexTravelState { current: PlaceId::Heartwood });
        world.insert_resource(LaunchDoor::InYard);
        let [x, z] = PREDATOR_HOME;
        world.insert_resource(SoftPresence { position: Vec3::new(x, y, z), velocity: Vec3::ZERO, grounded, prev_tick: None, curr_tick: None });
        world.insert_resource(CombatState { player_taken: taken, lunge_timer: timer, hurt, ..Default::default() });
        world.insert_resource(FirstHarvestEpiphany::default());
        world.clear_trackers();
        world.run_system_once(predator_lunge).unwrap();
        world
    }

    #[test]
    fn jump_dodge_predator_lunge_reads_presence_grounded() {
        let mut air = run_predator_lunge(0.0, false, 30, 0.0, LUNGE_SECS, 7);
        let c = air.resource::<CombatState>();
        assert_eq!((c.player_taken, c.hurt, c.lunge_timer, c.predator_taken, c.strikes, c.respawned, c.down_secs), (30, 7, 0.0, 0, 0, false, 0.0));
        let p = air.resource::<SoftPresence>();
        assert!(!p.grounded && p.position == Vec3::new(PREDATOR_HOME[0], 0.0, PREDATOR_HOME[1]) && p.velocity == Vec3::ZERO && p.prev_tick.is_none() && p.curr_tick.is_none());
        let epi = air.resource::<FirstHarvestEpiphany>();
        assert!(epi.pulse_line.is_empty() && epi.pulse_until == 0.0);
        assert_eq!(air.query::<&AudioPlayer>().iter(&air).count(), 0, "hurt edge sound not on a miss");
        assert!(!air.is_resource_changed::<SoftPresence>() && !air.is_resource_changed::<FirstHarvestEpiphany>());
        let high = run_predator_lunge(6.0, true, 0, 0.0, LUNGE_SECS, 0);
        let c = high.resource::<CombatState>();
        assert_eq!((c.player_taken, c.hurt), (LUNGE_HIT, 1));
        let p = high.resource::<SoftPresence>();
        assert!(p.grounded && p.position == Vec3::new(PREDATOR_HOME[0], 6.0, PREDATOR_HOME[1]) && p.velocity == Vec3::ZERO);
        let epi = high.resource::<FirstHarvestEpiphany>();
        assert_eq!(epi.pulse_line, hurt_line(LUNGE_HIT));
        assert_eq!(epi.pulse_until, high.resource::<Time>().elapsed_secs_f64() + CUE_SECS);
        let mut near = run_predator_lunge(1.5, false, HEALTH - LUNGE_HIT, 0.0, LUNGE_SECS * 4.0, 0);
        let c = near.resource::<CombatState>();
        assert_eq!((c.player_taken, c.hurt, c.lunge_timer), (HEALTH - LUNGE_HIT, 0, 0.0));
        let p = near.resource::<SoftPresence>();
        assert!(!p.grounded && p.position == Vec3::new(PREDATOR_HOME[0], 1.5, PREDATOR_HOME[1]));
        assert!(near.resource::<FirstHarvestEpiphany>().pulse_line.is_empty());
        assert_eq!(near.query::<&AudioPlayer>().iter(&near).count(), 0);
        let dead = run_predator_lunge(3.0, false, HEALTH, 0.0, LUNGE_SECS, 0);
        assert_eq!(dead.resource::<CombatState>().player_taken, 0);
        let p = dead.resource::<SoftPresence>();
        assert!(p.grounded && p.velocity == Vec3::ZERO);
        assert_eq!((p.position.x, p.position.z), (HEARTWOOD_ENTRANCE[0], HEARTWOOD_ENTRANCE[1]));
        assert_eq!(dead.resource::<FirstHarvestEpiphany>().pulse_line, wake_cue());
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
        assert!(!predator_tick(&mut c, LUNGE_SECS, true, true, DifficultyTier::Sheltered) && c.player_taken == LUNGE_HIT);
        assert!(predator_tick(&mut c, LUNGE_SECS * 14.0, true, true, DifficultyTier::Sheltered), "200 health, 15 per lunge");
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
    fn combat_text_acceptance_t4_t5_hooks() {
        use crate::combat_text::{hook_lunge_after, hook_player_strike, CombatTextKind, CombatTextQueue};
        let (mut q, mut c) = (CombatTextQueue::default(), CombatState::default());
        hook_player_strike(&mut q, PlaceId::Sanctuary, 0, 0, false, Vec3::ZERO);
        assert!(q.events.len() == 1 && fire_cue(PlaceId::Sanctuary, &c, false, 0).is_none());
        c.lunge_timer = LUNGE_SECS;
        let b = c.player_taken;
        predator_tick(&mut c, 0.0, true, true, DifficultyTier::Unbroken);
        hook_lunge_after(&mut q, b, c.player_taken, 1, true, Vec3::ZERO, PREDATOR_NAME);
        assert_eq!(q.events.last().unwrap().amount, c.player_taken - b);
        c.lunge_timer = LUNGE_SECS;
        let b2 = c.player_taken;
        predator_tick(&mut c, 0.0, true, false, DifficultyTier::Sheltered);
        hook_lunge_after(&mut q, b2, c.player_taken, 1, false, Vec3::ZERO, PREDATOR_NAME);
        assert_eq!(q.events.last().unwrap().kind, CombatTextKind::EnemyMissed);
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
        world.insert_resource(CombatState { player_taken: HEALTH - LUNGE_HIT, ..Default::default() });
        world.insert_resource(FirstHarvestEpiphany::default());
        world.run_system_once(predator_lunge).unwrap();
        assert_eq!(world.resource::<FirstHarvestEpiphany>().pulse_line, wake_cue());
        assert_eq!(world.resource::<CombatState>().player_taken, 0);
    }

    #[test]
    fn sanctuary_tints_only_in_sanctuary_and_globals_unchanged() {
        assert_eq!((PREDATOR_COLOR, FLASH_COLOR, LUNGE_TELL_COLOR), (Color::srgb(0.81, 0.62, 0.55), Color::srgb(1.0, 0.96, 0.9), Color::srgb(0.86, 0.85, 0.82)), "P1 globals unchanged");
        for p in [PlaceId::Heartwood, PlaceId::Depths] { assert!([0.0, 0.5, 1.0].iter().all(|s| flash_color_at(p, *s) == flash_color(*s)) && tell_colors(p) == (PREDATOR_COLOR, FLASH_COLOR, LUNGE_TELL_COLOR)); }
        assert_eq!((flash_color_at(PlaceId::Sanctuary, 0.0), tell_colors(PlaceId::Sanctuary)), (SANCTUARY_PREDATOR_COLOR, (SANCTUARY_PREDATOR_COLOR, SANCTUARY_FLASH_COLOR, SANCTUARY_LUNGE_TELL_COLOR)));
    }
    #[test]
    fn sanctuary_tint_on_entry_and_restore_on_leaving() {
        let mut world = World::new();
        world.insert_resource(Time::<()>::default());
        world.insert_resource(SoftPresence::default());
        world.init_resource::<CombatState>();
        world.init_resource::<Assets<StandardMaterial>>();
        let (body, ring) = { let mut a = world.resource_mut::<Assets<StandardMaterial>>(); (a.add(PREDATOR_COLOR), a.add(LUNGE_TELL_COLOR)) };
        world.spawn((MeshMaterial3d(body.clone()), Predator));
        world.spawn((MeshMaterial3d(ring.clone()), LungeTell));
        let (id, mut travel) = (world.register_system(predator_lunge), HexTravelState::default());
        let base = |w: &World, h: &Handle<StandardMaterial>| w.resource::<Assets<StandardMaterial>>().get(h).unwrap().base_color;
        for (place, want) in [(PlaceId::Sanctuary, (SANCTUARY_PREDATOR_COLOR, SANCTUARY_LUNGE_TELL_COLOR)), (PlaceId::Heartwood, (PREDATOR_COLOR, LUNGE_TELL_COLOR))] {
            travel.current = place;
            world.insert_resource(travel.clone());
            world.run_system(id).unwrap();
            assert_eq!((base(&world, &body), base(&world, &ring)), want, "{place:?}");
        }
    }
    #[test]
    fn sanctuary_pillar_takes_the_body_tint_and_restores() {
        let mut world = World::new();
        world.insert_resource(Time::<()>::default()); world.insert_resource(SoftPresence::default()); world.init_resource::<CombatState>(); world.init_resource::<Assets<StandardMaterial>>();
        let (pillar, ring) = { let mut a = world.resource_mut::<Assets<StandardMaterial>>(); (a.add(PREDATOR_COLOR), a.add(LUNGE_TELL_COLOR)) };
        world.spawn((MeshMaterial3d(pillar.clone()), RiseTell)); world.spawn((MeshMaterial3d(ring.clone()), LungeTell));
        let (id, mut travel) = (world.register_system(predator_lunge), HexTravelState::default());
        for (place, want) in [(PlaceId::Sanctuary, (SANCTUARY_PREDATOR_COLOR, SANCTUARY_LUNGE_TELL_COLOR)), (PlaceId::Heartwood, (PREDATOR_COLOR, LUNGE_TELL_COLOR))] { travel.current = place; world.insert_resource(travel.clone()); world.run_system(id).unwrap(); let m = world.resource::<Assets<StandardMaterial>>(); assert_eq!((m.get(&pillar).unwrap().base_color, m.get(&ring).unwrap().base_color), want, "{place:?}"); }
    }
    #[test]
    fn hurt_readout_line_and_matcher() {
        assert!(hurt_line(LUNGE_HIT) == "You 185 / 200" && hurt_line(enemy_hit(DifficultyTier::Unbroken, LUNGE_HIT)) == "You 183 / 200" && hurt_line(HEALTH + 5) == "You 0 / 200");
        assert!([hurt_line(LUNGE_HIT), hurt_line(HEALTH + 5)].iter().all(|l| is_hurt_line(l) && ["loot", "instance", "heroic", "mythic", "rebirth", "outrider", "online"].iter().all(|w| !l.to_lowercase().contains(w))));
        assert!([wake_cue(), wake_cue_at(PlaceId::Sanctuary), fire_cue(PlaceId::Heartwood, &CombatState::default(), true, 0).unwrap(), LUNGE_TELL_TEXT.into(), "You +5 / 200".into(), "You 1x / 200".into(), String::new()].iter().all(|l| !is_hurt_line(l)));
    }
    #[test]
    fn hurt_readout_only_on_a_free_slot() {
        let (s, fire) = (DifficultyTier::Sheltered, format!("{PREDATOR_NAME} 185 / 200"));
        assert!(hurt_pulse(s, "", 0.25, 0.25, LUNGE_HIT).as_deref() == Some("You 185 / 200") && hurt_pulse(s, &fire, 0.5, 0.25, LUNGE_HIT).is_none() && hurt_pulse(s, &wake_cue(), 0.5, 0.25, LUNGE_HIT).is_none() && hurt_pulse(s, &fire, 0.5, 0.5, LUNGE_HIT).is_some(), "a live wake line is kept");
        assert_eq!(hurt_pulse(s, &hurt_line(LUNGE_HIT), 0.5, 0.25, 2 * LUNGE_HIT).as_deref(), Some("You 170 / 200"), "a hurt line replaces a hurt line");
    }
    #[test]
    fn hurt_readout_hidden_on_sworn_and_unbroken() { assert!(crate::difficulty::TIERS.iter().all(|t| hurt_pulse(*t, "", 0.0, 0.25, LUNGE_HIT).is_some() != hides_cues(*t))); }
    #[test]
    fn hurt_frame_writes_the_readout() {
        use bevy::ecs::system::RunSystemOnce;
        let mut world = World::new();
        let mut time = Time::<()>::default();
        time.advance_by(std::time::Duration::from_secs_f64(0.125));
        world.insert_resource(time);
        world.insert_resource(HexTravelState { current: PlaceId::Heartwood });
        world.insert_resource(LaunchDoor::InYard);
        world.insert_resource(SoftPresence { position: Vec3::new(PREDATOR_HOME[0], STAND_HEIGHT, PREDATOR_HOME[1]), ..Default::default() });
        world.insert_resource(CombatState { lunge_timer: 0.5, ..Default::default() });
        world.insert_resource(FirstHarvestEpiphany::default());
        world.run_system_once(predator_lunge).unwrap();
        let (epi, c) = (world.resource::<FirstHarvestEpiphany>(), world.resource::<CombatState>()); assert!(c.player_taken == LUNGE_HIT && epi.pulse_line == "You 185 / 200" && epi.pulse_until == 0.125 + CUE_SECS);
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
            assert_eq!(line, format!("{want}{} · level {}", crate::skill_xp::SKILL_NAMES[COMBAT], level.expect("crossing")));
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
        predator_tick(&mut c, 0.02, true, true, DifficultyTier::Sheltered);
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
    fn hit_pulse_scale_rests_at_one_and_peaks_small() {
        assert_eq!([0.0, -0.5, -1.0, f32::NAN].map(hit_pulse_scale), [1.0; 4]);
        let sweep: Vec<f32> = (0..=40).map(|i| hit_pulse_scale(i as f32 / 20.0)).collect();
        assert!(sweep.windows(2).all(|w| w[0] <= w[1]) && sweep[20] <= 1.12 && sweep[40] <= 1.12);
    }

    fn pulse_app() -> (App, Entity, Transform) {
        let (mut app, _) = flash_app();
        let at = Transform::from_xyz(PREDATOR_HOME[0], 0.8, PREDATOR_HOME[1]);
        let mut q = app.world_mut().query_filtered::<Entity, With<Predator>>();
        let e = q.single(app.world()).unwrap();
        app.world_mut().entity_mut(e).insert(at);
        (app, e, at)
    }
    #[test]
    fn hit_pulse_scales_restores_one_never_moves_or_changes_reach() {
        let (mut app, e, at) = pulse_app();
        app.world_mut().resource_mut::<CombatState>().predator_taken = HIT;
        step(&mut app, 0.0);
        let t = *app.world().get::<Transform>(e).unwrap();
        assert!(t.scale.x > 1.0 && (t.translation, t.rotation) == (at.translation, at.rotation));
        // in_reach takes no Transform (PREDATOR_HOME plus REACH): same answer at pulse peak and at rest.
        let ([x, z], ds) = (PREDATOR_HOME, [0.0, REACH, 2.0 * REACH]);
        let reach = |c: &CombatState| ds.map(|d| in_reach(PlaceId::Heartwood, c, x + d, z));
        let at_peak = reach(app.world().resource());
        step(&mut app, FLASH_SECS + 0.01);
        assert_eq!(reach(app.world().resource()), at_peak);
        assert_eq!(*app.world().get::<Transform>(e).unwrap(), at);
    }

    #[test]
    fn two_quick_hits_never_exceed_pulse_peak() {
        let (mut app, e, _) = pulse_app();
        for (taken, dt) in [(HIT, 0.0), (HIT, 0.02), (2 * HIT, 0.0), (2 * HIT, 0.03)] {
            app.world_mut().resource_mut::<CombatState>().predator_taken = taken;
            step(&mut app, dt);
            assert!(app.world().get::<Transform>(e).unwrap().scale.max_element() <= 1.12);
        }
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
        let (mut paid, mut lives, cap) = (0, 3, crate::skill_xp::COMBAT_PAYING_HITS_PER_DAY); // 25 hits a life: 3 lives cross the cap
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
        assert_eq!(rise_tell(PlaceId::Depths, HEALTH, onset + 1.0), None, "never in Depths");
        let (low, high) = (rise_tell_transform(0.2), rise_tell_transform(0.8));
        assert!(high.scale.y > low.scale.y && high.translation.y > low.translation.y, "it rises");
        let base = |t: Transform| t.translation.y - RISE_TELL_HEIGHT * t.scale.y * 0.5;
        assert!(base(low).abs() < 1e-5 && base(high).abs() < 1e-5, "grows from the ground");
        assert_eq!((high.translation.x, high.translation.z), (PREDATOR_HOME[0], PREDATOR_HOME[1]));
    }

    #[test]
    fn cleared_edges_hand_off_to_rise_tell() {
        let (hw, onset) = (PlaceId::Heartwood, PREDATOR_RESPAWN_SECS - PREDATOR_TELL_SECS);
        assert!(!cleared(hw, HEALTH - HIT, 5.0), "alive reads uncleared");
        assert!(cleared(hw, HEALTH, 0.0) && cleared(hw, HEALTH, 10.0) && cleared(hw, HEALTH, onset - 0.01), "kill or re-kill, mid-down, onset - eps");
        assert!(!cleared(hw, HEALTH, onset) && rise_tell(hw, HEALTH, onset).is_some(), "the pillar takes over at onset");
        assert!(!cleared(hw, 0, PREDATOR_RESPAWN_SECS) && !cleared(hw, 0, 0.0), "fire frame and respawned read uncleared");
        assert!(!cleared(PlaceId::Depths, HEALTH, 1.0), "never in Depths");
    }
    #[test]
    fn cleared_never_overlaps_rise_tell() {
        for (d, taken) in (0..=640).flat_map(|i| [(i as f32 * 0.05, HEALTH - HIT), (i as f32 * 0.05, HEALTH)]) {
            assert!(!(cleared(PlaceId::Heartwood, taken, d) && rise_tell(PlaceId::Heartwood, taken, d).is_some()), "overlap at {d} s");
        }
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
        app.world_mut().resource_mut::<HexTravelState>().current = PlaceId::Depths;
        assert_eq!(tick(&mut app, 0.1), Visibility::Hidden, "hidden in Depths");
        app.world_mut().resource_mut::<HexTravelState>().current = PlaceId::Heartwood;
        assert_eq!(tick(&mut app, 0.1), Visibility::Visible);
        assert_eq!(tick(&mut app, 1.0), Visibility::Hidden, "hidden after respawn");
        let c = app.world().resource::<CombatState>();
        assert!(c.respawned && c.predator_taken == 0 && c.down_secs == 0.0);
    }
    #[test]
    fn lunge_cues_per_tier() {
        for t in crate::difficulty::TIERS {
            let (mut c, mut was) = (CombatState::default(), false);
            predator_tick(&mut c, LUNGE_SECS - LUNGE_TELL_SECS - 1e-3, true, true, t);
            assert!(!tell_onset(&mut was, tell_visible(c.lunge_timer, true)), "timer unchanged on {t:?}");
            predator_tick(&mut c, 2e-3, true, true, t);
            let tell = tell_visible(c.lunge_timer, true);
            assert!(tell_onset(&mut was, tell) && !tell_onset(&mut was, tell), "tell fires once on {t:?}");
            assert_eq!(tell && !hides_cues(t), t <= DifficultyTier::Steady, "ring shows on tiers 1-2 only");
            predator_tick(&mut c, LUNGE_TELL_SECS, true, true, t);
            assert_eq!(c.player_taken, if t == DifficultyTier::Unbroken { 17 } else { LUNGE_HIT });
        }
        let low = LUNGE_TELL_TEXT.to_lowercase();
        assert!(["loot", "instance", "heroic", "mythic", "rebirth", "outrider", "online"].iter().all(|w| !low.contains(w)));
    }

    #[test]
    fn rewards_unbroken_day_pays_300_and_hit_21_nothing() {
        let (mut c, mut s) = (CombatState::default(), SkillXp::default());
        assert_eq!(crate::skill_xp::COMBAT_PAYING_HITS_PER_DAY, 20);
        for _ in 0..20 {
            c.predator_taken = 0;
            assert!(land_hit_on(&mut c, &mut s, true, DifficultyTier::Unbroken).0);
        }
        assert_eq!(s.xp[COMBAT], 300);
        c.predator_taken = 0;
        assert!(land_hit_on(&mut c, &mut s, true, DifficultyTier::Unbroken).0 && s.xp[COMBAT] == 300, "hit 21 pays nothing");
        let (mut c, mut s) = (CombatState::default(), SkillXp::default());
        land_hit_on(&mut c, &mut s, true, DifficultyTier::Sheltered);
        assert_eq!(s.xp[COMBAT], 10, "Sheltered is exactly x1.00");
        let (mut c, mut s) = (CombatState::default(), SkillXp::default());
        s.xp[COMBAT] = crate::skill_xp::XP_CAP - 5;
        land_hit_on(&mut c, &mut s, true, DifficultyTier::Unbroken);
        assert_eq!(s.xp[COMBAT], crate::skill_xp::XP_CAP, "XP_CAP still clamps");
    }
    #[test]
    fn muted_lunge_tell_text_shows_only_on_hidden_cue_tiers() {
        use bevy::ecs::system::RunSystemOnce;
        for t in crate::difficulty::TIERS {
            let mut world = World::new();
            world.insert_resource(Time::<()>::default());
            let mut travel = HexTravelState::default();
            travel.current = PlaceId::Heartwood;
            world.insert_resource(travel);
            world.insert_resource(LaunchDoor::InYard);
            let [x, z] = PREDATOR_HOME;
            world.insert_resource(SoftPresence { position: Vec3::new(x, STAND_HEIGHT, z), ..Default::default() });
            world.insert_resource(CombatState { lunge_timer: LUNGE_SECS - LUNGE_TELL_SECS, ..Default::default() });
            world.insert_resource(AreaDifficulty { default: t, ..Default::default() });
            world.insert_resource(FirstHarvestEpiphany::default());
            world.insert_resource(crate::local_settings::MasterMuteGain { gain: 1.0, muted: true });
            let ring = world.spawn((LungeTell, Visibility::Hidden)).id();
            world.run_system_once(lunge_tell).unwrap();
            let (line, vis) = (world.resource::<FirstHarvestEpiphany>().pulse_line.clone(), *world.get::<Visibility>(ring).unwrap());
            if hides_cues(t) {
                assert_eq!((line.as_str(), vis), (LUNGE_TELL_TEXT, Visibility::Hidden), "{t:?} muted keeps the text tell");
            } else {
                assert_eq!((line, vis), (FirstHarvestEpiphany::default().pulse_line, Visibility::Visible), "{t:?} muted writes no text tell");
            }
        }
    }

    /// COMBAT-REACH-1 (a) — the Sanctuary fight is reachable with no book and no settle.
    #[test]
    fn sanctuary_fight_reachable_without_the_book() {
        let c = CombatState::default();
        let [x, z] = STAVEHOLD_FIGHT_HOME;
        assert!(fight_present(PlaceId::Sanctuary) && in_reach(PlaceId::Sanctuary, &c, x, z));
        assert!(!shared::hex_travel::places_eligible(false, false));
        assert!(!fight_present(PlaceId::Depths) && !in_reach(PlaceId::Depths, &c, x, z));
    }

    /// COMBAT-REACH-1 (b) + CL-R3 — `predator_present` unchanged, and the Heartwood home is byte-identical.
    #[test]
    fn predator_present_and_heartwood_home_unchanged() {
        assert!(predator_present(PlaceId::Heartwood) && !predator_present(PlaceId::Sanctuary));
        assert_eq!((fight_home(PlaceId::Heartwood), home_shift(PlaceId::Heartwood)), ([-18.0, -18.0], Vec3::ZERO));
    }

    /// COMBAT-REACH-1 (c) + CL-R5 — Heartwood still needs the book, and only a Heartwood win records a clear.
    #[test]
    fn heartwood_still_needs_the_book_and_only_it_clears() {
        assert!(!shared::hex_travel::places_eligible(true, false) && shared::hex_travel::places_eligible(true, true));
        assert!(clears_area(PlaceId::Heartwood) && !clears_area(PlaceId::Sanctuary));
    }

    /// COMBAT-REACH-1 (d) + CL-R2/R4 + A1 — a Sanctuary defeat wakes at the gate, out of reach, so no tell stacks.
    #[test]
    fn sanctuary_defeat_wakes_at_the_gate_out_of_reach() {
        let (mut c, mut p) = (CombatState { player_taken: HEALTH, ..default() }, SoftPresence::default());
        reseat_at(PlaceId::Sanctuary, &mut c, &mut p);
        assert_eq!((p.position.x, p.position.z, c.player_taken), (STAVEHOLD_GATE_SPOT[0], STAVEHOLD_GATE_SPOT[1], 0));
        assert!(!in_reach(PlaceId::Sanctuary, &c, p.position.x, p.position.z) && !tell_visible(LUNGE_SECS, false));
        assert_eq!(wake_cue_at(PlaceId::Sanctuary), "You wake at the Stavehold gate");
        assert_eq!(wake_cue_at(PlaceId::Heartwood), wake_cue());
        assert_eq!(fire_cue(PlaceId::Sanctuary, &c, false, 0), None, "A1: no miss line out of reach");
        assert!(fire_cue(PlaceId::Sanctuary, &c, true, 0).is_some(), "A1: in reach a strike lands and speaks");
    }

    /// COMBAT-FEEL-SFX-1 (a) — the hit is its own file, never the sting.
    #[test]
    fn hit_asset_is_not_the_sting() {
        assert_ne!(HIT_ASSET, STING_ASSET);
        assert_eq!(HIT_ASSET, "audio/combat_hit.ogg");
    }

    /// COMBAT-FEEL-SFX-1 (b) — source pin: the tell still loads the sting, and the hit call sits after the landed strike.
    #[test]
    fn tell_still_loads_the_sting() {
        let src = include_str!("combat_core.rs");
        let start = src.find(concat!("pub fn play", "_tell(")).unwrap();
        let body = &src[start..start + src[start..].find("\n}\n").unwrap()];
        assert!(body.contains(concat!("assets.load(", "STING_ASSET)")) && !body.contains(concat!("HIT", "_ASSET")));
        assert_eq!(src.matches(concat!("play_hit", "(&mut cmd")).count(), 1, "one call site");
    }

    /// COMBAT-FEEL-SFX-1 (c) — never louder than the sting: -4.0 dBFS file peak x HIT_GAIN <= -27.7 dBFS.
    #[test]
    fn hit_gain_stays_under_the_sting() {
        assert!(HIT_GAIN <= 0.05 && HIT_GAIN > 0.0);
        assert!(-4.0 + 20.0 * HIT_GAIN.log10() <= -27.7);
        assert!(FIND_GAIN > 0.0 && FIND_GAIN <= HIT_GAIN && -14.6 + 20.0 * FIND_GAIN.log10() <= -27.7);
        let full = crate::local_settings::MasterMuteGain { gain: 1.0, muted: false };
        assert_eq!(hit_volume(true, Some(&full)), Some(HIT_GAIN));
        assert_eq!(find_volume(Some(&full)), Some(FIND_GAIN));
        assert_eq!(hit_volume(true, None), Some(HIT_GAIN));
        assert!(include_str!("../../assets/asset-licenses.csv").contains("find_pickup.ogg"));
    }

    /// COMBAT-FEEL-SFX-1 (d) — a miss, mute or zero gain gives no hit sound.
    #[test]
    fn miss_or_mute_gives_no_hit_sound() {
        let (full, muted) = (crate::local_settings::MasterMuteGain { gain: 1.0, muted: false }, crate::local_settings::MasterMuteGain { gain: 1.0, muted: true });
        assert_eq!(hit_volume(false, Some(&full)), None);
        assert_eq!(hit_volume(false, None), None);
        assert_eq!(hit_volume(true, Some(&muted)), None);
        assert_eq!(hit_volume(true, Some(&crate::local_settings::MasterMuteGain { gain: 0.0, muted: false })), None);
        assert_eq!(find_volume(Some(&muted)), None);
    }

    fn sfx_find_app(pending: &str) -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, bevy::asset::AssetPlugin::default()))
            .insert_resource(Time::<()>::default())
            .insert_resource(FirstHarvestEpiphany::default())
            .insert_resource(CalmQueue::default())
            .insert_resource(FoundFinds { pending: Some(pending.into()), ..Default::default() });
        app
    }

    #[test]
    fn sfx_wire_find_reveal_once() {
        use bevy::ecs::system::RunSystemOnce;
        let n = audio_output_safe() as usize;
        let mut app = sfx_find_app("hook");
        for _ in 0..2 {
            app.world_mut().run_system_once(show_find).unwrap();
            assert_eq!(app.world_mut().query::<&AudioPlayer>().iter(app.world_mut()).count(), n);
        }
        app.world_mut().resource_mut::<FoundFinds>().pending = None;
        app.world_mut().run_system_once(show_find).unwrap();
        assert_eq!(app.world_mut().query::<&AudioPlayer>().iter(app.world_mut()).count(), 0);
    }

    #[test]
    fn sfx_wire_one_thud_per_strike() {
        let (n, [px, pz], st) = (audio_output_safe() as usize, PREDATOR_HOME, CombatState::default());
        assert!(!(in_reach(PlaceId::Heartwood, &st, px, pz) && crate::droid::droid_in_reach(PlaceId::Heartwood, &Default::default(), px, pz)));
        let mut w = home_frame(combat_fire, None, st, DifficultyTier::Sheltered);
        assert!(w.resource::<CombatState>().strikes > 0 && w.query::<&AudioPlayer>().iter(&mut w).count() <= n);
    }

    #[test]
    fn sfx_wire_missing_asset_no_panic_find() { use bevy::ecs::system::RunSystemOnce; sfx_find_app("x").world_mut().run_system_once(show_find).unwrap(); }

    /// GEAR-SLOT-1 — one frame of `sys` at the Heartwood home in the yard: LMB just pressed, a focused window, a due lunge.
    fn home_frame<M>(sys: impl IntoSystem<(), (), M>, finds: Option<FoundFinds>, combat: CombatState, tier: DifficultyTier) -> World {
        use bevy::ecs::system::RunSystemOnce;
        let (mut world, mut time, mut lmb, [x, z]) = (World::new(), Time::<()>::default(), ButtonInput::<MouseButton>::default(), PREDATOR_HOME);
        time.advance_by(std::time::Duration::from_secs_f32(LUNGE_SECS));
        lmb.press(FIRE);
        world.insert_resource(time);
        world.insert_resource(lmb);
        world.insert_resource(LaunchDoor::InYard);
        world.insert_resource(HexTravelState { current: PlaceId::Heartwood });
        world.insert_resource(SoftPresence { position: Vec3::new(x, STAND_HEIGHT, z), ..Default::default() });
        world.insert_resource(combat);
        world.insert_resource(SkillXp::default());
        world.insert_resource(AreaDifficulty { default: tier, ..Default::default() });
        world.spawn((Window { focused: true, ..default() }, PrimaryWindow));
        if let Some(f) = finds { world.insert_resource(f); }
        world.run_system_once(sys).unwrap();
        world
    }

    /// GS2 — CUT table: one landed strike with each owned find equipped is HIT plus its standing step, never past 18, clamped at
    /// HEALTH. No slot, no FoundFinds, an unowned or unknown name and a cleared slot land exactly HIT; the lunge ignores the slot.
    #[test]
    fn gs2_equipped_standing_adds_to_one_outgoing_strike_only() {
        use shared::temper::{Standing, NAMED_FINDS};
        let table = [("Yard-Worn Tend Hook", Standing::Kept, 1), ("Steady Climate Pick", Standing::Kept, 1), ("Dawn-Fair Lane Crate", Standing::Proven, 2),
            ("Neighbour's Mend Spindle", Standing::Proven, 2), ("Quiet Harmony Loom", Standing::Storied, 3), ("Bookkeeper's Hook", Standing::Storied, 3)];
        let all = FoundFinds { names: NAMED_FINDS.iter().map(|f| f.name).collect(), ..Default::default() };
        let strike = |finds: Option<&FoundFinds>, taken: u32| {
            let (mut c, mut s) = (CombatState { predator_taken: taken, ..Default::default() }, SkillXp::default());
            (land_hit_paid(&mut c, &mut s, true, DifficultyTier::Sheltered, None, finds), c.predator_taken, s.xp[COMBAT])
        };
        for ((name, standing, bonus), f) in table.into_iter().zip(NAMED_FINDS.iter()) {
            let geared = FoundFinds { equipped: Some(name), ..all.clone() };
            assert_eq!((f.name, f.standing()), (name, standing));
            assert_eq!(strike(Some(&geared), 0), ((true, None), HIT + bonus, XP_PER_LANDED_HIT), "{name}");
            assert!(HIT + bonus <= 18 && strike(Some(&geared), HEALTH - HIT).1 == HEALTH, "{name} clamps at HEALTH");
        }
        let unowned = FoundFinds { names: vec![NAMED_FINDS[0].name], equipped: Some(NAMED_FINDS[5].name), ..Default::default() };
        let (unknown, mut cleared) = (FoundFinds { equipped: Some("no such find"), ..all.clone() }, FoundFinds { equipped: Some(NAMED_FINDS[5].name), ..all.clone() });
        cleared.clear_equipped();
        for finds in [None, Some(&all), Some(&unowned), Some(&unknown), Some(&cleared)] {
            assert_eq!(strike(finds, 0), ((true, None), HIT, XP_PER_LANDED_HIT));
        }
        let storied = FoundFinds { equipped: Some(NAMED_FINDS[5].name), ..all.clone() };
        let fire = |f| { let w = home_frame(combat_fire, f, CombatState::default(), DifficultyTier::Sheltered); (w.resource::<CombatState>().predator_taken, w.resource::<SkillXp>().xp[COMBAT]) };
        assert_eq!([fire(None), fire(Some(all.clone())), fire(Some(storied.clone()))], [(HIT, XP_PER_LANDED_HIT), (HIT, XP_PER_LANDED_HIT), (HIT + 3, XP_PER_LANDED_HIT)]);
        for t in crate::difficulty::TIERS {
            let hurt = |f| home_frame(predator_lunge, f, CombatState::default(), t).resource::<CombatState>().player_taken;
            assert_eq!([hurt(None), hurt(Some(storied.clone()))], [enemy_hit(t, LUNGE_HIT); 2], "{t:?} lunge ignores the slot");
        }
    }
    use bevy::ecs::system::RunSystemOnce;
    use crate::loot_drops::{find_line, loot_hash, tests::in_user_dir, FINDS_SAVE, LOOT_BASE_CHANCE, LOOT_BPS}; use shared::temper::NAMED_FINDS;
    fn fight_frame(place: PlaceId, taken: u32, finds: FoundFinds) -> World { // FIGHT-PROGRESS-1; home_frame stays byte-identical
        let (mut w, mut lmb, [x, z]) = (World::new(), ButtonInput::<MouseButton>::default(), fight_home(place));
        lmb.press(FIRE);
        w.insert_resource(Time::<()>::default()); w.insert_resource(lmb); w.insert_resource(LaunchDoor::InYard);
        w.insert_resource(HexTravelState { current: place }); w.insert_resource(SoftPresence { position: Vec3::new(x, STAND_HEIGHT, z), ..default() });
        w.insert_resource(CombatState { predator_taken: taken, ..default() }); w.insert_resource(SkillXp::default()); w.insert_resource(FirstHarvestEpiphany::default());
        w.insert_resource(AreaDifficulty::default()); w.insert_resource(finds); w.spawn((Window { focused: true, ..default() }, PrimaryWindow));
        w.run_system_once(combat_fire).unwrap(); w
    }
    fn refire(w: &mut World, taken: u32) {
        w.resource_mut::<CombatState>().predator_taken = taken;
        let mut lmb = ButtonInput::<MouseButton>::default(); lmb.press(FIRE); w.insert_resource(lmb);
        w.run_system_once(combat_fire).unwrap();
    }
    #[test]
    fn fp1_first_sanctuary_win_drops_one_find() {
        in_user_dir("fp1", |dir| {
            let w = fight_frame(PlaceId::Sanctuary, HEALTH - HIT, FoundFinds::default());
            assert_eq!(w.resource::<CombatState>().predator_taken, HEALTH);
            assert_eq!(w.resource::<SkillXp>().xp[COMBAT], XP_PER_LANDED_HIT);
            assert_eq!(w.resource::<FirstHarvestEpiphany>().pulse_line, "Heartwood Prowler is down");
            let f = w.resource::<FoundFinds>().clone();
            assert_eq!(f.names.len(), 1);
            let named = NAMED_FINDS.iter().find(|n| n.name == f.names[0]).expect("named");
            assert_eq!(f.pending.as_deref(), Some(find_line(named).as_str()));
            assert_eq!(FoundFinds::decode(Some(&std::fs::read_to_string(dir.join(FINDS_SAVE)).unwrap())).names, f.names);
        });
    }
    #[test]
    fn fp2_once_per_save_no_farm() {
        in_user_dir("fp2", |dir| {
            let mut w = fight_frame(PlaceId::Sanctuary, HEALTH - HIT, FoundFinds::default());
            let (names, bytes) = (w.resource::<FoundFinds>().names.clone(), std::fs::read(dir.join(FINDS_SAVE)).unwrap());
            w.resource_mut::<FoundFinds>().pending = None; refire(&mut w, HEALTH - HIT);
            let after = w.resource::<FoundFinds>().clone();
            assert!(after.names == names && after.pending.is_none() && std::fs::read(dir.join(FINDS_SAVE)).unwrap() == bytes);
            let owned = FoundFinds { names: vec![NAMED_FINDS[0].name], ..FoundFinds::default() };
            assert_eq!(fight_frame(PlaceId::Sanctuary, HEALTH - HIT, owned.clone()).resource::<FoundFinds>().clone(), owned);
            w.resource_mut::<Time>().advance_by(std::time::Duration::from_secs_f32(PREDATOR_RESPAWN_SECS)); w.run_system_once(predator_respawn).unwrap();
            assert_eq!(w.resource::<CombatState>().predator_taken, 0);
            refire(&mut w, HEALTH - HIT); assert_eq!(w.resource::<FoundFinds>().names, names);
        });
    }
    #[test]
    fn fp3_find_persists_and_a_later_win_adds_nothing() {
        in_user_dir("fp3", |dir| {
            let names = fight_frame(PlaceId::Sanctuary, HEALTH - HIT, FoundFinds::default()).resource::<FoundFinds>().names.clone();
            assert_eq!(names.len(), 1);
            let mut fresh = World::new(); fresh.insert_resource(FoundFinds::default()); fresh.run_system_once(load_finds).unwrap();
            assert_eq!(fresh.resource::<FoundFinds>().names, names);
            assert_eq!(fight_frame(PlaceId::Sanctuary, HEALTH - HIT, fresh.resource::<FoundFinds>().clone()).resource::<FoundFinds>().names, names);
            let (live, bak) = (dir.join(FINDS_SAVE), dir.join(format!("{FINDS_SAVE}.bak")));
            std::fs::copy(&live, &bak).unwrap(); std::fs::write(&live, b"{{{").unwrap();
            fresh.run_system_once(load_finds).unwrap();
            assert_eq!(fresh.resource::<FoundFinds>().names, names);
        });
    }
    #[test]
    fn fp4_find_line_shows_and_equips() {
        in_user_dir("fp4", |_| {
            let mut w = fight_frame(PlaceId::Sanctuary, HEALTH - HIT, FoundFinds::default());
            w.insert_resource(CalmQueue::default()); w.resource_mut::<FirstHarvestEpiphany>().pulse_until = 0.0;
            let line = w.resource::<FoundFinds>().pending.clone().unwrap();
            w.run_system_once(show_find).unwrap();
            assert_eq!(w.resource::<FirstHarvestEpiphany>().pulse_line, line);
            assert!(w.resource_mut::<FoundFinds>().cycle_equipped());
            let f = w.resource::<FoundFinds>();
            assert!(f.equipped == Some(f.names[0]) && f.encode().contains("equipped"));
            let (mut c, mut s, bonus) = (CombatState::default(), SkillXp::default(), equipped_strike_bonus(Some(f)));
            land_hit_paid(&mut c, &mut s, true, DifficultyTier::Sheltered, None, Some(f));
            assert!(c.predator_taken == HIT + bonus && (1..=3).contains(&bonus));
        });
    }
    #[test]
    fn fp5_heartwood_miss_and_sanctuary_skips_clear() {
        in_user_dir("fp5", |_| {
            let h = loot_hash(1, PlaceId::Heartwood as u32, 0);
            let bar = (crate::difficulty::drop_chance(DifficultyTier::Sheltered, LOOT_BASE_CHANCE) * LOOT_BPS as f32) as u32;
            assert!((h % LOOT_BPS) >= bar);
            let wood = fight_frame(PlaceId::Heartwood, HEALTH - HIT, FoundFinds::default());
            let missed = wood.resource::<FoundFinds>().clone();
            assert!(missed.names.is_empty() && missed.pending.is_none());
            assert_eq!(wood.resource::<AreaDifficulty>().clears, [0, 1, 0]);
            let yard = fight_frame(PlaceId::Sanctuary, HEALTH - HIT, FoundFinds::default());
            assert_eq!(yard.resource::<AreaDifficulty>().clears, [0; 3]);
            assert_eq!(yard.resource::<FoundFinds>().names.len(), 1);
        });
    }
    use crate::first_harvest_epiphany::{tend_harmony_pulse_line, HarvestPulseRoot, HarvestPulseText};
    use crate::first_session_guidance::FirstSessionGuidanceText;
    use crate::hud_anchor_registry::{HudSlab, ID_PICKUP, ID_PULSE, ID_SOVEREIGN, ID_THRIVING, ID_WELCOME, ID_WHISPER};
    use crate::hud_presets::{ActiveHudPreset, HudLayoutPlugin, HudPresetId, HudYielded};
    fn yard_find(place: PlaceId) -> FoundFinds { let mut f = FoundFinds::default(); assert!(f.first_find(1, place as u32) && f.names.len() == 1); f }
    fn wood_drop() -> FoundFinds {
        let place = PlaceId::Heartwood as u32;
        (0..4096).find_map(|s| { let mut f = FoundFinds::default(); f.drop_once(DifficultyTier::Sworn, s, place).then_some(f) }).expect("drop_once")
    }
    fn flv<R>(tag: &str, preset: HudPresetId, place: PlaceId, finds: FoundFinds, body: impl FnOnce(&mut App, &str) -> R) -> R {
        in_user_dir(tag, |_| {
            let expected = finds.pending.clone().unwrap_or_default();
            let mut app = App::new();
            app.add_plugins((MinimalPlugins, bevy::asset::AssetPlugin::default(), crate::input::InputPlugin, crate::first_session_guidance::FirstSessionGuidancePlugin, crate::first_harvest_epiphany::FirstHarvestEpiphanyPlugin, HudLayoutPlugin)).insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(std::time::Duration::ZERO)).insert_resource(ActiveHudPreset { id: Some(preset) }).insert_resource(LaunchDoor::InYard).insert_resource(HouseLabel::default()).insert_resource(PlacesPlate::default()).insert_resource(CalmQueue::default()).insert_resource(HexTravelState { current: place }).insert_resource(ButtonInput::<KeyCode>::default()).insert_resource(finds).insert_resource(crate::local_settings::LocalSettingsState { inner: shared::local_settings::LocalSettings::default(), dirty: false }).init_resource::<crate::hour_sacred::HourSacred>().init_resource::<crate::thriving_moments::ThrivingMoments>().init_resource::<crate::abundance_journey_echo::AbundanceJourneyEcho>().init_resource::<crate::mercy_harvest_nodes::NearbyMercyNode>().init_resource::<crate::harvest_feel::SoftRbePool>().init_resource::<crate::world_answer::WorldAnswer>().add_message::<bevy::input::keyboard::KeyboardInput>().add_message::<bevy::input::mouse::MouseButtonInput>().add_message::<bevy::input::touch::TouchInput>().add_message::<GamepadRumbleRequest>().add_systems(Update, show_find);
            body(&mut app, &expected)
        })
    }
    fn show2(app: &mut App) { app.world_mut().resource_mut::<FirstHarvestEpiphany>().pulse_until = 0.0; app.update(); app.update(); }
    fn pulse_of(app: &mut App) -> (String, Visibility) {
        let text = app.world_mut().query_filtered::<&Text, With<HarvestPulseText>>().single(app.world()).unwrap().as_str().to_string();
        (text, *app.world_mut().query_filtered::<&Visibility, With<HarvestPulseRoot>>().single(app.world()).unwrap())
    }
    fn visible(tag: &str, preset: HudPresetId, place: PlaceId, finds: FoundFinds) {
        flv(tag, preset, place, finds, |app, expected| { show2(app); assert_eq!(pulse_of(app), (expected.to_string(), Visibility::Visible), "{tag}"); });
    }
    #[test]
    fn flv1_classic_yard_find_line_visible() { visible("flv1", HudPresetId::Classic, PlaceId::Sanctuary, yard_find(PlaceId::Sanctuary)); }
    #[test]
    fn flv2_minimal_find_line_visible() { visible("flv2", HudPresetId::Minimal, PlaceId::Sanctuary, yard_find(PlaceId::Sanctuary)); }
    #[test]
    fn flv3_heartwood_drop_once_find_line_visible() { visible("flv3", HudPresetId::Classic, PlaceId::Heartwood, wood_drop()); }
    #[test]
    fn flv4_management_find_line_visible() { visible("flv4", HudPresetId::Management, PlaceId::Sanctuary, yard_find(PlaceId::Sanctuary)); }
    #[test]
    fn flv5_non_find_pulses_stay_hidden() {
        let rows = [(level_up_line(COMBAT, 2), Visibility::Visible), (tend_harmony_pulse_line("Sanctuary ember", 0.4), Visibility::Hidden), (LUNGE_TELL_TEXT.to_string(), Visibility::Visible)];
        flv("flv5", HudPresetId::Classic, PlaceId::Sanctuary, FoundFinds::default(), |app, _| {
            for (line, want) in rows {
                { let mut epi = app.world_mut().resource_mut::<FirstHarvestEpiphany>(); (epi.pulse_line, epi.pulse_until) = (line.clone(), CUE_SECS); }
                app.update();
                assert_eq!((pulse_of(app).1, app.world().resource::<FirstHarvestEpiphany>().pulse_line.as_str()), (want, line.as_str()), "{line}");
            }
        });
    }
    #[test]
    fn flv6_find_line_hides_after_cue() {
        flv("flv6", HudPresetId::Classic, PlaceId::Sanctuary, yard_find(PlaceId::Sanctuary), |app, expected| {
            show2(app);
            let (text, vis) = pulse_of(app);
            assert_eq!((text.as_str(), vis, app.world().resource::<FirstHarvestEpiphany>().pulse_until), (expected, Visibility::Visible, CUE_SECS));
            app.world_mut().resource_mut::<bevy::time::Time<bevy::time::Virtual>>().set_max_delta(std::time::Duration::from_secs(5));
            app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(std::time::Duration::from_secs_f64(CUE_SECS + 0.05)));
            app.update();
            let (text, vis) = pulse_of(app);
            assert_eq!((text.as_str(), vis, app.world().resource::<FirstHarvestEpiphany>().pulse_until), (expected, Visibility::Hidden, CUE_SECS));
        });
    }
    #[test]
    fn flv7_pause_or_places_hides_the_find_line() {
        for (tag, pause, places) in [("flv7p", true, false), ("flv7d", false, true)] {
            flv(tag, HudPresetId::Classic, PlaceId::Sanctuary, yard_find(PlaceId::Sanctuary), |app, expected| {
                app.world_mut().resource_mut::<HouseLabel>().settings_open = pause;
                app.world_mut().resource_mut::<PlacesPlate>().open = places;
                show2(app);
                assert_eq!(pulse_of(app), (expected.to_string(), Visibility::Hidden), "{tag}");
            });
        }
    }
    #[test]
    fn flv8_other_top_toast_occupants_stay_hidden() {
        let ids = [ID_PICKUP, ID_THRIVING, ID_SOVEREIGN, ID_WHISPER, ID_WELCOME];
        flv("flv8", HudPresetId::Classic, PlaceId::Sanctuary, yard_find(PlaceId::Sanctuary), |app, expected| {
            let spawned: Vec<_> = ids.into_iter().map(|id| (id, app.world_mut().spawn((Node::default(), Visibility::Visible, HudSlab(id))).id())).collect();
            show2(app);
            assert_eq!(pulse_of(app), (expected.to_string(), Visibility::Visible));
            for (id, entity) in spawned { assert_eq!(*app.world().get::<Visibility>(entity).unwrap(), Visibility::Hidden, "{id}"); }
            let yielded = &app.world().resource::<HudYielded>().0;
            assert!(ids.into_iter().all(|id| yielded.contains(&id)) && !yielded.contains(&ID_PULSE));
        });
    }
    #[test]
    fn flv10_find_line_stays_off_the_guidance_strip() {
        flv("flv10", HudPresetId::Classic, PlaceId::Sanctuary, yard_find(PlaceId::Sanctuary), |app, expected| {
            show2(app);
            let guide = app.world_mut().query_filtered::<&Text, With<FirstSessionGuidanceText>>().single(app.world()).unwrap().as_str().to_string();
            assert!(guide != expected && !guide.contains(expected), "{guide}");
            assert_eq!(pulse_of(app), (expected.to_string(), Visibility::Visible));
        });
    }

    fn thread(app: &mut App) { fn set(app: &mut App, label: impl bevy::ecs::schedule::ScheduleLabel) { app.edit_schedule(label, |s| { s.set_executor(bevy::ecs::schedule::SingleThreadedExecutor::new()); }); } set(app, Startup); set(app, First); set(app, PreUpdate); set(app, Update); set(app, PostUpdate); set(app, Last); }
    fn tick(app: &mut App, dt: f32) { app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(std::time::Duration::from_secs_f32(dt))); app.update(); }
    fn rings<C: Component>(app: &mut App) -> Vec<Visibility> { app.world_mut().query_filtered::<&Visibility, With<C>>().iter(app.world()).copied().collect() }
    fn yard<R>(tag: &str, preset: HudPresetId, place: PlaceId, tier: DifficultyTier, at: [f32; 2], body: impl FnOnce(&mut App) -> R) -> R {
        in_user_dir(tag, |_| { let mut app = App::new(); app.add_plugins((MinimalPlugins, bevy::asset::AssetPlugin::default(), crate::input::InputPlugin, crate::first_session_guidance::FirstSessionGuidancePlugin, crate::first_harvest_epiphany::FirstHarvestEpiphanyPlugin, HudLayoutPlugin, crate::ui_font::UiFontPlugin, CombatCorePlugin)).init_asset::<Mesh>().init_asset::<StandardMaterial>().insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(std::time::Duration::ZERO)).insert_resource(ActiveHudPreset { id: Some(preset) }).insert_resource(LaunchDoor::InYard).insert_resource(HouseLabel::default()).insert_resource(PlacesPlate::default()).insert_resource(CalmQueue::default()).insert_resource(HexTravelState { current: place }).insert_resource(ButtonInput::<KeyCode>::default()).insert_resource(SoftPresence { position: Vec3::new(at[0], 0.9, at[1]), grounded: true, ..default() }).insert_resource(AreaDifficulty { default: tier, ..Default::default() }).init_resource::<SkillXp>().insert_resource(crate::local_settings::LocalSettingsState { inner: shared::local_settings::LocalSettings::default(), dirty: false }).init_resource::<crate::hour_sacred::HourSacred>().init_resource::<crate::thriving_moments::ThrivingMoments>().init_resource::<crate::abundance_journey_echo::AbundanceJourneyEcho>().init_resource::<crate::mercy_harvest_nodes::NearbyMercyNode>().init_resource::<crate::harvest_feel::SoftRbePool>().init_resource::<crate::world_answer::WorldAnswer>().add_message::<bevy::input::keyboard::KeyboardInput>().add_message::<bevy::input::mouse::MouseButtonInput>().add_message::<bevy::input::touch::TouchInput>().add_message::<GamepadRumbleRequest>(); thread(&mut app); app.world_mut().resource_mut::<bevy::time::Time<bevy::time::Virtual>>().set_max_delta(std::time::Duration::from_secs(5)); app.world_mut().spawn((Window { focused: true, ..default() }, PrimaryWindow)); app.update(); body(&mut app) })
    }
    fn open_lunge(app: &mut App) -> (String, Vec<Visibility>, String, Visibility) { tick(app, LUNGE_SECS - LUNGE_TELL_SECS + 0.02); let wrote = app.world().resource::<FirstHarvestEpiphany>().pulse_line.clone(); let dots = rings::<LungeTell>(app); tick(app, 0.0); let (text, vis) = pulse_of(app); (wrote, dots, text, vis) }
    fn shown_lunge(tag: &str, preset: HudPresetId, tier: DifficultyTier) { yard(tag, preset, PlaceId::Heartwood, tier, PREDATOR_HOME, |app| { let (wrote, dots, text, vis) = open_lunge(app); assert_eq!((wrote.as_str(), text.as_str(), vis, app.world().resource::<CombatState>().player_taken), (LUNGE_TELL_TEXT, LUNGE_TELL_TEXT, Visibility::Visible, 0), "{tag}"); assert!(!dots.is_empty() && dots.iter().all(|d| *d == Visibility::Hidden), "{tag}"); }); }
    #[test]
    fn tv1_sworn_lunge_tell_visible_ring_hidden() { for (tag, tier) in [("tv1s", DifficultyTier::Sworn), ("tv1u", DifficultyTier::Unbroken)] { shown_lunge(tag, HudPresetId::Classic, tier); } }
    #[test]
    fn tv3_minimal_lunge_tell_visible() { shown_lunge("tv3l", HudPresetId::Minimal, DifficultyTier::Sworn); }
    #[test]
    fn tv4_management_lunge_tell_visible() { shown_lunge("tv4", HudPresetId::Management, DifficultyTier::Sworn); }
    #[test]
    fn tv5_sheltered_and_steady_write_no_lunge_tell() { for (tag, tier) in [("tv5s", DifficultyTier::Sheltered), ("tv5y", DifficultyTier::Steady)] { yard(tag, HudPresetId::Classic, PlaceId::Heartwood, tier, PREDATOR_HOME, |app| { let (wrote, dots, _, _) = open_lunge(app); assert!(wrote.is_empty() && !dots.is_empty() && dots.iter().all(|d| *d == Visibility::Visible), "{tag} {wrote}"); }); } }
    fn cross(app: &mut App, taken: u32) { app.world_mut().resource_mut::<SkillXp>().xp[COMBAT] = crate::skill_xp::xp_for_level(2) - XP_PER_LANDED_HIT; app.world_mut().resource_mut::<CombatState>().predator_taken = taken; app.world_mut().resource_mut::<StrikeBuffer>().fire = true; tick(app, 0.0); tick(app, 0.0); }
    #[test]
    fn lv2_combat_level_and_joined_kill_visible() {
        yard("lv2a", HudPresetId::Classic, PlaceId::Sanctuary, DifficultyTier::Sheltered, STAVEHOLD_FIGHT_HOME, |app| { cross(app, 0); assert_eq!(pulse_of(app), (level_up_line(COMBAT, 2), Visibility::Visible)); assert!(app.world().resource::<CombatState>().predator_taken < HEALTH); });
        yard("lv2b", HudPresetId::Classic, PlaceId::Heartwood, DifficultyTier::Sheltered, PREDATOR_HOME, |app| { cross(app, HEALTH - HIT); let want = { let combat = app.world().resource::<CombatState>(); assert!(combat.predator_taken >= HEALTH); fire_line(PlaceId::Heartwood, combat, true, XP_PER_LANDED_HIT, Some(2)).unwrap() }; assert!(want.ends_with(&level_up_line(COMBAT, 2))); assert_eq!(pulse_of(app), (want, Visibility::Visible)); });
    }
    #[test]
    fn lv3_combat_level_visible_on_the_next_frame() { yard("lv3", HudPresetId::Classic, PlaceId::Sanctuary, DifficultyTier::Sheltered, STAVEHOLD_FIGHT_HOME, |app| { let find = crate::loot_drops::find_line(&shared::temper::NAMED_FINDS[0]); { let mut epi = app.world_mut().resource_mut::<FirstHarvestEpiphany>(); (epi.pulse_line, epi.pulse_until) = (find.clone(), CUE_SECS); } tick(app, 0.0); tick(app, 0.0); assert_eq!(pulse_of(app), (find, Visibility::Visible)); cross(app, 0); assert_eq!(pulse_of(app), (level_up_line(COMBAT, 2), Visibility::Visible)); }); }
    #[test]
    fn nr1_tend_charter_fire_and_wake_stay_hidden() { yard("nr1", HudPresetId::Classic, PlaceId::Sanctuary, DifficultyTier::Sheltered, STAVEHOLD_FIGHT_HOME, |app| { let lines = [crate::first_harvest_epiphany::tend_pulse_at_place("Sanctuary ember", 0.4, Some("Sanctuary")), "Not your charter / Peace visitor".into(), fire_cue(PlaceId::Heartwood, &CombatState { predator_taken: HIT, ..default() }, true, XP_PER_LANDED_HIT).unwrap(), wake_cue_at(PlaceId::Heartwood), wake_cue_at(PlaceId::Sanctuary)]; for line in lines { { let mut epi = app.world_mut().resource_mut::<FirstHarvestEpiphany>(); (epi.pulse_line, epi.pulse_until) = (line.clone(), CUE_SECS); } tick(app, 0.0); assert_eq!((pulse_of(app).1, app.world().resource::<FirstHarvestEpiphany>().pulse_line.as_str()), (Visibility::Hidden, line.as_str()), "{line}"); } }); }
    #[test]
    fn ex1_lunge_tell_and_combat_level_hide_after_cue() {
        yard("ex1t", HudPresetId::Classic, PlaceId::Heartwood, DifficultyTier::Sworn, PREDATOR_HOME, |app| { let (_, _, text, vis) = open_lunge(app); assert_eq!((text.as_str(), vis), (LUNGE_TELL_TEXT, Visibility::Visible)); app.world_mut().resource_mut::<SoftPresence>().grounded = false; tick(app, LUNGE_TELL_SECS + 0.05); assert_eq!((pulse_of(app).1, app.world().resource::<FirstHarvestEpiphany>().pulse_line.as_str()), (Visibility::Hidden, LUNGE_TELL_TEXT)); });
        yard("ex1l", HudPresetId::Classic, PlaceId::Sanctuary, DifficultyTier::Sheltered, STAVEHOLD_FIGHT_HOME, |app| { cross(app, 0); assert_eq!(pulse_of(app).1, Visibility::Visible); app.world_mut().resource_mut::<SoftPresence>().grounded = false; tick(app, CUE_SECS as f32 + 0.05); assert_eq!((pulse_of(app).1, app.world().resource::<FirstHarvestEpiphany>().pulse_line.as_str()), (Visibility::Hidden, level_up_line(COMBAT, 2).as_str())); });
    }
    #[test]
    fn r5_pause_or_places_hides_tell_and_level() { for (tag, pause, places) in [("r5p", true, false), ("r5d", false, true)] { yard(tag, HudPresetId::Classic, PlaceId::Heartwood, DifficultyTier::Sworn, PREDATOR_HOME, |app| { let (_, _, text, vis) = open_lunge(app); assert_eq!((text.as_str(), vis), (LUNGE_TELL_TEXT, Visibility::Visible)); app.world_mut().resource_mut::<HouseLabel>().settings_open = pause; app.world_mut().resource_mut::<PlacesPlate>().open = places; tick(app, 0.0); assert_eq!(pulse_of(app).1, Visibility::Hidden, "{tag}"); { let mut epi = app.world_mut().resource_mut::<FirstHarvestEpiphany>(); (epi.pulse_line, epi.pulse_until) = (level_up_line(COMBAT, 2), CUE_SECS); } tick(app, 0.0); assert_eq!(pulse_of(app).1, Visibility::Hidden, "{tag} level"); }); } }
    #[test]
    fn r2_tell_leaves_other_top_toast_occupants_hidden() {
        let ids = [ID_PICKUP, ID_THRIVING, ID_SOVEREIGN, ID_WHISPER, ID_WELCOME];
        yard("r2", HudPresetId::Classic, PlaceId::Heartwood, DifficultyTier::Sworn, PREDATOR_HOME, |app| { let spawned: Vec<_> = ids.into_iter().map(|id| (id, app.world_mut().spawn((Node::default(), Visibility::Visible, HudSlab(id))).id())).collect(); let (_, _, text, vis) = open_lunge(app); assert_eq!((text.as_str(), vis), (LUNGE_TELL_TEXT, Visibility::Visible)); for (id, entity) in spawned { assert_eq!(*app.world().get::<Visibility>(entity).unwrap(), Visibility::Hidden, "{id}"); } let yielded = &app.world().resource::<HudYielded>().0; assert!(ids.into_iter().all(|id| yielded.contains(&id)) && !yielded.contains(&ID_PULSE)); });
    }

    // CARD COMBAT-CORE-2a SHELL-PLATE — key 2 raises Plate (×0.7 lunge, 3 s) inside the visible tell with a Shell find.
    const CC2A_PLATE_TINT: Color = Color::srgb(0.86, 0.60, 0.22);
    fn cc2a<R>(tag: &str, tier: DifficultyTier, shell: Option<usize>, body: impl FnOnce(&mut App) -> R) -> R {
        yard(tag, HudPresetId::Classic, PlaceId::Heartwood, tier, PREDATOR_HOME, |app| {
            let mut f = app.world_mut().resource_mut::<FoundFinds>();
            f.names = shared::temper::NAMED_FINDS.iter().map(|n| n.name).collect();
            if let Some(i) = shell { for _ in 0..=i { assert!(f.cycle_equipped()); } } // GEAR-EQUIP-1: what satchel + 4 runs
            drop(f);
            app.add_plugins(crate::player_lineage::PlayerLineagePlugin);
            let mat = app.world_mut().resource_mut::<Assets<StandardMaterial>>().add(Color::WHITE);
            let (prim, leaf) = (mat.clone(), app.world_mut().resource_mut::<Assets<StandardMaterial>>().add(Color::WHITE));
            app.world_mut().spawn((crate::human_presence::HumanPresence, Mesh3d::default(), MeshMaterial3d(prim)));
            app.world_mut().spawn(crate::human_presence::HumanPresence).with_child(MeshMaterial3d(leaf));
            tick(app, 0.0);
            body(app)
        })
    }
    /// The primitive root's material and the authored leaf's material, in that order.
    fn cc2a_lanes(app: &App) -> Vec<Color> {
        let (w, mats) = (app.world(), app.world().resource::<Assets<StandardMaterial>>());
        let mut q = w.try_query::<(&MeshMaterial3d<StandardMaterial>, Option<&ChildOf>, Has<crate::human_presence::HumanPresence>)>().unwrap();
        q.iter(w).filter(|(_, up, own)| *own || up.is_some_and(|c| w.get::<crate::human_presence::HumanPresence>(c.parent()).is_some())).map(|(m, _, own)| (!own, mats.get(&m.0).unwrap().base_color)).collect::<std::collections::BTreeMap<_, _>>().into_values().collect()
    }
    fn cc2a_tints(app: &App) -> Vec<Color> { let mut v = cc2a_lanes(app); v.dedup(); v }
    fn cc2a_keys(app: &mut App, keys: &[KeyCode]) {
        keys.iter().for_each(|k| app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(*k));
        tick(app, 0.0);
        keys.iter().for_each(|k| app.world_mut().resource_mut::<ButtonInput<KeyCode>>().reset(*k));
    }
    fn cc2a_press_at(app: &mut App, timer: f32) {
        app.world_mut().resource_mut::<CombatState>().lunge_timer = timer;
        cc2a_keys(app, &[KeyCode::Digit2]);
    }
    /// Steps 1/60 s for `secs`; returns each landed lunge's damage in order.
    fn cc2a_hits(app: &mut App, secs: f32) -> Vec<u32> {
        (0..(secs * 60.0).round() as usize).filter_map(|_| { let b = app.world().resource::<CombatState>().player_taken; tick(app, 1.0 / 60.0); Some(app.world().resource::<CombatState>().player_taken.saturating_sub(b)).filter(|d| *d > 0) }).collect()
    }
    const CC2A_TELL: f32 = LUNGE_SECS - LUNGE_TELL_SECS;

    #[test]
    fn cc2a_s1_shell_key2_in_tell_plates_lunges_for_three_seconds() {
        cc2a("cc2a1", DifficultyTier::Sheltered, Some(0), |app| {
            cc2a_press_at(app, CC2A_TELL);
            assert_eq!(cc2a_hits(app, 3.4), [11, 11, 11, 11, 11, 15], "plated wraps at +0.35 .. +2.75 s, plain at +3.35 s");
        });
    }

    #[test]
    fn cc2a_s2_unbroken_17_to_12_and_sworn_to_11() {
        for (tier, want) in [(DifficultyTier::Unbroken, [12, 17]), (DifficultyTier::Sworn, [11, 15]), (DifficultyTier::Steady, [11, 15])] {
            cc2a("cc2a2", tier, Some(0), |app| {
                assert_eq!(enemy_hit(tier, LUNGE_HIT), want[1]);
                cc2a_press_at(app, CC2A_TELL);
                assert_eq!(cc2a_hits(app, 0.4), [want[0]], "{tier:?}");
            });
        }
    }

    #[test]
    fn cc2a_s3_outside_the_visible_tell_no_plate() {
        for (tag, timer, at, down) in [("cc2a3a", 0.24, PREDATOR_HOME, false), ("cc2a3b", CC2A_TELL - 1e-3, PREDATOR_HOME, false), ("cc2a3c", 0.0, PREDATOR_HOME, false), ("cc2a3d", 0.4, [PREDATOR_HOME[0] + REACH + 1.0, PREDATOR_HOME[1]], false), ("cc2a3e", 0.4, PREDATOR_HOME, true)] {
            cc2a(tag, DifficultyTier::Sheltered, Some(0), |app| {
                app.world_mut().resource_mut::<SoftPresence>().position = Vec3::new(at[0], 0.9, at[1]);
                if down { app.world_mut().resource_mut::<CombatState>().predator_taken = HEALTH; }
                cc2a_press_at(app, timer);
                let c = app.world().resource::<CombatState>();
                assert_eq!((c.player_taken, c.strikes, c.predator_taken == HEALTH, cc2a_tints(app)), (0, 0, down, vec![crate::player_lineage::Lineage::Human.tint()]), "{tag}");
                if !down && at == PREDATOR_HOME { assert_eq!(cc2a_hits(app, 0.62), [15], "{tag}"); }
            });
        }
    }

    #[test]
    fn cc2a_s4_no_shell_find_no_plate() {
        let rows: Vec<(String, Option<usize>)> = std::iter::once(("none".to_string(), None)).chain((1..shared::temper::NAMED_FINDS.len()).map(|i| (format!("find{i}"), Some(i)))).collect();
        for (tag, eq) in rows {
            cc2a(&format!("cc2a4{tag}"), DifficultyTier::Sheltered, eq, |app| {
                if let Some(i) = eq { assert_ne!(app.world().resource::<FoundFinds>().equipped_find().unwrap().ward, Some(shared::temper::WardKind::Shell), "{i}"); }
                cc2a_press_at(app, CC2A_TELL);
                assert_eq!(cc2a_hits(app, 0.4), [15], "{tag}");
            });
        }
        cc2a("cc2a4owned", DifficultyTier::Sheltered, None, |app| {
            app.world_mut().resource_mut::<FoundFinds>().names = vec![shared::temper::NAMED_FINDS[0].name];
            cc2a_press_at(app, CC2A_TELL);
            assert_eq!(cc2a_hits(app, 0.4), [15], "owned but not equipped");
        });
    }

    #[test]
    fn cc2a_s5_once_per_lunge_then_next_lunge_after_expiry_arms_again() {
        cc2a("cc2a5", DifficultyTier::Sheltered, Some(0), |app| {
            cc2a_press_at(app, CC2A_TELL);
            cc2a_keys(app, &[KeyCode::Digit2]);
            assert_eq!(cc2a_hits(app, 3.4), [11, 11, 11, 11, 11, 15], "the same-lunge press changes nothing");
            let t = app.world().resource::<CombatState>().lunge_timer;
            assert!(t < CC2A_TELL, "{t}");
            tick(app, CC2A_TELL + 0.01 - t);
            cc2a_keys(app, &[KeyCode::Digit2]);
            assert_eq!(cc2a_hits(app, 0.4), [11], "a later lunge arms again");
        });
    }

    #[test]
    fn cc2a_s6_repress_while_up_is_ignored_no_refresh() {
        cc2a("cc2a6", DifficultyTier::Sheltered, Some(0), |app| {
            cc2a_press_at(app, CC2A_TELL);
            assert_eq!(cc2a_hits(app, 1.8), [11, 11, 11]);
            assert!(app.world().resource::<CombatState>().lunge_timer >= CC2A_TELL);
            cc2a_keys(app, &[KeyCode::Digit2]);
            assert_eq!(cc2a_hits(app, 1.6), [11, 11, 15], "Plate still ends 3 s after the first arm");
        });
    }

    #[test]
    fn cc2a_s7_jump_dodge_under_plate_takes_nothing_and_plate_decays() {
        cc2a("cc2a7", DifficultyTier::Sheltered, Some(0), |app| {
            cc2a_press_at(app, CC2A_TELL);
            app.world_mut().resource_mut::<SoftPresence>().grounded = false;
            assert_eq!(cc2a_hits(app, 3.1), Vec::<u32>::new());
            app.world_mut().resource_mut::<SoftPresence>().grounded = true;
            assert_eq!(cc2a_hits(app, 0.6), [15]);
        });
    }

    #[test]
    fn cc2a_s8_defeat_clears_plate_and_tint_reverts_at_three_seconds() {
        cc2a("cc2a8a", DifficultyTier::Sheltered, Some(0), |app| {
            let human = crate::player_lineage::Lineage::Human.tint();
            app.world_mut().resource_mut::<CombatState>().player_taken = HEALTH - 1;
            cc2a_press_at(app, CC2A_TELL);
            assert_eq!(cc2a_tints(app), [CC2A_PLATE_TINT]);
            assert_eq!(cc2a_hits(app, 0.4), Vec::<u32>::new(), "defeat reseats");
            assert_eq!((app.world().resource::<CombatState>().player_taken, cc2a_tints(app)), (0, vec![human]));
            app.world_mut().resource_mut::<SoftPresence>().position = Vec3::new(PREDATOR_HOME[0], 0.9, PREDATOR_HOME[1]);
            assert_eq!(cc2a_hits(app, 0.62), [15], "no Plate after the wake");
        });
        cc2a("cc2a8b", DifficultyTier::Sheltered, Some(0), |app| {
            cc2a_press_at(app, CC2A_TELL);
            app.insert_resource(HexTravelState { current: PlaceId::Sanctuary });
            let _ = cc2a_hits(app, 178.0 / 60.0);
            assert_eq!(cc2a_tints(app), [CC2A_PLATE_TINT], "up after 178 ticks");
            let _ = cc2a_hits(app, 3.0 / 60.0);
            assert_eq!(cc2a_tints(app), [crate::player_lineage::Lineage::Human.tint()], "down by 181 ticks");
        });
    }

    #[test]
    fn cc2a_s9_gates_block_arming() {
        for tag in ["settings", "places", "satchel", "title"] {
            cc2a(&format!("cc2a9{tag}"), DifficultyTier::Sheltered, Some(0), |app| {
                match tag { "settings" => app.world_mut().resource_mut::<HouseLabel>().settings_open = true, "places" => app.world_mut().resource_mut::<PlacesPlate>().open = true, "satchel" => { app.insert_resource(HumanInventory { open: true, ..default() }); } _ => { app.insert_resource(LaunchDoor::Title); } }
                cc2a_press_at(app, CC2A_TELL);
                assert_eq!(cc2a_tints(app), [crate::player_lineage::Lineage::Human.tint()], "{tag}");
                app.insert_resource(LaunchDoor::InYard);
                assert_eq!(cc2a_hits(app, 0.62), [15], "{tag}");
            });
        }
    }

    #[test]
    fn cc2a_s10_lunge_only_droid_shot_unchanged_and_reaches_apart() {
        cc2a("cc2a10", DifficultyTier::Sheltered, Some(0), |app| {
            cc2a_press_at(app, CC2A_TELL);
            let (mut state, mut taken) = (crate::droid::DroidState { shot_timer: crate::droid::DROID_SHOT_SECS - 1e-3, ..default() }, 0u32);
            crate::droid::shot_step(&mut state, &mut taken, 0.01, true, true, true, DifficultyTier::Sheltered);
            assert_eq!(taken, enemy_hit(DifficultyTier::Sheltered, crate::droid::DROID_HIT));
        });
        let gap = Vec2::from(PREDATOR_HOME).distance(Vec2::from(crate::droid::DROID_HOME)) - REACH - crate::droid::DROID_REACH;
        assert!(gap > 5.4 * 3.0, "SPRINT (human_presence.rs:141) for 3 s can't bridge {gap}");
    }

    #[test]
    fn cc2a_s11_slot_zero_untouched_and_key1_wins_the_frame() {
        cc2a("cc2a11", DifficultyTier::Sheltered, Some(0), |app| {
            app.world_mut().resource_mut::<CombatState>().lunge_timer = CC2A_TELL;
            cc2a_keys(app, &[KeyCode::Digit1, KeyCode::Digit2]);
            let c = app.world().resource::<CombatState>();
            assert_eq!((c.strikes, c.predator_taken, cc2a_tints(app)), (1, HIT + 1, vec![crate::player_lineage::Lineage::Human.tint()]));
            cc2a_keys(app, &[KeyCode::Digit2]);
            let (c, buf) = (app.world().resource::<CombatState>(), app.world().resource::<StrikeBuffer>());
            assert_eq!((c.strikes, c.predator_taken, buf.fire, buf.until), (1, HIT + 1, false, None));
        });
    }

    #[test]
    fn cc2a_s12_plate_tint_both_lanes_on_the_arm_frame() {
        cc2a("cc2a12", DifficultyTier::Sheltered, Some(0), |app| {
            let human = crate::player_lineage::Lineage::Human.tint();
            assert_eq!(cc2a_tints(app), [human]);
            cc2a_press_at(app, CC2A_TELL);
            assert_eq!(cc2a_lanes(app), [CC2A_PLATE_TINT; 2], "both lanes the arm frame");
            let lin = |c: Color| c.to_linear().to_vec3();
            for l in [crate::player_lineage::Lineage::Human, crate::player_lineage::Lineage::Cydruid, crate::player_lineage::Lineage::Quellorian, crate::player_lineage::Lineage::Draek, crate::player_lineage::Lineage::Ambrosian] {
                assert!(lin(CC2A_PLATE_TINT).distance(lin(l.tint())) > 0.2, "{l:?}");
            }
        });
    }

    #[test]
    fn cc2a_s13_s14_no_reused_cue_no_copy_no_cooldown() {
        cc2a("cc2a13", DifficultyTier::Sheltered, Some(0), |app| {
            let epi = app.world().resource::<FirstHarvestEpiphany>().pulse_line.clone();
            let audio = app.world_mut().query::<&AudioPlayer>().iter(app.world()).count();
            cc2a_press_at(app, CC2A_TELL);
            let c = app.world().resource::<CombatState>();
            assert_eq!((c.hurt, c.player_taken, app.world().resource::<FirstHarvestEpiphany>().pulse_line.clone()), (0, 0, epi));
            assert_eq!(app.world_mut().query::<&AudioPlayer>().iter(app.world()).count(), audio);
        });
        assert!(!slot_strikes(Some(1)) && slot_tooltip(1).is_some_and(|t| !t.to_lowercase().contains("cooldown"))); // ABILITY-TOOLTIP-SHOW-1 A3
        let prod = include_str!("combat_core.rs").split("\n#[cfg(test)]\nmod tests {").next().unwrap();
        let state = &prod[prod.find("pub struct CombatState {").unwrap()..];
        assert!(!state[..state.find("\n}").unwrap()].to_lowercase().contains("cooldown"));
    }

    // STRIKE-CANON-1 — COMBAT_AND_PVP.md:435/:676 the player strike lands 8; the lunge stays 15 (:486/:678).
    fn sc1_kill(finds: Option<&FoundFinds>) -> (u32, u32) {
        let (mut c, mut s, mut n) = (CombatState::default(), SkillXp::default(), 0);
        while c.predator_taken < HEALTH { assert!(land_hit_paid(&mut c, &mut s, true, DifficultyTier::Sheltered, None, finds).0); n += 1; } (n, s.xp[COMBAT])
    }
    #[test] fn sc1_s1_one_strike_lands_8_on_every_tier() { assert_eq!(crate::difficulty::TIERS.map(|t| { let mut c = CombatState::default(); assert!(land_hit_on(&mut c, &mut SkillXp::default(), true, t).0); c.predator_taken }), [8; 4]); }
    #[test] fn sc1_s2_24_strikes_leave_192_the_25th_downs_and_the_kill_pays_200() {
        let (mut c, mut s) = (CombatState::default(), SkillXp::default());
        assert!((0..24).all(|_| land_hit(&mut c, &mut s, true)) && c.predator_taken == 192 && land_hit(&mut c, &mut s, true) && c.predator_taken == HEALTH && sc1_kill(None) == (25, 200));
    }
    #[test] fn sc1_s3_kept_proven_storied_land_9_10_11_and_kill_in_23_20_19() {
        let all = FoundFinds { names: shared::temper::NAMED_FINDS.iter().map(|f| f.name).collect(), ..Default::default() };
        for (i, want) in [(0, (9, 23)), (2, (10, 20)), (4, (11, 19))] {
            let (geared, mut c) = (FoundFinds { equipped: Some(all.names[i]), ..all.clone() }, CombatState::default()); land_hit_paid(&mut c, &mut SkillXp::default(), true, DifficultyTier::Sheltered, None, Some(&geared));
            assert_eq!((c.predator_taken, sc1_kill(Some(&geared)).0), want, "{}", all.names[i]);
        }
    }
    #[test] fn sc1_s4_lunge_stays_15_and_the_hurt_lines_hold() {
        assert_eq!(crate::difficulty::TIERS.map(|t| { let mut c = CombatState::default(); predator_tick(&mut c, LUNGE_SECS, true, true, t); c.player_taken }), [15, 15, 15, 17]); assert!(hurt_line(15) == "You 185 / 200" && hurt_line(enemy_hit(DifficultyTier::Unbroken, 15)) == "You 183 / 200");
    }
    #[test] fn sc1_s5_the_droid_takes_8_and_its_shot_still_lands_25() {
        use crate::droid::{shot_step, strike_step, DroidState, DROID_SHOT_SECS};
        let (mut d, mut last, mut p) = (DroidState::default(), 0, 0);
        shot_step(&mut DroidState::default(), &mut p, DROID_SHOT_SECS, true, true, true, DifficultyTier::Sheltered);
        assert!(strike_step(&mut d, &mut last, 1, true, true).is_some() && (d.taken, p) == (8, 25));
    }
    #[test] fn sc1_s6_slot_tooltip_reads_8_and_reach() {
        let text = slot_tooltip(0).unwrap(); assert!(text.contains("for 8 when") && text.contains(&REACH.to_string()) && ["class", "soon", "coming", "unlock", "cooldown"].iter().all(|b| !text.to_lowercase().contains(b)), "{text}");
    }
    #[test] fn sc1_s7_ten_xp_a_landed_hit_and_hits_21_to_25_pay_nothing() {
        let (mut c, mut s, mut paid) = (CombatState::default(), SkillXp::default(), vec![]); for _ in 0..25 { if c.predator_taken >= HEALTH { c.predator_taken = 0 } let xp = s.xp[COMBAT]; assert!(land_hit(&mut c, &mut s, true)); paid.push(s.xp[COMBAT] - xp); }
        assert_eq!((XP_PER_LANDED_HIT, s.xp[COMBAT], &paid[..20], &paid[20..]), (10, 200, &[10; 20][..], &[0; 5][..]));
    }
    #[test] fn sc1_s8_a_25_strike_first_kill_finds_the_tend_hook() {
        let find = |n| { let mut f = FoundFinds::default(); assert!(f.first_find(n, PlaceId::Sanctuary as u32)); f.names[0] };
        assert_eq!((find(sc1_kill(None).0), find(14)), ("Yard-Worn Tend Hook", "Steady Climate Pick"));
    }
    #[test] fn sc1_s9_an_8_rise_flashes_and_reads_8_in_combat_text() {
        let ((mut app, _), mut q) = (flash_app(), crate::combat_text::CombatTextQueue::default());
        app.world_mut().resource_mut::<CombatState>().predator_taken = 8; crate::combat_text::hook_player_strike(&mut q, PlaceId::Heartwood, 0, 8, true, Vec3::ZERO);
        assert!(step(&mut app, 0.0) == (HIT_KICK, 1) && q.events.len() == 1 && (q.events[0].amount, q.events[0].kind) == (8, crate::combat_text::CombatTextKind::Dealt));
    }
    #[test] fn sc1_plate_pin_plated_lunge_11_and_12_on_unbroken() {
        let plated = |t| { let mut c = CombatState::default(); predator_tick_plated(&mut c, LUNGE_SECS, true, true, t, true); c.player_taken };
        assert_eq!((plate_hit(15), crate::difficulty::TIERS.map(plated)), (11, [11, 11, 11, 12]));
    }

    // STRIKE-FACING-1 — CUT 7059108080 + AMEND 7059114075: a strike lands only inside ±60° of the body's +Z; reach, Plate and the lunge stay distance-only.
    fn sf1_yaw(app: &mut App, deg: f32) { for mut t in app.world_mut().query_filtered::<&mut Transform, With<crate::human_presence::HumanPresence>>().iter_mut(app.world_mut()) { t.rotation = Quat::from_rotation_y(deg.to_radians()); } }
    /// The yard 2 m on −Z of the predator home (the home is dead ahead at yaw 0), with one body at `deg`.
    fn sf1<R>(tag: &str, at: [f32; 2], deg: f32, body: impl FnOnce(&mut App) -> R) -> R {
        yard(tag, HudPresetId::Classic, PlaceId::Heartwood, DifficultyTier::Sheltered, [PREDATOR_HOME[0] + at[0], PREDATOR_HOME[1] + at[1]], |app| { app.world_mut().spawn((crate::human_presence::HumanPresence, Transform::default())); sf1_yaw(app, deg); body(app) })
    }
    fn sf1_hit(app: &mut App, keys: &[KeyCode]) -> (u32, u32) { cc2a_keys(app, keys); let c = app.world().resource::<CombatState>(); (c.strikes, c.predator_taken) }
    fn sf1_at(deg: f32) -> u32 { sf1(&format!("sf1d{deg}"), [0.0, -2.0], deg, |app| sf1_hit(app, &[KeyCode::Digit1]).1) }
    #[test] fn sf1_f1_facing_away_in_reach_is_a_counted_miss() {
        sf1("sf1f1", [0.0, -2.0], 180.0, |app| { assert_eq!(sf1_hit(app, &[KeyCode::Digit1]), (1, 0)); assert_eq!(app.world().resource::<FirstHarvestEpiphany>().pulse_line, format!("Missed {PREDATOR_NAME}")); });
    }
    #[test] fn sf1_f2_facing_the_predator_lands_8() { assert_eq!(sf1_at(0.0), 8); }
    #[test] fn sf1_f3_59_lands_61_and_180_miss() { assert_eq!([59.0, 61.0, 180.0, -59.0, -61.0].map(sf1_at), [8, 0, 0, 8, 0]); }
    #[test] fn sf1_f4_within_a_tenth_of_a_metre_or_no_body_counts_as_in_front() {
        assert_eq!(sf1("sf1f4a", [0.0, -0.05], 180.0, |app| sf1_hit(app, &[KeyCode::Digit1])), (1, 8));
        assert_eq!(yard("sf1f4b", HudPresetId::Classic, PlaceId::Heartwood, DifficultyTier::Sheltered, [PREDATOR_HOME[0], PREDATOR_HOME[1] - 2.0], |app| sf1_hit(app, &[KeyCode::Digit1])), (1, 8));
    }
    #[test] fn sf1_f5_modern_and_classic_read_the_same_body() {
        let run = |p: crate::human_presence::ControlPreset| [0.0, 180.0].map(|d| sf1("sf1f5", [0.0, -2.0], d, |app| { app.insert_resource(p); sf1_hit(app, &[KeyCode::Digit1]).1 }));
        assert_eq!(run(crate::human_presence::ControlPreset::Modern), run(crate::human_presence::ControlPreset::Classic));
        let prod = include_str!("combat_core.rs").split("\n#[cfg(test)]\nmod tests {").next().unwrap();
        assert!(["ControlPreset", "FollowOrbit", "BodyFacing", "Camera3d"].iter().all(|w| !prod.contains(w)));
    }
    #[test] fn sf1_f6_key1_lmb_and_rt2_agree() {
        let press = |deg: f32, how: usize| sf1(&format!("sf1f6{how}"), [0.0, -2.0], deg, |app| {
            app.init_resource::<ButtonInput<MouseButton>>(); app.world_mut().spawn(Gamepad::default());
            if how == 1 { app.world_mut().resource_mut::<ButtonInput<MouseButton>>().press(FIRE); }
            if how == 2 { app.world_mut().query::<&mut Gamepad>().single_mut(app.world_mut()).unwrap().digital_mut().press(FIRE_PAD); }
            sf1_hit(app, if how == 0 { &[KeyCode::Digit1] } else { &[] })
        });
        for deg in [0.0, 180.0] { let r = [0, 1, 2].map(|h| press(deg, h)); assert!(r[0] == r[1] && r[1] == r[2] && r[0].0 == 1, "{deg}: {r:?}"); }
        assert_eq!(press(0.0, 2), (1, 8));
    }
    #[test] fn sf1_f7_a_same_frame_sync_body_turn_is_read_by_the_strike() {
        sf1("sf1f7", [0.0, 2.0], 0.0, |app| {
            app.set_error_handler(bevy::ecs::error::ignore);
            app.world_mut().resource_mut::<crate::local_settings::LocalSettingsState>().inner.control_preset = "classic".into();
            app.add_plugins(crate::human_presence::HumanPresencePlugin);
            app.world_mut().resource_mut::<SoftPresence>().velocity = Vec3::new(0.0, 0.0, -0.5);
            assert_eq!(sf1_hit(app, &[KeyCode::Digit1]), (1, HIT), "Classic BodyFacing yaw π turns the body to the home on the press frame");
        });
    }
    #[test] fn sf1_f8_droid_still_takes_8_and_the_predator_nothing() {
        sf1("sf1f8", [crate::droid::DROID_HOME[0] - PREDATOR_HOME[0], crate::droid::DROID_HOME[1] - PREDATOR_HOME[1]], 180.0, |app| {
            app.add_plugins(crate::droid::DroidPlugin);
            assert_eq!(sf1_hit(app, &[KeyCode::Digit1]), (1, 0));
            assert_eq!(app.world().resource::<crate::droid::DroidState>().taken, 8);
        });
    }
    #[test] fn sf1_f10_one_facing_helper_kept_out_of_in_reach_and_plate() {
        let prod = include_str!("combat_core.rs").split("\n#[cfg(test)]\nmod tests {").next().unwrap();
        let (reach, fire) = (&prod[prod.find("pub fn in_reach(").unwrap()..], &prod[prod.find("pub fn combat_fire(").unwrap()..]); let reach = &reach[..reach.find("\n}").unwrap()];
        assert!(prod.matches("fn facing_ok(").count() <= 1 && !reach.contains("facing") && !fire[..fire.find("plate_arm(&mut combat, reach").unwrap()].contains("facing_ok("));
    }
    /// 1/60 s steps; the predator damage after each.
    fn sf1_steps(app: &mut App, n: usize) -> u32 { (0..n).for_each(|_| tick(app, 1.0 / 60.0)); app.world().resource::<CombatState>().predator_taken }
    #[test] fn sf1_f11_buffered_press_lands_once_on_a_turn_inside_the_window_and_never_after() {
        sf1("sf1f11a", [0.0, -2.0], 180.0, |app| { app.init_resource::<StrikeBuffer>(); cc2a_keys(app, &[KeyCode::Digit1]); sf1_steps(app, 3); sf1_yaw(app, 0.0); assert_eq!(sf1_steps(app, 3), 8, "turned at 0.05 s"); assert_eq!(sf1_steps(app, 30), 8, "one hit"); });
        sf1("sf1f11b", [0.0, -2.0], 180.0, |app| { app.init_resource::<StrikeBuffer>(); cc2a_keys(app, &[KeyCode::Digit1]); let late = sf1_steps(app, 12); sf1_yaw(app, 0.0); assert_eq!(sf1_steps(app, 30), late, "turned at 0.2 s"); });
    }
    #[test] fn sf1_f12_plate_arms_the_same_facing_away() {
        sf1("sf1f12", [0.0, -2.0], 180.0, |app| {
            let mut f = app.world_mut().resource_mut::<FoundFinds>(); f.names = shared::temper::NAMED_FINDS.iter().map(|n| n.name).collect(); assert!(f.cycle_equipped()); drop(f);
            cc2a_press_at(app, CC2A_TELL);
            let c = app.world().resource::<CombatState>(); assert!(c.plate_armed && c.plate_secs > 0.0, "{c:?}");
        });
    }

    // ABILITY-TOOLTIP-SHOW-1 R1–R10 — CUT 7066278335 / ruling 7063122896: the AbilityTip slab shows slot 1's canon Plate text only
    // with the Shell ward equipped and the fight in reach; HudLayoutPlugin owns the final Visibility (R5, R2, §6.3).
    use crate::hud_anchor_registry::{ID_ABILITY_TIP, ID_CARE_STRIP};
    const ATS_PLATE: &str = "Plate, 3 s, incoming damage reduced by 30%.";
    /// Each AbilityTip slab: (final Visibility, its child texts).
    fn ats_tips(app: &mut App) -> Vec<(Visibility, Vec<String>)> {
        let w = app.world_mut();
        let roots: Vec<_> = w.query::<(Entity, &HudSlab, &Visibility)>().iter(w).filter(|(_, s, _)| s.0 == ID_ABILITY_TIP).map(|(e, _, v)| (e, *v)).collect();
        roots.into_iter().map(|(e, v)| (v, w.query::<(&Text, &ChildOf)>().iter(w).filter(|(_, c)| c.parent() == e).map(|(t, _)| t.as_str().to_string()).collect())).collect()
    }
    fn ats_all(app: &mut App, want: Visibility) -> bool { ats_tips(app).iter().all(|t| t.0 == want) }
    fn ats_yielded(app: &App) -> bool { app.world().resource::<HudYielded>().0.contains(&ID_ABILITY_TIP) }
    /// The cc2a yard at the Heartwood home (in reach), Shell equipped or not, Guidance dismissed so only the case under test yields.
    fn ats<R>(tag: &str, shell: bool, body: impl FnOnce(&mut App) -> R) -> R {
        cc2a(tag, DifficultyTier::Sheltered, shell.then_some(0), |app| { app.world_mut().resource_mut::<FirstSessionGuidance>().dismiss(); tick(app, 0.0); tick(app, 0.0); body(app) })
    }

    #[test]
    fn ats1_r1_one_ability_tip_slab_spawns() {
        yard("ats1r1", HudPresetId::Classic, PlaceId::Heartwood, DifficultyTier::Sheltered, PREDATOR_HOME, |app| assert_eq!(ats_tips(app).len(), 1));
    }
    #[test]
    fn ats1_r2_shell_in_reach_classic_final_visible() {
        ats("ats1r2", true, |app| { assert_eq!(ats_tips(app).iter().map(|t| t.0).collect::<Vec<_>>(), [Visibility::Visible]); assert!(!ats_yielded(app)); });
    }
    #[test]
    fn ats1_r3_child_text_is_the_canon_plate_line() {
        ats("ats1r3", true, |app| assert_eq!(ats_tips(app), [(Visibility::Visible, vec![ATS_PLATE.to_string()])]));
    }
    #[test]
    fn ats1_r4_yields_to_care_strip_and_guidance() {
        ats("ats1r4a", true, |app| {
            app.world_mut().spawn((HudSlab(ID_CARE_STRIP), Visibility::Visible, Node::default()));
            tick(app, 0.0);
            assert!(ats_all(app, Visibility::Hidden) && ats_yielded(app) == !ats_tips(app).is_empty(), "CareStrip shown");
        });
        cc2a("ats1r4b", DifficultyTier::Sheltered, Some(0), |app| {
            tick(app, 0.0);
            assert!(app.world().resource::<FirstSessionGuidance>().active && !app.world().resource::<FirstSessionGuidance>().dismissed);
            assert!(ats_all(app, Visibility::Hidden) && ats_yielded(app) == !ats_tips(app).is_empty(), "Guidance shown");
        });
    }
    #[test]
    fn ats1_r5_places_plate_hides_then_restores() {
        ats("ats1r5", true, |app| {
            app.world_mut().resource_mut::<PlacesPlate>().open = true;
            tick(app, 0.0);
            assert!(ats_all(app, Visibility::Hidden), "R5 modal");
            app.world_mut().resource_mut::<PlacesPlate>().open = false;
            tick(app, 0.0);
            assert!(ats_all(app, Visibility::Visible), "restored the next frame");
        });
    }
    #[test]
    fn ats1_r6_minimal_and_management_show_it() {
        for (tag, preset) in [("ats1r6m", HudPresetId::Minimal), ("ats1r6g", HudPresetId::Management)] {
            ats(tag, true, |app| { app.insert_resource(ActiveHudPreset { id: Some(preset) }); tick(app, 0.0); tick(app, 0.0); assert!(ats_all(app, Visibility::Visible) && !ats_yielded(app), "{tag}"); });
        }
    }
    #[test]
    fn ats1_r7_no_preset_keeps_the_writer_value() {
        ats("ats1r7", true, |app| {
            app.insert_resource(ActiveHudPreset { id: None });
            tick(app, 0.0);
            assert!(ats_all(app, Visibility::Visible) && app.world().resource::<HudYielded>().0.is_empty());
            app.world_mut().resource_mut::<SoftPresence>().position = Vec3::new(PREDATOR_HOME[0] + 2.0 * REACH, 0.9, PREDATOR_HOME[1]);
            tick(app, 0.0);
            assert!(ats_all(app, Visibility::Hidden), "out of reach, no preset");
        });
    }
    #[test]
    fn ats1_r8_slot_one_text_is_the_canon_plate_line() {
        assert_eq!(slot_tooltip(1), Some(ATS_PLATE.to_string()));
        assert!(docs_combat_456().contains(&format!("| **Bulwark** | {ATS_PLATE} ")));
    }
    fn docs_combat_456() -> String { include_str!("../../docs/design/COMBAT_AND_PVP.md").lines().nth(455).unwrap_or_default().to_string() }
    #[test]
    fn ats1_r9_budget_44_banned_words_and_plate_values() {
        for t in slot_tooltip(1).iter().chain([ATS_PLATE.to_string()].iter()) {
            assert!(t.chars().count() <= 44, "{t}");
            assert!(t.to_lowercase().split(|c: char| !c.is_alphanumeric()).all(|w| !["class", "soon", "coming", "unlock", "cooldown"].contains(&w)), "{t}"); // AMEND 7066441253: whole words; "incoming" is the canon word (COMBAT_AND_PVP.md:456)
        }
        assert!(slot_tooltip(2).is_none() && slot_tooltip(3).is_none() && PLATE_SECS == 3.0 && plate_hit(100) == 70);
    }
    #[test]
    fn ats1_r10_no_shell_or_out_of_reach_stays_hidden() {
        ats("ats1r10a", false, |app| { tick(app, 0.0); assert!(ats_all(app, Visibility::Hidden) && !ats_yielded(app), "no Shell, in reach"); });
        ats("ats1r10b", true, |app| {
            app.world_mut().resource_mut::<SoftPresence>().position = Vec3::new(PREDATOR_HOME[0] + 2.0 * REACH, 0.9, PREDATOR_HOME[1]);
            tick(app, 0.0);
            assert!(ats_all(app, Visibility::Hidden) && !ats_yielded(app), "Shell, out of reach");
            app.world_mut().resource_mut::<SoftPresence>().position = Vec3::new(PREDATOR_HOME[0], 0.9, PREDATOR_HOME[1]);
            app.world_mut().resource_mut::<CombatState>().predator_taken = HEALTH;
            tick(app, 0.0);
            assert!(ats_all(app, Visibility::Hidden), "Shell, predator down");
        });
    }

    // TIP-NEQ-1 N1 — CUT 7066705477: a steady, unyielded tip is not re-marked Changed each frame (Last-schedule tape on the tip).
    #[derive(Resource, Default)] struct TnTape(Vec<bool>);
    fn tn_tape(q: Query<(&HudSlab, Ref<Visibility>)>, mut tape: ResMut<TnTape>) { tape.0.extend(q.iter().filter(|(s, _)| s.0 == ID_ABILITY_TIP).map(|(_, v)| v.is_changed())); }
    #[test]
    fn tn1_n1_steady_tip_is_not_marked_changed() {
        for (tag, shell, want) in [("tn1a", true, Visibility::Visible), ("tn1b", false, Visibility::Hidden)] {
            ats(tag, shell, |app| {
                app.init_resource::<TnTape>().add_systems(Last, tn_tape);
                tick(app, 0.0); tick(app, 0.0); app.world_mut().resource_mut::<TnTape>().0.clear(); // warm-up: first run and Added read as changed
                tick(app, 0.0); tick(app, 0.0);
                assert!(ats_all(app, want) && !ats_yielded(app), "{tag}: steady and not yielded");
                assert_eq!(app.world().resource::<TnTape>().0, [false, false], "{tag}");
            });
        }
    }
}
