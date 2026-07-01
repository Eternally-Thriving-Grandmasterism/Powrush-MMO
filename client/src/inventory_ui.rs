/*!
 * client/src/inventory_ui.rs
 * Full 40-slot grid now renders real data from ClientInventory + ClientHotbar.
 */

use bevy::prelude::*;
use crate::inventory_replication::{ClientHotbar, ClientInventory};

pub fn update_inventory_grid(
    client_hotbar: Res<ClientHotbar>,
    client_inventory: Res<ClientInventory>,
    mut query: Query<(&InventorySlot, &mut UiImage, Option<&mut Text>)>,
) {
    for (slot, mut ui_image, mut text_opt) in query.iter_mut() {
        let idx = slot.index as usize;

        let (count, rarity) = if slot.is_hotbar && idx < 8 {
            let s = &client_hotbar.slots[idx];
            (s.count, s.rarity)
        } else if idx < 40 {
            // Non-hotbar general inventory slots
            let s = &client_inventory.slots[idx];
            (s.count, s.rarity)
        } else {
            (0, 0)
        };

        let color = if count > 0 {
            get_rarity_color(rarity)
        } else {
            Color::rgb(0.15, 0.15, 0.2)
        };

        ui_image.color = color;

        if let Some(text) = text_opt.as_mut() {
            text.sections[0].value = if count > 0 { format!("x{:02}", count) } else { String::new() };
        }
    }
}

// Plugin registration
pub struct InventoryUiPlugin;

impl Plugin for InventoryUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClientHotbar>()
           .init_resource::<ClientInventory>()
           .add_systems(Update, update_inventory_grid);
    }
}

// End of inventory_ui.rs