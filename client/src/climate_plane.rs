/*!
 * Climate Plane — Arc A (v22.0.0)
 *
 * Z travel moves the *place*: ground tint, sky, fog, stepping-stone path.
 * First hour defaults to Sanctuary. No server required.
 *
 * PATSAGi v22 | Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::pbr::{FogFalloff, FogSettings};
use bevy::prelude::*;

use crate::living_practice_loop::SoftPlayerRealm;
use crate::mercy_harvest_nodes::MercyHarvestNode;

const NODE_ANCHORS: [Vec3; 3] = [
    Vec3::new(3.6, 0.55, 0.0),
    Vec3::new(-2.4, 0.55, 3.1),
    Vec3::new(1.2, 0.55, -3.4),
];

#[derive(Clone, Copy)]
struct ClimateLook {
    name: &'static str,
    ground: Color,
    sky: Color,
    fog: Color,
    ambient: Color,
    node: Color,
    stone: Color,
}

fn look_for(realm: Option<u8>) -> ClimateLook {
    match realm {
        Some(2) => ClimateLook {
            name: "Verdant Bloom",
            ground: Color::srgb(0.12, 0.28, 0.16),
            sky: Color::srgb(0.42, 0.72, 0.58),
            fog: Color::srgba(0.35, 0.62, 0.48, 1.0),
            ambient: Color::srgb(0.55, 0.85, 0.62),
            node: Color::srgb(0.40, 0.95, 0.62),
            stone: Color::srgb(0.22, 0.38, 0.24),
        },
        Some(4) | Some(1) => ClimateLook {
            name: if realm == Some(1) {
                "Synthetic Lattice"
            } else {
                "Voidfarer Horizon"
            },
            ground: Color::srgb(0.10, 0.09, 0.14),
            sky: Color::srgb(0.14, 0.16, 0.28),
            fog: Color::srgba(0.12, 0.14, 0.24, 1.0),
            ambient: Color::srgb(0.40, 0.48, 0.70),
            node: Color::srgb(0.95, 0.82, 0.38),
            stone: Color::srgb(0.22, 0.20, 0.28),
        },
        Some(3) => ClimateLook {
            name: "Harmonic Chorus",
            ground: Color::srgb(0.16, 0.14, 0.22),
            sky: Color::srgb(0.48, 0.42, 0.68),
            fog: Color::srgba(0.40, 0.36, 0.58, 1.0),
            ambient: Color::srgb(0.62, 0.55, 0.82),
            node: Color::srgb(0.72, 0.62, 0.98),
            stone: Color::srgb(0.28, 0.24, 0.36),
        },
        _ => ClimateLook {
            name: "Sanctuary Prime",
            ground: Color::srgb(0.18, 0.22, 0.16),
            sky: Color::srgb(0.62, 0.78, 0.72),
            fog: Color::srgba(0.55, 0.72, 0.68, 1.0),
            ambient: Color::srgb(0.70, 0.82, 0.74),
            node: Color::srgb(0.35, 0.92, 0.62),
            stone: Color::srgb(0.28, 0.30, 0.24),
        },
    }
}

#[derive(Resource, Debug)]
pub struct ClimatePlane {
    pub applied: Option<u8>,
}

impl Default for ClimatePlane {
    fn default() -> Self {
        Self { applied: None }
    }
}

#[derive(Component)]
struct ClimateGround;
#[derive(Component)]
struct ClimateStone;
#[derive(Component)]
struct ClimateNameRoot;
#[derive(Component)]
struct ClimateNameText;

pub struct ClimatePlanePlugin;

impl Plugin for ClimatePlanePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClimatePlane>()
            .insert_resource(ClearColor(look_for(Some(0)).sky))
            .insert_resource(AmbientLight {
                color: look_for(Some(0)).ambient,
                brightness: 280.0,
            })
            .add_systems(Startup, (ensure_sanctuary, spawn_climate_place, spawn_climate_chip))
            .add_systems(Update, (apply_climate_look, update_climate_chip));
    }
}

fn ensure_sanctuary(mut realm: ResMut<SoftPlayerRealm>) {
    if realm.current.is_none() {
        realm.current = Some(0);
    }
}

fn spawn_climate_place(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cameras: Query<Entity, With<Camera3d>>,
    lights: Query<Entity, With<DirectionalLight>>,
) {
    let look = look_for(Some(0));
    let ground = meshes.add(Plane3d::default().mesh().size(56.0, 56.0));
    commands.spawn((
        PbrBundle {
            mesh: ground,
            material: materials.add(StandardMaterial {
                base_color: look.ground,
                perceptual_roughness: 0.92,
                metallic: 0.0,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        ClimateGround,
        Name::new("ClimateGround"),
    ));

    let stone_mesh = meshes.add(Cylinder::new(0.18, 0.08));
    let stone_mat = materials.add(StandardMaterial {
        base_color: look.stone,
        perceptual_roughness: 0.88,
        ..default()
    });
    for target in NODE_ANCHORS {
        let dir = Vec3::new(target.x, 0.0, target.z);
        let steps = 4;
        for i in 1..=steps {
            let t = i as f32 / (steps as f32 + 0.35);
            let p = dir * t;
            commands.spawn((
                PbrBundle {
                    mesh: stone_mesh.clone(),
                    material: stone_mat.clone(),
                    transform: Transform::from_xyz(p.x, 0.04, p.z),
                    ..default()
                },
                ClimateStone,
            ));
        }
    }

    if cameras.iter().next().is_none() {
        commands.spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 7.2, 11.5)
                    .looking_at(Vec3::new(0.0, 0.4, 0.0), Vec3::Y),
                ..default()
            },
            FogSettings {
                color: look.fog,
                falloff: FogFalloff::Linear {
                    start: 10.0,
                    end: 42.0,
                },
                ..default()
            },
        ));
    } else {
        for entity in &cameras {
            commands.entity(entity).insert(FogSettings {
                color: look.fog,
                falloff: FogFalloff::Linear {
                    start: 10.0,
                    end: 42.0,
                },
                ..default()
            });
        }
    }

    if lights.iter().next().is_none() {
        commands.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 8_500.0,
                shadows_enabled: false,
                color: Color::srgb(1.0, 0.96, 0.88),
                ..default()
            },
            transform: Transform::from_xyz(8.0, 18.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });
    }

    info!(target: "powrush::climate", "climate plane seeded — Sanctuary Prime");
}

fn spawn_climate_chip(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(18.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(280.0),
                    margin: UiRect::left(Val::Px(-140.0)),
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    ..default()
                },
                background_color: Color::srgba(0.04, 0.07, 0.08, 0.72).into(),
                border_color: Color::srgba(0.55, 0.78, 0.70, 0.40).into(),
                ..default()
            },
            ClimateNameRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "Sanctuary Prime",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.86, 0.96, 0.90),
                        ..default()
                    },
                ),
                ClimateNameText,
            ));
        });
}

fn apply_climate_look(
    realm: Res<SoftPlayerRealm>,
    mut plane: ResMut<ClimatePlane>,
    mut clear: ResMut<ClearColor>,
    mut ambient: ResMut<AmbientLight>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grounds: Query<&Handle<StandardMaterial>, With<ClimateGround>>,
    stones: Query<&Handle<StandardMaterial>, With<ClimateStone>>,
    nodes: Query<&Handle<StandardMaterial>, With<MercyHarvestNode>>,
    mut fogs: Query<&mut FogSettings>,
) {
    let id = realm.current.unwrap_or(0);
    if plane.applied == Some(id) && !realm.is_changed() {
        return;
    }
    plane.applied = Some(id);
    let look = look_for(Some(id));
    clear.0 = look.sky;
    ambient.color = look.ambient;

    for handle in &grounds {
        if let Some(mat) = materials.get_mut(handle) {
            mat.base_color = look.ground;
        }
    }
    for handle in &stones {
        if let Some(mat) = materials.get_mut(handle) {
            mat.base_color = look.stone;
        }
    }
    for handle in &nodes {
        if let Some(mat) = materials.get_mut(handle) {
            mat.base_color = look.node;
            mat.emissive = LinearRgba::from(look.node).with_alpha(1.0) * 2.2;
        }
    }
    for mut fog in &mut fogs {
        fog.color = look.fog;
    }
    info!(target: "powrush::climate", climate = look.name, id, "place shifted");
}

fn update_climate_chip(
    realm: Res<SoftPlayerRealm>,
    mut text_q: Query<&mut Text, With<ClimateNameText>>,
) {
    let look = look_for(realm.current);
    for mut text in &mut text_q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != look.name {
                s.value = look.name.to_string();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn climates_disagree() {
        assert_ne!(look_for(Some(0)).name, look_for(Some(2)).name);
        assert_ne!(look_for(Some(2)).name, look_for(Some(4)).name);
    }
}
