/*!
 * Telemetry & Distributed Tracing Setup for Powrush-MMO Server
 *
 * v18.73 Eternal Polish — OpenTelemetry + Jaeger with Sampling Configuration
 * AG-SML v1.0 | TOLC 8 Mercy Gates Layer 0 | Ra-Thor Lattice aligned
 */

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing_opentelemetry::OpenTelemetryLayer;
use opentelemetry::sdk::trace as sdktrace;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::global;
use opentelemetry::sdk::trace::Sampler;

/// Initialize distributed tracing with OpenTelemetry + Jaeger exporter + sampling.
///
/// Sampling is critical in production to control trace volume.
/// Recommended: ParentBased(TraceIdRatioBased(0.1)) for 10% sampling with context propagation.
pub fn init_telemetry() {
    // Configure sampler - adjust ratio based on load and observability needs
    let sampler = Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(0.1))); // 10% sampling

    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("powrush-mmo-server")
        .with_trace_config(
            sdktrace::config()
                .with_sampler(sampler)
        )
        .install_simple()
        .expect("Failed to install Jaeger tracer");

    global::set_text_map_propagator(TraceContextPropagator::new());

    let telemetry = OpenTelemetryLayer::new(tracer);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive("powrush_mmo=debug".parse().unwrap()))
        .with(tracing_subscriber::fmt::layer())
        .with(telemetry)
        .init();

    tracing::info!("Distributed tracing initialized with OpenTelemetry + Jaeger (10% sampling)");
}

/// Shutdown tracer provider gracefully on application exit.
pub fn shutdown_telemetry() {
    global::shutdown_tracer_provider();
}

/*
 * === USAGE EXAMPLE ===
 *
 * use crate::telemetry::{init_telemetry, shutdown_telemetry};
 *
 * #[tokio::main]
 * async fn main() {
 *     init_telemetry();
 *     // ... build app ...
 *     // shutdown_telemetry();
 * }
 */
