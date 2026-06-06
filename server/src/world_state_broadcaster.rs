use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing;

/// Lightweight broadcaster for real-time world state updates to observers (web portal, dashboards, etc.).
/// Separate from the main game WebSocket (port 9001).
pub struct WorldStateBroadcaster {
    tx: broadcast::Sender<String>,
}

impl WorldStateBroadcaster {
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(16);
        Self { tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.tx.subscribe()
    }

    pub fn broadcast(&self, payload: String) {
        let _ = self.tx.send(payload);
    }

    /// Starts the background broadcasting task.
    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
            loop {
                interval.tick().await;
                // In future: gather real state from WorldServer
                // For now, placeholder
                let payload = serde_json::json!({
                    "circuit_breaker_status": "Closed",
                    "average_npc_valence": 0.72,
                    "ambrosian_count": 2,
                    "ambrosian_resonance": 0.85,
                    "server_shining_score": 1240,
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }).to_string();

                self.broadcast(payload);
            }
        });
    }
}
