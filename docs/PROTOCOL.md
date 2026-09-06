# PROTOCOL.md — Powrush hex shard protocol (v1)

**protocol_id:** `powrush.hex.v1`  
**protocol_rev:** `1`  
**Contact:** info@Rathor.ai  
Workspace `21.88.0`. Design tick (CHANGELOG), not a Cargo bump.

Steward law: offline client is authority until a real shard is joined. Online shard is authority only after honest hello. Default client never listens. `POWRUSH_NET=off` by default. server/ stays parked in this rev. Peace keys WASD E I H R unchanged. No Ra-Thor path dep. No fake peers.

## Envelope (JSON)

Every message is one JSON object:

| field | type | meaning |
| --- | --- | --- |
| `v` | u32 | envelope version; v1 = `1` |
| `pid` | string | must be `powrush.hex.v1` |
| `kind` | string | body discriminant (op or shard reply) |
| `hex` | string | hex / shard id (e.g. `local-hex`) |
| `house` | string | house id or display label |
| `seq` | u64 | per-house monotonic sequence |
| `ts_ms` | u64 | client or shard wall clock ms |
| `body` | object | kind-specific payload |

Rev / pid mismatch → reject `PROTO`. Stale or replayed `seq` → `STALE_SEQ`.

## Roles

| role | when | authority |
| --- | --- | --- |
| **Offline client** | `POWRUSH_NET=off` (default) or no hello_ok | Client owns climate / standing / week / book / house on L0 disk. No listen socket. |
| **Online shard** | after `hello_ok` on an honest path | Shard owns apply / reject / snapshot / week / presence. Client proposes ops; shard decides. |

Offline fallback is the product path. Online is a mode beside a working House — never the gate.

## Client → shard ops (`kind`)

| kind | body notes |
| --- | --- |
| `tend` | care-tend on a node / well |
| `take` | glowing take; tired → `NO_TAKE` |
| `flow` | R-1 circulate |
| `reserve` | repair-rights reserve; bad shape → `BAD_RESERVE` |
| `mend` | MendSpool |
| `lane` | LaneCrate |
| `bind` | Ledger bind / escort step |
| `declare_lethal` | Ledger 3 only after book; before book → `NO_BOOK` |
| `clear_lethal` | clear declared lethal (tariff already paid stays) |
| `name_house` | charter label; not charter → `NOT_CHARTER` |
| `request_seat` | Embassy seat; needs Proof Pack else `NO_PACK` |
| `snapshot_req` | ask shard for ledger snapshot |
| `hello` | join / handshake; carries `protocol_rev` |

## Shard → client kinds

| kind | meaning |
| --- | --- |
| `apply` | op accepted; body may echo seq + delta |
| `reject` | op refused; body has `code` (+ optional reason) |
| `snapshot` | full ledger snapshot (see below) |
| `week` | week audit line (tons + restored) |
| `presence` | houses array **only** — never `n_online` |
| `hello_ok` | join accepted |
| `hello_no` | join refused (often `PROTO` / `COPY_DENIED`) |

## Ledger snapshot shape

`snapshot.body` (and offline L0 composite read) carries:

```
climate   — harmony, stress, regen, reserve_pool, restored_count, tons_moved, hex_id
standing  — peace, harmony, consumption, steward, human_hybrid_heat, declared_lethal, tariff_paid
week      — week_id, tons_moved, restored_count
book      — hour_two_held, hour_three_held (book flags)
house     — house_id, house_name (or Unnamed House)
```

No peer count. No talent tree. No fake online tally.

## Reject codes

| code | when |
| --- | --- |
| `STALE_SEQ` | seq ≤ last applied for that house |
| `NO_TAKE` | take on tired / resting / stressed well |
| `NOT_CHARTER` | name_house / charter act without charter right |
| `NO_BOOK` | declare_lethal before Hour three / book held |
| `NO_PACK` | request_seat without Proof Pack |
| `BAD_RESERVE` | reserve body illegal or empty pool misuse |
| `TELEPORT` | hex jump / spoofed hex without warrant |
| `PROTO` | pid / rev / envelope version mismatch |
| `COPY_DENIED` | join copy without consent |

## Join, leave, presence, authority (F2–F4)

### Authority

| mode | when | who owns climate / standing / week / book / house |
| --- | --- | --- |
| **Offline client** | default; after cancel; after `hello_no`; after leave / net drop | Client on L0 disk. Online UI stays grey (*off · no listen*). |
| **Online shard** | only after `hello_ok` on an honest consent path | Shard applies / rejects / snapshots / weeks / presence. Client proposes. |

Offline is the product path. Online is a mode beside a working House — never a login wall.

### Join (F2) — copy with consent

1. Player is offline on their local House + yard.
2. Join proposes `hello` with `protocol_rev` + `consent_copy`.
3. **With consent:** shard may copy local House → shard House. **Local yard remains** on L0 (offline save untouched as fallback).
4. **Cancel / decline consent:** stay offline; no shard House; Online stays grey.
5. **Without consent** (or steward deny): `COPY_DENIED` → `hello_no` → client **keeps offline authority**. No partial online.

Join never deletes the local yard. Join never silently overwrites a divergent hex history (see Merge veto).

### Leave / net drop (F3)

- Leave or unexpected disconnect → client resumes from the **last certified snapshot** (shard `snapshot` if one arrived; else last local L0 composite).
- Book flags (`hour_two_held` / `hour_three_held`) stay on disk conceptually — disconnect mid-tend does not wipe the book.
- Continue offline immediately. **No login wall.** No forced re-auth. Online row returns to honest grey.
- Mid-op (e.g. tend in flight) that never got `apply` is dropped; last applied snapshot wins.

### Presence honesty (F4)

- Shard may emit `presence` with `{ "houses": [ … ] }` — real seated houses, or silence (omit / empty).
- **Client cannot author `n_online`.** No invented peer count on wire, in snapshot, or on HUD.
- Fake / client-supplied online tally is reject-class honesty failure (treat as silence; never display).
- Real count only when the shard says so via the houses list length; otherwise silence.

### Merge veto

**Never merge two hex histories silently.** Thrive vs poor (or any divergent event logs on one hex id) stay separate slots / files. Steward or explicit player choice required to pick one lineage — no auto-blend of climate / standing / week.

### Rates & transport

- **Rates:** soft client propose rate; shard may reject floods as `STALE_SEQ` or drop. Exact caps are a later transport slice.
- **Transport:** WebSocket later. This rev defines JSON shapes + authority rules only — **no listen socket**, no WS client in the default binary, no server unpark. Online stays grey.
- **Dual-repo ingest:** optional `POWRUSH_INGEST` (L3) may soft-write `data/powrush_lived_tick.json` for Ra-Thor lattice read. Lattice does not write Powrush L0. Default ingest off.

## L0 disk paths (offline authority)

| path | role |
| --- | --- |
| `data/powrush_house.json` | House name / Unnamed |
| `data/powrush_shard_climate.json` | climate ledger |
| `data/powrush_shard_standing.json` | standing ledger |
| `data/powrush_week_audit.json` | week tons + restored |
| `data/powrush_hour_two.json` | hour-two / book progress |
| `data/powrush_lived_tick.json` | optional L3 ingest composite (off by default) |

## Flags & refused as v1

- `POWRUSH_NET=off` **default**. When off: offline client authority; Online UI stays honest *off (no listen)*.
- Postcard / mail / listen server / fake peers / server unpark / Ra-Thor Cargo path dep — **refused as v1**.

## Tests that must exist

Shared `hex_protocol` + `hex_join` (or equivalent) must cover:

1. `reject_declare_lethal_before_book_is_no_book` — declare_lethal without book → `NO_BOOK`
2. `reject_take_on_tired_is_no_take` — take while tired → `NO_TAKE`
3. `presence_has_no_n_online_field` / `presence_payload_rejects_client_n_online` — houses only; client cannot author `n_online`
4. `rev_mismatch_is_proto` — `rev != 1` / wrong pid → `PROTO`
5. `disconnect_mid_tend_keeps_last_snapshot_and_book` — fixture round-trip; book flags survive
6. `two_event_logs_diverge_climate_thrive_vs_poor` — same hex id, divergent climates; no silent merge
7. `copy_denied_hello_no_keeps_offline` — `COPY_DENIED` / `hello_no` → AuthorityMode::Offline
8. Envelope round-trip serde for a sample op + reject
9. Default net path does not claim listen / server unparked

## Related

`LAUNCH_UX.md` · `SHARD_JOIN.md` · T-net honest mode · L3 lived-tick ingest · Peace keys law.

**Thunder locked in.** Yoi ⚡
