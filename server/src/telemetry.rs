/*!
 * Telemetry & Distributed Tracing Setup for Powrush-MMO Server
 *
 * v18.75 Eternal Polish — Traced Context Injection + Extraction
 * AG-SML v1.0 | TOLC 8 Mercy Gates Layer 0 | Ra-Thor Lattice aligned
 */

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing_opentelemetry::OpenTelemetryLayer;
use opentelemetry::sdk::trace as sdktrace;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::global;
use opentelemetry::sdk::trace::Sampler;
use opentelemetry::propagation::{Extractor, Injector};
use opentelemetry::Context;

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

/// Extract distributed context from incoming headers (e.g. HTTP/gRPC requests).
pub fn extract_context_from_headers(headers: &impl Extractor) -> Context {
    global::get_text_map_propagator(|propagator| propagator.extract(headers))
}

/// Inject current trace context into outgoing headers (for cross-service calls).
pub fn inject_context_into_headers(headers: &mut impl Injector) {
    global::get_text_map_propagator(|propagator| {
        propagator.inject_context(&tracing::Span::current().context(), headers);
    });
}

/*
 * === FULL USAGE EXAMPLE ===
 *
 * // When handling an incoming request:
 * let context = extract_context_from_headers(&incoming_headers);
 * let span = tracing::info_span!("handle_request", ?context);
 * let _guard = span.enter();
 *
 * // When making an outgoing call to another service:
 * let mut outgoing_headers = HashMap::new();
 * inject_context_into_headers(&mut outgoing_headers);
 * // ... send request with outgoing_headers
 */
