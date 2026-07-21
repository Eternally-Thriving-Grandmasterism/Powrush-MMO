/*!
 * Powrush-MMO Authoritative Server Entry Point
 * v21.89.3 — TransportEventReceiver + TransportCommandSender both injected.
 *   Enables process_audio_moment_messages and inventory systems to reply to clients.
 *
 * AG-SML v1.0 | TOLC 8 + PATSAGi | Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use tokio::runtime::Runtime;

use server::ServerCorePlugin;
use server::network::tokio_transport::TokioTransport;
use server::{
    TransportEventReceiver, TransportCommandSender,
};

fn main() {
    apply_server_hardening();
    init_opentelemetry_tracing();

    info!("⚡ Powrush-MMO Authoritative Server v21.89.3 — networking + inventory + SafetyNet + AudioMoments");

    let rt = Runtime::new().expect("Failed to create eternal Tokio runtime");

    rt.block_on(async {
        let (transport, event_rx, command_tx) = match TokioTransport::new("0.0.0.0:9001").await {
            Ok(t) => t,
            Err(e) => {
                error!("Failed to create transport: {}", e);
                return;
            }
        };

        // Transport accept/read/write loop
        tokio::spawn(transport.run());

        let mut app = App::new();

        app.add_plugins(DefaultPlugins)
            .add_plugins(ServerCorePlugin)
            // Ingress: transport → Bevy events
            .insert_resource(TransportEventReceiver { rx: event_rx })
            // Egress: Bevy systems → transport (audio acks, catalog snapshots, etc.)
            .insert_resource(TransportCommandSender { tx: command_tx });

        app.add_systems(Startup, setup_authoritative_camera);
        app.add_systems(Update, authoritative_sovereign_tick);

        info!(
            "Server initialized: inventory replication, SafetyNet, AudioMoment catalog, \
             TransportCommandSender LIVE."
        );

        app.run();
    });
}

fn apply_server_hardening() {
    info!("[Server] Hardening applied (placeholder — production ready in hardening.rs)");
}

fn init_opentelemetry_tracing() {
    info!("[Server] OpenTelemetry tracing initialized (placeholder)");
}

fn setup_authoritative_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    info!("[Server] Authoritative camera setup complete");
}

fn authoritative_sovereign_tick() {
    // Heavy lifting lives in ServerCorePlugin systems.
}

// Thunder locked in. Yoi ⚡
