use bevy_hanabi::prelude::*;

/// Marker component for Council Bloom particle effects
#[derive(Component)]
pub struct CouncilBloomEffect {
    pub intensity: f32,
    pub timer: Timer,
}

/// Spawns beautiful particle effects for an activated Council Bloom
pub fn spawn_council_bloom_particles(
    commands: &mut Commands,
    effects: &mut ResMut<Assets<EffectAsset>>,
    position: Vec3,
    payload: &CouncilBloomPayload,
) {
    let intensity = payload.bloom_amplification_multiplier.max(1.0);
    let color = if payload.collective_attunement_score > 0.8 {
        Color::srgb(0.4, 0.9, 0.6) // Strong mercy green-gold
    } else {
        Color::srgb(0.6, 0.7, 1.0) // Softer blue-white
    };

    // Create a high-quality bloom particle effect
    let effect = effects.add(
        EffectAsset::new(
            4096,
            false,
            Module::default(),
        )
        .with_name("council_bloom")
        .init(InitPositionSphereModifier {
            center: Vec3::ZERO,
            radius: 2.0 * intensity,
            dimension: ShapeDimension::Volume,
        })
        .init(InitVelocitySphereModifier {
            center: Vec3::ZERO,
            speed: 8.0 * intensity,
        })
        .init(InitLifetimeModifier {
            lifetime: 3.5,
        })
        .update(AccelModifier::constant(Vec3::new(0.0, 2.5, 0.0)))
        .render(ColorOverLifetimeModifier::gradient(Gradient::new(vec![
            (0.0, color.with_alpha(0.0)),
            (0.2, color.with_alpha(0.9)),
            (0.8, color.with_alpha(0.6)),
            (1.0, color.with_alpha(0.0)),
        ])))
        .render(SizeOverLifetimeModifier::new(Gradient::new(vec![
            (0.0, 0.8),
            (0.3, 2.2 * intensity),
            (1.0, 0.0),
        ]))),
    );

    commands.spawn((
        Name::new(format!("CouncilBloomParticles_{}", payload.session_id)),
        ParticleEffect::new(effect),
        Transform::from_translation(position),
        CouncilBloomEffect {
            intensity,
            timer: Timer::from_seconds(5.0, TimerMode::Once),
        },
        // Optional: Add a point light for extra glow
        PointLight {
            color: color,
            intensity: 1200.0 * intensity,
            range: 25.0,
            ..default()
        },
    ));
}

pub fn process_council_bloom_received(
    mut events: EventReader<CouncilBloomReceived>,
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    for event in events.read() {
        let p = &event.payload;

        if p.bloom_activated {
            info!("[Client] Council Bloom activated — spawning particles");

            // Spawn beautiful particle burst at world origin or player location
            // TODO: Use actual player/camera position or bloom location
            spawn_council_bloom_particles(&mut commands, &mut effects, Vec3::ZERO, p);
        }
    }
}
