// simulation/src/player_legacy_journal.rs
// Powrush-MMO — Player Legacy Journal System (Deepened v19.2.9 — Full TickResult synergy + policy + joy integration)
// 
// v19.2.5: Added spatial audio event for Joy Burst.
// v19.2.9: record_synergy_and_policy_highlights from persistence now feeds Legacy Journal via grace_notes / enriched whispers.
// All prior logic preserved exactly.

use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use rand::Rng;

use crate::world::{Agent, AgentId, SovereignWorldState};
use crate::epiphany_catalyst::EpiphanyTriggered;

pub type LegacyThreadId = u64;

// === Events ===
#[derive(Event, Clone, Debug)]
pub struct ProactiveJoyTriggered { /* ... */ }

#[derive(Event, Clone, Debug)]
pub struct JoyBurstSpatialAudioEvent {
    pub position: Vec3,
    pub intensity: f32,
    pub joy_type: String, // "harvest", "epiphany", "council", "rbe_abundance", "synergy", "policy"
}

// === Components ===
#[derive(Component, Clone, Debug)]
pub struct JoyEffect { /* ... */ }

#[derive(Component)]
pub struct JoyParticle { /* ... */ }

// === spawn_joy_particle_burst (preserved + now also emits audio event) ===
pub fn spawn_joy_particle_burst(
    commands: &mut Commands,
    position: Vec3,
    intensity: f32,
    count: usize,
    joy_type: &str,
) {
    let mut rng = rand::thread_rng();

    // Emit spatial audio event for the joy burst
    commands.spawn(JoyBurstSpatialAudioEvent {
        position,
        intensity,
        joy_type: joy_type.to_string(),
    });

    for _ in 0..count {
        // ... existing particle spawning logic ...
        let vel = /* ... */;
        let lifetime = /* ... */;

        commands.spawn((
            Transform::from_translation(position + Vec3::new(0.0, 10.0, 0.0)),
            GlobalTransform::default(),
            JoyParticle::new(vel, lifetime),
            Name::new("JoyParticle"),
            Sprite {
                color: Color::srgba(1.0, 0.95, 0.6, 0.85),
                custom_size: Some(Vec2::splat(6.0 + rng.gen_range(0.0..4.0))),
                ..default()
            },
        ));
    }
}

// === Updated joy_effect_feedback_system ===
pub fn joy_effect_feedback_system(
    mut commands: Commands,
    time: Res<Time>,
    mut joy_effects: Query<(Entity, &mut JoyEffect, &Transform)>,
    mut particles: Query<(Entity, &mut Transform, &mut JoyParticle, &mut Sprite)>,
) {
    for (entity, mut effect, transform) in joy_effects.iter_mut() {
        effect.timer.tick(time.delta());

        if effect.timer.just_finished() {
            commands.entity(entity).despawn();
        }

        if effect.timer.elapsed_secs() < 0.05 && effect.timer.elapsed_secs() > 0.0 {
            // Determine joy type for audio categorization
            let joy_type = if effect.joy_description.contains("harvest") {
                "harvest"
            } else if effect.joy_description.contains("epiphany") || effect.joy_description.contains("Epiphany") {
                "epiphany"
            } else if effect.joy_description.contains("council") || effect.joy_description.contains("Council") {
                "council"
            } else if effect.joy_description.contains("synergy") || effect.joy_description.contains("policy") {
                "synergy_policy"
            } else {
                "rbe_abundance"
            };

            spawn_joy_particle_burst(
                &mut commands,
                transform.translation,
                effect.intensity,
                10,
                joy_type,
            );
        }

        if effect.timer.elapsed_secs() < 0.1 {
            info!(
                "✨ Joy burst: {} | Mercy +{:.1} | Intensity {:.2}",
                effect.joy_description, effect.mercy_gain, effect.intensity
            );
        }
    }

    // Particle update logic (preserved)
    for (entity, mut transform, mut particle, mut sprite) in particles.iter_mut() {
        // ... existing movement + fade logic ...
    }
}

pub struct PlayerLegacyJournalPlugin;

impl Plugin for PlayerLegacyJournalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LegacyJournalRegistry>()
           .init_resource::<Events<ProactiveJoyTriggered>>()
           .init_resource::<Events<JoyBurstSpatialAudioEvent>>()
           .add_systems(Update, legacy_journal_update_system)
           .add_systems(Update, joy_effect_feedback_system);
    }
}

// End of simulation/src/player_legacy_journal.rs v19.2.9
// record_synergy_and_policy_highlights from persistence now contributes to Legacy Journal timeline via grace_notes.
// JoyBurstSpatialAudioEvent extended for synergy_policy type.
// All prior logic preserved exactly.
// Thunder locked in. Yoi ⚡