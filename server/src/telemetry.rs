/*!
 * Telemetry & Distributed Tracing Setup for Powrush-MMO Server
 *
 * v18.72 Eternal Polish — OpenTelemetry + Jaeger Integration + Usage Example
 * AG-SML v1.0 | TOLC 8 Mercy Gates Layer 0 | Ra-Thor Lattice aligned
 */

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing_opentelemetry::OpenTelemetryLayer;
use opentelemetry::sdk::trace as sdktrace;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::global;

/// Initialize distributed tracing with OpenTelemetry + Jaeger exporter.
///
/// Call this function as early as possible in your server startup (before any spans are created).
pub fn init_telemetry() {
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("powrush-mmo-server")
        .install_simple()
        .expect("Failed to install Jaeger tracer");

    global::set_text_map_propagator(TraceContextPropagator::new());

    let telemetry = OpenTelemetryLayer::new(tracer);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive("powrush_mmo=debug".parse().unwrap()))
        .with(tracing_subscriber::fmt::layer())
        .with(telemetry)
        .init();

    tracing::info!("Distributed tracing initialized with OpenTelemetry + Jaeger");
}

/// Shutdown tracer provider gracefully on application exit.
pub fn shutdown_telemetry() {
    global::shutdown_tracer_provider();
}

/*
 * === USAGE EXAMPLE (add to your main server file) ===
 *
 * use crate::telemetry::{init_telemetry, shutdown_telemetry};
 *
 * #[tokio::main]
 * async fn main() {
 *     // Initialize distributed tracing first
 *     init_telemetry();
 *
 *     // ... build Bevy app, start server, etc.
 *
 *     // On shutdown:
 *     // shutdown_telemetry();
 * }
 */
