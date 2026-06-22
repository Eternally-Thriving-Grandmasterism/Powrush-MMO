/*!
 * Visual particle effects for policy-driven epigenetic changes.
 */

use bevy::prelude::*;

// Enhanced event handler that spawns visual effects
pub fn handle_epigenetic_policy_effect_applied(
    mut commands: Commands,
    mut events: EventReader<EpigeneticPolicyEffectApplied>,
    query: Query<&Transform, With<SpatialParticipant>>,
) {
    for event in events.read() {
        // Try to find the agent's position
        if let Some(transform) = query.iter().next() {  // Simplified - in real code filter by agent_id
            spawn_policy_particle_effect(&mut commands, transform.translation, event.policy_type);
        }

        info!(
            "[Client] Epigenetic effect from {:?} applied to agent {}",
            event.policy_type, event.agent_id
        );
    }
}

fn spawn_policy_particle_effect(
    commands: &mut Commands,
    position: Vec3,
    policy_type: PolicyType,
) {
    let color = match policy_type {
        PolicyType::AbundanceBoost => Color::srgb(0.2, 0.9, 0.4),      // Green
        PolicyType::SustainabilityFocus => Color::srgb(0.3, 0.8, 0.9), // Cyan
        PolicyType::HarmonyStabilization => Color::srgb(0.9, 0.6, 0.9), // Pink
        PolicyType::GeneralProsperity => Color::srgb(1.0, 0.85, 0.3),   // Gold
        _ => Color::WHITE,
    };

    commands.spawn((
        SpatialEntityBundle::default(),
        Transform::from_translation(position),
        Visibility::default(),
        Name::new(format!("PolicyEffect_{:?}", policy_type)),
        // In a full implementation, add a ParticleSystem component here
        // or a custom VisualEffect component that your renderer consumes.
    ));

    // Future enhancement: Add a timer component to despawn after X seconds
}
