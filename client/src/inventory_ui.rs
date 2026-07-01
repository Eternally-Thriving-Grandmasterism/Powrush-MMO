/*!
 * client/src/inventory_ui.rs
 * Added item rarity color coding (0-5) with mercy-themed palette.
 */

use bevy::prelude::*;
use crate::inventory_replication::receive_inventory_update;
use crate::rbe_client_sync::GpuSimulationState;

/// Returns a mercy-themed color for item rarity (0 = common → 5 = mythic)
pub fn get_rarity_color(rarity: u8) -> Color {
    match rarity {
        0 => Color::rgb(0.6, 0.6, 0.65),      // Common - muted silver
        1 => Color::rgb(0.4, 0.7, 0.4),      // Uncommon - green abundance
        2 => Color::rgb(0.4, 0.6, 0.9),      // Rare - blue resonance
        3 => Color::rgb(0.7, 0.5, 0.9),      // Epic - purple
        4 => Color::rgb(0.95, 0.7, 0.3),     // Legendary - gold
        5 => Color::rgb(1.0, 0.4, 0.6),      // Mythic - divine pink/crimson
        _ => Color::rgb(0.5, 0.5, 0.55),
    }
}

// ... existing code (hotbar functions, InventorySlot, etc.) ...

/// Enhanced grid rendering with rarity color coding
pub fn update_inventory_grid(
    gpu_state: Res<GpuSimulationState>,
    mut query: Query<(&InventorySlot, &mut UiImage, Option<&mut Text>)>,
) {
    for (slot, mut ui_image, mut text_opt) in query.iter_mut() {
        let idx = slot.index as usize;

        // For now demo with placeholder rarity. Real data will come from synced HotbarSlot
        let rarity = (idx % 6) as u8; // placeholder until full item data is wired
        let base_color = get_rarity_color(rarity);

        let count = if slot.is_hotbar && idx < 8 {
            gpu_state.hotbar.get(idx).map(|s| s.count).unwrap_or(0)
        } else {
            ((idx % 7) + 2) as u32
        };

        if let Some(text) = text_opt.as_mut() {
            text.sections[0].value = if count > 0 { format!("x{:02}", count) } else { String::new() };
        }

        ui_image.color = if count > 0 { base_color } else { Color::rgb(0.15, 0.15, 0.2) };
    }
}

// Plugin and other systems remain unchanged
pub struct InventoryUiPlugin;

impl Plugin for InventoryUiPlugin {
    fn build(&self, app: &mut App) {
        // ...
        app.add_systems(Update, update_inventory_grid /* ... */);
    }
}

// End of inventory_ui.rs