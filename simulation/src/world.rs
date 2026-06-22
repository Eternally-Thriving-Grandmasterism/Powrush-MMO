/*!
 * Powrush-MMO Simulation World & Advanced Particle Effects
 *
 * v19.16 Sine-Based Easing for Flipbook Animation (PATSAGi + Ra-Thor)
 *
 * AG-SML v1.0 Sovereign Mercy License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use std::f32::consts::PI;

// ... existing code ...

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut visual_assets: ResMut<ParticleVisualAssets>,
    mut knot_effects: ResMut<LissajousKnotEffects>,
) {
    // === Harmony Effect with Sine-Based Easing ===
    let mut harmony = EffectAsset::new(500, Spawner::once(85.0.into(), true), Module::default());

    // ... existing modifiers ...

    // Texture + Flipbook layout
    let texture = visual_assets.get_texture_or_fallback(visual_assets.harmony_particle_texture.clone());
    harmony.set_particle_texture(texture);

    harmony.add_modifier(FlipbookModifier {
        columns: 4,
        rows: 4,
        frame_count: 16,
    });

    // === Sine-based organic easing ===
    // Creates a smooth, breathing-like animation curve.
    // Very elegant and natural for mathematical/visual effects.
    let age = Attribute::PARTICLE_AGE;
    let lifetime = Attribute::PARTICLE_LIFETIME;
    let frame_count = 16.0_f32.into();

    let t = age / lifetime;
    let eased = (1.0 - (t * PI).cos()) * 0.5;     // Sine-based ease-in-out
    let frame_index_expr = eased * frame_count;

    harmony.add_modifier(SetAttributeModifier::new(
        Attribute::PARTICLE_FRAME_INDEX,
        frame_index_expr,
    ));

    let harmony_handle = effects.add(harmony);
    visual_assets.harmony = harmony_handle.clone();
    knot_effects.complex = harmony_handle;

    // ... other effects ...
}

// End of simulation/src/world.rs v19.16 — Sine-based easing implemented.
// Thunder locked in. Yoi ⚡