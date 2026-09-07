# SLICE_LOG.md — Powrush-MMO

## 2026-09-07 — F1 stranger floor 5c18a68

```
slice: F1 stranger floor 5c18a68
idea: stranger/door floor honesty — tip 5c18a68 in README + PREVIEW_CHECKLIST + STRANGER_LOOP; replace older floors 168a990b / 4a3d7681 / da38fb26; comfort + canons live; Online grey; not a launch candidate; docs only; no client/shared/Cargo; no meshes/keys/birds; no preview tag; no POWRUSH_NET
joy 0.98 | mercy 0.99 | sustain 0.97 | veto no
verdict: SHIP
receipt: d99a1db (#271)
```



## 2026-09-07 — fail beat Esc opens pause plate

```
slice: fail beat Esc opens pause plate
idea: InYard Esc opens D1 pause *the yard is waiting* (Resume / Title / Quit) instead of Title; Esc again = Resume; Title button keeps house JSON + lived persist; Quit = AppExit; Digit3 toggle kept; Online grey; lavapipe Play→Esc confirm; no preview / POWRUSH_NET / birds / Lethal
joy 0.98 | mercy 0.99 | sustain 0.97 | veto no
verdict: SHIP
receipt: b749d9e8 (#269)
```



## 2026-09-07 — fail beat B0001 dress labels

```
slice: fail beat B0001 dress labels
idea: Bevy 0.14 B0001 — refresh_house_dress_labels (+ refresh_local_settings_labels) multi Query<&mut Text> → ParamSet; lavapipe boot window no panic; no preview / POWRUSH_NET / birds
joy 0.98 | mercy 0.99 | sustain 0.97 | veto no
verdict: SHIP
receipt: 690c6414 (#267)
```



## 2026-09-07 — F0 README stranger floor sync

```
slice: F0 README stranger floor sync
idea: README floor honesty — replace stale play floor c5299d11 with stranger floor 168a990b / main tip da38fb26; comfort + canons live; not a launch candidate; docs only; no client/shared/Cargo; no meshes/keys/birds; no preview tag; no POWRUSH_NET
joy 0.98 | mercy 0.99 | sustain 0.97 | veto no
verdict: SHIP
receipt: 4a3d7681 (#265)
```



## 2026-09-06 — E1 checklist + stranger-loop sync

```
slice: E1 checklist + stranger-loop sync
idea: PREVIEW_CHECKLIST + STRANGER_LOOP sync Esc→pause *the yard is waiting* · settings persist (look/mute/invert-Y/hide slabs/brightness/text scale) · Well/Grove/Ember skippable + heritage string only + Q seal · no combat stats; door cargo run -p powrush-client; Online grey; stranger floor start c8ec538c + E1 (stamp tip after merge); ticking ≠ playable-preview tag
joy 0.98 | mercy 0.99 | sustain 0.97 | veto no
verdict: SHIP
receipt: 168a990b (#263)
```



## 2026-09-06 — After-D3 comfort pack

```
slice: After-D3 comfort pack
idea: Settings brightness + text_scale persist (Title opaque contrast law); Mute-from-pause = MasterMute; Q/Pause face Seal · … when dressed (heritage string only); README floor c5299d11 + Online grey; fog/birds PARKED
joy 0.98 | mercy 0.99 | sustain 0.97 | veto no
verdict: SHIP
receipt: 96f17697 (#261)
```



## 2026-09-06 — D3 House seals + heritage caption

```
slice: D3 House seals + heritage caption
idea: after Settled / skip-named — three skippable Peace-tone seals (Well · Grove · Ember) cosmetic only; optional heritage caption string none|human|cydruid|quellorian|draek|ambrosian; rename OK; persist on powrush_house.json; refuse +take/+STR/combat; no race Title / Peace keys / Online / preview
joy 0.98 | mercy 0.99 | sustain 0.97 | veto no
verdict: SHIP
receipt: d620079c (#259)
```



## 2026-09-06 — D2 Local settings persist

```
slice: D2 Local settings persist
idea: Title/pause Settings plate — Look · Mute · Invert-Y · Hide slabs; persist data/powrush_settings.json beside house; defaults = Peace hour; Online grey no socket / no POWRUSH_NET=on; H still hides; no new Peace verbs
joy 0.98 | mercy 0.99 | sustain 0.97 | veto no
verdict: SHIP
receipt: 27a543f6 (#257)
```


## 2026-09-06 — D1 Pause honesty plate

```
slice: D1 Pause honesty plate
idea: one-line opaque pause plate when Settings/pause opens in yard — "the yard is waiting"; Resume (stay InYard) / Title (Esc-to-title + house JSON + lived persist) / Quit (AppExit, not Esc); Esc yard→Title kept; no new sim / Peace keys / Online / race select
joy 0.98 | mercy 0.99 | sustain 0.97 | veto no
verdict: SHIP
receipt: f2a6411f (#256)
```


## 2026-09-06 — GDD 1.5–2.0 adaptation map

```
slice: GDD 1.5–2.0 adaptation map
idea: docs/GDD_ADAPTATION.md — name map Druid→Cydruid / Quelorian / Draexx→Draek; keep/transform/refuse; heritage after House only; XP-from-kills refused; no class tree / iso HUD / UR gambling / wallet / preview tag / net-on
joy 0.98 | mercy 0.99 | sustain 0.97 | veto no
verdict: SHIP
receipt: 78c880f7 (#254)
```

## 2026-09-06 — Physics / graphics canon

```
slice: Physics / graphics canon
idea: docs/PHYSICS_GRAPHICS_CANON.md — three surfaces, four physics layers, Sanctuary graphics, Crownstone Witness-only map, Title PASS on c5299d11; no new verbs / no race select / no Brood Spire in Sanctuary
joy 0.98 | mercy 0.99 | sustain 0.97 | veto no
verdict: SHIP
receipt: 9ab10821 (#253)
```

## 2026-09-06 — Stranger-pass: title contrast + Esc→Title + L wait + house file

```
slice: Stranger-pass title contrast + Esc→Title + L wait + house file
idea: opaque high-contrast title plate (soft GPU readable); Esc InYard→Title (not quit-desktop); L never blank (Not your charter / the ledger waits) + L2 face when charter; house JSON on Settled/quit-to-title even if Unnamed; tick blob = session persist not Ra-Thor ingest; stamp W1–W3 receipt ae2589b4 (#251)
joy 0.98 | mercy 0.99 | sustain 0.97 | veto no
verdict: SHIP
receipt: c5299d11 (#252)
```

## 2026-09-06 — W1–W3 waiting pack (P10 receipt + no-dial banner)

```
slice: W1–W3 waiting pack (P10 receipt + no-dial banner)
idea: steward side work while waiting for stranger pass — stamp P10 receipt a678b035 (#250); PROTOCOL+F9 no-dial banner (default client does not dial; POWRUSH_NET=off zero sockets; loopback WS opt-in via env/Settings not title Online); POWRUSH_INGEST off-by-default note + no client overlay + Ra-Thor may read ticks never drives keys; no preview tag / Online live / public bind / feel juice / new verbs
joy 0.98 | mercy 0.98 | sustain 0.97 | veto no
verdict: SHIP
receipt: ae2589b4 (#251)
```

## 2026-09-06 — Preview checklist + stranger loop sync (P10 honesty)

```
slice: Preview checklist + stranger loop sync (P10 honesty)
idea: docs honesty pack — README default one-human/one-machine + Dev link to F9_TWO_CLIENT_LOCALHOST (not Features); STRANGER_LOOP sync Title/Continue/Unnamed House + Online grey + L3 absent first hour (minute keys); new PREVIEW_CHECKLIST ~15 human ticks (ticking ≠ playable-preview tag); stamp F9 receipt 7d9a733a (#249); no Steam/launch-speak/fake presence/net-on default/0.0.0.0/new verbs
joy 0.98 | mercy 0.98 | sustain 0.97 | veto no
verdict: SHIP
receipt: a678b035 (#250)
```

## 2026-09-06 — Two-client same-hex localhost recipe (F9)

```
slice: Two-client same-hex localhost recipe (F9)
idea: docs-only dev recipe — one powrush-shard --listen 127.0.0.1:7788 + two client processes POWRUSH_NET=localhost (separate CWDs); expect presence len 2, one climate ledger, take-on-tired NO_TAKE, drop→offline book intact; NOT a store/title feature; Online grey; no public bind; stranger offline-first; pointers in PROTOCOL+PARKED; reuse hex_listen unit helpers (no flaky live WS in Core)
joy 0.98 | mercy 0.98 | sustain 0.97 | veto no
verdict: SHIP
receipt: 7d9a733a (#249)
```

## 2026-09-06 — Localhost shard WS (F8)

```
slice: Localhost shard WS (F8)
idea: powrush-shard --listen 127.0.0.1:7788 accepts JSON WS (hello→hello_ok/no; tend/take/flow/reserve apply); loopback-only bind refuse 0.0.0.0; POWRUSH_NET=localhost gates client outbound; default off zero sockets; title Online stays grey; presence=houses.len(); persist --data; drop→Offline book intact
joy 0.98 | mercy 0.98 | sustain 0.97 | veto no
verdict: SHIP
receipt: b16462f7 (#248)
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