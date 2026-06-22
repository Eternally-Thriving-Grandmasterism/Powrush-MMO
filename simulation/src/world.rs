/*!
 * Exploring different Lissajous frequency ratios in Hanabi.
 */

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_effects: ResMut<PolicyParticleEffects>,
) {
    // ... other effects unchanged ...

    // === HarmonyStabilization - 3:2 Lissajous frequency ratio ===
    // Classic beautiful multi-looped figure-8 variant
    let mut harmony = EffectAsset::new(420, Spawner::once(65.0.into(), true), Module::default());

    harmony
        .init(PositionSphereModifier::new(0.9))
        // Y-axis frequency (higher)
        .init(InitVelocityTangentModifier::new(
            Vec3::Y,
            2.4,               // Frequency ~3
            0.2,
        ))
        // X-axis frequency (lower)
        .init(InitVelocityTangentModifier::new(
            Vec3::X,
            1.6,               // Frequency ~2
            0.5,
        ))
        .init(AccelerationModifier::new(Vec3::new(0.0, 0.7, 0.0)))
        .init(TurbulenceModifier::new(0.3, 0.2))
        .init(SizeOverLifetimeModifier::new(Gradient::linear(0.6, 0.12)))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(
                Color::srgb(0.95, 0.55, 0.9),
                Color::srgba(0.95, 0.55, 0.9, 0.0),
            ),
        )));

    particle_effects.harmony = effects.add(harmony);
}

// === Frequency Ratio Guide ===
// 1:1  → Circle / Ellipse
// 2:1  → Classic Figure-8 (parabola)
// 3:1  → More loops
// 3:2  → Beautiful complex figure-8 (current)
// 4:3  → Intricate multi-loop patterns
// 5:2  → Very complex Lissajous curves
