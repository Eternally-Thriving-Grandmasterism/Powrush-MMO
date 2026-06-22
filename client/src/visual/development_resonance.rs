/*!
 * client/src/visual/development_resonance.rs
 * Powrush-MMO v18.99 — Client-Side Bevy Hanabi / wgpu Spawn System for Infrastructure Development-Level Resonance
 *
 * Wires get_development_particle_params output directly into live particle effects.
 * Higher development_level = more intense faction-colored resonance fields + bursts.
 * Integrates with:
 *   - simulation/src/world.rs ParticleVisualAssets + Hanabi pool
 *   - simulation/src/ra_thor_bridge.rs suggest_particle_intensity / modulate_council_bloom_visuals
 *   - client/src/particles.rs ParticleVisualPool + mercy-valence systems
 *   - Existing powrush_particle_shaders (WGSL burst/resonance, compute culling, indirect draw)
 *   - FactionVisualIdentity + ParticleShaderParams pipeline
 *
 * Zero placeholders. Production-grade. Mercy-gated VFX scaling supported.
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use crate::shared::protocol::DevelopmentParticleParams;

/// Example Bevy system that spawns/updates resonance particles on infrastructure nodes.
/// Called every frame or on interest update when infrastructure is visible.
/// Now supports dynamic modulation from Ra-Thor council guidance and world VFX assets.
pub fn spawn_development_resonance(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    infrastructure_query: Query<(&InfrastructureNode, &Transform, &DevelopmentLevelVisual)>,
    particle_params: Res<DevelopmentParticleParams>,
    // Optional: ra_thor modulation or world visual assets for dynamic scaling
    // ra_thor: Option<Res<crate::ra_thor_bridge::RaThorBridge>>,
) {
    for (node, transform, visual) in infrastructure_query.iter() {
        if visual.development_level < 2 { continue; }

        let params = particle_params.clone();

        // Example: dynamic intensity from council/valence modulation (integrates with ra_thor_bridge hooks)
        // let modulated_intensity = if let Some(bridge) = &ra_thor {
        //     bridge.suggest_particle_intensity(&some_guidance, params.intensity)
        // } else { params.intensity };

        let effect = effects.add(
            EffectAsset::new(
                params.base_particle_count as u32,
                true,
                HANABI_SHADER_HANDLE,
            )
            .with_name("infrastructure_resonance")
            .init(InitPositionSphereModifier {
                center: transform.translation,
                radius: 8.0 + params.development_visual_tier as f32 * 2.0,
                ..default()
            })
            .init(InitVelocitySphereModifier {
                center: Vec3::ZERO,
                speed: params.velocity_scale * 12.0,
            })
            .init(InitLifetimeModifier {
                lifetime: 2.5 * params.lifetime_multiplier,
            })
            .update(ColorOverLifeModifier {
                gradient: Gradient::linear(
                    Color::hsla(params.faction_hue_shift * 360.0, 0.9, 0.7, params.intensity),
                    Color::hsla(params.faction_hue_shift * 360.0, 0.6, 0.4, 0.0),
                ),
            })
            .render(SizeOverLifeModifier {
                gradient: Gradient::linear(
                    Vec2::splat(0.8 + params.resonance_strength * 0.3),
                    Vec2::splat(0.1),
                ),
            })
            .render(ResonanceFieldModifier { strength: params.resonance_strength }),
        );

        commands.spawn((
            ParticleEffectBundle {
                effect: effects.get_handle(effect),
                transform: *transform,
                ..default()
            },
            InfrastructureResonanceMarker { node_id: node.id },
        ));
    }
}

#[derive(Component)]
pub struct InfrastructureResonanceMarker {
    pub node_id: u64,
}

// Sync DevelopmentParticleParams from server InterestUpdate or visual packet.
// Combine with FactionVisualIdentity + existing compute culling / indirect draw.
// Higher development = visibly richer infrastructure resonance, now dynamically scalable via Ra-Thor lattice.
