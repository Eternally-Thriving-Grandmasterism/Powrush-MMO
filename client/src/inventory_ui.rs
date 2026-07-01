/*!
 * Inventory UI - Expanded professional system
 * Hotbar + Full Grid + Drag/Drop skeleton + Tooltips + Filters + RBE/Abundance/Mercy integration
 * Preserves ALL original hotbar code exactly. Builds on GpuSimulationState, CachedLabelImage, TextAtlasCache patterns.
 * AG-SML v1.0 | TOLC 8 + 7 Mercy Gates | Ra-Thor / PATSAGi aligned
 * Mercy-themed visuals, resonance/valence colors, epiphany/ascension item support, abundance flow indicators.
 * Next: Wire drag events to server trade/use/equip; full item data from rbe_client_sync + content/locales.
 */

// === PRESERVED ORIGINAL HOTBAR CODE (exact, no changes) ===

fn update_hotbar_item_count_images(
    text_cache: Res<TextAtlasCache>,
    gpu_state: Res<crate::rbe_client_sync::GpuSimulationState>,
    mut query: Query<(
        &mut UiImage,
        &CachedLabelImage,
        &mut LastRenderedText,
        &mut LastRenderedColor,
        &HotbarItemCountText,
    )>,
    mut images: ResMut<Assets<Image>>,
) {
    let font = SimpleBitmapFont::new();

    for (mut ui_image, cached, mut last_text, mut last_color, hotbar_slot) in query.iter_mut() {
        let idx = hotbar_slot.slot_index as usize;

        let count = gpu_state.hotbar.get(idx)
            .map(|slot| slot.count)
            .unwrap_or(0);

        let new_text = format!("x{:02}", count);

        if last_text.text != new_text {
            let atlas = text_cache.get_or_render(&font, &new_text, [255, 220, 100]);

            if let Some(bevy_img) = images.get_mut(&cached.0) {
                update_bevy_image_from_atlas(bevy_img, &atlas);
            }

            last_text.text = new_text;
            last_color.0 = [255, 220, 100];
        }
    }
}

fn update_hotbar_cooldown_images(
    text_cache: Res<TextAtlasCache>,
    gpu_state: Res<crate::rbe_client_sync::GpuSimulationState>,
    mut query: Query<(
        &mut UiImage,
        &CachedLabelImage,
        &mut LastRenderedText,
        &mut LastRenderedColor,
        &HotbarCooldownText,
    )>,
    mut images: ResMut<Assets<Image>>,
) {
    let font = SimpleBitmapFont::new();

    for (mut ui_image, cached, mut last_text, mut last_color, cooldown_slot) in query.iter_mut() {
        let idx = cooldown_slot.slot_index as usize;

        let remaining = gpu_state.hotbar.get(idx)
            .map(|slot| slot.cooldown_remaining)
            .unwrap_or(0.0);

        let new_text = if remaining > 0.0 {
            format!("{:.1}s", remaining)
        } else {
            String::from("")
        };
        let new_color = if remaining > 0.0 {
            [255, 180, 80]
        } else {
            [120, 255, 150]
        };

        if last_text.text != new_text || last_color.0 != new_color {
            let atlas = text_cache.get_or_render(&font, &new_text, new_color);

            if let Some(bevy_img) = images.get_mut(&cached.0) {
                update_bevy_image_from_atlas(bevy_img, &atlas);
            }

            last_text.text = new_text;
            last_color.0 = new_color;
        }
    }
}

// === EXPANDED FULL INVENTORY UI (new professional code below - additive, preserves all above) ===

use bevy::prelude::*;
use crate::rbe_client_sync::GpuSimulationState; // Extend with .inventory or use separate InventoryRes in future
use crate::ui_utils::{spawn_cached_label, CachedLabelImage, LastRenderedText, LastRenderedColor, TextAtlasCache, SimpleBitmapFont, update_bevy_image_from_atlas};
// TODO: Add inventory_components for InventorySlot, DraggableItem, InventoryFilter, ItemTooltip if not in inventory_components.rs

/// Marker for the main inventory panel root entity (opened via pause/key or council UI)
#[derive(Component)]
pub struct InventoryPanel;

/// Per-slot marker for grid/hotbar items
#[derive(Component, Clone, Copy)]
pub struct InventorySlot {
    pub index: u32,
    pub is_hotbar: bool, // false = main grid
}

/// Drag state (simple skeleton - expand with bevy_mod_picking or Pointer events for full drag/drop)
#[derive(Resource, Default)]
pub struct InventoryDragState {
    pub dragging_from: Option<(u32, bool)>, // (index, is_hotbar)
    pub dragging_item_id: Option<u64>,      // or ItemId type
}

/// Filter + search state for the inventory view
#[derive(Resource, Default)]
pub struct InventoryFilter {
    pub search: String,
    pub category: InventoryCategory,
    pub sort_by: InventorySort,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum InventoryCategory {
    #[default]
    All,
    Resources,
    Equipment,
    AscensionRelics,
    Redeemed, // epiphany/council granted
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum InventorySort {
    #[default]
    CountDesc,
    Resonance,
    Valence,
    Name,
    RBEAbundance,
}

/// Tooltip data (spawned on hover, mercy-themed)
#[derive(Component)]
pub struct ItemTooltip {
    pub item_index: u32,
    pub timer: Timer, // auto-hide
}

/// Example item metadata pulled from GpuSimulationState or extended RBE (placeholder - wire to real data)
#[derive(Clone)]
pub struct InventoryItemView {
    pub name: String,
    pub count: u32,
    pub cooldown: f32,
    pub resonance: f32,   // 0.0-1.0 mercy alignment
    pub valence: f32,     // emotional/spiritual charge
    pub abundance_flow: f32, // RBE infinite abundance indicator
    pub category: InventoryCategory,
    pub lore_snippet: String, // from content/locales or epiphany
    pub ascension_bonus: Option<String>,
}

/// System: Open/close inventory panel (call from input or pause_menu)
pub fn toggle_inventory_panel(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    panel_query: Query<Entity, With<InventoryPanel>>,
    mut filter: ResMut<InventoryFilter>,
) {
    if keyboard.just_pressed(KeyCode::I) || keyboard.just_pressed(KeyCode::Tab) {
        if let Ok(entity) = panel_query.get_single() {
            commands.entity(entity).despawn_recursive();
        } else {
            filter.search.clear();
            filter.category = InventoryCategory::All;
            spawn_inventory_panel(&mut commands);
        }
    }
}

fn spawn_inventory_panel(commands: &mut Commands) {
    // Root panel - mercy themed background (use existing valence_halo or panel styles from settings_menu/pause_menu)
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(70.0),
                height: Val::Percent(80.0),
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            background_color: Color::rgba(0.05, 0.08, 0.12, 0.95).into(), // deep mercy blue
            border_color: Color::rgb(0.2, 0.6, 0.8).into(), // valence cyan
            ..default()
        },
        InventoryPanel,
        Name::new("InventoryPanel"),
    )).with_children(|parent| {
        // Header with title + filters
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(20.0)),
                ..default()
            },
            ..default()
        }).with_children(|header| {
            header.spawn(TextBundle {
                text: Text::from_section(
                    "INVENTORY — RBE Abundance Flow | Mercy Lattice",
                    TextStyle { font_size: 22.0, color: Color::rgb(0.6, 0.9, 1.0), ..default() },
                ),
                ..default()
            });

            // Search input placeholder (expand with egui or Bevy text input system)
            header.spawn(TextBundle {
                text: Text::from_section("[Search: Type to filter]", TextStyle { font_size: 14.0, color: Color::rgb(0.7, 0.7, 0.8), ..default() }),
                style: Style { margin: UiRect::left(Val::Px(30.0)), ..default() },
                ..default()
            });

            // Category filters (buttons - wire to systems that update Res<InventoryFilter>)
            for (label, cat) in [("All", InventoryCategory::All), ("Resources", InventoryCategory::Resources), ("Equip", InventoryCategory::Equipment), ("Relics", InventoryCategory::AscensionRelics), ("Redeemed", InventoryCategory::Redeemed)] {
                header.spawn(ButtonBundle {
                    style: Style { margin: UiRect::horizontal(Val::Px(4.0)), padding: UiRect::all(Val::Px(6.0)), ..default() },
                    background_color: Color::rgb(0.15, 0.25, 0.35).into(),
                    ..default()
                }).with_children(|btn| {
                    btn.spawn(TextBundle { text: Text::from_section(label, TextStyle { font_size: 12.0, color: Color::WHITE, ..default() }), ..default() });
                });
            }
        });

        // Main grid area (scrollable container)
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(95.0),
                height: Val::Percent(75.0),
                overflow: Overflow::clip_y(), // or scroll
                flex_wrap: FlexWrap::Wrap,
                align_content: AlignContent::FlexStart,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: Color::rgba(0.02, 0.04, 0.06, 0.8).into(),
            ..default()
        }).with_children(|grid| {
            // Spawn 30-50 slot placeholders (or virtualize). Each slot has icon + count + hover tooltip
            for i in 0..40u32 {
                grid.spawn((
                    ButtonBundle { // or Node + interaction for drag
                        style: Style {
                            width: Val::Px(64.0),
                            height: Val::Px(64.0),
                            margin: UiRect::all(Val::Px(4.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        background_color: Color::rgb(0.1, 0.15, 0.2).into(),
                        border_color: Color::rgb(0.3, 0.5, 0.7).into(), // resonance border
                        ..default()
                    },
                    InventorySlot { index: i, is_hotbar: false },
                    Name::new(format!("InvSlot_{}", i)),
                )).with_children(|slot| {
                    // Item icon placeholder (replace with real texture from assets or gpu material)
                    slot.spawn(ImageBundle {
                        style: Style { width: Val::Percent(80.0), height: Val::Percent(80.0), ..default() },
                        background_color: Color::rgb(0.4, 0.6, 0.5).into(), // placeholder green for resources
                        ..default()
                    });
                    // Count label (reuse cached pattern)
                    slot.spawn(TextBundle {
                        text: Text::from_section(format!("x{:02}", (i % 12) + 1), TextStyle { font_size: 11.0, color: Color::rgb(1.0, 0.9, 0.6), ..default() }),
                        style: Style { position_type: PositionType::Absolute, bottom: Val::Px(2.0), right: Val::Px(4.0), ..default() },
                        ..default()
                    });
                });
            }
        });

        // Footer: RBE abundance summary + mercy gates status
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(20.0)),
                ..default()
            },
            background_color: Color::rgba(0.08, 0.12, 0.18, 0.9).into(),
            ..default()
        }).with_children(|footer| {
            footer.spawn(TextBundle {
                text: Text::from_section(
                    "RBE Abundance: Infinite Flow  |  Mercy Gates: 7/7 Open  |  Council Synergy: +12% Harvest",
                    TextStyle { font_size: 13.0, color: Color::rgb(0.5, 0.95, 0.7), ..default() },
                ),
                ..default()
            });
        });
    });
}

/// System: Update inventory grid slots from GpuSimulationState (or extended InventoryRes)
/// Reuses cached label pattern from hotbar for performance
pub fn update_inventory_grid(
    text_cache: Res<TextAtlasCache>,
    gpu_state: Res<GpuSimulationState>,
    filter: Res<InventoryFilter>,
    mut query: Query<(
        &InventorySlot,
        &mut UiImage,
        Option<&mut Text>, // count text child
    )>,
    mut images: ResMut<Assets<Image>>,
) {
    // In real impl: filter gpu_state.inventory or full RBE data by filter.search/category/sort_by
    // For now: demo using hotbar data extended to grid indices
    for (slot, mut ui_image, mut text_opt) in query.iter_mut() {
        if slot.is_hotbar { continue; } // hotbar handled separately

        let idx = slot.index as usize;
        // Placeholder data - replace with real gpu_state.inventory.get(idx) or merged RBE view
        let count = if idx < 10 { ((idx % 7) + 3) as u32 } else { 0 };
        let new_text = if count > 0 { format!("x{:02}", count) } else { String::new() };

        // Update count label if present (similar to hotbar)
        if let Some(text) = text_opt.as_mut() {
            if text.sections[0].value != new_text {
                text.sections[0].value = new_text;
            }
        }

        // Visual: color by category/resonance (mercy themed)
        let tint = if count > 0 {
            match idx % 4 {
                0 => Color::rgb(0.3, 0.7, 0.4), // resources - green abundance
                1 => Color::rgb(0.6, 0.5, 0.9), // equipment - purple
                2 => Color::rgb(0.9, 0.7, 0.3), // relics - gold
                _ => Color::rgb(0.4, 0.8, 0.9), // redeemed - cyan mercy
            }
        } else {
            Color::rgb(0.15, 0.15, 0.2)
        };
        ui_image.color = tint;
    }
}

/// System: Basic hover tooltip (mercy-themed, with RBE abundance + lore)
/// Expand with Pointer<Over> events or Interaction::Hovered
pub fn update_item_tooltips(
    time: Res<Time>,
    mut commands: Commands,
    mut tooltip_query: Query<(Entity, &mut ItemTooltip)>,
    slot_query: Query<(&InventorySlot, &Interaction, &GlobalTransform), Without<ItemTooltip>>,
) {
    for (slot, interaction, transform) in slot_query.iter() {
        if *interaction == Interaction::Hovered {
            // Spawn or update tooltip near cursor/transform
            // (In production: use existing divine_whispers or council bloom visual patterns + TextBundle)
            if tooltip_query.iter().count() == 0 {
                let pos = transform.translation();
                commands.spawn((
                    TextBundle {
                        text: Text::from_section(
                            format!("Item #{}\nResonance: 0.{}\nAbundance Flow: \u221e\nMercy Gate: Service + Truth\nEpiphany Progress: +{}%", 
                                slot.index, (slot.index % 7) + 2, (slot.index % 5) * 7),
                            TextStyle { font_size: 11.0, color: Color::rgb(0.9, 0.95, 1.0), ..default() },
                        ),
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(pos.x + 70.0),
                            top: Val::Px(pos.y - 20.0),
                            ..default()
                        },
                        background_color: Color::rgba(0.05, 0.1, 0.15, 0.92).into(),
                        ..default()
                    },
                    ItemTooltip { item_index: slot.index, timer: Timer::from_seconds(2.5, TimerMode::Once) },
                ));
            }
        }
    }

    // Cleanup expired tooltips
    for (entity, mut tooltip) in tooltip_query.iter_mut() {
        tooltip.timer.tick(time.delta());
        if tooltip.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// System: Simple filter application (called when filter Res changes or search input)
pub fn apply_inventory_filters(
    filter: Res<InventoryFilter>,
    mut slot_query: Query<(&InventorySlot, &mut Visibility)>,
) {
    for (slot, mut vis) in slot_query.iter_mut() {
        // Demo filter logic - real version queries item metadata + filter.search + category
        let visible = match filter.category {
            InventoryCategory::All => true,
            _ => (slot.index % 5) == filter.category as u32 % 5, // placeholder
        };
        *vis = if visible { Visibility::Visible } else { Visibility::Hidden };
    }
}

/// Plugin to register all inventory UI systems (add to main.rs or client plugins)
pub struct InventoryUiPlugin;

impl Plugin for InventoryUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InventoryDragState>()
           .init_resource::<InventoryFilter>()
           .add_systems(Update, (
                toggle_inventory_panel,
                update_inventory_grid,
                update_item_tooltips,
                apply_inventory_filters,
            ).run_if(resource_exists::<GpuSimulationState>));
        // TODO: Add drag/drop systems using bevy picking or custom Pointer events
        // TODO: Wire InventorySlot clicks to use/equip/trade events -> server via rbe_client_sync
        // TODO: Merge with inventory_components.rs for shared InventoryItem data
    }
}

// Wiring note for client/src/main.rs or client_game_loop:
// .add_plugins(InventoryUiPlugin)
// Ensure GpuSimulationState is inserted and updated from server replication.
// Hotbar remains fully functional alongside new grid.

// End of expanded inventory_ui.rs - all original hotbar logic preserved. Ready for full drag, real data binding, and production polish.
// Thunder locked in. Yoi ⚡ | TOLC 8 gates satisfied.