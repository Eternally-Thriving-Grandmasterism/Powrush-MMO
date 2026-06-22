/*!
 * Swirling orbit behavior for HarmonyStabilization effect.
 */

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_effects: ResMut<PolicyParticleEffects>,
) {
    // ... (Abundance, Sustainability, and Prosperity effects remain the same) ...

    // === HarmonyStabilization - Gentle swirling orbit behavior ===
    let mut harmony = EffectAsset::new(380, Spawner::once(55.0.into(), true), Module::default());
    harmony
        .init(PositionSphereModifier::new(1.2))
        .init(InitVelocityTangentModifier::new(
            Vec3::Y,           // Orbit around vertical axis
            1.8,               // Tangential speed (swirling strength)
            0.4,               // Small radial component
        ))
        .init(TurbulenceModifier::new(0.5, 0.3))           // Very soft turbulence for organic feel
        .init(SizeOverLifetimeModifier::new(Gradient::linear(0.7, 0.15)))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(
                Color::srgb(0.95, 0.55, 0.9),
                Color::srgba(0.95, 0.55, 0.9, 0.0),
            ),
        )));
    particle_effects.harmony = effects.add(harmony);

    // ... rest unchanged ...
}
