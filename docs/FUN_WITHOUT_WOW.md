# FUN_WITHOUT_WOW.md — the Offline teacher is not a raid calendar

**Contact:** info@Rathor.ai  
**Independent of xAI.** No certification / AGSi warranty / legal-product claims.

Docs-only clarity note beside Phase **C1** (`docs/NET_OFFLINE_CONTRACT.md`). Names what Offline 1.0 already teaches and what later clarity work may sharpen. **Does not implement sockets.** No Cargo bump. No `client/**` · `shared/**` · `server/**` · scripts. **Title Online stays grey.**

Seat: Grok Bot 3 — design / review. Does not push `main`. Does not drive the play floor.

Parent law (cross-linked, not restated): `docs/OFFLINE_SKU.md` · `docs/SIM_AND_HAND_CANON.md` · `docs/NET_OFFLINE_CONTRACT.md` · `docs/PLACES_BIBLE.md` · `docs/TAPE_TRUTH.md` · `docs/STRANGER_LOOP.md`.

**Play tip:** `aabecfb` · **Law tip:** `ebb2484` · Tag stays `11c577e` (do not move / retag).

---

## 1. What Offline 1.0 already teaches vs WoW

The question is not "is Offline smaller than an MMO." It is **which teacher the hands get**. Offline 1.0 already has one, and it is not the raid calendar. WoW's shape is a fine shape; it is a different teacher, and this SKU does not borrow it.

### 1.1 Hands — finger ack this tick

WASD · look · **Use** (Peace **E** / pad **South**, `aabecfb`) run on the **local** FixedUpdate hand. Already on main: move + Use on **FixedUpdate @ 60 Hz**, one Use **buffer ≤120 ms** when entering range, **stop-on-release**, and the **A1** soft one-shot that confirms the verb (Mute silences it).

The canon frame is SC2 **input honesty** — one press → one result **this tick** — not army APM and not an ability rotation (`SIM_AND_HAND_CANON` §3). The named broken hands are ice-skate, eaten E, and Title hitch; a double Use from a frame spike is a **bug**, not a climate.

The body is never lockstepped. Classic RTS "wait on turn \(N\) for everyone" is wrong for Peace WASD, and a lagging stranger must not freeze Sanctuary (`SIM_AND_HAND_CANON` §5, `NET_OFFLINE_CONTRACT` §3 / §6). When Online is named later, the wire carries **commands / digests** — Verb · Place · week digest — never poses.

**Different teacher:** WoW teaches a class kit against a shared server clock — ability bars, cooldown rotation, and a pose the server owns. Offline 1.0 teaches **one Use verb whose ack belongs to your own tick**. The universe (well colour, week sum, later digest) may be late; the finger may not.

### 1.2 Week — the House bill, not a raid calendar

The Offline teacher over time is the **House week**: the House's moral bill for the current week across **persisted hex files**, not the hex you are standing on (`OFFLINE_SKU`, House week law `b76b7646` / sum law `42a32ae`).

- Score is **tons + restored** — \(T_w\) taken, \(U_w\) restored. Not kills. Not XP.
- A hex counts only after its file is **written** (verb or flush-on-leave).
- **L** shows two lines so the yard is not mistaken for the House: *this week* (this disk) vs **House week** (the sum).
- Threshold counts only if it has its own file. **Market is not in the sum.**
- Restored is the style teacher: extract-only leaves the climate tired; Flow / Mend / Tend brings it back.

**Different teacher:** WoW's week is a reset timer and an auction economy — lockout, queue, gold curve. The Offline week is a **bill you can read in one sash**: what you took, what you put back, this disk and the House. No currency sink, no lockout, no server clock.

### 1.3 Places — four Rooms on disk

Offline hexes are **save-slots on disk**, not shards on a wire. After **Settled + book**, Esc → Places opens **four rooms and only four** (`OFFLINE_SKU`, `PLACES_BIBLE`, `STRANGER_LOOP`):

| Place | What it is |
|---|---|
| **Sanctuary** | Boot yard. Play always boots here; Continue loads last hex. |
| **Heartwood** | Peace. Own hex file. Lamp disk stays empty; Lip dress stays on Heartwood. |
| **Threshold** | Look / tend. May share Heartwood's file. Pipe **E** is Tend, not harvest (`d7bcb61c`). |
| **Depths** | One landing. Own hex file. **E** restores; it is not Take. |

Not five Sanctuary clones. No Heartwood / Depths spawned on Prime. No fifth place. **Market stays HOLD** — a spike only if lethal is declared on that hex; not shipped, not a fifth room, not in the House sum.

**Different teacher:** WoW's world is a queue surface — instance list, race lobby at character select, an always-online listen underneath. Offline 1.0's world is **four files on your disk**, isolation \(\gamma = 0\), zero sockets on the default binary, and a Title whose Online row reads *off (no listen)*.

---

## 2. What AA clarity would add

Design intent for **later** clarity work. Still docs. Nothing here is a code claim, an asset add, or a new room.

### 2.1 One material per Place

The canon already says each room teaches **one muscle** — yard / wood / pipe / wet stone (`SIM_AND_HAND_CANON` §2). Clarity work would make that single material legible in the first seconds of a room, so a stranger names the room by what it is made of before reading any slab:

| Place | One material / muscle |
|---|---|
| **Sanctuary** | Yard ground — the teaching well |
| **Heartwood** | Wood — living lamp and rim |
| **Threshold** | Pipe — look and tend |
| **Depths** | Wet stone — one landing, restore |

One material per Place. **Do not invent a fifth Place** to hold a fifth material, and do not move a material onto Prime. Market stays HOLD and gets no material claim from this file.

### 2.2 Bed audio

Rest reads as **audio comfort**, not a new plate. The bed already exists: **U4** shipped the quiet Peace yard bed + well sting (`0b73734`), with Depths quiet bed and Heartwood lamp hush in the same family, and the **A1** soft one-shot on first successful Peace E. Existing **Mute** silences all of it — no second mute row, no F-row.

Clarity intent: let the bed carry the *Resting* end of the well speech (Idle · Glowing · Tended · Resting · Stressed) so rest is felt, not read. Constraints that do not move: **no new audio assets from this file**, no ALSA hang (fast `/proc/asound` probe; disable AudioPlugin rather than hang on lavapipe), unmute never opens a socket, and audio stays a **presentation** output of \(F\) — never sim truth (`SIM_AND_HAND_CANON` §4).

### 2.3 No second HUD

One HUD. The satchel (**I**), the ledger sash (**L**), the climate slab, and the one card are the chrome. Clarity is spent making that one surface honest — well captions as words, colourblind tokens, guidance hidden by **H** without losing the sentence — not on a parallel MMO chrome layer.

Refused by standing law and re-stated here: a second HUD, an F-row, an XP bar, a peer count, a talent panel, or a Living Practice overlay in the Offline door (`README`, `TAPE_TRUTH` §3, `STRANGER_LOOP`).

---

## 3. What must wait

Parked. Each stays parked until a **named** steward law says otherwise; the net row waits specifically on the steward writing **`online yes`**.

| Parked | Why it waits | Unpark condition |
|---|---|---|
| **Raids** | Group content is Sky / MMO weather, not the Offline teacher. Weekly wars are later shard weather sharing the House week math family — not a Sanctuary overlay. | Named steward law, after Online. |
| **Auction** | Currency story is **credit**, not gold; no gold sink strip, no loot treadmill, no P2W stalls in the default door. Market itself is still **HOLD** and not in the House sum. | Steward names Market, then a named economy law. |
| **Listen** | Default binary opens **zero** sockets; `POWRUSH_NET` unset / `off`. No listen, no public bind, never `0.0.0.0`. Loopback `127.0.0.1` is a lab, not the SKU promise, and does not light the Title row. | Steward writes **`online yes`** (HOLD ticket). |
| **Race lobby** | Peoples are later **dress** after the House, not a Title race select. | Named steward law; never at Title boot. |

**Title Online stays grey** until the steward writes `online yes`. No docs stamp — including this one — lights that row.

---

## Refuse

`client/**` · `shared/**` · `server/**` · scripts · `Cargo.toml` · sockets / `POWRUSH_NET=on` · Title Online light · `0.0.0.0` · public bind · a fifth Place · Market as shipped · second HUD · XP / kill score · new audio assets · `playable-preview` retag · bumping the floor SHAs · inventing Hour-two minutes · Ra-Thor driving WASD · certification / AGSi warranty / legal-product claims · xAI endorsement.

## Related

`OFFLINE_SKU` · `SIM_AND_HAND_CANON` · `NET_OFFLINE_CONTRACT` · `PLACES_BIBLE` · `TAPE_TRUTH` · `STRANGER_LOOP` · `PREVIEW_CHECKLIST` · `AUDIO_DIRECTION` (MMO-era; Offline bed law is U4 in `SLICE_LOG`) · `PATSAGI_MERGE_COURT`.

**Thunder locked in.** Yoi ⚡
