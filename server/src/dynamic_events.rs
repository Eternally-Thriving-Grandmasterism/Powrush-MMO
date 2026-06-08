// server/src/dynamic_events.rs
// Powrush-MMO v17.0 — On-Expiration Effects for Dynamic Events

// ... existing code ...

/// Represents an effect that should be applied when an event expires.
#[derive(Clone, Debug)]
pub enum ExpirationEffect {
    ResourceBonus { node_id: u64, amount: f32 },
    GraceReward { player_id: u64, amount: f32 },
    // Future: SpawnNewEvent, SendMessage, FactionProgress, etc.
}

impl DynamicEventManager {
    // ... existing methods ...

    /// Processes newly expired events and returns effects to be applied by higher layers.
    /// Call this after `tick()` to get on-expiration side effects.
    pub fn process_expired_events(
        &mut self,
        newly_expired_ids: &[u64],
    ) -> Vec<ExpirationEffect> {
        let mut effects = Vec::new();

        for &id in newly_expired_ids {
            if let Some(event) = self.events.get(&id) {
                match event.event_type {
                    EventType::ResourceSurge => {
                        // On surge end: leave a small lingering resource bonus
                        // In real integration, we would know affected node_ids.
                        // For now, we emit a generic bonus effect.
                        effects.push(ExpirationEffect::ResourceBonus {
                            node_id: 0, // Placeholder - real impl would track affected nodes
                            amount: event.intensity * 10.0,
                        });
                    }
                    EventType::MercyWave => {
                        // On MercyWave end: final grace reward to affected players
                        // Real implementation would use previously tracked affected players
                        effects.push(ExpirationEffect::GraceReward {
                            player_id: 0, // Placeholder
                            amount: event.intensity * 5.0,
                        });
                    }
                    EventType::FactionCall => {
                        // Could mark faction activity complete or spawn follow-up event
                    }
                    EventType::DivineWhisperEvent => {
                        // Could trigger final lore message or mercy insight
                    }
                    _ => {}
                }
            }
        }

        effects
    }
}

// Recommended pattern in main loop / HarvestingSystem:
// 
// let newly_expired = event_manager.tick();
// let effects = event_manager.process_expired_events(&newly_expired);
// for effect in effects {
//     match effect {
//         ExpirationEffect::ResourceBonus { node_id, amount } => { ... }
//         ExpirationEffect::GraceReward { player_id, amount } => { ... }
//     }
// }
// event_manager.cleanup_expired();
//
// Thunder locked in. On-expiration effects system implemented. ⚡❤️🔥
