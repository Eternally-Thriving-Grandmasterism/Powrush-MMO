/*!
 * Full EffectAsset creation for all policy particle effects (Bevy 0.14 + Hanabi 0.13).
 */

use bevy_hanabi::prelude::*;

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_effects: ResMut<PolicyParticleEffects>,
) {
    // === 1. AbundanceBoost - Energetic green upward burst with turbulence ===
    let mut abundance = EffectAsset::new(
        600,
        Spawner::once(100.0.into(), true),
        Module::default(),
    );
    abundance
        .init(PositionSphereModifier::new(0.6))
        .init(InitVelocityTangentModifier::new(Vec3::Y, 4.5, 0.3))
        .init(AccelerationModifier::new(Vec3::new(0.0, 3.0, 0.0)))
        .init(TurbulenceModifier::new(1.8, 1.2))
        .init(SizeOverLifetimeModifier::new(Gradient::linear(0.9, 0.1)))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(
                Color::srgb(0.15, 0.95, 0.35),
                Color::srgba(0.15, 0.95, 0.35, 0.0),
            ),
        )));
    particle_effects.abundance = effects.add(abundance);

    // === 2. SustainabilityFocus - Calm cyan flow with drag + attractor ===
    let mut sustainability = EffectAsset::new(
        450,
        Spawner::once(70.0.into(), true),
        Module::default(),
    );
    sustainability
        .init(PositionSphereModifier::new(1.0))
        .init(InitVelocityTangentModifier::new(Vec3::Y, 2.8, 0.4))
        .init(DragModifier::new(0.7))
        .init(AttractorModifier::new(Vec3::ZERO, 1.2, 4.0))
        .init(SizeOverLifetimeModifier::new(Gradient::linear(0.7, 0.15)))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(
                Color::srgb(0.25, 0.85, 0.95),
                Color::srgba(0.25, 0.85, 0.95, 0.0),
            ),
        )));
    particle_effects.sustainability = effects.add(sustainability);

    // === 3. HarmonyStabilization - Pink 5:3:4 3D Lissajous knot with breathing ===
    let mut harmony = EffectAsset::new(
        500,
        Spawner::once(85.0.into(), true),
        Module::default(),
    );
    harmony
        .init(PositionSphereModifier::new(0.7))
        .init(InitVelocityTangentModifier::new(Vec3::X, 2.5, 0.18))
        .init(InitVelocityTangentModifier::new(Vec3::Y, 1.5, 0.32))
        .init(InitVelocityTangentModifier::new(Vec3::Z, 2.0, 0.22))
        .init(AccelerationModifier::new(Vec3::new(0.06, 0.0, 0.06)))
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

    // === 4. GeneralProsperity - Gold expanding burst with gentle gravity ===
    let mut prosperity = EffectAsset::new(
        520,
        Spawner::once(85.0.into(), true),
        Module::default(),
    );
    prosperity
        .init(PositionSphereModifier::new(0.85))
        .init(InitVelocityTangentModifier::new(Vec3::X, 1.8, 0.3))
        .init(InitVelocityTangentModifier::new(Vec3::Y, 2.4, 0.2))
        .init(InitVelocityTangentModifier::new(Vec3::Z, 1.2, 0.4))
        .init(AccelerationModifier::new(Vec3::new(0.0, -0.6, 0.0)))
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
