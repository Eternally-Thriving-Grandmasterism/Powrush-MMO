/*!
 * Phase shift effects on Lissajous knot topology.
 */

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_effects: ResMut<PolicyParticleEffects>,
) {
    // ... other effects unchanged ...

    // === HarmonyStabilization - Exploring phase shift effects ===
    // Different radial components create effective phase shifts between axes
    let mut harmony = EffectAsset::new(480, Spawner::once(80.0.into(), true), Module::default());

    harmony
        .init(PositionSphereModifier::new(0.6))
        // X axis with small radial component (phase ~0)
        .init(InitVelocityTangentModifier::new(Vec3::X, 1.8, 0.10))
        // Y axis with medium radial component (phase shift)
        .init(InitVelocityTangentModifier::new(Vec3::Y, 2.4, 0.35))
        // Z axis with larger radial component (stronger phase shift)
        .init(InitVelocityTangentModifier::new(Vec3::Z, 3.0, 0.55))
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

// === Phase Shift Effects on Knot Topology ===
// In true Lissajous knots, phase shifts (φx, φy, φz) dramatically change
// the geometry and can even change whether the curve is knotted.
//
// In our Hanabi approximation, we simulate phase shifts by varying
// the radial velocity component in InitVelocityTangentModifier.
// Smaller radial = less phase offset; larger radial = stronger phase shift.
//
// This allows us to explore how different "phases" affect the
// complexity and crossing patterns of the resulting 3D knot.
