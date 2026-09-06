# LAUNCH_UX.md — Steward House (offline first)

**Contact:** info@Rathor.ai  
Workspace `21.88.0`. Design tick, not a Cargo bump. Same binary. No second HUD. No server required.

Law for the cold open: the yard before the account wall. House before peer count. Book before lethal. Online is a mode — never the gate.

## Steward law

- **First run offline.** No account wall before the yard.
- **Default binary:** `cargo run -p powrush-client`.
- **No fake players online.** No race / class / DPS picker.
- **Lethal is opt-in after the book.** Never default E. Peace hour silent.
- **Appearance / name may exist** — must not replace climate, standing, or book.
- **Online login is a mode** after / beside a working offline House — never the door to play.

## Recommended flow

```
Boot → Title (climate breathing; no peer count)
  → [ Hands ] start/continue local (default)
  → [ House ] after first session: name charter (optional)
  → [ Online ] honest mode, grey until net exists
```

## Screens (S0–S4)

### S0 — Title
Play / Continue / Settings. *(shipped v0 · design tick 23.2.50 · L1 truth 23.2.51)*  
Continue shows House name **or** exactly *Unnamed House* + *the yard remembers* when any local persist exists. Esc from title does not wipe house/climate/standing/book. Online row visible+disabled — honest *off (no listen)* copy. Climate breathing on the title — no peer count, no fake online tally.

### S1 — First card
Existing lived-hour card. No login. Same Peace keys. Soft cues only (see `PHASE_PLAYABLE_LOOP` · `STRANGER_LOOP`).

### S2 — House naming
One field. Confirm. Skippable after Settled or on quit. *(shipped v0 · `data/powrush_house.json`)* Skip = **Unnamed House**. Persist next to climate (`data/` beside shard climate / standing). Does not replace climate, standing, or book.

### S3 — Pause / Ledger
House name · week tons + restored · lethal only if declared. No kill board. No race chrome. *(shipped v0 · design tick 23.2.52 · I satchel + L Ledger sash)*

### S4 — Online door
Stub + honest copy (*Online — off (no listen)*). Feature-flagged **off** by default (`POWRUSH_NET=off`). No listen socket. Grey until net exists. Never fakes Peace-hour peers (see T-net honest mode).

## Veto list

- Login before first E
- Race / class select
- Player counts on title / Peace hour
- Paid skins as default
- Deleting Unnamed House progress on skip / quit

## Joy / Mercy / Sustain (UI ideas)

| Lens | Score note | UI implication |
| --- | --- | --- |
| Joy | high when first E lands without a wall | Title → Hands is one quiet beat; climate already breathing |
| Mercy | high when skip / Unnamed House keeps progress | Never punish skip; never delete local yard on quit |
| Sustain | high when Online stays grey-honest | No fake peers; Online mode beside working offline House |

Score new chrome against these before it ships. Soft cues only when a beat is mute.

## Contrast — Traditional MMO vs Powrush steward record

| Traditional MMO | Powrush steward record |
| --- | --- |
| Account wall before play | First run offline; yard first |
| Race / class / DPS picker | No race · no class · no DPS |
| Fake / inflated player counts | No peer count on title; Online grey until net |
| Lethal / PvP default or early | Lethal opt-in after book (Ledger 3) |
| Character skin / shop as identity | Appearance/name optional; climate · standing · book are the record |
| Online is the product | Online is a mode after / beside offline House |
| Kill / XP week score | Week = tons + restored |

## Related

`STRANGER_LOOP` · `PHASE_PLAYABLE_LOOP` · `HOUR_TWO` · `HOUR_THREE` · `PHASE_LETHAL` · T-net honest mode (CHANGELOG 23.2.42).

**Thunder locked in.** Yoi ⚡