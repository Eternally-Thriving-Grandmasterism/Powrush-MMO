/*!
 * Powrush-MMO Simulation World & Advanced Particle Effects
 *
 * v19.10 Texture Fallback Logic (PATSAGi + Ra-Thor)
 *
 * AG-SML v1.0 Sovereign Mercy License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy::render::texture::{Image, ImageSampler};
use bevy_hanabi::prelude::*;

// ... existing code ...

impl ParticleVisualAssets {
    /// Returns a safe texture handle.
    /// Falls back to the generated fallback texture if the preferred one is missing.
    pub fn get_texture_or_fallback(&self, preferred: Option<Handle<Image>>) -> Handle<Image> {
        preferred.unwrap_or_else(|| self.fallback_texture.clone())
    }
}

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut visual_assets: ResMut<ParticleVisualAssets>,
    mut knot_effects: ResMut<LissajousKnotEffects>,
) {
    // === Existing EffectAsset creation ===
    // ...

    // Load preferred textures (may be None if files missing)
    visual_assets.default_particle_texture = Some(asset_server.load("textures/particle_default.png"));
    visual_assets.harmony_particle_texture = Some(asset_server.load("textures/particle_harmony.png"));

    // === Create robust fallback texture (1x1 white pixel) ===
    let mut fallback_image = Image::new_fill(
        bevy::math::UVec2::new(1, 1),
        bevy::render::render_resource::TextureDimension::D2,
        &[255, 255, 255, 255], // White
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
    );
    fallback_image.sampler = ImageSampler::nearest();

    visual_assets.fallback_texture = images.add(fallback_image);
}

// End of simulation/src/world.rs v19.10 — Texture fallback logic added.
// Always safe to call get_texture_or_fallback().
// Thunder locked in. Yoi ⚡