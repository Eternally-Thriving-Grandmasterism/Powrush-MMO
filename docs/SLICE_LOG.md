# SLICE_LOG.md — Powrush-MMO

## 2026-09-06 — Localhost shard WS (F8)

```
slice: Localhost shard WS (F8)
idea: powrush-shard --listen 127.0.0.1:7788 accepts JSON WS (hello→hello_ok/no; tend/take/flow/reserve apply); loopback-only bind refuse 0.0.0.0; POWRUSH_NET=localhost gates client outbound; default off zero sockets; title Online stays grey; presence=houses.len(); persist --data; drop→Offline book intact
joy 0.98 | mercy 0.98 | sustain 0.97 | veto no
verdict: SHIP
receipt: pending-merge
```

## 2026-09-06 — Parked powrush-shard binary (F7)

```
slice: Parked powrush-shard binary (F7)
idea: parked crate powrush-shard (not workspace member / not client door); shared hex_shard_apply load snapshot + apply tend/take JSONL + write ledger; --listen accepted but parked/not enabled; soft cap 32 Houses; lethal-before-book reject; Core green
joy 0.98 | mercy 0.98 | sustain 0.97 | veto no
verdict: SHIP
receipt: 82dbb4bf (#247)
```

## 2026-09-06 — Join / drop / presence offline fallback (F2–F4)

```
slice: Join / drop / presence offline fallback (F2–F4)
idea: PROTOCOL+SHARD_JOIN authority (offline client / online shard); join=copy-with-consent yard remains; leave/drop=last snapshot offline no login wall; presence houses-only no client n_online; never silent hex merge; hex_join reject tests; no listen
joy 0.98 | mercy 0.98 | sustain 0.97 | veto no
verdict: SHIP
receipt: 61103815 (#246)
```

## 2026-09-06 — Shard protocol + shared net types (F1)

```
slice: Shard protocol + shared net types (F1)
idea: docs/PROTOCOL.md powrush.hex.v1 rev1 + shared/hex_protocol Envelope/Op/RejectCode/Presence/Snapshot; offline client authority default; POWRUSH_NET=off; no listen / no WS / no server unpark; reject NO_BOOK/NO_TAKE/PROTO; presence houses-only
joy 0.98 | mercy 0.98 | sustain 0.97 | veto no
verdict: SHIP
receipt: a5f9eb4c (#245)
```

## 2026-09-06 — Optional lived-tick ingest (L3)

```
slice: Optional lived-tick ingest (L3)
idea: POWRUSH_INGEST default off; when on soft-write versioned data/powrush_lived_tick.json (house·climate·standing·week·hour flags); no Ra-Thor dep; Mode B offline unchanged
joy 0.98 | mercy 0.98 | sustain 0.97 | veto no
verdict: SHIP
receipt: 42d5eb06 (#244)
```


## 2026-09-06 — Pause / Ledger face (L2)

```
slice: Pause / Ledger face (L2)
idea: I satchel + L Ledger sash show House name (or Unnamed House) · week tons+restored · lethal only if declared; no peer count / talent / fake online; no second HUD
joy 0.98 | mercy 0.98 | sustain 0.97 | veto no
verdict: SHIP
receipt: fe6ddedd (#243)
```

## 2026-09-06 — Title continue truth (L1)

```
slice: Title continue truth (L1)
idea: Continue Unnamed House + yard remembers; Esc-from-title preserves persist; Online disabled honest; SmolStr drain; first-run no wall hardened
joy 0.98 | mercy 0.99 | sustain 0.98 | veto no
verdict: SHIP
receipt: e7b60943 (#242)
```

## 2026-09-06 — Title + Continue + skippable House name

```
slice: Title + Continue + skippable House name
idea: S0 title (Play/Continue/Settings; Online grey); S2 skippable House name → data/powrush_house.json; first run no name wall; Continue restores book+climate+standing path; lethal false until L3
joy 0.98 | mercy 0.99 | sustain 0.97 | veto no
verdict: SHIP
receipt: 5db1c0ad (#240)
```

## 2026-09-06 — P2 stranger-path feel juice

```
slice: P2 feel (stranger-path juice)
idea: unmute week after Mend/Lane; soft well + week slab breath; no second HUD
joy 0.98 | mercy 0.98 | sustain 0.97 | veto no
verdict: SHIP
receipt: c57e626c (#238)
```

## 2026-09-06 — Launch UX steward-house

```
slice: Launch UX steward-house
idea: docs/LAUNCH_UX.md — offline-first Title→Hands→House→Online; S0–S4; veto list; Joy/Mercy/Sustain; Traditional vs steward contrast
joy 0.98 | mercy 0.99 | sustain 0.98 | veto no
verdict: SHIP
receipt: a6969048 (#239)
```

## 2026-09-05 — Stranger-loop proof

```
slice: Stranger-loop proof
idea: one-page keys script + persist/flag round-trip (hour2/3, climate, standing, week); Peace lethal silent
joy 0.98 | mercy 0.98 | sustain 0.97 | veto no
verdict: SHIP
receipt: 84f7430c (#236)
```

## 2026-09-05 — Playable-loop polish

```
slice: Playable-loop polish
idea: stranger 40–90 min offline cues — yard → House → book → climate → week → optional lethal
joy 0.98 | mercy 0.98 | sustain 0.97 | veto no
verdict: SHIP
receipt: 5b41628e (#234)
```

## 2026-09-05 — Mythic verbs

```
slice: Mythic
idea: Witness/Offer/Attune read-first after book+seat; gate hour_three
joy 0.98 | mercy 0.99 | sustain 0.97 | veto no
verdict: SHIP
receipt: 141505d0 (#233)
```

## 2026-09-05 — Lethal

```
slice: Lethal
verdict: SHIP
receipt: 03bddc41 (#232)
```