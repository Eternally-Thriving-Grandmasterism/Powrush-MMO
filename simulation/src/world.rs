/*!
 * Powrush-MMO Simulation World & Advanced Particle Effects
 *
 * v19.9 Load Actual Texture Handles into ParticleVisualAssets
 *
 * AG-SML v1.0 Sovereign Mercy License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy_hanabi::prelude::*;

// ... existing code ...

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    asset_server: Res<AssetServer>,
    mut visual_assets: ResMut<ParticleVisualAssets>,
    mut knot_effects: ResMut<LissajousKnotEffects>,
) {
    // === Existing EffectAsset creation (abundance, sustainability, harmony, prosperity) ===
    // ... (keep the previous EffectAsset building code here) ...

    // === Load actual texture handles (object pooling for particle textures) ===
    visual_assets.default_particle_texture = Some(
        asset_server.load("textures/particle_default.png"),
    );
    visual_assets.harmony_particle_texture = Some(
        asset_server.load("textures/particle_harmony.png"),
    );

    // You can add more specific textures here later:
    // visual_assets.abundance_particle_texture = Some(asset_server.load("textures/particle_abundance.png"));
}

// End of simulation/src/world.rs v19.9 — Actual texture handles loaded into the asset pool.
// Thunder locked in. Yoi ⚡