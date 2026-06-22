/*!
 * bevy_hanabi integration for high-quality GPU particles.
 */

use bevy_hanabi::prelude::*;

// Call this once during app startup to register effects
pub fn setup_policy_particle_effects(mut effects: ResMut<Assets<EffectAsset>>) {
    // Abundance Boost - Green burst
    let mut abundance = EffectAsset::new(
        256,
        Spawner::once(50.0.into(), true),
        Module::default(),
    );
    // ... configure color, velocity, etc. (simplified here)
    effects.add(abundance);

    // Add more for other policy types...
}

// Updated spawn using Hanabi
fn spawn_policy_particle_effect(
    commands: &mut Commands,
    effects: &Res<Assets<EffectAsset>>,
    position: Vec3,
    policy_type: PolicyType,
) {
    // In real implementation, load the correct handle based on policy_type
    let effect_handle = effects.get_handle(0); // placeholder

    commands.spawn((
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect_handle),
            transform: Transform::from_translation(position),
            ..default()
        },
        VisualEffect { lifetime: 2.5 },
    ));
}
