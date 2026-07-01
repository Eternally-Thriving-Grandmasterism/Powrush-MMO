/*!
 * client/src/inventory_ui.rs
 * Now renders using real HotbarSlot data from ClientHotbar for hotbar slots.
 */

use bevy::prelude::*;
use crate::inventory_replication::{receive_inventory_update, ClientHotbar};

// ... existing get_rarity_color, InventorySlot, etc. ...

pub fn update_inventory_grid(
    client_hotbar: Res<ClientHotbar>,
    gpu_state: Res<GpuSimulationState>,
    mut query: Query<(&InventorySlot, &mut UiImage, Option<&mut Text>)>,
) {
    for (slot, mut ui_image, mut text_opt) in query.iter_mut() {
        let idx = slot.index as usize;

        let (count, rarity) = if slot.is_hotbar && idx < 8 {
            let s = &client_hotbar.slots[idx];
            (s.count, s.rarity)
        } else {
            // Non-hotbar grid slots still use demo data for now
            (((idx % 7) + 2) as u32, (idx % 6) as u8)
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

// Update plugin to init ClientHotbar
pub struct InventoryUiPlugin;

impl Plugin for InventoryUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClientHotbar>()
           // ... other inits and systems
           .add_systems(Update, update_inventory_grid);
    }
}

// End of inventory_ui.rs