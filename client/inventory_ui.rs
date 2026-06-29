// client/inventory_ui.rs
// LastRenderedText dirty checking for Global Confidence

use crate::engine::ui::{TextAtlasCache, SimpleBitmapFont, update_bevy_image_from_atlas};

#[derive(Component, Clone)]
struct LastRenderedText {
    text: String,
    color: [u8; 3],
}

fn update_global_confidence_image(
    text_cache: Res<TextAtlasCache>,
    gpu_state: Res<crate::rbe_client_sync::GpuSimulationState>,
    mut query: Query<(
        &mut UiImage,
        &CachedLabelImage,
        Option<&mut LastRenderedText>,
    ), With<GlobalConfidenceText>>,
    mut images: ResMut<Assets<Image>>,
) {
    let font = SimpleBitmapFont::new();

    for (mut ui_image, cached, mut last_rendered) in query.iter_mut() {
        let conf = gpu_state.global_confidence;
        let new_text = format!("Global: {:.1}%", conf * 100.0);
        let new_color = [77, 242, 140];

        let needs_update = match last_rendered.as_ref() {
            Some(last) => last.text != new_text || last.color != new_color,
            None => true,
        };

        if needs_update {
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

            if let Some(last) = last_rendered.as_mut() {
                last.text = new_text;
                last.color = new_color;
            }
        }
    }
}

// Inventory HUD now has full LastRenderedText dirty checking.