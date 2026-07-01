/*!
 * client/src/inventory_ui.rs
 * Rarity-aware validation + dynamic drop target highlighting.
 */

use bevy::prelude::*;

// ... existing code ...

/// Result of move validation
#[derive(Clone, Copy)]
pub struct MoveValidity {
    pub allowed: bool,
    pub rarity_level: u8, // for highlighting
}

/// Rarity-aware validation
pub fn validate_move(source: &InventorySlot, target: &InventorySlot) -> MoveValidity {
    // Placeholder: simulate rarity from index until real HotbarSlot data is on slots
    let source_rarity = (source.index % 6) as u8;
    let target_rarity = (target.index % 6) as u8;

    // Example mercy-themed rule:
    // High rarity items (4+) prefer to stay in "worthy" slots or can go anywhere for now
    let allowed = true; // Currently permissive. Add real rules here.

    MoveValidity {
        allowed,
        rarity_level: source_rarity.max(target_rarity),
    }
}

/// Highlights drop targets while dragging based on validity and rarity
pub fn highlight_drop_zones_during_drag(
    drag: Res<InventoryDragState>,
    mut slot_query: Query<(&InventorySlot, &Interaction, &mut UiImage, Entity)>,
) {
    if !drag.is_dragging {
        // Reset any highlighted slots when not dragging
        for (_, _, mut image, _) in slot_query.iter_mut() {
            if image.color.a() > 0.9 { // crude way to detect highlighted state
                image.color = image.color.with_a(0.85);
            }
        }
        return;
    }

    let source = match drag.source {
        Some(s) => s,
        None => return,
    };

    for (slot, interaction, mut image, _entity) in slot_query.iter_mut() {
        if *interaction == Interaction::Hovered {
            let validity = validate_move(&source, slot);

            if validity.allowed {
                // Valid target - highlight with rarity color
                let highlight = get_rarity_color(validity.rarity_level).with_a(0.6);
                image.color = highlight;
            } else {
                // Invalid - warning color
                image.color = Color::rgb(0.9, 0.3, 0.3).with_a(0.5);
            }
        }
    }
}

// Update handle_drop to use the new validation
pub fn handle_drop(
    mut commands: Commands,
    mut drag: ResMut<InventoryDragState>,
    mut demo_inv: ResMut<DemoInventory>,
    mut gpu_state: ResMut<GpuSimulationState>,
    target_query: Query<(&InventorySlot, &Interaction, Entity)>,
) {
    if !drag.is_dragging || drag.source.is_none() { return; }
    let src = drag.source.unwrap();

    for (tgt, interaction, _e) in target_query.iter() {
        if *interaction == Interaction::Pressed || *interaction == Interaction::Hovered {
            if tgt.index == src.index && tgt.is_hotbar == src.is_hotbar {
                cancel_drag(&mut commands, &mut drag);
                return;
            }

            let validity = validate_move(&src, tgt);

            if validity.allowed {
                // Perform move (same logic as before)
                if src.is_hotbar && tgt.is_hotbar {
                    // hotbar swap
                    let s = src.index as usize;
                    let t = tgt.index as usize;
                    if s < 8 && t < 8 {
                        let tmp = gpu_state.hotbar[s].count;
                        gpu_state.hotbar[s].count = gpu_state.hotbar[t].count;
                        gpu_state.hotbar[t].count = tmp;
                    }
                } else {
                    demo_inv.swap(src.index as usize, tgt.index as usize);
                }
                info!("[Inventory] Valid move executed");
            } else {
                warn!("[Inventory] Move rejected by rarity validation");
            }

            cancel_drag(&mut commands, &mut drag);
            return;
        }
    }
    cancel_drag(&mut commands, &mut drag);
}

// Add the new highlighting system to the plugin
pub struct InventoryUiPlugin;

impl Plugin for InventoryUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            // ... other systems
            highlight_drop_zones_during_drag,
            handle_drop,
            // ...
        ));
    }
}

// End of inventory_ui.rs