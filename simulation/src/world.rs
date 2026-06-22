/*!
 * Powrush-MMO Simulation World & Advanced Particle Effects
 *
 * v19.11 Hanabi Texture Modifiers (PATSAGi + Ra-Thor)
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
    mut images: ResMut<Assets<Image>>,
    mut visual_assets: ResMut<ParticleVisualAssets>,
    mut knot_effects: ResMut<LissajousKnotEffects>,
) {
    // === Create EffectAssets as before ===

    // 3. HarmonyStabilization (main visual)
    let mut harmony = EffectAsset::new(500, Spawner::once(85.0.into(), true), Module::default());
    // ... all the previous modifiers (position, velocity, acceleration, turbulence, size, color) ...

    // === Apply Hanabi Texture Modifier ===
    let harmony_texture = visual_assets.get_texture_or_fallback(visual_assets.harmony_particle_texture.clone());
    harmony.set_particle_texture(harmony_texture);

    let harmony_handle = effects.add(harmony);
    visual_assets.harmony = harmony_handle.clone();
    knot_effects.complex = harmony_handle;

    // You can do the same for other effects:
    // let abundance_texture = visual_assets.get_texture_or_fallback(visual_assets.default_particle_texture.clone());
    // abundance.set_particle_texture(abundance_texture);

    // ... rest of the function (prosperity, etc.) ...
}

// End of simulation/src/world.rs v19.11 — Hanabi Texture Modifiers integrated with fallback.
// Thunder locked in. Yoi ⚡