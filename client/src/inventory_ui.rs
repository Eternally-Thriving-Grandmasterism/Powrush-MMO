/*!
 * client/src/inventory_ui.rs
 * Rarity color coding extended to hotbar rendering.
 */

use bevy::prelude::*;

/// Returns mercy-themed color for rarity (0-5)
pub fn get_rarity_color(rarity: u8) -> Color {
    match rarity {
        0 => Color::rgb(0.6, 0.6, 0.65),   // Common
        1 => Color::rgb(0.4, 0.7, 0.4),   // Uncommon
        2 => Color::rgb(0.4, 0.6, 0.9),   // Rare
        3 => Color::rgb(0.7, 0.5, 0.9),   // Epic
        4 => Color::rgb(0.95, 0.7, 0.3),  // Legendary
        5 => Color::rgb(1.0, 0.4, 0.6),   // Mythic
        _ => Color::rgb(0.5, 0.5, 0.55),
    }
}

// === HOTBAR RARITY COLORING ===

/// Applies rarity color to hotbar slot backgrounds/borders
/// Currently uses placeholder rarity. Will read from extended GpuSimulationState when full item data is wired.
pub fn apply_hotbar_rarity_colors(
    mut query: Query<(&HotbarItemCountText, &mut UiImage)>,
) {
    for (hotbar_slot, mut ui_image) in query.iter_mut() {
        let idx = hotbar_slot.slot_index as usize;

        // Placeholder rarity until we extend GpuSimulationState with full HotbarSlot
        let rarity = (idx % 6) as u8;
        let color = get_rarity_color(rarity);

        // Tint the slot background with rarity color (subtle)
        ui_image.color = color.with_a(0.85);
    }
}

// ... existing hotbar count/cooldown systems and InventoryUiPlugin ...

impl Plugin for InventoryUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            // ... existing hotbar systems ...
            apply_hotbar_rarity_colors,
            // ...
        ));
    }
}

// End of inventory_ui.rs