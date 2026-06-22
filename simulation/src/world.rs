/*!
 * Advanced Hanabi particle physics modifiers.
 */

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_effects: ResMut<PolicyParticleEffects>,
) {
    // === AbundanceBoost - Energetic with turbulence and upward acceleration ===
    let mut abundance = EffectAsset::new(600, Spawner::once(100.0.into(), true), Module::default());
    abundance
        .init(PositionSphereModifier::new(0.6))
        .init(VelocityConeModifier::new(Vec3::Y, 35.0_f32.to_radians(), 5.5))
        .init(AccelerationModifier::new(Vec3::new(0.0, 2.5, 0.0)))           // Upward push
        .init(TurbulenceModifier::new(1.2, 0.8))                             // Organic energy
        .init(SizeOverLifetimeModifier::new(Gradient::linear(0.8, 0.0)))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(Color::srgb(0.15, 0.95, 0.35), Color::srgba(0.15, 0.95, 0.35, 0.0)),
        )));
    particle_effects.abundance = effects.add(abundance);

    // === SustainabilityFocus - Stable flow with light drag and gentle attraction ===
    let mut sustainability = EffectAsset::new(450, Spawner::once(70.0.into(), true), Module::default());
    sustainability
        .init(PositionSphereModifier::new(1.0))
        .init(VelocityLinearModifier::new(Vec3::new(0.0, 2.2, 0.0)))
        .init(DragModifier::new(0.6))                                        // Creates stable flow
        .init(AttractorModifier::new(Vec3::ZERO, 0.8, 3.0))                  // Gentle pull toward center
        .init(SizeOverLifetimeModifier::new(Gradient::linear(0.6, 0.1)))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(Color::srgb(0.25, 0.85, 0.95), Color::srgba(0.25, 0.85, 0.95, 0.0)),
        )));
    particle_effects.sustainability = effects.add(sustainability);

    // === HarmonyStabilization - Soft swirling + low turbulence ===
    let mut harmony = EffectAsset::new(380, Spawner::once(55.0.into(), true), Module::default());
    harmony
        .init(PositionSphereModifier::new(1.2))
        .init(VelocityLinearModifier::new(Vec3::new(0.0, 1.2, 0.0)))
        .init(TurbulenceModifier::new(0.6, 0.4))                             // Soft organic movement
        .init(SizeOverLifetimeModifier::new(Gradient::linear(0.7, 0.15)))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(Color::srgb(0.95, 0.55, 0.9), Color::srgba(0.95, 0.55, 0.9, 0.0)),
        )));
    particle_effects.harmony = effects.add(harmony);

    // === GeneralProsperity - Expanding with outward force and slight gravity ===
    let mut prosperity = EffectAsset::new(520, Spawner::once(85.0.into(), true), Module::default());
    prosperity
        .init(PositionSphereModifier::new(0.8))
        .init(VelocityConeModifier::new(Vec3::Y, 50.0_f32.to_radians(), 4.0))
        .init(AccelerationModifier::new(Vec3::new(0.0, -0.8, 0.0)))          // Gentle downward gravity feel
        .init(SizeOverLifetimeModifier::new(Gradient::linear(0.9, 0.2)))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(Color::srgb(1.0, 0.88, 0.25), Color::srgba(1.0, 0.88, 0.25, 0.0)),
        )));
    particle_effects.prosperity = effects.add(prosperity);
}
