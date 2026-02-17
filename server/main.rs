// ... existing imports ...
use powrush_divine_module::MercyCore;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut mercy_core = MercyCore::new();

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        let mercy_clone = mercy_core.clone(); // or Arc<Mutex> if mutable state grows
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, mercy_clone).await {
                tracing::error!("Connection error: {}", e);
            }
        });
    }
}

async fn handle_connection(mut socket: TcpStream, mut mercy_core: MercyCore) -> Result<()> {
    // ... existing handshake / read loop ...
    while let Some(msg) = read_message(&mut socket).await? {
        // Mercy gate every incoming message
        match mercy_core.gate_server_message(&msg).await {
            Ok(gated) => {
                // Process gated message
                send_response(&mut socket, &gated).await?;
            }
            Err(e) => {
                tracing::warn!("Mercy gate blocked: {}", e);
                send_error(&mut socket, "Mercy gate violation").await?;
            }
        }
    }
    Ok(())
}
