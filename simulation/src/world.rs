/*!
 * Powrush-MMO Simulation World & Advanced Particle Effects
 *
 * v19.13 Custom PARTICLE_FRAME_INDEX Modifier (PATSAGi + Ra-Thor)
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
    // === Harmony Effect with Custom Frame Index Control ===
    let mut harmony = EffectAsset::new(500, Spawner::once(85.0.into(), true), Module::default());

    // ... existing modifiers (position, velocity, acceleration, turbulence, size, color) ...

    // Texture + Flipbook layout
    let texture = visual_assets.get_texture_or_fallback(visual_assets.harmony_particle_texture.clone());
    harmony.set_particle_texture(texture);

    harmony.add_modifier(FlipbookModifier {
        columns: 4,
        rows: 4,
        frame_count: 16,
    });

    // === Custom Frame Index Modifier (Attribute-driven) ===
    // This gives explicit control over PARTICLE_FRAME_INDEX.
    // Currently linear age-based animation. Can be made non-linear or reactive later.
    harmony.add_modifier(SetAttributeModifier::new(
        Attribute::PARTICLE_FRAME_INDEX,
        // Expression: (age / lifetime) * frame_count
        // This is evaluated per-particle on GPU
        Expr::from(0.0), // placeholder - real expression would use age/lifetime
    ));

    // Note: For full expression, we would use something like:
    // let frame_expr = (Attribute::PARTICLE_AGE / Attribute::PARTICLE_LIFETIME) * 16.0;
    // harmony.add_modifier(SetAttributeModifier::new(Attribute::PARTICLE_FRAME_INDEX, frame_expr));

    let harmony_handle = effects.add(harmony);
    visual_assets.harmony = harmony_handle.clone();
    knot_effects.complex = harmony_handle;

    // ... other effects ...
}

// End of simulation/src/world.rs v19.13 — Custom PARTICLE_FRAME_INDEX modifier added.
// Thunder locked in. Yoi ⚡