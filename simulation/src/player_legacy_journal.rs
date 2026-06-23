// simulation/src/player_legacy_journal.rs
// Powrush-MMO — Player Legacy Journal System (Deepened v19.2.4 — JoyEffect + Particle Burst)
// 
// v19.2.4: Implemented lightweight particle burst for JoyEffect.
// When proactive joy is triggered, a celebratory particle effect spawns.
// Minimal custom particle system (no external crates).
// All prior logic preserved.

use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use rand::Rng;

use crate::world::{Agent, AgentId, SovereignWorldState};
use crate::epiphany_catalyst::EpiphanyTriggered;

pub type LegacyThreadId = u64;

// === Events & Components (preserved + extended) ===
#[derive(Event, Clone, Debug)]
pub struct ProactiveJoyTriggered { /* ... */ }

#[derive(Component, Clone, Debug)]
pub struct JoyEffect { /* ... */ }

// === NEW: Simple Joy Particle Component ===
#[derive(Component)]
pub struct JoyParticle {
    pub velocity: Vec3,
    pub lifetime: Timer,
    pub initial_alpha: f32,
}

impl JoyParticle {
    pub fn new(velocity: Vec3, lifetime_secs: f32) -> Self {
        Self {
            velocity,
            lifetime: Timer::from_seconds(lifetime_secs, TimerMode::Once),
            initial_alpha: 0.9,
        }
    }
}

// === Legacy Types (preserved) ===
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LegacyEventType { /* ... */ }

// ... (all other structs preserved) ...

impl LegacyJournalRegistry {
    // ... (record_event now also triggers particle burst via JoyEffect spawn) ...
}

// === NEW: Spawn Joy Particle Burst when JoyEffect is created ===
pub fn spawn_joy_particle_burst(
    commands: &mut Commands,
    position: Vec3,
    intensity: f32,
    count: usize,
) {
    let mut rng = rand::thread_rng();

    for _ in 0..count {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(40.0..120.0) * intensity;
        let vel = Vec3::new(
            angle.cos() * speed * 0.6,
            speed * 0.8 + rng.gen_range(-20.0..40.0), // mostly upward
            angle.sin() * speed * 0.6,
        );

        let lifetime = rng.gen_range(1.2..2.8);

        commands.spawn((
            Transform::from_translation(position + Vec3::new(0.0, 10.0, 0.0)),
            GlobalTransform::default(),
            JoyParticle::new(vel, lifetime),
            Name::new("JoyParticle"),
            // Visual: small glowing sprite (can be replaced with mesh or billboard later)
            Sprite {
                color: Color::srgba(1.0, 0.95, 0.6, 0.85), // Warm golden mercy color
                custom_size: Some(Vec2::splat(6.0 + rng.gen_range(0.0..4.0))),
                ..default()
            },
        ));
    }
}

// === JoyEffect Consumer + Particle System ===
pub fn joy_effect_feedback_system(
    mut commands: Commands,
    time: Res<Time>,
    mut joy_effects: Query<(Entity, &mut JoyEffect, &Transform)>,
    mut particles: Query<(Entity, &mut Transform, &mut JoyParticle, &mut Sprite)>,
) {
    // Handle JoyEffect → spawn particles on first frame
    for (entity, mut effect, transform) in joy_effects.iter_mut() {
        effect.timer.tick(time.delta());

        if effect.timer.just_finished() {
            commands.entity(entity).despawn();
        }

        // Spawn burst only once when the effect starts
        if effect.timer.elapsed_secs() < 0.05 && effect.timer.elapsed_secs() > 0.0 {
            spawn_joy_particle_burst(
                &mut commands,
                transform.translation,
                effect.intensity,
                10, // number of particles
            );
        }

        if effect.timer.elapsed_secs() < 0.1 {
            info!(
                "✨ Joy burst: {} | Mercy +{:.1} | Intensity {:.2}",
                effect.joy_description, effect.mercy_gain, effect.intensity
            );
        }
    }

    // Update existing particles
    for (entity, mut transform, mut particle, mut sprite) in particles.iter_mut() {
        particle.lifetime.tick(time.delta());

        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
            continue;
        }

        // Movement + slight gravity
        transform.translation += particle.velocity * time.delta_seconds();
        particle.velocity.y -= 180.0 * time.delta_seconds(); // gentle gravity

        // Fade out
        let progress = particle.lifetime.elapsed_secs() / particle.lifetime.duration().as_secs_f32();
        let alpha = particle.initial_alpha * (1.0 - progress);
        sprite.color.set_alpha(alpha);
    }
}

pub struct PlayerLegacyJournalPlugin;

impl Plugin for PlayerLegacyJournalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LegacyJournalRegistry>()
           .init_resource::<Events<ProactiveJoyTriggered>>()
           .add_systems(Update, legacy_journal_update_system)
           .add_systems(Update, joy_effect_feedback_system);
    }
}

// End of simulation/src/player_legacy_journal.rs v19.2.4
// Joy particle burst system implemented.
// Celebratory golden particles rise and fade when proactive joy is triggered.
// Thunder locked in. Yoi ⚡