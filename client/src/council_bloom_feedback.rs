/*!
 * Council Bloom Feedback — Client-Side
 *
 * Modern optimized implementation using bevy_hanabi
 * - Cached EffectAsset for performance
 * - CouncilBloomReceived event driven
 * - Intensity & attunement reactive particles + light
 * - Concurrent limit + automatic cleanup
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::replication::{CouncilBloomPayload, CouncilBloomReceived};

/// Marker for active bloom effects
#[derive(Component)]
pub struct CouncilBloomEffect {
    pub intensity: f32,
    pub timer: Timer,
}

/// Cached particle asset resource
#[derive(Resource)]
pub struct CouncilBloomParticleAssets {
    pub effect_handle: Handle<EffectAsset>,
}

/// Plugin for Council Bloom client feedback
pub struct CouncilBloomFeedbackPlugin;

impl Plugin for CouncilBloomFeedbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CouncilBloomReceived>()
           .add_systems(Startup, setup_council_bloom_particles)
           .add_systems(Update, (
               process_council_bloom_received,
               despawn_old_bloom_effects,
           ).chain());
    }
}

/// One-time setup: create and cache the bloom particle effect
fn setup_council_bloom_particles(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut commands: Commands,
) {
    let effect = effects.add(
        EffectAsset::new(4096, false, Module::default())
            .with_name("council_bloom")
            .init(InitPositionSphereModifier {
                center: Vec3::ZERO,
                radius: 2.5,
                dimension: ShapeDimension::Volume,
            })
            .init(InitVelocitySphereModifier {
                center: Vec3::ZERO,
                speed: 9.0,
            })
            .init(InitLifetimeModifier { lifetime: 3.8 })
            .update(AccelModifier::constant(Vec3::new(0.0, 3.0, 0.0)))
            .render(ColorOverLifetimeModifier::gradient(Gradient::new(vec![
                (0.0, Color::srgba(0.3, 0.9, 0.5, 0.0)),
                (0.15, Color::srgba(0.5, 1.0, 0.6, 0.95)),
                (0.75, Color::srgba(0.4, 0.85, 0.9, 0.7)),
                (1.0, Color::srgba(0.2, 0.6, 0.9, 0.0)),
            ])))
            .render(SizeOverLifetimeModifier::new(Gradient::new(vec![
                (0.0, 0.6),
                (0.25, 2.8),
                (1.0, 0.0),
            ]))),
    );

    commands.insert_resource(CouncilBloomParticleAssets { effect_handle: effect });
    info!("[Client] Council Bloom particle assets cached");
}

/// Process incoming blooms and spawn effects (with concurrency guard)
fn process_council_bloom_received(
    mut events: EventReader<CouncilBloomReceived>,
    mut commands: Commands,
    assets: Res<CouncilBloomParticleAssets>,
    active: Query<&CouncilBloomEffect>,
) {
    let active_count = active.iter().count();

    for event in events.read() {
        let p = &event.payload;
        if p.bloom_activated && active_count < 4 {
            spawn_council_bloom_particles(&mut commands, &assets, Vec3::ZERO, p);
        }
    }
}

/// Spawn optimized bloom particles + light
fn spawn_council_bloom_particles(
    commands: &mut Commands,
    assets: &CouncilBloomParticleAssets,
    position: Vec3,
    payload: &CouncilBloomPayload,
) {
    let intensity = payload.bloom_amplification_multiplier.max(1.0);
    let color = if payload.collective_attunement_score > 0.8 {
        Color::srgb(0.35, 0.95, 0.55)
    } else {
        Color::srgb(0.5, 0.75, 1.0)
    };

    commands.spawn((
        Name::new(format!("CouncilBloom_{}", payload.session_id)),
        ParticleEffect::new(assets.effect_handle.clone()),
        Transform::from_translation(position),
        CouncilBloomEffect {
            intensity,
            timer: Timer::from_seconds(5.5, TimerMode::Once),
        },
        PointLight {
            color,
            intensity: 1400.0 * intensity,
            range: 28.0,
            shadows_enabled: false,
            ..default()
        },
    ));
}

/// Cleanup expired bloom effects
fn despawn_old_bloom_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut CouncilBloomEffect)>,
) {
    for (entity, mut effect) in &mut query {
        effect.timer.tick(time.delta());
        if effect.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// End of modern CouncilBloomFeedback module
