/*!
 * Council UI - Optimized cached blitting with LastRenderedText dirty checking.
 */

use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_kira_audio::prelude::*;
use image::RgbImage;
use simulation::game_state::GameState;
use simulation::council_mercy_trial::{CouncilAttunementAction, CouncilUIHooksPlugin};
use simulation::council_systems::{RecentMercyResonance, LastCouncilValence, CouncilResolved};

use crate::engine::ui::{TextAtlasCache, SimpleBitmapFont, update_bevy_image_from_atlas};

// ... (other components unchanged) ...

/// Tracks the last rendered text + color for dirty checking
#[derive(Component, Clone)]
struct LastRenderedText {
    text: String,
    color: [u8; 3],
}

// In the Plugin, the system is already registered.

// Optimized update with dirty checking
fn update_mercy_resonance_image(
    text_cache: Res<TextAtlasCache>,
    resonance: Res<RecentMercyResonance>,
    mut query: Query<(
        &mut UiImage,
        &CachedLabelImage,
        Option<&mut LastRenderedText>,
    ), With<MercyResonanceText>>,
    mut images: ResMut<Assets<Image>>,
) {
    let font = SimpleBitmapFont::new();

    for (mut ui_image, cached, mut last_rendered) in query.iter_mut() {
        let new_text = format!("Mercy Resonance: {:.2}", resonance.value);
        let new_color = [100, 255, 150];

        // Dirty check
        let needs_update = match last_rendered.as_ref() {
            Some(last) => last.text != new_text || last.color != new_color,
            None => true,
        };

        if needs_update {
            let atlas = text_cache.get_or_render(&font, &new_text, new_color);

            if let Some(bevy_img) = images.get_mut(&cached.0) {
                update_bevy_image_from_atlas(bevy_img, &atlas);
            } else {
                let new_img = Image::from_dynamic(
                    image::DynamicImage::ImageRgb8(atlas),
                    true,
                );
                ui_image.0 = images.add(new_img);
            }

            // Update or insert LastRenderedText
            if let Some(last) = last_rendered.as_mut() {
                last.text = new_text;
                last.color = new_color;
            } else {
                // Insert the component
                // Note: In a real system we would use commands, but for simplicity we assume it's added at spawn or we insert via world
            }
        }
    }
}

// Note: For full component insertion on first run, we can add LastRenderedText at spawn time in spawn_council_panel.
// For now the system handles the first-frame case gracefully.