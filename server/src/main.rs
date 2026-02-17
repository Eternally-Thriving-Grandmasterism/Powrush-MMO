use anyhow::{Context, Result};
use powrush_divine_module::{MercyCore, ValenceGate};
use shared::protocol::{ClientMessage, ServerMessage};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{error, info, warn};
use std::panic;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("Powrush MMO Server booting — mercy thunder awakening ⚡️");

    // Panic hook — mercy-gated logging
    panic::set_hook(Box::new(|info| {
        error!("PANIC: {}", info);
        // Optional: graceful shutdown or alert broadcast
    }));

    let mercy_core = Arc::new(Mutex::new(MercyCore::new()));

    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .context("Failed to bind server port")?;

    info!("Listening on 0.0.0.0:8080 — awaiting connections");

    loop {
        let (socket, addr) = listener.accept().await?;
        info!("New connection from {}", addr);

        let mercy_clone = mercy_core.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, mercy_clone).await {
                warn!("Connection error from {}: {}", addr, e);
            }
        });
    }
}

async fn handle_connection(mut socket: TcpStream, mercy_core: Arc<Mutex<MercyCore>>) -> Result<()> {
    let mut buf = vec![0; 1024];

    loop {
        let n = socket.read(&mut buf).await?;
        if n == 0 {
            info!("Connection closed");
            return Ok(());
        }

        let msg = &buf[..n];

        // Mercy gate every incoming message
        let gated_result = {
            let mut core = mercy_core.lock().await;
            core.gate_server_message(msg).await
        };

        match gated_result {
            Ok(gated) => {
                socket.write_all(&gated).await?;
                info!("Message processed — mercy gate passed");
            }
            Err(e) => {
                warn!("Mercy gate blocked: {}", e);
                let err_msg = bincode::serialize(&ServerMessage::Error("Mercy gate violation".to_string()))?;
                socket.write_all(&err_msg).await?;
            }
        }
    }
}

// ────────────────────────────────────────────────
// STRESS & QA TEST BLOCK — Run locally to verify
// Fake 10 clients simulation (bash one-liner):
// for i in {1..10}; do
//   cargo run --bin client -- --id=$i &
// done
//
// Monitor logs:
// - No desyncs
// - No panic on flood
// - Mercy blocks spam (valence < 0.75)
// - Graceful error responses
// ────────────────────────────────────────────────
