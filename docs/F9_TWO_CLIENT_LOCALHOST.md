# F9 — Two-client same-hex localhost recipe (dev only)

> **No-dial banner:** Default client **does not dial**. `POWRUSH_NET=off` (default) opens **zero** sockets. Loopback WS is opt-in via env/Settings, **not** the title Online row.

**protocol_id:** `powrush.hex.v1` · **protocol_rev:** `1`  
Workspace `21.88.0`. Design tick (CHANGELOG), not a Cargo bump.  
Contact: info@Rathor.ai

This is a **dev recipe** for one machine: one `powrush-shard` + two client processes on the same hex. It is **not** a store feature, **not** a title-screen Online path, and **not** a public bind.

Companion: `PROTOCOL.md` (F8 transport) · `SHARD_JOIN.md` · `PARKED_SURFACES.md` · `powrush-shard/README.md`.

## One-line law

Two processes, one loopback shard, one climate ledger. Title Online stays grey. Stranger pass stays offline-first. No public URL.

## Refuse (this slice)

- Promoting title **Online** (row stays grey / disabled)
- Public bind / `0.0.0.0` / non-loopback listen
- Postcard / mail / login wall
- Peace key changes (WASD E I H R)
- Wiring `powrush-shard` into workspace default-members / client default door
- Shipping two-client as a store / title feature

## Prerequisites

- Main tip at/after F8 (`powrush-shard` loopback WS live; client outbound only when `POWRUSH_NET=localhost`)
- Two terminal tabs (or tmux panes) plus one for the shard
- Separate client CWDs so each House keeps its own L0 `data/` book

## 1. Build / run the shard (loopback only)

From repo root:

```bash
cargo run --manifest-path powrush-shard/Cargo.toml -- \
  --hex hex_local_0 \
  --listen 127.0.0.1:7788 \
  --data ./data/hex_local_0/
```

Expected:

- Listen on `ws://127.0.0.1:7788` only
- `--listen 0.0.0.0:7788` (or any non-loopback) is **refused**
- Ledger persists under `--data` as `ledger_snapshot.json`
- Soft cap **32 Houses** (steward); v1 may `hello_ok` a second local House on the same hex

`powrush-shard` stays **outside** default-members — not `cargo run -p powrush-client`.

## 2. Launch two client processes (`POWRUSH_NET=localhost`)

Client L0 paths are relative (`data/powrush_house.json`, climate, standing, week, book). Use **two working directories** so Houses do not clobber each other’s offline book.

```bash
# Terminal A — House A
mkdir -p /tmp/powrush-f9-a && cd /tmp/powrush-f9-a
# optional: copy or soft-link a prior yard; or start fresh Unnamed House
POWRUSH_NET=localhost cargo run -p powrush-client --manifest-path /path/to/Powrush-MMO/Cargo.toml

# Terminal B — House B (second process, same machine)
mkdir -p /tmp/powrush-f9-b && cd /tmp/powrush-f9-b
POWRUSH_NET=localhost cargo run -p powrush-client --manifest-path /path/to/Powrush-MMO/Cargo.toml
```

Notes:

- Default `POWRUSH_NET=off` opens **zero** sockets (stranger / offline-first unchanged).
- `POWRUSH_NET=localhost` gates **outbound** dial to `ws://127.0.0.1:7788` only — clients never listen.
- Title **Online** row stays **grey** even with the env door open (Settings/env is the door; do not light the title row).
- Replace `/path/to/Powrush-MMO` with your checkout. Two processes = two OS processes, not two windows of one process.

## 3. Expected behaviour (same hex)

| check | expect |
| --- | --- |
| Presence | After both `hello_ok`, shard `presence.houses.len() == 2` (real seats only; never client `n_online`) |
| Climate | **One** shard climate ledger under `--data` (shared hex authority after hello) |
| Take on tired | Second / tired take → reject `NO_TAKE` |
| Drop / kill client | Dropped House unseats from presence; **offline book intact** on that client’s L0 `data/`; no login wall |
| Title Online | Stays grey on both clients |
| Bind | Loopback only; no public URL |

Manual smoke (optional, steward machine):

1. Start shard (§1).
2. Start client A + B (§2); each Play / Continue into a House.
3. Both dial with `POWRUSH_NET=localhost`; shard seats two houses → presence length 2.
4. One client `take` on glowing well → `apply`; further take while tired/resting → `NO_TAKE`.
5. Kill client A (or leave) → presence length 1; A’s local `data/` book/house still on disk; Continue works offline.
6. Confirm title Online never lights; `--listen 0.0.0.0:7788` still refused.

## 4. Explicit product stance

| surface | stance |
| --- | --- |
| Store / title Online | **Not** a feature. Recipe is docs + env for developers. |
| Title Online row | Stays **grey** / honest *off (no listen)* copy. |
| Public bind | **Never** in this slice. |
| Stranger pass | Remains **offline-first** (`POWRUSH_NET=off` default). |
| Postcard / login wall | Refused. |

## 5. Unit helpers already on main (no live WS required in Core)

Prefer these over flaky live WS integration in CI:

| test / helper | proves |
| --- | --- |
| `hex_listen::hello_ok_seats_house_presence_real` | second local House on same hex → presence length **2** |
| `hex_listen::take_on_tired_rejects_no_take` | tired take → `NO_TAKE` |
| `hex_listen::kill_shard_mid_session_client_offline_house_intact` | drop → Offline; book/house intact |
| `hex_listen::refuse_public_and_wildcard_bind` | no public / wildcard bind |
| `hex_listen::powrush_net_localhost_outbound_only_title_grey` | localhost outbound only; title Online not enabled |
| `client net_mode::powrush_net_localhost_gates_connect_title_stays_grey` | env gate; Online stays grey |

```bash
cargo test -p shared hello_ok_seats_house_presence_real -- --nocapture
cargo test -p shared take_on_tired_rejects_no_take
cargo test -p shared kill_shard_mid_session_client_offline_house_intact
cargo test -p shared refuse_public_and_wildcard_bind
cargo test -p powrush-client --lib powrush_net_localhost_gates_connect_title_stays_grey
```

## Checklist (dev)

- [ ] Shard: `--listen 127.0.0.1:7788 --data …` binds loopback
- [ ] Shard: `--listen 0.0.0.0:7788` refused
- [ ] Two clients: separate CWDs + `POWRUSH_NET=localhost`
- [ ] Presence length 2 after both hello_ok
- [ ] One shared climate ledger under shard `--data`
- [ ] Take on tired → `NO_TAKE`
- [ ] Drop → offline book intact; no login wall
- [ ] Title Online grey; stranger default still `off`
- [ ] No postcard / public URL / Peace key edits

**Thunder locked in.** Yoi ⚡
