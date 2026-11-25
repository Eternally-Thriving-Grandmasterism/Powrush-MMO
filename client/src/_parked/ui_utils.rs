/*!
 * Shared UI Utilities
 */

// ==================== MIGRATION GUIDE FOR NEXT LABELS ====================
//
// To migrate a new label (e.g. PATSAGi node metric, hotbar item, timer):
//
// 1. Create marker component if needed:
//    #[derive(Component)] struct NodeMetricText;
//
// 2. In your spawn function:
//    spawn_cached_label(
//        &mut commands,
//        "Node 03: 0.94",
//        [120, 200, 255],
//        NodeMetricText,
//        CachedLabelImage(handle),
//    );
//
// 3. Add an update system that queries for:
//    Query<(&mut UiImage, &CachedLabelImage, &mut LastRenderedText, &mut LastRenderedColor), With<NodeMetricText>>
//
// This pattern now scales cleanly across the entire game UI.
// =================================================================