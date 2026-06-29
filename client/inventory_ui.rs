// client/inventory_ui.rs
// Optimized image conversion using the new helper

use crate::engine::ui::{TextAtlasCache, SimpleBitmapFont, update_bevy_image_from_atlas};

// In update_global_confidence_image:
fn update_global_confidence_image(
    text_cache: Res<TextAtlasCache>,
    gpu_state: Res<crate::rbe_client_sync::GpuSimulationState>,
    mut query: Query<(&mut UiImage, &CachedLabelImage), With<GlobalConfidenceText>>,
    mut images: ResMut<Assets<Image>>,
) {
    let font = SimpleBitmapFont::new();

    for (mut ui_image, cached) in query.iter_mut() {
        let conf = gpu_state.global_confidence;
        let text = format!("Global: {:.1}%", conf * 100.0);

        let atlas = text_cache.get_or_render(&font, &text, [77, 242, 140]);

        if let Some(bevy_img) = images.get_mut(&cached.0) {
            update_bevy_image_from_atlas(bevy_img, &atlas);
        } else {
            let new_img = Image::from_dynamic(image::DynamicImage::ImageRgb8(atlas), true);
            ui_image.0 = images.add(new_img);
        }
    }
}

// Inventory HUD now uses the optimized in-place image update path.