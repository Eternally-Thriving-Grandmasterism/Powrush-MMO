# Observability Stack for Powrush-MMO

Complete local observability stack with **distributed tracing + metrics**.

## What's Included

- **OpenTelemetry Collector** — OTLP receiver + tail sampling + Prometheus export
- **Jaeger** — Distributed trace visualization
- **Prometheus** — Metrics storage (scrapes Collector)
- **Grafana** — Dashboards (pre-configured to use Prometheus)

## Quick Start

```bash
cd observability
 docker compose up -d
```

This starts:

- OpenTelemetry Collector (OTLP on `4317`)
- Jaeger UI → `http://localhost:16686`
- Prometheus → `http://localhost:9090`
- Grafana → `http://localhost:3000` (login: `admin` / `powrush`)

## Connect the Powrush Server

```bash
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317 cargo run -p server
```

## Key Features Enabled

### Tail Sampling (Smart Trace Retention)
During high-volume server wars or council events, the Collector keeps:

- All traces with `high_priority` attribute
- Long-running traces (>500ms)
- All error traces
- 10% of normal traces (probabilistic)

This prevents overwhelming Jaeger while preserving the most important diplomacy/war signals.

### Metrics
The Collector now also exports metrics on port `8889`. Prometheus scrapes them and makes `powrush_*` metrics available in Grafana.

You can visualize:
- `powrush_high_priority_messages`
- `powrush_normal_priority_messages`
- Future Bevy diagnostics we expose

## What You Will See

- **Traces**: Full `broadcast_diplomacy_priority_queue` spans with attributes (`high_priority`, `normal_priority`, `clients`)
- **Metrics**: Priority queue counters + any future instrumentation

Thunder locked in. Yoi ⚡
