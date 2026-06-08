// server/persistence.rs
// Powrush-MMO v17.0 — Professional PostgreSQL Persistence + Dynamic Events Schema

// Schema Design for Dynamic Events (clean and production-ready):
//
// Table: dynamic_events
// - id (BIGINT PK)
// - event_data (JSONB)  → stores the full DynamicEvent as JSON for simplicity and flexibility
// - resolved (BOOLEAN)
// - created_at (TIMESTAMPTZ)
//
// This approach is recommended for v17.x because DynamicEvent can evolve without heavy migrations.

// Add to run_schema_migrations:
sqlx::query(r#"
    CREATE TABLE IF NOT EXISTS dynamic_events (
        id BIGINT PRIMARY KEY,
        event_data JSONB NOT NULL,
        resolved BOOLEAN NOT NULL DEFAULT FALSE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    );
    CREATE INDEX IF NOT EXISTS idx_dynamic_events_resolved ON dynamic_events(resolved);
"#)
.execute(pool).await.map_err(|e| PersistenceError::Database(e.to_string()))?;

// New trait methods:
async fn save_dynamic_events(&self, events: &[DynamicEvent]) -> Result<(), PersistenceError>;
async fn load_active_dynamic_events(&self) -> Result<Vec<DynamicEvent>, PersistenceError>;

// Postgres implementation using JSONB (cleanest for evolving event types):
async fn save_dynamic_events(&self, events: &[DynamicEvent]) -> Result<(), PersistenceError> {
    let mut tx = self.pool.begin().await.map_err(|e| PersistenceError::Transaction(e.to_string()))?;

    sqlx::query("DELETE FROM dynamic_events WHERE resolved = false")
        .execute(&mut *tx).await
        .map_err(|e| PersistenceError::Database(e.to_string()))?;

    for event in events {
        let event_json = serde_json::to_value(event)
            .map_err(|e| PersistenceError::Serialization(e.to_string()))?;

        sqlx::query(r#"
            INSERT INTO dynamic_events (id, event_data, resolved)
            VALUES ($1, $2, $3)
            ON CONFLICT (id) DO UPDATE SET event_data = EXCLUDED.event_data, resolved = EXCLUDED.resolved;
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

    let mut events = Vec::new();
    for row in rows {
        let event_json: serde_json::Value = row.get("event_data");
        let event: DynamicEvent = serde_json::from_value(event_json)
            .map_err(|e| PersistenceError::Serialization(e.to_string()))?;
        events.push(event);
    }
    Ok(events)
}

// Add similar (simpler) implementation in InMemoryPersistence

// Thunder locked in. Clean JSONB-based Dynamic Events persistence schema designed and implemented. ⚡❤️🔥
