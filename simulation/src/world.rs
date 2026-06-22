/*!
 * Powrush-MMO Simulation World & Advanced Particle Effects
 *
 * v19.5 Optimized Particle Pool Memory Allocation (PATSAGi + Ra-Thor)
 * — Pre-allocated capacity + prewarm system to avoid runtime allocations
 * — High-frequency spawn friendly with minimal memory churn
 *
 * AG-SML v1.0 Sovereign Mercy License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy_hanabi::prelude::*;

// ... (previous code) ...

// ============================================================================
// OPTIMIZED PARTICLE VISUAL POOL (v19.5)
// ============================================================================

const DEFAULT_VISUAL_POOL_CAPACITY: usize = 64;

#[derive(Resource)]
pub struct ParticleVisualPool {
    pub available: Vec<Entity>,
}

impl Default for ParticleVisualPool {
    fn default() -> Self {
        Self {
            available: Vec::with_capacity(DEFAULT_VISUAL_POOL_CAPACITY),
        }
    }
}

/// Pre-warm the visual effect pool by creating dormant entities upfront.
/// Call this on Startup to avoid allocations during intense gameplay moments.
pub fn prewarm_visual_pool(
    mut commands: Commands,
    mut pool: ResMut<ParticleVisualPool>,
) {
    for _ in 0..DEFAULT_VISUAL_POOL_CAPACITY {
        let entity = commands.spawn((
            Visibility::Hidden,
            PooledVisualEffect,
            Name::new("PooledVisualEffect"),
        )).id();

        pool.available.push(entity);
    }
}

// ... (rest of the pooling logic from v19.4 remains) ...

// End of simulation/src/world.rs v19.5 — Pool memory allocation optimized with pre-allocation + prewarm.
// Thunder locked in. Yoi ⚡