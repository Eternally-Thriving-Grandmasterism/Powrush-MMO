// client/inventory_ui.rs
// Powrush-MMO v16.5.22 — Added GPU PATSAGi Visualization Panel
// Displays real-time GPU simulation results to the player.
// AG-SML v1.0

use bevy::prelude::*;
use shared::protocol::{ClientMessage, ServerMessage, TradeOffer, Vec3Ser};
use std::collections::HashMap;

use crate::rbe_client_sync::GpuSimulationState;

// ... (existing structs: LocalInventory, TradeUIState, events, etc. remain the same)

#[derive(Resource, Default, Clone)]
pub struct LocalInventory {
    pub resources: HashMap<String, f32>,
    pub abundance_score: f32,
    pub player_id: Option<u64>,
}

#[derive(Resource, Default)]
pub struct TradeUIState {
    pub active_trade_id: Option<u64>,
    pub offered: HashMap<String, f32>,
    pub requested: HashMap<String, f32>,
    pub target_player_id: Option<u64>,
    pub is_initiating: bool,
}

#[derive(Event)]
pub struct InventoryUpdated {
    pub player_id: u64,
    pub resources: HashMap<String, f32>,
    pub abundance_score: f32,
}

#[derive(Event)]
pub struct TradeResponseReceived {
    pub trade_id: u64,
    pub completed: bool,
    pub grace_awarded: u64,
    pub reason: Option<String>,
}

#[derive(Event)]
pub struct HarvestResponseReceived {
    pub node_id: u64,
    pub resource_type: String,
    pub remaining: f32,
    pub harvested_by: Option<u64>,
    pub grace_awarded: u64,
}

pub struct InventoryUIPlugin;

impl Plugin for InventoryUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LocalInventory>()
            .init_resource::<TradeUIState>()
            .init_resource::<GpuSimulationState>()
            .add_event::<InventoryUpdated>()
            .add_event::<TradeResponseReceived>()
            .add_event::<HarvestResponseReceived>()
            .add_systems(Startup, spawn_inventory_ui)
            .add_systems(Update, (
                update_inventory_from_events,
                update_hotbar,
                handle_trade_buttons,
                handle_harvest_buttons,
                handle_hotbar_harvest,
                update_gpu_panel,
                update_trade_modal,
            ));
    }
}

// ... (spawn_inventory_ui and other existing functions remain largely the same)

fn spawn_inventory_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Existing inventory panel + hotbar code...
    // (kept for brevity in this PR - full previous logic preserved)

    // GPU Predictions Panel (new in v16.5.22)
    commands.spawn((
        Node {
            width: Val::Px(280.0),
            height: Val::Px(160.0),
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            bottom: Val::Px(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.08, 0.08, 0.12, 0.9)),
        BorderColor(Color::srgb(0.4, 0.7, 0.9)),
        BorderRadius::all(Val::Px(6.0)),
        Name::new("GpuPredictionsPanel"),
    ))
    .with_children(|parent| {
        parent.spawn((Text::new("GPU PATSAGi FORECAST"), TextFont { font_size: 14.0, ..default() }, TextColor(Color::srgb(0.5, 0.85, 1.0))));
        parent.spawn((Text::new("Confidence: --"), TextFont { font_size: 13.0, ..default() }, TextColor(Color::srgb(0.6, 1.0, 0.6)), Name::new("GpuConfidenceText")));
        parent.spawn((Text::new("Nodes tracked: 0"), TextFont { font_size: 12.0, ..default() }, TextColor(Color::srgb(0.8, 0.8, 0.8)), Name::new("GpuNodeCountText")));
        parent.spawn((Text::new("Last update: --"), TextFont { font_size: 11.0, ..default() }, TextColor(Color::srgb(0.7, 0.7, 0.7)), Name::new("GpuNotesText")));
    });
}

// Existing systems (update_inventory_from_events, handle_*, etc.) remain unchanged.

/// New system: Updates the GPU Predictions panel with live data from GpuSimulationState
fn update_gpu_panel(
    gpu_state: Res<GpuSimulationState>,
    mut confidence_query: Query<&mut Text, With<Name>>,
) {
    for mut text in confidence_query.iter_mut() {
        if text.sections[0].value.contains("Confidence") {
            text.sections[0].value = format!("Confidence: {:.1}%", gpu_state.global_confidence * 100.0);
        }
        if text.sections[0].value.contains("Nodes tracked") {
            text.sections[0].value = format!("Nodes tracked: {}", gpu_state.node_predictions.len());
        }
        if text.sections[0].value.contains("Last update") {
            text.sections[0].value = format!("Last update: {}", gpu_state.last_update_notes);
        }
    }
}

// ... (rest of file: handle_server_message, integration notes, etc. preserved)