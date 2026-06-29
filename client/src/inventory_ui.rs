/*!
 * Inventory + Hotbar UI - Wiring + Real State Connection
 */

// ==================== TEMPORARY RESOURCE (replace with real one) ====================

#[derive(Resource, Default)]
pub struct HotbarState {
    /// Item counts per slot (index 0..7)
    pub counts: [u32; 8],
}

// ==================== UPDATE SYSTEM (now reads from real state) ====================

fn update_hotbar_item_count_images(
    text_cache: Res<TextAtlasCache>,
    hotbar: Res<HotbarState>,           // <-- Real inventory state
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
        let count = hotbar.counts.get(idx).copied().unwrap_or(0);

        let new_text = format!("x{:02}", count);
        let new_color = [255, 220, 100];

        let text_changed = last_text.text != new_text;
        let color_changed = last_color.0 != new_color;

        if text_changed || color_changed {
            let atlas = text_cache.get_or_render(&font, &new_text, new_color);

            if let Some(bevy_img) = images.get_mut(&cached.0) {
                update_bevy_image_from_atlas(bevy_img, &atlas);
            }

            last_text.text = new_text;
            last_color.0 = new_color;
        }
    }
}

// ==================== PLUGIN WIRING ====================

// Add this in your InventoryPlugin or main app setup:
//
// app
//     .init_resource::<HotbarState>()
//     .add_systems(Update, update_hotbar_item_count_images);
//
// When you have the real inventory resource, replace HotbarState above
// with your actual type and remove this temporary one.