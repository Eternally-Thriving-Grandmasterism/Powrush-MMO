/*!
 * Inventory + Hotbar UI - Hotbar Item Count Migration
 */

use bevy::prelude::*;
use crate::ui_utils::spawn_cached_label;

/// Marker for hotbar slot item count labels
#[derive(Component, Clone, Copy)]
pub struct HotbarItemCountText {
    pub slot_index: u8,
}

// ==================== SPAWN ====================

fn spawn_hotbar_item_counts(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    for slot in 0..=7 {
        let initial_text = "x00";
        let initial_color = [255, 220, 100];

        let handle = images.add(Image::from_dynamic(
            image::DynamicImage::ImageRgb8(image::RgbImage::new(60, 18)),
            true,
        ));

        spawn_cached_label(
            &mut commands,
            initial_text,
            initial_color,
            HotbarItemCountText { slot_index: slot },
            CachedLabelImage(handle),
        );
    }
}

// ==================== UPDATE SYSTEM ====================

fn update_hotbar_item_count_images(
    text_cache: Res<TextAtlasCache>,
    // TODO: Connect to real inventory/hotbar state
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

    for (mut ui_image, cached, mut last_text, mut last_color, hotbar) in query.iter_mut() {
        // Placeholder - replace with real inventory data
        let count = 12;
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

// Hotbar Item Count labels are now using the full cached blitting pipeline.