/*!
 * Deeper exploration of Lissajous knot topology.
 */

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_effects: ResMut<PolicyParticleEffects>,
) {
    // ... other effects unchanged ...

    // === HarmonyStabilization - Classic (3,2,5) Lissajous knot approximation ===
    // This frequency triple is known to produce trefoil-like knot topology
    let mut harmony = EffectAsset::new(500, Spawner::once(85.0.into(), true), Module::default());

    harmony
        .init(PositionSphereModifier::new(0.65))
        // X frequency ~3
        .init(InitVelocityTangentModifier::new(Vec3::X, 1.8, 0.15))
        // Y frequency ~2
        .init(InitVelocityTangentModifier::new(Vec3::Y, 1.2, 0.28))
        // Z frequency ~5
        .init(InitVelocityTangentModifier::new(Vec3::Z, 3.0, 0.18))
        // Phase offset via different radial components
        .init(AccelerationModifier::new(Vec3::new(0.05, 0.0, 0.05)))
        .init(AccelerationModifier::new(Vec3::new(0.0, 0.5, 0.0)))
        .init(TurbulenceModifier::new(0.18, 0.1))
        .init(SizeOverLifetimeModifier::new(Gradient::linear(0.48, 0.04)))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(
                Color::srgb(0.95, 0.55, 0.9),
                Color::srgba(0.95, 0.55, 0.9, 0.0),
            ),
        )));

    particle_effects.harmony = effects.add(harmony);
}

// === Lissajous Knot Topology Notes ===
// Real Lissajous knots are defined by three sinusoidal equations:
// x = sin(a*t + phi1)
// y = sin(b*t + phi2)
// z = sin(c*t + phi3)
// with a:b:c being small integers (frequency ratios).
//
// Certain ratios like (3,2,5), (3,4,5), (2,3,7) produce knotted curves.
// Our tangent velocity approximation creates visually similar 3D closed paths.
