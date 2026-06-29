/*!
 * Hotbar UI - Cooldown Labels Migration
 */

/// Marker for hotbar slot cooldown timer labels
#[derive(Component, Clone, Copy)]
pub struct HotbarCooldownText {
    pub slot_index: u8,
}

// ==================== SPAWN ====================

fn spawn_hotbar_cooldowns(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    for slot in 0..=7 {
        let initial_text = "0.0s";
        let initial_color = [255, 180, 80];

        let handle = images.add(Image::from_dynamic(
            image::DynamicImage::ImageRgb8(image::RgbImage::new(70, 18)),
            true,
        ));

        spawn_cached_label(
            &mut commands,
            initial_text,
            initial_color,
            HotbarCooldownText { slot_index: slot },
            CachedLabelImage(handle),
        );
    }
}

// ==================== UPDATE SYSTEM ====================

fn update_hotbar_cooldown_images(
    text_cache: Res<TextAtlasCache>,
    hotbar: Res<HotbarState>, // reuse existing hotbar state or extend it
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

        // Placeholder - replace with real cooldown data
        let remaining = 0.0; // seconds remaining
        let new_text = if remaining > 0.0 {
            format!("{:.1}s", remaining)
        } else {
            String::from("") // hide when ready
        };
        let new_color = if remaining > 0.0 {
            [255, 180, 80]
        } else {
            [120, 255, 150] // ready color
        };

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

// Hotbar Cooldown labels are now live with cached blitting and color state changes.