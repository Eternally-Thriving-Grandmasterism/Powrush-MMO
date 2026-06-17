/*!
 * Telemetry & Distributed Tracing Setup for Powrush-MMO Server
 *
 * v18.74 Eternal Polish — Distributed Context Propagation
 * AG-SML v1.0 | TOLC 8 Mercy Gates Layer 0 | Ra-Thor Lattice aligned
 */

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing_opentelemetry::OpenTelemetryLayer;
use opentelemetry::sdk::trace as sdktrace;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::global;
use opentelemetry::sdk::trace::Sampler;
use opentelemetry::Context;
use opentelemetry::propagation::Extractor;

/// Initialize distributed tracing with OpenTelemetry + Jaeger + sampling.
pub fn init_telemetry() {
    let sampler = Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(0.1)));

    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("powrush-mmo-server")
        .with_trace_config(sdktrace::config().with_sampler(sampler))
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

pub fn shutdown_telemetry() {
    global::shutdown_tracer_provider();
}

/// Helper to extract distributed context from incoming headers (e.g. HTTP/gRPC).
/// Useful when receiving requests from other services or clients.
pub fn extract_context_from_headers(headers: &impl Extractor) -> Context {
    global::get_text_map_propagator(|propagator| propagator.extract(headers))
}

/*
 * === USAGE EXAMPLE FOR CONTEXT PROPAGATION ===
 *
 * When making outgoing calls or handling incoming requests:
 *
 * use opentelemetry::propagation::Injector;
 * use tracing_opentelemetry::OpenTelemetrySpanExt;
 *
 * // In an async handler:
 * let span = tracing::info_span!("handle_player_action");
 * let _guard = span.enter();
 *
 * // To propagate context when calling another service:
 * let mut headers = HashMap::new();
 * global::get_text_map_propagator(|prop| {
 *     prop.inject_context(&tracing::Span::current().context(), &mut headers);
 * });
 */
