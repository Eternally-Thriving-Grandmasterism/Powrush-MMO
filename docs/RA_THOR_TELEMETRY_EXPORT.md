# Ra-Thor telemetry export — Powrush-MMO

**Contract:** Ra-Thor `crates/reality-thriving-transfer`  
**Schemas:** `powrush_telemetry_v1` · `powrush_telemetry_batch_v1`  
**Contact:** info@Rathor.ai

## Live path (preferred)

`TelemetryCollector` now holds a **`GlobalTransferSession`** and updates it on every:

- `collect_tick(world_tick, mercy_flow)`
- `record_tick_result(...)` — council participants, epiphany, harvest, errors

```rust
use simulation::telemetry::TelemetryCollector;

let mut telemetry = TelemetryCollector::new("live_council_run");
// each sim tick:
telemetry.record_tick_result(tick, mercy_flow, council_n, epiphany_n, harvest_n, had_errors);

// end of session:
let json = telemetry.export_transfer_json()?;
// or: telemetry.write_transfer_json_to("session.json")?;
```

### Demo binary (no full world required)

```bash
cargo run -p powrush-simulation --bin transfer_session_demo -- --ticks 100 --out /tmp/powrush_live.json
```

Feed `/tmp/powrush_live.json` (or stdout) into Ra-Thor:

```text
KardashevOrchestrationCouncil::deliberate_from_powrush_batch_json
```

## Profile / offline export (still valid)

```bash
python3 tools/export_powrush_telemetry.py --profile high_mercy
python3 tools/export_powrush_telemetry.py --batch -o /tmp/powrush_batch.json
```

Profiles: `high_mercy` · `marginal` · `early` (aligned with Ra-Thor fixtures).

## full_rbe

See **[FULL_RBE_STATUS.md](./FULL_RBE_STATUS.md)** — feature stays off until deps resolve.
