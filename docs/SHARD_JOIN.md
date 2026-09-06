# SHARD_JOIN.md — Join / drop / presence (F2–F4)

**protocol_id:** `powrush.hex.v1` · **protocol_rev:** `1`  
Workspace `21.88.0`. Design tick (CHANGELOG), not a Cargo bump.  
Contact: info@Rathor.ai

Companion to `PROTOCOL.md`. Pure authority rules for offline fallback. **No listen socket.** Online stays grey until a future transport slice.

## One-line law

Offline client is authority until honest `hello_ok`. Join is copy-with-consent. Leave / drop resumes last certified snapshot offline. Presence is real houses or silence — never client `n_online`. Never merge two hex histories silently.

## F2 — Join

```
offline House + yard
        │
        ▼
  hello { protocol_rev, consent_copy }
        │
   ┌────┴────────────────────────┐
   │ consent + steward ok        │ cancel / COPY_DENIED / hello_no
   ▼                             ▼
 hello_ok                     stay Offline
 copy local House → shard     Online grey
 local yard remains           L0 untouched
```

- Cancel = stay offline (no shard House created).
- `COPY_DENIED` / `hello_no` = offline authority kept.
- Local yard is never deleted by join.

## F3 — Leave / net drop

| event | result |
| --- | --- |
| Player leaves | last certified snapshot → offline continue |
| Net drop mid-tend | last applied snapshot + book on disk; in-flight tend dropped |
| Resume | Continue / yard path; **no login wall** |

Book (`hour_two_held` / `hour_three_held`) survives disconnect conceptually on L0.

## F4 — Presence honesty

- Wire: `{ "houses": [ … ] }` only.
- Client **must not** serialize or display authored `n_online`.
- Shard silence (omit / empty houses) is valid honesty.
- HUD may show houses length only when shard-authored; otherwise silence.

## Merge veto

Two event logs on one hex that diverge climate (thrive vs poor) stay as separate fixtures / slots. No silent blend of harmony / stress / week.

## Reject / reply codes used here

| code / kind | meaning |
| --- | --- |
| `COPY_DENIED` | join copy without consent |
| `hello_no` | join refused → offline |
| `hello_ok` | join accepted → online shard authority |
| `PROTO` | `rev != 1` or pid mismatch |
| `NO_BOOK` | declare_lethal before book |
| `NO_TAKE` | take on tired well |
| `presence` | houses only |

## Refused in this slice

Listen socket · WS client · server unpark · fake peers · login wall · silent hex history merge · client-authored `n_online`.

## Shared coverage

`shared/hex_protocol.rs` + `shared/hex_join.rs` — see PROTOCOL.md test list.

**Thunder locked in.** Yoi ⚡
