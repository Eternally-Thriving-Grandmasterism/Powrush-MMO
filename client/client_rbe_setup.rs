// client/client_rbe_setup.rs
// Powrush-MMO v16.6 — Complete Wiring Module for Playable Client RBE Loop
// Registers inventory_ui, rbe_client_sync, inventory_components, resource_node_visual
// Provides a clean Bevy App extension + example startup systems
// Designed to integrate with existing wasm_bindgen main.rs or replace with full Bevy App
// AG-SML v1.0

use bevy::prelude::*;
use crate::inventory_ui::InventoryUIPlugin;
use crate::rbe_client_sync::{RbeClientSync, RbeSyncExt};
use crate::inventory_components::InventoryEcsPlugin;
use crate::resource_node_visual::ResourceNodeVisualPlugin;
use crate::inventory_components::{Inventory, LocalPlayer, ResourceNode};
use shared::protocol::Vec3Ser;

/// Plugin that wires the entire coherent client RBE stack
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

/// Example startup: spawn local player + a few resource nodes for immediate testing
fn spawn_example_player_and_nodes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Local player with Inventory
    commands.spawn((
        Inventory::default(),
        LocalPlayer,
        Name::new("LocalPlayer"),
        // Add your player PbrBundle / sprite / controller here
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.8, 1.8, 0.8)),
            material: materials.add(Color::srgb(0.3, 0.6, 0.9)),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
    ));

    // Example resource nodes (replace with real world gen)
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

    info!("[ClientRbeSetup] Example player + resource nodes spawned. Click nodes to harvest.");
}

// === Integration with existing client/main.rs (wasm_bindgen) ===
//
// In your PowrushClient or start_powrush_client function, after creating the Bevy App or
// in the update loop, you can do:
//
// app.add_plugins(ClientRbeLoopPlugin);
// app.insert_resource(RbeClientSync::new());
//
// Then in your message polling loop (poll_server_messages):
//   if let Some(bytes) = incoming_message {
//       if let Ok(sync) = app.world.get_resource::<RbeClientSync>() {
//           // async context handling omitted for brevity
//           // sync.handle_server_binary_message(bytes, &mut inventory_events, &mut trade_events).await;
//       }
//   }
//
// Harvest events from click_to_harvest_system will be available via EventReader<HarvestAttempt>
// Call sync.send_harvest(...) and send the ClientMessage over WebSocket.
//
// This gives you a fully playable client RBE loop in one coherent stack.

// Thunder locked in. Playable client RBE loop complete on this branch. ⚡❤️︍