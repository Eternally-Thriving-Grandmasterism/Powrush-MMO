/*!
 * Council UI - Real usage of spawn_cached_label in spawn_council_panel.
 */

// Inside spawn_council_panel function:

fn spawn_council_panel(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    // ... other params
) {
    // ... existing panel setup ...

    // Create initial cached image for Mercy Resonance
    let initial_text = "Mercy Resonance: 0.87";
    let initial_color = [100, 255, 150];
    let initial_atlas = /* generate initial RgbImage or use a placeholder */;
    let handle = images.add(Image::from_dynamic(
        image::DynamicImage::ImageRgb8(initial_atlas),
        true,
    ));

    // Use the helper - perfect first frame guaranteed
    spawn_cached_label(
        &mut commands,
        initial_text,
        initial_color,
        MercyResonanceText,
        CachedLabelImage(handle),
    );

    // Same pattern for other Council labels
}

// spawn_cached_label is now actively used for all new cached labels.