/// System that cleans up expired Council Bloom particle effects
pub fn despawn_old_bloom_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut CouncilBloomEffect)>,
) {
    for (entity, mut bloom_effect) in &mut query {
        bloom_effect.timer.tick(time.delta());

        if bloom_effect.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Optimized spawn function with performance guards
pub fn spawn_council_bloom_particles(
    commands: &mut Commands,
    assets: &CouncilBloomParticleAssets,
    position: Vec3,
    payload: &CouncilBloomPayload,
    active_blooms: usize, // pass current count for limiting
) {
    // Performance guard: limit concurrent bloom effects
    if active_blooms >= 4 {
        return; // Skip spawning if too many active blooms
    }

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
            shadows_enabled: false, // Performance: shadows are expensive
            ..default()
        },
    ));
}

pub fn process_council_bloom_received(
    mut events: EventReader<CouncilBloomReceived>,
    mut commands: Commands,
    particle_assets: Res<CouncilBloomParticleAssets>,
    active_query: Query<&CouncilBloomEffect>, // count active blooms
) {
    let active_blooms = active_query.iter().count();

    for event in events.read() {
        let p = &event.payload;

        if p.bloom_activated {
            spawn_council_bloom_particles(
                &mut commands,
                &particle_assets,
                Vec3::ZERO,
                p,
                active_blooms,
            );
        }
    }
}
