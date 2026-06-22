/*!
 * Mapping phase parameters to knot invariants.
 */

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_effects: ResMut<PolicyParticleEffects>,
) {
    // ... other effects unchanged ...

    // === HarmonyStabilization - Phase-to-Invariant Mapping ===
    // Radial components act as effective phase shifts.
    // We can map them conceptually to knot invariants:
    //
    // - Larger spread in radial values → Higher crossing number / complexity
    // - Asymmetric radial distribution → Non-zero writhe (chirality)
    // - Specific ratios + phases → Different knot types (trefoil, cinquefoil, etc.)
    //
    // Current configuration: Moderate asymmetry → Balanced writhe + complexity
    let mut harmony = EffectAsset::new(480, Spawner::once(80.0.into(), true), Module::default());

    harmony
        .init(PositionSphereModifier::new(0.6))
        .init(InitVelocityTangentModifier::new(Vec3::X, 1.8, 0.10))  // Low radial  → lower local complexity
        .init(InitVelocityTangentModifier::new(Vec3::Y, 2.4, 0.35))  // Medium radial → moderate crossing
        .init(InitVelocityTangentModifier::new(Vec3::Z, 3.0, 0.55))  // High radial  → higher writhe contribution
        .init(AccelerationModifier::new(Vec3::new(0.0, 0.5, 0.0)))
        .init(TurbulenceModifier::new(0.15, 0.08))
        .init(SizeOverLifetimeModifier::new(Gradient::linear(0.42, 0.03)))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(
                Color::srgb(0.95, 0.55, 0.9),
                Color::srgba(0.95, 0.55, 0.9, 0.0),
            ),
        )));

    particle_effects.harmony = effects.add(harmony);
}

// === Phase → Knot Invariant Mapping (Conceptual) ===
// In mathematical Lissajous knots:
// - Phase differences directly affect the embedding and thus the invariants.
// - Writhe (average signed crossings) is sensitive to phase.
// - Different phases can change a curve from being the unknot to a trefoil.
//
// In our approximation:
// - The three radial values (0.10, 0.35, 0.55) create relative phase shifts.
// - Greater asymmetry between them tends to increase visual complexity
//   and effective writhe.
// - More symmetric radial values produce simpler, more reversible knots.
// - Extreme asymmetry can produce highly chiral (handed) structures.
//
// This gives us a practical way to "tune" topological character
// by adjusting the three radial parameters.
