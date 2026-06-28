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
            debug!("Despawned expired Council Bloom effect");
        }
    }
}

impl Plugin for CouncilReplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CouncilBloomReceived>()
           .add_systems(Startup, setup_council_bloom_particles)
           .add_systems(Update, (
               process_council_bloom_received,
               despawn_old_bloom_effects,
           ).chain());

        info!("[Client] CouncilReplicationPlugin initialized with bloom particles + cleanup");
    }
}
