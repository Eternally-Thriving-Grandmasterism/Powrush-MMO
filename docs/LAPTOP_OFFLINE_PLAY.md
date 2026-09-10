# LAPTOP_OFFLINE_PLAY.md — human laptop review (Steam Offline 1.0)

**Contact:** info@Rathor.ai  
**Independent of xAI.** No certification / AGSi warranty / legal-product claims.

This pack is for a **human** on a real laptop. It is **not** a lavapipe agent walk, **not** Hour-two minutes, **not** a Title Online light, **not** a Cargo bump.

---

## Review tip (use this)

| Field | Value |
|---|---|
| **Review SHA** | `8311ac26` on `main` (feel-move) |
| Full tip | `8311ac263401705f13964e2ac1557a34d41b73e9` |
| Floor | `2163551` |
| Workspace | `21.88.0` (design tick only) |

**Do not** check out tag `playable-preview` / SHA `11c577e` for this review. That tag is the **older stranger floor**. This pack reviews **main at `8311ac26`**.

---

## Door

Feel / play content tip remains **`8311ac26`** (feel-move). Prefer current `main` for this pack’s scripts once Phase A0 is on main.

```bash
git clone https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO.git
cd Powrush-MMO
git checkout 8311ac26   # or main after A0 scripts land
# optional Core ballot (same as local court):
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
```

**Preferred Offline door** (forces `POWRUSH_NET=off`, never lights Title Online):

```bash
./scripts/play-offline.sh
# Windows PowerShell:
#   ./scripts/play-offline.ps1
```

Equivalent bare door: `cargo run -p powrush-client` with `POWRUSH_NET` **unset** or `off`. Do **not** set `POWRUSH_NET=on`.

---

## Core ballot (this tip, Bot 1)

Commands (not `--workspace`):

```bash
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
```

| crate | result on `8311ac26` |
|---|---|
| `rsil-identity` | **5** passed |
| `shared` | **313** passed |
| `powrush-client --lib` | **212** passed |

---

## Title / Online / LAN (pass bar)

- Title shows **Play / Continue / Settings** and a **grey Online** row.
- Online grey / *off (no listen)* is a **pass**. Do not try to light it.
- **LAN default off.** Loopback (if present) is Settings-only and is **not** the SKU promise.
- Fail if a socket opens on default boot, if Online enables, or if anything binds `0.0.0.0`.

---

## Keys (Peace)

| Hands | Key |
|---|---|
| Move | WASD |
| Look | mouse |
| Jump | Space |
| Sprint | Shift |
| Use | E |
| Satchel | I |
| Hide guidance | H |
| Allocate | R |
| Pause / Places door | Esc |
| Recipes / found path | Q |
| Ledger / week face | L |

Ra-Thor does **not** drive these keys.

---

## Honesty — what E does where

| Place / claim | E / Use |
|---|---|
| **Sanctuary** well (in range, Use not claimed) | May **Take** into the satchel (and hold-E tend). |
| **Threshold** pipe | Use is **Tend / dress**, not harvest Take. |
| **Heartwood Wards** | Use is **dress / tend**, not Take; does not write the house book. |
| **Depths** Peace node | Use is **restore**, not Take; writes Depths hex file; satchel/tons stay flat. |

If slab shows Wards / pipe / Depths claim, do not expect a harvest Take.

---

## A1 (audio) — note only

- First **successful** Peace E may play a **soft one-shot** once.
- Later Uses keep the well sting path.
- **Mute** silences it. Do not retune gains in this review.

## feel-move — note only

On tip `8311ac26`:

- Locomotion on **FixedUpdate @ 60 Hz**
- **Accel** toward wish; **stop-on-release** (no ice-skate)
- One **Use buffer ≤120 ms** when entering range

**Note it. Do not retune** accel, decel, Hz, or buffer window in this pack.

---

## Places (after Settled + book)

Disk-only rooms on main:

1. **Sanctuary** — boot / yard  
2. **Heartwood**  
3. **Threshold**  
4. **Depths**

**Market** stays HOLD. Esc opens pause with Places on every hex once eligible. No fifth place.

---

## Saves

- Prefer the **writable user-dir** JSON (House / settings / genshare / hex files).
- cwd `data/` is the **fallback / shape** names only — install-dir-only is a fail on a real install.
- Quit via **Title** (Esc → Title → Quit). Rerun: Continue should see the yard remember when persist exists.

Optional clean slot: `POWRUSH_USER_DIR=/tmp/powrush-laptop-review`.

---

## 8-step session (20–40 min)

Human fills blanks. **OS / GPU / Time stay blank in this docs file** — steward / reviewer invent nothing here.

| # | Beat | Do | Pass cue |
|---|---|---|---|
| 1 | Boot Title | `cargo run -p powrush-client` | Readable **Play / Continue / Settings** + **grey Online** |
| 2 | Settings honesty | Open Settings | LAN off (or loopback only); harm row not an armed first-hour admit; Online still grey |
| 3 | Sanctuary hands | Play → WASD / Space / Shift | Walk stops on release; no server process required |
| 4 | Well Use + A1 | Walk to glow → E (then I) | Visible feedback; satchel shows take; first successful E may soft-sting once (Mute kills it) |
| 5 | Hide + allocate | H, then R + flow/reserve as taught | Guidance can hide; allocate still visible; teaching claim optional |
| 6 | Pause / ledger | Esc; Q; L | Pause plate; Places only after Settled+book; L shows week / charter lines without Online |
| 7 | Other rooms (if book) | Places → Heartwood / Threshold / Depths | Pipe / Wards / Depths **claim Use** (dress or restore), not Sanctuary Take |
| 8 | Quit / Continue | Title → Quit → rerun → Continue | User-dir (or `data/` fallback) still has House/hex JSON; yard remembers |

Budget: about **20–40 minutes**. Stop early on any fail below.

### Fail fast

- E does nothing visible on an in-range Sanctuary well.
- Client requires a server / `POWRUSH_NET=on` / `0.0.0.0`.
- Online lights or shows fake peers.
- Second HUD before first tend.
- Depths / pipe / Wards Take into satchel.
- Feel-move ice-skates after release (note only — do not patch in this pack).

---

## Report template

Copy from `docs/FIRST_HOUR_PLAYTEST.md`. Leave fields blank until the human fills them. **Do not invent OS / GPU / Time.**

```
OS / GPU:
Commit: 8311ac26
Time to first tend:
E feedback: yes/no
Satchel correct: yes/no
Allocate visible: yes/no
Save/load: yes/no
Card hid with H: yes/no
Extract tired the well: yes/no
Flow restored the well: yes/no
Online grey: yes/no
Feel-move stop-on-release: yes/no
Use buffer noticed: yes/no / n/a
Places after book: Sanctuary / Heartwood / Threshold / Depths / n/a
Confusion point:
Suggested one-line fix:
```

---

## Refuse

Do **not** for this review:

- `POWRUSH_NET=on`
- Bind or recommend `0.0.0.0`
- Treat **F9** as a store / Title Online feature
- Open or fold a **Simulator** repo as the game
- Let **Ra-Thor drive keys** / WASD
- Retag `playable-preview` or bump floor `2163551`
- Invent **Hour-two minutes**
- Light **Title Online**
- Spawn a child agent to “play” for the steward

---

## Related law

`docs/FIRST_HOUR_PLAYTEST.md` · `docs/OFFLINE_SKU.md` · `docs/LOCAL_COURT.md` · `docs/AAA_FEEL_WORKPACK.md` · `docs/SLICE_LOG.md`

**Thunder locked in.** Yoi ⚡
