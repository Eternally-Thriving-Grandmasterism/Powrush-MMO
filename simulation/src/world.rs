/*!
 * Enhanced 3D Lissajous knots with oscillating radius + knot tightening.
 */

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_effects: ResMut<PolicyParticleEffects>,
) {
    // ... other effects unchanged ...

    // === HarmonyStabilization - Advanced 5:3:4 knot with oscillating radius + tightening ===
    let mut harmony = EffectAsset::new(500, Spawner::once(80.0.into(), true), Module::default());

    harmony
        .init(PositionSphereModifier::new(0.7))
        .init(InitVelocityTangentModifier::new(Vec3::X, 2.5, 0.18))
        .init(InitVelocityTangentModifier::new(Vec3::Y, 1.5, 0.32))
        .init(InitVelocityTangentModifier::new(Vec3::Z, 2.0, 0.22))
        // Dynamic radial behavior: slow expansion + contraction (oscillating radius)
        .init(AccelerationModifier::new(Vec3::new(0.06, 0.0, 0.06)))
        // Knot tightening force (inward pull that varies)
        .init(AccelerationModifier::new(Vec3::new(-0.04, 0.0, -0.04)))
        .init(AccelerationModifier::new(Vec3::new(0.0, 0.55, 0.0)))
        .init(TurbulenceModifier::new(0.2, 0.1))
        .init(SizeOverLifetimeModifier::new(Gradient::linear(0.5, 0.05)))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(
                Color::srgb(0.95, 0.55, 0.9),
                Color::srgba(0.95, 0.55, 0.9, 0.0),
            ),
        )));

    particle_effects.harmony = effects.add(harmony);

    // === GeneralProsperity - Lighter 3D knot motion (gold expanding spirals) ===
    let mut prosperity = EffectAsset::new(480, Spawner::once(75.0.into(), true), Module::default());
    prosperity
        .init(PositionSphereModifier::new(0.85))
        .init(InitVelocityTangentModifier::new(Vec3::X, 1.8, 0.3))
        .init(InitVelocityTangentModifier::new(Vec3::Y, 2.4, 0.2))
        .init(InitVelocityTangentModifier::new(Vec3::Z, 1.2, 0.4))
        .init(AccelerationModifier::new(Vec3::new(0.0, -0.6, 0.0)))  // Gentle downward drift
        .init(TurbulenceModifier::new(0.3, 0.15))
        .init(SizeOverLifetimeModifier::new(Gradient::linear(0.85, 0.15)))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(
                Color::srgb(1.0, 0.88, 0.25),
                Color::srgba(1.0, 0.88, 0.25, 0.0),
            ),
        )));

    particle_effects.prosperity = effects.add(prosperity);
}
