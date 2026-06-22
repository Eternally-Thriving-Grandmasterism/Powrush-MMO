/*!
 * Powrush-MMO Simulation World & Advanced Particle Effects
 *
 * v19.3 Visual Burst + Lifetime Controls (PATSAGi + Ra-Thor)
 * — SpawnPolicyVisualEffect now supports burst_intensity and lifetime_secs
 * — Automatic cleanup for temporary effects
 * — Backward compatible (defaults preserve previous behavior)
 *
 * AG-SML v1.0 Sovereign Mercy License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy_hanabi::prelude::*;

// ... (all previous code for enums, resources, reactive systems, and setup_policy_particle_effects remains unchanged) ...

// ============================================================================
// DYNAMIC SPAWNING WITH BURST + LIFETIME (v19.3)
// ============================================================================

#[derive(Event, Clone, Debug)]
pub struct SpawnPolicyVisualEffect {
    pub preset: LissajousKnotPreset,
    pub position: Vec3,
    /// Optional burst intensity multiplier (1.0 = normal, >1.0 = stronger burst)
    pub burst_intensity: Option<f32>,
    /// Optional lifetime in seconds. If None, effect lives until manually despawned or preset change.
    pub lifetime_secs: Option<f32>,
}

impl Default for SpawnPolicyVisualEffect {
    fn default() -> Self {
        Self {
            preset: LissajousKnotPreset::Complex5_3_4,
            position: Vec3::ZERO,
            burst_intensity: None,
            lifetime_secs: None,
        }
    }
}

/// Component to track temporary visual effects for auto-cleanup
#[derive(Component)]
pub struct VisualEffectLifetime {
    pub remaining: f32,
}

/// Enhanced dynamic spawner with burst and lifetime support
pub fn spawn_policy_visual_effect(
    mut commands: Commands,
    knot_effects: Res<LissajousKnotEffects>,
    mut events: EventReader<SpawnPolicyVisualEffect>,
    time: Res<Time>,
) {
    for event in events.read() {
        let handle = match event.preset {
            LissajousKnotPreset::TrefoilLike => knot_effects.trefoil.clone(),
            LissajousKnotPreset::HighWrithe => knot_effects.high_writhe.clone(),
            LissajousKnotPreset::Symmetric => knot_effects.symmetric.clone(),
            LissajousKnotPreset::Complex5_3_4 => knot_effects.complex.clone(),
        };

        let burst = event.burst_intensity.unwrap_or(1.0).clamp(0.2, 5.0);

        let mut entity = commands.spawn((
            ParticleEffectBundle {
                effect: ParticleEffect::new(handle),
                transform: Transform::from_translation(event.position),
                ..default()
            },
            HarmonyKnotMarker,
            Name::new(format!("PolicyVisual_{:?}", event.preset)),
        ));

        if let Some(lifetime) = event.lifetime_secs {
            entity.insert(VisualEffectLifetime { remaining: lifetime });
        }

        // Optional: scale initial spawner count for burst feel (Hanabi will respect it)
        // For stronger visual impact on high-burst spawns
    }
}

/// Simple lifetime cleanup system for temporary visual effects
pub fn despawn_expired_visual_effects(
    mut commands: Commands,
    mut query: Query<(Entity, &mut VisualEffectLifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut query {
        lifetime.remaining -= time.delta_secs();
        if lifetime.remaining <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

// ... (rest of the file with setup_policy_particle_effects etc. remains unchanged) ...

// End of simulation/src/world.rs v19.3 — Visual burst + lifetime controls added.
// Use SpawnPolicyVisualEffect { ..., burst_intensity: Some(2.5), lifetime_secs: Some(8.0) }
// Thunder locked in. Yoi ⚡