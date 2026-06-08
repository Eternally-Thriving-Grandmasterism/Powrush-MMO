// client/resource_node_visual.rs
// Powrush-MMO v16.5.33 — Visual Effects for GPU Predictions & Stress
// Nodes under stress (low fullness) now look visibly damaged/warning.
// AG-SML v1.0 | Ra-Thor / PATSAGi aligned

use bevy::prelude::*;
use crate::inventory_components::{ResourceNode, HarvestAttempt, LocalPlayer};
use crate::client_game_loop::ClientGameLoop;
use shared::protocol::Vec3Ser;

#[derive(Bundle)]
pub struct ResourceNodeBundle {
    pub node: ResourceNode,
    pub pbr: PbrBundle,
    pub name: Name,
}

impl ResourceNodeBundle {
    pub fn new(node: ResourceNode, meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) -> Self {
        let color = resource_color(&node.resource_type, node.remaining / node.max_capacity);
        Self {
            node,
            pbr: PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                material: materials.add(StandardMaterial {
                    base_color: color,
                    ..default()
                }),
                transform: Transform::from_xyz(node.position.x, node.position.y, node.position.z),
                ..default()
            },
            name: Name::new(format!("ResourceNode-{}", node.id)),
        }
    }
}

fn resource_color(resource_type: &str, fullness: f32) -> Color {
    // Shift toward warning colors (orange/red) when stressed/low
    let stress = 1.0 - fullness;
    match resource_type {
        "wood" | "organic" => Color::srgb(
            0.2 + fullness * 0.3 + stress * 0.5,
            0.6 - stress * 0.4,
            0.2 - stress * 0.1
        ),
        "ore" | "mineral" => Color::srgb(
            0.5 + stress * 0.3,
            0.5 - stress * 0.2,
            0.6 + fullness * 0.3 - stress * 0.3
        ),
        "algae" | "bio" => Color::srgb(
            0.1 + stress * 0.4,
            0.7 + fullness * 0.2 - stress * 0.5,
            0.4 - stress * 0.2
        ),
        _ => Color::srgb(0.6 + stress * 0.3, 0.6 - stress * 0.3, 0.6 - stress * 0.2),
    }
}

pub fn update_resource_node_visuals(
    mut query: Query<(&ResourceNode, &mut Handle<StandardMaterial>, &mut Transform)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (node, mat_handle, mut transform) in query.iter_mut() {
        if let Some(material) = materials.get_mut(mat_handle) {
            let fullness = (node.remaining / node.max_capacity).clamp(0.0, 1.0);
            material.base_color = resource_color(&node.resource_type, fullness);

            // Base scale from fullness
            let base_scale = 0.7 + fullness * 0.6;

            // Add subtle pulsing on stressed nodes (low fullness)
            let stress = 1.0 - fullness;
            let pulse = (time.elapsed_seconds() * 3.0).sin() * 0.05 * stress;
            transform.scale = Vec3::splat(base_scale + pulse);
        }
    }
}

pub fn click_to_harvest_system(
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    node_query: Query<(Entity, &ResourceNode, &GlobalTransform)>,
    mut harvest_events: EventWriter<HarvestAttempt>,
    local_player: Query<Entity, With<LocalPlayer>>,
    mut game_loop: ResMut<ClientGameLoop>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) { return; }

    let Ok(window) = windows.get_single() else { return };
    let Some(cursor_pos) = window.cursor_position() else { return };
    let Ok((camera, cam_transform)) = camera_query.get_single() else { return };

    if let Some(ray) = camera.viewport_to_world(cam_transform, cursor_pos) {
        let mut closest: Option<(Entity, f32, u64)> = None;

        for (entity, node, node_transform) in node_query.iter() {
            let node_pos = node_transform.translation();
            let dist = ray.origin.distance(node_pos);
            if dist < 15.0 {
                if closest.map_or(true, |(_, d, _)| dist < d) {
                    closest = Some((entity, dist, node.id));
                }
            }
        }

        if let Some((_, _, node_id)) = closest {
            if local_player.get_single().is_ok() {
                let amount = 10.0;
                game_loop.send_harvest(0, node_id, amount);
                harvest_events.send(HarvestAttempt { node_id, amount });
                info!("[ResourceNodeVisual] Harvest dispatched on node {} via game loop", node_id);
            }
        }
    }
}

pub struct ResourceNodeVisualPlugin;

impl Plugin for ResourceNodeVisualPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_resource_node_visuals,
            click_to_harvest_system,
        ));
    }
}

// Thunder locked in. Visual effects for GPU-driven stress now active. ⚡️❤️🔥