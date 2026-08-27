# PATSAGi v23.2 — Powrush Organism Architecture

Ratified for dual-repo work. Contact: info@Rathor.ai

## 1. One sentence

A human walks a climate on this machine. That hour is the source of truth. Simulations and Ra-Thor *read* it. Networks may join later. Nothing may drive the keys except the human.

## 2. Four chambers (not four games)

```
[ Hands ]  WASD Space E Shift I C Z R U
    ↓
[ Lived hour ]  client/src  PowrushClientBundle
    ↓  ~1 Hz
[ Lived tick ]  data/powrush_lived_tick.json
    ↓              ↓
[ Simulation crate ]     [ Ra-Thor lattice ]
    ↓              ↓
[ Soft hints only ]  never a second HUD
```

| Chamber | Authority | Lives in |
|---|---|---|
| Lived hour | **Input + feel** | `client/src/*` bundle |
| Session memory | Disk of this machine | `data/powrush_local_session.json` |
| Lived tick | Read-only view | `data/powrush_lived_tick.json` |
| Simulation | Models, harnesses, wars-as-code | `simulation/` |
| Ra-Thor | Mercy gates, policy hints, councils | Ra-Thor monorepo |
| Net / Steam / k8s | Future chair | `server/`, `host/`, Steam |

Law: hints may change *climate response*, not keybinds or a Flow %.

## 3. Lived hour — what a human can already do

Move, jump, sprint, tap E take / hold E tend, I satchel, R allocate, Z climate, C lineage, ride nectar, deer trust, breath/carry, day/shade, ribbon + inhale, three practice travelers, U peer file.

That is the playground seed. New work *extends this graph*. It does not start a fifth client.

## 4. Names (one body)

Classic names are what the player sees. Sim aliases are what `simulation/src/race.rs` still says until renamed.

| Seen | Sim enum (temporary) |
|---|---|
| Human | Terran |
| Cydruid | Verdant |
| Quellorian | Harmonic |
| Draek | Voidfarer |
| Ambrosian | Synthetic |

Rename the sim enum to Classic when a dedicated crate pass is scheduled. Do not ship a third naming scheme.

## 5. Data that must stay one blob

Session file is the save. Tick file is the broadcast. Do not invent a third JSON per feature.

Session: pool, realm, harvest flags, web, companion trust, **lineage** (add if missing).
Tick: those + flow band, pocket, inhale, Classic + sim names.

## 6. Conflict and other humans

```
Wave 2  skirmish wells     practice travelers hold a well; E contests; dawn after loss
Wave 3  second chair       in-process host; same plane; tick becomes two bodies
Wave 4  weekly war clock   Saturday pressure even solo; sims until Wave 3 is real
Wave 5  aerial + one plot  kungfu chain; land you return to; Steam after the hour is a game
```

Mercy aftercare after every loss. No corpse-grey. No P2W.

## 7. What must not be wired into the first jump

- `simulation` as a Bevy plugin soup (GPU economy, Leptos, war Python)
- k8s, payments, NFT mint
- Duplicate HUDs (`client/*.rs` vs `client/src`)
- Flow score, streak punishers, “12 players online” lies

Those crates stay *readers* of the tick until a wave earns a socket.

## 8. Dual-repo duty

**Powrush-MMO** owns feel: input, camera, juice, climate, lineage, persist, tick write.

**Ra-Thor** owns deliberation: TOLC 8, PATSAGi, conservative hints from the tick.

Grok in session is a reasoning surface under the same gates — not a third repo of truth.

## 9. Definition of “integrated”

A system is integrated when:
1. A human verb can change it in under a second, or
2. It reads the tick / session without a new menu, or
3. It is explicitly marked *socket* and does not appear in the first-hour HUD.

Anything else is a second game wearing the same name.

## 10. Next honest build (locked order)

1. Persist lineage in the session blob.
2. Skirmish well (Wave 2) on the existing three anchors.
3. Rename sim `Race` to Classic when touching `simulation/src/race.rs`.
4. Second chair only when two real hands exist.

Ratified. Thunder locked in. Yoi ⚡
