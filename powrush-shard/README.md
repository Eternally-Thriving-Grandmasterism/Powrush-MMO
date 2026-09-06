# powrush-shard (loopback WS — not default door)

Hex-shard binary with **loopback-only** WebSocket listen. **Not** the default door — `cargo run -p powrush-client` must not depend on this crate.

## Soft cap

**32 Houses** per hex (steward soft cap). Documented in `shared/hex_shard_apply::SOFT_CAP_HOUSES`.

## CLI

```bash
# Loopback WS listen (F8)
cargo run --manifest-path powrush-shard/Cargo.toml -- \
  --hex hex_local_0 \
  --listen 127.0.0.1:7788 \
  --data ./data/hex_local_0/

# Offline dry-apply
cargo run --manifest-path powrush-shard/Cargo.toml -- \
  --hex hex_local_0 \
  --data ./data/hex_local_0/ \
  --dry-apply ./events.jsonl
```

`--listen` binds **loopback only**. `0.0.0.0`, `::`, and non-loopback addresses are refused.

## Wire (JSON envelopes)

1. Client → `hello` → shard `hello_ok` / `hello_no` (v1 may hello_ok a second local House on same hex).
2. Client → `tend` / `take` / `flow` / `reserve` / … → shard `apply` / `reject` (`NO_TAKE`, `NO_BOOK`, `STALE_SEQ`, `PROTO`, …).
3. Snapshot persisted under `--data/ledger_snapshot.json`.
4. Client drop → Offline; book/house intact. Presence = `houses.len()` from real seats.

## Client gate

Outbound WS only when `POWRUSH_NET=localhost`. Default `POWRUSH_NET=off` opens zero sockets. Title **Online** row stays **grey** — Settings/env is the door.

## Refuse

Public bind · TLS theatre on localhost · Postcard · Login wall · Lighting title Online · Wiring this crate into workspace `members` / client default graph.
