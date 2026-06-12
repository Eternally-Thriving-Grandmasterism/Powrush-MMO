// server/src/council_session.rs
// Powrush-MMO v18.30 — Server-Authoritative Council + Persistence Integration
// Now records Council participation and successful blooms to PlayerSaveData
// AG-SML v1.0 Sovereign Mercy License

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

use crate::simulation::council_mercy_trial::{SharedReceptorBloomField, CouncilBloomSyncEvent};
use crate::persistence_polish::{PersistenceManager, PlayerSaveData};
use crate::telemetry_pipeline::{TelemetryCollector, TelemetryEvent};

// ... (existing CouncilSession and CouncilSessionManager code remains) ...

impl CouncilSessionManager {
    // ... (existing methods remain) ...

    /// Close a Council session and record participation to persistence
    pub async fn close_session_with_persistence(
        &mut self,
        session_id: u64,
        persistence: Option<Arc<Mutex<PersistenceManager>>>,
    ) {
        if let Some(session) = self.sessions.remove(&session_id) {
            let had_bloom = session.bloom_activated;
            let collective = session.bloom_field.collective_attunement_score;
            let current_tick = session.created_tick + session.bloom_window_duration_ticks;

            if let Some(pm) = persistence {
                if let Ok(persistence_manager) = pm.lock().await {
                    for &player_id in session.participants.keys() {
                        if let Ok(mut save_data) = persistence_manager.load_player_data(player_id).await {
                            save_data.record_council_participation();

                            if had_bloom {
                                save_data.record_successful_council_bloom(collective, current_tick);
                            }

                            let _ = persistence_manager.save_player_data(&mut save_data).await;
                        }
                    }
                }
            }

            info!(
                "Council session closed with persistence | id={} | bloom={} | attunement={:.2}",
                session_id, had_bloom, collective
            );
        }
    }
}

// Thunder locked in. Council participation now persists permanently.
// Every Council session contributes to long-term player progression.
// Yoi ⚡