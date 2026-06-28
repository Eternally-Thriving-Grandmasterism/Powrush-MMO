/// Resource that holds the cached Council Bloom particle effect asset
#[derive(Resource)]
pub struct CouncilBloomParticleAssets {
    pub effect_handle: Handle<EffectAsset>,
}

/// Initializes the cached Council Bloom particle effect (call once at startup)
pub fn setup_council_bloom_particles(
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

    commands.insert_resource(CouncilBloomParticleAssets {
        effect_handle: effect,
    });

    info!("[Client] Council Bloom particle effect cached and ready");
}

/// Spawns Council Bloom particles using the cached effect handle (optimized)
pub fn spawn_council_bloom_particles(
    commands: &mut Commands,
    assets: &CouncilBloomParticleAssets,
    position: Vec3,
    payload: &CouncilBloomPayload,
) {
    let intensity = payload.bloom_amplification_multiplier.max(1.0);

    commands.spawn((
        Name::new(format!("CouncilBloomParticles_{}", payload.session_id)),
        ParticleEffect::new(assets.effect_handle.clone()),
        Transform::from_translation(position),
        CouncilBloomEffect {
            intensity,
            timer: Timer::from_seconds(5.5, TimerMode::Once),
        },
        PointLight {
            color: if payload.collective_attunement_score > 0.8 {
                Color::srgb(0.35, 0.95, 0.55)
            } else {
                Color::srgb(0.5, 0.75, 1.0)
            },
            intensity: 1400.0 * intensity,
            range: 28.0,
            ..default()
        },
    ));
}

pub fn process_council_bloom_received(
    mut events: EventReader<CouncilBloomReceived>,
    mut commands: Commands,
    particle_assets: Res<CouncilBloomParticleAssets>,
) {
    for event in events.read() {
        let p = &event.payload;

        if p.bloom_activated {
            info!("[Client] Council Bloom activated — spawning optimized particles");
            spawn_council_bloom_particles(&mut commands, &particle_assets, Vec3::ZERO, p);
        }
    }
}

impl Plugin for CouncilReplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CouncilBloomReceived>()
           .add_systems(Startup, setup_council_bloom_particles)
           .add_systems(Update, process_council_bloom_received);

        info!("[Client] CouncilReplicationPlugin initialized (with cached particles)");
    }
}
