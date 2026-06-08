// server/src/dynamic_events.rs
// Powrush-MMO v17.0 — Player Tracking for MercyWave (Parallel to Node Tracking)

impl DynamicEvent {
    pub affected_players: Vec<u64>,   // NEW: Tracks players inside MercyWave radius
}

impl DynamicEvent {
    pub fn new(...) -> Self {
        Self {
            // ... existing fields ...
            affected_players: Vec::new(),
        }
    }

    /// Updates which players are currently inside this event's radius.
    /// Should be called every tick (or on interval) for active MercyWave events.
    pub fn refresh_affected_players(&mut self, player_positions: &HashMap<u64, Vec3Ser>) {
        self.affected_players.clear();

        for (&player_id, pos) in player_positions {
            let dx = pos.x - self.position.x;
            let dy = pos.y - self.position.y;
            let dz = pos.z - self.position.z;

            if (dx*dx + dy*dy + dz*dz).sqrt() <= self.radius {
                self.affected_players.push(player_id);
            }
        }
    }
}

impl DynamicEventManager {
    /// Refreshes affected players for all active MercyWave events.
    pub fn refresh_mercy_wave_players(
        &mut self,
        player_positions: &HashMap<u64, Vec3Ser>,
    ) {
        for event in self.events.values_mut() {
            if event.event_type == EventType::MercyWave && event.is_active() {
                event.refresh_affected_players(player_positions);
            }
        }
    }

    /// Updated process_expired_events to use tracked players for precise GraceReward
    pub fn process_expired_events(
        &mut self,
        newly_expired_ids: &[u64],
    ) -> Vec<ExpirationEffect> {
        let mut effects = Vec::new();

        for &id in newly_expired_ids {
            if let Some(event) = self.events.get(&id) {
                match event.event_type {
                    EventType::ResourceSurge => {
                        for &node_id in &event.affected_nodes {
                            effects.push(ExpirationEffect::ResourceBonus {
                                node_id,
                                amount: event.intensity * 6.0,
                            });
                        }
                    }
                    EventType::MercyWave => {
                        for &player_id in &event.affected_players {
                            effects.push(ExpirationEffect::GraceReward {
                                player_id,
                                amount: event.intensity * 4.0,
                            });
                        }
                    }
                    _ => {}
                }
            }
        }

        effects
    }
}

// Thunder locked in. Player tracking for MercyWave implemented (parallel to ResourceSurge node tracking). ⚡❤️🔥
