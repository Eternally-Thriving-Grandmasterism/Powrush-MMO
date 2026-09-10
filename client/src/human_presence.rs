/*!
 * Human Presence — v22.11.0
 *
 * Ground, jump, mount, breath, carry.
 * Feel-move: FixedUpdate @ 60 Hz, accel toward wish, stop-on-release (no ice-skate).
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::companion_bond::CompanionBond;
use crate::input::PlayerInput;
use crate::harvest_feel::SoftRbePool;
use crate::living_body::LivingBody;
use crate::local_settings::LocalFeedbackFeel;

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

#[derive(Component)]
pub struct HumanPresence;

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
            .add_systems(Startup, spawn_human_presence)
            .add_systems(Update, (arm_jump_latch, sync_body, follow_camera).chain())
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
    let capsule = meshes.add(Capsule3d::new(0.28, 1.12));
    let mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.20, 0.26, 0.24),
        perceptual_roughness: 0.74,
        ..default()
    });
    commands.spawn((
        PbrBundle {
            mesh: capsule,
            material: mat,
            transform: Transform::from_xyz(0.0, STAND, 0.0),
            ..default()
        },
        HumanPresence,
        Name::new("HumanPresence"),
    ));
    info!(target: "powrush::presence", "human-scale body on the climate plane");
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

fn sync_body(
    presence: Res<SoftPresence>,
    mut q: Query<&mut Transform, With<HumanPresence>>,
) {
    for mut tf in &mut q {
        tf.translation = presence.position;
        let horiz = Vec3::new(presence.velocity.x, 0.0, presence.velocity.z);
        if horiz.length_squared() > 0.04 {
            let dir = horiz.normalize();
            tf.rotation = Quat::from_rotation_y(dir.x.atan2(dir.z));
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
    fn release_decel_faster_than_ice_skate() {
        // One tick of zero wish must cut speed; ice-skate would keep full velocity.
        let dt = 1.0 / 60.0;
        let before = Vec2::new(WALK, 0.0);
        let after = approach_horizontal(before, Vec2::ZERO, MOVE_ACCEL, MOVE_DECEL, dt);
        assert!(after.length() < before.length());
        assert!(after.x > 0.0); // not teleport-stop in one tick, but braking
    }
}
