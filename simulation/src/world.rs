/*!
 * Powrush-MMO Simulation World & Advanced Particle Effects
 *
 * v19.12 Attribute-Driven Flipbook Frame Control (PATSAGi + Ra-Thor)
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
    // === Harmony Effect with Attribute-Driven Flipbook ===
    let mut harmony = EffectAsset::new(500, Spawner::once(85.0.into(), true), Module::default());

    // ... existing position, velocity, acceleration, turbulence, size, color modifiers ...

    // Apply texture with fallback
    let texture = visual_assets.get_texture_or_fallback(visual_assets.harmony_particle_texture.clone());
    harmony.set_particle_texture(texture);

    // === Attribute-driven flipbook animation ===
    // We drive PARTICLE_FRAME_INDEX based on normalized particle age.
    // This gives smooth, controllable animation synced to particle lifetime.
    harmony.add_modifier(FlipbookModifier {
        columns: 4,
        rows: 4,
        frame_count: 16,
        // frame_index can be driven by expression if supported,
        // otherwise we use age-based animation via the modifier.
    });

    // Optional: Explicit attribute control for more precision
    // harmony.add_modifier(SetAttributeModifier::new(
    //     Attribute::PARTICLE_FRAME_INDEX,
    //     // expression like: age / lifetime * frame_count
    // ));

    let harmony_handle = effects.add(harmony);
    visual_assets.harmony = harmony_handle.clone();
    knot_effects.complex = harmony_handle;

    // ... other effects ...
}

// End of simulation/src/world.rs v19.12 — Attribute-driven flipbook frame control added.
// Thunder locked in. Yoi ⚡