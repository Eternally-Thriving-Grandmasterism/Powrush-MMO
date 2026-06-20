# Observability Stack for Powrush-MMO

Complete local observability stack with **distributed tracing + metrics**.

## What's Included

- **OpenTelemetry Collector** — OTLP receiver + tail sampling + Prometheus export
- **Jaeger** — Distributed trace visualization
- **Prometheus** — Metrics storage
- **Grafana** — Dashboards with pre-built Powrush Diplomacy view

## Quick Start

```bash
cd observability
 docker compose up -d
```

Access points:
- **Jaeger**: http://localhost:16686
- **Prometheus**: http://localhost:9090
- **Grafana**: http://localhost:3000 (admin / powrush)

## Connect the Powrush Server

```bash
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317 cargo run -p server
```

## Diplomacy Priority Queue Dashboard

A ready-made Grafana dashboard focused on the diplomacy/war priority system is included:

`grafana-dashboard-diplomacy.json`

### How to import it:

1. Open Grafana at http://localhost:3000
2. Go to **Dashboards** → **Import**
3. Upload the file `observability/grafana-dashboard-diplomacy.json`
4. Select the Prometheus data source

The dashboard shows:
- High vs Normal priority message rates
- Connected clients during broadcasts
- Recent high-priority diplomacy events

## Tail Sampling Behavior

During busy server wars or council events, the Collector intelligently keeps:
- All `high_priority` diplomacy traces
- Long-running traces (>500ms)
- All error traces
- 10% of normal traces

This protects Jaeger from overload while preserving the most valuable signals from `broadcast_diplomacy_priority_queue`.

Thunder locked in. Yoi ⚡
