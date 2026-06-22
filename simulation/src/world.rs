/*!
 * Torus knot variant exploration for Harmony effect.
 */

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_effects: ResMut<PolicyParticleEffects>,
) {
    // ... other effects unchanged ...

    // === HarmonyStabilization - (3,4) Torus knot approximation ===
    // Winds 3 times meridionally and 4 times longitudinally around a torus
    let mut harmony = EffectAsset::new(520, Spawner::once(90.0.into(), true), Module::default());

    harmony
        .init(PositionSphereModifier::new(0.7))
        // Major toroidal circle (around Y axis)
        .init(InitVelocityTangentModifier::new(Vec3::Y, 2.0, 0.1))
        // Minor poloidal winding (around local tube)
        .init(InitVelocityTangentModifier::new(Vec3::X, 1.5, 0.45))
        // Additional Z component for 3D toroidal structure
        .init(InitVelocityTangentModifier::new(Vec3::Z, 2.7, 0.2))
        .init(AccelerationModifier::new(Vec3::new(0.0, 0.4, 0.0)))
        .init(TurbulenceModifier::new(0.2, 0.1))
        .init(SizeOverLifetimeModifier::new(Gradient::linear(0.45, 0.05)))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(
                Color::srgb(0.95, 0.55, 0.9),
                Color::srgba(0.95, 0.55, 0.9, 0.0),
            ),
        )));

    particle_effects.harmony = effects.add(harmony);
}

// === Torus Knot Notes ===
// A (p,q) torus knot winds p times around the major circle
// and q times around the minor tube of a torus.
// Common examples:
// (2,3) = Trefoil knot
// (3,4) = Solomon's knot (link)
// (2,5), (3,5), (4,5) produce more complex knots
//
// Our implementation approximates toroidal winding using
// multiple perpendicular tangent velocity components.
