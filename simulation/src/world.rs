/*!
 * 5:3 Lissajous ratio with phase offset for richer patterns.
 */

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_effects: ResMut<PolicyParticleEffects>,
) {
    // ... other effects unchanged ...

    // === HarmonyStabilization - 5:3 Lissajous with phase offset ===
    let mut harmony = EffectAsset::new(450, Spawner::once(70.0.into(), true), Module::default());

    harmony
        .init(PositionSphereModifier::new(0.85))
        // Primary axis (frequency ~5)
        .init(InitVelocityTangentModifier::new(
            Vec3::Y,
            2.8,
            0.15,
        ))
        // Secondary axis (frequency ~3) with phase offset via different radial component
        .init(InitVelocityTangentModifier::new(
            Vec3::X,
            1.68,              // ~3/5 of primary
            0.65,              // Different radial component creates phase shift
        ))
        .init(AccelerationModifier::new(Vec3::new(0.0, 0.65, 0.0)))
        .init(TurbulenceModifier::new(0.3, 0.18))
        .init(SizeOverLifetimeModifier::new(Gradient::linear(0.55, 0.1)))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(
                Color::srgb(0.95, 0.55, 0.9),
                Color::srgba(0.95, 0.55, 0.9, 0.0),
            ),
        )));

    particle_effects.harmony = effects.add(harmony);
}
