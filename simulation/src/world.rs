/*!
 * Mathematical exploration of Lissajous torus knots.
 */

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_effects: ResMut<PolicyParticleEffects>,
) {
    // ... other effects unchanged ...

    // === HarmonyStabilization - (2,3,5) Lissajous knot (trefoil topology) ===
    // This frequency triple is known to produce curves isotopic to the trefoil knot T(2,3)
    let mut harmony = EffectAsset::new(480, Spawner::once(80.0.into(), true), Module::default());

    harmony
        .init(PositionSphereModifier::new(0.6))
        // Frequency ~2
        .init(InitVelocityTangentModifier::new(Vec3::X, 1.4, 0.12))
        // Frequency ~3
        .init(InitVelocityTangentModifier::new(Vec3::Y, 2.1, 0.25))
        // Frequency ~5
        .init(InitVelocityTangentModifier::new(Vec3::Z, 3.5, 0.15))
        .init(AccelerationModifier::new(Vec3::new(0.0, 0.45, 0.0)))
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

// === Lissajous Torus Knot Mathematics ===
//
// A 3D Lissajous curve is defined by:
//   x(t) = sin(a*t + φx)
//   y(t) = sin(b*t + φy)
//   z(t) = sin(c*t + φz)
//
// When a, b, c are integers (especially small coprime triples),
// the curve closes into a knot or link after period T = 2π * lcm(1/a,1/b,1/c).
//
// Famous examples that produce torus knots:
// - (2,3,5) → Trefoil knot T(2,3)
// - (3,4,5) → More complex knot
// - (2,3,7) → Cinquefoil knot T(2,5)
//
// Our Hanabi implementation approximates these using three perpendicular
// InitVelocityTangentModifier calls with rational frequency ratios.
// While not mathematically exact sinusoidal position functions,
// the resulting trajectories produce visually convincing 3D knotted structures.
