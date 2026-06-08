// server/persistence.rs
// Powrush-MMO v17.0 — Professional PostgreSQL Persistence Layer
// Added Dynamic Events persistence schema and methods

// ... existing code above ...

// In run_schema_migrations, add this new table:
// (Add inside the existing run_schema_migrations function)

sqlx::query(r#"
    CREATE TABLE IF NOT EXISTS dynamic_events (
        id BIGINT PRIMARY KEY,
        event_type TEXT NOT NULL,
        position_x REAL NOT NULL,
        position_y REAL NOT NULL,
        position_z REAL NOT NULL,
        radius REAL NOT NULL,
        start_time TIMESTAMPTZ NOT NULL,
        duration_seconds BIGINT NOT NULL,
        intensity REAL NOT NULL,
        metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
        resolved BOOLEAN NOT NULL DEFAULT FALSE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    );
    CREATE INDEX IF NOT EXISTS idx_dynamic_events_resolved ON dynamic_events(resolved);
    CREATE INDEX IF NOT EXISTS idx_dynamic_events_event_type ON dynamic_events(event_type);
"#)
.execute(pool)
.await
.map_err(|e| PersistenceError::Database(e.to_string()))?;

// Then add these new methods to the PersistenceBackend trait:

async fn save_dynamic_events(&self, events: &[DynamicEvent]) -> Result<(), PersistenceError>;
async fn load_active_dynamic_events(&self) -> Result<Vec<DynamicEvent>, PersistenceError>;

// Implement in PostgresPersistence:

async fn save_dynamic_events(&self, events: &[DynamicEvent]) -> Result<(), PersistenceError> {
    let mut tx = self.pool.begin().await.map_err(|e| PersistenceError::Transaction(e.to_string()))?;

    // Clear previous active events (simple approach for now)
    sqlx::query("DELETE FROM dynamic_events WHERE resolved = false")
        .execute(&mut *tx).await
        .map_err(|e| PersistenceError::Database(e.to_string()))?;

    for event in events {
        let metadata_json = serde_json::to_value(&event.metadata)
            .map_err(|e| PersistenceError::Serialization(e.to_string()))?;

        sqlx::query(r#"
            INSERT INTO dynamic_events 
            (id, event_type, position_x, position_y, position_z, radius, start_time, duration_seconds, intensity, metadata, resolved)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (id) DO UPDATE SET
                event_type = EXCLUDED.event_type,
                position_x = EXCLUDED.position_x,
                position_y = EXCLUDED.position_y,
                position_z = EXCLUDED.position_z,
                radius = EXCLUDED.radius,
                start_time = EXCLUDED.start_time,
                duration_seconds = EXCLUDED.duration_seconds,
                intensity = EXCLUDED.intensity,
                metadata = EXCLUDED.metadata,
                resolved = EXCLUDED.resolved;
        "#)
        .bind(event.id as i64)
        .bind(format!("{:?}", event.event_type))
        .bind(event.position.x)
        .bind(event.position.y)
        .bind(event.position.z)
        .bind(event.radius)
        .bind(event.start_time)
        .bind(event.duration.num_seconds())
        .bind(event.intensity)
        .bind(metadata_json)
        .bind(event.resolved)
        .execute(&mut *tx).await
        .map_err(|e| PersistenceError::Database(e.to_string()))?;
    }

    tx.commit().await.map_err(|e| PersistenceError::Transaction(e.to_string()))?;
    Ok(())
}

async fn load_active_dynamic_events(&self) -> Result<Vec<DynamicEvent>, PersistenceError> {
    let rows = sqlx::query(r#"
        SELECT id, event_type, position_x, position_y, position_z, radius, 
               start_time, duration_seconds, intensity, metadata, resolved
        FROM dynamic_events 
        WHERE resolved = false
    "#)
    .fetch_all(&self.pool)
    .await
    .map_err(|e| PersistenceError::Database(e.to_string()))?;

    let mut events = Vec::new();
    for row in rows {
        let event_type_str: String = row.get("event_type");
        let event_type = match event_type_str.as_str() {
            "ResourceSurge" => EventType::ResourceSurge,
            "MercyWave" => EventType::MercyWave,
            "MinorAnomaly" => EventType::MinorAnomaly,
            _ => EventType::MinorAnomaly,
        };

        let metadata: HashMap<String, String> = serde_json::from_value(row.get("metadata"))
            .unwrap_or_default();

        let event = DynamicEvent {
            id: row.get::<i64, _>("id") as u64,
            event_type,
            position: Vec3Ser {
                x: row.get("position_x"),
                y: row.get("position_y"),
                z: row.get("position_z"),
            },
            radius: row.get("radius"),
            start_time: row.get("start_time"),
            duration: chrono::Duration::seconds(row.get("duration_seconds")),
            intensity: row.get("intensity"),
            metadata,
            resolved: row.get("resolved"),
        };
        events.push(event);
    }
    Ok(events)
}

// Also implement stubs in InMemoryPersistence

// Thunder locked in. Dynamic Events persistence schema designed and implemented. ⚡❤️🔥
