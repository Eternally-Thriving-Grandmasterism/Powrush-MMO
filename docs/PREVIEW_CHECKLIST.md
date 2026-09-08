# PREVIEW_CHECKLIST.md — human tick boxes (v23.2.62)

**Contact:** info@Rathor.ai  
Workspace `21.88.0`. Design tick, not a Cargo bump.

One cold stranger. Same binary: `cargo run -p powrush-client`. No second HUD. No server required.

**Steward law:** ticking every box here makes a stranger pass *possible* and keeps the repo honest. It does **not** cut a `playable-preview` git tag. A human says the preview tag when it sings — Proceed ≠ playable-preview tag. Checklist ticks still ≠ tag even after Wave P.

**Wave P (2026-09-07):** steward preview **yes**. Annotated tag `playable-preview` = walked SHA **`11c577e`** (human-cut; tag object `e37ed6e`; **not** the glow commit). Play/stranger floor remains `2163551` (G0). Online grey; no public bind. Tag alone does not turn on GenShare sockets, birds, or default Grove. Do not retag `playable-preview`.

**E4 (2026-09-08):** comfort after the tag is extra, not a second launch. Current tip may be `3fc46c9` (Hour-two welcome-back glow #292). Welcome-back: *same slab breathes once*, then rests. First boot quiet. No XP. Play floor stays `2163551`. Online grey; no public bind. No sockets, no birds, no default Grove, no combat. **Minutes / OS / GPU / Time** (`docs/HOUR_TWO_PLAYTEST.md` report) are **not a bot field** — leave blank for the human.

**P3 LAN (Settings loopback lab):** Settings **LAN · off|loopback** persists beside Grove (default **off**; unknown → off). Loopback is **127.0.0.1** only — not a Peace Online tick, not Title Online. Title Online stays **grey** and does not bind. LAN off = today's boot (no listen, no outbound). Do not retag `playable-preview` (`11c577e`). Leave `HOUR_TWO_PLAYTEST` minutes blank.

**L1 hex sign:** After Settled + book, Settings / Q / Ledger may confirm **this hex admits harm** (default **off**; missing/unknown → off). Reuses standing `declare_lethal`. Confirm without book does nothing. No ton mint. Sanctuary E unchanged. Do not retag `playable-preview`.

**Stranger floor:** `2163551` (G0 floor stamp; #279 merge; Settled Esc/Q/L click-clean on lavapipe with `POWRUSH_GEN=light`; G0 default OFF; comfort/canons). Door stays `cargo run -p powrush-client`. Online grey; not a launch candidate.

## Tick (~15)

- [ ] **Boot** — `cargo run -p powrush-client` opens Title (Play / Continue / Settings); climate readable; no account wall
- [ ] **Online grey** — title Online row visible+disabled; honest *off (no listen)*; no peer count / fake presence
- [ ] **net-off = no socket** — default `POWRUSH_NET=off` opens zero sockets (no outbound, no listen)
- [ ] **E** — first tend/take: glow + feedback; well speech Idle / Glowing / Tended / Resting / Stressed
- [ ] **Peace keys** — WASD / E / I / H / R still the first-hour hands; no new default verbs
- [ ] **Continue / Unnamed House** — with local persist, Continue shows House name **or** exactly *Unnamed House* + *the yard remembers*
- [ ] **House name skippable** — Skip (or quit path) → Unnamed House; progress kept; no name wall on first Play
- [ ] **Esc → pause** — Esc in yard opens opaque pause *the yard is waiting* → Resume / Title / Quit (Quit ≠ Esc; Title keeps house JSON + lived persist)
- [ ] **Use / Pause / sticks** — Use is **E** / gamepad **South** / on-screen **Use**; pause is **Esc** / **Start** / overlay **Pause**; on-screen sticks **cull on plates** (Title/pause/Settings/L/Q/I); Settings **Sticks · auto** keeps mouse Title clean. See `INPUT_CANON` (I0 wired).
- [ ] **Settings persist** — Look · Mute · Invert-Y · Hide slabs · Brightness · Text scale · **Grove off|light** · **LAN off|loopback** on Title/pause Settings plate; persists in `data/powrush_settings.json` (Grove and LAN default **off**); survives quit/rerun; **env not required** — Settings alone enables light path; LAN loopback is a 127.0.0.1 lab only (Title Online stays grey); Title opaque contrast stays law
- [ ] **Seals / heritage (after Settled)** — Well / Grove / Ember skippable (cosmetic only); heritage string only (`none|human|cydruid|quellorian|draek|ambrosian`); Q shows Seal · … when dressed; **no combat stats** / +take / +STR
- [ ] **Book** — Hour three path reaches *the book is yours* (fabricator → Embassy seat) without a login wall
- [ ] **Climate** — slab + `data/` climate truth survive quit/rerun (harmony / stress / tired / circulating)
- [ ] **Week tons+restored** — week line is *this week · N tons · M restored* (not kills / XP)
- [ ] **L3 absent on first hour** — default `POWRUSH_INGEST=off`; no **ingest overlay** on Peace boot. Note: `data/powrush_lived_tick.json` may still exist as **session persist** (Mode B resume) — that is not Ra-Thor ingest. Checklist “no tick” = no ingest overlay; do not delete the blob
- [ ] **G0 opt-in** — default boot: no extra scatter (Grove/G0 **off**). Steward check: Settings Grove · light alone enables light path (**no env required**); `POWRUSH_GEN=light` still OR-equivalent. Same hex seed each run; Title/pause/Settings/Ledger plates still click; fog behind UI only. Never default gen on; no birds; no GenShare sockets
- [ ] **Title contrast** — Play/Continue/Online/Settings readable (opaque high-contrast plate)
- [ ] **Lavapipe click-clean** — soft GPU first-class walk proof per `docs/LAVAPIPE_CLICK_CLEAN.md` (readable · hits UI not world · labeled action fires · overlay culled · no 2nd Camera3d); not a unit-test name; not screenshot-only
- [ ] **House file** — after Settled or quit-to-title, `data/powrush_house.json` exists even if name skipped (Continue: *Unnamed House · the yard remembers*)
- [ ] **Lethal quiet** — Peace boot has no lethal UI; Settings / Q / Ledger confirm **this hex admits harm** only after Settled + book (default **off**); never default E
- [ ] **Quit / rerun remembers** — house / climate / standing / book / settings JSON intact after Esc quit and relaunch
- [ ] **Pause / Ledger face** — I or L shows House (or Unnamed House) + week tons+restored; Seal · … when dressed; no fake online tally
- [ ] **Stranger loop** — `docs/STRANGER_LOOP.md` minutes match what the binary does (Title → yard → pause → House → seals → book). Playtest **minutes / OS / GPU / Time** are not a bot field — do not fill `HOUR_TWO_PLAYTEST` for the human

## Explicit refuse (do not tick these into existence)

- `POWRUSH_NET=on` as default · bind `0.0.0.0` · lighting title Online · Steam store copy · fake presence · new Peace verbs · combat stats on seals/heritage · `POWRUSH_GEN` default on · GenShare sockets · fog/birds as preview juice · cutting `playable-preview` from this checklist alone
- **GenShare is not a Peace tick** — recipe law lives in `docs/GENSHARE.md`; do not invent a Peace-hour GenShare checkbox or claim sockets shipped. L0 file may exist offline (`data/powrush_genshare.jsonl`); not a Peace Online tick.

## Related

`STRANGER_LOOP` · `INPUT_CANON` · `PLACES_BIBLE` · `LAVAPIPE_CLICK_CLEAN` · `GENSHARE` · `LAUNCH_UX` · `FIRST_HOUR_PLAYTEST` · `PHASE_PLAYABLE_LOOP` · Dev recipe `F9_TWO_CLIENT_LOCALHOST` (not a title feature).

**Thunder locked in.** Yoi ⚡
