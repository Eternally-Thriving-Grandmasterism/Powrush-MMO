/*!
 * Mercy Harvest Nodes — embodied first-hour world (v22.6.0)
 *
 * Reach uses the human body. Recovery rate follows BiomeFeel.
 *
 * PATSAGi + TOLC 8 | Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::human_presence::SoftPresence;
use crate::living_ecology::BiomeFeel;
use crate::living_practice_loop::SoftPlayerRealm;

pub const HARVEST_REACH: f32 = 2.85;
const RECOVER_PER_SEC: f32 = 0.038;
const STING_SHARED: &str = "audio/mercy_harvest_sting.ogg";
const STING_SANCTUARY: &str = "audio/mercy_harvest_sting_sanctuary.ogg";
const STING_VERDANT: &str = "audio/mercy_harvest_sting_verdant.ogg";
const STING_HORIZON: &str = "audio/mercy_harvest_sting_horizon.ogg";

#[derive(Component, Debug)]
pub struct MercyHarvestNode {
    pub name: &'static str,
    pub vitality: f32,
    pub harvests: u32,
    pub pulse: f32,
}

#[derive(Resource, Debug)]
pub struct NearbyMercyNode {
    pub entity: Option<Entity>,
    pub name: Option<&'static str>,
    pub distance: f32,
    pub in_range: bool,
    pub nodes_exist: bool,
    pub last_harvested: Option<Entity>,
}

impl Default for NearbyMercyNode {
    fn default() -> Self {
        Self {
            entity: None,
            name: None,
            distance: f32::MAX,
            in_range: false,
            nodes_exist: false,
            last_harvested: None,
        }
    }
}

pub fn sting_path_for_realm(realm: Option<u8>) -> &'static str {
    match realm {
        Some(0) | Some(3) => STING_SANCTUARY,
        Some(2) => STING_VERDANT,
        Some(4) | Some(1) => STING_HORIZON,
        _ => STING_SHARED,
    }
}

pub struct MercyHarvestNodesPlugin;

impl Plugin for MercyHarvestNodesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NearbyMercyNode>()
            .add_systems(Startup, spawn_mercy_nodes)
            .add_systems(
                Update,
                (track_nearby_node, pulse_harvested_nodes, try_soft_harvest_sting),
            );
    }
}

fn spawn_mercy_nodes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut nearby: ResMut<NearbyMercyNode>,
) {
    let mesh = meshes.add(Sphere::new(0.48));
    let placements: [(&'static str, Vec3, Color); 3] = [
        (
            "Sanctuary ember",
            Vec3::new(3.6, 0.55, 0.0),
            Color::srgb(0.35, 0.95, 0.62),
        ),
        (
            "Verdant well",
            Vec3::new(-2.4, 0.55, 3.1),
            Color::srgb(0.45, 0.88, 0.95),
        ),
        (
            "Horizon seed",
            Vec3::new(1.2, 0.55, -3.4),
            Color::srgb(0.95, 0.86, 0.42),
        ),
    ];

    for (name, pos, color) in placements {
        let emissive = LinearRgba::from(color).with_alpha(1.0) * 2.4;
        commands
            .spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: materials.add(StandardMaterial {
                        base_color: color,
                        emissive,
                        perceptual_roughness: 0.35,
                        metallic: 0.05,
                        ..default()
                    }),
                    transform: Transform::from_translation(pos),
                    ..default()
                },
                MercyHarvestNode {
                    name,
                    vitality: 1.0,
                    harvests: 0,
                    pulse: 0.0,
                },
                Name::new(format!("MercyNode:{name}")),
            ))
            .with_children(|c| {
                c.spawn(PointLightBundle {
                    point_light: PointLight {
                        color,
                        intensity: 420.0,
                        range: 6.5,
                        shadows_enabled: false,
                        ..default()
                    },
                    ..default()
                });
            });
    }
    nearby.nodes_exist = true;
    info!(target: "powrush::nodes", "three mercy harvest nodes seeded in the walk plane");
}

fn player_xy(presence: Option<&SoftPresence>) -> Vec3 {
    presence.map(|p| p.position).unwrap_or(Vec3::ZERO)
}

fn track_nearby_node(
    presence: Option<Res<SoftPresence>>,
    nodes: Query<(Entity, &Transform, &MercyHarvestNode)>,
    mut nearby: ResMut<NearbyMercyNode>,
) {
    let pos = player_xy(presence.as_deref());
    nearby.nodes_exist = !nodes.is_empty();
    let mut best: Option<(Entity, &'static str, f32)> = None;
    for (entity, tf, node) in &nodes {
        let d = tf.translation.distance(pos);
        match best {
            None => best = Some((entity, node.name, d)),
            Some((_, _, bd)) if d < bd => best = Some((entity, node.name, d)),
            _ => {}
        }
    }
    if let Some((entity, name, d)) = best {
        nearby.entity = Some(entity);
        nearby.name = Some(name);
        nearby.distance = d;
        nearby.in_range = d <= HARVEST_REACH;
    } else {
        nearby.entity = None;
        nearby.name = None;
        nearby.distance = f32::MAX;
        nearby.in_range = false;
    }
}

fn pulse_harvested_nodes(
    time: Res<Time>,
    feel: Option<Res<BiomeFeel>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut nodes: Query<(
        &mut MercyHarvestNode,
        &mut Transform,
        &Handle<StandardMaterial>,
        Option<&Children>,
    )>,
    mut lights: Query<&mut PointLight>,
) {
    let dt = time.delta_seconds();
    let t = time.elapsed_seconds();
    let mul = feel.map(|f| f.regen_mul).unwrap_or(1.0);
    for (mut node, mut tf, handle, children) in &mut nodes {
        if node.pulse > 0.0 {
            node.pulse = (node.pulse - dt * 1.35).max(0.0);
        } else if node.vitality < 1.0 {
            node.vitality = (node.vitality + dt * RECOVER_PER_SEC * mul).min(1.0);
        }
        let breathe = 1.0 + (t * 1.7).sin() * 0.06 * node.vitality;
        let burst = 1.0 + node.pulse * 0.38;
        tf.scale = Vec3::splat(0.92 * breathe * burst);
        if let Some(mat) = materials.get_mut(handle) {
            let glow = 2.4 + node.pulse * 6.2;
            mat.emissive = LinearRgba::from(mat.base_color) * glow;
        }
        if let Some(children) = children {
            for child in children.iter() {
                if let Ok(mut light) = lights.get_mut(*child) {
                    light.intensity = 420.0 + node.pulse * 2800.0;
                    light.range = 6.5 + node.pulse * 3.5;
                }
            }
        }
    }
}

pub fn apply_node_harvest(node: &mut MercyHarvestNode) {
    node.harvests = node.harvests.saturating_add(1);
    node.vitality = (node.vitality * 0.92).max(0.45);
    node.pulse = 1.0;
}

pub fn apply_node_tend(node: &mut MercyHarvestNode) {
    node.vitality = (node.vitality + 0.14).min(1.0);
    node.pulse = 0.55;
}

fn try_soft_harvest_sting(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    nearby: Res<NearbyMercyNode>,
    realm: Option<Res<SoftPlayerRealm>>,
    mut last: Local<Option<u32>>,
    nodes: Query<&MercyHarvestNode>,
) {
    let Some(entity) = nearby.last_harvested else {
        return;
    };
    let Ok(node) = nodes.get(entity) else {
        return;
    };
    if *last == Some(node.harvests) {
        return;
    }
    *last = Some(node.harvests);
    let path = sting_path_for_realm(realm.and_then(|r| r.current));
    commands.spawn(AudioBundle {
        source: asset_server.load(path),
        settings: PlaybackSettings::DESPAWN,
        ..default()
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn harvest_leaves_node_alive() {
        let mut n = MercyHarvestNode {
            name: "test",
            vitality: 1.0,
            harvests: 0,
            pulse: 0.0,
        };
        apply_node_harvest(&mut n);
        assert!(n.vitality < 1.0 && n.vitality >= 0.45);
        assert_eq!(n.harvests, 1);
    }

    #[test]
    fn tend_restores_vitality() {
        let mut n = MercyHarvestNode {
            name: "test",
            vitality: 0.50,
            harvests: 1,
            pulse: 0.0,
        };
        apply_node_tend(&mut n);
        assert!(n.vitality > 0.50);
    }
}
