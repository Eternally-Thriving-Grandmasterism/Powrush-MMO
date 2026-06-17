/*!
 * Telemetry & Distributed Tracing Setup for Powrush-MMO Server
 *
 * v18.78 Eternal Polish — Correlation IDs Implementation
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
use ulid::Ulid;

/// Initialize distributed tracing following best practices.
pub fn init_telemetry() {
    let sampler = Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(0.1)));

    let resource = opentelemetry::sdk::Resource::new(vec![
        opentelemetry::KeyValue::new("service.name", "powrush-mmo-server"),
        opentelemetry::KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
        opentelemetry::KeyValue::new("deployment.environment", std::env::var("DEPLOY_ENV").unwrap_or_else(|_| "development".to_string())),
    ]);

    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("powrush-mmo-server")
        .with_trace_config(
            sdktrace::config()
                .with_sampler(sampler)
                .with_resource(resource)
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

    tracing::info!("Distributed tracing initialized (best practices + correlation IDs)");
}

pub fn shutdown_telemetry() {
    global::shutdown_tracer_provider();
}

/// Extract distributed context from incoming headers.
pub fn extract_context_from_headers(headers: &impl Extractor) -> Context {
    global::get_text_map_propagator(|propagator| propagator.extract(headers))
}

/// Inject current trace context into outgoing headers.
pub fn inject_context_into_headers(headers: &mut impl Injector) {
    global::get_text_map_propagator(|propagator| {
        propagator.inject_context(&tracing::Span::current().context(), headers);
    });
}

/// Generate a new Correlation ID (ULID for sortability + uniqueness).
pub fn generate_correlation_id() -> String {
    Ulid::new().to_string()
}

/// Attach a correlation ID to the current tracing span.
/// This makes the ID appear in all logs and traces within this span.
pub fn with_correlation_id(correlation_id: &str) {
    tracing::Span::current().record("correlation_id", correlation_id);
}

/*
 * === CORRELATION ID BEST PRACTICES ===
 *
 * 1. Generate once per incoming request / important operation:
 *    let correlation_id = generate_correlation_id();
 *
 * 2. Attach it early in the handler:
 *    with_correlation_id(&correlation_id);
 *
 * 3. It will now appear in all `tracing::info!`, `tracing::error!`, etc.
 *    inside the current span and child spans.
 *
 * 4. Propagate it to other services via headers if needed (in addition to trace context).
 */
