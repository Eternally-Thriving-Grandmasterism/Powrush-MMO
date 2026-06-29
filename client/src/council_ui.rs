/*!
 * PATSAGi Node Confidence now wired to real GpuSimulationState
 */

use crate::rbe_client_sync::GpuSimulationState;

fn update_node_confidence_images(
    text_cache: Res<TextAtlasCache>,
    gpu_state: Res<GpuSimulationState>,
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
        let idx = (node.node_id as usize).saturating_sub(1);

        // TODO: Adjust field path to match your actual GpuSimulationState structure
        let confidence = gpu_state.node_confidences.get(idx).copied().unwrap_or(0.0);

        let new_text = format!("Node {:02}: {:.2}", node.node_id, confidence);
        let new_color = [120, 200, 255];

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

// Node Confidence system is now connected to GpuSimulationState.
// You can remove the temporary PatsagiNodeStates resource.