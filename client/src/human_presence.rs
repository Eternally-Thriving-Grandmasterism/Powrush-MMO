/*!
 * Human Presence — v22.12.0
 *
 * Ground, jump, mount, breath, carry.
 * Feel-move: FixedUpdate @ 60 Hz, accel toward wish, stop-on-release (no ice-skate).
 *
 * H-2026-09-11-B (PERSON_READ_SPEC §4): the body is a stacked-capsule person
 * instead of one capsule, and the stance shows what the sim already holds —
 * idle stand, stride, reach-to-glow on the existing Peace **E**, heavy / winded
 * carry. Presentation only: no new verb, no sim write, no HUD surface, so the
 * read survives the **H** hush and the climate slab keeps Place · well mood.
 * Asset budget: capsule / sphere primitives plus the one Sanctuary warm-gold
 * accent at the hand. No mesh, no anim pack, no new texture.
 *
 * H-2026-09-11-E3 (NPC_SCHEDULE_SPEC): local person-read rhymes with LivingDay
 * schedule posts, sacred-five work intent, and Place·mood greet. The local
 * stacked-capsule stays presentation — not an NPC roster. Hour finishes with
 * zero scheduled persons. No mesh dump, no second HUD.
 *
 * H-2026-09-16-MESH-PERSONA-HANDS: Comfort L/M/H [`MeshLodPlan`] drives
 * [`procedural_detail_scale`] on the lived stacked-capsule. Low stays
 * primitives / capsule-readable. High may gate `persona_commit_dress_active`
 * only when an authored glb is already present — no `.glb` dump. face ≠ class.
 * Practices after House. No race lobby. No second HUD.
 * Cite [`docs/MESH_PERSONA_COURT.md`] · [`docs/ASSET_BUDGET_COURT.md`] @ `5eff19c`.
 *
 * CARD L3 PEOPLE-DOOR-LAND — reuse spawn_human_presence. Wake maps
 * PeopleLanding onto existing Place boot / Threshold shelf transform.
 * Do not rewrite the stacked-capsule mesh. 0 meshes.
 *
 * CARD L7 ARRIVAL-BEAT — People-door land plays one short beat from
 * existing fog / camera / SoftPresence. Keyed by PeopleLanding, not a
 * new hex. Ambrosian = lift on Sanctuary disk (FORK A). mothership-over-Earth
 * PRESENTATION; no hull mesh. 0 meshes.
 *
 * CARD F5 WRONG-DOOR-BOUNCE — unsealed light may take a People-door and
 * feel that existing L7 beat. Decline / wrong door seats the garden wake
 * (Sanctuary yard helper) and disarms the beat. No L7 fog / camera WRITE.
 * No fifth Place · no Title race lobby.
 *
 * CARD OPT-REDUCED-MOTION-PULSE — camera punch and the hand-accent kick
 * follow `camera_punch_scale`. Reduced motion (Comfort Low sets the flag)
 * holds that punch at rest. No new verb. No second HUD.
 *
 * CARD OPT-PUNCH-LOW — Graphics Low multiplies camera punch amplitude by a
 * modest cap. Medium and High stay full. A punch scale of 0 (reduced motion)
 * stays 0: reduced motion wins over the Low cap. No Ultra. No rumble rewrite.
 *
 * CARD CAM-MOUSE-ORBIT-1 — hold the right mouse button to orbit the follow
 * camera (yaw, pitch, distance). The scroll wheel zooms inside a clamp.
 * Defaults match CAM_BACK / CAM_UP. No orbit on the title screen or while
 * settings are open (same gate as STILL-BEFORE-YARD-1). Camera collision
 * stays deferred.
 *
 * CARD CAM-FEEL-PASS-1 — orbit yaw and pitch read `LocalLookFeel`. Sensitivity
 * scales both axes and invert-Y flips pitch. Saved defaults (sensitivity 1,
 * invert off) match the raw per-pixel rates. The follow lerp is
 * `1 - (1 - 0.12)^(dt * 60)`, the old 0.12 weight at 60 fps.
 *
 * CARD CAM-MOUSELOOK-1A — optional mouse-look (default off) orbits while the
 * cursor is grabbed, without holding the right button. Grab is `Locked`
 * (`bevy_winit` falls back to `Confined`) and the cursor is hidden, only when
 * mouse-look is on, the door is InYard, settings are closed, HUD edit is off,
 * and the primary window is focused. Every other frame releases (`None`,
 * visible). The following eligible frame grabs again. mouse_look off leaves
 * the RMB orbit and the cursor untouched.
 *
 * CARD CAM-RAW (PLAYTEST-CAM-1) — `raw_look` (default off) gives 1:1 look:
 * `follow_camera` uses a lerp weight of 1 instead of the follow-lerp
 * smoothing, so the camera lands on the orbit target the same frame. Orbit
 * input is already raw per-pixel and is not changed. The pitch clamp stays.
 * Pinned by `look_follow_t_raw_skips_smoothing`.
 *
 * CARD CAM-COLLIDE-1 — after `follow_camera`, one mesh ray from the follow
 * pivot toward the camera. A closer hit snaps the camera to hit − margin,
 * never under the floor, so the lens does not sit inside a wall. No hit
 * leaves that frame's translation. Easing back out is `follow_camera`'s
 * frame-rate independent lerp (`1 - (1 - 0.12)^(dt * 60)`). Orbit, mouse
 * look, and the glance are not written. Runs only InYard.
 *
 * CARD PAD-LOOK-STICK-1 — the first gamepad's right stick orbits yaw and pitch
 * while that same yard gate is open. Per-axis 0.18 deadzone, rescaled to 0..1.
 * Full deflection is 2.5 rad/s yaw and 1.875 rad/s pitch, times look
 * sensitivity and dt, through `apply_delta` (stick-up looks up). Invert-Y
 * flips pitch only. The mouse block is unchanged. No new setting.
 *
 * CARD MOVE-STRAFE-DEFAULT-1 — Modern (default) rotates WASD, pad, and touch
 * by the follow-orbit yaw. W goes along the camera's forward, A/D strafe,
 * and the body faces that forward while moving. Classic keeps W/S on the
 * body's facing and turns with keyboard A/D; the orbit yaw follows the body.
 * Pad and touch stay camera-relative in both presets.
 *
 * CARD CONTROLS-PRESETS-UI-1 — `ControlPreset` follows LocalSettings
 * `control_preset` at boot and whenever that field changes. Movement
 * logic is unchanged. Missing or unknown stays Modern.
 *
 * CARD KEY-REBIND-1 — `keyboard_axes` reads forward, back, left, and right
 * from the saved Peace key map. Arrow keys stay a fallback only when that
 * arrow is not bound to an action, so one key does not drive two actions.
 * Classic turning uses the mapped left and right keys. Default bindings
 * move as they do today.
 *
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use std::collections::HashSet;
use std::f32::consts::{FRAC_PI_2, PI};

use bevy::ecs::system::SystemParam;
use bevy::input::mouse::{
    AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollPixelsPerLine, MouseScrollUnit,
};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::companion_bond::CompanionBond;
use crate::vegetation::{CamTrunk, TrunkFadeCache};
use crate::hour_sacred::PeopleLanding;
use crate::input::PlayerInput;
use crate::title_screen::{HouseLabel, LaunchDoor};
use crate::harvest_feel::SoftRbePool;
use crate::living_body::{BodyTell, LivingBody};
use crate::living_day::{
    greet_by_place_mood, preferred_work, schedule_copy_is_honest, still_frame_line, DayPeriod,
    PreferredWork, SacredVerb, SchedulePlace, WellMood,
};
use crate::local_settings::{
    LocalFeedbackFeel, LocalLookFeel, LocalMeshLodFeel, LocalSettingsState,
};
use shared::local_settings::{GraphicsPreset, LocalSettings, PeaceKey};

/// H-2026-09-16-MESH-PERSONA-HANDS — Comfort MeshLodPlan (PATHS-only compile).
#[path = "gltf_integration.rs"]
mod mesh_lod;
use crate::mercy_harvest_nodes::{MercyHarvestNode, HARVEST_REACH};
use crate::soft_play_bindings;

const STAND: f32 = 0.90;
const WALK: f32 = 3.4;
const SPRINT: f32 = 5.4;
const GRAVITY: f32 = 18.0;
const JUMP: f32 = 5.6;
/// CARD JUMP-BUFFER-1 — airborne jump press window in seconds (tunable; council-set, Sherif may veto).
pub const JUMP_BUFFER_SECS: f32 = 0.12;
const CAM_BACK: f32 = 6.4;
const CAM_UP: f32 = 2.35;
/// Radians of yaw per mouse pixel while the right button is held.
const ORBIT_YAW_PER_PX: f32 = 0.004;
/// Radians of pitch per mouse pixel while the right button is held.
const ORBIT_PITCH_PER_PX: f32 = 0.003;
/// CARD PAD-LOOK-STICK-1 — full right-stick deflection, radians per second.
/// Pitch keeps the mouse ratio `ORBIT_PITCH_PER_PX / ORBIT_YAW_PER_PX`.
const STICK_LOOK_YAW_RATE: f32 = 2.5;
const STICK_LOOK_PITCH_RATE: f32 = 1.875;
/// Distance change per scroll line. Wheel up (positive y) zooms in.
const ORBIT_DIST_PER_LINE: f32 = 0.85;
/// Matches Bevy's `MouseScrollPixelsPerLine` default when that resource is absent.
const ORBIT_PIXELS_PER_LINE: f32 = 100.0;
const CAM_DIST_MIN: f32 = 2.2;
const CAM_DIST_MAX: f32 = 16.0;
/// Above horizontal, so the camera stays over the ground at every allowed distance.
const CAM_PITCH_MIN: f32 = 0.05;
/// Short of vertical, so `look_at` cannot flip over the pole.
const CAM_PITCH_MAX: f32 = FRAC_PI_2 - 0.08;
/// Follow smoothing at 60 fps. Other rates use `1 - (1 - this)^(dt * 60)`.
const FOLLOW_LERP_AT_60: f32 = 0.12;
/// CARD CAM-COLLIDE-1 — gap kept between a wall hit and the pulled camera.
const CAM_WALL_MARGIN: f32 = 0.3;
/// CARD CAM-COLLIDE-1 — closest the wall pull may place the camera to the pivot.
const CAM_WALL_FLOOR: f32 = 0.6;
/// CARD PT-COLLIDE-1 — player body radius for the trunk push-out. Playtest-tunable placeholder.
const PLAYER_TRUNK_RADIUS: f32 = 0.3;
/// Horizontal accel toward wish speed (units/s²).
pub const MOVE_ACCEL: f32 = 28.0;
/// Horizontal decel when wish is released — stop-on-release, no ice-skate.
pub const MOVE_DECEL: f32 = 48.0;

// --- Person rig ----------------------------------------------------------
// Local frame: the root entity rides at STAND (hip height). Rig front is +Z,
// matching the yaw `sync_body` already writes from the movement direction.
// A positive rotation about X bows forward, so a hanging arm reaches forward
// on a *negative* shoulder pitch.
const HIP_R: f32 = 0.17;
const HIP_LEN: f32 = 0.12;
const WAIST_Y: f32 = -0.05;
const TORSO_R: f32 = 0.205;
const TORSO_LEN: f32 = 0.38;
const TORSO_Y: f32 = 0.33;
const HEAD_R: f32 = 0.15;
const HEAD_Y: f32 = 0.74;
const BROW_R: f32 = 0.055;
const SHOULDER_X: f32 = 0.235;
const SHOULDER_Y: f32 = 0.60;
const ARM_R: f32 = 0.068;
const ARM_LEN: f32 = 0.40;
const ARM_Y: f32 = -0.27;
const HAND_Y: f32 = -0.545;
const LEG_R: f32 = 0.10;
const LEG_LEN: f32 = 0.48;
const LEG_Y: f32 = -0.34;
const HIP_PIVOT_X: f32 = 0.115;
const HIP_PIVOT_Y: f32 = -0.20;
const ACCENT_R: f32 = 0.055;

/// HANDS desaturated earth body. One material family for every part.
const BODY_EARTH: Color = Color::srgb(0.20, 0.26, 0.24);
/// The one Sanctuary accent (ART_BIBLE HANDS lane) — warm gold at the hand.
const SANCTUARY_GOLD: Color = Color::srgb(0.86, 0.66, 0.29);

/// Shoulder swing of the attending arm at a full Use read (radians forward).
pub const REACH_PITCH: f32 = 1.22;
/// Waist bow toward the glow at a full Use read (radians).
pub const REACH_LEAN: f32 = 0.21;
/// Arm swing at full sprint (radians).
pub const STRIDE_PITCH: f32 = 0.52;
/// Leg swing at full sprint (radians).
pub const STRIDE_HIP: f32 = 0.46;
/// The Use read holds this long (seconds) after the tap so a still frame
/// still names Peace E — the tip is support, never the only proof.
pub const ATTEND_LINGER: f32 = 0.55;
/// Horizontal speed (m/s) above which the stance reads as a stride.
pub const STRIDE_SPEED: f32 = 0.35;

#[derive(Component)]
pub struct HumanPresence;

/// Rig joints the pose systems drive. `f32` is the side: -1 left, +1 right.
#[derive(Component, Clone, Copy, Debug)]
enum PersonPart {
    Waist,
    Head,
    Shoulder(f32),
    Hip(f32),
}

/// The single warm-gold accent at the reaching hand.
#[derive(Component)]
struct PersonAccent;

/// Child mesh whose local scale follows Comfort [`mesh_lod::procedural_detail_scale`].
/// Root / joints stay unscaled so feet stay on the plane and Peace E still reads.
#[derive(Component)]
struct PersonLodMesh;

/// What the still frame must name about Peace **E**.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stance {
    /// Standing still, unladen.
    Idle,
    /// Walking or sprinting.
    Stride,
    /// Attending the glow on the existing Use.
    Reach,
    /// Carrying a full pool.
    Heavy,
    /// Out of breath.
    Winded,
}

/// Use read for the body: the same `PlayerInput.interact` Peace hook, latched
/// long enough to be seen, plus the glow it belongs to.
#[derive(Resource, Debug, Default)]
pub struct PersonAttend {
    /// 0..1 — 1 on the Use edge and while Use is held, decaying after.
    pub level: f32,
    /// Nearest glow worth attending, when the yard has one.
    pub toward: Option<Vec3>,
    /// 0..1 nearness to that glow (1 inside arm's reach).
    pub near_glow: f32,
}

/// Walk and breath clocks for the pose. Presentation only.
#[derive(Resource, Debug, Default)]
struct PersonRhythm {
    stride: f32,
    breath: f32,
}

/// World-metre heights of the rig landmarks while standing.
#[derive(Clone, Copy, Debug)]
pub struct Silhouette {
    pub feet: f32,
    pub legs_top: f32,
    pub hips: (f32, f32),
    pub torso: (f32, f32),
    pub head: (f32, f32),
}

/// Where the stacked person sits at STAND — head over torso over stance mass.
pub fn silhouette() -> Silhouette {
    let half_leg = LEG_LEN * 0.5 + LEG_R;
    let half_torso = TORSO_LEN * 0.5 + TORSO_R;
    let half_hip = HIP_LEN * 0.5 + HIP_R;
    let leg_center = STAND + HIP_PIVOT_Y + LEG_Y;
    let torso_center = STAND + WAIST_Y + TORSO_Y;
    let head_center = STAND + WAIST_Y + HEAD_Y;
    Silhouette {
        feet: leg_center - half_leg,
        legs_top: leg_center + half_leg,
        hips: (STAND - half_hip, STAND + half_hip),
        torso: (torso_center - half_torso, torso_center + half_torso),
        head: (head_center - HEAD_R, head_center + HEAD_R),
    }
}

/// CARD VP-CHAR-1 — character dress/material response per preset so the
/// person sits in the shipped golden-hour grade (VP-LIGHT-1 / VP-GRADE-1)
/// instead of reading flat. Material only: no rig, anim, VFX, light or bloom.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CharacterMaterialPlan {
    /// Cloth/body (also the skin-tone head — one family so lineage tint holds).
    pub cloth_roughness: f32,
    /// Cloth Fresnel reflectance: low so the grade's warm key wraps softly.
    pub cloth_reflectance: f32,
    /// Accent (hand gold) metal response.
    pub accent_metallic: f32,
    pub accent_roughness: f32,
}

/// Low/Medium stay cheap and readable: non-metal accent so the Use/Mute
/// emissive read is never darkened. High/Ultra carry a real metal accent.
pub const fn character_material_plan(preset: GraphicsPreset) -> CharacterMaterialPlan {
    match preset {
        GraphicsPreset::Mobile | GraphicsPreset::Low | GraphicsPreset::Medium => {
            CharacterMaterialPlan {
                cloth_roughness: 0.86,
                cloth_reflectance: 0.30,
                accent_metallic: 0.0,
                accent_roughness: 0.42,
            }
        }
        GraphicsPreset::High | GraphicsPreset::Ultra => CharacterMaterialPlan {
            cloth_roughness: 0.82,
            cloth_reflectance: 0.35,
            accent_metallic: 0.65,
            accent_roughness: 0.30,
        },
    }
}

/// Comfort MeshLodPlan → lived stacked-capsule detail scale.
pub fn presence_lod_scale(preset: GraphicsPreset) -> f32 {
    mesh_lod::lived_presence_detail_scale(&mesh_lod::plan_for_preset(preset))
}

/// High PersonaCommit dress only when the plan asks and `asset_present`.
/// Never dumps a `.glb`. Low never dresses.
pub fn presence_dress_active(preset: GraphicsPreset, asset_present: bool) -> bool {
    mesh_lod::persona_commit_dress_active(&mesh_lod::plan_for_preset(preset), asset_present)
}

/// Comfort Low stays primitives / capsule-readable (joints unscaled).
pub fn presence_low_stays_capsule_readable() -> bool {
    let plan = mesh_lod::plan_for_preset(GraphicsPreset::Low);
    let scale = mesh_lod::lived_presence_detail_scale(&plan);
    plan.primitives_only
        && !plan.persona_commit_dress
        && !mesh_lod::persona_commit_dress_active(&plan, true)
        && presence_still_capsule_readable(scale)
}

/// Standing stack law is Medium joints; part scale thins/fills, does not drop feet.
pub fn presence_still_capsule_readable(scale: f32) -> bool {
    let s = silhouette();
    scale > 0.5
        && scale <= 1.15 + f32::EPSILON
        && s.feet.abs() < 0.06
        && s.head.1 - s.feet > 1.6
        && s.head.0 < s.torso.1
        && s.torso.0 < s.hips.1
        && HEAD_R < TORSO_R
}

/// face ≠ class — presence never picks a class from a face / people label.
pub fn presence_face_picks_class() -> bool {
    !mesh_lod::face_is_not_class()
}

/// Practices after House — never grey Peace E / I / H / R.
pub fn presence_practices_gate_peace() -> bool {
    !mesh_lod::practices_after_house()
}

/// No Title race lobby from this body.
pub fn presence_opens_race_lobby() -> bool {
    !mesh_lod::race_lobby_closed()
}

fn lod_mesh_tf(x: f32, y: f32, z: f32, scale: f32) -> Transform {
    Transform::from_xyz(x, y, z).with_scale(Vec3::splat(scale))
}

fn ease(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Name the stance a still frame should read. Use wins: the verb is the point.
pub fn stance_read(speed: f32, attend: f32, tell: BodyTell) -> Stance {
    if attend > 0.12 {
        return Stance::Reach;
    }
    match tell {
        BodyTell::Winded => Stance::Winded,
        BodyTell::Heavy => Stance::Heavy,
        BodyTell::Easy if speed > STRIDE_SPEED => Stance::Stride,
        BodyTell::Easy => Stance::Idle,
    }
}

/// Waist bow: the reach folds the chest toward the glow, the load folds it down.
pub fn waist_pitch(attend: f32, tell: BodyTell) -> f32 {
    REACH_LEAN * ease(attend) + tell.waist_load()
}

/// Head tilt: eyes go to the glow on Use, chin drops under load.
pub fn head_pitch(attend: f32, tell: BodyTell) -> f32 {
    0.26 * ease(attend) + tell.waist_load() * 0.35
}

/// Knee give (metres the waist drops) from the reach and the load.
pub fn crouch_drop(attend: f32, tell: BodyTell) -> f32 {
    0.03 * ease(attend) + tell.knee_give()
}

/// Shoulder pitch for one arm. Negative swings the hand forward to the glow.
pub fn shoulder_pitch(side: f32, speed: f32, phase: f32, attend: f32) -> f32 {
    let a = ease(attend);
    let offset = if side > 0.0 { 0.0 } else { PI };
    let swing = STRIDE_PITCH * (speed / SPRINT).clamp(0.0, 1.0) * (phase + offset).sin();
    // The right arm leads the reach; the left trails so the pose is not a shrug.
    let reach = if side > 0.0 {
        -REACH_PITCH
    } else {
        -REACH_PITCH * 0.38
    };
    reach * a + swing * (1.0 - a)
}

/// Hip pitch for one leg — opposite the same-side arm, planted while attending.
pub fn hip_pitch(side: f32, speed: f32, phase: f32, attend: f32) -> f32 {
    let offset = if side > 0.0 { PI } else { 0.0 };
    let swing = STRIDE_HIP * (speed / SPRINT).clamp(0.0, 1.0) * (phase + offset).sin();
    swing * (1.0 - ease(attend))
}

/// Hold the Use read through the linger so a still frame catches the verb.
pub fn attend_level(current: f32, fired: bool, held: bool, dt: f32) -> f32 {
    if fired || held {
        return 1.0;
    }
    (current - dt / ATTEND_LINGER).max(0.0)
}

/// 1 inside arm's reach of a glow, fading to 0 one reach further out.
pub fn glow_proximity(distance: f32) -> f32 {
    if distance <= HARVEST_REACH {
        return 1.0;
    }
    (1.0 - (distance - HARVEST_REACH) / HARVEST_REACH).clamp(0.0, 1.0)
}

/// Warm-gold hand accent: brightest attending a glow, never a second biome.
pub fn accent_glow(attend: f32, near_glow: f32, kick: f32) -> f32 {
    let reach = ease(attend) * (0.45 + 0.55 * near_glow.clamp(0.0, 1.0));
    (0.25 + 0.85 * reach + kick.clamp(0.0, 1.0) * 0.35).clamp(0.25, 1.6)
}

/// CARD OPT-PUNCH-LOW — Graphics Low ceiling on camera punch amplitude.
/// Modest. Medium and High leave the scale unchanged. No Ultra.
const GRAPHICS_LOW_PUNCH_AMPLITUDE: f32 = 0.45;

/// CARD OPT-REDUCED-MOTION-PULSE — harvest kick times `camera_punch_scale`.
/// Reduced motion (and Comfort Low) pass scale 0, so the punch is rest.
pub fn scaled_presence_punch(kick: f32, punch_scale: f32) -> f32 {
    let kick = if kick.is_finite() { kick.max(0.0) } else { 0.0 };
    let scale = if punch_scale.is_finite() {
        punch_scale.clamp(0.0, 1.0)
    } else {
        0.0
    };
    kick * scale
}

/// CARD OPT-PUNCH-LOW — Graphics Low multiplies an already-scaled punch.
/// Scale 0 stays 0, so reduced motion wins over this cap.
pub fn punch_scale_for_graphics(punch_scale: f32, preset: GraphicsPreset) -> f32 {
    let scale = if punch_scale.is_finite() {
        punch_scale.clamp(0.0, 1.0)
    } else {
        0.0
    };
    if scale == 0.0 {
        return 0.0;
    }
    match preset {
        GraphicsPreset::Mobile | GraphicsPreset::Low => scale * GRAPHICS_LOW_PUNCH_AMPLITUDE,
        GraphicsPreset::Medium | GraphicsPreset::High | GraphicsPreset::Ultra => scale,
    }
}

/// Camera nudge from a harvest punch. Y lifts, Z pulls in. Scale 0 is static.
pub fn presence_punch_camera_delta(kick: f32, punch_scale: f32) -> Vec3 {
    let punch = scaled_presence_punch(kick, punch_scale);
    Vec3::new(0.0, punch * 0.22, -punch * 0.35)
}

/// Look-at lift that rides the same punch. Scale 0 stays on the rest aim.
pub fn presence_punch_look_lift(kick: f32, punch_scale: f32) -> f32 {
    scaled_presence_punch(kick, punch_scale) * 0.08
}

/// Yaw that points the rig front (+Z) at a target on the walk plane.
pub fn yaw_toward(from: Vec3, to: Vec3) -> Option<f32> {
    let flat = Vec3::new(to.x - from.x, 0.0, to.z - from.z);
    if flat.length_squared() < 0.01 {
        return None;
    }
    let dir = flat.normalize();
    Some(dir.x.atan2(dir.z))
}

#[derive(Resource, Debug)]
pub struct SoftPresence {
    pub position: Vec3,
    pub velocity: Vec3,
    pub grounded: bool,
    /// PLAYTEST-STRAFE-FIX-1 — `position` at the start of the last fixed tick.
    pub prev_tick: Option<Vec3>,
    /// PLAYTEST-STRAFE-FIX-1 — `position` at the end of the last fixed tick.
    pub curr_tick: Option<Vec3>,
}

impl SoftPresence {
    /// PLAYTEST-STRAFE-FIX-1 — render-side interpolated position.
    ///
    /// `prev_tick.lerp(curr_tick, overstep)`. Falls back to the raw
    /// `position` before the first tick or when something outside
    /// locomotion moved it (teleport / bath / landing), so no stale lerp.
    pub fn render_position(&self, overstep: f32) -> Vec3 {
        match (self.prev_tick, self.curr_tick) {
            (Some(prev), Some(curr)) if curr == self.position => {
                prev.lerp(curr, overstep.clamp(0.0, 1.0))
            }
            _ => self.position,
        }
    }
}

impl Default for SoftPresence {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, STAND, 0.0),
            velocity: Vec3::ZERO,
            grounded: true,
            prev_tick: None,
            curr_tick: None,
        }
    }
}

/// CARD L7 — existing Sanctuary yard helper. Human camera eases DOWN onto this.
pub fn sanctuary_yard_wake() -> Vec3 {
    Vec3::new(0.0, STAND, 0.0)
}

/// Ambrosian well-from-above lift on the same Sanctuary disk (FORK A).
/// mothership-over-Earth PRESENTATION; no hull mesh. Above a Peace jump apex
/// so a short look-down beat can read without a new PlaceId.
pub const AMBROSIAN_WELL_LIFT: f32 = 3.4;

/// Y above the yard helper that still reads as the Ambrosian lift (not a jump).
pub const AMBROSIAN_LIFT_DETECT: f32 = 2.0;

/// CARD L7 — camera ease keyed by PeopleLanding. No new Camera3d.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArrivalCameraEase {
    /// Human / SanctuaryYard — ease DOWN onto the existing Sanctuary wake.
    Down,
    /// Ambrosian — ease UP, look-down on the same yard/well. No hull mesh.
    Up,
    /// Quellorian shelf is the culture — no extra lift.
    None,
}

/// CARD L3 / L7 — People-door wake. Reuse spawn_human_presence mesh.
/// Named shelf transform when present; else Place boot (Sanctuary / apply_place origin).
/// Ambrosian: SAME PlaceId::Sanctuary as Human; wake Y is the well-from-above lift
/// (FORK A — mothership-over-Earth PRESENTATION; no hull mesh). Not a planet hex.
pub fn people_landing_wake(landing: PeopleLanding) -> Vec3 {
    match landing {
        PeopleLanding::Threshold => {
            // Shelf is the culture — no extra lift.
            Vec3::from_array(shared::threshold_shelf::THRESHOLD_SHELF_CENTER)
        }
        PeopleLanding::SanctuaryYard => sanctuary_yard_wake(),
        PeopleLanding::SanctuaryWellFromAbove => {
            sanctuary_yard_wake() + Vec3::Y * AMBROSIAN_WELL_LIFT
        }
        PeopleLanding::Heartwood | PeopleLanding::DepthsTealWayHome => sanctuary_yard_wake(),
    }
}

/// SoftPresence.y still reads as the Ambrosian Sanctuary lift (short beat).
pub fn presence_reads_ambrosian_lift(position: Vec3) -> bool {
    position.y > sanctuary_yard_wake().y + AMBROSIAN_LIFT_DETECT
}

/// CARD L7 — one arrival-beat camera ease. Quellorian stays None (shelf height).
pub fn arrival_camera_ease(landing: PeopleLanding) -> ArrivalCameraEase {
    match landing {
        PeopleLanding::SanctuaryYard => ArrivalCameraEase::Down,
        PeopleLanding::SanctuaryWellFromAbove => ArrivalCameraEase::Up,
        PeopleLanding::Threshold => ArrivalCameraEase::None,
        PeopleLanding::Heartwood | PeopleLanding::DepthsTealWayHome => ArrivalCameraEase::Down,
    }
}

/// CARD L7 — skip House / no land → no beat flag. Land → one beat, no new mesh.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ArrivalBeat {
    pub armed: bool,
    pub landing: Option<PeopleLanding>,
}

/// Beat flag after apply_people_landing. Skip House keeps this unarmed.
pub fn arrival_beat_after_land(landing: Option<PeopleLanding>) -> ArrivalBeat {
    match landing {
        Some(land) => ArrivalBeat {
            armed: true,
            landing: Some(land),
        },
        None => ArrivalBeat::default(),
    }
}

/// Run the presentation beat (camera / fog tokens already in ClimatePlanePlugin).
/// 0 meshes. Garden title light yields — lived Place is the level.
pub fn run_arrival_beat(landing: PeopleLanding) -> ArrivalBeat {
    let _ease = arrival_camera_ease(landing);
    arrival_beat_after_land(Some(landing))
}

/// Seat SoftPresence at the People-door wake. Mesh spawn stays spawn_human_presence.
/// Ambrosian sits the lift Y so existing follow_camera / FogSettings can play the beat.
pub fn wake_people_landing(presence: &mut SoftPresence, landing: PeopleLanding) {
    presence.position = people_landing_wake(landing);
    presence.velocity = Vec3::ZERO;
    presence.grounded = !presence_reads_ambrosian_lift(presence.position);
}

/// CARD F5 — garden bounce wake. Existing Sanctuary yard helper. Not an L7 land.
pub fn garden_bounce_wake() -> Vec3 {
    sanctuary_yard_wake()
}

/// CARD F5 — decline / wrong door seats the garden wake. Drops Ambrosian lift.
/// Does not run [`run_arrival_beat`].
pub fn wake_garden_bounce(presence: &mut SoftPresence) {
    presence.position = garden_bounce_wake();
    presence.velocity = Vec3::ZERO;
    presence.grounded = true;
}

/// CARD F5 — bounce does not arm the L7 arrival beat.
pub fn wrong_door_bounce_disarms_arrival_beat() -> ArrivalBeat {
    arrival_beat_after_land(None)
}

/// Latch jump from Update input so FixedUpdate never multi-fires or misses the edge.
#[derive(Resource, Debug, Default)]
struct JumpLatch {
    pending: bool,
    /// CARD JUMP-BUFFER-1 — seconds left on an airborne press; decrement on airborne ticks only.
    buffer: f32,
    /// Set only by the STAND clamp in `apply_locomotion`; read and cleared every tick.
    clamp_landed: bool,
}

pub struct HumanPresencePlugin;

impl Plugin for HumanPresencePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SoftPresence>()
            .init_resource::<JumpLatch>()
            .init_resource::<PersonAttend>()
            .init_resource::<PersonRhythm>()
            .init_resource::<FollowOrbit>()
            .init_resource::<ControlPreset>()
            .init_resource::<BodyFacing>()
            .init_resource::<MoveKeySnapshot>()
            .init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<AccumulatedMouseMotion>()
            .init_resource::<AccumulatedMouseScroll>()
            .add_systems(
                PreUpdate,
                snapshot_move_keys
                    .after(bevy::input::InputSystems)
                    .before(crate::input::PeaceRemapSet),
            )
            .add_systems(Startup, (spawn_human_presence, seed_control_preset_from_settings))
            .add_systems(
                Update,
                (
                    (
                        apply_presence_mesh_lod,
                        arm_jump_latch,
                        latch_use_attend,
                        sync_control_preset_from_settings,
                        sync_body,
                        pose_person,
                        sync_mouse_look_grab,
                        follow_camera,
                    )
                        .chain(),
                    pull_camera_from_walls.after(follow_camera),
                ),
            )
            .add_systems(FixedUpdate, apply_locomotion);
    }
}

fn spawn_human_presence(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    existing: Query<Entity, With<HumanPresence>>,
    feel: Option<Res<LocalMeshLodFeel>>,
) {
    if existing.iter().next().is_some() {
        return;
    }
    let preset = feel.map(|f| f.preset).unwrap_or(GraphicsPreset::Medium);
    let plan = mesh_lod::plan_for_preset(preset);
    let scale = mesh_lod::lived_presence_detail_scale(&plan);
    let _dress = mesh_lod::persona_commit_dress_active(
        &plan,
        mesh_lod::optional_authored_glb_present(),
    );
    // One material family for every part, so the lineage tint still paints the
    // whole person from the root handle.
    let mat_plan = character_material_plan(preset);
    let earth = materials.add(StandardMaterial {
        base_color: BODY_EARTH,
        perceptual_roughness: mat_plan.cloth_roughness,
        reflectance: mat_plan.cloth_reflectance,
        ..default()
    });
    let accent = materials.add(StandardMaterial {
        base_color: SANCTUARY_GOLD,
        emissive: LinearRgba::from(SANCTUARY_GOLD) * 0.25,
        perceptual_roughness: mat_plan.accent_roughness,
        metallic: mat_plan.accent_metallic,
        ..default()
    });
    let hips = meshes.add(Capsule3d::new(HIP_R, HIP_LEN));
    let torso = meshes.add(Capsule3d::new(TORSO_R, TORSO_LEN));
    let head = meshes.add(Sphere::new(HEAD_R));
    let brow = meshes.add(Sphere::new(BROW_R));
    let arm = meshes.add(Capsule3d::new(ARM_R, ARM_LEN));
    let leg = meshes.add(Capsule3d::new(LEG_R, LEG_LEN));
    let hand = meshes.add(Sphere::new(ACCENT_R));

    commands
        .spawn((
            (
                Mesh3d(hips),
                MeshMaterial3d(earth.clone()),
                Transform::from_xyz(0.0, STAND, 0.0),
            ),
            HumanPresence,
            Name::new("HumanPresence"),
        ))
        .with_children(|root| {
            root.spawn((
                (
                    Transform::from_xyz(0.0, WAIST_Y, 0.0),
                    Visibility::default(),
                ),
                PersonPart::Waist,
                Name::new("PersonWaist"),
            ))
            .with_children(|waist| {
                waist.spawn((
                    (
                        Mesh3d(torso),
                        MeshMaterial3d(earth.clone()),
                        lod_mesh_tf(0.0, TORSO_Y, 0.0, scale),
                    ),
                    PersonLodMesh,
                ));
                waist
                    .spawn((
                        (
                            Mesh3d(head),
                            MeshMaterial3d(earth.clone()),
                            lod_mesh_tf(0.0, HEAD_Y, 0.0, scale),
                        ),
                        PersonPart::Head,
                        PersonLodMesh,
                        Name::new("PersonHead"),
                    ))
                    .with_children(|face| {
                        // Brow nub: which way the person looks reads in a still.
                        // face ≠ class — look direction only, never a race lobby.
                        face.spawn((
                            (
                                Mesh3d(brow),
                                MeshMaterial3d(earth.clone()),
                                lod_mesh_tf(0.0, 0.01, HEAD_R * 0.86, scale),
                            ),
                            PersonLodMesh,
                        ));
                    });
                for side in [-1.0f32, 1.0] {
                    waist
                        .spawn((
                            (
                                Transform::from_xyz(
                                    side * SHOULDER_X,
                                    SHOULDER_Y,
                                    0.0,
                                ),
                                Visibility::default(),
                            ),
                            PersonPart::Shoulder(side),
                            Name::new("PersonShoulder"),
                        ))
                        .with_children(|limb| {
                            limb.spawn((
                                (
                                    Mesh3d(arm.clone()),
                                    MeshMaterial3d(earth.clone()),
                                    lod_mesh_tf(0.0, ARM_Y, 0.0, scale),
                                ),
                                PersonLodMesh,
                            ));
                            if side > 0.0 {
                                limb.spawn((
                                    (
                                        Mesh3d(hand.clone()),
                                        MeshMaterial3d(accent.clone()),
                                        lod_mesh_tf(0.0, HAND_Y, 0.0, scale),
                                    ),
                                    PersonAccent,
                                    PersonLodMesh,
                                    Name::new("PersonAccent"),
                                ));
                            }
                        });
                }
            });
            for side in [-1.0f32, 1.0] {
                root.spawn((
                    (
                        Transform::from_xyz(side * HIP_PIVOT_X, HIP_PIVOT_Y, 0.0),
                        Visibility::default(),
                    ),
                    PersonPart::Hip(side),
                    Name::new("PersonHip"),
                ))
                .with_children(|limb| {
                    limb.spawn((
                        (
                            Mesh3d(leg.clone()),
                            MeshMaterial3d(earth.clone()),
                            lod_mesh_tf(0.0, LEG_Y, 0.0, scale),
                        ),
                        PersonLodMesh,
                    ));
                });
            }
        });
    info!(
        target: "powrush::presence",
        "standing person on the climate plane — head, torso, stance mass"
    );
}

/// Apply Comfort MeshLodPlan scale when Esc Graphics changes. Joints stay put.
fn apply_presence_mesh_lod(
    feel: Option<Res<LocalMeshLodFeel>>,
    mut meshes: Query<&mut Transform, With<PersonLodMesh>>,
) {
    let Some(feel) = feel else {
        return;
    };
    if !feel.is_changed() {
        return;
    }
    let plan = mesh_lod::plan_for_preset(feel.preset);
    let scale = mesh_lod::lived_presence_detail_scale(&plan);
    let _dress = mesh_lod::persona_commit_dress_active(
        &plan,
        mesh_lod::optional_authored_glb_present(),
    );
    for mut tf in &mut meshes {
        tf.scale = Vec3::splat(scale);
    }
}

fn preset_from_settings(settings: &LocalSettings) -> ControlPreset {
    if settings.control_preset_is_classic() {
        ControlPreset::Classic
    } else {
        ControlPreset::Modern
    }
}

fn seed_control_preset_from_settings(
    settings: Option<Res<LocalSettingsState>>,
    mut preset: ResMut<ControlPreset>,
) {
    let Some(settings) = settings else {
        return;
    };
    *preset = preset_from_settings(&settings.inner);
}

fn sync_control_preset_from_settings(
    settings: Option<Res<LocalSettingsState>>,
    mut preset: ResMut<ControlPreset>,
) {
    let Some(settings) = settings else {
        return;
    };
    if !settings.is_changed() {
        return;
    }
    let next = preset_from_settings(&settings.inner);
    if *preset != next {
        *preset = next;
    }
}

fn arm_jump_latch(input: Res<PlayerInput>, mut latch: ResMut<JumpLatch>) {
    if input.jump {
        latch.pending = true;
    }
}

/// CARD CONTROLS-PRESETS-UI-1 — follows `LocalSettings.control_preset`.
/// Modern is the serde default, so an old settings file stays Modern.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ControlPreset {
    /// Camera-relative move. While moving, the body faces the camera forward.
    #[default]
    Modern,
    /// W/S along the body's facing. Keyboard A/D turns. Pad and touch do not.
    Classic,
}

/// Radians per second for Classic keyboard A/D.
/// A (and Left) increases body yaw — a left turn when the rig front is +Z.
const CLASSIC_TURN_RATE: f32 = 2.5;

/// Body heading for Classic, and the Modern facing stored while moving.
#[derive(Resource, Debug, Clone, Copy)]
struct BodyFacing {
    yaw: f32,
    /// Classic A/D is held, so a still body still shows the turn.
    turning: bool,
}

impl Default for BodyFacing {
    fn default() -> Self {
        Self {
            // Rig front (+Z) along the default camera forward (-Z).
            yaw: PI,
            turning: false,
        }
    }
}

fn wrap_pi(yaw: f32) -> f32 {
    let tau = PI * 2.0;
    let mut y = (yaw + PI) % tau;
    if y < 0.0 {
        y += tau;
    }
    y - PI
}

/// Physical keys copied before `input.rs` writes them onto the canon WASD keys.
///
/// A custom binding clears its source key. This snapshot is that source, so
/// movement still sees the saved map. Uncaptured (lib tests, first frame
/// before PreUpdate) falls through to `ButtonInput`.
#[derive(Resource, Default)]
struct MoveKeySnapshot {
    pressed: HashSet<KeyCode>,
    captured: bool,
}

fn snapshot_move_keys(
    keyboard: Option<Res<ButtonInput<KeyCode>>>,
    mut snap: ResMut<MoveKeySnapshot>,
) {
    let Some(keyboard) = keyboard else {
        snap.captured = false;
        snap.pressed.clear();
        return;
    };
    snap.pressed.clear();
    snap.pressed.extend(keyboard.get_pressed().copied());
    snap.captured = true;
}

fn move_key_held(
    code: KeyCode,
    snap: Option<&MoveKeySnapshot>,
    keyboard: Option<&ButtonInput<KeyCode>>,
) -> bool {
    if let Some(snap) = snap.filter(|snap| snap.captured) {
        return snap.pressed.contains(&code);
    }
    keyboard.is_some_and(|keyboard| keyboard.pressed(code))
}

/// True when `arrow` is already the saved key for any Peace action.
fn peace_arrow_is_bound(settings: &LocalSettings, arrow: PeaceKey) -> bool {
    settings.key_move_up == arrow
        || settings.key_move_down == arrow
        || settings.key_move_left == arrow
        || settings.key_move_right == arrow
        || settings.key_jump == arrow
        || settings.key_sprint == arrow
        || settings.key_use == arrow
        || settings.key_satchel == arrow
        || settings.key_hide == arrow
        || settings.key_allocate == arrow
}

fn axis_held(
    held: &impl Fn(KeyCode) -> bool,
    settings: &LocalSettings,
    bound: PeaceKey,
    arrow: PeaceKey,
) -> bool {
    if held(soft_play_bindings::peace_key_code(bound)) {
        return true;
    }
    // Arrow fallback only when that arrow is not already bound.
    if bound != arrow && !peace_arrow_is_bound(settings, arrow) {
        return held(soft_play_bindings::peace_key_code(arrow));
    }
    false
}

/// Forward, back, left, and right from the saved Peace map.
///
/// Missing settings use the Peace defaults, so W A S D and the four arrows
/// match the pre-rebind axes. An arrow bound to another action does not also
/// move. Classic A/D turning reads the same left and right axes.
fn keyboard_axes(held: impl Fn(KeyCode) -> bool, settings: Option<&LocalSettings>) -> Vec2 {
    let defaults = LocalSettings::default();
    let settings = settings.unwrap_or(&defaults);
    let mut axes = Vec2::ZERO;
    if axis_held(&held, settings, settings.key_move_up, PeaceKey::ArrowUp) {
        axes.y += 1.0;
    }
    if axis_held(&held, settings, settings.key_move_down, PeaceKey::ArrowDown) {
        axes.y -= 1.0;
    }
    if axis_held(&held, settings, settings.key_move_left, PeaceKey::ArrowLeft) {
        axes.x -= 1.0;
    }
    if axis_held(
        &held,
        settings,
        settings.key_move_right,
        PeaceKey::ArrowRight,
    ) {
        axes.x += 1.0;
    }
    axes
}

/// Pad, touch, and scripted sticks all land in `PlayerInput.movement`.
/// With WASD held, what remains after the key vector is the stick.
fn stick_axes(movement: Vec2, keys: Vec2) -> Vec2 {
    if keys.length_squared() < 1e-8 {
        return movement;
    }
    if movement.length_squared() < 1e-8 {
        return Vec2::ZERO;
    }
    let keys = if keys.length_squared() > 1.0 {
        keys.normalize()
    } else {
        keys
    };
    let extra = movement - keys;
    if extra.length_squared() < 1e-4 {
        Vec2::ZERO
    } else {
        extra
    }
}

fn clamp_planar(wish: Vec3) -> Vec3 {
    if wish.length_squared() > 1.0 {
        wish.normalize()
    } else {
        wish
    }
}

/// Local stick: +x right, +y forward. Yaw 0 keeps today's world frame (W is -Z).
fn camera_relative_wish(movement: Vec2, yaw: f32) -> Vec3 {
    let wish = clamp_planar(Vec3::new(movement.x, 0.0, -movement.y));
    Quat::from_rotation_y(yaw) * wish
}

fn body_forward(yaw: f32) -> Vec3 {
    Quat::from_rotation_y(yaw) * Vec3::Z
}

/// Rig yaw that points +Z along the camera's horizontal forward.
fn camera_forward_yaw(orbit_yaw: f32) -> f32 {
    wrap_pi(orbit_yaw + PI)
}

fn strafe_wish(
    preset: ControlPreset,
    movement: Vec2,
    keys: Vec2,
    orbit_yaw: f32,
    facing_yaw: f32,
) -> Vec3 {
    match preset {
        ControlPreset::Modern => {
            let axes = if movement.length_squared() > 1e-8 {
                movement
            } else {
                keys
            };
            camera_relative_wish(axes, orbit_yaw)
        }
        ControlPreset::Classic => {
            let stick = stick_axes(movement, keys);
            if keys.length_squared() < 1e-8 {
                camera_relative_wish(stick, orbit_yaw)
            } else {
                let mut wish = Vec3::ZERO;
                if keys.y != 0.0 {
                    wish += body_forward(facing_yaw) * keys.y;
                }
                if stick.length_squared() > 1e-8 {
                    wish += camera_relative_wish(stick, orbit_yaw);
                }
                clamp_planar(wish)
            }
        }
    }
}

/// Approach horizontal velocity: accel toward wish, decel to zero on release.
pub fn approach_horizontal(current: Vec2, wish: Vec2, accel: f32, decel: f32, dt: f32) -> Vec2 {
    if wish.length_squared() < 1e-8 {
        let speed = current.length();
        let max_step = decel * dt;
        if speed <= max_step {
            return Vec2::ZERO;
        }
        return current * ((speed - max_step) / speed);
    }
    let delta = wish - current;
    let max_step = accel * dt;
    if delta.length_squared() <= max_step * max_step {
        return wish;
    }
    current + delta.normalize() * max_step
}

fn apply_locomotion(
    input: Res<PlayerInput>,
    time: Res<Time<Fixed>>,
    bond: Option<Res<CompanionBond>>,
    body: Option<Res<LivingBody>>,
    door: Option<Res<LaunchDoor>>,
    house: Option<Res<HouseLabel>>,
    preset: Option<Res<ControlPreset>>,
    mut orbit: Option<ResMut<FollowOrbit>>,
    mut facing: Option<ResMut<BodyFacing>>,
    keyboard: Option<Res<ButtonInput<KeyCode>>>,
    move_keys: Option<Res<MoveKeySnapshot>>,
    settings: Option<Res<LocalSettingsState>>,
    mut latch: ResMut<JumpLatch>,
    mut presence: ResMut<SoftPresence>,
    trees: (Option<Res<crate::living_practice_loop::SoftPlayerRealm>>, Option<Res<crate::vegetation::FieldTrunks>>),
) {
    // PLAYTEST-STRAFE-FIX-1 — snapshot last tick's end before integrating.
    // CARD JUMP-BUFFER-1 — reseat guard, read before anything moves the position this tick.
    let seated = presence.curr_tick == Some(presence.position);
    presence.prev_tick = Some(presence.position);
    let dt = time.delta_secs();
    let sprint_ok = body.as_ref().map(|b| b.can_sprint()).unwrap_or(true);
    let mut speed = if input.sprint && sprint_ok {
        SPRINT
    } else {
        WALK
    };
    if let Some(b) = body.as_ref() {
        speed *= b.carry_mul();
    }
    if bond.map(|b| b.mounted).unwrap_or(false) {
        speed *= 1.28;
    }
    // CARD STILL-BEFORE-YARD-1 — wish and jump only in the open yard.
    // A closed gate zeros horizontal velocity so a walk stops. Gravity still runs below.
    // A missing door or house keeps today's move.
    let yard_step = match (door.as_deref(), house.as_deref()) {
        (Some(door), Some(label)) => *door == LaunchDoor::InYard && !label.settings_open,
        _ => true,
    };
    if yard_step {
        // CARD MOVE-STRAFE-DEFAULT-1 — yaw 0 keeps today's wish (x, 0, -y).
        let preset_now = preset.as_deref().copied().unwrap_or_default();
        let orbit_yaw = orbit.as_deref().map(|o| o.yaw).unwrap_or(0.0);
        let keys = keyboard_axes(
            |code| move_key_held(code, move_keys.as_deref(), keyboard.as_deref()),
            settings.as_deref().map(|state| &state.inner),
        );
        let facing_yaw = facing.as_deref().map(|f| f.yaw).unwrap_or(PI);
        let wish = strafe_wish(preset_now, input.movement, keys, orbit_yaw, facing_yaw);
        if preset_now == ControlPreset::Classic && keys.x != 0.0 {
            if let Some(facing) = facing.as_deref_mut() {
                // A is -1. Subtracting that increases yaw: a left turn.
                facing.yaw = wrap_pi(facing.yaw - keys.x * CLASSIC_TURN_RATE * dt);
                facing.turning = true;
                let followed = wrap_pi(facing.yaw + PI);
                if let Some(orbit) = orbit.as_deref_mut() {
                    orbit.yaw = followed;
                }
            }
        } else if let Some(facing) = facing.as_deref_mut() {
            facing.turning = false;
            if preset_now == ControlPreset::Modern && wish.length_squared() > 1e-8 {
                facing.yaw = camera_forward_yaw(orbit_yaw);
            }
        }
        let wish_vel = Vec2::new(wish.x * speed, wish.z * speed);
        let horiz = approach_horizontal(
            Vec2::new(presence.velocity.x, presence.velocity.z),
            wish_vel,
            MOVE_ACCEL,
            MOVE_DECEL,
            dt,
        );
        presence.velocity.x = horiz.x;
        presence.velocity.z = horiz.y;
    } else {
        presence.velocity.x = 0.0;
        presence.velocity.z = 0.0;
        if let Some(facing) = facing.as_deref_mut() {
            facing.turning = false;
        }
    }
    presence.velocity.y -= GRAVITY * dt;
    // CARD JUMP-BUFFER-1 — the clamp-landing flag is read and cleared on every tick.
    let clamp_landed = std::mem::take(&mut latch.clamp_landed);
    let buffered = yard_step && latch.buffer > 0.0 && clamp_landed && seated;
    if presence.grounded && buffered {
        // Buffered jump on the first grounded tick after a clamp landing: one press, one jump.
        presence.velocity.y = JUMP;
        presence.grounded = false;
        latch.pending = false;
        latch.buffer = 0.0;
    } else if yard_step && presence.grounded && latch.pending {
        presence.velocity.y = JUMP;
        presence.grounded = false;
        latch.pending = false;
        latch.buffer = 0.0;
    } else if !presence.grounded {
        // Airborne: a pending edge arms the buffer, then it decrements on the same tick.
        // Decrement on airborne ticks only. Pending never carries to landing (no auto-hop).
        if yard_step && latch.pending {
            latch.buffer = JUMP_BUFFER_SECS;
        }
        latch.pending = false;
        latch.buffer = if yard_step { latch.buffer - dt } else { 0.0 };
    } else {
        // Grounded tick that does not fire: drop the buffer; a paused grounded press keeps pending.
        latch.buffer = 0.0;
    }
    let velocity = presence.velocity;
    presence.position += velocity * dt;
    // CARD PT-COLLIDE-1 — trunks push out on x/z, only for the tree set shown in this realm.
    use crate::living_ecology::{yard_trees_shown, YARD_TREE_SPOTS, YARD_TRUNK_R};
    let yard = yard_trees_shown(trees.0.and_then(|r| r.current)).then_some(YARD_TREE_SPOTS).into_iter().flatten().map(|spot| (spot.xz(), YARD_TRUNK_R));
    let field = trees.1.iter().flat_map(|f| f.0.iter().copied());
    push_out_of_trunks(&mut presence.position, yard.chain(field));
    if presence.position.y <= STAND {
        presence.position.y = STAND;
        presence.velocity.y = 0.0;
        presence.grounded = true;
        latch.clamp_landed = true;
    }
    presence.curr_tick = Some(presence.position);
}

/// CARD PT-COLLIDE-1 — push `pos` out of each trunk circle on x/z only; `y`, grounded and the jump latch are untouched.
fn push_out_of_trunks(pos: &mut Vec3, trunks: impl IntoIterator<Item = (Vec2, f32)>) {
    for (center, radius) in trunks {
        let (off, reach) = (pos.xz() - center, radius + PLAYER_TRUNK_RADIUS);
        if off.length_squared() < reach * reach {
            let out = center + off.try_normalize().unwrap_or(Vec2::X) * reach;
            (pos.x, pos.z) = (out.x, out.y);
        }
    }
}

/// Latch the existing Peace **E** into a Use read the body can show.
///
/// Reads `PlayerInput.interact` (the same edge `feel_move` folds the ≤120 ms
/// buffer into) and the held canonical Use key so hold-E tend keeps reaching.
/// Writes nothing but presentation state.
fn latch_use_attend(
    time: Res<Time>,
    input: Res<PlayerInput>,
    keyboard: Res<ButtonInput<KeyCode>>,
    presence: Res<SoftPresence>,
    nodes: Query<&Transform, With<MercyHarvestNode>>,
    mut attend: ResMut<PersonAttend>,
) {
    let mut nearest: Option<(Vec3, f32)> = None;
    for tf in &nodes {
        let d = tf.translation.distance(presence.position);
        if nearest.map(|(_, best)| d < best).unwrap_or(true) {
            nearest = Some((tf.translation, d));
        }
    }
    let held = keyboard.pressed(soft_play_bindings::INTERACT);
    attend.level = attend_level(attend.level, input.interact, held, time.delta_secs());
    attend.near_glow = nearest.map(|(_, d)| glow_proximity(d)).unwrap_or(0.0);
    attend.toward = if attend.level > 0.0 {
        nearest.map(|(pos, _)| pos)
    } else {
        None
    };
}

fn sync_body(
    presence: Res<SoftPresence>,
    attend: Res<PersonAttend>,
    preset: Option<Res<ControlPreset>>,
    orbit: Option<Res<FollowOrbit>>,
    facing: Option<Res<BodyFacing>>,
    fixed: Option<Res<Time<Fixed>>>,
    mut q: Query<&mut Transform, With<HumanPresence>>,
) {
    let preset = preset.as_deref().copied().unwrap_or_default();
    let overstep = fixed.map(|t| t.overstep_fraction()).unwrap_or(1.0);
    let render_pos = presence.render_position(overstep);
    for mut tf in &mut q {
        tf.translation = render_pos;
        let horiz = Vec3::new(presence.velocity.x, 0.0, presence.velocity.z);
        if horiz.length_squared() > 0.04 {
            let yaw = match preset {
                // True strafe: face the camera forward, not the A/D direction.
                ControlPreset::Modern => {
                    camera_forward_yaw(orbit.as_deref().map(|o| o.yaw).unwrap_or(0.0))
                }
                ControlPreset::Classic => facing.as_deref().map(|f| f.yaw).unwrap_or_else(|| {
                    let dir = horiz.normalize();
                    dir.x.atan2(dir.z)
                }),
            };
            tf.rotation = Quat::from_rotation_y(yaw);
        } else if facing.as_deref().is_some_and(|f| f.turning) {
            if let Some(facing) = facing.as_deref() {
                tf.rotation = Quat::from_rotation_y(facing.yaw);
            }
        } else if let Some(target) = attend.toward {
            // Standing Use: turn to the glow so the reach points at it.
            if let Some(yaw) = yaw_toward(presence.position, target) {
                tf.rotation = tf.rotation.slerp(Quat::from_rotation_y(yaw), 0.22);
            }
        }
    }
}

/// Dress the stacked person with the stance the sim already holds.
fn pose_person(
    time: Res<Time>,
    presence: Res<SoftPresence>,
    body: Option<Res<LivingBody>>,
    pool: Option<Res<SoftRbePool>>,
    feedback: Res<LocalFeedbackFeel>,
    attend: Res<PersonAttend>,
    mut rhythm: ResMut<PersonRhythm>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut parts: Query<(&PersonPart, &mut Transform)>,
    accents: Query<&MeshMaterial3d<StandardMaterial>, With<PersonAccent>>,
) {
    let dt = time.delta_secs();
    let speed = Vec2::new(presence.velocity.x, presence.velocity.z).length();
    let tell = body.as_ref().map(|b| b.tell()).unwrap_or_default();
    let breath_rise = body.as_ref().map(|b| b.breath_rise()).unwrap_or(0.012);
    let breath_rate = body.as_ref().map(|b| b.breath_rate()).unwrap_or(0.95);
    let kick = scaled_presence_punch(
        pool.map(|p| p.kick).unwrap_or(0.0),
        feedback.camera_punch_scale,
    );

    rhythm.stride = (rhythm.stride + dt * speed * 2.2) % (PI * 2.0);
    rhythm.breath = (rhythm.breath + dt * breath_rate * PI * 2.0) % (PI * 2.0);
    let a = attend.level;

    for (part, mut tf) in &mut parts {
        match *part {
            PersonPart::Waist => {
                tf.rotation = Quat::from_rotation_x(waist_pitch(a, tell));
                tf.translation.y = WAIST_Y - crouch_drop(a, tell);
            }
            PersonPart::Head => {
                tf.rotation = Quat::from_rotation_x(head_pitch(a, tell));
                tf.translation.y = HEAD_Y + rhythm.breath.sin() * breath_rise;
            }
            PersonPart::Shoulder(side) => {
                tf.rotation = Quat::from_rotation_x(shoulder_pitch(
                    side,
                    speed,
                    rhythm.stride,
                    a,
                )) * Quat::from_rotation_z(side * tell.arm_flare());
            }
            PersonPart::Hip(side) => {
                tf.rotation = Quat::from_rotation_x(hip_pitch(side, speed, rhythm.stride, a));
            }
        }
    }

    let glow = accent_glow(a, attend.near_glow, kick);
    for handle in &accents {
        if let Some(mut mat) = materials.get_mut(handle) {
            mat.emissive = LinearRgba::from(SANCTUARY_GOLD) * glow;
        }
    }
}

/// CARD CAM-MOUSE-ORBIT-1 — yaw, pitch, and distance around the player.
/// Defaults reconstruct today's CAM_BACK / CAM_UP offset.
#[derive(Resource, Debug, Clone, Copy)]
struct FollowOrbit {
    yaw: f32,
    pitch: f32,
    distance: f32,
}

impl Default for FollowOrbit {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: default_orbit_pitch(),
            distance: default_orbit_distance(),
        }
    }
}

impl FollowOrbit {
    fn offset(self) -> Vec3 {
        orbit_offset(self.yaw, self.pitch, self.distance)
    }
}

fn default_orbit_pitch() -> f32 {
    CAM_UP.atan2(CAM_BACK)
}

fn default_orbit_distance() -> f32 {
    CAM_BACK.hypot(CAM_UP)
}

/// Yaw 0, positive pitch: camera sits on +Z, above the player.
fn orbit_offset(yaw: f32, pitch: f32, distance: f32) -> Vec3 {
    let horiz = distance * pitch.cos();
    Vec3::new(horiz * yaw.sin(), distance * pitch.sin(), horiz * yaw.cos())
}

/// Same gate as STILL-BEFORE-YARD-1. Orbit input only in the open yard.
/// A missing door or house keeps today's follow, so the mouse still applies.
fn orbit_input_open(door: Option<&LaunchDoor>, house: Option<&HouseLabel>) -> bool {
    match (door, house) {
        (Some(door), Some(label)) => *door == LaunchDoor::InYard && !label.settings_open,
        _ => true,
    }
}

fn scroll_lines(scroll: &AccumulatedMouseScroll, pixels_per_line: f32) -> f32 {
    let per = if pixels_per_line.is_finite() && pixels_per_line > 0.0 {
        pixels_per_line
    } else {
        ORBIT_PIXELS_PER_LINE
    };
    match scroll.unit {
        MouseScrollUnit::Line => scroll.delta.y,
        MouseScrollUnit::Pixel => scroll.delta.y / per,
    }
}

/// CARD CAM-MOUSELOOK-1A — grab only in the open, focused yard.
/// Settings and HUD edit keep the cursor free. A lost focus releases it.
fn mouse_look_grab_eligible(
    mouse_look: bool,
    in_yard: bool,
    settings_open: bool,
    hud_edit: bool,
    focused: bool,
) -> bool {
    mouse_look && in_yard && !settings_open && !hud_edit && focused
}

/// `Locked` hides the cursor. `bevy_winit` falls back to `Confined` when the
/// platform rejects `Locked`. A free cursor is `None` and visible.
fn mouse_look_cursor(eligible: bool) -> (CursorGrabMode, bool) {
    if eligible {
        (CursorGrabMode::Locked, false)
    } else {
        (CursorGrabMode::None, true)
    }
}

/// `Locked` is the request. `Confined` still counts: that is the platform fallback.
fn cursor_counts_as_grabbed(mode: CursorGrabMode) -> bool {
    matches!(mode, CursorGrabMode::Locked | CursorGrabMode::Confined)
}

/// Each Update asks again, before `follow_camera`. A menu, HUD edit, title
/// door, or lost focus releases on that frame. The next eligible frame grabs.
fn sync_mouse_look_grab(
    look: Option<Res<LocalLookFeel>>,
    door: Option<Res<LaunchDoor>>,
    house: Option<Res<HouseLabel>>,
    edit: Option<Res<crate::hud_edit_mode::HudEditMode>>,
    mut windows: Query<(&Window, &mut CursorOptions), With<PrimaryWindow>>,
) {
    let mouse_look = look.as_ref().is_some_and(|feel| feel.mouse_look);
    let in_yard = door.as_deref().copied() == Some(LaunchDoor::InYard);
    let settings_open = house.as_deref().is_some_and(|label| label.settings_open);
    let hud_edit = edit.as_deref().is_some_and(|mode| mode.active);
    for (window, mut cursor) in &mut windows {
        let eligible = mouse_look_grab_eligible(
            mouse_look,
            in_yard,
            settings_open,
            hud_edit,
            window.focused,
        );
        let (grab, visible) = mouse_look_cursor(eligible);
        // Write only when the cursor would change, so mouse_look off leaves
        // a default cursor untouched.
        if cursor.grab_mode != grab {
            cursor.grab_mode = grab;
        }
        if cursor.visible != visible {
            cursor.visible = visible;
        }
    }
}

/// Per-axis deadzone, then rescale the live band onto 0..1. Matches the
/// left stick's 0.18 cutoff (`|v| > deadzone`) and removes the edge jump.
fn rescale_stick_axis(value: f32) -> f32 {
    if !value.is_finite() {
        return 0.0;
    }
    let deadzone = crate::input::STICK_DEADZONE;
    let mag = value.abs();
    if mag <= deadzone {
        return 0.0;
    }
    let span = 1.0 - deadzone;
    if span <= 0.0 {
        return 0.0;
    }
    ((mag - deadzone) / span).min(1.0).copysign(value)
}

/// Right-stick look. `(stick x, stick y, dt, look_sens, invert_y)` becomes
/// the same delta `LocalLookFeel::apply_delta` returns for the mouse: pixels
/// after sensitivity, with invert-Y on pitch only. `dy = -stickY` so stick-up
/// looks up. A centred stick or a non-finite `dt` is exactly `(0, 0)`.
fn stick_look_delta(
    stick_x: f32,
    stick_y: f32,
    dt: f32,
    look_sens: f32,
    invert_y: bool,
) -> (f32, f32) {
    if !dt.is_finite() || !look_sens.is_finite() {
        return (0.0, 0.0);
    }
    let x = rescale_stick_axis(stick_x);
    let y = rescale_stick_axis(stick_y);
    if x == 0.0 && y == 0.0 {
        return (0.0, 0.0);
    }
    // Pixel rates that the existing per-pixel orbit turns back into rad/s.
    let raw_dx = x * STICK_LOOK_YAW_RATE * dt / ORBIT_YAW_PER_PX;
    let raw_dy = -y * STICK_LOOK_PITCH_RATE * dt / ORBIT_PITCH_PER_PX;
    LocalLookFeel {
        sensitivity: look_sens,
        invert_y,
        mouse_look: false,
        raw_look: false,
    }
    .apply_delta(raw_dx, raw_dy)
}

/// First connected pad only, same query order as move input. No pad is `(0, 0)`.
fn first_pad_right_stick(pads: &Query<&Gamepad>) -> (f32, f32) {
    let Some(gamepad) = pads.iter().next() else {
        return (0.0, 0.0);
    };
    (
        gamepad.get(GamepadAxis::RightStickX).unwrap_or(0.0),
        gamepad.get(GamepadAxis::RightStickY).unwrap_or(0.0),
    )
}

/// Writes a non-zero stick delta with the same yaw and pitch signs as the mouse.
/// A centred stick does not write. Call this before `apply_orbit_input` so that
/// function's pitch clamp still covers the stick.
fn apply_stick_look(
    orbit: &mut FollowOrbit,
    stick_x: f32,
    stick_y: f32,
    dt: f32,
    look: &LocalLookFeel,
) {
    let (stick_dx, stick_dy) =
        stick_look_delta(stick_x, stick_y, dt, look.sensitivity, look.invert_y);
    if stick_dx.is_finite() && stick_dx != 0.0 {
        orbit.yaw -= stick_dx * ORBIT_YAW_PER_PX;
    }
    if stick_dy.is_finite() && stick_dy != 0.0 {
        orbit.pitch += stick_dy * ORBIT_PITCH_PER_PX;
    }
}

fn apply_orbit_input(
    orbit: &mut FollowOrbit,
    buttons: &ButtonInput<MouseButton>,
    motion: &AccumulatedMouseMotion,
    scroll: &AccumulatedMouseScroll,
    pixels_per_line: f32,
    look: &LocalLookFeel,
    grabbed: bool,
) {
    if buttons.pressed(MouseButton::Right) || (look.mouse_look && grabbed) {
        // Sensitivity scales both axes. Invert-Y flips pitch only.
        // Scroll distance stays on the raw line rate so default feel matches today.
        let (dx, dy) = look.apply_delta(motion.delta.x, motion.delta.y);
        if dx.is_finite() {
            // Mouse right turns the view right: the camera swings toward -X.
            orbit.yaw -= dx * ORBIT_YAW_PER_PX;
        }
        if dy.is_finite() {
            // Mouse down (positive y) raises the camera and looks down at the player.
            orbit.pitch += dy * ORBIT_PITCH_PER_PX;
        }
    }
    let lines = scroll_lines(scroll, pixels_per_line);
    if lines.is_finite() && lines != 0.0 {
        orbit.distance -= lines * ORBIT_DIST_PER_LINE;
    }
    orbit.pitch = orbit.pitch.clamp(CAM_PITCH_MIN, CAM_PITCH_MAX);
    orbit.distance = orbit.distance.clamp(CAM_DIST_MIN, CAM_DIST_MAX);
}

/// `1 - (1 - 0.12)^(dt * 60)`. Exactly `FOLLOW_LERP_AT_60` when `dt` is 1/60.
fn follow_lerp_t(dt: f32) -> f32 {
    if !dt.is_finite() || dt <= 0.0 {
        return 0.0;
    }
    let t = 1.0 - (1.0 - FOLLOW_LERP_AT_60).powf(dt * 60.0);
    if t.is_finite() {
        t.clamp(0.0, 1.0)
    } else {
        1.0
    }
}

/// CARD CAM-RAW — raw look drops the follow-lerp smoothing (weight 1).
fn look_follow_t(dt: f32, raw_look: bool) -> f32 {
    if raw_look {
        1.0
    } else {
        follow_lerp_t(dt)
    }
}

/// Time and the pad query share one param. `follow_camera` is already at
/// Bevy's 16-parameter ceiling.
#[derive(SystemParam)]
struct FollowTimePads<'w, 's> {
    time: Res<'w, Time>,
    fixed: Option<Res<'w, Time<Fixed>>>,
    pads: Query<'w, 's, &'static Gamepad>,
}

fn follow_camera(
    clock: FollowTimePads,
    presence: Res<SoftPresence>,
    pool: Option<Res<SoftRbePool>>,
    feedback: Res<LocalFeedbackFeel>,
    look: Res<LocalLookFeel>,
    mesh_lod: Option<Res<LocalMeshLodFeel>>,
    realm: Option<Res<crate::living_practice_loop::SoftPlayerRealm>>,
    door: Option<Res<LaunchDoor>>,
    house: Option<Res<HouseLabel>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    scroll_px: Option<Res<MouseScrollPixelsPerLine>>,
    mut orbit: ResMut<FollowOrbit>,
    mut cams: Query<&mut Transform, With<Camera3d>>,
    cursors: Query<&CursorOptions, With<PrimaryWindow>>,
) {
    let dt = clock.time.delta_secs();
    let raw_look = look.raw_look;
    // CARD CAM-MOUSE-ORBIT-1 — same yard gate as STILL-BEFORE-YARD-1.
    if orbit_input_open(door.as_deref(), house.as_deref()) {
        let pixels_per_line = scroll_px.map(|px| **px).unwrap_or(ORBIT_PIXELS_PER_LINE);
        let grabbed = cursors
            .iter()
            .any(|cursor| cursor_counts_as_grabbed(cursor.grab_mode));
        let (stick_x, stick_y) = first_pad_right_stick(&clock.pads);
        apply_stick_look(&mut orbit, stick_x, stick_y, dt, &look);
        apply_orbit_input(
            &mut orbit,
            &mouse_buttons,
            &mouse_motion,
            &mouse_scroll,
            pixels_per_line,
            &look,
            grabbed,
        );
    }
    let kick = pool.map(|p| p.kick).unwrap_or(0.0);
    let preset = mesh_lod
        .map(|f| f.preset)
        .unwrap_or(GraphicsPreset::Medium);
    let punch_scale = punch_scale_for_graphics(feedback.camera_punch_scale, preset);
    let punch_delta = presence_punch_camera_delta(kick, punch_scale);
    let look_lift = presence_punch_look_lift(kick, punch_scale);
    // CARD L7 — Ambrosian lift: ease UP and look-down on the existing yard/well.
    // Same Camera3d / SoftPresence. No hull mesh. Human / other lands keep yard follow.
    // PLAYTEST-STRAFE-FIX-1 — follow + look_at the same interpolated point.
    let overstep = clock.fixed.as_ref().map(|t| t.overstep_fraction()).unwrap_or(1.0);
    let pos = presence.render_position(overstep);
    let look_down = presence_reads_ambrosian_lift(pos);
    let desired = if look_down {
        pos + Vec3::new(0.0, CAM_UP + 1.1, CAM_BACK - 1.4)
    } else {
        pos + orbit.offset() + punch_delta
    };
    let look = if look_down {
        Vec3::new(pos.x, 0.45, pos.z)
    } else {
        pos + Vec3::Y * (follow_look_height_for_realm(realm.and_then(|r| r.current)) + look_lift)
    };
    let follow_t = look_follow_t(dt, raw_look);
    for mut cam in &mut cams {
        cam.translation = cam.translation.lerp(desired, follow_t);
        cam.look_at(look, Vec3::Y);
    }
}

/// Distance from the follow pivot after a wall test.
///
/// `want` is the current distance from the pivot to the camera. `hit` is the
/// nearest accepted mesh along that ray. No hit leaves `want`. A hit closer
/// than the camera snaps to `hit - margin`, and the result is never under the floor.
fn pulled_camera_distance(want: f32, hit: Option<f32>) -> f32 {
    let Some(hit) = hit else {
        return want;
    };
    if !want.is_finite() || !hit.is_finite() || hit >= want {
        return want;
    }
    (hit - CAM_WALL_MARGIN).max(CAM_WALL_FLOOR)
}

/// Snap when the wall is closer. Ease back out with the same frame-rate
/// independent weight as CAM-FEEL-PASS-1 when the resolved distance is farther.
fn camera_distance_after_pull(current: f32, resolved: f32, dt: f32) -> f32 {
    if !current.is_finite() || !resolved.is_finite() {
        return current;
    }
    if resolved < current {
        resolved
    } else if resolved > current {
        let t = follow_lerp_t(dt);
        current + (resolved - current) * t
    } else {
        current
    }
}

/// The presence root and every descendant are not walls.
fn follow_ray_keeps_entity(entity: Entity, body: &[Entity]) -> bool {
    !body.contains(&entity)
}

fn extend_presence_body(entity: Entity, children: &Query<&Children>, body: &mut Vec<Entity>) {
    let start = body.len();
    body.push(entity);
    let mut index = start;
    while index < body.len() {
        let node = body[index];
        index += 1;
        if let Ok(kids) = children.get(node) {
            body.extend(kids.iter());
        }
    }
}

/// CARD CAM-COLLIDE-1 — one ray from the follow pivot toward the camera.
///
/// Chained after `follow_camera`. A closer hit snaps translation in
/// (hit − margin, floored) so the camera never spends the frame inside a wall.
/// No hit leaves the translation `follow_camera` just wrote; that system eases
/// back out with [`follow_lerp_t`]. Does not write orbit, yaw, pitch, or rotation,
/// so mouse look and the glance keep the facing they already set. InYard only.
fn pull_camera_from_walls(
    time: Res<Time>,
    presence: Res<SoftPresence>,
    realm: Option<Res<crate::living_practice_loop::SoftPlayerRealm>>,
    door: Option<Res<LaunchDoor>>,
    mut ray_cast: MeshRayCast,
    roots: Query<Entity, With<HumanPresence>>,
    children: Query<&Children>,
    mut cams: Query<&mut Transform, With<Camera3d>>,
    mut body: Local<Vec<Entity>>,
    (trunks, mut trunk_mats, mut fade, mut occluding): (Query<(), With<CamTrunk>>, TrunkMats, FadeRes, Local<Vec<Entity>>),
) {
    occluding.clear();
    if door.as_deref().copied() != Some(LaunchDoor::InYard) {
        // CARD PT-CAM-TRUNK-1 A3 — leaving the yard still restores faded trunks.
        fade_occluding_trunks(&occluding, &mut trunk_mats, &mut fade);
        return;
    }
    body.clear();
    for root in &roots {
        extend_presence_body(root, &children, &mut body);
    }
    // CARD PT-CAM-TRUNK-1 A1 — trunks leave the pull ray in the filter, so a wall behind one still pulls.
    let filter = |entity: Entity| follow_ray_keeps_entity(entity, body.as_slice()) && !trunks.contains(entity);
    let trunk_filter = |entity: Entity| trunks.contains(entity);
    let trunk_settings = MeshRayCastSettings::default().with_filter(&trunk_filter).never_early_exit();
    let settings = MeshRayCastSettings::default()
        .with_filter(&filter)
        .with_visibility(RayCastVisibility::Visible)
        .always_early_exit();
    let pivot = presence.position
        + Vec3::Y * follow_look_height_for_realm(realm.and_then(|realm| realm.current));
    let dt = time.delta_secs();
    for mut cam in &mut cams {
        let to_cam = cam.translation - pivot;
        let current = to_cam.length();
        if !current.is_finite() || current <= f32::EPSILON {
            continue;
        }
        let Some(direction) = Dir3::new(to_cam).ok() else {
            continue;
        };
        // The ray stops at the camera. Anything farther is not between the pivot and the lens.
        let hit = {
            let hits = ray_cast.cast_ray(Ray3d::new(pivot, direction), &settings);
            hits.iter()
                .map(|(_, hit)| hit.distance)
                .find(|distance| distance.is_finite() && *distance < current)
        };
        let ray = Ray3d::new(pivot, direction);
        let hits = ray_cast.cast_ray(ray, &trunk_settings).iter().filter(|(_, hit)| hit.distance < current);
        occluding.extend(hits.map(|(entity, _)| *entity));
        let resolved = pulled_camera_distance(current, hit);
        let distance = camera_distance_after_pull(current, resolved, dt);
        if (distance - current).abs() > f32::EPSILON {
            cam.translation = pivot + *direction * distance;
        }
    }
    fade_occluding_trunks(&occluding, &mut trunk_mats, &mut fade);
}

/// CARD PT-CAM-TRUNK-1 T-b — faded trunk alpha. Playtest-tunable placeholder.
const TRUNK_FADE_ALPHA: f32 = 0.35;

/// CARD PT-CAM-TRUNK-1 H-T2 — the original handle a faded trunk gets back on restore.
#[derive(Component)]
struct TrunkFaded(Handle<StandardMaterial>);

type TrunkMats<'w, 's> = Query<'w, 's, (Entity, &'static mut MeshMaterial3d<StandardMaterial>, Option<&'static TrunkFaded>), With<CamTrunk>>;
type FadeRes<'w, 's> = (Option<ResMut<'w, Assets<StandardMaterial>>>, Option<ResMut<'w, TrunkFadeCache>>, Commands<'w, 's>);

/// CARD PT-CAM-TRUNK-1 T-b — occluding trunks swap to a cached Blend clone; others get their original back.
/// Shared materials are never mutated. No-op without `Assets<StandardMaterial>` (A5).
fn fade_occluding_trunks(occluding: &[Entity], trunk_mats: &mut TrunkMats, fade: &mut FadeRes) {
    let (Some(materials), Some(cache), commands) = fade else { return };
    for (entity, mut mat, faded) in trunk_mats.iter_mut() {
        match (occluding.contains(&entity), faded) {
            (true, None) => {
                let src = mat.0.clone();
                let clone = cache.0.iter().find(|(id, _)| *id == src.id()).map(|(_, h)| h.clone());
                mat.0 = clone.unwrap_or_else(|| {
                    let mut faded = materials.get(&src).cloned().unwrap_or_default();
                    faded.alpha_mode = AlphaMode::Blend;
                    faded.base_color.set_alpha(TRUNK_FADE_ALPHA);
                    let handle = materials.add(faded);
                    cache.0.push((src.id(), handle.clone()));
                    handle
                });
                commands.entity(entity).insert(TrunkFaded(src));
            }
            (false, Some(TrunkFaded(original))) => {
                mat.0 = original.clone();
                commands.entity(entity).remove::<TrunkFaded>();
            }
            _ => {}
        }
    }
}

/// CARD VP-SKY-1 — rest look height in the Sanctuary valley, so the sky and
/// the mountain ring sit in frame.
pub const SANCTUARY_FOLLOW_LOOK_Y: f32 = 1.15;
/// Rest look height everywhere else (unchanged from before VP-SKY-1).
pub const FOLLOW_LOOK_Y: f32 = 0.45;

/// Rest look height above the presence for the follow camera, by realm.
/// Sanctuary (`None` / `Some(0)`) is 1.15; every other realm stays 0.45.
/// The Ambrosian look-down in `follow_camera` does not use this.
pub fn follow_look_height_for_realm(realm: Option<u8>) -> f32 {
    match realm {
        None | Some(0) => SANCTUARY_FOLLOW_LOOK_Y,
        _ => FOLLOW_LOOK_Y,
    }
}

// --- Schedule presence rhyme (H-2026-09-11-E3 stub/feel) -------------------
// Local person-read rhymes with LivingDay posts / sacred five / Place·mood
// greet. Presentation only — not an NPC roster. No peer spawn. Hour finishes
// with zero scheduled persons.

/// Known posts the local person-read may rhyme with (same four as LivingDay).
pub fn known_schedule_posts() -> [&'static str; 4] {
    [
        SchedulePlace::Sanctuary.post(),
        SchedulePlace::Heartwood.post(),
        SchedulePlace::Threshold.post(),
        SchedulePlace::Depths.post(),
    ]
}

/// Sacred five the local body may show as work intent — same verbs, no NPC-only.
pub fn sacred_five() -> [SacredVerb; 5] {
    [
        SacredVerb::Tend,
        SacredVerb::Take,
        SacredVerb::Flow,
        SacredVerb::Reserve,
        SacredVerb::Mend,
    ]
}

/// Local person-read work intent at a Place / period (reuses LivingDay).
pub fn presence_work_intent(place: SchedulePlace, period: DayPeriod) -> PreferredWork {
    preferred_work(place, period)
}

/// Still-frame: presence at a known post doing Place-honest work.
/// Period · post · verb — no second HUD, no roster spawn.
pub fn presence_still_frame(place: SchedulePlace, phase: f32) -> String {
    still_frame_line(place, phase)
}

/// Place · well mood greet on the one HUD (H hush). Reuses LivingDay helper.
pub fn presence_greet_rhyme(place: SchedulePlace, mood: WellMood) -> &'static str {
    greet_by_place_mood(place, mood)
}

/// Reach (Peace E) is the local body attending Place-honest work at the post.
pub fn presence_shows_place_work(stance: Stance) -> bool {
    matches!(stance, Stance::Reach)
}

/// This file remains the local stacked-capsule body — not an NPC roster product.
pub fn local_body_is_npc_roster() -> bool {
    false
}

/// Scheduled peer persons this slice adds. Zero — helpers, not a spawn.
pub fn scheduled_person_count() -> usize {
    0
}

/// Hour-finish bar: the hour still finishes if every person is removed.
pub fn hour_finishes_without_persons() -> bool {
    scheduled_person_count() == 0 && !local_body_is_npc_roster()
}

/// Presence / greet copy refuses crime · ownership · theft · fence · gold · Market · Online.
pub fn presence_copy_is_honest(s: &str) -> bool {
    schedule_copy_is_honest(s)
}

#[cfg(test)]
mod tests {
    // PLAYTEST-STRAFE-FIX-1 — render-side interpolation (Candidate B).
    #[test]
    fn strafe_fix_1_prev_curr_update_per_fixed_tick() {
        use bevy::ecs::system::RunSystemOnce;
        let mut world = World::new();
        let mut fixed = Time::<Fixed>::from_hz(60.0);
        fixed.advance_by(std::time::Duration::from_secs_f64(1.0 / 60.0));
        world.insert_resource(fixed);
        world.insert_resource(crate::input::PlayerInput {
            movement: Vec2::new(1.0, 0.0),
            ..Default::default()
        });
        world.insert_resource(super::JumpLatch::default());
        world.insert_resource(super::SoftPresence::default());
        let start = world.resource::<super::SoftPresence>().position;
        world.run_system_once(super::apply_locomotion).unwrap();
        let p = world.resource::<super::SoftPresence>();
        assert_eq!(p.prev_tick, Some(start));
        assert_eq!(p.curr_tick, Some(p.position));
        let end1 = p.position;
        world.run_system_once(super::apply_locomotion).unwrap();
        let p = world.resource::<super::SoftPresence>();
        assert_eq!(p.prev_tick, Some(end1));
        assert_eq!(p.curr_tick, Some(p.position));
    }

    #[test]
    fn strafe_fix_1_render_position_lerps_by_overstep() {
        let prev = Vec3::new(0.0, 0.9, 0.0);
        let curr = Vec3::new(2.0, 0.9, -4.0);
        let p = super::SoftPresence {
            position: curr,
            prev_tick: Some(prev),
            curr_tick: Some(curr),
            ..Default::default()
        };
        assert_eq!(p.render_position(0.0), prev);
        assert_eq!(p.render_position(1.0), curr);
        assert_eq!(p.render_position(0.5), Vec3::new(1.0, 0.9, -2.0));
        // Before the first tick, or after an out-of-tick teleport: raw position.
        let raw = super::SoftPresence { position: curr, ..Default::default() };
        assert_eq!(raw.render_position(0.5), curr);
        let moved = super::SoftPresence { position: Vec3::splat(9.0), ..p };
        assert_eq!(moved.render_position(0.5), Vec3::splat(9.0));
    }

    #[test]
    fn strafe_fix_1_sync_body_and_camera_use_interpolated_point() {
        let src = include_str!("human_presence.rs");
        let body = &src[src.find("fn sync_body(").unwrap()..];
        let body = &body[..body.find("\n}\n").unwrap()];
        assert!(body.contains("overstep_fraction()"));
        assert!(body.contains("tf.translation = render_pos;"));
        let cam = &src[src.find("fn follow_camera(").unwrap()..];
        let cam = &cam[..cam.find("\n}\n").unwrap()];
        assert!(cam.contains("overstep_fraction()"));
        assert!(cam.contains("let pos = presence.render_position(overstep);"));
        // look_at target and follow point are built from `pos`, never raw position.
        assert!(!cam.contains("presence.position"));
        assert!(cam.contains("cam.look_at(look, Vec3::Y);"));
        assert!(cam.contains("pos + Vec3::Y *"));
    }

    #[test]
    fn vp_char_1_low_medium_accent_non_metal_keeps_use_mute_readable() {
        for p in [GraphicsPreset::Mobile, GraphicsPreset::Low, GraphicsPreset::Medium] {
            let m = super::character_material_plan(p);
            assert_eq!(m.accent_metallic, 0.0);
            assert!(m.cloth_roughness >= 0.8);
        }
        // Use/Mute glow range unchanged: rest floor and full-Use ceiling.
        assert!((super::accent_glow(0.0, 0.0, 0.0) - 0.25).abs() < 1e-6);
        assert!(super::accent_glow(1.0, 1.0, 0.0) > 1.0);
    }

    #[test]
    fn vp_char_1_high_ultra_richer_metal_only_on_accent() {
        for p in [GraphicsPreset::High, GraphicsPreset::Ultra] {
            let m = super::character_material_plan(p);
            assert!(m.accent_metallic > 0.5 && m.accent_metallic <= 1.0);
            assert!(m.accent_roughness < 0.42);
        }
        assert_eq!(
            super::character_material_plan(GraphicsPreset::High),
            super::character_material_plan(GraphicsPreset::Ultra),
            "no Ultra-only stack"
        );
    }

    #[test]
    fn vp_char_1_silhouette_contrast_and_values_in_range() {
        for p in [
            GraphicsPreset::Mobile,
            GraphicsPreset::Low,
            GraphicsPreset::Medium,
            GraphicsPreset::High,
            GraphicsPreset::Ultra,
        ] {
            let m = super::character_material_plan(p);
            for v in [m.cloth_roughness, m.cloth_reflectance, m.accent_roughness] {
                assert!((0.0..=1.0).contains(&v));
            }
            // Cloth stays rougher than the accent so the gold hand pops.
            assert!(m.cloth_roughness > m.accent_roughness);
        }
        let body = super::BODY_EARTH.to_srgba();
        let gold = super::SANCTUARY_GOLD.to_srgba();
        assert!(gold.red - body.red > 0.5, "accent vs body value contrast");
    }

    use super::*;
    use crate::local_settings::LocalSettingsState;
    use shared::local_settings::{LocalSettings, PeaceKey};

    /// CARD VP-SKY-1 — follow-camera look height: Sanctuary 1.15, every other
    /// realm 0.45, and the Ambrosian look-down stays exactly 0.45.
    #[test]
    fn vp_sky_follow_look_height_sanctuary_other_and_ambrosian() {
        assert_eq!(follow_look_height_for_realm(None), 1.15);
        assert_eq!(follow_look_height_for_realm(Some(0)), 1.15);
        for realm in 1u8..=8 {
            assert_eq!(follow_look_height_for_realm(Some(realm)), 0.45, "realm {realm}");
        }
        // Ambrosian look-down literal in follow_camera is untouched.
        let src = include_str!("human_presence.rs");
        let body = &src[src.find("fn follow_camera(").expect("follow_camera")..];
        let body = &body[..body.find("\n}\n").expect("end of follow_camera")];
        assert!(body.contains(
            "let look = if look_down {\n        Vec3::new(pos.x, 0.45, pos.z)\n    } else {"
        ));
        assert!(body.contains("follow_look_height_for_realm(realm.and_then(|r| r.current))"));
    }

    #[test]
    fn accel_ramps_toward_walk() {
        let dt = 1.0 / 60.0;
        let mut v = Vec2::ZERO;
        let wish = Vec2::new(WALK, 0.0);
        for _ in 0..30 {
            v = approach_horizontal(v, wish, MOVE_ACCEL, MOVE_DECEL, dt);
        }
        assert!(
            (v.x - WALK).abs() < 0.05,
            "expected near walk speed, got {}",
            v.x
        );
        assert!(v.y.abs() < 1e-5);
    }

    #[test]
    fn stop_on_release_zeros_without_ice_skate() {
        let dt = 1.0 / 60.0;
        let mut v = Vec2::new(WALK, 0.0);
        // ~0.15s of decel at 48 u/s² clears 3.4 walk.
        for _ in 0..12 {
            v = approach_horizontal(v, Vec2::ZERO, MOVE_ACCEL, MOVE_DECEL, dt);
        }
        assert!(
            v.length() < 0.05,
            "expected stop-on-release, residual {}",
            v.length()
        );
    }

    #[test]
    fn parts_stack_into_a_standing_person() {
        let s = silhouette();
        // Feet on the plane, head where the old single capsule topped out.
        assert!(s.feet.abs() < 0.06, "feet floating at {}", s.feet);
        assert!(
            (s.head.1 - 1.74).abs() < 0.02,
            "head top drifted to {}",
            s.head.1
        );
        assert!(s.head.1 - s.feet > 1.6, "person too short");
        // Head over torso over hips over legs, each mass touching the next.
        assert!(s.head.0 < s.torso.1, "head floats off the torso");
        assert!(s.head.0 > s.torso.0, "head sank into the torso");
        assert!(s.torso.0 < s.hips.1, "torso floats off the hips");
        assert!(s.hips.0 <= s.legs_top, "hips float off the legs");
        // Head reads smaller than the torso mass — person, not blob.
        assert!(HEAD_R < TORSO_R);
    }

    #[test]
    fn stance_names_use_over_load_and_stride() {
        assert_eq!(stance_read(0.0, 0.0, BodyTell::Easy), Stance::Idle);
        assert_eq!(stance_read(WALK, 0.0, BodyTell::Easy), Stance::Stride);
        assert_eq!(stance_read(0.0, 0.0, BodyTell::Heavy), Stance::Heavy);
        assert_eq!(stance_read(0.0, 0.0, BodyTell::Winded), Stance::Winded);
        // Peace E is the point: the reach reads over carry and over the stride.
        assert_eq!(stance_read(WALK, 1.0, BodyTell::Winded), Stance::Reach);
        assert_eq!(stance_read(0.0, 0.4, BodyTell::Heavy), Stance::Reach);
    }

    #[test]
    fn idle_reach_and_carry_are_told_apart_in_a_still() {
        let idle_arm = shoulder_pitch(1.0, 0.0, 0.0, 0.0);
        let reach_arm = shoulder_pitch(1.0, 0.0, 0.0, 1.0);
        // The attending arm swings forward far enough to read from the yard.
        assert!(reach_arm < -0.9, "reach arm only at {reach_arm}");
        assert!(idle_arm.abs() < 0.05, "idle arm not at rest: {idle_arm}");
        assert!(idle_arm - reach_arm > 0.9);

        let idle_lean = waist_pitch(0.0, BodyTell::Easy);
        let reach_lean = waist_pitch(1.0, BodyTell::Easy);
        let heavy_lean = waist_pitch(0.0, BodyTell::Heavy);
        let winded_lean = waist_pitch(0.0, BodyTell::Winded);
        assert!(reach_lean - idle_lean > 0.15);
        assert!(heavy_lean - idle_lean > 0.1);
        assert!(winded_lean > heavy_lean);
        // Carry drops the knees; an idle stand does not.
        assert_eq!(crouch_drop(0.0, BodyTell::Easy), 0.0);
        assert!(crouch_drop(0.0, BodyTell::Winded) > crouch_drop(0.0, BodyTell::Heavy));
    }

    #[test]
    fn reach_leads_with_one_arm_and_plants_the_legs() {
        let lead = shoulder_pitch(1.0, 0.0, 0.0, 1.0);
        let trail = shoulder_pitch(-1.0, 0.0, 0.0, 1.0);
        assert!(lead < trail, "both arms swung the same amount");
        assert!(trail < 0.0, "trailing arm did not join the reach");
        // Legs stop swinging while the body attends the glow.
        let planted = hip_pitch(1.0, SPRINT, PI * 0.5, 1.0);
        assert!(planted.abs() < 1e-4, "legs still striding: {planted}");
        let striding = hip_pitch(1.0, SPRINT, PI * 0.5, 0.0);
        assert!(striding.abs() > 0.3, "stride swing too small: {striding}");
    }

    #[test]
    fn arms_swing_opposite_the_same_side_leg() {
        let phase = PI * 0.5;
        let arm = shoulder_pitch(1.0, WALK, phase, 0.0);
        let leg = hip_pitch(1.0, WALK, phase, 0.0);
        assert!(arm * leg < 0.0, "arm {arm} and leg {leg} swung together");
    }

    #[test]
    fn use_read_lingers_past_the_tap() {
        let dt = 1.0 / 60.0;
        let mut level = attend_level(0.0, true, false, dt);
        assert_eq!(level, 1.0);
        // Still reading the verb a third of a second after the edge.
        for _ in 0..20 {
            level = attend_level(level, false, false, dt);
        }
        assert!(level > 0.3, "Use read faded too fast: {level}");
        for _ in 0..20 {
            level = attend_level(level, false, false, dt);
        }
        assert_eq!(level, 0.0, "Use read never let go");
        // Hold-E tend keeps the reach up.
        assert_eq!(attend_level(0.2, false, true, dt), 1.0);
    }

    #[test]
    fn accent_is_brightest_attending_a_glow() {
        let idle = accent_glow(0.0, 0.0, 0.0);
        let near = accent_glow(0.0, 1.0, 0.0);
        let using = accent_glow(1.0, 1.0, 0.0);
        let far_using = accent_glow(1.0, 0.0, 0.0);
        assert!(using > far_using);
        assert!(far_using > idle);
        assert_eq!(idle, near, "the accent must not light up on its own");
        assert!(using <= 1.6, "accent blew past one accent: {using}");
        assert_eq!(glow_proximity(HARVEST_REACH), 1.0);
        assert_eq!(glow_proximity(HARVEST_REACH * 2.4), 0.0);
    }

    /// CARD OPT-REDUCED-MOTION-PULSE — reduced motion and Comfort Low hold
    /// the harvest punch at rest. Full motion still takes the first-take kick.
    #[test]
    fn reduced_motion_caps_presence_punch() {
        use shared::local_settings::LocalSettings;

        let full_delta = presence_punch_camera_delta(1.0, 1.0);
        let full_lift = presence_punch_look_lift(1.0, 1.0);
        assert!((full_delta.y - 0.22).abs() < 1e-5);
        assert!((full_delta.z - (-0.35)).abs() < 1e-5);
        assert!((full_lift - 0.08).abs() < 1e-5);
        assert!((scaled_presence_punch(0.42, 1.0) - 0.42).abs() < 1e-5);

        let rest_delta = presence_punch_camera_delta(1.0, 0.0);
        assert_eq!(rest_delta, Vec3::ZERO);
        assert_eq!(presence_punch_look_lift(1.0, 0.0), 0.0);
        assert_eq!(scaled_presence_punch(1.0, 0.0), 0.0);

        let accent_full = accent_glow(1.0, 1.0, scaled_presence_punch(1.0, 1.0));
        let accent_rest = accent_glow(1.0, 1.0, scaled_presence_punch(1.0, 0.0));
        assert!(accent_full > accent_rest);
        assert_eq!(accent_rest, accent_glow(1.0, 1.0, 0.0));

        let mut low = LocalSettings::peace_defaults();
        low.set_graphics_preset(GraphicsPreset::Low);
        assert!(low.reduced_motion, "Comfort Low is reduced motion");
        assert_eq!(low.camera_punch_scale(), 0.0);
        assert_eq!(
            presence_punch_camera_delta(1.0, low.camera_punch_scale()),
            Vec3::ZERO
        );
        assert_eq!(
            accent_glow(1.0, 1.0, scaled_presence_punch(1.0, low.camera_punch_scale())),
            accent_rest
        );

        let mut toggled = LocalSettings::peace_defaults();
        toggled.toggle_reduced_motion();
        assert!(toggled.reduced_motion);
        assert_eq!(
            presence_punch_camera_delta(1.0, toggled.camera_punch_scale()),
            Vec3::ZERO
        );

        let mut mid = LocalSettings::peace_defaults();
        mid.set_graphics_preset(GraphicsPreset::Medium);
        assert!(!mid.reduced_motion);
        assert_eq!(mid.camera_punch_scale(), 1.0);
        assert_eq!(
            presence_punch_camera_delta(1.0, mid.camera_punch_scale()),
            full_delta
        );
    }

    /// CARD OPT-PUNCH-LOW — Graphics Low caps camera punch. Reduced motion
    /// (scale 0, including Comfort Low) stays at rest and wins over the cap.
    #[test]
    fn graphics_low_caps_camera_punch_reduced_motion_wins() {
        use shared::local_settings::LocalSettings;

        assert_eq!(punch_scale_for_graphics(0.0, GraphicsPreset::Low), 0.0);
        assert_eq!(
            presence_punch_camera_delta(
                1.0,
                punch_scale_for_graphics(0.0, GraphicsPreset::Low)
            ),
            Vec3::ZERO
        );
        assert_eq!(
            presence_punch_look_lift(1.0, punch_scale_for_graphics(0.0, GraphicsPreset::Low)),
            0.0
        );

        let mut comfort_low = LocalSettings::peace_defaults();
        comfort_low.set_graphics_preset(GraphicsPreset::Low);
        assert!(comfort_low.reduced_motion);
        assert_eq!(comfort_low.camera_punch_scale(), 0.0);
        let comfort_scale =
            punch_scale_for_graphics(comfort_low.camera_punch_scale(), comfort_low.graphics_preset);
        assert_eq!(comfort_scale, 0.0);
        assert_eq!(presence_punch_camera_delta(1.0, comfort_scale), Vec3::ZERO);

        let mut motion_off = LocalSettings::peace_defaults();
        motion_off.set_graphics_preset(GraphicsPreset::Low);
        motion_off.toggle_reduced_motion();
        assert_eq!(motion_off.graphics_preset, GraphicsPreset::Low);
        assert!(!motion_off.reduced_motion);
        assert_eq!(motion_off.camera_punch_scale(), 1.0);
        let low_scale =
            punch_scale_for_graphics(motion_off.camera_punch_scale(), motion_off.graphics_preset);
        assert!((low_scale - GRAPHICS_LOW_PUNCH_AMPLITUDE).abs() < 1e-5);
        assert!(low_scale > 0.0 && low_scale < 1.0);

        let full = presence_punch_camera_delta(1.0, 1.0);
        let low_delta = presence_punch_camera_delta(1.0, low_scale);
        let low_lift = presence_punch_look_lift(1.0, low_scale);
        assert!(low_delta.length() > 0.0);
        assert!(low_delta.length() < full.length());
        assert!(low_lift > 0.0 && low_lift < presence_punch_look_lift(1.0, 1.0));

        assert_eq!(punch_scale_for_graphics(1.0, GraphicsPreset::Medium), 1.0);
        assert_eq!(punch_scale_for_graphics(1.0, GraphicsPreset::High), 1.0);
        assert_eq!(
            punch_scale_for_graphics(1.0, GraphicsPreset::Mobile),
            punch_scale_for_graphics(1.0, GraphicsPreset::Low)
        );
        assert_eq!(
            punch_scale_for_graphics(1.0, GraphicsPreset::Ultra),
            punch_scale_for_graphics(1.0, GraphicsPreset::High)
        );
        assert_eq!(
            presence_punch_camera_delta(
                1.0,
                punch_scale_for_graphics(1.0, GraphicsPreset::Medium)
            ),
            full
        );
        assert_eq!(
            presence_punch_camera_delta(
                1.0,
                punch_scale_for_graphics(1.0, GraphicsPreset::High)
            ),
            full
        );
        assert_eq!(GraphicsPreset::ALL.len(), 5);
        assert_eq!(GraphicsPreset::Low.label(), "Low");
        assert_eq!(GraphicsPreset::Medium.label(), "Medium");
        assert_eq!(GraphicsPreset::High.label(), "High");
        assert!(!GraphicsPreset::Low.label().contains("Ultra"));
        assert!(!GraphicsPreset::Medium.label().contains("Ultra"));
        assert!(!GraphicsPreset::High.label().contains("Ultra"));
    }

    #[test]
    fn standing_use_turns_the_front_to_the_glow() {
        let from = Vec3::new(0.0, STAND, 0.0);
        let target = Vec3::new(3.6, 0.55, 0.0);
        let yaw = yaw_toward(from, target).expect("a glow to the side has a yaw");
        let front = Quat::from_rotation_y(yaw) * Vec3::Z;
        assert!(front.x > 0.99, "front did not turn to the glow: {front:?}");
        assert!(yaw_toward(from, from).is_none());
    }

    #[test]
    fn release_decel_faster_than_ice_skate() {
        // One tick of zero wish must cut speed; ice-skate would keep full velocity.
        let dt = 1.0 / 60.0;
        let before = Vec2::new(WALK, 0.0);
        let after = approach_horizontal(before, Vec2::ZERO, MOVE_ACCEL, MOVE_DECEL, dt);
        assert!(after.length() < before.length());
        assert!(after.x > 0.0); // not teleport-stop in one tick, but braking
    }

    #[test]
    fn presence_rhymes_living_day_posts_and_sacred_five() {
        assert_eq!(
            known_schedule_posts(),
            [
                "Sanctuary well",
                "Heartwood Wards",
                "Threshold pipe",
                "Depths landing",
            ]
        );
        assert_eq!(
            sacred_five(),
            [
                SacredVerb::Tend,
                SacredVerb::Take,
                SacredVerb::Flow,
                SacredVerb::Reserve,
                SacredVerb::Mend,
            ]
        );
        // Default LivingDay phase 0.18 is Day — Sanctuary well · Take.
        let work = presence_work_intent(SchedulePlace::Sanctuary, DayPeriod::Day);
        assert_eq!(work.post, "Sanctuary well");
        assert_eq!(work.verb, SacredVerb::Take);
        let line = presence_still_frame(SchedulePlace::Sanctuary, 0.18);
        assert_eq!(line, still_frame_line(SchedulePlace::Sanctuary, 0.18));
        assert_eq!(line, "Day · Sanctuary well · Take");
        assert!(presence_copy_is_honest(&line));
        // Reach is the still-frame "doing the Place's job" on the local body.
        assert!(presence_shows_place_work(Stance::Reach));
        assert!(!presence_shows_place_work(Stance::Idle));
        assert!(!presence_shows_place_work(Stance::Stride));
    }

    #[test]
    fn presence_work_is_place_honest() {
        for period in [
            DayPeriod::Dawn,
            DayPeriod::Day,
            DayPeriod::Dusk,
            DayPeriod::Night,
        ] {
            let pipe = presence_work_intent(SchedulePlace::Threshold, period);
            assert_ne!(pipe.verb, SacredVerb::Take, "Threshold {period:?}");
            assert_eq!(pipe.post, "Threshold pipe");
            let landing = presence_work_intent(SchedulePlace::Depths, period);
            assert_ne!(landing.verb, SacredVerb::Take, "Depths {period:?}");
            assert!(
                matches!(landing.verb, SacredVerb::Tend | SacredVerb::Mend),
                "Depths restore verb"
            );
            assert_eq!(landing.post, "Depths landing");
        }
        assert_eq!(
            presence_work_intent(SchedulePlace::Heartwood, DayPeriod::Day).post,
            "Heartwood Wards"
        );
    }

    #[test]
    fn greet_rhyme_on_one_hud_h_hush() {
        for place in [
            SchedulePlace::Sanctuary,
            SchedulePlace::Heartwood,
            SchedulePlace::Threshold,
            SchedulePlace::Depths,
        ] {
            for mood in [
                WellMood::Idle,
                WellMood::Glowing,
                WellMood::Tended,
                WellMood::Resting,
                WellMood::Stressed,
            ] {
                let greet = presence_greet_rhyme(place, mood);
                assert_eq!(greet, greet_by_place_mood(place, mood));
                assert!(presence_copy_is_honest(greet), "{greet}");
            }
        }
    }

    #[test]
    fn local_body_not_roster_hour_finishes_without_persons() {
        assert!(!local_body_is_npc_roster());
        assert_eq!(scheduled_person_count(), 0);
        assert!(hour_finishes_without_persons());
        // Removing scheduled persons cannot strand these flags — count stays zero.
        assert_eq!(scheduled_person_count(), 0);
        assert!(hour_finishes_without_persons());
    }

    #[test]
    fn refuse_crime_gold_market_online_copy() {
        assert!(!presence_copy_is_honest("sell gold on Market"));
        assert!(!presence_copy_is_honest("crime meter Online"));
        assert!(!presence_copy_is_honest("theft and fence ownership"));
        assert!(!presence_copy_is_honest("pickpocket the stall"));
        assert!(presence_copy_is_honest("Day · Sanctuary well · Take"));
        assert!(presence_copy_is_honest(
            "Threshold pipe — Tend, not Take."
        ));
        for place in [
            SchedulePlace::Sanctuary,
            SchedulePlace::Heartwood,
            SchedulePlace::Threshold,
            SchedulePlace::Depths,
        ] {
            let line = presence_still_frame(place, 0.18);
            assert!(presence_copy_is_honest(&line), "{line}");
        }
    }

    #[test]
    fn comfort_mesh_lod_plan_scales_stacked_capsule_low_stays_readable() {
        assert!((presence_lod_scale(GraphicsPreset::Low) - 0.65).abs() < f32::EPSILON);
        assert!((presence_lod_scale(GraphicsPreset::Medium) - 1.0).abs() < f32::EPSILON);
        assert!((presence_lod_scale(GraphicsPreset::High) - 1.15).abs() < f32::EPSILON);
        assert_eq!(
            presence_lod_scale(GraphicsPreset::Mobile),
            presence_lod_scale(GraphicsPreset::Low)
        );
        assert_eq!(
            presence_lod_scale(GraphicsPreset::Ultra),
            presence_lod_scale(GraphicsPreset::High)
        );
        assert!(presence_low_stays_capsule_readable());
        assert!(presence_still_capsule_readable(0.65));
        assert!(presence_still_capsule_readable(1.0));
        assert!(presence_still_capsule_readable(1.15));

        let low = mesh_lod::plan_for_preset(GraphicsPreset::Low);
        assert!(low.primitives_only);
        assert!(!mesh_lod::lived_presence_uses_authored_glb(&low));
        assert!(!presence_dress_active(GraphicsPreset::Low, true));
        assert!(!presence_dress_active(GraphicsPreset::Medium, true));
        assert!(presence_dress_active(GraphicsPreset::High, true));
        assert!(!presence_dress_active(GraphicsPreset::High, false));
        assert_eq!(
            presence_dress_active(GraphicsPreset::High, mesh_lod::optional_authored_glb_present()),
            mesh_lod::lived_persona_dress_active(&mesh_lod::plan_for_preset(GraphicsPreset::High))
        );
    }

    #[test]
    fn comfort_lod_does_not_mute_peace_e_or_open_race_lobby() {
        assert_eq!(attend_level(0.0, true, false, 1.0 / 60.0), 1.0);
        assert_eq!(stance_read(0.0, 1.0, BodyTell::Easy), Stance::Reach);
        assert_eq!(stance_read(WALK, 1.0, BodyTell::Winded), Stance::Reach);
        assert!(!presence_face_picks_class());
        assert!(!presence_practices_gate_peace());
        assert!(!presence_opens_race_lobby());
        assert!(mesh_lod::face_is_not_class());
        assert!(mesh_lod::practices_after_house());
        assert!(mesh_lod::race_lobby_closed());
        assert_eq!(GraphicsPreset::ALL.len(), 5);
        assert_eq!(GraphicsPreset::Low.label(), "Low");
        assert_eq!(GraphicsPreset::Medium.label(), "Medium");
        assert_eq!(GraphicsPreset::High.label(), "High");
    }

    /// CARD L7 — Human wake is the existing Sanctuary yard helper.
    #[test]
    fn l7_human_wake_is_sanctuary_yard_helper() {
        let wake = people_landing_wake(PeopleLanding::SanctuaryYard);
        assert_eq!(wake, sanctuary_yard_wake());
        assert_eq!(arrival_camera_ease(PeopleLanding::SanctuaryYard), ArrivalCameraEase::Down);
        assert!(!presence_reads_ambrosian_lift(wake));
        let beat = run_arrival_beat(PeopleLanding::SanctuaryYard);
        assert!(beat.armed);
        assert_eq!(beat.landing, Some(PeopleLanding::SanctuaryYard));
    }

    /// CARD L7 — Ambrosian wake Y > Human. Same Sanctuary disk. No hull mesh.
    #[test]
    fn l7_ambrosian_wake_y_above_human() {
        let human = people_landing_wake(PeopleLanding::SanctuaryYard);
        let amb = people_landing_wake(PeopleLanding::SanctuaryWellFromAbove);
        assert!(amb.y > human.y);
        assert_eq!(amb.x, human.x);
        assert_eq!(amb.z, human.z);
        assert!(presence_reads_ambrosian_lift(amb));
        assert_eq!(
            arrival_camera_ease(PeopleLanding::SanctuaryWellFromAbove),
            ArrivalCameraEase::Up
        );
        assert_eq!(
            arrival_camera_ease(PeopleLanding::Threshold),
            ArrivalCameraEase::None
        );
        assert!(!arrival_beat_after_land(None).armed);
    }

    /// CARD F5 — unsealed light may cross door → L7 beat armed · still unsealed until E/Q.
    #[test]
    fn f5_unsealed_light_may_cross_door_l7_beat_armed_still_unsealed_until_eq() {
        use crate::hour_sacred::{
            confirm_gate_seal, still_unsealed_until_eq, soul_is_light, HousePeople,
        };
        use shared::local_settings::PeaceKey;

        let mut presence = SoftPresence::default();
        wake_people_landing(&mut presence, PeopleLanding::SanctuaryYard);
        let beat = run_arrival_beat(PeopleLanding::SanctuaryYard);
        assert!(beat.armed);
        assert_eq!(presence.position, sanctuary_yard_wake());
        assert!(still_unsealed_until_eq(None));
        assert!(soul_is_light(None));
        let pending = Some((HousePeople::Human, PeopleLanding::SanctuaryYard));
        assert!(confirm_gate_seal(PeaceKey::Digit1, pending).is_none());
        let sealed = confirm_gate_seal(PeaceKey::E, pending).expect("E");
        assert!(!still_unsealed_until_eq(Some(sealed)));
    }

    /// CARD F5 — decline / wrong door → garden wake · beat disarmed · not Ambrosian lift.
    #[test]
    fn f5_decline_wrong_door_garden_light_not_sealed_place_id_garden() {
        let mut presence = SoftPresence::default();
        wake_people_landing(&mut presence, PeopleLanding::SanctuaryWellFromAbove);
        assert!(presence_reads_ambrosian_lift(presence.position));
        assert!(run_arrival_beat(PeopleLanding::SanctuaryWellFromAbove).armed);

        wake_garden_bounce(&mut presence);
        assert_eq!(presence.position, garden_bounce_wake());
        assert_eq!(presence.position, sanctuary_yard_wake());
        assert!(!presence_reads_ambrosian_lift(presence.position));
        assert!(!wrong_door_bounce_disarms_arrival_beat().armed);
        assert!(presence.grounded);
    }

    /// CARD F5 — Peace recall does not clear seal / vision home for light.
    #[test]
    fn f5_peace_recall_does_not_clear_seal_vision_home_for_light() {
        use crate::hour_sacred::{peace_recall, HousePeople, PEACE_RECALL_VISION_HOME};

        let sealed = Some((HousePeople::Draek, PeopleLanding::DepthsTealWayHome));
        let (after, line) = peace_recall(sealed);
        assert_eq!(after, sealed);
        assert_eq!(line, PEACE_RECALL_VISION_HOME);
        let (light_after, light_line) = peace_recall(None);
        assert!(light_after.is_none());
        assert_eq!(light_line, "vision home");
    }

    /// CARD F5 — PlaceId / LOCAL_HEXES len == 3.
    #[test]
    fn f5_place_id_local_hexes_len_three() {
        use shared::hex_travel::{PlaceId, LOCAL_HEXES};

        assert_eq!(LOCAL_HEXES.len(), 3);
        match PlaceId::Sanctuary {
            PlaceId::Sanctuary | PlaceId::Heartwood | PlaceId::Depths => {}
        }
    }

    /// CARD F5 — STEWARD_ONLINE_YES false.
    #[test]
    fn f5_steward_online_yes_false() {
        use shared::persona::{ONLINE_PICKER_ENABLED, STEWARD_ONLINE_YES};

        assert!(!STEWARD_ONLINE_YES);
        assert!(!ONLINE_PICKER_ENABLED);
        assert!(!shared::hex_protocol::default_client_listens());
    }

    /// CARD F5 — no fifth Place · no Title race lobby.
    #[test]
    fn f5_no_fifth_place_no_title_race_lobby() {
        use crate::hour_sacred::{f5_title_is_race_lobby, garden_roster_is_race_portrait_lobby};
        use shared::hex_travel::LOCAL_HEXES;

        assert_eq!(LOCAL_HEXES.len(), 3);
        assert!(!presence_opens_race_lobby());
        assert!(!f5_title_is_race_lobby());
        assert!(!garden_roster_is_race_portrait_lobby());
        assert!(mesh_lod::race_lobby_closed());
    }

    /// CARD STILL-BEFORE-YARD-1 — held W. Caller steps the fixed clock.
    fn yard_w_world(door: Option<LaunchDoor>, settings_open: Option<bool>) -> App {
        use bevy::time::TimeUpdateStrategy;

        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(Time::<Fixed>::from_hz(60.0))
            .insert_resource(TimeUpdateStrategy::FixedTimesteps(12))
            .init_resource::<SoftPresence>()
            .init_resource::<JumpLatch>()
            .insert_resource(PlayerInput {
                movement: Vec2::new(0.0, 1.0),
                ..Default::default()
            })
            .add_systems(FixedUpdate, apply_locomotion);
        if let Some(door) = door {
            app.insert_resource(door);
        }
        if let Some(open) = settings_open {
            app.insert_resource(HouseLabel {
                house: shared::house_name::HouseName::default(),
                persist_present: false,
                hour_two_held: false,
                book_held: false,
                settings_open: open,
                draft: String::new(),
                naming_offered: false,
                seals_offered: false,
            });
        }
        app
    }

    /// The first `update` only stamps the clock; the second runs the steps.
    fn yard_w_app(door: Option<LaunchDoor>, settings_open: Option<bool>) -> App {
        let mut app = yard_w_world(door, settings_open);
        app.update();
        app.update();
        assert!(
            app.world().resource::<Time<Fixed>>().elapsed_secs() > 0.0,
            "fixed clock did not step"
        );
        app
    }

    /// (a) Title + W stays at spawn.
    #[test]
    fn still_before_yard_title_w_stays_at_spawn() {
        let app = yard_w_app(Some(LaunchDoor::Title), Some(false));
        let spawn = Vec3::new(0.0, STAND, 0.0);
        let presence = app.world().resource::<SoftPresence>();
        assert_eq!(presence.position, spawn);
        assert_eq!(presence.velocity, Vec3::ZERO);
        assert!(presence.grounded);
    }

    /// (b) Esc pause (settings_open, InYard) + W leaves horizontal position and velocity.
    #[test]
    fn still_before_yard_pause_w_leaves_horizontal_unchanged() {
        let app = yard_w_app(Some(LaunchDoor::InYard), Some(true));
        let presence = app.world().resource::<SoftPresence>();
        assert_eq!(presence.position.x, 0.0);
        assert_eq!(presence.position.z, 0.0);
        assert_eq!(presence.velocity.x, 0.0);
        assert_eq!(presence.velocity.z, 0.0);
    }

    /// (c) An airborne body under pause still lands.
    #[test]
    fn still_before_yard_pause_airborne_still_lands() {
        let mut app = yard_w_app(Some(LaunchDoor::InYard), Some(true));
        {
            let mut presence = app.world_mut().resource_mut::<SoftPresence>();
            presence.position = Vec3::new(0.0, STAND + 0.25, 0.0);
            presence.velocity = Vec3::ZERO;
            presence.grounded = false;
        }
        app.update();
        let presence = app.world().resource::<SoftPresence>();
        assert!(presence.grounded);
        assert_eq!(presence.position.y, STAND);
        assert_eq!(presence.velocity.y, 0.0);
        assert_eq!(presence.position.x, 0.0);
        assert_eq!(presence.position.z, 0.0);
        assert_eq!(presence.velocity.x, 0.0);
        assert_eq!(presence.velocity.z, 0.0);
    }

    /// CARD JUMP-BUFFER-1 — open yard, one exact 1/60 fixed tick per update; `drop` starts at STAND + 2.
    fn jb_app(drop: bool) -> App {
        let mut app = yard_w_app(None, None);
        app.insert_resource(bevy::time::TimeUpdateStrategy::FixedTimesteps(1));
        let mut presence = app.world_mut().resource_mut::<SoftPresence>();
        (presence.position.y, presence.velocity, presence.grounded) = (STAND + if drop { 2.0 } else { 0.0 }, Vec3::ZERO, !drop);
        app
    }
    fn jb_step(app: &mut App) -> bool {
        app.update();
        assert_eq!(app.world().resource::<Time<Fixed>>().delta_secs(), 1.0 / 60.0);
        app.world().resource::<SoftPresence>().velocity.y == JUMP
    }
    /// Arms `k` airborne ticks before the clamp land (arm and landing tick counted); `hook` runs after tick land - `back`. Returns JUMP rises.
    fn jb_rises(k: usize, held: bool, back: usize, hook: fn(&mut SoftPresence)) -> usize {
        let mut dry = jb_app(true);
        let n = (1..).find(|_| !jb_step(&mut dry) && dry.world().resource::<SoftPresence>().grounded).unwrap();
        let mut app = jb_app(true);
        app.add_systems(Update, arm_jump_latch);
        (1..=n + 60).filter(|&t| {
            // Held: one `jump: true` frame; Update arms after this tick, so it lands on the next.
            app.world_mut().resource_mut::<PlayerInput>().jump = held && t + k == n;
            app.world_mut().resource_mut::<JumpLatch>().pending |= !held && t + k == n + 1;
            let rise = jb_step(&mut app);
            if t + back == n { hook(&mut app.world_mut().resource_mut::<SoftPresence>()); }
            rise
        }).count()
    }
    #[test]
    fn jump_buffer_6_ticks_jumps_12_does_not() {
        assert_eq!((jb_rises(6, false, 0, |_| {}), jb_rises(12, false, 0, |_| {})), (1, 0));
    }
    #[test]
    fn jump_buffer_boundary_7_jumps_8_does_not() {
        assert_eq!((jb_rises(7, false, 0, |_| {}), jb_rises(8, false, 0, |_| {})), (1, 0));
    }
    #[test]
    fn jump_buffer_held_key_one_rise() {
        assert_eq!(jb_rises(4, true, 0, |_| {}), 1);
    }
    #[test]
    fn jump_buffer_grounded_press_unchanged() {
        let mut app = jb_app(false);
        app.world_mut().resource_mut::<JumpLatch>().pending = true;
        assert!(jb_step(&mut app) && !app.world().resource::<SoftPresence>().grounded);
        assert!(!app.world().resource::<JumpLatch>().pending);
    }
    #[test]
    fn jump_buffer_reseat_does_not_hop() {
        assert_eq!(jb_rises(3, false, 0, |p| { let landed = p.position; wake_garden_bounce(p); assert_ne!(p.position, landed); }), 0);
        assert_eq!(jb_rises(4, false, 2, |p| { assert!(!p.grounded); p.grounded = true; }), 0);
    }

    /// (d) NameHouse + W does not move.
    #[test]
    fn still_before_yard_name_house_w_does_not_move() {
        let app = yard_w_app(Some(LaunchDoor::NameHouse), Some(false));
        let spawn = Vec3::new(0.0, STAND, 0.0);
        let presence = app.world().resource::<SoftPresence>();
        assert_eq!(presence.position, spawn);
        assert_eq!(presence.velocity, Vec3::ZERO);
    }

    /// (e) InYard, pause closed, W moves.
    #[test]
    fn still_before_yard_inyard_w_moves() {
        let app = yard_w_app(Some(LaunchDoor::InYard), Some(false));
        let presence = app.world().resource::<SoftPresence>();
        assert!(
            presence.position.z < 0.0,
            "W should step -Z, z={}",
            presence.position.z
        );
        assert!(presence.velocity.z < 0.0);
    }

    /// Walk W in the yard, then Esc. Horizontal position holds on the paused frames.
    #[test]
    fn still_before_yard_esc_holds_walk_xz() {
        let mut app = yard_w_world(Some(LaunchDoor::InYard), Some(false));
        app.update();
        app.update();
        app.update();
        let walked = app.world().resource::<SoftPresence>();
        let (x, z) = (walked.position.x, walked.position.z);
        assert!(z < 0.0, "W should have left spawn, z={z}");
        app.world_mut().resource_mut::<HouseLabel>().settings_open = true;
        for _ in 0..3 {
            app.update();
            let presence = app.world().resource::<SoftPresence>();
            assert_eq!(presence.position.x, x);
            assert_eq!(presence.position.z, z);
        }
    }

    fn yaw_near(a: f32, b: f32) -> bool {
        wrap_pi(a - b).abs() < 1e-3
    }

    fn body_front(app: &mut App) -> Vec3 {
        let world = app.world_mut();
        let mut bodies = world.query::<&Transform>();
        let tf = bodies.iter(world).next().expect("body");
        tf.rotation * Vec3::Z
    }

    /// CARD MOVE-STRAFE-DEFAULT-1 — same clock as the yard gate, plus orbit and preset.
    fn strafe_world(
        preset: ControlPreset,
        yaw: f32,
        movement: Vec2,
        door: Option<LaunchDoor>,
        settings_open: Option<bool>,
        keys: &[KeyCode],
    ) -> App {
        use bevy::time::TimeUpdateStrategy;

        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(Time::<Fixed>::from_hz(60.0))
            .insert_resource(TimeUpdateStrategy::FixedTimesteps(12))
            .init_resource::<SoftPresence>()
            .init_resource::<JumpLatch>()
            .init_resource::<PersonAttend>()
            .insert_resource(preset)
            .init_resource::<BodyFacing>()
            .init_resource::<FollowOrbit>()
            .insert_resource(PlayerInput {
                movement,
                ..Default::default()
            })
            .add_systems(FixedUpdate, apply_locomotion)
            .add_systems(Update, sync_body);
        app.world_mut().resource_mut::<FollowOrbit>().yaw = yaw;
        if let Some(door) = door {
            app.insert_resource(door);
        }
        if let Some(open) = settings_open {
            app.insert_resource(quiet_house(open));
        }
        if !keys.is_empty() {
            app.init_resource::<ButtonInput<KeyCode>>();
            let mut keyboard = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            for key in keys {
                keyboard.press(*key);
            }
        }
        app.world_mut()
            .spawn((HumanPresence, Transform::default()));
        app
    }

    fn strafe_app(
        preset: ControlPreset,
        yaw: f32,
        movement: Vec2,
        door: Option<LaunchDoor>,
        settings_open: Option<bool>,
        keys: &[KeyCode],
    ) -> App {
        let mut app = strafe_world(preset, yaw, movement, door, settings_open, keys);
        app.update();
        app.update();
        app
    }

    /// Default yaw 0: Modern W matches the pre-strafe yard step (-Z, no X).
    #[test]
    fn move_strafe_modern_default_yaw_moves_as_today() {
        assert_eq!(ControlPreset::default(), ControlPreset::Modern);
        let wish = camera_relative_wish(Vec2::new(0.0, 1.0), 0.0);
        assert!(wish.x.abs() < 1e-5 && (wish.z + 1.0).abs() < 1e-5);
        let mut app = strafe_app(
            ControlPreset::Modern,
            0.0,
            Vec2::new(0.0, 1.0),
            Some(LaunchDoor::InYard),
            Some(false),
            &[],
        );
        let (px, pz, vz) = {
            let presence = app.world().resource::<SoftPresence>();
            (presence.position.x, presence.position.z, presence.velocity.z)
        };
        let today = yard_w_app(Some(LaunchDoor::InYard), Some(false));
        let today_p = today.world().resource::<SoftPresence>();
        assert!(pz < 0.0);
        assert!(px.abs() < 1e-4);
        assert!((px - today_p.position.x).abs() < 1e-4);
        assert!((pz - today_p.position.z).abs() < 1e-4);
        assert!((vz - today_p.velocity.z).abs() < 1e-4);
        let front = body_front(&mut app);
        assert!(
            front.z < -0.9 && front.x.abs() < 1e-3,
            "Modern W faces camera forward, front={front:?}"
        );
    }

    /// Yaw 90°: W goes along the camera's forward (-X), same distance as yaw-0 -Z.
    #[test]
    fn move_strafe_modern_yaw_90_w_follows_camera_forward() {
        let yaw0 = strafe_app(
            ControlPreset::Modern,
            0.0,
            Vec2::new(0.0, 1.0),
            Some(LaunchDoor::InYard),
            Some(false),
            &[],
        );
        let mut yaw90 = strafe_app(
            ControlPreset::Modern,
            FRAC_PI_2,
            Vec2::new(0.0, 1.0),
            Some(LaunchDoor::InYard),
            Some(false),
            &[],
        );
        let p0z = yaw0.world().resource::<SoftPresence>().position.z;
        let (p90x, p90z, v90x, v90z) = {
            let p90 = yaw90.world().resource::<SoftPresence>();
            (p90.position.x, p90.position.z, p90.velocity.x, p90.velocity.z)
        };
        assert!(
            (p90x - p0z).abs() < 1e-3,
            "yaw 90 W x {p90x} should match yaw 0 z {p0z}"
        );
        assert!(p90z.abs() < 1e-3, "z={p90z}");
        assert!(v90x < 0.0);
        assert!(v90z.abs() < 1e-3);
        assert!(yaw_near(
            yaw90.world().resource::<BodyFacing>().yaw,
            camera_forward_yaw(FRAC_PI_2)
        ));
        let front = body_front(&mut yaw90);
        assert!(
            front.x < -0.9 && front.z.abs() < 1e-3,
            "body faces camera forward, front={front:?}"
        );
    }

    /// Classic keyboard A turns in place. It does not strafe.
    #[test]
    fn move_strafe_classic_a_turns_without_translating() {
        let mut app = strafe_app(
            ControlPreset::Classic,
            0.0,
            Vec2::new(-1.0, 0.0),
            Some(LaunchDoor::InYard),
            Some(false),
            &[KeyCode::KeyA],
        );
        let presence = app.world().resource::<SoftPresence>();
        assert!(presence.position.x.abs() < 1e-4, "x={}", presence.position.x);
        assert!(presence.position.z.abs() < 1e-4, "z={}", presence.position.z);
        assert!(presence.velocity.x.abs() < 1e-4);
        assert!(presence.velocity.z.abs() < 1e-4);
        let facing = app.world().resource::<BodyFacing>().yaw;
        let turned = wrap_pi(facing - PI);
        assert!(turned > 0.2, "A should turn left, delta={turned}");
        let orbit = app.world().resource::<FollowOrbit>().yaw;
        assert!(
            yaw_near(orbit, wrap_pi(facing + PI)),
            "orbit {orbit} should follow body {facing}"
        );
        let front = body_front(&mut app);
        let want = body_forward(facing);
        assert!(
            front.distance(want) < 1e-3,
            "front={front:?} want={want:?}"
        );
    }

    /// Classic W follows the body, even when that is not the camera forward.
    #[test]
    fn move_strafe_classic_w_follows_body_facing() {
        let mut app = strafe_world(
            ControlPreset::Classic,
            0.0,
            Vec2::new(0.0, 1.0),
            Some(LaunchDoor::InYard),
            Some(false),
            &[KeyCode::KeyW],
        );
        app.world_mut().resource_mut::<BodyFacing>().yaw = 0.0;
        app.update();
        app.update();
        let presence = app.world().resource::<SoftPresence>();
        assert!(
            presence.position.z > 0.0,
            "body facing +Z should step +Z, z={}",
            presence.position.z
        );
        assert!(presence.position.x.abs() < 1e-4);
        assert!(yaw_near(app.world().resource::<BodyFacing>().yaw, 0.0));
        assert!(yaw_near(app.world().resource::<FollowOrbit>().yaw, 0.0));
    }

    /// Pad and touch share `PlayerInput.movement`. Both stay camera-relative
    /// in Modern and Classic. Classic turning stays on the keyboard.
    #[test]
    fn move_strafe_pad_and_touch_camera_relative_in_both_presets() {
        let modern = strafe_app(
            ControlPreset::Modern,
            FRAC_PI_2,
            Vec2::new(0.0, 1.0),
            Some(LaunchDoor::InYard),
            Some(false),
            &[],
        );
        let classic = strafe_app(
            ControlPreset::Classic,
            FRAC_PI_2,
            Vec2::new(0.0, 1.0),
            Some(LaunchDoor::InYard),
            Some(false),
            &[],
        );
        let modern_p = modern.world().resource::<SoftPresence>();
        let classic_p = classic.world().resource::<SoftPresence>();
        assert!(modern_p.position.x < -0.2, "x={}", modern_p.position.x);
        assert!(modern_p.position.z.abs() < 1e-3);
        assert!((modern_p.position.x - classic_p.position.x).abs() < 1e-4);
        assert!((modern_p.position.z - classic_p.position.z).abs() < 1e-4);
        assert!(yaw_near(classic.world().resource::<BodyFacing>().yaw, PI));
        assert!(yaw_near(
            classic.world().resource::<FollowOrbit>().yaw,
            FRAC_PI_2
        ));

        let left = strafe_app(
            ControlPreset::Classic,
            0.0,
            Vec2::new(-1.0, 0.0),
            Some(LaunchDoor::InYard),
            Some(false),
            &[],
        );
        let left_p = left.world().resource::<SoftPresence>();
        assert!(
            left_p.position.x < -0.2,
            "stick left should strafe, x={}",
            left_p.position.x
        );
        assert!(left_p.position.z.abs() < 1e-3);
        assert!(yaw_near(left.world().resource::<BodyFacing>().yaw, PI));
        assert!(yaw_near(left.world().resource::<FollowOrbit>().yaw, 0.0));

        // Modern A/D is a strafe: the body keeps the camera forward, not the stick direction.
        let mut modern_left = strafe_app(
            ControlPreset::Modern,
            0.0,
            Vec2::new(-1.0, 0.0),
            Some(LaunchDoor::InYard),
            Some(false),
            &[],
        );
        let (mx, mz) = {
            let presence = modern_left.world().resource::<SoftPresence>();
            (presence.position.x, presence.position.z)
        };
        assert!(mx < -0.2);
        assert!(mz.abs() < 1e-3);
        let front = body_front(&mut modern_left);
        assert!(
            front.z < -0.9 && front.x.abs() < 1e-3,
            "strafe should not turn the body, front={front:?}"
        );
    }

    /// Title and settings still block wish and Classic A/D. Same gate as #736.
    #[test]
    fn move_strafe_title_and_settings_gate_holds() {
        let title = strafe_app(
            ControlPreset::Modern,
            FRAC_PI_2,
            Vec2::new(0.0, 1.0),
            Some(LaunchDoor::Title),
            Some(false),
            &[],
        );
        let title_p = title.world().resource::<SoftPresence>();
        assert_eq!(title_p.position.x, 0.0);
        assert_eq!(title_p.position.z, 0.0);
        assert_eq!(title_p.velocity.x, 0.0);
        assert_eq!(title_p.velocity.z, 0.0);

        let settings = strafe_app(
            ControlPreset::Modern,
            FRAC_PI_2,
            Vec2::new(0.0, 1.0),
            Some(LaunchDoor::InYard),
            Some(true),
            &[],
        );
        let settings_p = settings.world().resource::<SoftPresence>();
        assert_eq!(settings_p.position.x, 0.0);
        assert_eq!(settings_p.position.z, 0.0);
        assert_eq!(settings_p.velocity.x, 0.0);
        assert_eq!(settings_p.velocity.z, 0.0);

        let title_a = strafe_app(
            ControlPreset::Classic,
            0.0,
            Vec2::new(-1.0, 0.0),
            Some(LaunchDoor::Title),
            Some(false),
            &[KeyCode::KeyA],
        );
        assert!(yaw_near(title_a.world().resource::<BodyFacing>().yaw, PI));
        assert_eq!(title_a.world().resource::<FollowOrbit>().yaw, 0.0);
        assert_eq!(title_a.world().resource::<SoftPresence>().position.x, 0.0);
        assert_eq!(title_a.world().resource::<SoftPresence>().position.z, 0.0);

        let settings_a = strafe_app(
            ControlPreset::Classic,
            0.0,
            Vec2::new(-1.0, 0.0),
            Some(LaunchDoor::InYard),
            Some(true),
            &[KeyCode::KeyA],
        );
        assert!(yaw_near(
            settings_a.world().resource::<BodyFacing>().yaw,
            PI
        ));
        assert_eq!(settings_a.world().resource::<FollowOrbit>().yaw, 0.0);
        assert_eq!(
            settings_a.world().resource::<SoftPresence>().position.x,
            0.0
        );
        assert_eq!(
            settings_a.world().resource::<SoftPresence>().position.z,
            0.0
        );
    }

    /// CARD KEY-REBIND-1 — settings plus raw keys. Movement stays at zero so
    /// the axes, not a stick, are what step the body.
    fn rebind_app(preset: ControlPreset, settings: LocalSettings, keys: &[KeyCode]) -> App {
        let mut app = strafe_world(
            preset,
            0.0,
            Vec2::ZERO,
            Some(LaunchDoor::InYard),
            Some(false),
            keys,
        );
        app.insert_resource(LocalSettingsState {
            inner: settings,
            dirty: false,
        });
        app.update();
        app.update();
        app
    }

    fn xz_near(app: &App, x: f32, z: f32) -> bool {
        let presence = app.world().resource::<SoftPresence>();
        (presence.position.x - x).abs() < 1e-4 && (presence.position.z - z).abs() < 1e-4
    }

    /// Defaults: W and ArrowUp step the same as today's yard W. Classic A turns.
    #[test]
    fn key_rebind_defaults_match_wasd_and_arrows() {
        let today = yard_w_app(Some(LaunchDoor::InYard), Some(false));
        let (tx, tz) = {
            let presence = today.world().resource::<SoftPresence>();
            (presence.position.x, presence.position.z)
        };
        assert!(tz < 0.0, "yard W should step -Z, z={tz}");

        let keyed = rebind_app(
            ControlPreset::Modern,
            LocalSettings::default(),
            &[KeyCode::KeyW],
        );
        assert!(
            xz_near(&keyed, tx, tz),
            "default W left today's step, pos={:?}",
            keyed.world().resource::<SoftPresence>().position
        );

        let bare = strafe_app(
            ControlPreset::Modern,
            0.0,
            Vec2::ZERO,
            Some(LaunchDoor::InYard),
            Some(false),
            &[KeyCode::KeyW],
        );
        assert!(
            xz_near(&bare, tx, tz),
            "missing settings stay on the default map"
        );

        let arrows = rebind_app(
            ControlPreset::Modern,
            LocalSettings::default(),
            &[KeyCode::ArrowUp],
        );
        assert!(
            xz_near(&arrows, tx, tz),
            "ArrowUp stays the forward fallback"
        );

        let classic = rebind_app(
            ControlPreset::Classic,
            LocalSettings::default(),
            &[KeyCode::KeyA],
        );
        let turned = wrap_pi(classic.world().resource::<BodyFacing>().yaw - PI);
        assert!(
            turned > 0.2,
            "default A should still turn left, delta={turned}"
        );
        assert!(xz_near(&classic, 0.0, 0.0), "Classic A does not strafe");

        let arrow_left = rebind_app(
            ControlPreset::Classic,
            LocalSettings::default(),
            &[KeyCode::ArrowLeft],
        );
        assert!(
            yaw_near(
                arrow_left.world().resource::<BodyFacing>().yaw,
                classic.world().resource::<BodyFacing>().yaw
            ),
            "default ArrowLeft still turns with A"
        );
        assert!(xz_near(&arrow_left, 0.0, 0.0));
    }

    /// Remap forward to F: F moves, W does not, and an unbound arrow still does.
    #[test]
    fn key_rebind_forward_other_key_moves_and_w_does_not() {
        let today = yard_w_app(Some(LaunchDoor::InYard), Some(false));
        let (tx, tz) = {
            let presence = today.world().resource::<SoftPresence>();
            (presence.position.x, presence.position.z)
        };
        let mut settings = LocalSettings::default();
        settings.key_move_up = PeaceKey::F;

        let moved = rebind_app(ControlPreset::Modern, settings.clone(), &[KeyCode::KeyF]);
        assert!(
            xz_near(&moved, tx, tz),
            "F should step like W, pos={:?}",
            moved.world().resource::<SoftPresence>().position
        );

        let w = rebind_app(ControlPreset::Modern, settings.clone(), &[KeyCode::KeyW]);
        assert!(
            xz_near(&w, 0.0, 0.0),
            "W is no longer forward, pos={:?}",
            w.world().resource::<SoftPresence>().position
        );
        let w_vel = w.world().resource::<SoftPresence>().velocity;
        assert_eq!(w_vel.x, 0.0);
        assert_eq!(w_vel.z, 0.0);

        let arrow = rebind_app(ControlPreset::Modern, settings, &[KeyCode::ArrowUp]);
        assert!(
            xz_near(&arrow, tx, tz),
            "unbound ArrowUp stays a forward fallback"
        );
    }

    /// An arrow saved on another action does not also move or Classic-turn.
    #[test]
    fn key_rebind_arrow_bound_elsewhere_does_not_move() {
        let mut settings = LocalSettings::default();
        settings.key_use = PeaceKey::ArrowUp;
        let up = rebind_app(ControlPreset::Modern, settings.clone(), &[KeyCode::ArrowUp]);
        assert!(
            xz_near(&up, 0.0, 0.0),
            "ArrowUp bound to Use must not walk, pos={:?}",
            up.world().resource::<SoftPresence>().position
        );
        let up_vel = up.world().resource::<SoftPresence>().velocity;
        assert_eq!(up_vel.x, 0.0);
        assert_eq!(up_vel.z, 0.0);

        let w = rebind_app(ControlPreset::Modern, settings, &[KeyCode::KeyW]);
        assert!(
            w.world().resource::<SoftPresence>().position.z < 0.0,
            "W is still forward"
        );

        let mut classic = LocalSettings::default();
        classic.key_satchel = PeaceKey::ArrowLeft;
        let left = rebind_app(ControlPreset::Classic, classic, &[KeyCode::ArrowLeft]);
        assert!(
            yaw_near(left.world().resource::<BodyFacing>().yaw, PI),
            "ArrowLeft bound to satchel must not turn"
        );
        assert!(xz_near(&left, 0.0, 0.0));
    }

    /// Classic turning follows the saved left and right keys.
    #[test]
    fn key_rebind_classic_turns_on_remapped_keys() {
        let mut settings = LocalSettings::default();
        settings.key_move_left = PeaceKey::Q;
        settings.key_move_right = PeaceKey::C;

        let q = rebind_app(ControlPreset::Classic, settings.clone(), &[KeyCode::KeyQ]);
        let a = rebind_app(
            ControlPreset::Classic,
            LocalSettings::default(),
            &[KeyCode::KeyA],
        );
        assert!(
            yaw_near(
                q.world().resource::<BodyFacing>().yaw,
                a.world().resource::<BodyFacing>().yaw
            ),
            "Q should turn as far as default A"
        );
        assert!(xz_near(&q, 0.0, 0.0), "remapped left does not strafe");

        let old_a = rebind_app(ControlPreset::Classic, settings.clone(), &[KeyCode::KeyA]);
        assert!(
            yaw_near(old_a.world().resource::<BodyFacing>().yaw, PI),
            "A is no longer turn-left"
        );
        assert!(xz_near(&old_a, 0.0, 0.0));

        let c = rebind_app(ControlPreset::Classic, settings.clone(), &[KeyCode::KeyC]);
        let d = rebind_app(
            ControlPreset::Classic,
            LocalSettings::default(),
            &[KeyCode::KeyD],
        );
        assert!(
            yaw_near(
                c.world().resource::<BodyFacing>().yaw,
                d.world().resource::<BodyFacing>().yaw
            ),
            "C should turn as far as default D"
        );
        assert!(xz_near(&c, 0.0, 0.0));

        let old_d = rebind_app(ControlPreset::Classic, settings, &[KeyCode::KeyD]);
        assert!(
            yaw_near(old_d.world().resource::<BodyFacing>().yaw, PI),
            "D is no longer turn-right"
        );
        assert!(xz_near(&old_d, 0.0, 0.0));
    }

    /// `input.rs` clears a custom source after copying it onto canon W.
    /// The pre-remap snapshot still carries F, so forward still steps.
    #[test]
    fn key_rebind_mapped_forward_survives_canon_remap() {
        use bevy::input::keyboard::KeyboardInput;
        use bevy::input::mouse::MouseButtonInput;
        use bevy::input::touch::TouchInput;
        use bevy::time::TimeUpdateStrategy;

        let mut settings = LocalSettings::default();
        settings.key_move_up = PeaceKey::F;
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(Time::<Fixed>::from_hz(60.0))
            .insert_resource(TimeUpdateStrategy::FixedTimesteps(12))
            .add_message::<KeyboardInput>()
            .add_message::<MouseButtonInput>()
            .add_message::<TouchInput>()
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<SoftPresence>()
            .init_resource::<JumpLatch>()
            .init_resource::<MoveKeySnapshot>()
            .init_resource::<BodyFacing>()
            .init_resource::<FollowOrbit>()
            .insert_resource(ControlPreset::Modern)
            .insert_resource(LocalSettingsState {
                inner: settings,
                dirty: false,
            })
            .add_plugins(crate::input::InputPlugin)
            .add_systems(
                PreUpdate,
                snapshot_move_keys
                    .after(bevy::input::InputSystems)
                    .before(crate::input::PeaceRemapSet),
            )
            .add_systems(FixedUpdate, apply_locomotion);
        // The first update stamps the clock. Re-press so the stepping update
        // still sees F before remap clears it.
        for _ in 0..2 {
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(KeyCode::KeyF);
            app.update();
        }

        let keyboard = app.world().resource::<ButtonInput<KeyCode>>();
        assert!(
            !keyboard.pressed(KeyCode::KeyF),
            "remap consumes the custom forward key"
        );
        let presence = app.world().resource::<SoftPresence>();
        assert!(
            presence.position.z < 0.0,
            "saved forward still steps after remap, z={}",
            presence.position.z
        );
        assert!(presence.position.x.abs() < 1e-4);
    }

    fn quiet_house(settings_open: bool) -> HouseLabel {
        HouseLabel {
            house: shared::house_name::HouseName::default(),
            persist_present: false,
            hour_two_held: false,
            book_held: false,
            settings_open,
            draft: String::new(),
            naming_offered: false,
            seals_offered: false,
        }
    }

    /// Spawn the follow camera and the orbit resources. Door and house are optional.
    fn orbit_world(door: Option<LaunchDoor>, settings_open: Option<bool>) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<SoftPresence>()
            .init_resource::<FollowOrbit>()
            .init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<AccumulatedMouseMotion>()
            .init_resource::<AccumulatedMouseScroll>()
            .init_resource::<LocalLookFeel>()
            .insert_resource(LocalFeedbackFeel::default())
            .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
                std::time::Duration::from_secs_f64(1.0 / 60.0),
            ))
            .add_systems(Update, follow_camera);
        if let Some(door) = door {
            app.insert_resource(door);
        }
        if let Some(open) = settings_open {
            app.insert_resource(quiet_house(open));
        }
        app.world_mut().spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 8.0, 14.0),
        ));
        app
    }

    fn settle_follow(app: &mut App) {
        for _ in 0..80 {
            app.update();
        }
    }

    fn cam_translation(app: &mut App) -> Vec3 {
        let mut q = app.world_mut().query::<(&Transform, &Camera3d)>();
        q.iter(app.world()).next().expect("camera").0.translation
    }

    fn cam_up_y(app: &mut App) -> f32 {
        let mut q = app.world_mut().query::<(&Transform, &Camera3d)>();
        q.iter(app.world()).next().expect("camera").0.up().as_vec3().y
    }

    /// Defaults equal today's CAM_BACK / CAM_UP view.
    #[test]
    fn cam_mouse_orbit_default_matches_fixed_offset() {
        let orbit = FollowOrbit::default();
        let offset = orbit.offset();
        let fixed = Vec3::new(0.0, CAM_UP, CAM_BACK);
        assert!(
            offset.distance(fixed) < 1e-4,
            "default orbit {offset:?} left the fixed offset {fixed:?}"
        );
        assert!(default_orbit_pitch() > CAM_PITCH_MIN);
        assert!(default_orbit_pitch() < CAM_PITCH_MAX);
        assert!(default_orbit_distance() > CAM_DIST_MIN);
        assert!(default_orbit_distance() < CAM_DIST_MAX);
    }

    /// A player who never touches the mouse settles on the same follow point.
    #[test]
    fn cam_mouse_orbit_untouched_settles_on_fixed_view() {
        let mut app = orbit_world(Some(LaunchDoor::InYard), Some(false));
        settle_follow(&mut app);
        let presence = app.world().resource::<SoftPresence>().position;
        let want = presence + Vec3::new(0.0, CAM_UP, CAM_BACK);
        let got = cam_translation(&mut app);
        assert!(
            got.distance(want) < 0.02,
            "untouched camera {got:?} left {want:?}"
        );
        assert!(cam_up_y(&mut app) > 0.0, "default look flipped");
    }

    /// Hold right mouse plus motion changes yaw and pitch.
    #[test]
    fn cam_mouse_orbit_rmb_motion_changes_yaw_and_pitch() {
        let mut app = orbit_world(Some(LaunchDoor::InYard), Some(false));
        {
            let world = app.world_mut();
            world
                .resource_mut::<ButtonInput<MouseButton>>()
                .press(MouseButton::Right);
            world.resource_mut::<AccumulatedMouseMotion>().delta = Vec2::new(30.0, 20.0);
        }
        app.update();
        let orbit = *app.world().resource::<FollowOrbit>();
        assert!(
            (orbit.yaw - (-30.0 * ORBIT_YAW_PER_PX)).abs() < 1e-4,
            "mouse right should turn the view right, yaw={}",
            orbit.yaw
        );
        assert!(
            (orbit.pitch - (default_orbit_pitch() + 20.0 * ORBIT_PITCH_PER_PX)).abs() < 1e-4,
            "mouse down should raise the camera, pitch={}",
            orbit.pitch
        );
        assert!((orbit.distance - default_orbit_distance()).abs() < 1e-4);
    }

    /// Motion without the right button does not change yaw or pitch.
    #[test]
    fn cam_mouse_orbit_motion_without_rmb_holds_angles() {
        let mut app = orbit_world(Some(LaunchDoor::InYard), Some(false));
        app.world_mut().resource_mut::<AccumulatedMouseMotion>().delta = Vec2::new(80.0, -40.0);
        app.update();
        let orbit = *app.world().resource::<FollowOrbit>();
        assert_eq!(orbit.yaw, 0.0);
        assert!((orbit.pitch - default_orbit_pitch()).abs() < 1e-5);
        assert!((orbit.distance - default_orbit_distance()).abs() < 1e-5);
    }

    /// Pitch cannot flip over the pole or drop the camera under the ground.
    #[test]
    fn cam_mouse_orbit_pitch_clamped_no_flip_or_underground() {
        let mut app = orbit_world(Some(LaunchDoor::InYard), Some(false));
        {
            let world = app.world_mut();
            world
                .resource_mut::<ButtonInput<MouseButton>>()
                .press(MouseButton::Right);
            world.resource_mut::<AccumulatedMouseMotion>().delta = Vec2::new(0.0, -100_000.0);
        }
        app.update();
        let low = *app.world().resource::<FollowOrbit>();
        assert_eq!(low.pitch, CAM_PITCH_MIN);
        assert!(low.offset().y > 0.0, "pitch floor still buried the camera");
        app.world_mut().resource_mut::<AccumulatedMouseMotion>().delta = Vec2::ZERO;
        settle_follow(&mut app);
        let grounded = cam_translation(&mut app);
        assert!(
            grounded.y > 0.15,
            "camera went under the ground at y={}",
            grounded.y
        );
        assert!(cam_up_y(&mut app) > 0.0, "low pitch flipped the camera");

        app.world_mut().resource_mut::<AccumulatedMouseMotion>().delta = Vec2::new(0.0, 100_000.0);
        app.update();
        let high = *app.world().resource::<FollowOrbit>();
        assert_eq!(high.pitch, CAM_PITCH_MAX);
        assert!(high.pitch < FRAC_PI_2);
        assert!(high.pitch.cos() > 0.0, "pitch reached the pole");
        app.world_mut().resource_mut::<AccumulatedMouseMotion>().delta = Vec2::ZERO;
        settle_follow(&mut app);
        let up_y = cam_up_y(&mut app);
        // Near overhead, world-up on the camera is small but still positive.
        // A flip would point up.y down through the ground.
        assert!(up_y > 0.0, "high pitch flipped the camera, up.y={up_y}");
        let overhead = cam_translation(&mut app);
        assert!(
            overhead.y > STAND,
            "high pitch dropped the camera, y={}",
            overhead.y
        );
    }

    /// Scroll changes distance and clamps to min and max. The right button is not required.
    #[test]
    fn cam_mouse_orbit_scroll_clamps_distance() {
        let mut app = orbit_world(Some(LaunchDoor::InYard), Some(false));
        {
            let mut scroll = app.world_mut().resource_mut::<AccumulatedMouseScroll>();
            scroll.unit = MouseScrollUnit::Line;
            scroll.delta = Vec2::new(0.0, 1.0);
        }
        app.update();
        let zoomed = app.world().resource::<FollowOrbit>().distance;
        assert!(
            (zoomed - (default_orbit_distance() - ORBIT_DIST_PER_LINE)).abs() < 1e-4,
            "wheel up should zoom in, distance={zoomed}"
        );

        {
            let mut scroll = app.world_mut().resource_mut::<AccumulatedMouseScroll>();
            scroll.unit = MouseScrollUnit::Pixel;
            scroll.delta = Vec2::new(0.0, ORBIT_PIXELS_PER_LINE);
        }
        app.update();
        let pixels = app.world().resource::<FollowOrbit>().distance;
        assert!(
            (pixels - (default_orbit_distance() - 2.0 * ORBIT_DIST_PER_LINE)).abs() < 1e-4,
            "100 px should match one line, distance={pixels}"
        );

        {
            let mut scroll = app.world_mut().resource_mut::<AccumulatedMouseScroll>();
            scroll.unit = MouseScrollUnit::Line;
            scroll.delta = Vec2::new(0.0, 10_000.0);
        }
        app.update();
        assert_eq!(app.world().resource::<FollowOrbit>().distance, CAM_DIST_MIN);

        app.world_mut().resource_mut::<AccumulatedMouseScroll>().delta = Vec2::new(0.0, -10_000.0);
        app.update();
        assert_eq!(app.world().resource::<FollowOrbit>().distance, CAM_DIST_MAX);

        let orbit = *app.world().resource::<FollowOrbit>();
        assert_eq!(orbit.yaw, 0.0);
        assert!((orbit.pitch - default_orbit_pitch()).abs() < 1e-5);
    }

    fn mouse_orbit_attempt(app: &mut App) {
        {
            let world = app.world_mut();
            world
                .resource_mut::<ButtonInput<MouseButton>>()
                .press(MouseButton::Right);
            world.resource_mut::<AccumulatedMouseMotion>().delta = Vec2::new(40.0, 25.0);
            let mut scroll = world.resource_mut::<AccumulatedMouseScroll>();
            scroll.unit = MouseScrollUnit::Line;
            scroll.delta = Vec2::new(0.0, 2.0);
        }
        app.update();
    }

    fn assert_orbit_still_default(app: &App) {
        let orbit = *app.world().resource::<FollowOrbit>();
        assert_eq!(orbit.yaw, 0.0);
        assert!((orbit.pitch - default_orbit_pitch()).abs() < 1e-5);
        assert!((orbit.distance - default_orbit_distance()).abs() < 1e-5);
    }

    /// No orbit on the title screen.
    #[test]
    fn cam_mouse_orbit_title_ignores_mouse() {
        let mut app = orbit_world(Some(LaunchDoor::Title), Some(false));
        mouse_orbit_attempt(&mut app);
        assert_orbit_still_default(&app);
    }

    /// No orbit while settings are open.
    #[test]
    fn cam_mouse_orbit_settings_open_ignores_mouse() {
        let mut app = orbit_world(Some(LaunchDoor::InYard), Some(true));
        mouse_orbit_attempt(&mut app);
        assert_orbit_still_default(&app);
    }

    /// Same gate as #736: NameHouse is not the open yard.
    #[test]
    fn cam_mouse_orbit_name_house_ignores_mouse() {
        let mut app = orbit_world(Some(LaunchDoor::NameHouse), Some(false));
        mouse_orbit_attempt(&mut app);
        assert_orbit_still_default(&app);
    }

    /// A missing door or house keeps today's follow, so the mouse still orbits.
    #[test]
    fn cam_mouse_orbit_missing_door_still_reads_mouse() {
        let mut app = orbit_world(None, None);
        mouse_orbit_attempt(&mut app);
        let orbit = *app.world().resource::<FollowOrbit>();
        assert!(orbit.yaw < 0.0);
        assert!(orbit.pitch > default_orbit_pitch());
        assert!(orbit.distance < default_orbit_distance());
    }

    /// PlayerInput does not orbit. Right-stick look reads the pad axes.
    #[test]
    fn cam_mouse_orbit_player_input_does_not_orbit() {
        let mut app = orbit_world(Some(LaunchDoor::InYard), Some(false));
        app.insert_resource(PlayerInput {
            movement: Vec2::new(1.0, 1.0),
            interact: true,
            interact_held: true,
            jump: true,
            sprint: true,
            ..Default::default()
        });
        app.update();
        assert_orbit_still_default(&app);
    }

    /// Ambrosian look-down keeps the fixed offset. An orbited yaw does not move it.
    #[test]
    fn cam_mouse_orbit_ambrosian_look_down_keeps_fixed_offset() {
        let mut app = orbit_world(Some(LaunchDoor::InYard), Some(false));
        let ambro = Vec3::new(1.0, sanctuary_yard_wake().y + AMBROSIAN_LIFT_DETECT + 1.0, -2.0);
        let fixed = ambro + Vec3::new(0.0, CAM_UP + 1.1, CAM_BACK - 1.4);
        app.world_mut().resource_mut::<SoftPresence>().position = ambro;
        app.world_mut().resource_mut::<FollowOrbit>().yaw = FRAC_PI_2;
        {
            let mut q = app.world_mut().query::<&mut Transform>();
            q.iter_mut(app.world_mut()).next().expect("camera").translation = fixed;
        }
        app.update();
        let got = cam_translation(&mut app);
        assert!(
            got.distance(fixed) < 1e-3,
            "Ambrosian look-down left the fixed offset, got {got:?} want {fixed:?}"
        );
    }

    /// Saved look defaults (sensitivity 1, invert off) keep today's per-pixel orbit.
    #[test]
    fn cam_feel_default_look_matches_today_orbit() {
        let feel = LocalLookFeel::default();
        assert!((feel.sensitivity - 1.0).abs() < 1e-6);
        assert!(!feel.invert_y);
        let mut app = orbit_world(Some(LaunchDoor::InYard), Some(false));
        {
            let world = app.world_mut();
            world
                .resource_mut::<ButtonInput<MouseButton>>()
                .press(MouseButton::Right);
            world.resource_mut::<AccumulatedMouseMotion>().delta = Vec2::new(30.0, 20.0);
        }
        app.update();
        let orbit = *app.world().resource::<FollowOrbit>();
        assert!((orbit.yaw - (-30.0 * ORBIT_YAW_PER_PX)).abs() < 1e-4);
        assert!((orbit.pitch - (default_orbit_pitch() + 20.0 * ORBIT_PITCH_PER_PX)).abs() < 1e-4);
        assert!((orbit.distance - default_orbit_distance()).abs() < 1e-4);
    }

    /// Look sensitivity scales yaw and pitch. Distance stays on the raw scroll rate.
    #[test]
    fn cam_feel_sensitivity_scales_yaw_and_pitch() {
        let mut app = orbit_world(Some(LaunchDoor::InYard), Some(false));
        app.insert_resource(LocalLookFeel {
            sensitivity: 2.0,
            invert_y: false,
            mouse_look: false,
            raw_look: false,
        });
        {
            let world = app.world_mut();
            world
                .resource_mut::<ButtonInput<MouseButton>>()
                .press(MouseButton::Right);
            world.resource_mut::<AccumulatedMouseMotion>().delta = Vec2::new(30.0, 20.0);
            let mut scroll = world.resource_mut::<AccumulatedMouseScroll>();
            scroll.unit = MouseScrollUnit::Line;
            scroll.delta = Vec2::new(0.0, 1.0);
        }
        app.update();
        let orbit = *app.world().resource::<FollowOrbit>();
        assert!(
            (orbit.yaw - (-30.0 * 2.0 * ORBIT_YAW_PER_PX)).abs() < 1e-4,
            "sensitivity should scale yaw, got {}",
            orbit.yaw
        );
        assert!(
            (orbit.pitch - (default_orbit_pitch() + 20.0 * 2.0 * ORBIT_PITCH_PER_PX)).abs()
                < 1e-4,
            "sensitivity should scale pitch, got {}",
            orbit.pitch
        );
        assert!(
            (orbit.distance - (default_orbit_distance() - ORBIT_DIST_PER_LINE)).abs() < 1e-4,
            "sensitivity must not scale zoom, distance={}",
            orbit.distance
        );
    }

    /// Invert-Y flips pitch and leaves yaw on the same sign.
    #[test]
    fn cam_feel_invert_y_flips_pitch_only() {
        let mut app = orbit_world(Some(LaunchDoor::InYard), Some(false));
        app.insert_resource(LocalLookFeel {
            sensitivity: 1.0,
            invert_y: true,
            mouse_look: false,
            raw_look: false,
        });
        {
            let world = app.world_mut();
            world
                .resource_mut::<ButtonInput<MouseButton>>()
                .press(MouseButton::Right);
            world.resource_mut::<AccumulatedMouseMotion>().delta = Vec2::new(30.0, 20.0);
        }
        app.update();
        let orbit = *app.world().resource::<FollowOrbit>();
        assert!((orbit.yaw - (-30.0 * ORBIT_YAW_PER_PX)).abs() < 1e-4);
        assert!(
            (orbit.pitch - (default_orbit_pitch() - 20.0 * ORBIT_PITCH_PER_PX)).abs() < 1e-4,
            "invert-Y should flip pitch, got {}",
            orbit.pitch
        );
    }

    /// Yaw and pitch radians the stick delta becomes through the same per-pixel rates.
    fn stick_orbit_radians(x: f32, y: f32, dt: f32, sens: f32, invert_y: bool) -> (f32, f32) {
        let (dx, dy) = stick_look_delta(x, y, dt, sens, invert_y);
        (-dx * ORBIT_YAW_PER_PX, dy * ORBIT_PITCH_PER_PX)
    }

    fn spawn_right_stick(app: &mut App, x: f32, y: f32) {
        let mut pad = Gamepad::default();
        pad.analog_mut().set(GamepadAxis::RightStickX, x);
        pad.analog_mut().set(GamepadAxis::RightStickY, y);
        app.world_mut().spawn(pad);
    }

    /// Inside the 0.18 deadzone, and a non-finite dt, the delta is exactly 0.
    /// One axis inside the zone does not mute the other.
    #[test]
    fn stick_look_inside_deadzone_is_zero() {
        assert_eq!(crate::input::STICK_DEADZONE, 0.18);
        assert_eq!(stick_look_delta(0.1, -0.1, 1.0, 1.0, false), (0.0, 0.0));
        assert_eq!(stick_look_delta(0.18, 0.18, 1.0, 1.0, false), (0.0, 0.0));
        assert_eq!(stick_look_delta(-0.18, 0.0, 1.0, 1.0, false), (0.0, 0.0));
        assert_eq!(stick_look_delta(0.0, 0.0, 1.0, 1.0, false), (0.0, 0.0));
        assert_eq!(stick_look_delta(1.0, 1.0, f32::NAN, 1.0, false), (0.0, 0.0));
        assert_eq!(
            stick_look_delta(1.0, 1.0, f32::INFINITY, 1.0, false),
            (0.0, 0.0)
        );
        let (dx, dy) = stick_look_delta(0.1, 1.0, 1.0, 1.0, false);
        assert_eq!(dx, 0.0);
        let pitch = dy * ORBIT_PITCH_PER_PX;
        assert!(
            (pitch - (-STICK_LOOK_PITCH_RATE)).abs() < 1e-4,
            "live pitch axis should stay full, got {pitch}"
        );
    }

    /// Full deflection for 1 s at look_sens 1. Stick right yaws −2.5. Stick up pitches −1.875.
    #[test]
    fn stick_look_full_deflection_one_second_matches_rates() {
        let (yaw, pitch) = stick_orbit_radians(1.0, 1.0, 1.0, 1.0, false);
        assert!((yaw - (-STICK_LOOK_YAW_RATE)).abs() < 1e-4, "yaw {yaw}");
        assert!(
            (pitch - (-STICK_LOOK_PITCH_RATE)).abs() < 1e-4,
            "pitch {pitch}"
        );
        assert!((STICK_LOOK_YAW_RATE - 2.5).abs() < 1e-6);
        assert!((STICK_LOOK_PITCH_RATE - 1.875).abs() < 1e-6);
    }

    /// The live band past 0.18 maps onto 0..1. Halfway is half rate, not the raw axis.
    #[test]
    fn stick_look_deadzone_rescales_to_unit_range() {
        let deadzone = crate::input::STICK_DEADZONE;
        let mid = deadzone + 0.5 * (1.0 - deadzone);
        let (yaw, pitch) = stick_orbit_radians(mid, -mid, 1.0, 1.0, false);
        assert!((yaw - (-1.25)).abs() < 1e-4, "yaw {yaw}");
        assert!((pitch - 0.9375).abs() < 1e-4, "pitch {pitch}");
        let (full_yaw, _) = stick_orbit_radians(2.0, 0.0, 1.0, 1.0, false);
        assert!(
            (full_yaw - (-STICK_LOOK_YAW_RATE)).abs() < 1e-4,
            "past full scale stays at the rate, yaw {full_yaw}"
        );
    }

    /// look_sens 2 doubles yaw and pitch.
    #[test]
    fn stick_look_sensitivity_two_doubles_both() {
        let (yaw, pitch) = stick_orbit_radians(1.0, 1.0, 1.0, 2.0, false);
        assert!((yaw - (-5.0)).abs() < 1e-4, "yaw {yaw}");
        assert!((pitch - (-3.75)).abs() < 1e-4, "pitch {pitch}");
    }

    /// Invert-Y flips the pitch sign and leaves yaw alone.
    #[test]
    fn stick_look_invert_y_flips_pitch_sign_only() {
        let (yaw, pitch) = stick_orbit_radians(1.0, 1.0, 1.0, 1.0, true);
        assert!((yaw - (-2.5)).abs() < 1e-4, "yaw {yaw}");
        assert!((pitch - 1.875).abs() < 1e-4, "pitch {pitch}");
    }

    /// Mouse-only orbit, and a centred stick beside it, stay on today's pixel rates.
    #[test]
    fn stick_look_mouse_only_result_is_unchanged() {
        let look = LocalLookFeel {
            sensitivity: 1.0,
            invert_y: false,
            mouse_look: false,
            raw_look: false,
        };
        let plain = orbit_after(look, true, false, Vec2::new(30.0, 20.0));
        assert!((plain.yaw - (-30.0 * ORBIT_YAW_PER_PX)).abs() < 1e-4);
        assert!((plain.pitch - (default_orbit_pitch() + 20.0 * ORBIT_PITCH_PER_PX)).abs() < 1e-4);
        assert!((plain.distance - default_orbit_distance()).abs() < 1e-4);

        let mut with_stick = FollowOrbit::default();
        let mut buttons = ButtonInput::default();
        buttons.press(MouseButton::Right);
        let motion = AccumulatedMouseMotion {
            delta: Vec2::new(30.0, 20.0),
        };
        let scroll = AccumulatedMouseScroll::default();
        apply_stick_look(&mut with_stick, 0.1, -0.1, 1.0, &look);
        apply_orbit_input(
            &mut with_stick,
            &buttons,
            &motion,
            &scroll,
            ORBIT_PIXELS_PER_LINE,
            &look,
            false,
        );
        assert!((with_stick.yaw - plain.yaw).abs() < 1e-6);
        assert!((with_stick.pitch - plain.pitch).abs() < 1e-6);
        assert!((with_stick.distance - plain.distance).abs() < 1e-6);
    }

    /// The existing pitch clamp still bounds a full second of stick look.
    #[test]
    fn stick_look_pitch_clamp_still_applies() {
        let look = LocalLookFeel {
            sensitivity: 1.0,
            invert_y: false,
            mouse_look: false,
            raw_look: false,
        };
        let mut up = FollowOrbit::default();
        let buttons = ButtonInput::default();
        let motion = AccumulatedMouseMotion::default();
        let scroll = AccumulatedMouseScroll::default();
        apply_stick_look(&mut up, 1.0, 1.0, 1.0, &look);
        apply_orbit_input(
            &mut up,
            &buttons,
            &motion,
            &scroll,
            ORBIT_PIXELS_PER_LINE,
            &look,
            false,
        );
        assert!((up.yaw - (-2.5)).abs() < 1e-3, "yaw {}", up.yaw);
        assert_eq!(up.pitch, CAM_PITCH_MIN);

        let mut down = FollowOrbit::default();
        apply_stick_look(&mut down, 0.0, -1.0, 1.0, &look);
        apply_orbit_input(
            &mut down,
            &buttons,
            &motion,
            &scroll,
            ORBIT_PIXELS_PER_LINE,
            &look,
            false,
        );
        assert_eq!(down.pitch, CAM_PITCH_MAX);
    }

    /// Manual time's first frame has delta 0. The next frame carries 1/60.
    fn step_primed(app: &mut App) -> f32 {
        app.update();
        app.update();
        let dt = app.world().resource::<Time>().delta_secs();
        assert!(dt > 0.0, "primed frame should have a timestep");
        dt
    }

    /// The first pad's right stick orbits in the open yard. Title and settings do not.
    #[test]
    fn stick_look_right_stick_orbits_in_the_open_yard_only() {
        let mut yard = orbit_world(Some(LaunchDoor::InYard), Some(false));
        spawn_right_stick(&mut yard, 1.0, 0.0);
        let dt = step_primed(&mut yard);
        let orbit = *yard.world().resource::<FollowOrbit>();
        assert!(
            (orbit.yaw - (-STICK_LOOK_YAW_RATE * dt)).abs() < 1e-4,
            "yard stick yaw {} dt {dt}",
            orbit.yaw
        );
        assert!((orbit.pitch - default_orbit_pitch()).abs() < 1e-4);

        let mut title = orbit_world(Some(LaunchDoor::Title), Some(false));
        spawn_right_stick(&mut title, 1.0, 1.0);
        step_primed(&mut title);
        assert_orbit_still_default(&title);

        let mut settings = orbit_world(Some(LaunchDoor::InYard), Some(true));
        spawn_right_stick(&mut settings, 1.0, 1.0);
        step_primed(&mut settings);
        assert_orbit_still_default(&settings);
    }

    /// A centred first pad hides a deflected second pad.
    #[test]
    fn stick_look_first_pad_only() {
        let mut app = orbit_world(Some(LaunchDoor::InYard), Some(false));
        spawn_right_stick(&mut app, 0.0, 0.0);
        spawn_right_stick(&mut app, 1.0, 1.0);
        step_primed(&mut app);
        assert_orbit_still_default(&app);
    }

    fn look_feel(mouse_look: bool) -> LocalLookFeel {
        LocalLookFeel {
            sensitivity: 1.0,
            invert_y: false,
            mouse_look,
            raw_look: false,
        }
    }

    fn orbit_after(look: LocalLookFeel, rmb: bool, grabbed: bool, delta: Vec2) -> FollowOrbit {
        let mut orbit = FollowOrbit::default();
        let mut buttons = ButtonInput::default();
        if rmb {
            buttons.press(MouseButton::Right);
        }
        let motion = AccumulatedMouseMotion { delta };
        let scroll = AccumulatedMouseScroll::default();
        apply_orbit_input(
            &mut orbit,
            &buttons,
            &motion,
            &scroll,
            ORBIT_PIXELS_PER_LINE,
            &look,
            grabbed,
        );
        orbit
    }

    /// Grab only when mouse-look is on, in the yard, menu closed, HUD edit off, focused.
    /// CARD CAM-RAW — raw look is 1:1 (weight 1); off keeps the follow lerp.
    #[test]
    fn look_follow_t_raw_skips_smoothing() {
        let dt = 1.0 / 60.0;
        assert!((look_follow_t(dt, true) - 1.0).abs() < f32::EPSILON);
        assert!((look_follow_t(dt, false) - follow_lerp_t(dt)).abs() < f32::EPSILON);
        assert!((look_follow_t(dt, false) - FOLLOW_LERP_AT_60).abs() < 1e-5);
    }

    #[test]
    fn mouse_look_grab_eligible_only_in_open_focused_yard() {
        assert!(mouse_look_grab_eligible(true, true, false, false, true));
        assert!(!mouse_look_grab_eligible(false, true, false, false, true));
        assert!(!mouse_look_grab_eligible(true, false, false, false, true));
        assert!(!mouse_look_grab_eligible(true, true, true, false, true));
        assert!(!mouse_look_grab_eligible(true, true, false, true, true));
        assert!(!mouse_look_grab_eligible(true, true, false, false, false));
    }

    /// Eligible asks for Locked and a hidden cursor. Otherwise None and visible.
    /// Confined still counts as grabbed: that is the platform fallback for Locked.
    #[test]
    fn mouse_look_cursor_locks_and_confined_counts_as_grabbed() {
        let (grab, visible) = mouse_look_cursor(true);
        assert_eq!(grab, CursorGrabMode::Locked);
        assert!(!visible);
        let (grab, visible) = mouse_look_cursor(false);
        assert_eq!(grab, CursorGrabMode::None);
        assert!(visible);
        assert!(cursor_counts_as_grabbed(CursorGrabMode::Locked));
        assert!(cursor_counts_as_grabbed(CursorGrabMode::Confined));
        assert!(!cursor_counts_as_grabbed(CursorGrabMode::None));
    }

    /// mouse_look off keeps the RMB orbit and ignores a grabbed cursor.
    #[test]
    fn mouse_look_off_keeps_rmb_orbit_and_ignores_grab() {
        let delta = Vec2::new(30.0, 20.0);
        let off = look_feel(false);
        let held = orbit_after(off, false, true, delta);
        assert_eq!(held.yaw, 0.0);
        assert!((held.pitch - default_orbit_pitch()).abs() < 1e-5);
        let rmb = orbit_after(off, true, false, delta);
        assert!((rmb.yaw - (-30.0 * ORBIT_YAW_PER_PX)).abs() < 1e-4);
        assert!((rmb.pitch - (default_orbit_pitch() + 20.0 * ORBIT_PITCH_PER_PX)).abs() < 1e-4);
        let both = orbit_after(off, true, true, delta);
        assert!((both.yaw - rmb.yaw).abs() < 1e-6);
        assert!((both.pitch - rmb.pitch).abs() < 1e-6);
    }

    /// mouse_look on orbits without RMB only while the cursor is grabbed.
    #[test]
    fn mouse_look_grabbed_orbits_without_rmb() {
        let delta = Vec2::new(30.0, 20.0);
        let on = look_feel(true);
        let free = orbit_after(on, false, false, delta);
        assert_eq!(free.yaw, 0.0);
        assert!((free.pitch - default_orbit_pitch()).abs() < 1e-5);
        let grabbed = orbit_after(on, false, true, delta);
        assert!((grabbed.yaw - (-30.0 * ORBIT_YAW_PER_PX)).abs() < 1e-4);
        assert!(
            (grabbed.pitch - (default_orbit_pitch() + 20.0 * ORBIT_PITCH_PER_PX)).abs() < 1e-4
        );
        let rmb = orbit_after(on, true, false, delta);
        assert!((rmb.yaw - grabbed.yaw).abs() < 1e-6);
        assert!((rmb.pitch - grabbed.pitch).abs() < 1e-6);
    }

    fn spawn_primary_cursor(app: &mut App, hit_test: bool) {
        app.world_mut().spawn((
            Window::default(),
            PrimaryWindow,
            CursorOptions {
                visible: true,
                grab_mode: CursorGrabMode::None,
                hit_test,
            },
        ));
    }

    fn primary_cursor(app: &mut App) -> (CursorGrabMode, bool, bool) {
        let mut q = app
            .world_mut()
            .query_filtered::<&CursorOptions, With<PrimaryWindow>>();
        let cursor = q.iter(app.world()).next().expect("primary cursor");
        (cursor.grab_mode, cursor.visible, cursor.hit_test)
    }

    fn grab_app(mouse_look: bool, door: LaunchDoor, settings_open: bool) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(look_feel(mouse_look))
            .insert_resource(door)
            .insert_resource(quiet_house(settings_open))
            .add_systems(Update, sync_mouse_look_grab);
        spawn_primary_cursor(&mut app, false);
        app
    }

    /// Eligible yard grabs Locked and hides the cursor. Settings, HUD edit,
    /// title, and a lost focus release. The next eligible frame grabs again.
    /// mouse_look off leaves the cursor where it was, including hit_test.
    #[test]
    fn sync_mouse_look_grab_releases_until_the_next_eligible_frame() {
        let mut off = grab_app(false, LaunchDoor::InYard, false);
        off.update();
        assert_eq!(primary_cursor(&mut off), (CursorGrabMode::None, true, false));

        let mut title = grab_app(true, LaunchDoor::Title, false);
        title.update();
        assert_eq!(
            primary_cursor(&mut title),
            (CursorGrabMode::None, true, false)
        );

        let mut app = grab_app(true, LaunchDoor::InYard, true);
        app.update();
        assert_eq!(primary_cursor(&mut app), (CursorGrabMode::None, true, false));

        app.world_mut().resource_mut::<HouseLabel>().settings_open = false;
        app.update();
        assert_eq!(
            primary_cursor(&mut app),
            (CursorGrabMode::Locked, false, false)
        );

        {
            let mut edit = crate::hud_edit_mode::HudEditMode::default();
            edit.active = true;
            app.insert_resource(edit);
        }
        app.update();
        assert_eq!(primary_cursor(&mut app), (CursorGrabMode::None, true, false));

        app.world_mut()
            .resource_mut::<crate::hud_edit_mode::HudEditMode>()
            .active = false;
        app.update();
        assert_eq!(
            primary_cursor(&mut app),
            (CursorGrabMode::Locked, false, false)
        );

        {
            let mut q = app
                .world_mut()
                .query_filtered::<&mut Window, With<PrimaryWindow>>();
            q.iter_mut(app.world_mut()).next().expect("window").focused = false;
        }
        app.update();
        assert_eq!(primary_cursor(&mut app), (CursorGrabMode::None, true, false));

        {
            let mut q = app
                .world_mut()
                .query_filtered::<&mut Window, With<PrimaryWindow>>();
            q.iter_mut(app.world_mut()).next().expect("window").focused = true;
        }
        app.update();
        assert_eq!(
            primary_cursor(&mut app),
            (CursorGrabMode::Locked, false, false)
        );
    }

    /// The grab system and the orbit share one frame: grabbed mouse-look turns
    /// without RMB. mouse_look off does not, and the cursor stays free.
    #[test]
    fn sync_mouse_look_grab_orbits_when_grabbed_and_stays_idle_when_off() {
        let mut on = orbit_world(Some(LaunchDoor::InYard), Some(false));
        on.insert_resource(look_feel(true));
        on.add_systems(Update, sync_mouse_look_grab.before(follow_camera));
        spawn_primary_cursor(&mut on, true);
        on.world_mut().resource_mut::<AccumulatedMouseMotion>().delta = Vec2::new(30.0, 20.0);
        on.update();
        let orbit = *on.world().resource::<FollowOrbit>();
        assert!((orbit.yaw - (-30.0 * ORBIT_YAW_PER_PX)).abs() < 1e-4);
        assert!((orbit.pitch - (default_orbit_pitch() + 20.0 * ORBIT_PITCH_PER_PX)).abs() < 1e-4);
        assert_eq!(
            primary_cursor(&mut on),
            (CursorGrabMode::Locked, false, true)
        );

        let mut idle = orbit_world(Some(LaunchDoor::InYard), Some(false));
        idle.insert_resource(look_feel(false));
        idle.add_systems(Update, sync_mouse_look_grab.before(follow_camera));
        spawn_primary_cursor(&mut idle, true);
        idle.world_mut().resource_mut::<AccumulatedMouseMotion>().delta = Vec2::new(80.0, -40.0);
        idle.update();
        assert_orbit_still_default(&idle);
        assert_eq!(
            primary_cursor(&mut idle),
            (CursorGrabMode::None, true, true)
        );
    }

    fn follow_after(dt_secs: f32, steps: u32) -> Vec3 {
        let mut app = orbit_world(Some(LaunchDoor::InYard), Some(false));
        app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f32(dt_secs),
        ));
        // The first update only stamps the clock, so its delta stays zero.
        app.update();
        for _ in 0..steps {
            app.update();
        }
        cam_translation(&mut app)
    }

    /// The 60 fps weight stays 0.12, and 30 fps matches 144 fps over the same time.
    #[test]
    fn cam_feel_follow_matches_at_30_and_144_fps() {
        let at_60 = follow_lerp_t(1.0 / 60.0);
        assert!(
            (at_60 - FOLLOW_LERP_AT_60).abs() < 1e-5,
            "60 fps weight left 0.12, got {at_60}"
        );

        let start = Vec3::new(0.0, 8.0, 14.0);
        let one = follow_after(1.0 / 60.0, 1);
        let presence = Vec3::new(0.0, STAND, 0.0);
        let want = presence + Vec3::new(0.0, CAM_UP, CAM_BACK);
        let expected = start.lerp(want, FOLLOW_LERP_AT_60);
        assert!(
            one.distance(expected) < 1e-3,
            "one 60 fps frame left the legacy 0.12 step, got {one:?} want {expected:?}"
        );

        // 5/30 s = 24/144 s. Not settled, so a no-op lerp cannot pass.
        let slow = follow_after(1.0 / 30.0, 5);
        let fast = follow_after(1.0 / 144.0, 24);
        assert!(
            slow.distance(fast) < 1e-3,
            "30 fps {slow:?} left 144 fps {fast:?} over the same time"
        );
        assert!(
            slow.distance(start) > 1.0,
            "follow did not move off the start, got {slow:?}"
        );
        assert!(
            slow.distance(want) > 0.5,
            "follow already sat on the target, got {slow:?}"
        );
    }

    /// A closer hit snaps to hit − margin. The floor holds. No hit leaves the distance.
    #[test]
    fn cam_collide_distance_snaps_to_hit_margin_floor_and_miss() {
        let want = 6.0;
        let hit = 3.0;
        assert_eq!(
            pulled_camera_distance(want, Some(hit)),
            hit - CAM_WALL_MARGIN,
            "a closer hit pulls in to hit − margin"
        );
        let inside = CAM_WALL_MARGIN;
        assert!(inside - CAM_WALL_MARGIN < CAM_WALL_FLOOR);
        assert_eq!(
            pulled_camera_distance(want, Some(inside)),
            CAM_WALL_FLOOR,
            "the floor holds when hit − margin would pass it"
        );
        assert_eq!(pulled_camera_distance(want, None), want);
        assert_eq!(pulled_camera_distance(want, Some(want)), want);
        assert_eq!(pulled_camera_distance(want, Some(want + 1.0)), want);
    }

    /// Pull-in snaps. Easing back out uses the CAM-FEEL-PASS-1 weight, so 30 fps matches 144 fps.
    #[test]
    fn cam_collide_pull_snaps_and_ease_out_matches_feel_pass() {
        assert_eq!(
            camera_distance_after_pull(8.0, 2.0, 1.0 / 30.0),
            2.0,
            "pull-in snaps"
        );
        let current = 2.0;
        let resolved = 8.0;
        let once = camera_distance_after_pull(current, resolved, 1.0 / 60.0);
        let expect = current + (resolved - current) * FOLLOW_LERP_AT_60;
        assert!(
            (once - expect).abs() < 1e-5,
            "60 fps ease left the 0.12 weight, got {once} want {expect}"
        );
        let step = |dt: f32, n: u32| {
            let mut distance = current;
            for _ in 0..n {
                distance = camera_distance_after_pull(distance, resolved, dt);
            }
            distance
        };
        let slow = step(1.0 / 30.0, 5);
        let fast = step(1.0 / 144.0, 24);
        assert!(
            (slow - fast).abs() < 1e-3,
            "ease out at 30 fps {slow} left 144 fps {fast}"
        );
        assert!(slow > current + 0.5, "ease out did not move, got {slow}");
        assert!(slow < resolved - 0.5, "ease out already sat on the target, got {slow}");
    }

    /// CARD PT-PICKUP-1 H4 — real order (Update push, FixedUpdate locomotion, no wish): 1 s peak matches at 30/144 fps, under cap.
    #[test]
    fn nectar_push_matches_30_and_144_fps_under_cap() {
        use crate::flow_weather::{ride_nectar, NECTAR_CAP, NECTAR_FROM, NECTAR_TO};
        let peak = |fps: u32| {
            let (mut app, dir) = (App::new(), (NECTAR_TO - NECTAR_FROM).normalize());
            app.add_plugins(MinimalPlugins).init_resource::<PlayerInput>().init_resource::<JumpLatch>().init_resource::<SoftPresence>();
            app.insert_resource(crate::living_practice_loop::SoftPlayerRealm { current: Some(0) }).insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(std::time::Duration::from_secs_f64(1.0 / fps as f64)));
            app.add_systems(Update, ride_nectar).add_systems(FixedUpdate, apply_locomotion).world_mut().resource_mut::<SoftPresence>().position = (NECTAR_FROM + NECTAR_TO) * 0.5;
            let along = |app: &mut App| { app.update(); let v = app.world().resource::<SoftPresence>().velocity; v.x * dir.x + v.z * dir.z };
            ((0..=fps).map(|_| along(&mut app)).fold(0.0f32, f32::max), app.world().resource::<Time<Fixed>>().elapsed_secs())
        };
        let ((slow, t30), (fast, t144)) = (peak(30), peak(144));
        assert!(t30 > 0.9 && t144 > 0.9, "fixed time did not step: {t30} {t144}");
        assert!((slow - fast).abs() < 0.1 && slow > 2.0 && slow.max(fast) <= NECTAR_CAP + 1e-4, "30 fps {slow} vs 144 fps {fast}");
    }

    fn collide_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<SoftPresence>()
            .init_resource::<Assets<Mesh>>()
            .insert_resource(LaunchDoor::InYard)
            .add_systems(Update, pull_camera_from_walls);
        app
    }

    fn pivot_of(app: &App) -> Vec3 {
        let presence = app.world().resource::<SoftPresence>().position;
        presence + Vec3::Y * follow_look_height_for_realm(None)
    }

    /// A thin wall facing the pivot. Near face sits `near_z` along +Z from the pivot.
    fn spawn_wall(app: &mut App, near_z: f32) -> Entity {
        let half_z = 0.1;
        let pivot = pivot_of(app);
        let center = pivot + Vec3::Z * (near_z + half_z);
        let transform = Transform::from_translation(center);
        let mut meshes = app.world_mut().resource_mut::<Assets<Mesh>>();
        let handle = meshes.add(Mesh::from(Cuboid::new(4.0, 4.0, half_z * 2.0)));
        app.world_mut()
            .spawn((
                Mesh3d(handle),
                transform,
                GlobalTransform::from(transform),
                bevy::camera::primitives::Aabb::from_min_max(
                    Vec3::new(-2.0, -2.0, -half_z),
                    Vec3::new(2.0, 2.0, half_z),
                ),
                InheritedVisibility::VISIBLE,
                ViewVisibility::VISIBLE,
            ))
            .id()
    }

    fn spawn_follow_cam(app: &mut App, distance: f32) {
        let pivot = pivot_of(app);
        app.world_mut().spawn((
            Camera3d::default(),
            Transform::from_translation(pivot + Vec3::Z * distance),
        ));
    }

    fn cam_distance_from_pivot(app: &mut App) -> f32 {
        let pivot = pivot_of(app);
        cam_translation(app).distance(pivot)
    }

    /// The system pulls the lens to the wall hit minus the margin.
    #[test]
    fn cam_collide_system_pulls_in_to_hit_margin() {
        let mut app = collide_app();
        let near_z = 3.0;
        spawn_wall(&mut app, near_z);
        spawn_follow_cam(&mut app, 6.0);
        app.update();
        let got = cam_distance_from_pivot(&mut app);
        let want = pulled_camera_distance(6.0, Some(near_z));
        assert!(
            (got - want).abs() < 1e-3,
            "camera distance {got} left hit−margin {want}"
        );
    }

    /// A hit inside the floor stays on the floor instead of the pivot.
    #[test]
    fn cam_collide_system_floor_holds() {
        let mut app = collide_app();
        spawn_wall(&mut app, CAM_WALL_MARGIN * 0.5);
        spawn_follow_cam(&mut app, 6.0);
        app.update();
        let got = cam_distance_from_pivot(&mut app);
        assert!(
            (got - CAM_WALL_FLOOR).abs() < 1e-3,
            "floor did not hold, distance {got}"
        );
    }

    /// No mesh along the ray leaves the follow distance alone.
    #[test]
    fn cam_collide_system_miss_leaves_distance() {
        let mut app = collide_app();
        let distance = 6.0;
        spawn_follow_cam(&mut app, distance);
        app.update();
        let got = cam_distance_from_pivot(&mut app);
        assert!(
            (got - distance).abs() < 1e-4,
            "miss moved the camera to {got}"
        );
    }

    /// The presence body and its children are not walls, even when a mesh sits on the ray.
    #[test]
    fn cam_collide_system_ignores_presence_body() {
        let mut app = collide_app();
        let near_z = 3.0;
        let wall = spawn_wall(&mut app, near_z);
        let root = app.world_mut().spawn(HumanPresence).id();
        app.world_mut().entity_mut(wall).insert(ChildOf(root));
        let distance = 6.0;
        spawn_follow_cam(&mut app, distance);
        app.update();
        let got = cam_distance_from_pivot(&mut app);
        assert!(
            (got - distance).abs() < 1e-3,
            "presence body pulled the camera to {got}"
        );

        let stranger = Entity::PLACEHOLDER;
        let mut body = Vec::new();
        let world = app.world_mut();
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            body.push(node);
            if let Some(kids) = world.get::<Children>(node) {
                stack.extend(kids.iter());
            }
        }
        assert!(body.contains(&root));
        assert!(body.contains(&wall));
        assert!(!follow_ray_keeps_entity(root, &body));
        assert!(!follow_ray_keeps_entity(wall, &body));
        assert!(follow_ray_keeps_entity(stranger, &body));
    }

    /// Title and other doors do not pull, so orbit on the way in stays put.
    #[test]
    fn cam_collide_skips_when_not_in_yard() {
        let mut app = collide_app();
        app.insert_resource(LaunchDoor::Title);
        spawn_wall(&mut app, 3.0);
        let distance = 6.0;
        spawn_follow_cam(&mut app, distance);
        app.update();
        let got = cam_distance_from_pivot(&mut app);
        assert!(
            (got - distance).abs() < 1e-4,
            "door other than InYard pulled the camera to {got}"
        );
    }

    /// CARD PT-COLLIDE-1 F1 — walking into a trunk stops at its radius on x/z; y untouched.
    #[test]
    fn pt_collide_walk_stops_at_trunk_radius() {
        let trunk = (Vec2::new(1.0, 2.0), 0.22);
        let mut pos = Vec3::new(1.0, 0.9, 1.0);
        (0..120).for_each(|_| { pos.z += 0.05; push_out_of_trunks(&mut pos, [trunk]); });
        assert!((pos.xz().distance(trunk.0) - trunk.1 - PLAYER_TRUNK_RADIUS).abs() < 1e-4, "{pos}");
        assert!(pos.z < trunk.0.y && pos.y == 0.9, "walked through or moved y: {pos}");
    }

    fn spawn_tree_part(app: &mut App, mesh: Mesh, at: Vec3, half: Vec3) -> Entity {
        let aabb = bevy::camera::primitives::Aabb::from_min_max(-half, half);
        let handle = app.world_mut().resource_mut::<Assets<Mesh>>().add(mesh);
        let (t, vis) = (Transform::from_translation(at), (InheritedVisibility::VISIBLE, ViewVisibility::VISIBLE));
        app.world_mut().spawn((Mesh3d(handle), t, GlobalTransform::from(t), aabb, vis)).id()
    }

    /// CARD PT-COLLIDE-1 F2 (a) / PT-CAM-TRUNK-1 T-c — pivot pressed on a marked yard trunk, lens
    /// behind it: the trunk fades to its own Blend clone, the distance holds, and moving away restores it.
    #[test]
    fn pt_collide_cam_pivot_beside_trunk() {
        let trunk = crate::living_ecology::YARD_TREE_SPOTS[0];
        let mut app = collide_app();
        app.init_resource::<Assets<StandardMaterial>>().init_resource::<TrunkFadeCache>();
        let mut stand = Vec3::new(trunk.x, STAND, trunk.z - 0.1);
        push_out_of_trunks(&mut stand, [(trunk.xz(), crate::living_ecology::YARD_TRUNK_R)]);
        app.world_mut().resource_mut::<SoftPresence>().position = stand;
        let e = spawn_tree_part(&mut app, Mesh::from(Cylinder::new(0.22, 2.4)), trunk, Vec3::new(0.22, 1.2, 0.22));
        let shared = app.world_mut().resource_mut::<Assets<StandardMaterial>>().add(StandardMaterial::default());
        app.world_mut().entity_mut(e).insert((CamTrunk, MeshMaterial3d(shared.clone())));
        spawn_follow_cam(&mut app, 4.0);
        app.update();
        let now = app.world().get::<MeshMaterial3d<StandardMaterial>>(e).unwrap().0.clone();
        let faded = app.world().resource::<Assets<StandardMaterial>>().get(&now).unwrap();
        assert!(now != shared && matches!(faded.alpha_mode, AlphaMode::Blend) && faded.base_color.alpha() <= 0.4);
        assert!((cam_distance_from_pivot(&mut app) - 4.0).abs() < 1e-3, "lens pulled into the trunk");
        app.world_mut().resource_mut::<SoftPresence>().position.x += 5.0;
        spawn_follow_cam(&mut app, 4.0);
        let mut cams = app.world_mut().query_filtered::<Entity, With<Camera3d>>();
        let old = cams.iter(app.world()).next().unwrap();
        app.world_mut().despawn(old);
        app.update();
        assert_eq!(app.world().get::<MeshMaterial3d<StandardMaterial>>(e).unwrap().0, shared);
    }

    /// CARD PT-CAM-TRUNK-1 A1 — a wall behind a marked trunk still pulls to hit − margin.
    #[test]
    fn pt_cam_trunk_wall_behind_trunk_still_pulls() {
        let mut app = collide_app();
        let at = pivot_of(&app) + Vec3::Z * 1.5;
        let e = spawn_tree_part(&mut app, Mesh::from(Cylinder::new(0.22, 2.4)), at, Vec3::new(0.22, 1.2, 0.22));
        app.world_mut().entity_mut(e).insert(CamTrunk);
        spawn_wall(&mut app, 3.0);
        spawn_follow_cam(&mut app, 6.0);
        app.update();
        let want = pulled_camera_distance(6.0, Some(3.0));
        assert!((cam_distance_from_pivot(&mut app) - want).abs() < 1e-3);
    }

    /// CARD PT-COLLIDE-1 F2 — (b) lens inside a yard canopy cone (r 1.05).
    #[test]
    fn pt_collide_cam_lens_in_canopy() {
        let mut app = collide_app();
        let (canopy, half) = (pivot_of(&app) + Vec3::new(0.0, 0.3, 3.0), Vec3::new(1.05, 1.3, 1.05));
        spawn_tree_part(&mut app, Mesh::from(Cone { radius: 1.05, height: 2.6 }), canopy, half);
        spawn_follow_cam(&mut app, 3.0);
        app.update();
        let got = cam_distance_from_pivot(&mut app);
        assert!(got < 3.0 - 1.05 * 0.5, "lens stayed in the canopy at {got}");
    }

    fn preset_follow_app(inner: LocalSettings) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ControlPreset>()
            .insert_resource(LocalSettingsState {
                inner,
                dirty: false,
            })
            .add_systems(Startup, seed_control_preset_from_settings)
            .add_systems(Update, sync_control_preset_from_settings);
        app
    }

    /// CARD CONTROLS-PRESETS-UI-1 — the resource follows LocalSettings at boot and on change.
    #[test]
    fn control_preset_resource_follows_settings() {
        let legacy = r#"{"schema":"powrush_settings_v1","look_sensitivity":1.0,"mute":false,"invert_y":false,"hide_slabs":false}"#;
        let old = LocalSettings::from_json(legacy).unwrap();
        assert_eq!(old.control_preset, "modern");
        let mut boot_old = preset_follow_app(old);
        boot_old.update();
        assert_eq!(
            *boot_old.world().resource::<ControlPreset>(),
            ControlPreset::Modern
        );

        let mut classic = LocalSettings::peace_defaults();
        classic.cycle_control_preset();
        let mut app = preset_follow_app(classic);
        app.update();
        assert_eq!(*app.world().resource::<ControlPreset>(), ControlPreset::Classic);
        app.world_mut()
            .resource_mut::<LocalSettingsState>()
            .inner
            .cycle_control_preset();
        app.update();
        assert_eq!(*app.world().resource::<ControlPreset>(), ControlPreset::Modern);
        app.world_mut()
            .resource_mut::<LocalSettingsState>()
            .inner
            .control_preset = "classic".into();
        app.update();
        assert_eq!(*app.world().resource::<ControlPreset>(), ControlPreset::Classic);
    }
}
