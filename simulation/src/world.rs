/*!
 * Powrush-MMO Simulation World & Advanced Particle Effects
 *
 * v19.18 Texture Sampler for Animation Curves (PATSAGi + Ra-Thor)
 *
 * AG-SML v1.0 Sovereign Mercy License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy_hanabi::prelude::*;

// ... existing code ...

#[derive(Resource, Default)]
pub struct ParticleVisualAssets {
    pub abundance: Handle<EffectAsset>,
    pub sustainability: Handle<EffectAsset>,
    pub harmony: Handle<EffectAsset>,
    pub prosperity: Handle<EffectAsset>,

    pub default_particle_texture: Option<Handle<Image>>,
    pub harmony_particle_texture: Option<Handle<Image>>,

    // NEW: Texture-based animation curve (1D curve texture)
    pub animation_curve_texture: Option<Handle<Image>>,
    pub fallback_texture: Handle<Image>,
}

impl ParticleVisualAssets {
    pub fn get_texture_or_fallback(&self, preferred: Option<Handle<Image>>) -> Handle<Image> {
        preferred.unwrap_or_else(|| self.fallback_texture.clone())
    }

    /// Returns the animation curve texture or falls back to the default fallback texture.
    pub fn get_animation_curve_texture(&self) -> Handle<Image> {
        self.animation_curve_texture.clone().unwrap_or_else(|| self.fallback_texture.clone())
    }
}

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut visual_assets: ResMut<ParticleVisualAssets>,
    mut knot_effects: ResMut<LissajousKnotEffects>,
) {
    // ... existing EffectAsset creation for harmony, abundance, etc. ...

    // Load animation curve texture (optional but powerful)
    visual_assets.animation_curve_texture = Some(
        asset_server.load("textures/animation_curves.png"),
    );

    // Create fallback if not already created
    // ... existing fallback creation ...
}

// End of simulation/src/world.rs v19.18 — Texture sampler infrastructure for animation curves added.
// Thunder locked in. Yoi ⚡