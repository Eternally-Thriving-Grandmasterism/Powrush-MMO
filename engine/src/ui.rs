// ... (previous content of engine/src/ui.rs unchanged up to the draw call example section) ...

// ==================== TEXTATLAS CACHE DRAW CALL USAGE ====================

/// Example: Full draw call usage with TextAtlasCache
pub fn example_text_atlas_draw_calls() {
    // ... (example unchanged) ...
}

// ==================== OPTIMIZED BEVY IMAGE CONVERSION ====================

/// Efficiently updates an existing Bevy `Image` from a cached `RgbImage` atlas.
/// 
/// - Fast path: If dimensions match, copies raw pixel data in place (no allocation).
/// - Slow path: Recreates the Bevy Image only when size changes.
/// 
/// Use this in UI systems to avoid recreating textures every frame.
pub fn update_bevy_image_from_atlas(target: &mut Image, atlas: &RgbImage) {
    let atlas_width = atlas.width();
    let atlas_height = atlas.height();

    if target.width() != atlas_width || target.height() != atlas_height {
        // Size changed — recreate
        *target = Image::from_dynamic(
            image::DynamicImage::ImageRgb8(atlas.clone()),
            true,
        );
    } else if let Some(data) = target.data.as_mut() {
        // Fast path: in-place pixel copy
        let raw = atlas.as_raw();
        if data.len() == raw.len() {
            data.copy_from_slice(raw);
        }
    }
}

// Example usage (recommended for UI text):
// let cache = TextAtlasCache::with_pixel_weigher(2048);