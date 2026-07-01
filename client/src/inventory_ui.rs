/*!
 * client/src/inventory_ui.rs
 * Visual sync flash effect on hotbar when updated from server.
 */

use bevy::prelude::*;
use crate::inventory_replication::{ClientHotbar, HotbarSyncFlash};

/// Applies rarity colors + sync flash overlay to main hotbar
pub fn apply_hotbar_rarity_colors(
    client_hotbar: Res<ClientHotbar>,
    mut flash: ResMut<HotbarSyncFlash>,
    time: Res<Time>,
    mut query: Query<(&HotbarItemCountText, &mut UiImage)>,
) {
    flash.timer.tick(time.delta());

    let flash_active = !flash.timer.finished();
    let flash_color = Color::rgba(0.4, 0.9, 1.0, 0.35); // Mercy cyan flash

    for (hotbar_slot, mut ui_image) in query.iter_mut() {
        let idx = hotbar_slot.slot_index as usize;

        if idx < client_hotbar.slots.len() {
            let slot = &client_hotbar.slots[idx];
            let base = if slot.count > 0 {
                get_rarity_color(slot.rarity)
            } else {
                Color::rgb(0.15, 0.15, 0.2)
            };

            if flash_active {
                // Blend base rarity color with flash
                ui_image.color = base.mix(&flash_color, 0.6);
            } else {
                ui_image.color = base.with_a(0.85);
            }
        }
    }
}

// Make sure HotbarSyncFlash is initialized
pub struct InventoryUiPlugin;

impl Plugin for InventoryUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClientHotbar>()
           .init_resource::<HotbarSyncFlash>()
           .add_systems(Update, apply_hotbar_rarity_colors);
    }
}

// End of inventory_ui.rs