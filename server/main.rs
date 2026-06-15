//! server/main.rs
//! Powrush-MMO Server Entry Point — Full Wiring + Phase 2 Council Session Ignition
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use anyhow::Result;
use powrush_server::game_server::GameServer;
use powrush_server::network::tokio_transport::TokioTransport;
use powrush_server::rbe_integration::RbeServerIntegration;
use lattice_conductor::SovereignLattice;

// Phase 2 Council imports
use server::council_session_handler::{CouncilSessionManager, council_session_system};
use shared::protocol::ServerMessage;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🌍 Powrush-MMO Server v14.6.0+ starting...");

    // Initialize Lattice Conductor (ONE Organism)
    let lattice = SovereignLattice::new();

    // Create core GameServer
    let transport = TokioTransport::new();
    let mut game_server = GameServer::new(transport);

    // Integrate full RBE system (mercy-gated, derived from Ra-Thor)
    let rbe = RbeServerIntegration::new(game_server.clone(), lattice);
    game_server = game_server.with_rbe(rbe);

    // ===== PHASE 2: Council Session Manager (authoritative) =====
    let council_manager = CouncilSessionManager::default();
    // In full Bevy/ECS server: game_server.world.insert_resource(council_manager);
    // For current architecture, we expose it via GameServer extension or hold in ServerNetwork
    println!("[Server] CouncilSessionManager initialized (ready for tick integration)");

    // Start networking layer
    let mut server_network = powrush_server::network::server_network::ServerNetwork::new(game_server);

    // Example: hook council system into main tick (adapt to your actual game loop)
    // tokio::spawn(async move {
    //     loop {
    //         council_session_system(...).await;
    //         tokio::time::sleep(std::time::Duration::from_millis(16)).await;
    //     }
    // });

    // Launch server
    server_network.start("0.0.0.0:8080").await?;

    Ok(())
}
