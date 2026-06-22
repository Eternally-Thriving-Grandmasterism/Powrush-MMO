/*!
 * Powrush-MMO Simulation World & Advanced Particle Effects
 *
 * v19.17 Cubic Bezier Easing Helper (PATSAGi + Ra-Thor)
 *
 * AG-SML v1.0 Sovereign Mercy License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy_hanabi::prelude::*;

// ... existing code ...

/// Builds a cubic Bezier easing expression for frame index animation.
///
/// P0 = 0, P3 = 1 (standard easing behavior).
/// p1 and p2 are the X positions of the two control points.
fn cubic_bezier_frame_index(
    age: Expr,
    lifetime: Expr,
    p1: f32,
    p2: f32,
    frame_count: Expr,
) -> Expr {
    let t = age / lifetime;
    let one_minus_t = 1.0_f32 - t;

    let term1 = (3.0_f32 * one_minus_t * one_minus_t * t) * p1.into();
    let term2 = (3.0_f32 * one_minus_t * t * t) * p2.into();
    let term3 = t * t * t;

    let eased = term1 + term2 + term3;
    eased * frame_count
}

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut visual_assets: ResMut<ParticleVisualAssets>,
    mut knot_effects: ResMut<LissajousKnotEffects>,
) {
    // === Harmony Effect with Cubic Bezier Easing ===
    let mut harmony = EffectAsset::new(500, Spawner::once(85.0.into(), true), Module::default());

    // ... existing modifiers ...

    let texture = visual_assets.get_texture_or_fallback(visual_assets.harmony_particle_texture.clone());
    harmony.set_particle_texture(texture);

    harmony.add_modifier(FlipbookModifier {
        columns: 4,
        rows: 4,
        frame_count: 16,
    });

    // === Use the new cubic Bezier helper ===
    // Classic ease-in-out-ish curve (0.42, 0.0, 0.58, 1.0)
    let age = Attribute::PARTICLE_AGE;
    let lifetime = Attribute::PARTICLE_LIFETIME;
    let frame_count = 16.0_f32.into();

    let frame_index_expr = cubic_bezier_frame_index(age, lifetime, 0.42, 0.58, frame_count);

    harmony.add_modifier(SetAttributeModifier::new(
        Attribute::PARTICLE_FRAME_INDEX,
        frame_index_expr,
    ));

    let harmony_handle = effects.add(harmony);
    visual_assets.harmony = harmony_handle.clone();
    knot_effects.complex = harmony_handle;

    // ... other effects ...
}

// End of simulation/src/world.rs v19.17 — Cubic Bezier helper implemented.
// Thunder locked in. Yoi ⚡