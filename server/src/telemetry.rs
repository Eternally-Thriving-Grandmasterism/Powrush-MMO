/*!
 * Telemetry & Distributed Tracing Setup for Powrush-MMO Server
 *
 * v18.76 Eternal Polish — Distributed Tracing Best Practices
 * AG-SML v1.0 | TOLC 8 Mercy Gates Layer 0 | Ra-Thor Lattice aligned
 */

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing_opentelemetry::OpenTelemetryLayer;
use opentelemetry::sdk::trace as sdktrace;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::global;
use opentelemetry::sdk::trace::Sampler;
use opentelemetry::sdk::Resource;
use opentelemetry::KeyValue;
use opentelemetry::propagation::{Extractor, Injector};
use opentelemetry::Context;

/// Initialize distributed tracing following production best practices.
///
/// Best practices implemented:
/// - Parent-based sampling with configurable ratio
/// - Resource attributes (service name, version, environment)
/// - Proper context propagation (W3C Trace Context)
/// - Graceful shutdown support
pub fn init_telemetry() {
    // Sampling: ParentBased + TraceIdRatioBased (configurable via env in future)
    let sampler = Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(0.1)));

    // Resource attributes (best practice for service identification in backends)
    let resource = Resource::new(vec![
        KeyValue::new("service.name", "powrush-mmo-server"),
        KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
        KeyValue::new("deployment.environment", std::env::var("DEPLOY_ENV").unwrap_or_else(|_| "development".to_string())),
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

    tracing::info!("Distributed tracing initialized (best practices: sampling + resource attributes)");
}

pub fn shutdown_telemetry() {
    global::shutdown_tracer_provider();
}

/// Extract context from incoming headers (W3C Trace Context)
pub fn extract_context_from_headers(headers: &impl Extractor) -> Context {
    global::get_text_map_propagator(|propagator| propagator.extract(headers))
}

/// Inject current trace context into outgoing headers
pub fn inject_context_into_headers(headers: &mut impl Injector) {
    global::get_text_map_propagator(|propagator| {
        propagator.inject_context(&tracing::Span::current().context(), headers);
    });
}

/*
 * === BEST PRACTICES USAGE ===
 *
 * 1. Always use `#[instrument]` on important functions
 * 2. Record errors: `tracing::error!(error = %e, "operation failed");`
 * 3. Use `tracing::info_span!("name", field1 = value)` for custom spans
 * 4. Propagate context on every cross-service or async boundary
 */
