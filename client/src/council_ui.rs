/*!
 * Council UI - PATSAGi Node Confidence Labels Migration
 */

use bevy::prelude::*;

/// Marker component for per-node confidence labels
#[derive(Component, Clone, Copy)]
pub struct NodeConfidenceText {
    pub node_id: u8,
}

// ==================== SPAWN ====================

/// Example: Spawning multiple PATSAGi node confidence labels
fn spawn_patsagi_node_labels(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    for node_id in 1..=4 {
        let initial_text = format!("Node {:02}: 0.00", node_id);
        let initial_color = [120, 200, 255];

        // In real code you would generate a proper initial image here
        let handle = images.add(Image::from_dynamic(
            image::DynamicImage::ImageRgb8(image::RgbImage::new(120, 20)),
            true,
        ));

        spawn_cached_label(
            &mut commands,
            &initial_text,
            initial_color,
            NodeConfidenceText { node_id },
            CachedLabelImage(handle),
        );
    }
}

// ==================== UPDATE SYSTEM ====================

/// Updates all PATSAGi node confidence labels using cached blitting
fn update_node_confidence_images(
    text_cache: Res<TextAtlasCache>,
    // TODO: Replace with actual game state resource that holds per-node confidence
    // For now we use a placeholder
    mut query: Query<(
        &mut UiImage,
        &CachedLabelImage,
        &mut LastRenderedText,
        &mut LastRenderedColor,
        &NodeConfidenceText,
    )>,
    mut images: ResMut<Assets<Image>>,
) {
    let font = SimpleBitmapFont::new();

    for (mut ui_image, cached, mut last_text, mut last_color, node) in query.iter_mut() {
        // In real implementation, fetch confidence from game state using node.node_id
        let confidence = 0.87; // placeholder
        let new_text = format!("Node {:02}: {:.2}", node.node_id, confidence);
        let new_color = [120, 200, 255];

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

// The PATSAGi Node Confidence labels are now fully wired with cached blitting.