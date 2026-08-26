/*!
 * Human Presence — v22.4.0
 *
 * A body on the ground. Eye-height, shadow, camera that follows.
 * Believable temporary reality — not a floating HUD.
 *
 * PATSAGi v22.4 | Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::input::Player;
use crate::prediction::{PredictedAbility, PredictedPosition};

const EYE: f32 = 0.92;
const CAM_BACK: f32 = 6.4;
const CAM_UP: f32 = 2.35;

#[derive(Component)]
struct HumanBody;

pub struct HumanPresencePlugin;

impl Plugin for HumanPresencePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_human_presence)
            .add_systems(Update, (sync_body, follow_camera));
    }
}

fn spawn_human_presence(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    existing: Query<Entity, With<Player>>,
) {
    if existing.iter().next().is_some() {
        return;
    }
    let capsule = meshes.add(Capsule3d::new(0.28, 1.12));
    let mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.22, 0.28, 0.26),
        perceptual_roughness: 0.72,
        ..default()
    });
    commands.spawn((
        PbrBundle {
            mesh: capsule,
            material: mat,
            transform: Transform::from_xyz(0.0, EYE, 0.0),
            ..default()
        },
        PredictedPosition {
            position: Vec3::new(0.0, EYE, 0.0),
            velocity: Vec3::ZERO,
            ..default()
        },
        PredictedAbility::default(),
        Player,
        HumanBody,
        Name::new("HumanPresence"),
    ));
    info!(target: "powrush::presence", "human-scale body on the climate plane");
}

fn sync_body(
    mut q: Query<(&PredictedPosition, &mut Transform), With<HumanBody>>,
) {
    for (pred, mut tf) in &mut q {
        tf.translation = pred.position;
        let horiz = Vec3::new(pred.velocity.x, 0.0, pred.velocity.z);
        if horiz.length_squared() > 0.04 {
            let dir = horiz.normalize();
            tf.rotation = Quat::from_rotation_y(dir.x.atan2(dir.z));
        }
    }
}

fn follow_camera(
    bodies: Query<&Transform, (With<HumanBody>, Without<Camera3d>)>,
    mut cams: Query<&mut Transform, With<Camera3d>>,
) {
    let Ok(body) = bodies.get_single() else {
        return;
    };
    for mut cam in &mut cams {
        let desired = body.translation + Vec3::new(0.0, CAM_UP, CAM_BACK);
        cam.translation = cam.translation.lerp(desired, 0.12);
        let look = body.translation + Vec3::Y * 0.45;
        cam.look_at(look, Vec3::Y);
    }
}
