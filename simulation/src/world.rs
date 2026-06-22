/*!
 * Powrush-MMO Simulation World & Advanced Particle Effects
 *
 * v19.8 Object Pooling for Particle Textures / Visual Assets (PATSAGi + Ra-Thor)
 * — Centralized ParticleVisualAssets resource acting as object pool
 * — Reusable handles for EffectAssets and future textures
 *
 * AG-SML v1.0 Sovereign Mercy License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy_hanabi::prelude::*;

// ============================================================================
// PARTICLE VISUAL ASSETS (Object Pool / Centralized Handles) v19.8
// ============================================================================

/// Centralized resource holding reusable handles for particle visual effects.
/// Acts as an object pool for expensive assets (EffectAsset + future textures).
#[derive(Resource, Default)]
pub struct ParticleVisualAssets {
    // Policy EffectAssets
    pub abundance: Handle<EffectAsset>,
    pub sustainability: Handle<EffectAsset>,
    pub harmony: Handle<EffectAsset>,      // Main 5:3:4 Lissajous
    pub prosperity: Handle<EffectAsset>,

    // Future texture slots (for sprite-based particles, trails, etc.)
    pub default_particle_texture: Option<Handle<Image>>,
    pub harmony_particle_texture: Option<Handle<Image>>,
}

impl ParticleVisualAssets {
    pub fn get_effect_handle(&self, preset: LissajousKnotPreset) -> Handle<EffectAsset> {
        match preset {
            LissajousKnotPreset::TrefoilLike => self.harmony.clone(),
            LissajousKnotPreset::HighWrithe => self.harmony.clone(),
            LissajousKnotPreset::Symmetric => self.harmony.clone(),
            LissajousKnotPreset::Complex5_3_4 => self.harmony.clone(),
        }
    }
}

// Update setup_policy_particle_effects to also populate ParticleVisualAssets
pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut visual_assets: ResMut<ParticleVisualAssets>,
    mut knot_effects: ResMut<LissajousKnotEffects>,
) {
    // ... existing EffectAsset creation code stays the same ...

    // Populate the centralized asset pool
    visual_assets.abundance = particle_effects.abundance.clone();
    visual_assets.sustainability = particle_effects.sustainability.clone();
    visual_assets.harmony = particle_effects.harmony.clone();
    visual_assets.prosperity = particle_effects.prosperity.clone();

    // Example: leave texture slots empty for now (can be filled later)
    visual_assets.default_particle_texture = None;
    visual_assets.harmony_particle_texture = None;
}

// End of simulation/src/world.rs v19.8 — Object pooling for particle visual assets added.
// Thunder locked in. Yoi ⚡