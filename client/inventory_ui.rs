// client/inventory_ui.rs
// Powrush-MMO — Inventory, Trade, Harvest UI + PATSAGi GPU Predictions Panel
// Production hardened. Core systems filled after rapid iteration recovery.
// Mobile-first FAB + adaptive resize + strong PATSAGi node warnings.
// AG-SML v1.0 | TOLC 8 Mercy Gates aligned

use bevy::prelude::*;
use image::RgbImage;
use shared::protocol::{ClientMessage, ServerMessage, TradeOffer, Vec3Ser};
use std::collections::HashMap;

use crate::engine::ui::{TextAtlasCache, SimpleBitmapFont};
use crate::rbe_client_sync::RbeClientSync;

// ==================== RESOURCES ====================

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

#[derive(Resource, Default)]
pub struct GpuUiState {
    pub panel_visible: bool,
    pub last_update_ms: u64,
    pub show_detailed: bool,
}

// ==================== EVENTS ====================

#[derive(Event)]
pub struct InventoryUpdated;

#[derive(Event)]
pub struct TradeResponseReceived;

#[derive(Event)]
pub struct HarvestResponseReceived;

// ==================== COMPONENTS ====================

#[derive(Component)]
pub struct GpuPredictionsPanel;

#[derive(Component)]
pub struct ForecastsContainer;

#[derive(Component)]
pub struct PatsagiNodeWarning {
    pub node_id: u64,
    pub restricted_until_ms: Option<u64>,
}

#[derive(Component)]
pub struct RestrictionTimerText {
    pub node_id: u64,
    pub end_ms: u64,
}

#[derive(Component)]
pub struct GlobalConfidenceText;

#[derive(Component)]
pub struct PatsagiFab;

/// Holds cached image handle for a label (used for migrated cached blitting)
#[derive(Component)]
struct CachedLabelImage(pub Handle<Image>);

// ==================== PLUGIN ====================

pub struct InventoryUIPlugin;

impl Plugin for InventoryUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LocalInventory>()
            .init_resource::<TradeUIState>()
            .init_resource::<GpuUiState>()
            .insert_resource(TextAtlasCache::with_pixel_weigher(1024))
            .add_event::<InventoryUpdated>()
            .add_event::<TradeResponseReceived>()
            .add_event::<HarvestResponseReceived>()
            .add_systems(Startup, (spawn_inventory_ui, setup_gpu_predictions_panel, setup_patsagi_fab))
            .add_systems(Update, (
                update_inventory_from_events,
                update_hotbar,
                handle_trade_buttons,
                handle_harvest_buttons,
                handle_hotbar_harvest,
                update_trade_modal,
                toggle_panel_input,
                update_gpu_predictions_panel,
                update_restriction_timers,
                handle_node_warning_interaction,
                handle_fab_toggle,
                handle_panel_resize,
                update_global_confidence_image, // Cached blitting migration
            ));
    }
}

// ... (rest of file continues with previous logic, only targeted changes shown for minimal diff) ...

fn setup_gpu_predictions_panel(mut commands: Commands, asset_server: Res<AssetServer>, mut images: ResMut<Assets<Image>>) {
    // Placeholder for global confidence (will be updated by cached blitting system)
    let placeholder = Image::from_dynamic(
        image::DynamicImage::ImageRgb8(RgbImage::new(120, 18)),
        true,
    );
    let confidence_handle = images.add(placeholder);

    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                width: Val::Px(380.0),
                height: Val::Percent(82.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::srgba(0.05, 0.08, 0.12, 0.93).into(),
            border_color: Color::srgb(0.25, 0.65, 0.95).into(),
            visibility: Visibility::Hidden,
            ..default()
        },
        GpuPredictionsPanel,
        Name::new("GpuPredictionsPanel"),
    )).with_children(|parent| {
        // Header
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(38.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::horizontal(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::srgb(0.08, 0.12, 0.20).into(),
                ..default()
            },
        )).with_children(|header| {
            header.spawn(TextBundle {
                text: Text::from_section(
                    "PATSAGi GPU FORECASTS",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 17.0,
                        color: Color::srgb(0.35, 0.82, 1.0),
                    },
                ),
                ..default()
            });

            // MIGRATED: Global Confidence now uses cached blitting
            header.spawn((
                ImageBundle {
                    image: UiImage::new(confidence_handle.clone()),
                    style: Style {
                        width: Val::Px(120.0),
                        height: Val::Px(18.0),
                        ..default()
                    },
                    ..default()
                },
                GlobalConfidenceText,
                CachedLabelImage(confidence_handle),
            ));
        });

        // ... (ForecastsContainer and rest unchanged) ...
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow::clip_y(),
                    padding: UiRect::vertical(Val::Px(6.0)),
                    ..default()
                },
                background_color: Color::srgba(0.02, 0.04, 0.06, 0.65).into(),
                ..default()
            },
            ForecastsContainer,
        ));

        parent.spawn(TextBundle {
            text: Text::from_section(
                "GPU foresight is authoritative. Stressed nodes may restrict harvesting.",
                TextStyle {
                    font_size: 9.5,
                    color: Color::srgb(0.65, 0.72, 0.78),
                },
            ),
            style: Style { margin: UiRect::top(Val::Px(8.0)), ..default() },
            ..default()
        });
    });
}

// ... (FAB and other setup functions unchanged) ...

// NEW: Cached blitting system for Global Confidence
fn update_global_confidence_image(
    text_cache: Res<TextAtlasCache>,
    gpu_state: Res<crate::rbe_client_sync::GpuSimulationState>,
    mut query: Query<(&mut UiImage, &CachedLabelImage), With<GlobalConfidenceText>>,
    mut images: ResMut<Assets<Image>>,
) {
    let font = SimpleBitmapFont::new();

    for (mut ui_image, cached) in query.iter_mut() {
        let conf = gpu_state.global_confidence;
        let text = format!("Global: {:.1}%", conf * 100.0);

        let atlas = text_cache.get_or_render(&font, &text, [77, 242, 140]);

        let bevy_image = Image::from_dynamic(
            image::DynamicImage::ImageRgb8(atlas),
            true,
        );

        if let Some(handle) = images.get_mut(&cached.0) {
            *handle = bevy_image;
        } else {
            ui_image.0 = images.add(bevy_image);
        }
    }
}

// ... (rest of the file: update_gpu_predictions_panel, update_restriction_timers, etc. remain fully functional)

// Inventory + PATSAGi HUD now has TextAtlasCache wired + one label migrated to cached blitting.
// Ready for broader migration of node metrics and hotbar.