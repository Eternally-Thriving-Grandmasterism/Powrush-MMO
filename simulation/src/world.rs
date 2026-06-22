/*!
 * Unique, well-defined Hanabi EffectAssets for each PolicyType.
 */

use bevy_hanabi::prelude::*;

#[derive(Resource, Default)]
pub struct PolicyParticleEffects {
    pub abundance: Handle<EffectAsset>,
    pub sustainability: Handle<EffectAsset>,
    pub harmony: Handle<EffectAsset>,
    pub prosperity: Handle<EffectAsset>,
}

pub fn setup_policy_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_effects: ResMut<PolicyParticleEffects>,
) {
    // === AbundanceBoost - Energetic green upward burst ===
    let mut abundance = EffectAsset::new(
        512,
        Spawner::once(80.0.into(), true),
        Module::default(),
    );
    abundance
        .init(PositionSphereModifier::new(0.5))
        .init(VelocityConeModifier::new(Vec3::Y, 30.0_f32.to_radians(), 4.0))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(Color::srgb(0.2, 0.9, 0.4), Color::srgba(0.2, 0.9, 0.4, 0.0)),
        )));
    particle_effects.abundance = effects.add(abundance);

    // === SustainabilityFocus - Calm cyan flowing particles ===
    let mut sustainability = EffectAsset::new(
        384,
        Spawner::once(60.0.into(), true),
        Module::default(),
    );
    sustainability
        .init(PositionSphereModifier::new(0.8))
        .init(VelocityLinearModifier::new(Vec3::new(0.0, 1.5, 0.0)))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(Color::srgb(0.3, 0.8, 0.9), Color::srgba(0.3, 0.8, 0.9, 0.0)),
        )));
    particle_effects.sustainability = effects.add(sustainability);

    // === HarmonyStabilization - Soft pink harmonious spread ===
    let mut harmony = EffectAsset::new(
        320,
        Spawner::once(50.0.into(), true),
        Module::default(),
    );
    harmony
        .init(PositionSphereModifier::new(1.0))
        .init(VelocityLinearModifier::new(Vec3::new(0.0, 0.8, 0.0)))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(Color::srgb(0.9, 0.6, 0.9), Color::srgba(0.9, 0.6, 0.9, 0.0)),
        )));
    particle_effects.harmony = effects.add(harmony);

    // === GeneralProsperity - Rich gold expanding burst ===
    let mut prosperity = EffectAsset::new(
        448,
        Spawner::once(70.0.into(), true),
        Module::default(),
    );
    prosperity
        .init(PositionSphereModifier::new(0.7))
        .init(VelocityConeModifier::new(Vec3::Y, 45.0_f32.to_radians(), 3.5))
        .init(SetColorModifier::new(ColorOverLifetimeModifier::new(
            Gradient::linear(Color::srgb(1.0, 0.85, 0.3), Color::srgba(1.0, 0.85, 0.3, 0.0)),
        )));
    particle_effects.prosperity = effects.add(prosperity);
}

// Helper to get the correct handle
impl PolicyParticleEffects {
    pub fn get(&self, policy_type: PolicyType) -> Handle<EffectAsset> {
        match policy_type {
            PolicyType::AbundanceBoost => self.abundance.clone(),
            PolicyType::SustainabilityFocus => self.sustainability.clone(),
            PolicyType::HarmonyStabilization => self.harmony.clone(),
            PolicyType::GeneralProsperity => self.prosperity.clone(),
            _ => self.prosperity.clone(),
        }
    }
}
