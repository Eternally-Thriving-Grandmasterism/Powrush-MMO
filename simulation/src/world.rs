/*!
 * 3D Lissajous knot topology exploration.
 */

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_effects: ResMut<PolicyParticleEffects>,
) {
    // ... other effects unchanged ...

    // === HarmonyStabilization - 3D Lissajous knot (3:4:5 approximation) ===
    let mut harmony = EffectAsset::new(480, Spawner::once(75.0.into(), true), Module::default());

    harmony
        .init(PositionSphereModifier::new(0.8))
        // X-axis component (frequency ~3)
        .init(InitVelocityTangentModifier::new(
            Vec3::X,
            1.5,
            0.4,
        ))
        // Y-axis component (frequency ~4)
        .init(InitVelocityTangentModifier::new(
            Vec3::Y,
            2.0,
            0.3,
        ))
        // Z-axis component (frequency ~5) - creates 3D knot topology
        .init(InitVelocityTangentModifier::new(
            Vec3::Z,
            2.5,
            0.25,
        ))
        .init(TurbulenceModifier::new(0.25, 0.15))
        .init(SizeOverLifetimeModifier::new(Gradient::linear(0.55, 0.08)))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(
                Color::srgb(0.95, 0.55, 0.9),
                Color::srgba(0.95, 0.55, 0.9, 0.0),
            ),
        )));

    particle_effects.harmony = effects.add(harmony);
}

// === 3D Lissajous Knot Notes ===
// Using three perpendicular tangent velocities with rational frequency ratios
// (e.g. 3:4:5) creates closed 3D curves known as Lissajous knots.
// These can form beautiful topological structures like trefoil-like loops.
