/*!
 * Council UI - System wiring + real game state connection
 */

// ==================== RESOURCE (replace with your actual one) ====================

/// Temporary resource until connected to the real PATSAGi simulation state.
/// Replace this with your actual resource (e.g. PatsagiState, NodeStates, etc.)
#[derive(Resource, Default)]
pub struct PatsagiNodeStates {
    pub confidences: [f32; 8], // node_id 1..=8
}

// ==================== UPDATE SYSTEM (now uses real state) ====================

fn update_node_confidence_images(
    text_cache: Res<TextAtlasCache>,
    node_states: Res<PatsagiNodeStates>,   // <-- Real game state
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
        let node_idx = (node.node_id as usize).saturating_sub(1);
        let confidence = node_states.confidences.get(node_idx).copied().unwrap_or(0.0);

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

// ==================== PLUGIN WIRING ====================

// In your CouncilUIHooksPlugin or main app setup, add:
//
// app
//     .init_resource::<PatsagiNodeStates>()
//     .add_systems(Update, update_node_confidence_images);
//
// When you have the real resource (e.g. from simulation crate), just replace
// PatsagiNodeStates with your actual type and remove the temporary one above.