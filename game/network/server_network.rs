//! game/network/server_network.rs
//! High-level Server Networking Coordinator
//! Ties together transport, framing, game server, and delta compression
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use std::sync::Arc;
use tokio::net::TcpListener;
use crate::network::tokio_transport::TokioTransport;
use crate::server::game_server::GameServer;

pub struct ServerNetwork {
    game_server: Arc<GameServer>,
}

impl ServerNetwork {
    pub fn new(game_server: GameServer) -> Self {
        Self {
            game_server: Arc::new(game_server),
        }
    }

    pub async fn start(&self, addr: &str) -> anyhow::Result<()> {
        let listener = TcpListener::bind(addr).await?;
        println!("🌐 Powrush Server Network listening on {}", addr);

        loop {
            let (stream, peer) = listener.accept().await?;
            println!("🔌 New connection from {}", peer);

            let transport = TokioTransport::from_stream(stream);
            let game_server = self.game_server.clone();

            tokio::spawn(async move {
                game_server.handle_new_connection(transport).await;
            });
        }
    }
}
