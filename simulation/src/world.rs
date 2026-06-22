/*!
 * Powrush-MMO Simulation World & Advanced Particle Effects
 *
 * v19.7 Bounded Freelist-Style Particle Pool (PATSAGi + Ra-Thor)
 * — Bounded pool with max_size
 * — Clean reset helper
 * — Clear overflow policy
 *
 * AG-SML v1.0 Sovereign Mercy License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy_hanabi::prelude::*;

// ============================================================================
// BOUNDED FREELIST VISUAL EFFECT POOL (v19.7)
// ============================================================================

const DEFAULT_VISUAL_POOL_CAPACITY: usize = 64;
const MAX_VISUAL_POOL_SIZE: usize = 256;

#[derive(Resource)]
pub struct ParticleVisualPool {
    pub available: Vec<Entity>,
    pub max_size: usize,
}

impl Default for ParticleVisualPool {
    fn default() -> Self {
        Self {
            available: Vec::with_capacity(DEFAULT_VISUAL_POOL_CAPACITY),
            max_size: MAX_VISUAL_POOL_SIZE,
        }
    }
}

impl ParticleVisualPool {
    pub fn with_capacity(capacity: usize, max_size: usize) -> Self {
        Self {
            available: Vec::with_capacity(capacity),
            max_size,
        }
    }

    pub fn is_full(&self) -> bool {
        self.available.len() >= self.max_size
    }
}

#[derive(Component)]
pub struct PooledVisualEffect;

/// Cleanly resets an entity so it can be reused from the pool.
fn reset_visual_effect_for_reuse(
    commands: &mut Commands,
    entity: Entity,
    new_handle: Handle<EffectAsset>,
    new_position: Vec3,
    new_lifetime: Option<f32>,
) {
    commands.entity(entity)
        .insert((
            ParticleEffect::new(new_handle),
            Transform::from_translation(new_position),
            Visibility::Visible,
            HarmonyKnotMarker,
        ))
        .remove::<VisualEffectLifetime>();

    if let Some(lifetime) = new_lifetime {
        commands.entity(entity).insert(VisualEffectLifetime { remaining: lifetime });
    }
}

/// Bounded freelist spawn
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

        let entity = if let Some(pooled) = pool.available.pop() {
            reset_visual_effect_for_reuse(
                &mut commands,
                pooled,
                handle,
                event.position,
                event.lifetime_secs,
            );
            pooled
        } else if !pool.is_full() {
            // Create new (we haven't reached max yet)
            let mut ec = commands.spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(handle),
                    transform: Transform::from_translation(event.position),
                    ..default()
                },
                HarmonyKnotMarker,
                PooledVisualEffect,
            ));

            if let Some(lifetime) = event.lifetime_secs {
                ec.insert(VisualEffectLifetime { remaining: lifetime });
            }
            ec.id()
        } else {
            // Pool is at max capacity - either reject or force spawn
            // For now we force spawn (can change to warning + skip later)
            let mut ec = commands.spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(handle),
                    transform: Transform::from_translation(event.position),
                    ..default()
                },
                HarmonyKnotMarker,
            ));

            if let Some(lifetime) = event.lifetime_secs {
                ec.insert(VisualEffectLifetime { remaining: lifetime });
            }
            ec.id()
        };
    }
}

/// Return expired effects to the pool (or despawn if pool is full)
pub fn return_expired_visual_effects_to_pool(
    mut commands: Commands,
    mut pool: ResMut<ParticleVisualPool>,
    mut query: Query<(Entity, &mut VisualEffectLifetime, Option<&PooledVisualEffect>)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime, is_pooled) in &mut query {
        lifetime.remaining -= time.delta_secs();

        if lifetime.remaining <= 0.0 {
            if is_pooled.is_some() && !pool.is_full() {
                // Reset and return to pool
                commands.entity(entity)
                    .remove::<ParticleEffect>()
                    .remove::<VisualEffectLifetime>()
                    .insert(Visibility::Hidden);

                pool.available.push(entity);
            } else {
                // Either not pooled or pool is full -> actual despawn
                commands.entity(entity).despawn();
            }
        }
    }
}

// Prewarm system (unchanged from v19.6 but works with bounded pool)
pub fn prewarm_visual_pool(
    mut commands: Commands,
    mut pool: ResMut<ParticleVisualPool>,
) {
    let to_create = DEFAULT_VISUAL_POOL_CAPACITY.min(pool.max_size);
    for _ in 0..to_create {
        if pool.available.len() < pool.max_size {
            let entity = commands
                .spawn((
                    Visibility::Hidden,
                    PooledVisualEffect,
                    Name::new("PooledVisualEffect"),
                ))
                .id();
            pool.available.push(entity);
        }
    }
}

// End of simulation/src/world.rs v19.7 — Bounded freelist pool implemented.
// Thunder locked in. Yoi ⚡