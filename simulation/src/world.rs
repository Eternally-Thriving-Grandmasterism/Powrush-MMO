/*!
 * Figure-8 (Lissajous) particle paths using advanced tangent techniques.
 */

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_effects: ResMut<PolicyParticleEffects>,
) {
    // ... other effects unchanged ...

    // === HarmonyStabilization - Figure-8 Lissajous orbiting ===
    let mut harmony = EffectAsset::new(420, Spawner::once(65.0.into(), true), Module::default());

    harmony
        .init(PositionSphereModifier::new(0.9))
        // Primary orbital motion (Y axis)
        .init(InitVelocityTangentModifier::new(
            Vec3::Y,
            2.0,
            0.25,
        ))
        // Secondary perpendicular motion to create figure-8 crossing
        .init(InitVelocityTangentModifier::new(
            Vec3::X,           // Perpendicular axis
            1.3,
            0.6,
        ))
        // Gentle upward drift
        .init(AccelerationModifier::new(Vec3::new(0.0, 0.7, 0.0)))
        .init(TurbulenceModifier::new(0.35, 0.2))
        .init(SizeOverLifetimeModifier::new(Gradient::linear(0.6, 0.12)))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(
                Color::srgb(0.95, 0.55, 0.9),
                Color::srgba(0.95, 0.55, 0.9, 0.0),
            ),
        )));

    particle_effects.harmony = effects.add(harmony);
}
