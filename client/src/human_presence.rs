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
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use std::f32::consts::PI;

use bevy::prelude::*;

use crate::companion_bond::CompanionBond;
use crate::input::PlayerInput;
use crate::harvest_feel::SoftRbePool;
use crate::living_body::{BodyTell, LivingBody};
use crate::living_day::{
    greet_by_place_mood, preferred_work, schedule_copy_is_honest, still_frame_line, DayPeriod,
    PreferredWork, SacredVerb, SchedulePlace, WellMood,
};
use crate::local_settings::LocalFeedbackFeel;
use crate::mercy_harvest_nodes::{MercyHarvestNode, HARVEST_REACH};
use crate::soft_play_bindings;

const STAND: f32 = 0.90;
const WALK: f32 = 3.4;
const SPRINT: f32 = 5.4;
const GRAVITY: f32 = 18.0;
const JUMP: f32 = 5.6;
const CAM_BACK: f32 = 6.4;
const CAM_UP: f32 = 2.35;
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
            .add_systems(Startup, spawn_human_presence)
            .add_systems(
                Update,
                (
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
) {
    if existing.iter().next().is_some() {
        return;
    }
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
            PbrBundle {
                mesh: hips,
                material: earth.clone(),
                transform: Transform::from_xyz(0.0, STAND, 0.0),
                ..default()
            },
            HumanPresence,
            Name::new("HumanPresence"),
        ))
        .with_children(|root| {
            root.spawn((
                SpatialBundle {
                    transform: Transform::from_xyz(0.0, WAIST_Y, 0.0),
                    ..default()
                },
                PersonPart::Waist,
                Name::new("PersonWaist"),
            ))
            .with_children(|waist| {
                waist.spawn(PbrBundle {
                    mesh: torso,
                    material: earth.clone(),
                    transform: Transform::from_xyz(0.0, TORSO_Y, 0.0),
                    ..default()
                });
                waist
                    .spawn((
                        PbrBundle {
                            mesh: head,
                            material: earth.clone(),
                            transform: Transform::from_xyz(0.0, HEAD_Y, 0.0),
                            ..default()
                        },
                        PersonPart::Head,
                        Name::new("PersonHead"),
                    ))
                    .with_children(|face| {
                        // Brow nub: which way the person looks reads in a still.
                        face.spawn(PbrBundle {
                            mesh: brow,
                            material: earth.clone(),
                            transform: Transform::from_xyz(0.0, 0.01, HEAD_R * 0.86),
                            ..default()
                        });
                    });
                for side in [-1.0f32, 1.0] {
                    waist
                        .spawn((
                            SpatialBundle {
                                transform: Transform::from_xyz(
                                    side * SHOULDER_X,
                                    SHOULDER_Y,
                                    0.0,
                                ),
                                ..default()
                            },
                            PersonPart::Shoulder(side),
                            Name::new("PersonShoulder"),
                        ))
                        .with_children(|limb| {
                            limb.spawn(PbrBundle {
                                mesh: arm.clone(),
                                material: earth.clone(),
                                transform: Transform::from_xyz(0.0, ARM_Y, 0.0),
                                ..default()
                            });
                            if side > 0.0 {
                                limb.spawn((
                                    PbrBundle {
                                        mesh: hand.clone(),
                                        material: accent.clone(),
                                        transform: Transform::from_xyz(0.0, HAND_Y, 0.0),
                                        ..default()
                                    },
                                    PersonAccent,
                                    Name::new("PersonAccent"),
                                ));
                            }
                        });
                }
            });
            for side in [-1.0f32, 1.0] {
                root.spawn((
                    SpatialBundle {
                        transform: Transform::from_xyz(side * HIP_PIVOT_X, HIP_PIVOT_Y, 0.0),
                        ..default()
                    },
                    PersonPart::Hip(side),
                    Name::new("PersonHip"),
                ))
                .with_children(|limb| {
                    limb.spawn(PbrBundle {
                        mesh: leg.clone(),
                        material: earth.clone(),
                        transform: Transform::from_xyz(0.0, LEG_Y, 0.0),
                        ..default()
                    });
                });
            }
        });
    info!(
        target: "powrush::presence",
        "standing person on the climate plane — head, torso, stance mass"
    );
}

fn arm_jump_latch(input: Res<PlayerInput>, mut latch: ResMut<JumpLatch>) {
    if input.jump {
        latch.pending = true;
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
    mut latch: ResMut<JumpLatch>,
    mut presence: ResMut<SoftPresence>,
) {
    let dt = time.delta_seconds();
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
    let wish = Vec3::new(input.movement.x, 0.0, -input.movement.y);
    let wish = if wish.length_squared() > 1.0 {
        wish.normalize()
    } else {
        wish
    };
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
    presence.velocity.y -= GRAVITY * dt;
    if presence.grounded && latch.pending {
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
    attend.level = attend_level(attend.level, input.interact, held, time.delta_seconds());
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
    mut q: Query<&mut Transform, With<HumanPresence>>,
) {
    for mut tf in &mut q {
        tf.translation = presence.position;
        let horiz = Vec3::new(presence.velocity.x, 0.0, presence.velocity.z);
        if horiz.length_squared() > 0.04 {
            let dir = horiz.normalize();
            tf.rotation = Quat::from_rotation_y(dir.x.atan2(dir.z));
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
    attend: Res<PersonAttend>,
    mut rhythm: ResMut<PersonRhythm>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut parts: Query<(&PersonPart, &mut Transform)>,
    accents: Query<&Handle<StandardMaterial>, With<PersonAccent>>,
) {
    let dt = time.delta_seconds();
    let speed = Vec2::new(presence.velocity.x, presence.velocity.z).length();
    let tell = body.as_ref().map(|b| b.tell()).unwrap_or_default();
    let breath_rise = body.as_ref().map(|b| b.breath_rise()).unwrap_or(0.012);
    let breath_rate = body.as_ref().map(|b| b.breath_rate()).unwrap_or(0.95);
    let kick = pool.map(|p| p.kick).unwrap_or(0.0);

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
        if let Some(mat) = materials.get_mut(handle) {
            mat.emissive = LinearRgba::from(SANCTUARY_GOLD) * glow;
        }
    }
}

fn follow_camera(
    presence: Res<SoftPresence>,
    pool: Option<Res<SoftRbePool>>,
    feedback: Res<LocalFeedbackFeel>,
    mut cams: Query<&mut Transform, With<Camera3d>>,
) {
    let punch = pool.map(|p| p.kick).unwrap_or(0.0) * feedback.camera_punch_scale;
    let desired = presence.position + Vec3::new(0.0, CAM_UP + punch * 0.22, CAM_BACK - punch * 0.35);
    let look = presence.position + Vec3::Y * (0.45 + punch * 0.08);
    for mut cam in &mut cams {
        cam.translation = cam.translation.lerp(desired, 0.12);
        cam.look_at(look, Vec3::Y);
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
}
