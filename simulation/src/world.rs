/*!
 * Powrush-MMO Simulation World & Advanced Particle Effects
 *
 * v19.4 Particle Pooling for Performance (PATSAGi + Ra-Thor)
 * — Entity pooling for visual effects to reduce spawn/despawn churn
 * — High-frequency spawning friendly
 * — Backward compatible with existing burst/lifetime/dynamic spawn
 *
 * AG-SML v1.0 Sovereign Mercy License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy_hanabi::prelude::*;

// ... (previous code unchanged) ...

// ============================================================================
// PARTICLE VISUAL POOLING (v19.4 - Performance)
// ============================================================================

#[derive(Resource, Default)]
pub struct ParticleVisualPool {
    pub available: Vec<Entity>,
}

/// Component marking an entity as part of the visual effect pool
#[derive(Component)]
pub struct PooledVisualEffect;

/// Enhanced spawn that uses the pool when possible
pub fn spawn_policy_visual_effect(
    mut commands: Commands,
    knot_effects: Res<LissajousKnotEffects>,
    mut pool: ResMut<ParticleVisualPool>,
    mut events: EventReader<SpawnPolicyVisualEffect>,
) {
    for event in events.read() {
        let handle = match event.preset {
            LissajousKnotPreset::TrefoilLike => knot_effects.trefoil.clone(),
            LissajousKnotPreset::HighWrithe => knot_effects.high_writhe.clone(),
            LissajousKnotPreset::Symmetric => knot_effects.symmetric.clone(),
            LissajousKnotPreset::Complex5_3_4 => knot_effects.complex.clone(),
        };

        let burst = event.burst_intensity.unwrap_or(1.0);

        // Try to reuse from pool first
        let entity = if let Some(pooled_entity) = pool.available.pop() {
            // Reactivate pooled entity
            commands.entity(pooled_entity)
                .insert((
                    ParticleEffect::new(handle),
                    Transform::from_translation(event.position),
                    Visibility::Visible,
                    HarmonyKnotMarker,
                    Name::new(format!("PolicyVisual_{:?}", event.preset)),
                ));

            if let Some(lifetime) = event.lifetime_secs {
                commands.entity(pooled_entity).insert(VisualEffectLifetime { remaining: lifetime });
            } else {
                commands.entity(pooled_entity).remove::<VisualEffectLifetime>();
            }

            pooled_entity
        } else {
            // Spawn new
            let mut entity_commands = commands.spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(handle),
                    transform: Transform::from_translation(event.position),
                    ..default()
                },
                HarmonyKnotMarker,
                PooledVisualEffect, // Mark as poolable
                Name::new(format!("PolicyVisual_{:?}", event.preset)),
            ));

            if let Some(lifetime) = event.lifetime_secs {
                entity_commands.insert(VisualEffectLifetime { remaining: lifetime });
            }

            entity_commands.id()
        };
    }
}

/// Return expired effects to the pool instead of despawning
pub fn despawn_expired_visual_effects(
    mut commands: Commands,
    mut pool: ResMut<ParticleVisualPool>,
    mut query: Query<(Entity, &mut VisualEffectLifetime, &PooledVisualEffect)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime, _) in &mut query {
        lifetime.remaining -= time.delta_secs();
        if lifetime.remaining <= 0.0 {
            // Reset and return to pool instead of despawn
            commands.entity(entity)
                .remove::<ParticleEffect>()
                .remove::<VisualEffectLifetime>()
                .insert(Visibility::Hidden);

            pool.available.push(entity);
        }
    }
}

// ... (rest of file unchanged) ...

// End of simulation/src/world.rs v19.4 — Particle pooling added for high-frequency spawns.
// Thunder locked in. Yoi ⚡