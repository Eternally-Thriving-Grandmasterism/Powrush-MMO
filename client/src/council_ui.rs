/*!
 * Council UI - LastRenderedText + LastRenderedColor for fine-grained dirty checking.
 */

use bevy::prelude::*;

// Existing LastRenderedText
#[derive(Component, Clone)]
struct LastRenderedText {
    text: String,
}

// New: Separate color tracking
#[derive(Component, Clone)]
struct LastRenderedColor(pub [u8; 3]);

fn update_mercy_resonance_image(
    text_cache: Res<TextAtlasCache>,
    resonance: Res<RecentMercyResonance>,
    mut query: Query<(
        &mut UiImage,
        &CachedLabelImage,
        Option<&mut LastRenderedText>,
        Option<&mut LastRenderedColor>,
    ), With<MercyResonanceText>>,
    mut images: ResMut<Assets<Image>>,
) {
    let font = SimpleBitmapFont::new();

    for (mut ui_image, cached, mut last_text, mut last_color) in query.iter_mut() {
        let new_text = format!("Mercy Resonance: {:.2}", resonance.value);
        let new_color = [100, 255, 150];

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

            // Update trackers
            if let Some(last) = last_text.as_mut() {
                last.text = new_text;
            } else {
                // In production we would insert via commands on first run
            }

            if let Some(last) = last_color.as_mut() {
                last.0 = new_color;
            }
        }
    }
}

// LastRenderedColor implemented for independent color change detection.