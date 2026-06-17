/*!
 * Telemetry & Distributed Tracing Setup for Powrush-MMO Server
 *
 * v18.77 Eternal Polish — Enhanced Context Propagation Code Examples
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

    tracing::info!("Distributed tracing initialized (best practices)");
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

/*
 * =====================================================
 * COMPLETE CONTEXT PROPAGATION CODE EXAMPLES
 * =====================================================
 *
 * === 1. In an async handler (e.g. player action) ===
 *
 * async fn handle_player_action(incoming_headers: &impl Extractor) {
 *     let parent_context = extract_context_from_headers(incoming_headers);
 *
 *     let span = tracing::info_span!("handle_player_action", player_id = 123);
 *     let _guard = span.enter();
 *
 *     // Your business logic here...
 * }
 *
 * === 2. Making an outgoing call to another service ===
 *
 * async fn call_other_service() {
 *     let mut headers = std::collections::HashMap::new();
 *     inject_context_into_headers(&mut headers);
 *
 *     // Send request with `headers` (e.g. via reqwest, tonic, etc.)
 *     // The receiving service can then extract the context.
 * }
 *
 * === 3. Using #[instrument] (recommended for most functions) ===
 *
 * #[tracing::instrument(skip(batch_queue))]
 * pub fn tick_all(...) { ... }
 */
