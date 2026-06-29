/*!
 * Hotbar systems now wired to real GpuSimulationState
 */

use crate::rbe_client_sync::GpuSimulationState;

fn update_hotbar_item_count_images(
    text_cache: Res<TextAtlasCache>,
    gpu_state: Res<GpuSimulationState>,
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

        // TODO: Adjust field path to match your actual GpuSimulationState
        let count = gpu_state.hotbar_counts.get(idx).copied().unwrap_or(0);

        let new_text = format!("x{:02}", count);
        let new_color = [255, 220, 100];

        if last_text.text != new_text || last_color.0 != new_color {
            let atlas = text_cache.get_or_render(&font, &new_text, new_color);

            if let Some(bevy_img) = images.get_mut(&cached.0) {
                update_bevy_image_from_atlas(bevy_img, &atlas);
            }

            last_text.text = new_text;
            last_color.0 = new_color;
        }
    }
}

fn update_hotbar_cooldown_images(
    text_cache: Res<TextAtlasCache>,
    gpu_state: Res<GpuSimulationState>,
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

        // TODO: Adjust field path to match your actual GpuSimulationState
        let remaining = gpu_state.hotbar_cooldowns.get(idx).copied().unwrap_or(0.0);

        let new_text = if remaining > 0.0 {
            format!("{:.1}s", remaining)
        } else {
            String::from("")
        };
        let new_color = if remaining > 0.0 {
            [255, 180, 80]
        } else {
            [120, 255, 150]
        };

        if last_text.text != new_text || last_color.0 != new_color {
            let atlas = text_cache.get_or_render(&font, &new_text, new_color);

            if let Some(bevy_img) = images.get_mut(&cached.0) {
                update_bevy_image_from_atlas(bevy_img, &atlas);
            }

            last_text.text = new_text;
            last_color.0 = new_color;
        }
    }
}

// Hotbar systems are now connected to GpuSimulationState.
// Remove the temporary HotbarState resource if no longer used.