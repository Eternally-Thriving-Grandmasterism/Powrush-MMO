// client/inventory_ui.rs
// spawn_cached_label helper for PATSAGi / Inventory HUD

use bevy::prelude::*;

/// Spawns a cached text label with proper first-frame LastRendered* initialization.
fn spawn_cached_label(
    commands: &mut Commands,
    initial_text: &str,
    initial_color: [u8; 3],
    marker: impl Component,
    cached_image: CachedLabelImage,
) -> Entity {
    commands
        .spawn((
            marker,
            cached_image,
            LastRenderedText {
                text: initial_text.to_string(),
            },
            LastRenderedColor(initial_color),
        ))
        .id()
}

// Usage example for Global Confidence:
// spawn_cached_label(
//     &mut commands,
//     "Global: 94.2%",
//     [77, 242, 140],
//     GlobalConfidenceText,
//     CachedLabelImage(handle),
// );