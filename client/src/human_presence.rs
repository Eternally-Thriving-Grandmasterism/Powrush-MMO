/*!
 * Human Presence — v22.9.0
 *
 * Grounded body. Mount is a gift of trust, not a vehicle stat.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::companion_bond::CompanionBond;
use crate::input::PlayerInput;

const STAND: f32 = 0.90;
const WALK: f32 = 3.4;
const SPRINT: f32 = 5.4;
const GRAVITY: f32 = 18.0;
const JUMP: f32 = 5.6;
const CAM_BACK: f32 = 6.4;
const CAM_UP: f32 = 2.35;

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

pub struct HumanPresencePlugin;

impl Plugin for HumanPresencePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SoftPresence>()
            .add_systems(Startup, spawn_human_presence)
            .add_systems(Update, (apply_locomotion, sync_body, follow_camera).chain());
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

fn apply_locomotion(
    input: Res<PlayerInput>,
    time: Res<Time>,
    bond: Option<Res<CompanionBond>>,
    mut presence: ResMut<SoftPresence>,
) {
    let dt = time.delta_seconds();
    let mut speed = if input.sprint { SPRINT } else { WALK };
    if bond.map(|b| b.mounted).unwrap_or(false) {
        speed *= 1.28;
    }
    let wish = Vec3::new(input.movement.x, 0.0, -input.movement.y);
    let wish = if wish.length_squared() > 1.0 {
        wish.normalize()
    } else {
        wish
    };
    presence.velocity.x = wish.x * speed;
    presence.velocity.z = wish.z * speed;
    presence.velocity.y -= GRAVITY * dt;
    if presence.grounded && input.jump {
        presence.velocity.y = JUMP;
        presence.grounded = false;
    }
    presence.position += presence.velocity * dt;
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
    mut cams: Query<&mut Transform, With<Camera3d>>,
) {
    let desired = presence.position + Vec3::new(0.0, CAM_UP, CAM_BACK);
    let look = presence.position + Vec3::Y * 0.45;
    for mut cam in &mut cams {
        cam.translation = cam.translation.lerp(desired, 0.12);
        cam.look_at(look, Vec3::Y);
    }
}
