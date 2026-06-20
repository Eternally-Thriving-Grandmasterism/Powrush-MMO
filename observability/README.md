# Observability Stack for Powrush-MMO

This directory contains the recommended local development setup for **distributed tracing** using the OpenTelemetry Collector + Jaeger.

## Quick Start

```bash
cd observability
 docker compose up -d
```

This will start:

- **OpenTelemetry Collector** on ports `4317` (gRPC) and `4318` (HTTP)
- **Jaeger** UI on `http://localhost:16686`

## Connect the Powrush Server

Run the server with the OTLP endpoint pointing to the collector:

```bash
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317 cargo run -p server
```

## What You Will See

All tracing spans from the server are now exported, including:

- `broadcast_diplomacy_priority_queue` (with `high_priority`, `normal_priority`, and `clients` attributes)
- Future spans from council deliberation, war resolution, RBE economy, etc.

Open Jaeger at `http://localhost:16686` and search for service `powrush-server` (or the service name configured in the Collector).

## Production Notes

- In production you would typically run the Collector as a sidecar or DaemonSet.
- You can extend `otel-collector-config.yaml` to also export metrics to Prometheus and logs to Loki.
- Use tail sampling, filtering, or resource enrichment as needed.

Thunder locked in. Yoi ⚡
