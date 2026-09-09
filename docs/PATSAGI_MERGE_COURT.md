# PATSAGi merge court

Merge law for Powrush-MMO. Independent of xAI. No certification or warranty claims.

Steward steers; does not rubber-stamp AUTO. Ra-Thor does not drive WASD. Grok Bots design/review; they do not push `main`.

Missing label = no auto-merge.

## Verdicts

### AUTO

Core green, ballot complete, no deny-list paths. Label `patsgi-auto`. GitHub may squash-merge.

Core:

```bash
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
```

`cargo test --workspace` is not product-green.

### STEER

Playfeel / two-agent collision / unclear Hour 1. Steward one-liner, then continue.

### HOLD

Law change. Steward only. Do not auto-merge HOLD paths. Do not continue HOLD PRs.

## Deny-list (HOLD if touched)

- `Cargo.toml` workspace members / default bins
- `LICENSE` / `COMMERCIAL*`
- `server/**`
- `payments/**`
- `k8s/**`
- `client/src/title_screen.rs` when enabling Online
- Unparking `simulation/` / `host/` crates as default play

Do not widen this list. Do not enable Title Online. No public listen. Do not unpark `server/` as the default binary.

## Seats (short)

- **Steward (Sherif)** steers HOLD; does not rubber-stamp AUTO.
- **Powrush Cursor** codes this repo; does not push `main` or edit Ra-Thor.
- **Ra-Thor Cursor** does lattice only; may read `data/powrush_lived_tick.json`; does not drive keys, WASD, or Title Online.
- **Grok Bots** design/review; they do not push `main`.

See `AGENTS.md` for slice order and the README Agent landing (walked U0–U8 — do not rebuild those).
