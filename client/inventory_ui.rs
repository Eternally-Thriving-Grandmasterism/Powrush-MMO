// client/inventory_ui.rs
// Spawn-time initialization

// In the inventory / PATSAGi HUD spawn function:

commands.spawn((
    // ... existing bundles ...
    GlobalConfidenceText,
    CachedLabelImage(cached_handle),
    LastRenderedText {
        text: String::new(),
    },
    LastRenderedColor([0, 0, 0]),
));

// This guarantees the dirty-checking components exist from frame 1.