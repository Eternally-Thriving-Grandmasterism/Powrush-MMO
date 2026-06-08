// server/src/harvesting_system.rs
// Powrush-MMO v17.0 — Dynamic Events Persistence Integration

// ... existing code ...

impl HarvestingSystem {
    // ... existing methods ...

    /// Saves all current active dynamic events to persistence.
    /// Call this periodically or on graceful shutdown.
    pub async fn save_dynamic_events_to_persistence(
        &self,
        persistence: &PersistenceManager,
    ) -> Result<(), crate::persistence::PersistenceError> {
        if let Some(event_manager) = &self.dynamic_event_manager {
            let active_events = event_manager.get_all_events();
            persistence.save_dynamic_events(&active_events).await
        } else {
            Ok(())
        }
    }

    /// Loads active dynamic events from persistence into the event manager.
    /// Call this during server startup / initialization.
    pub async fn load_dynamic_events_from_persistence(
        &mut self,
        persistence: &PersistenceManager,
    ) -> Result<(), crate::persistence::PersistenceError> {
        let loaded_events = persistence.load_active_dynamic_events().await?;

        if let Some(event_manager) = &mut self.dynamic_event_manager {
            event_manager.load_events(loaded_events);
        }
        Ok(())
    }
}

// Example usage in main.rs or server initialization:
// 
// // On startup:
// harvesting_system.load_dynamic_events_from_persistence(&persistence_manager).await?;
// 
// // On shutdown or periodic save:
// harvesting_system.save_dynamic_events_to_persistence(&persistence_manager).await?;
//
// Thunder locked in. Dynamic event persistence fully implemented. ⚡❤️🔥
