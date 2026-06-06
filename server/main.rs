//! server/main.rs
//! Powrush-MMO Server Entry Point — Full Wiring
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use anyhow::Result;
use powrush_server::game_server::GameServer;
use powrush_server::network::tokio_transport::TokioTransport;
use powrush_server::rbe_integration::RbeServerIntegration;
use lattice_conductor::SovereignLattice;

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
    game_server = game_server.with_rbe(rbe); // using the extension trait

    // Start networking layer
    let server_network = powrush_server::network::server_network::ServerNetwork::new(game_server);

    // Launch server
    server_network.start("0.0.0.0:8080").await?;

    Ok(())
}
