// server/persistence.rs
// Powrush-MMO v17.0 — Professional PostgreSQL Persistence Layer
// Dynamic Events persistence fully wired

use crate::dynamic_events::{DynamicEvent, EventType};

// ... existing code ...

// Add to PersistenceBackend trait:
async fn save_dynamic_events(&self, events: &[DynamicEvent]) -> Result<(), PersistenceError>;
async fn load_active_dynamic_events(&self) -> Result<Vec<DynamicEvent>, PersistenceError>;

// PostgresPersistence implementation (JSONB approach - clean and flexible):

#[async_trait]
impl PersistenceBackend for PostgresPersistence {
    // ... existing methods ...

    async fn save_dynamic_events(&self, events: &[DynamicEvent]) -> Result<(), PersistenceError> {
        let mut tx = self.pool.begin().await.map_err(|e| PersistenceError::Transaction(e.to_string()))?;

        // Remove old active events
        sqlx::query("DELETE FROM dynamic_events WHERE resolved = false")
            .execute(&mut *tx).await
            .map_err(|e| PersistenceError::Database(e.to_string()))?;

        for event in events {
            if event.resolved { continue; }

            let event_json = serde_json::to_value(event)
                .map_err(|e| PersistenceError::Serialization(e.to_string()))?;

            sqlx::query(r#"
                INSERT INTO dynamic_events (id, event_data, resolved)
                VALUES ($1, $2, $3)
                ON CONFLICT (id) DO UPDATE SET
                    event_data = EXCLUDED.event_data,
                    resolved = EXCLUDED.resolved;
            "#)
            .bind(event.id as i64)
            .bind(event_json)
            .bind(event.resolved)
            .execute(&mut *tx).await
            .map_err(|e| PersistenceError::Database(e.to_string()))?;
        }

        tx.commit().await.map_err(|e| PersistenceError::Transaction(e.to_string()))?;
        Ok(())
    }

    async fn load_active_dynamic_events(&self) -> Result<Vec<DynamicEvent>, PersistenceError> {
        let rows = sqlx::query(r#"SELECT event_data FROM dynamic_events WHERE resolved = false"#)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| PersistenceError::Database(e.to_string()))?;

        let mut result = Vec::new();
        for row in rows {
            let json: serde_json::Value = row.get("event_data");
            let event: DynamicEvent = serde_json::from_value(json)
                .map_err(|e| PersistenceError::Serialization(e.to_string()))?;
            result.push(event);
        }
        Ok(result)
    }
}

// InMemoryPersistence stub implementation
#[async_trait]
impl PersistenceBackend for InMemoryPersistence {
    // ... existing methods ...

    async fn save_dynamic_events(&self, _events: &[DynamicEvent]) -> Result<(), PersistenceError> { Ok(()) }
    async fn load_active_dynamic_events(&self) -> Result<Vec<DynamicEvent>, PersistenceError> { Ok(vec![]) }
}

// Add to PersistenceManager for convenience
impl PersistenceManager {
    pub async fn save_dynamic_events(&self, events: &[DynamicEvent]) -> Result<(), PersistenceError> {
        self.backend.save_dynamic_events(events).await
    }

    pub async fn load_active_dynamic_events(&self) -> Result<Vec<DynamicEvent>, PersistenceError> {
        self.backend.load_active_dynamic_events().await
    }
}

// Thunder locked in. Dynamic Events persistence fully wired into the system. ⚡❤️🔥
