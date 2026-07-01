/*!
 * client/src/inventory_ui.rs
 * Main hotbar now also uses real HotbarSlot data from ClientHotbar for rarity coloring.
 */

use bevy::prelude::*;
use crate::inventory_replication::ClientHotbar;

// ... existing get_rarity_color ...

/// Main hotbar rarity coloring using real server data
pub fn apply_hotbar_rarity_colors(
    client_hotbar: Res<ClientHotbar>,
    mut query: Query<(&HotbarItemCountText, &mut UiImage)>,
) {
    for (hotbar_slot, mut ui_image) in query.iter_mut() {
        let idx = hotbar_slot.slot_index as usize;

        if idx < client_hotbar.slots.len() {
            let slot = &client_hotbar.slots[idx];
            let base_color = if slot.count > 0 {
                get_rarity_color(slot.rarity)
            } else {
                Color::rgb(0.15, 0.15, 0.2)
            };
            ui_image.color = base_color.with_a(0.85);
        }
    }
}

// Make sure ClientHotbar is initialized
pub struct InventoryUiPlugin;

impl Plugin for InventoryUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClientHotbar>()
           .add_systems(Update, apply_hotbar_rarity_colors);
    }
}

// End of inventory_ui.rs