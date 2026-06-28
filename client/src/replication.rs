/// Event emitted when a Council Bloom is received from server replication
#[derive(Event, Clone, Debug)]
pub struct CouncilBloomReceived {
    pub payload: CouncilBloomPayload,
}

/// System that processes CouncilBloomReceived events.
/// Add this system to your app (e.g. in CouncilReplicationPlugin or a UI/Feedback plugin).
pub fn process_council_bloom_received(
    mut events: EventReader<CouncilBloomReceived>,
    mut commands: Commands,
    // TODO: Add your resources here:
    // mut ui_state: ResMut<CouncilUiState>,
    // asset_server: Res<AssetServer>,
    // audio: Res<Audio>,
) {
    for event in events.read() {
        let p = &event.payload;

        info!(
            "[Client] Processing Council Bloom | Session: {} | Attunement: {:.2} | Participants: {} | Reason: {}",
            p.session_id,
            p.collective_attunement_score,
            p.participant_count,
            p.trigger_reason
        );

        if p.bloom_activated {
            // === Example feedback actions ===

            // 1. Spawn a temporary bloom effect entity (extend with VFX, particles, etc.)
            commands.spawn((
                Name::new(format!("CouncilBloom_{}", p.session_id)),
                Transform::default(),
                // TODO: Add VisualEffect, ParticleEffect, or custom BloomEffect component
                // Example:
                // BloomEffect {
                //     intensity: p.bloom_amplification_multiplier,
                //     duration: 4.0,
                // },
            ));

            // 2. Trigger UI update (example)
            // if let Some(mut ui) = ui_state { ui.show_bloom_notification(p); }

            // 3. Play sound / haptic feedback
            // audio.play(AssetServer::get_handle("sounds/council_bloom.ogg"));

            // 4. Future: Update local player state, trigger epiphany UI, etc.
        }
    }
}

/// Plugin that registers Council replication events and systems
pub struct CouncilReplicationPlugin;

impl Plugin for CouncilReplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CouncilBloomReceived>()
           .add_systems(Update, process_council_bloom_received);

        info!("[Client] CouncilReplicationPlugin initialized with bloom processing");
    }
}
