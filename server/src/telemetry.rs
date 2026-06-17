/*!
 * Telemetry & Distributed Tracing Setup for Powrush-MMO Server
 *
 * v18.71 Eternal Polish — OpenTelemetry + Jaeger Integration
 * AG-SML v1.0 | TOLC 8 Mercy Gates Layer 0 | Ra-Thor Lattice aligned
 */

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing_opentelemetry::OpenTelemetryLayer;
use opentelemetry::sdk::trace as sdktrace;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::global;

/// Initialize distributed tracing with OpenTelemetry + Jaeger exporter.
///
/// This sets up the global tracer provider and configures `tracing` to export spans
/// to Jaeger (via OTLP or agent).
///
/// Usage:
/// ```ignore
/// fn main() {
///     init_telemetry();
///     // ... rest of app
/// }
/// ```
pub fn init_telemetry() {
    // Set up Jaeger exporter (via OTLP gRPC by default)
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("powrush-mmo-server")
        .install_simple()
        .expect("Failed to install Jaeger tracer");

    // Set global propagator for context propagation across services
    global::set_text_map_propagator(TraceContextPropagator::new());

    // Create OpenTelemetry tracing layer
    let telemetry = OpenTelemetryLayer::new(tracer);

    // Build the subscriber with env filter + OpenTelemetry
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive("powrush_mmo=debug".parse().unwrap()))
        .with(tracing_subscriber::fmt::layer())
        .with(telemetry)
        .init();

    tracing::info!("Distributed tracing initialized with OpenTelemetry + Jaeger");
}

/// Shutdown tracer provider on application exit (important for flushing spans)
pub fn shutdown_telemetry() {
    global::shutdown_tracer_provider();
}
