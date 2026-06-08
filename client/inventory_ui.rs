// client/inventory_ui.rs
// Powrush-MMO v16.5.24 — Enhanced GPU Panel with Per-Node Predictions
// Shows detailed per-node GPU forecasts in the UI.
// AG-SML v1.0

use bevy::prelude::*;
use shared::protocol::{ClientMessage, ServerMessage, TradeOffer, Vec3Ser};
use std::collections::HashMap;

use crate::rbe_client_sync::GpuSimulationState;

// ... (previous structs and events preserved)

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
pub struct InventoryUpdated { /* ... */ }
#[derive(Event)]
pub struct TradeResponseReceived { /* ... */ }
#[derive(Event)]
pub struct HarvestResponseReceived { /* ... */ }

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

fn spawn_inventory_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Existing inventory + hotbar + GPU panel structure...
    // GPU panel now includes per-node section

    commands.spawn((
        Node {
            width: Val::Px(280.0),
            height: Val::Px(200.0),
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
        parent.spawn((Text::new("Global Confidence: --"), TextFont { font_size: 13.0, ..default() }, TextColor(Color::srgb(0.6, 1.0, 0.6)), Name::new("GpuConfidenceText")));
        parent.spawn((Text::new("Nodes tracked: 0"), TextFont { font_size: 12.0, ..default() }, TextColor(Color::srgb(0.8, 0.8, 0.8)), Name::new("GpuNodeCountText")));

        // Per-node predictions section
        parent.spawn((Text::new("Top Node Forecasts:"), TextFont { font_size: 12.0, ..default() }, TextColor(Color::srgb(0.9, 0.9, 0.6)), Node { margin: UiRect::top(Val::Px(6.0)), ..default() }));
        parent.spawn((Text::new("(No data yet)"), TextFont { font_size: 11.0, ..default() }, TextColor(Color::srgb(0.7, 0.7, 0.7)), Name::new("GpuNodePredictionsText")));

        parent.spawn((Text::new("Last update: --"), TextFont { font_size: 11.0, ..default() }, TextColor(Color::srgb(0.7, 0.7, 0.7)), Name::new("GpuNotesText")));
    });
}

/// Enhanced GPU panel update with per-node predictions
fn update_gpu_panel(
    gpu_state: Res<GpuSimulationState>,
    mut text_query: Query<(&mut Text, &Name)>,
) {
    for (mut text, name) in text_query.iter_mut() {
        if name.as_str() == "GpuConfidenceText" {
            text.sections[0].value = format!("Global Confidence: {:.1}%", gpu_state.global_confidence * 100.0);
        }
        if name.as_str() == "GpuNodeCountText" {
            text.sections[0].value = format!("Nodes tracked: {}", gpu_state.node_predictions.len());
        }
        if name.as_str() == "GpuNotesText" {
            text.sections[0].value = format!("Last update: {}", gpu_state.last_update_notes);
        }
        if name.as_str() == "GpuNodePredictionsText" {
            if gpu_state.node_predictions.is_empty() {
                text.sections[0].value = "(No data yet)".to_string();
            } else {
                // Show top 3 nodes by predicted depletion
                let mut sorted: Vec<_> = gpu_state.node_predictions.iter().collect();
                sorted.sort_by(|a, b| b.1.predicted_depletion.partial_cmp(&a.1.predicted_depletion).unwrap());

                let top_nodes: Vec<String> = sorted.iter().take(3).map(|(id, pred)| {
                    format!("Node {}: Dep {:.0}% | Regen {:.3}", id, pred.predicted_depletion * 100.0, pred.recommended_regen_rate)
                }).collect();

                text.sections[0].value = top_nodes.join("\n");
            }
        }
    }
}

// ... (rest of the file with existing systems preserved)