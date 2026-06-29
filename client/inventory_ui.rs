// client/inventory_ui.rs
// Option wrappers removed

fn update_global_confidence_image(
    text_cache: Res<TextAtlasCache>,
    gpu_state: Res<crate::rbe_client_sync::GpuSimulationState>,
    mut query: Query<(
        &mut UiImage,
        &CachedLabelImage,
        &mut LastRenderedText,
        &mut LastRenderedColor,
    ), With<GlobalConfidenceText>>,
    mut images: ResMut<Assets<Image>>,
) {
    let font = SimpleBitmapFont::new();

    for (mut ui_image, cached, mut last_text, mut last_color) in query.iter_mut() {
        let conf = gpu_state.global_confidence;
        let new_text = format!("Global: {:.1}%", conf * 100.0);
        let new_color = [77, 242, 140];

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

// Clean, direct dirty checking with no Option handling.