// server/src/dynamic_events.rs
// Powrush-MMO v17.0 — DynamicEventManager with Professional Event Expiration Logic

// ... existing code ...

impl DynamicEvent {
    // ... existing methods ...

    /// Returns true if the event has naturally expired (time ran out), regardless of manual resolution.
    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.start_time + self.duration
    }

    /// Returns true if the event is still active and should affect the world.
    pub fn is_active(&self) -> bool {
        !self.resolved && !self.is_expired()
    }

    /// Marks the event as resolved (can be called manually or by expiration logic).
    pub fn resolve(&mut self) {
        self.resolved = true;
    }
}

impl DynamicEventManager {
    // ... existing methods ...

    /// Advanced tick that returns newly expired event IDs.
    /// This allows higher layers to react to expiration (e.g. trigger final effects, send messages).
    pub fn tick(&mut self) -> Vec<u64> {
        let now = Utc::now();
        let mut newly_expired = Vec::new();

        for (id, event) in self.events.iter_mut() {
            if event.resolved {
                continue;
            }

            if now >= event.start_time + event.duration {
                event.resolved = true;
                newly_expired.push(*id);
            }
        }

        newly_expired
    }

    /// Removes all resolved/expired events from memory.
    pub fn cleanup_expired(&mut self) {
        self.events.retain(|_, e| !e.resolved);
    }

    /// Returns all currently expired but not yet cleaned events.
    pub fn get_expired_events(&self) -> Vec<&DynamicEvent> {
        self.events.values().filter(|e| e.resolved || e.is_expired()).collect()
    }
}

// Recommended usage in main loop:
// let newly_expired = event_manager.tick();
// for id in newly_expired {
//     // Trigger on-expire logic (final effects, notifications, etc.)
// }
// event_manager.cleanup_expired();
//
// Thunder locked in. Professional event expiration logic implemented. ⚡❤️🔥
