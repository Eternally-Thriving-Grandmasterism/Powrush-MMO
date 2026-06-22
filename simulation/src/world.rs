/*!
 * 5:3:4 Lissajous knot with dynamic topological behavior.
 */

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_effects: ResMut<PolicyParticleEffects>,
) {
    // ... other effects unchanged ...

    // === HarmonyStabilization - 5:3:4 Lissajous knot with knot tightening ===
    let mut harmony = EffectAsset::new(480, Spawner::once(80.0.into(), true), Module::default());

    harmony
        .init(PositionSphereModifier::new(0.75))
        // X component (~5)
        .init(InitVelocityTangentModifier::new(Vec3::X, 2.5, 0.2))
        // Y component (~3)
        .init(InitVelocityTangentModifier::new(Vec3::Y, 1.5, 0.35))
        // Z component (~4)
        .init(InitVelocityTangentModifier::new(Vec3::Z, 2.0, 0.25))
        // Dynamic knot behavior: slow radial expansion + contraction
        .init(AccelerationModifier::new(Vec3::new(0.08, 0.0, 0.08)))   // Gentle outward radial force
        .init(AccelerationModifier::new(Vec3::new(0.0, 0.6, 0.0)))     // Upward helical drift
        .init(TurbulenceModifier::new(0.22, 0.12))
        .init(SizeOverLifetimeModifier::new(Gradient::linear(0.5, 0.06)))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(
                Color::srgb(0.95, 0.55, 0.9),
                Color::srgba(0.95, 0.55, 0.9, 0.0),
            ),
        )));

    particle_effects.harmony = effects.add(harmony);
}
