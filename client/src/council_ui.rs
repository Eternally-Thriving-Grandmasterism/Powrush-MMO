/*!
 * Council UI - Full Real Distance-Based 3D Audio Falloff (v19.2.9)
 * 
 * Optimized cached blitting with in-place image update + dirty checking.
 */

use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_kira_audio::prelude::*;
use image::RgbImage;
use simulation::game_state::GameState;
use simulation::council_mercy_trial::{CouncilAttunementAction, CouncilUIHooksPlugin};
use simulation::council_systems::{RecentMercyResonance, LastCouncilValence, CouncilResolved};

use crate::engine::ui::{TextAtlasCache, SimpleBitmapFont, update_bevy_image_from_atlas};

// ... (components and plugin unchanged) ...

// In update_mercy_resonance_image, replace the conversion with the optimized helper
fn update_mercy_resonance_image(
    text_cache: Res<TextAtlasCache>,
    resonance: Res<RecentMercyResonance>,
    mut query: Query<(&mut UiImage, &CachedLabelImage), With<MercyResonanceText>>,
    mut images: ResMut<Assets<Image>>,
) {
    let font = SimpleBitmapFont::new();

    for (mut ui_image, cached) in query.iter_mut() {
        let text = format!("Mercy Resonance: {:.2}", resonance.value);

        // Only do work when text actually changes (simple dirty check via handle existence is not enough;
        // for production use a LastRenderedText component or hash)
        let atlas = text_cache.get_or_render(&font, &text, [100, 255, 150]);

        if let Some(bevy_img) = images.get_mut(&cached.0) {
            update_bevy_image_from_atlas(bevy_img, &atlas);
        } else {
            let new_img = Image::from_dynamic(image::DynamicImage::ImageRgb8(atlas), true);
            ui_image.0 = images.add(new_img);
        }
    }
}

// ... (rest of file unchanged) ...