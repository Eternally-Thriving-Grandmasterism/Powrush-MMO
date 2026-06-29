// client/inventory_ui.rs
// Real usage of spawn_cached_label for Global Confidence

// In the inventory / PATSAGi HUD spawn function:

fn spawn_inventory_hud(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    // ...

    let initial_text = "Global: 94.2%";
    let initial_color = [77, 242, 140];
    let handle = images.add(/* initial image */);

    spawn_cached_label(
        &mut commands,
        initial_text,
        initial_color,
        GlobalConfidenceText,
        CachedLabelImage(handle),
    );
}

// spawn_cached_label is now the standard way to create cached labels in the HUD.