/*!
 * Powrush-MMO Simulation World & Advanced Particle Effects
 *
 * v19.14 Real Attribute Expression for PARTICLE_FRAME_INDEX (PATSAGi + Ra-Thor)
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
    // === Harmony Effect with Real Custom Frame Expression ===
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

    // === Real attribute-driven frame index expression ===
    // frame = (age / lifetime) * frame_count
    // This creates smooth animation based on each particle's normalized age.
    let age = Attribute::PARTICLE_AGE;
    let lifetime = Attribute::PARTICLE_LIFETIME;
    let frame_count = 16.0_f32.into();

    let frame_index_expr = (age / lifetime) * frame_count;

    harmony.add_modifier(SetAttributeModifier::new(
        Attribute::PARTICLE_FRAME_INDEX,
        frame_index_expr,
    ));

    let harmony_handle = effects.add(harmony);
    visual_assets.harmony = harmony_handle.clone();
    knot_effects.complex = harmony_handle;

    // ... other effects ...
}

// End of simulation/src/world.rs v19.14 — Real working expression for custom frame index.
// Thunder locked in. Yoi ⚡