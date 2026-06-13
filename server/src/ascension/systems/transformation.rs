/*!
 * Ambrosian Visual Transformation + Particle Effects
 * Leverages bevy_hanabi and current rendering pipeline.
 */

use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use crate::ascension::events::AmbrosianTransformation;

pub fn handle_ascension_transformation_system(
    mut commands: Commands,
    mut events: EventReader<AmbrosianTransformation>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    for event in events.read() {
        // Spawn dramatic ascension particle burst
        let burst_effect = effects.add(create_ascension_burst_effect());

        commands.spawn((
            ParticleEffectBundle {
                effect: ParticleEffect::new(burst_effect),
                transform: Transform::from_translation(Vec3::ZERO), // TODO: use actual player position
                ..default()
            },
            Name::new("Ascension Burst"),
        ));

        // Apply visual + mechanical state change
        commands.entity(event.entity)
            .insert(AmbrosianAscended)
            .insert(AmbrosianVisualState::Transformed); // TODO: define visual state component
    }
}

fn create_ascension_burst_effect() -> EffectAsset {
    // TODO: Full Hanabi configuration for purple/pink + blue crystal burst
    // Strong bloom, light rays, floating mercy sigils
    EffectAsset::new(
        5000,
        Handle::default(),
        // emitter, color gradients, etc. to be expanded
    )
}

#[derive(Component)]
pub struct AmbrosianVisualState {
    pub state: VisualState,
}

#[derive(Clone, Copy, Debug)]
pub enum VisualState {
    Normal,
    Transformed,
}
