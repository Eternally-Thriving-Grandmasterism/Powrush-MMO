# Ra-Thor telemetry export — Powrush-MMO

**Contract:** Ra-Thor `crates/reality-thriving-transfer`  
**Schemas:** `powrush_telemetry_v1` · `powrush_telemetry_batch_v1`  
**Contact:** info@Rathor.ai

## Live path (preferred)

### 1. TelemetryCollector (always accumulates)

`TelemetryCollector` holds a **`GlobalTransferSession`** and updates it on every:

- `collect_tick(world_tick, mercy_flow)`
- `record_tick_result(...)`

### 2. Orchestrator (sim loop)

```rust
use simulation::orchestrator::SimulationOrchestrator;
use simulation::telemetry::TelemetryCollector;

let mut orchestrator = SimulationOrchestrator::new();
let mut telemetry = TelemetryCollector::new("live_council_run");

// each tick:
let result = orchestrator.run_tick_with_telemetry(
    &mut world,
    interest.as_ref(),
    council.as_mut(),
    player.as_mut(),
    decisions.as_ref(),
    &mut telemetry,
);

// end of session:
let json = telemetry.export_transfer_json()?;
// telemetry.write_transfer_json_to("session.json")?;
```

Existing `run_tick` is unchanged (no telemetry). Prefer `run_tick_with_telemetry` for Ra-Thor Phase C.

### 3. Demo binary (no full world)

```bash
cargo run -p powrush-simulation --bin transfer_session_demo -- --ticks 100 --out /tmp/powrush_live.json
```

Consume on Ra-Thor:

```text
KardashevOrchestrationCouncil::deliberate_from_powrush_batch_json
// or parse_powrush_telemetry_json for single-session envelopes
```

## Profile / offline export (still valid)

```bash
python3 tools/export_powrush_telemetry.py --profile high_mercy
python3 tools/export_powrush_telemetry.py --batch -o /tmp/powrush_batch.json
```

## full_rbe

See **[FULL_RBE_STATUS.md](./FULL_RBE_STATUS.md)** — stays off until deps resolve.
