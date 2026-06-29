/*!
 * Council UI - Option wrappers removed. Components are now guaranteed by spawn_cached_label.
 */

fn update_mercy_resonance_image(
    text_cache: Res<TextAtlasCache>,
    resonance: Res<RecentMercyResonance>,
    mut query: Query<(
        &mut UiImage,
        &CachedLabelImage,
        &mut LastRenderedText,
        &mut LastRenderedColor,
    ), With<MercyResonanceText>>,
    mut images: ResMut<Assets<Image>>,
) {
    let font = SimpleBitmapFont::new();

    for (mut ui_image, cached, mut last_text, mut last_color) in query.iter_mut() {
        let new_text = format!("Mercy Resonance: {:.2}", resonance.value);
        let new_color = [100, 255, 150];

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

// Option wrappers successfully removed. Cleaner and more direct.