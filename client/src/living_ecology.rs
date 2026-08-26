/*!
 * Living Ecology — v22.5.0
 *
 * Seeded from content/biomes/verdant_heartwood_ecology.json.
 * Ancient trees occupy space. A resonant companion approaches care
 * and withdraws from take — the same lesson as base-reality wildlife.
 * No faction mannequins. No fake player counts.
 *
 * PATSAGi v22.4 law | Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::living_practice_loop::SoftPlayerRealm;
use crate::world_answer::{AnswerKind, WorldAnswer};

const DEER_NEAR: Vec3 = Vec3::new(1.8, 0.55, 1.4);
const DEER_FAR: Vec3 = Vec3::new(8.5, 0.55, 6.2);

#[derive(Component)]
struct EcologyProp {
    kind: PropKind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PropKind {
    Tree,
    Stone,
    Deer,
}

#[derive(Component)]
struct ResonantDeer;

#[derive(Resource, Debug)]
struct EcologyState {
    last_kind: AnswerKind,
}

impl Default for EcologyState {
    fn default() -> Self {
        Self {
            last_kind: AnswerKind::Idle,
        }
    }
}

pub struct LivingEcologyPlugin;

impl Plugin for LivingEcologyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EcologyState>()
            .add_systems(Startup, spawn_ecology)
            .add_systems(Update, (remember_care, dress_for_climate, move_deer));
    }
}

fn spawn_ecology(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let trunk = meshes.add(Cylinder::new(0.22, 2.4));
    let canopy = meshes.add(Sphere::new(0.85));
    let wood = materials.add(StandardMaterial {
        base_color: Color::srgb(0.28, 0.18, 0.10),
        perceptual_roughness: 0.9,
        ..default()
    });
    let leaf = materials.add(StandardMaterial {
        base_color: Color::srgb(0.16, 0.42, 0.20),
        perceptual_roughness: 0.7,
        ..default()
    });
    let tree_spots = [
        Vec3::new(-5.2, 1.2, -2.4),
        Vec3::new(-6.4, 1.2, 2.8),
        Vec3::new(5.8, 1.2, -4.1),
        Vec3::new(6.6, 1.2, 3.2),
        Vec3::new(-3.8, 1.2, 6.0),
    ];
    for pos in tree_spots {
        commands.spawn((
            PbrBundle {
                mesh: trunk.clone(),
                material: wood.clone(),
                transform: Transform::from_translation(pos),
                ..default()
            },
            EcologyProp { kind: PropKind::Tree },
        ));
        commands.spawn((
            PbrBundle {
                mesh: canopy.clone(),
                material: leaf.clone(),
                transform: Transform::from_translation(pos + Vec3::Y * 1.35),
                ..default()
            },
            EcologyProp { kind: PropKind::Tree },
        ));
    }

    let rock = meshes.add(Sphere::new(0.42));
    let stone = materials.add(StandardMaterial {
        base_color: Color::srgb(0.32, 0.30, 0.28),
        perceptual_roughness: 0.95,
        ..default()
    });
    for pos in [
        Vec3::new(4.2, 0.22, -5.5),
        Vec3::new(-4.8, 0.22, -5.0),
        Vec3::new(7.0, 0.22, 0.4),
    ] {
        commands.spawn((
            PbrBundle {
                mesh: rock.clone(),
                material: stone.clone(),
                transform: Transform::from_translation(pos).with_scale(Vec3::new(1.4, 0.6, 1.1)),
                ..default()
            },
            EcologyProp {
                kind: PropKind::Stone,
            },
        ));
    }

    let deer_mesh = meshes.add(Capsule3d::new(0.16, 0.55));
    let deer_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.42, 0.30, 0.18),
        perceptual_roughness: 0.65,
        ..default()
    });
    commands.spawn((
        PbrBundle {
            mesh: deer_mesh,
            material: deer_mat,
            transform: Transform::from_translation(DEER_FAR),
            ..default()
        },
        EcologyProp { kind: PropKind::Deer },
        ResonantDeer,
        Name::new("ResonantDeer"),
    ));

    info!(target: "powrush::ecology", "Heartwood trees + resonant companion seeded");
}

fn remember_care(answer: Res<WorldAnswer>, mut eco: ResMut<EcologyState>) {
    if answer.kind != AnswerKind::Idle {
        eco.last_kind = answer.kind;
    }
}

fn dress_for_climate(
    realm: Res<SoftPlayerRealm>,
    mut q: Query<(&EcologyProp, &mut Visibility)>,
) {
    let id = realm.current.unwrap_or(0);
    for (prop, mut vis) in &mut q {
        let show = match prop.kind {
            PropKind::Tree => matches!(id, 0 | 2 | 3),
            PropKind::Stone => matches!(id, 1 | 4),
            PropKind::Deer => matches!(id, 0 | 2),
        };
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn move_deer(
    eco: Res<EcologyState>,
    time: Res<Time>,
    mut q: Query<&mut Transform, With<ResonantDeer>>,
) {
    let target = match eco.last_kind {
        AnswerKind::Tend | AnswerKind::Flow => DEER_NEAR,
        AnswerKind::Take => DEER_FAR,
        _ => Vec3::new(5.2, 0.55, 3.6),
    };
    let dt = time.delta_seconds();
    for mut tf in &mut q {
        tf.translation = tf.translation.lerp(target, (1.6 * dt).min(1.0));
    }
}
