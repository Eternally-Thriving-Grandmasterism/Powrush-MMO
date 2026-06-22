/*!
 * Full visual particle effects with lifetime and UI notification support.
 */

use bevy::prelude::*;

#[derive(Component)]
pub struct VisualEffect {
    pub lifetime: f32,
}

#[derive(Event, Clone, Debug)]
pub struct ShowFloatingText {
    pub position: Vec3,
    pub text: String,
    pub color: Color,
}

// Enhanced spawn function with lifetime
fn spawn_policy_particle_effect(
    commands: &mut Commands,
    position: Vec3,
    policy_type: PolicyType,
) {
    let color = match policy_type {
        PolicyType::AbundanceBoost => Color::srgb(0.2, 0.9, 0.4),
        PolicyType::SustainabilityFocus => Color::srgb(0.3, 0.8, 0.9),
        PolicyType::HarmonyStabilization => Color::srgb(0.9, 0.6, 0.9),
        PolicyType::GeneralProsperity => Color::srgb(1.0, 0.85, 0.3),
        _ => Color::WHITE,
    };

    commands.spawn((
        SpatialEntityBundle::default(),
        Transform::from_translation(position),
        Visibility::default(),
        VisualEffect { lifetime: 2.5 },
        Name::new(format!("PolicyEffect_{:?}", policy_type)),
    ));

    // Emit floating text event for UI
    commands.spawn(ShowFloatingText {
        position,
        text: format!("+ Epigenetic Shift ({:?})", policy_type),
        color,
    });
}

// Cleanup system for visual effects
pub fn cleanup_visual_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut VisualEffect)>,
) {
    for (entity, mut effect) in &mut query {
        effect.lifetime -= time.delta_secs();
        if effect.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
