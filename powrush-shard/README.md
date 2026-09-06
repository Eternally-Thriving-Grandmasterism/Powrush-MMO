# powrush-shard (parked)

Parked hex-shard binary. **Not** the default door — `cargo run -p powrush-client` must not depend on this crate.

## Soft cap

**32 Houses** per hex (steward soft cap). Documented in `shared/hex_shard_apply::SOFT_CAP_HOUSES`.

## CLI

```bash
# Offline dry-apply (preferred parked path)
cargo run --manifest-path powrush-shard/Cargo.toml -- \
  --hex hex_local_0 \
  --data ./data/hex_local_0/ \
  --dry-apply ./events.jsonl

# --listen is accepted but parked / not enabled
cargo run --manifest-path powrush-shard/Cargo.toml -- \
  --hex hex_local_0 \
  --listen 127.0.0.1:7788 \
  --data ./data/hex_local_0/
```

`--listen` prints that WS listen is not wired yet and exits unless `--dry-apply` is also set (then apply proceeds offline after the parked notice).

## What it does

1. Load one hex JSON / ledger snapshot from `--data/ledger_snapshot.json` (or fresh fixture).
2. Apply verb events from JSONL (`tend` / `take` / …) via `shared::hex_shard_apply`.
3. Write `ledger_snapshot.json` back under `--data`.

## Refuse (this rev)

Lighting Online row · login wall · fake peers · Ra-Thor hard dep · postcard as v1 · adding this crate to workspace `members` / client default graph.
