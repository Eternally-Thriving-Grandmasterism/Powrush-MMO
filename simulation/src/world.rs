/*!
 * Runtime preset switching for Lissajous knots.
 */

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CurrentLissajousKnotPreset {
    pub preset: LissajousKnotPreset,
}

#[derive(Resource, Default)]
pub struct LissajousKnotEffects {
    pub trefoil: Handle<EffectAsset>,
    pub high_writhe: Handle<EffectAsset>,
    pub symmetric: Handle<EffectAsset>,
    pub complex: Handle<EffectAsset>,
}

// Call this in Startup after creating the individual EffectAssets
pub fn register_lissajous_knot_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut knot_effects: ResMut<LissajousKnotEffects>,
) {
    // In a full implementation, create four separate EffectAssets here
    // using the radial values from each preset.
    // For now we register placeholders that can be filled at runtime.

    // Example: knot_effects.trefoil = effects.add(create_trefoil_effect());
    // ...
}

// System that updates the active particle effect when preset changes
pub fn update_active_lissajous_knot(
    mut commands: Commands,
    current: Res<CurrentLissajousKnotPreset>,
    knot_effects: Res<LissajousKnotEffects>,
    mut query: Query<(Entity, &mut ParticleEffect), With<HarmonyKnotMarker>>,
) {
    let target_handle = match current.preset {
        LissajousKnotPreset::TrefoilLike => knot_effects.trefoil.clone(),
        LissajousKnotPreset::HighWrithe => knot_effects.high_writhe.clone(),
        LissajousKnotPreset::Symmetric => knot_effects.symmetric.clone(),
        LissajousKnotPreset::Complex5_3_4 => knot_effects.complex.clone(),
    };

    for (entity, mut effect) in &mut query {
        if effect.effect != target_handle {
            effect.effect = target_handle.clone();
            // Optional: reset spawner or add visual transition
        }
    }
}

// Marker component to identify the active Harmony knot particle entity
#[derive(Component)]
pub struct HarmonyKnotMarker;
