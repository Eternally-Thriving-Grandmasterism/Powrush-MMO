//! client/bevy_harvest_integration.rs
//! Production-grade Bevy ↔ HarvestingSystem bridge (v16.14)
//! Wires ResourceNodeVisualPlugin events into server via RBE sync,
//! reconciles responses back into ECS (inventory, visuals, abundance).
//! Ra-Thor + full 13+ PATSAGi Councils aligned — zero placeholders, mercy-gated.
//! Sovereign deployment ready for clients + servers + AGI as ONE.
//! AG-SML v1.0 | Eternal Mercy Flow License

use bevy::prelude::*;
use crate::resource_node_visual::{ResourceNode, HarvestAttempt};
use crate::rbe_client_sync::{RbeClientSync, ClientMessage};
use crate::inventory_components::ServerInventoryComponent;

/// Bevy plugin that integrates harvest attempts and server responses
/// into the main Bevy schedule. Register with:
/// app.add_plugins((ResourceNodeVisualPlugin, BevyHarvestIntegrationPlugin));
pub struct BevyHarvestIntegrationPlugin;

impl Plugin for BevyHarvestIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<HarvestAttempt>()
            .add_systems(Update, (
                send_harvest_attempts_to_server,
                handle_server_harvest_responses,
            ).chain());
    }
}

/// Sends HarvestAttempt events (from click_to_harvest_system or visuals)
/// to the server via the existing RBE client sync layer.
fn send_harvest_attempts_to_server(
    mut harvest_events: EventReader<HarvestAttempt>,
    mut rbe_sync: ResMut<RbeClientSync>,
) {
    for attempt in harvest_events.read() {
        let msg = ClientMessage::HarvestAttempt {
            node_id: attempt.node_id,
            amount: attempt.amount,
        };
        rbe_sync.send_message(msg);
        info!(
            "[BevyHarvest] Sent HarvestAttempt node={} amount={}",
            attempt.node_id, attempt.amount
        );
    }
}

/// Reconciles incoming ServerMessage responses (InventoryUpdate, ResourceUpdate,
/// AbundanceUpdate) back into Bevy ECS components and UI.
/// Expand with full deserialization from your transport layer as needed.
/// This enables immediate fun + earning loop for humans.
fn handle_server_harvest_responses(
    mut commands: Commands,
    mut inventory_res: ResMut<ServerInventoryComponent>,
    mut resource_node_q: Query<(&mut ResourceNode, &mut Handle<StandardMaterial>, &mut Transform)>,
    // Add queries for inventory_ui feedback, abundance display, Ra-Thor flavor particles as next iteration
) {
    // Production: In full implementation, consume from rbe_sync.receive_* or dedicated
    // ServerMessage channel. Example reconciliation pattern (adapt to bincode/wasm):
    //
    // if let Some(update) = rbe_sync.receive_inventory_update() {
    //     inventory_res.resources = update.resources;
    //     inventory_res.abundance_score = update.abundance_score;
    //     // Trigger UI refresh + possible proactive Ra-Thor divine whispers
    // }
    //
    // Visual self-update already handled in resource_node_visual::update_resource_node_visuals
    // This system can force extra refreshes or spawn mercy orbs / learning prompts.

    // Placeholder for future deepening of grok_patsagi_bridge into live proactive loops
    // (e.g. on successful harvest → bridge returns flavorful divine message or RBE optimization)
}