/*!
 * Mercy Harvest Nodes — embodied first-hour world (v21.99.0)
 *
 * Three glowing nodes in the XY walk plane. E harvests only when near.
 * Nodes dim slightly but keep glowing (restraint, not extraction).
 * Soft audio sting is attempted; missing assets fail silently.
 *
 * PATSAGi + TOLC 8 | Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::prediction::PredictedPosition;

/// How close a human must stand (world units) before E is world-care.
pub const HARVEST_REACH: f32 = 2.85;

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

pub struct MercyHarvestNodesPlugin;

impl Plugin for MercyHarvestNodesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NearbyMercyNode>()
            .add_systems(Startup, spawn_mercy_nodes)
            .add_systems(
                Update,
                (
                    track_nearby_node,
                    pulse_harvested_nodes,
                    try_soft_harvest_sting,
                ),
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
        commands.spawn((
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
        ));
    }
    nearby.nodes_exist = true;
    info!(target: "powrush::nodes", "three mercy harvest nodes seeded in the walk plane");
}

fn player_xy(query: &Query<&PredictedPosition>) -> Vec3 {
    if let Some(p) = query.iter().next() {
        return p.position;
    }
    Vec3::ZERO
}

fn track_nearby_node(
    player: Query<&PredictedPosition>,
    nodes: Query<(Entity, &Transform, &MercyHarvestNode)>,
    mut nearby: ResMut<NearbyMercyNode>,
) {
    let pos = player_xy(&player);
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
    nearby: Res<NearbyMercyNode>,
    mut nodes: Query<(&mut MercyHarvestNode, &mut Transform)>,
) {
    let dt = time.delta_seconds();
    for (mut node, mut tf) in &mut nodes {
        if nearby.last_harvested == Some(/* compared below via harvests pulse */ Entity::PLACEHOLDER)
        {
            // no-op guard; pulse is driven by node.pulse
        }
        if node.pulse > 0.0 {
            node.pulse = (node.pulse - dt * 1.35).max(0.0);
        }
        let breathe = 1.0 + (time.elapsed_seconds() * 1.7).sin() * 0.06 * node.vitality;
        let burst = 1.0 + node.pulse * 0.28;
        let s = 0.92 * breathe * burst;
        tf.scale = Vec3::splat(s);
    }
}

/// Called by first-harvest epiphany after a successful in-range E.
pub fn apply_node_harvest(node: &mut MercyHarvestNode) {
    node.harvests = node.harvests.saturating_add(1);
    node.vitality = (node.vitality * 0.92).max(0.45);
    node.pulse = 1.0;
}

fn try_soft_harvest_sting(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    nearby: Res<NearbyMercyNode>,
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
    // Soft-fail: if the ogg is absent Bevy logs once; playback still attempts.
    commands.spawn(AudioBundle {
        source: asset_server.load("audio/mercy_harvest_sting.ogg"),
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
        assert!(n.pulse > 0.0);
    }

    #[test]
    fn reach_is_human_scale() {
        assert!(HARVEST_REACH > 1.5 && HARVEST_REACH < 5.0);
    }
}
