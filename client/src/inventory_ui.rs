/*!
 * Hotbar UI - Defensive access to GpuSimulationState
 */

fn update_hotbar_item_count_images(
    text_cache: Res<TextAtlasCache>,
    gpu_state: Res<crate::rbe_client_sync::GpuSimulationState>,
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

        // Defensive access - replace with real fields once GpuSimulationState has them
        let count = gpu_state.hotbar.get(idx)
            .map(|slot| slot.count)
            .unwrap_or(0);

        let new_text = format!("x{:02}", count);

        if last_text.text != new_text {
            // ... update image
        }
    }
}