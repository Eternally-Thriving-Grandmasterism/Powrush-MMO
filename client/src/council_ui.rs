/*!
 * Council UI - Spawn time initialization of LastRendered* components.
 */

// In spawn_council_panel (or wherever MercyResonanceText is created):

fn spawn_council_panel(...) {
    // ... existing code ...

    commands.spawn((
        // ... existing UI bundles ...
        MercyResonanceText,
        CachedLabelImage(cached_handle),
        LastRenderedText {
            text: String::new(),   // Will be updated on first system run
        },
        LastRenderedColor([0, 0, 0]),
    ));

    // Same pattern for other cached labels
}

// This ensures the components always exist after spawn,
// making dirty checking logic simpler and more predictable.