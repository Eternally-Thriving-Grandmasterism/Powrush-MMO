/*!
 * Living Ecology — v22.7.0
 *
 * Heartwood · Crystal Spires · Abyssal Depths (content/biomes/*).
 * No ocean simulation — night, close air, glowing threads.
 *
 * PATSAGi v22.4 | Contact: info@Rathor.ai | Yoi ⚡
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
    Crystal,
    Mycelium,
}

#[derive(Component)]
struct ResonantDeer;

#[derive(Component)]
struct CrystalGlow {
    handle: Handle<StandardMaterial>,
}

#[derive(Component)]
struct MyceliumGlow {
    handle: Handle<StandardMaterial>,
}

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

#[derive(Resource, Debug)]
pub struct BiomeFeel {
    pub regen_mul: f32,
    pub name: &'static str,
}

impl Default for BiomeFeel {
    fn default() -> Self {
        Self {
            regen_mul: 1.0,
            name: "Sanctuary",
        }
    }
}

pub struct LivingEcologyPlugin;

impl Plugin for LivingEcologyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EcologyState>()
            .init_resource::<BiomeFeel>()
            .add_systems(Startup, spawn_ecology)
            .add_systems(
                Update,
                (
                    remember_care,
                    dress_for_climate,
                    move_deer,
                    sing_or_silence_spires,
                    pulse_mycelium,
                ),
            );
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

    let spire = meshes.add(Cylinder::new(0.16, 3.6));
    let crystal_spots = [
        Vec3::new(5.0, 1.8, -6.2),
        Vec3::new(-5.4, 1.8, -6.8),
        Vec3::new(7.4, 1.8, 1.6),
        Vec3::new(-7.2, 1.8, 2.2),
    ];
    for pos in crystal_spots {
        let handle = materials.add(StandardMaterial {
            base_color: Color::srgb(0.55, 0.78, 0.95),
            emissive: LinearRgba::new(0.25, 0.45, 0.70, 1.0),
            perceptual_roughness: 0.18,
            metallic: 0.12,
            ..default()
        });
        commands.spawn((
            PbrBundle {
                mesh: spire.clone(),
                material: handle.clone(),
                transform: Transform::from_translation(pos),
                ..default()
            },
            EcologyProp {
                kind: PropKind::Crystal,
            },
            CrystalGlow { handle },
        ));
    }

    // Threads between the three harvest anchors — remember care, no water lie.
    let thread_mesh = meshes.add(Cylinder::new(0.035, 1.0));
    let anchors = [
        Vec3::new(3.6, 0.08, 0.0),
        Vec3::new(-2.4, 0.08, 3.1),
        Vec3::new(1.2, 0.08, -3.4),
    ];
    let pairs = [(0, 1), (1, 2), (2, 0)];
    for (a, b) in pairs {
        let from = anchors[a];
        let to = anchors[b];
        let mid = (from + to) * 0.5;
        let delta = to - from;
        let len = delta.length().max(0.2);
        let dir = delta.normalize();
        let rot = Quat::from_rotation_arc(Vec3::Y, dir);
        let handle = materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.55, 0.42),
            emissive: LinearRgba::new(0.08, 0.35, 0.28, 1.0),
            perceptual_roughness: 0.4,
            ..default()
        });
        commands.spawn((
            PbrBundle {
                mesh: thread_mesh.clone(),
                material: handle.clone(),
                transform: Transform {
                    translation: mid,
                    rotation: rot,
                    scale: Vec3::new(1.0, len, 1.0),
                },
                ..default()
            },
            EcologyProp {
                kind: PropKind::Mycelium,
            },
            MyceliumGlow { handle },
        ));
    }

    info!(target: "powrush::ecology", "Heartwood + Spires + Abyssal threads seeded");
}

fn remember_care(answer: Res<WorldAnswer>, mut eco: ResMut<EcologyState>) {
    if answer.kind != AnswerKind::Idle {
        eco.last_kind = answer.kind;
    }
}

fn dress_for_climate(
    realm: Res<SoftPlayerRealm>,
    mut feel: ResMut<BiomeFeel>,
    mut q: Query<(&EcologyProp, &mut Visibility)>,
) {
    let id = realm.current.unwrap_or(0);
    feel.name = match id {
        2 => "Verdant Heartwood",
        4 | 1 => "Crystal Spires",
        3 => "Abyssal Depths",
        _ => "Sanctuary",
    };
    feel.regen_mul = match id {
        4 | 1 => 1.6,
        3 => 1.9,
        2 => 1.0,
        _ => 1.0,
    };
    for (prop, mut vis) in &mut q {
        let show = match prop.kind {
            PropKind::Tree => matches!(id, 0 | 2),
            PropKind::Stone => matches!(id, 1 | 4),
            PropKind::Deer => matches!(id, 0 | 2),
            PropKind::Crystal => matches!(id, 1 | 4),
            PropKind::Mycelium => id == 3,
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

fn sing_or_silence_spires(
    eco: Res<EcologyState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q: Query<&CrystalGlow>,
) {
    let peak = matches!(eco.last_kind, AnswerKind::Tend | AnswerKind::Flow);
    let silent = matches!(eco.last_kind, AnswerKind::Take);
    let e = if peak {
        LinearRgba::new(0.55, 0.85, 1.2, 1.0)
    } else if silent {
        LinearRgba::new(0.08, 0.12, 0.22, 1.0)
    } else {
        LinearRgba::new(0.25, 0.45, 0.70, 1.0)
    };
    for glow in &q {
        if let Some(mat) = materials.get_mut(&glow.handle) {
            mat.emissive = e;
        }
    }
}

fn pulse_mycelium(
    eco: Res<EcologyState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q: Query<&MyceliumGlow>,
) {
    let surge = matches!(eco.last_kind, AnswerKind::Tend | AnswerKind::Flow);
    let night = matches!(eco.last_kind, AnswerKind::Take);
    let e = if surge {
        LinearRgba::new(0.25, 1.1, 0.70, 1.0)
    } else if night {
        LinearRgba::new(0.02, 0.08, 0.07, 1.0)
    } else {
        LinearRgba::new(0.08, 0.35, 0.28, 1.0)
    };
    for glow in &q {
        if let Some(mat) = materials.get_mut(&glow.handle) {
            mat.emissive = e;
        }
    }
}
