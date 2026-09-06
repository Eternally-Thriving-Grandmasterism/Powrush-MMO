# PREVIEW_CHECKLIST.md — human tick boxes (v23.2.59)

**Contact:** info@Rathor.ai  
Workspace `21.88.0`. Design tick, not a Cargo bump.

One cold stranger. Same binary: `cargo run -p powrush-client`. No second HUD. No server required.

**Steward law:** ticking every box here makes a stranger pass *possible* and keeps the repo honest. It does **not** cut a `playable-preview` git tag. A human says the preview tag when it sings — Proceed ≠ playable-preview tag.

## Tick (~15)

- [ ] **Boot** — `cargo run -p powrush-client` opens Title (Play / Continue / Settings); climate readable; no account wall
- [ ] **Online grey** — title Online row visible+disabled; honest *off (no listen)*; no peer count / fake presence
- [ ] **net-off = no socket** — default `POWRUSH_NET=off` opens zero sockets (no outbound, no listen)
- [ ] **E** — first tend/take: glow + feedback; well speech Idle / Glowing / Tended / Resting / Stressed
- [ ] **Peace keys** — WASD / E / I / H / R still the first-hour hands; no new default verbs
- [ ] **Continue / Unnamed House** — with local persist, Continue shows House name **or** exactly *Unnamed House* + *the yard remembers*
- [ ] **House name skippable** — Skip (or quit path) → Unnamed House; progress kept; no name wall on first Play
- [ ] **Book** — Hour three path reaches *the book is yours* (fabricator → Embassy seat) without a login wall
- [ ] **Climate** — slab + `data/` climate truth survive quit/rerun (harmony / stress / tired / circulating)
- [ ] **Week tons+restored** — week line is *this week · N tons · M restored* (not kills / XP)
- [ ] **L3 absent on first hour** — default `POWRUSH_INGEST=off`; no lived-tick write forced on Peace boot
- [ ] **Lethal quiet** — Peace boot has no lethal UI; Ledger **3** only after book; never default E
- [ ] **Quit / rerun remembers** — house / climate / standing / book JSON intact after Esc quit and relaunch
- [ ] **Pause / Ledger face** — I or L shows House (or Unnamed House) + week tons+restored; no fake online tally
- [ ] **Stranger loop** — `docs/STRANGER_LOOP.md` minutes match what the binary does (Title → yard → House → book)

## Explicit refuse (do not tick these into existence)

- `POWRUSH_NET=on` as default · bind `0.0.0.0` · lighting title Online · Steam store copy · fake presence · new Peace verbs · cutting `playable-preview` from this checklist alone

## Related

`STRANGER_LOOP` · `LAUNCH_UX` · `FIRST_HOUR_PLAYTEST` · `PHASE_PLAYABLE_LOOP` · Dev recipe `F9_TWO_CLIENT_LOCALHOST` (not a title feature).

**Thunder locked in.** Yoi ⚡