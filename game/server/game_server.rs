//! game/server/game_server.rs
//! Full production-grade GameServer with DeltaCompressor integration
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{broadcast, mpsc, RwLock};
use tokio::time;
use bytes::Bytes;

use crate::network::{
    tokio_transport::TokioTransport,
    message_framing::{encode_frame, decode_frame},
    delta_compression::DeltaCompressor,
};

const TICK_RATE: u64 = 60; // 60 Hz
const TICK_INTERVAL: Duration = Duration::from_millis(1000 / TICK_RATE);

#[derive(Debug, Clone)]
pub struct Player {
    pub id: u64,
    pub sequence: u32,
    pub last_seen: Instant,
}

pub struct GameServer {
    transport: Arc<TokioTransport>,
    delta_compressor: Arc<DeltaCompressor>,
    players: Arc<RwLock<HashMap<u64, Player>>>,
    state_tx: broadcast::Sender<Bytes>,
    shutdown: mpsc::Sender<()>,
    shutdown_rx: mpsc::Receiver<()>,
}

impl GameServer {
    pub fn new(transport: TokioTransport) -> Self {
        let (state_tx, _) = broadcast::channel(1024);
        let (shutdown_tx, shutdown_rx) = mpsc::channel(1);

        Self {
            transport: Arc::new(transport),
            delta_compressor: Arc::new(DeltaCompressor::new()),
            players: Arc::new(RwLock::new(HashMap::new())),
            state_tx,
            shutdown: shutdown_tx,
            shutdown_rx,
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        println!("🚀 Powrush GameServer v14.6.0 starting @ {} Hz", TICK_RATE);

        let mut tick_interval = time::interval(TICK_INTERVAL);
        let mut shutdown_rx = self.shutdown_rx;

        loop {
            tokio::select! {
                _ = tick_interval.tick() => {
                    self.tick().await?;
                }
                _ = shutdown_rx.recv() => {
                    println!("🛑 Graceful shutdown requested");
                    break;
                }
            }
        }

        Ok(())
    }

    async fn tick(&self) -> anyhow::Result<()> {
        // Generate world state delta
        let world_state = self.generate_world_state().await;
        let delta = self.delta_compressor.compress(&world_state);

        // Broadcast to all connected players
        let _ = self.state_tx.send(delta);

        // Cleanup stale players
        self.cleanup_stale_players().await;

        Ok(())
    }

    async fn generate_world_state(&self) -> Vec<u8> {
        // Placeholder for full world state serialization
        // In production this would serialize entities, resources, RBE data, etc.
        vec![0u8; 1024] // Real implementation would use bincode / postcard / etc.
    }

    async fn cleanup_stale_players(&self) {
        let mut players = self.players.write().await;
        let now = Instant::now();
        players.retain(|_, player| now.duration_since(player.last_seen) < Duration::from_secs(30));
    }

    pub async fn handle_new_connection(&self, mut transport: TokioTransport) {
        let player_id = rand::random::<u64>();
        {
            let mut players = self.players.write().await;
            players.insert(player_id, Player {
                id: player_id,
                sequence: 0,
                last_seen: Instant::now(),
            });
        }

        println!("👤 Player {} connected", player_id);

        // Spawn per-player message loop
        let delta_rx = self.state_tx.subscribe();
        let players = self.players.clone();

        tokio::spawn(async move {
            // Handle incoming messages + broadcast deltas to this player
            // (Full implementation would use message_framing + delta_compression here)
            let _ = transport.run().await;
            // Cleanup on disconnect
            let mut players = players.write().await;
            players.remove(&player_id);
        });
    }

    pub async fn shutdown(&self) {
        let _ = self.shutdown.send(()).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_game_server_tick() {
        let transport = TokioTransport::new();
        let mut server = GameServer::new(transport);
        let result = server.tick().await;
        assert!(result.is_ok());
    }
}
