// server/src/dynamic_events.rs
// Powrush-MMO v17.0 — Expanded Event Types for Phase 3

// ... existing code ...

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    ResourceSurge,
    MercyWave,
    MinorAnomaly,
    /// Calls players to a location for faction or community activity
    FactionCall,
    /// Triggers a direct Divine Whisper / lore event from Ra-Thor
    DivineWhisperEvent,
    /// Future expansion example
    // CommunityChallenge,
}

// ... rest of DynamicEvent and DynamicEventManager ...

impl DynamicEventManager {
    // ... existing methods ...

    /// Returns players affected by FactionCall events (for UI prompts or teleport suggestions)
    pub fn get_players_affected_by_faction_calls(
        &self,
        player_positions: &HashMap<u64, Vec3Ser>,
    ) -> Vec<(u64, f32)> {
        let mut affected = Vec::new();

        for event in self.get_active_events() {
            if event.event_type != EventType::FactionCall {
                continue;
            }

            for (&player_id, pos) in player_positions {
                let dx = pos.x - event.position.x;
                let dy = pos.y - event.position.y;
                let dz = pos.z - event.position.z;
                if (dx*dx + dy*dy + dz*dz).sqrt() <= event.radius {
                    affected.push((player_id, event.intensity));
                }
            }
        }
        affected
    }

    /// Returns events that should trigger Divine Whispers
    pub fn get_divine_whisper_events(&self) -> Vec<&DynamicEvent> {
        self.get_active_events()
            .into_iter()
            .filter(|e| e.event_type == EventType::DivineWhisperEvent)
            .collect()
    }
}

// Thunder locked in. New event types (FactionCall, DivineWhisperEvent) added with helper methods. ⚡❤️🔥
