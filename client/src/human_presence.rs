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
 * settings are open (same gate as STILL-BEFORE-YARD-1). Invert-Y, sensitivity,
 * and camera collision stay deferred.
 *
 * CARD MOVE-STRAFE-DEFAULT-1 — Modern (default) rotates WASD, pad, and touch
 * by the follow-orbit yaw. W goes along the camera's forward, A/D strafe,
 * and the body faces that forward while moving. Classic keeps W/S on the
 * body's facing and turns with keyboard A/D; the orbit yaw follows the body.
 * Pad and touch stay camera-relative in both presets. The preset is a
 * resource and is not persisted (UI is CONTROLS-PRESETS-UI-1).
 *
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use std::f32::consts::{FRAC_PI_2, PI};

use bevy::input::mouse::{
    AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollPixelsPerLine, MouseScrollUnit,
};
use bevy::prelude::*;

use crate::companion_bond::CompanionBond;
use crate::hour_sacred::PeopleLanding;
use crate::input::PlayerInput;
use crate::title_screen::{HouseLabel, LaunchDoor};
use crate::harvest_feel::SoftRbePool;
use crate::living_body::{BodyTell, LivingBody};
use crate::living_day::{
    greet_by_place_mood, preferred_work, schedule_copy_is_honest, still_frame_line, DayPeriod,
    PreferredWork, SacredVerb, SchedulePlace, WellMood,
};
use crate::local_settings::{LocalFeedbackFeel, LocalMeshLodFeel};
use shared::local_settings::GraphicsPreset;

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
const CAM_BACK: f32 = 6.4;
const CAM_UP: f32 = 2.35;
/// Radians of yaw per mouse pixel while the right button is held.
const ORBIT_YAW_PER_PX: f32 = 0.004;
/// Radians of pitch per mouse pixel while the right button is held.
const ORBIT_PITCH_PER_PX: f32 = 0.003;
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
}

impl Default for SoftPresence {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, STAND, 0.0),
            velocity: Vec3::ZERO,
            grounded: true,
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
            .init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<AccumulatedMouseMotion>()
            .init_resource::<AccumulatedMouseScroll>()
            .add_systems(Startup, spawn_human_presence)
            .add_systems(
                Update,
                (
                    apply_presence_mesh_lod,
                    arm_jump_latch,
                    latch_use_attend,
                    sync_body,
                    pose_person,
                    follow_camera,
                )
                    .chain(),
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
    let earth = materials.add(StandardMaterial {
        base_color: BODY_EARTH,
        perceptual_roughness: 0.74,
        ..default()
    });
    let accent = materials.add(StandardMaterial {
        base_color: SANCTUARY_GOLD,
        emissive: LinearRgba::from(SANCTUARY_GOLD) * 0.25,
        perceptual_roughness: 0.42,
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

fn arm_jump_latch(input: Res<PlayerInput>, mut latch: ResMut<JumpLatch>) {
    if input.jump {
        latch.pending = true;
    }
}

/// CARD MOVE-STRAFE-DEFAULT-1 — not persisted. Settings UI is a later card.
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

/// WASD and arrows, matching `input.rs` (out of bounds for this card).
fn keyboard_axes(keyboard: &ButtonInput<KeyCode>) -> Vec2 {
    let mut axes = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        axes.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        axes.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        axes.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
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
    mut latch: ResMut<JumpLatch>,
    mut presence: ResMut<SoftPresence>,
) {
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
        let keys = keyboard.as_deref().map(keyboard_axes).unwrap_or(Vec2::ZERO);
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
    if yard_step && presence.grounded && latch.pending {
        presence.velocity.y = JUMP;
        presence.grounded = false;
        latch.pending = false;
    } else if !presence.grounded {
        // Airborne: drop a stale latch so landing does not auto-hop.
        latch.pending = false;
    }
    let velocity = presence.velocity;
    presence.position += velocity * dt;
    if presence.position.y <= STAND {
        presence.position.y = STAND;
        presence.velocity.y = 0.0;
        presence.grounded = true;
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
    mut q: Query<&mut Transform, With<HumanPresence>>,
) {
    let preset = preset.as_deref().copied().unwrap_or_default();
    for mut tf in &mut q {
        tf.translation = presence.position;
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

fn apply_orbit_input(
    orbit: &mut FollowOrbit,
    buttons: &ButtonInput<MouseButton>,
    motion: &AccumulatedMouseMotion,
    scroll: &AccumulatedMouseScroll,
    pixels_per_line: f32,
) {
    if buttons.pressed(MouseButton::Right) {
        if motion.delta.x.is_finite() {
            // Mouse right turns the view right: the camera swings toward -X.
            orbit.yaw -= motion.delta.x * ORBIT_YAW_PER_PX;
        }
        if motion.delta.y.is_finite() {
            // Mouse down (positive y) raises the camera and looks down at the player.
            orbit.pitch += motion.delta.y * ORBIT_PITCH_PER_PX;
        }
    }
    let lines = scroll_lines(scroll, pixels_per_line);
    if lines.is_finite() && lines != 0.0 {
        orbit.distance -= lines * ORBIT_DIST_PER_LINE;
    }
    orbit.pitch = orbit.pitch.clamp(CAM_PITCH_MIN, CAM_PITCH_MAX);
    orbit.distance = orbit.distance.clamp(CAM_DIST_MIN, CAM_DIST_MAX);
}

fn follow_camera(
    presence: Res<SoftPresence>,
    pool: Option<Res<SoftRbePool>>,
    feedback: Res<LocalFeedbackFeel>,
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
) {
    // CARD CAM-MOUSE-ORBIT-1 — same yard gate as STILL-BEFORE-YARD-1.
    if orbit_input_open(door.as_deref(), house.as_deref()) {
        let pixels_per_line = scroll_px.map(|px| **px).unwrap_or(ORBIT_PIXELS_PER_LINE);
        apply_orbit_input(
            &mut orbit,
            &mouse_buttons,
            &mouse_motion,
            &mouse_scroll,
            pixels_per_line,
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
    let look_down = presence_reads_ambrosian_lift(presence.position);
    let desired = if look_down {
        presence.position + Vec3::new(0.0, CAM_UP + 1.1, CAM_BACK - 1.4)
    } else {
        presence.position + orbit.offset() + punch_delta
    };
    let look = if look_down {
        Vec3::new(presence.position.x, 0.45, presence.position.z)
    } else {
        presence.position + Vec3::Y * (follow_look_height_for_realm(realm.and_then(|r| r.current)) + look_lift)
    };
    for mut cam in &mut cams {
        cam.translation = cam.translation.lerp(desired, 0.12);
        cam.look_at(look, Vec3::Y);
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
    use super::*;

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
            "let look = if look_down {\n        Vec3::new(presence.position.x, 0.45, presence.position.z)\n    } else {"
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
            .insert_resource(LocalFeedbackFeel::default())
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

    /// Gamepad and WASD stay on PlayerInput. They do not orbit the camera.
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
}
