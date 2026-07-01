/*!
 * Powrush-MMO Authoritative Server Entry Point
 * Now with full TokioTransport + mpsc to Bevy Event bridge integration.
 */

use bevy::prelude::*;
use tokio::runtime::Runtime;

use server::ServerCorePlugin;
use server::network::tokio_transport::TokioTransport;
use server::lib::TransportEventReceiver;

// ... other imports ...

fn main() {
    apply_server_hardening();
    init_opentelemetry_tracing();

    info!("⚡ Powrush-MMO Authoritative Server v20.8 — Full networking + inventory replication");

    let rt = Runtime::new().expect("Failed to create eternal Tokio runtime");

    rt.block_on(async {
        // Create transport
        let (transport, event_rx, command_tx) = match TokioTransport::new("0.0.0.0:9001").await {
            Ok(t) => t,
            Err(e) => {
                error!("Failed to create transport: {}", e);
                return;
            }
        };

        // Spawn the transport run loop in a separate tokio task
        tokio::spawn(transport.run());

        // Create Bevy app
        let mut app = App::new();

        app.add_plugins(DefaultPlugins)
            .add_plugins(ServerCorePlugin)
            // Insert the receiver so the bridge can drain it
            .insert_resource(TransportEventReceiver { rx: event_rx });

        // Optional: store command_tx if you want to send ServerMessages from Bevy systems
        // app.insert_resource(TransportCommandSender { tx: command_tx });

        app.add_systems(Startup, setup_authoritative_camera)
           .add_systems(Update, authoritative_sovereign_tick)
           .run();
    });
}

// ... rest of file (stubs remain for now) ...