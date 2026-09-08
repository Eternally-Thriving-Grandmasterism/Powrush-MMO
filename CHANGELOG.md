# CHANGELOG.md — Powrush-MMO

## [23.2.81] — 2026-09-08 — U7: Threshold shelf

Heartwood now carries a roof-height Threshold shelf outside both the empty lamp disk and the water bath. Its complete local vocabulary is look and tend on the existing Peace Use; neither action writes a hex stub over the house book, so `hour_three_complete`, the embassy seat, Places eligibility, and Sanctuary return remain held. No Market, socket, public bind, or Title Online path was added. Seed hashing and isolation gamma remain unchanged. `playable-preview` stays `11c577e`. Floor stays `2163551`. Workspace stays 21.88.0.

## [23.2.80] — 2026-09-08 — U6: Heartwood Lip

Heartwood now places two walkway capsules and hanging roof ribs on dry Lip ground, with every mesh footprint outside the empty lamp disk and no Heartwood mesh on Sanctuary. Stepping into the pond is a local bath back to the Lip; it does not touch the house pack, so `hour_three_complete`, the embassy seat, and Places return survive. Seed hashing and hex isolation remain unchanged. Title Online stays grey. `playable-preview` stays `11c577e`. Floor stays `2163551`. Workspace stays 21.88.0.

## [23.2.79] — 2026-09-08 — U5: Steam Deck title click-clean

The default window is 1280×800. The unchanged 420px Title plate now has a 24px responsive safe-area guard and a full-screen pointer-focus blocker, keeping Play, Continue, Settings, and grey Online inside the Deck surface and preventing click-through to the world. The dedicated UI camera remains above the single world Camera3d; soft-GPU MSAA remains off. Title Online stays grey with no listen. `playable-preview` stays `11c577e`. Floor stays `2163551`. Workspace stays 21.88.0.

## [23.2.78] — 2026-09-08 — U4: Peace yard audio bed + well sting

Quiet Peace-yard bed plus a well sting on the existing well Use. Pause/Settings Mute silences both; unmuted does not open a socket. No ALSA card: AudioPlugin stays off so boot cannot hang. Title Online stays grey. `playable-preview` stays `11c577e`. Floor stays `2163551`. Workspace stays 21.88.0.

## [23.2.77] — 2026-09-08 — U3: Heartwood lamp spatial rules

Heartwood hex generation refuses buildings in water and in the lamp disk. The disk stays empty of structures. Embassy lamp stays the house seat cue (display-only on the stub) — not the spatial gate. Same Peace E. Title Online stays grey. `playable-preview` stays `11c577e`. Floor stays `2163551`. Workspace stays 21.88.0.

## [23.2.76] — 2026-09-08 — U2 fail beat: house book survives Heartwood stub

Travel must not drop `hour_three_complete` or replace the house embassy seat with the Heartwood stub. Places stays a live door on Sanctuary and the stub so confirm leave can go and return. Hex climate still writes only `powrush_hex_<id>.json`. Title Online stays grey. `playable-preview` stays `11c577e`. Floor stays `2163551`. Workspace stays 21.88.0.

## [23.2.75] — 2026-09-08 — U2: local hex travel (disk only)

After Settled + book, Places (Sanctuary / Heartwood) can confirm leave, write `powrush_hex_<id>.json` into the U1 user dir, and load the other place. Play always boots Sanctuary Prime. Continue without the book still boots Sanctuary. Heartwood is a stub (lamp empty, same Peace E, no hanging mesh). Isolation gamma = 0. Title Online stays grey. `playable-preview` stays `11c577e`. Floor stays `2163551`. Workspace stays 21.88.0.

## [23.2.74] — 2026-09-08 — U1: writable user-dir saves

House, settings, GenShare, and the other client `powrush_*` JSON land in a writable OS user-data dir: Linux `$XDG_DATA_HOME/powrush` or `~/.local/share/powrush`; Windows `%LOCALAPPDATA%\Powrush`; macOS `~/Library/Application Support/Powrush`. Lab / lavapipe: `POWRUSH_USER_DIR` (directory that holds the files). Cwd `data/` is adopted only when that user dir is empty of `powrush_*` files. Fresh first boot writes a new house there — not the F-book fixture. Title Online stays grey. `playable-preview` stays `11c577e`. Floor stays `2163551`. Workspace stays 21.88.0.

## [23.2.73] — 2026-09-08 — U0: Steam Offline SKU law (docs only)

Docs only: SKU = Steam Offline; full MMO is a second product. Title Online stays *off (no listen)*. LAN default off. Hexes are disk save-slots sharing one House and one House-week footer; per-hex climate; isolation (gamma = 0). Play boots Sanctuary Prime. User-dir persist is install law (U1 implements). `playable-preview` stays `11c577e`. Floor stays `2163551`. Minutes / OS / GPU / Time left blank. Workspace stays 21.88.0.

## [23.2.72] — 2026-09-08 — F-book: Settled+book fixture (harm row off, test-only)

Test-only Settled + hour-three / book fixture at `tests/fixtures/f-book/data/` so a lavapipe walk can see *this hex admits harm · off*. `declared_lethal` stays false; no tons minted; week stays tons + restored. Not written to default `data/`; fresh `cargo run -p powrush-client` still boots first hour. Parent copies the fixture into a temp cwd `data/` — do not copy into the repo door. `playable-preview` stays `11c577e`. Floor stays `2163551`. Workspace stays 21.88.0.

## [23.2.71] — 2026-09-08 — E5: L1 receipt 7ea48328 (lethal-off walk, minutes blank)

Docs only: L1 hex sign is on main `7ea48328` (#295). After Settled + book, *this hex admits harm* stays default **off**; no ton mint; Sanctuary E unchanged. First hour the row must not arm — lavapipe walk saw *Not your charter* (inert, unclicked). Online grey; no listen; nothing on `0.0.0.0`. `playable-preview` stays `11c577e`. Floor stays `2163551`. Minutes / OS / GPU / Time left blank. Workspace stays 21.88.0.

## [23.2.70] — 2026-09-08 — L1: DeclareLethal hex sign (this hex admits harm)

After Settled + book, the human may optionally confirm lethal on **this hex only**. Confirm lives on Settings and the existing Q/Ledger face: **this hex admits harm** (default **off**; missing/unknown → off). Reuses `ShardStanding::declare_lethal` / `clear_lethal` — no second flag. Confirm without Settled + book does nothing; the plate says *the ledger waits* / *Not your charter*. Tariff may raise stress / dent harmony on that hex only; week stays tons + restored (no ton mint). Peace yard and Sanctuary **E** unchanged. No combat as default Use. No new Peace keys. Title Online stays grey. LAN row stays off|loopback (default off). `playable-preview` stays `11c577e`. Floor stays `2163551`. Workspace stays 21.88.0. L1 is a sign, not a weapon.

## [23.2.69] — 2026-09-08 — P3: Settings LAN off / loopback

Settings plate **LAN · off|loopback** persists beside Grove in `data/powrush_settings.json` (default **off**; missing/unknown/`on` → off). Loopback reuses the existing F8 localhost door: listen/bind **127.0.0.1** only (`127.0.0.1:7788`), never `0.0.0.0`, never a public bind, never `POWRUSH_NET=on` from Title. Title Online stays grey and does not bind; Settings Online stub hard-refuse stays. LAN off boots exactly as today — no listen, no outbound. Loopback may send the existing L0 GenShare line + climate only. No fake peers, no player counts, no combat, no birds, no default Grove, no Depths mesh, no XP, no second Camera3d. `playable-preview` stays `11c577e`. Floor stays `2163551`. Workspace stays 21.88.0.

## [23.2.68] — 2026-09-08 — E4: preview tag 11c577e + welcome-back breath (minutes blank)

Docs only: `playable-preview` stays walked SHA `11c577e` (no retag); tip `3fc46c9` welcome-back is the same slab breathing once, then rest — first boot quiet, no XP. Floor stays `2163551`. Online grey; no public bind; no sockets / LAN (P3 next) / birds / default Grove / combat. Minutes / OS / GPU / Time left blank for the human. Workspace stays 21.88.0.

## [23.2.67] — 2026-09-08 — Hour-two: welcome-back reward glow (existing slab)

Returning after Hour two held: the existing Welcome slab border breathes once (well/bench glow decay), then rests. First boot stays quiet — no glow without the held pack. No new HUD, verbs, or XP. Peace keys unchanged. Workspace stays 21.88.0.

## [23.2.66] — 2026-09-07 — Wave P: playable-preview tag receipt (11c577e)

Docs only: steward preview yes; annotated `playable-preview` @ tip `11c577e`; play floor stays `2163551`; Online grey; no GenShare sockets / birds / default Grove from tag alone. Workspace stays 21.88.0.

## [23.2.65] — 2026-09-07 — GenShare L0: persist grove seed JSONL (offline, no port)

Method A offline recipe: `data/powrush_genshare.jsonl` append-only beside house/climate; `shared/genshare` envelope + unit tests (roundtrip, epoch wins); Grove light path loads matching house⊕hex seed or computes+appends; HexScatter alias; defer rebuild while plates open. Optional climate `seed_u64`/`gen_epoch` (Method D soft). No sockets, Title Online, Wave P, postcard, or Method B UI. Workspace stays 21.88.0.



## [23.2.64] — 2026-09-07 — I0 Controls + one Use map

One Use verb across keyboard/mouse, gamepad, and touch overlay (`INPUT_CANON`): **E / South / overlay Use**; Jump = Space / LB; Start = Pause; on-screen sticks `auto|on|off` (default auto — mouse Title stays click-clean; cull on plates); Settings Controls rows persist beside Grove. Nintendo face remap stub (`auto`). No second Camera3d, Online socket, birds, Wave P, or preview tag. Workspace stays 21.88.0.

## [23.2.63] — 2026-09-07 — G0.5 Settings Grove off/light

Settings plate **Grove · off|light** persists in `data/powrush_settings.json` (default **off**; missing/unknown → off). Enabled when Grove is light **OR** env `POWRUSH_GEN=light` — same G0 light-gen path, not a second system. Cull when Title/pause/Settings/L/Q open. No birds, sockets, second Camera3d, Avian/Rapier, GenShare, preview tag. Peace keys untouched. Workspace stays 21.88.0.

## [23.2.62] — 2026-09-07 — G0 light gen (hex-seed scatter + climate fog)

Optional `POWRUSH_GEN=light` (default **off** — lavapipe door-safe): deterministic grove seed from house⊕hex⊕climate_epoch; tiny atlas trees/stones (≤4 mesh types, ≤12 instances); FogSettings on world Camera3d only from stress/harmony; cull scatter when Title / pause / Settings / L / Q open; no birds, no second Camera3d, no Avian/Rapier, no combat stats, no sockets, no Wave P. Shared `powrush_gen` unit tests (seed stability, flag off by default, cull helper). F3 re-walk with env set; revert if occlusion. Workspace stays 21.88.0.

## [23.2.61] — 2026-09-06 — Stranger-pass: title contrast + Esc→Title + L wait + house file

Title door readable on soft GPU / Mesa: opaque light-on-dark plate (no alpha-on-fog); Esc from InYard → `LaunchDoor::Title` (quit via window close / Settings). L sash never blank before Settled — *Not your charter* / *the ledger waits*; L2 house+week face when charter live; I keeps abundance copy. `data/powrush_house.json` written on Settled offer and quit-to-title even if name skipped (Unnamed). Tick path honesty: default `powrush_lived_tick.json` is session persist, not Ra-Thor ingest; checklist “no tick” = no ingest overlay; blob kept. Stamps W1–W3 waiting-pack SLICE_LOG receipt `ae2589b4 (#251)`. Peace keys unchanged. No Online live, no preview tag, no Vulkan product requirement, no harvest_feel rewrite. Workspace stays 21.88.0.

## [23.2.60] — 2026-09-06 — W1–W3 waiting pack (P10 receipt + no-dial banner)

Steward side work while waiting for stranger pass (docs only). Stamps P10 SLICE_LOG receipt `pending-merge` → `a678b035 (#250)`. Top-of-doc **no-dial banner** on `docs/PROTOCOL.md` + `docs/F9_TWO_CLIENT_LOCALHOST.md`: default client **does not dial**; `POWRUSH_NET=off` (default) opens zero sockets; loopback WS is opt-in via env/Settings, not the title Online row. Strengthen PROTOCOL steward law never-listens → does-not-dial. Explicit L3 note: `POWRUSH_INGEST` off by default; writes `data/powrush_lived_tick.json` only when on; **no client overlay**; Ra-Thor may read ticks, never drives keys (`PROTOCOL` + `PARKED_SURFACES`). Waiting-pack SLICE_LOG entry receipt pending-merge. No preview tag, no Online live, no public bind, no feel juice, no new verbs. Workspace stays 21.88.0.

## [23.2.59] — 2026-09-06 — Preview checklist + stranger loop sync (docs / honesty)

P10 honesty pack (docs only): README default is **one human, one machine**; Title / Continue / *Unnamed House* called out; Dev section links `docs/F9_TWO_CLIENT_LOCALHOST.md` (not a Features/store claim). `docs/STRANGER_LOOP.md` synced to L1 title truth + minute keys; Online grey; L3 ingest absent on first hour. New `docs/PREVIEW_CHECKLIST.md` (~15 human tick boxes) — explicit: ticking ≠ cutting `playable-preview` tag. Stamps F9 SLICE_LOG receipt `7d9a733a (#249)`. No Steam copy, no launch-speak, no fake presence, no `POWRUSH_NET=on` default, no `0.0.0.0` bind, no new verbs. Workspace stays 21.88.0.

## [23.2.58] — 2026-09-06 — F9 two-client same-hex localhost recipe (docs / dev only)

F9 **dev recipe** (not a store or title feature): `docs/F9_TWO_CLIENT_LOCALHOST.md` — build/run `powrush-shard --listen 127.0.0.1:7788 --data …`; launch **two** client processes with `POWRUSH_NET=localhost` from separate CWDs (separate L0 `data/` books); expect presence length 2, one shared climate ledger, take-on-tired `NO_TAKE`, drop → offline book intact. Title Online stays **grey**; no public bind; stranger pass stays offline-first. Short pointers in `PROTOCOL.md` + `PARKED_SURFACES.md`. Reuses existing `hex_listen` unit helpers (presence=2, NO_TAKE, drop intact, refuse public bind) — no flaky live WS integration required in Core. Also stamps F8 SLICE_LOG receipt `b16462f7 (#248)`. Workspace stays 21.88.0.

## [23.2.57] — 2026-09-06 — Localhost shard WS (flag-gated; title Online grey)

F8: `powrush-shard --listen 127.0.0.1:7788` accepts JSON WebSocket envelopes (tokio-tungstenite). Bind **loopback only** — refuse `0.0.0.0` / non-loopback. `hello` → `hello_ok` / `hello_no` (v1 may hello_ok a second local House on same hex). `tend` / `take` / `flow` / `reserve` apply via `shared/hex_protocol` + `hex_shard_apply` with `NO_TAKE` / `NO_BOOK` / `STALE_SEQ` / `PROTO`. Persist `ledger_snapshot.json` under `--data`. Client drop → Offline; book/house intact. Presence = `houses.len()` from real seats. Client connects **only** if `POWRUSH_NET=localhost`; default `off` opens zero sockets. Title Online row stays **grey**. `powrush-shard` remains outside default-members / not client door. Shared `hex_listen` bind + hello helpers + tests. Also stamps F7 SLICE_LOG receipt `82dbb4bf (#247)`. Workspace stays 21.88.0.

## [23.2.56] — 2026-09-06 — Parked powrush-shard binary (not default door)

F7 parked crate `powrush-shard/` (commented beside `server/` in workspace members — **not** default `cargo run -p powrush-client`). Shared `hex_shard_apply`: load one hex ledger snapshot, apply verb events (tend/take/…) from JSONL via `hex_protocol` reject helpers, write `ledger_snapshot.json`. CLI `--hex` / `--data` / `--dry-apply`; `--listen` accepted but prints parked / not enabled (no WS bind). Soft cap **32 Houses** documented. Tests: offline tend/take on fixture; lethal-before-book → NO_BOOK; client default still no listen; soft-cap seat. No Online lighting, no login wall, no fake peers, no Ra-Thor path dep, no postcard v1. Also stamps F2 SLICE_LOG receipt `61103815 (#246)`. Workspace stays 21.88.0.

## [23.2.55] — 2026-09-06 — Join / drop / presence + authority rules (offline fallback)

F2–F4 steward docs + reject tests: expand `docs/PROTOCOL.md` join/leave/presence/authority; add `docs/SHARD_JOIN.md`. Offline client authority until honest `hello_ok`; join = copy local House → shard House **with consent** (local yard remains; cancel/`COPY_DENIED`/`hello_no` stay offline); leave/net drop = last certified snapshot + book on disk, continue offline, **no login wall**; presence = houses only — client cannot author `n_online`; never merge two hex histories silently (thrive vs poor fixtures). Shared `hex_join` helpers + tests (NO_BOOK, NO_TAKE, disconnect mid-tend fixture, presence reject, `rev != 1` → PROTO, diverge climate, COPY_DENIED keeps offline). **No listen socket**, no WS, no server unpark; Online stays grey. Also stamps F1 SLICE_LOG receipt `a5f9eb4c (#245)`. Workspace stays 21.88.0.

## [23.2.54] — 2026-09-06 — Shard protocol + shared net types (offline fallback)

F1 steward protocol: `docs/PROTOCOL.md` locks `protocol_id` `powrush.hex.v1` / `protocol_rev` `1`, envelope fields (v/pid/kind/hex/house/seq/ts_ms/body), offline client authority vs online shard authority, client ops (tend/take/flow/reserve/mend/lane/bind/declare_lethal/clear_lethal/name_house/request_seat/snapshot_req/hello), shard replies (apply/reject/snapshot/week/presence/hello_ok/hello_no), ledger snapshot shape, reject codes (STALE_SEQ/NO_TAKE/NOT_CHARTER/NO_BOOK/NO_PACK/BAD_RESERVE/TELEPORT/PROTO/COPY_DENIED), join copy-with-consent, presence = houses array only, L0 disk paths, dual-repo ingest note, WS transport later. Shared `hex_protocol` serde types + pure reject helpers; **no listen socket**, no WS client in default client, no server unpark. `POWRUSH_NET=off` default. Peace keys unchanged. No Ra-Thor Cargo path dep. No fake peers. Also stamps L3 SLICE_LOG receipt `42d5eb06 (#244)`. Workspace stays 21.88.0.

## [23.2.53] — 2026-09-06 — Optional lived-tick ingest (off by default)

L3 optional lattice ingest: env `POWRUSH_INGEST` default **off**. When `on`/`1`/`true`, soft-write versioned `data/powrush_lived_tick.json` (schema `powrush_lived_tick_v1`) with house id/name, climate snapshot, standing (incl `declared_lethal`), week tons+restored, hour flags; nested hour keeps Mode B resume. When off, ingest write is never called — bare LivedHour persist unchanged. Shared `lived_tick_ingest` + thin client hook. Soft-fail I/O; never blocks WASD. No Ra-Thor checkout / path dep. No Online enable, no server unpark, no login wall, no public launch speak. Also stamps L2 SLICE_LOG receipt `fe6ddedd (#243)`. Workspace stays 21.88.0.


## [23.2.52] — 2026-09-06 — Pause / Ledger face (house + week)

L2 S3 Pause/Ledger face on existing panels: **I** (satchel) and **L** (Ledger sash) show House name or *Unnamed House*, week line (`this week · N tons · M restored`), and lethal clause only when already `declared_lethal`. No talent tree, no peer count, no fake online, no second HUD. Shared `pause_ledger_face` + proof tests. Peace keys WASD E I H R unchanged. No harvest_feel / rbe rewrite. server/ parked. Workspace stays 21.88.0.


## [23.2.51] — 2026-09-06 — Title continue truth (Unnamed House + yard remembers)

L1 title truth: Continue shows House name or exactly *Unnamed House* + *the yard remembers* whenever local persist exists. Esc from title closes Settings only — house / climate / standing / book JSON untouched. Online row visible+disabled with honest *off (no listen)* copy. First-run Play still has no name wall. Bevy 0.14 SmolStr ReceivedCharacter drained off NameHouse to stop Continue/name flicker. Proof tests: first-run, Unnamed cue, Esc persist, Online disabled. Peace keys WASD E I H R unchanged. No harvest_feel / rbe rewrite. server/ parked. Default POWRUSH_NET=off. Workspace stays 21.88.0.

## [23.2.50] — 2026-09-06 — Title + Continue + skippable House name

S0 title door: Play / Continue / Settings; Online grey stub (feature off). Continue shows House name + *the yard remembers* when local persist exists. S2 skippable House naming after Settled or Escape quit path → `data/powrush_house.json` beside climate; Skip = Unnamed House. First run has no name wall before Hands. Proof: title_house_proof + house_name tests; Continue restores book+climate+standing flags; lethal false until L3 on Peace/fresh. No login wall, no race/class, no peer count, no harvest_feel rewrite. Workspace stays 21.88.0.

## [23.2.49] — 2026-09-06 — P2 stranger-path feel juice

Mute hole: fabricator MendSpool / LaneCrate now `refresh_climate_slab` so the week audit answers (*this week · N tons · M restored*) instead of standing-only overwrite. Soft well_glow on contest win; WeekFeelGlow breath on the existing climate slab. Bench S+ light kept. R+ fog/pulse kept. No second HUD. No new verbs. No harvest_feel rewrite. Workspace stays 21.88.0.

## [23.2.48] — 2026-09-06 — Launch UX steward-house (docs)

Docs `LAUNCH_UX.md`: offline-first steward law (no account wall before yard; default `cargo run -p powrush-client`; no fake peers / race-class DPS; lethal opt-in after book; Online as mode beside offline House). Screens S0–S4, veto list, Joy/Mercy/Sustain UI notes, Traditional MMO vs steward contrast. Workspace stays 21.88.0.

## [23.2.47] — 2026-09-05 — Stranger-loop proof

One-page `docs/STRANGER_LOOP.md` (keys-only beats). Shared persist/flag proof: hour two held, hour three / book, climate harmony/stress, standing `declared_lethal == false` until Ledger 3, week tons + restored; Peace/fresh fixture non-lethal; quit/rerun JSON round-trip. Resume soft cues already on card / welcome / Ledger sash — no new verbs. Workspace stays 21.88.0.

## [23.2.46] — 2026-09-05 — Playable-loop polish

Docs `PHASE_PLAYABLE_LOOP.md` for the stranger offline path. Soft cues on the existing card + welcome + Ledger sash: after harvest toward House; after Hour two name climate/week; after book Ledger 3 optional (never default E, never Peace lethal UI). Mythic SLICE_LOG receipt fixed. Workspace stays 21.88.0.

## [23.2.45] — 2026-09-05 — Mythic verbs read-first

Witness / Offer / Attune gated on Hour three held + seat. Not damage. Peace unchanged. Workspace stays 21.88.0.

## [23.2.44] — 2026-09-05 — Lethal opt-in Ledger 3

DeclaredLethal after Hour three held only. Digit3 on Ledger. Tariff from reserve else restored debt. Harmony/stress hit. Clear keeps tariff paid. Peace hour unchanged. Week still tons + restored. Workspace stays 21.88.0.

## [23.2.43] — 2026-09-05 — U divergent shard slots

`ShardBank` with thrive vs poor hex slots. Separate paths under `data/shards/`. No race menu. Workspace stays 21.88.0.

## [23.2.42] — 2026-09-05 — T-net honest mode label

`NetMode::Offline` default. `HonestShard` labelled; never fakes Peace-hour peers. server/ still parked. Workspace stays 21.88.0.

## [23.2.41] — 2026-09-05 — T-offline shard sim harness

`ShardSim` time-lapses climate / standing / week without a second human. Not on the boot card. No fake peers. Workspace stays 21.88.0.

## [23.2.40] — 2026-09-05 — S+ civic juice (fabricator bench light)

MendSpool / LaneCrate soft-light the fab slab border. Not a second HUD. Workspace stays 21.88.0.

## [23.2.39] — 2026-09-05 — Phase S week audit preview

Local week line: tons + restored from the climate ledger. Persist `data/powrush_week_audit.json`. One honest slab. No kills, no war HUD. Workspace stays 21.88.0.

## [23.2.38] — 2026-09-05 — R+ climate feel (fog / pulse / sting)

Climate stress closes fog and cools the mist. Harmony opens ambient. Stress spikes pulse the near well (ecological sting, not HP). No second HUD. Workspace stays 21.88.0.

## [23.2.37] — 2026-09-05 — Phase R local hex standing

Standing beside climate at `data/powrush_shard_standing.json`. peace / harmony / consumption / steward; human_hybrid_heat=0; declared_lethal=false (parked). Same verbs write climate + standing. Embassy intact. No race select, no standing HUD. Workspace stays 21.88.0.

## [23.2.36] — 2026-09-05 — Phase Q solo shard climate

One hex ledger at `data/powrush_shard_climate.json`. Same verbs: care-tend / glowing take / tired refuse / flow / reserve / MendSpool / LaneCrate. Face stays well speech. Optional *the well is tired* / *the yard is circulating* on the existing slab. No factions, no War week, no server, no Embassy rewrite. Workspace stays 21.88.0.

## [23.2.35] — 2026-09-05 — Hour three civic door (Proof Pack + Embassy)

Steward-approved proceed. Fabricator after Hour two held + House arrival. MendSpool + LaneCrate → Proof Pack. Embassy E Request seat. Persist *Hour three held · the book is yours*. Welcome slab names the book. Card teaches Q → Embassy → held. No G Voice / War week / Crownstone. Workspace stays 21.88.0.

## [23.2.34] — 2026-09-05 — Teaching sentence align + native work pack

`lived_hour_bind` Flow line matches playtest: *flow restored the well*. Docs: `WORK_PACK_NATIVE.md` (X-Grok collab). No Hour-three Bevy. No harvest_feel rewrite. Workspace stays 21.88.0.

## [23.2.33] — 2026-09-05 — Hour-three delivery map (docs)

Docs only. Locks Hour three order: Proof Pack → Embassy seat → *Hour three held*. Gate: human Report on `docs/HOUR_TWO_PLAYTEST.md`. No Bevy systems. No server. Workspace stays 21.88.0.

## [23.2.32] — 2026-09-05 — Hour-two welcome wire (Slice 25)

Existing WelcomeBack slab now reads `welcome_line`. Quit after Settled, rerun: *Welcome back · Hour two held · the yard remembers*. First boot stays quiet. No new HUD. No harvest_feel rewrite. No server. No Embassy. Workspace stays 21.88.0. Design tick, not a Cargo bump.

## [23.2.31] — 2026-09-05 — Hour-two resume (Slice 24)

Quit after Settled. Rerun. Welcome slab: Welcome back · Hour two held · the yard remembers. Card skips the first-hour walk when the pack is complete. Existing WelcomeBack surface — not a new HUD. No harvest_feel rewrite. No server. No Embassy. Workspace stays 21.88.0. Design tick, not a Cargo bump.

## [23.2.30] — 2026-09-05 — Hour-two card sentences (Slice 23)

Same card. After allocate it says Tab the ridge, then Q plant a House stake, then L opens the Ledger, then E Bind then escort, then Hour two held. H still hides. No new HUD. No harvest_feel rewrite. No server. No Embassy. Workspace stays 21.88.0. Design tick, not a Cargo bump.

## [23.2.29] — 2026-09-05 — Hour-two persist + Settled (Slice 22 / Workstream C)

Hour two is held when House is live, the I2 spill was seen, and Ledger Bind/Escort settles. data/powrush_hour_two.json now keeps session + factory + witness + board (old SpaceSession-only files still load). Slab: Hour two held · the yard remembers. Peace E/I/H/R untouched. No harvest_feel rewrite. No server. No Embassy. Workspace stays 21.88.0. Design tick, not a Cargo bump.

## [23.2.28] — 2026-09-05 — Hour-two Charter door (Slice 21 / Workstream C)

After a first-hour allocate, Tab steps the Frontier ridge as a Peace visitor. Q founds a local House. Offline extractor spill and L Ledger Bind/Escort become reachable. Persist data/powrush_hour_two.json. Peace E/I/H/R untouched. No harvest_feel rewrite. No server. No Embassy. Workspace stays 21.88.0. Design tick, not a Cargo bump.

## [23.2.0] — 2026-09-02 — Standalone cargo (human game)

Lived first hour is the product. Workspace stays 21.88.0. See git history for 23.2.1–23.2.27.

Contact: **info@Rathor.ai**. Thunder locked in. Yoi ⚡