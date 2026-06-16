// client/client_rbe_setup.rs
// Powrush-MMO — Wiring Module for the Playable Client RBE Loop
// Registers Inventory, Resource Nodes, RBE Sync, and UI systems into a single coherent plugin.
// Production-oriented setup with clear integration path for wasm_bindgen / full Bevy clients.
// AG-SML v1.0 | TOLC 8 Mercy Gates aligned

use bevy::prelude::*;
use crate::inventory_ui::InventoryUIPlugin;
use crate::rbe_client_sync::RbeClientSync;
use crate::inventory_components::InventoryEcsPlugin;
use crate::resource_node_visual::ResourceNodeVisualPlugin;
use crate::inventory_components::{Inventory, LocalPlayer, ResourceNode};
use shared::protocol::Vec3Ser;

/// Plugin that wires the full coherent client RBE stack.
pub struct ClientRbeLoopPlugin;

impl Plugin for ClientRbeLoopPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InventoryUIPlugin,
            InventoryEcsPlugin,
            ResourceNodeVisualPlugin,
        ))
        .insert_resource(RbeClientSync::new())
        .add_systems(Startup, spawn_example_player_and_nodes);
    }
}

/// Example startup system that creates a local player + sample resource nodes.
/// In a real game, replace this with proper world generation / streaming.
fn spawn_example_player_and_nodes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Local player with Inventory component
    commands.spawn((
        Inventory::default(),
        LocalPlayer,
        Name::new("LocalPlayer"),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.8, 1.8, 0.8)),
            material: materials.add(Color::srgb(0.3, 0.6, 0.9)),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
    ));

    // Example resource nodes (for testing harvest flow)
    let example_nodes = vec![
        ResourceNode {
            id: 1,
            resource_type: "ore".to_string(),
            remaining: 92.0,
            max_capacity: 100.0,
            position: Vec3Ser { x: 8.0, y: 0.5, z: 5.0 },
            ..default()
        },
        ResourceNode {
            id: 2,
            resource_type: "wood".to_string(),
            remaining: 67.0,
            max_capacity: 80.0,
            position: Vec3Ser { x: -6.0, y: 0.3, z: -4.0 },
            ..default()
        },
        ResourceNode {
            id: 3,
            resource_type: "algae".to_string(),
            remaining: 45.0,
            max_capacity: 60.0,
            position: Vec3Ser { x: 2.0, y: 0.2, z: 12.0 },
            ..default()
        },
    ];

    for node in example_nodes {
        commands.spawn(crate::resource_node_visual::ResourceNodeBundle::new(
            node,
            &mut meshes,
            &mut materials,
        ));
    }

    info!("[ClientRbeSetup] Example player + resource nodes spawned for testing.");
}

// === Integration Guidance ===
//
// This plugin is designed to be added to your Bevy App (whether in wasm_bindgen main.rs
// or a full native Bevy setup).
//
// Recommended usage:
//   app.add_plugins(ClientRbeLoopPlugin);
//
// Then in your message polling / networking layer:
//   - Call RbeClientSync::handle_server_binary_message(...) when receiving data
//   - Read HarvestAttempt / InventoryUpdated events
//   - Call game_loop.send_harvest(...) when player triggers harvest
//
// This gives you a fully playable RBE client loop with inventory, resource nodes,
// prediction, and PATSAGi monitoring integration.
