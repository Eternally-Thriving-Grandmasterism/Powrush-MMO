# CHANGELOG.md — Powrush-MMO

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