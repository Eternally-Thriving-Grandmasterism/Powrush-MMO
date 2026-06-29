/*!
 * Council UI - spawn_cached_label helper with perfect first-frame initialization.
 */

use bevy::prelude::*;

/// Spawns a cached text label with LastRenderedText + LastRenderedColor
/// already initialized to the correct first-frame values.
/// This eliminates any wasted work on frame 1.
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

// Example usage in spawn_council_panel:
// let handle = images.add(...);
// spawn_cached_label(
//     &mut commands,
//     "Mercy Resonance: 0.87",
//     [100, 255, 150],
//     MercyResonanceText,
//     CachedLabelImage(handle),
// );