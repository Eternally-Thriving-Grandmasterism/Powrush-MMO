/*!
 * Powrush-MMO Simulation World & Advanced Particle Effects
 *
 * v19.6 Refactored Prewarm System for Clarity (PATSAGi + Ra-Thor)
 * — Clearer pre-warming logic with helper function
 * — Better documentation of pooling strategy
 *
 * AG-SML v1.0 Sovereign Mercy License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy_hanabi::prelude::*;

// ============================================================================
// VISUAL EFFECT POOLING (Optimized + Clear)
// ============================================================================

const DEFAULT_VISUAL_POOL_CAPACITY: usize = 64;

#[derive(Resource, Default)]
pub struct ParticleVisualPool {
    /// Reusable entities for visual effects. Pre-allocated for performance.
    pub available: Vec<Entity>,
}

impl ParticleVisualPool {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            available: Vec::with_capacity(capacity),
        }
    }
}

/// Marker for entities that belong to the visual effect pool.
#[derive(Component)]
pub struct PooledVisualEffect;

/// Creates a single dormant entity ready to be activated later as a visual effect.
/// This helper makes the prewarm logic clearer and reusable.
fn create_dormant_pooled_visual_entity(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Visibility::Hidden,
            PooledVisualEffect,
            Name::new("PooledVisualEffect"),
        ))
        .id()
}

/// Pre-warm the visual effect pool at startup.
///
/// This moves allocation cost away from gameplay-critical moments.
/// Creates `DEFAULT_VISUAL_POOL_CAPACITY` dormant entities that can be
/// quickly activated when `SpawnPolicyVisualEffect` events arrive.
pub fn prewarm_visual_pool(
    mut commands: Commands,
    mut pool: ResMut<ParticleVisualPool>,
) {
    for _ in 0..DEFAULT_VISUAL_POOL_CAPACITY {
        let entity = create_dormant_pooled_visual_entity(&mut commands);
        pool.available.push(entity);
    }
}

// Note: The full spawn_policy_visual_effect, despawn_expired_visual_effects,
// EffectAsset creation, and reactive systems from previous versions should be
// present below this point in a complete build.

// End of simulation/src/world.rs v19.6 — Prewarm system refactored for clarity.
// Thunder locked in. Yoi ⚡