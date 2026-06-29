// client/inventory_ui.rs
// LastRenderedText + LastRenderedColor dirty checking

#[derive(Component, Clone)]
struct LastRenderedText {
    text: String,
}

#[derive(Component, Clone)]
struct LastRenderedColor(pub [u8; 3]);

fn update_global_confidence_image(
    text_cache: Res<TextAtlasCache>,
    gpu_state: Res<crate::rbe_client_sync::GpuSimulationState>,
    mut query: Query<(
        &mut UiImage,
        &CachedLabelImage,
        Option<&mut LastRenderedText>,
        Option<&mut LastRenderedColor>,
    ), With<GlobalConfidenceText>>,
    mut images: ResMut<Assets<Image>>,
) {
    let font = SimpleBitmapFont::new();

    for (mut ui_image, cached, mut last_text, mut last_color) in query.iter_mut() {
        let conf = gpu_state.global_confidence;
        let new_text = format!("Global: {:.1}%", conf * 100.0);
        let new_color = [77, 242, 140];

        let text_changed = match last_text.as_ref() {
            Some(last) => last.text != new_text,
            None => true,
        };

        let color_changed = match last_color.as_ref() {
            Some(last) => last.0 != new_color,
            None => true,
        };

        if text_changed || color_changed {
            let atlas = text_cache.get_or_render(&font, &new_text, new_color);

            if let Some(bevy_img) = images.get_mut(&cached.0) {
                update_bevy_image_from_atlas(bevy_img, &atlas);
            } else {
                let new_img = Image::from_dynamic(
                    image::DynamicImage::ImageRgb8(atlas),
                    true,
                );
                ui_image.0 = images.add(new_img);
            }

            if let Some(last) = last_text.as_mut() {
                last.text = new_text;
            }
            if let Some(last) = last_color.as_mut() {
                last.0 = new_color;
            }
        }
    }
}

// LastRenderedColor implemented for independent color dirty checking in HUD.