/*!
 * Shared UI Utilities for Powrush-MMO
 * 
 * Contains reusable helpers for cached text labels with dirty checking.
 * AG-SML v1.0
 */

use bevy::prelude::*;
use crate::CachedLabelImage; // Adjust if CachedLabelImage lives elsewhere

/// Spawns a cached text label with LastRenderedText + LastRenderedColor
/// pre-initialized for perfect first-frame behavior.
pub fn spawn_cached_label(
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
            crate::LastRenderedText {
                text: initial_text.to_string(),
            },
            crate::LastRenderedColor(initial_color),
        ))
        .id()
}

// Future helpers can be added here:
// - spawn_patsagi_metric_label
// - spawn_hotbar_slot
// etc.