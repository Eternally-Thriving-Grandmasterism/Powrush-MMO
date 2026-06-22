/*!
 * Exploring advanced TangentModifier usage for complex particle paths.
 */

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_effects: ResMut<PolicyParticleEffects>,
) {
    // ... other effects unchanged ...

    // === HarmonyStabilization - Advanced helical orbiting with radial expansion ===
    let mut harmony = EffectAsset::new(400, Spawner::once(60.0.into(), true), Module::default());

    harmony
        .init(PositionSphereModifier::new(1.0))
        // Strong tangential velocity for orbiting
        .init(InitVelocityTangentModifier::new(
            Vec3::Y,           // Orbit axis
            2.2,               // Tangential speed
            0.3,               // Small initial radial velocity
        ))
        // Gentle outward radial acceleration → slowly expanding orbit
        .init(AccelerationModifier::new(Vec3::new(0.15, 0.0, 0.15)))
        // Light upward drift for helical motion
        .init(AccelerationModifier::new(Vec3::new(0.0, 0.9, 0.0)))
        .init(TurbulenceModifier::new(0.4, 0.25))
        .init(SizeOverLifetimeModifier::new(Gradient::linear(0.65, 0.1)))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(
                Color::srgb(0.95, 0.55, 0.9),
                Color::srgba(0.95, 0.55, 0.9, 0.0),
            ),
        )));

    particle_effects.harmony = effects.add(harmony);
}
