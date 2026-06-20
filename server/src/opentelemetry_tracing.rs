// server/src/opentelemetry_tracing.rs
// OpenTelemetry Distributed Tracing Initialization for Powrush-MMO Server
// Wires tracing spans (including diplomacy priority queue) to OTLP collectors (Jaeger, Tempo, etc.)
// PATSAGi + Ra-Thor aligned observability
// AG-SML v1.0

use opentelemetry::global;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::trace::{BatchConfig, BatchSpanProcessor, TracerProvider};
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use tracing_opentelemetry::layer as otel_layer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize OpenTelemetry distributed tracing.
/// Call this early in main(), before creating the Bevy App.
///
/// By default exports to localhost:4317 (OTLP/gRPC).
/// Set OTEL_EXPORTER_OTLP_ENDPOINT env var to override.
pub fn init_opentelemetry_tracing() {
    // Set global propagator for context propagation across services/realms
    global::set_text_map_propagator(TraceContextPropagator::new());

    // Create OTLP span exporter (gRPC by default)
    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_endpoint(
            std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:4317".to_string()),
        )
        .build()
        .expect("Failed to create OTLP span exporter");

    // Batch processor for efficient export
    let batch_processor = BatchSpanProcessor::builder(exporter, opentelemetry_sdk::runtime::Tokio)
        .with_batch_config(BatchConfig::default())
        .build();

    // Build TracerProvider
    let provider = TracerProvider::builder()
        .with_span_processor(batch_processor)
        .build();

    // Set as global provider
    let _ = global::set_tracer_provider(provider.clone());

    // Create OpenTelemetry tracing layer
    let otel_layer = otel_layer().with_tracer(provider.tracer("powrush-server"));

    // Combine with env filter + fmt layer
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,powrush=debug"));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .with(otel_layer)
        .init();

    tracing::info!(
        target: "opentelemetry",
        "OpenTelemetry distributed tracing initialized (OTLP gRPC) — diplomacy/war spans will be exported"
    );
}

/// Shutdown OpenTelemetry provider (call on graceful server exit if needed)
pub fn shutdown_opentelemetry() {
    global::shutdown_tracer_provider();
}
