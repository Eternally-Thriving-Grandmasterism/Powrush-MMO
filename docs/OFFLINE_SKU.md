# OFFLINE_SKU.md — Steam Offline is the first product (U0)

**Contact:** info@Rathor.ai  
Workspace `21.88.0`. Design tick, not a Cargo bump.  
**Floor stays:** `2163551`.  
**`playable-preview` stays** walked SHA **`11c577e`** (tag object `e37ed6e`; not the glow commit). Do not retag until a human names a Steam retag.  
**Independent of xAI.** No certification / AGSi warranty / legal-product claims.

**Agent landing:** walked slices (U0–U12 on main, including U12 House week #319 `b76b7646`), Core gate, and do-not list live in README (dated section after Floor). This file is SKU law; do not reopen it or freelance a new product.

**Slice:** U13 — docs only. Content list of what is already on main, plus steward House week law. Not a sim. No `client/` / `shared/` / `server/` / Cargo. Do not run lavapipe. Do not invent Hour-two minutes / OS / GPU / Time. Do not retag `playable-preview`. Do not bump floor `2163551`. Bot does not create a Steam partner account.

---

## Law (already decided — do not reopen)

**SKU = Steam Offline.** Full MMO is a **second product**. Do not block the first on the second.

This file is the Offline 1.0 product law. Later slices implement; they do not reopen these sentences.

## Title / Online / LAN

- Title button stays **Online · off (no listen)** in this SKU. No listen. No public bind.
- **LAN default off.** Loopback may exist (Settings **LAN · off|loopback**, `127.0.0.1` only). Loopback is **not** the SKU promise.
- Never `0.0.0.0`. Never light Title Online from loopback, GenShare, or docs.

## Hexes = save-slots on disk

Offline hexes are **save-slots on disk**. They are **not** shards on a wire.

- Slots share **one House** and **one House-week footer** (sum tons + restored).
- **Per-hex climate.** Never couple lethal flags, seeds, or tons across hexes.
- Not a server. Cloud save later = those JSON files (see User-dir).

## Sanctuary Prime is the boot hex

- **Sanctuary Prime** is the boot hex.
- **Play** (first hands) **always** boots Sanctuary.
- **Continue** loads last hex (U2 on main). Play still boots Sanctuary.

## Places after Settled + book (disk only)

After Settled + book, **disk-only** places. Do **not** ship five Sanctuary clones. Do **not** spawn Heartwood / Depths on Sanctuary. Do **not** add a fifth place. Door-feel law (docs, not Hands): `SETTLED_DOOR_CLARITY` — Settled + book opens Places as a door, not a list-row teleport.

Steam Offline 1.0 rooms (on main — U13 stamps, does not rebuild):

| Place | Law |
|---|---|
| **Sanctuary** | yard / boot hex. Own hex file when written. |
| **Heartwood** | Peace. Own hex file. On main. |
| **Threshold** | look / tend. May share Heartwood's file. On main. Pipe Tend is not harvest (`d7bcb61c`). |
| **Depths** | one landing. Own hex file. On main. |

**Market** stays **HOLD**. Spike **only if** lethal is declared **on that hex**. Not shipped. Not a fifth room. Not in the House sum.

Sanctuary stays the yard. The other three rooms are later hexes, not overlays on Prime.

Esc opens pause with Places on every hex (U8.1). Comfort later, not this PR: Places fat-tap, Depths tend node.

## Climate isolation (Offline 1.0)

- Isolation in Offline 1.0: **gamma = 0**.
- **Sanctuary forever isolated** unless a later **named** law.
- **Leak tick is U3.5, not U0.** Do not implement leak here.
- Never couple lethal flags, seeds, or tons across hexes.

## User-dir saves (install law — U1 on main)

House / settings / genshare / hex files **must** live in a **writable user dir**, not Program Files only.

- Not a server.
- Cloud save later = those JSON files.
- **U1 shipped** the path. Each place keeps `powrush_hex_<id>.json` in that user dir.

Today's cwd `data/` shapes stay the names:

| file | role |
|---|---|
| `powrush_house.json` | one House (name / Unnamed) |
| `powrush_settings.json` | Grove / LAN / look / mute / … |
| `powrush_genshare.jsonl` | offline recipe (no listen) |
| hex climate / standing / week / hour files | per-hex slots + shared week footer |

U1 moved these into the writable user dir. Install-dir-only is a fail.

## Not in Offline 1.0

Explicitly **not** this SKU:

- public Online
- `0.0.0.0`
- shard wars
- GenShare listen
- postcard L1
- XP bar
- race select
- Brood Spire on first card
- always-on lethal
- Pages / deploy from the coding PAT

## Peace + lethal (unchanged)

- **Peace keys unchanged.**
- **Sanctuary E unchanged.**
- Lethal is a **sign after book**, default **off**, **no ton mint**.
- First hour / no book: the harm row must not arm.

## House week (steward law — #319 / `b76b7646`)

House week is the House's **moral bill** for the current week across **persisted hex files**, not the hex you are standing on.

- A file only counts after it has been written (verb or flush-on-leave).
- Threshold may share Heartwood's file. Market is **not** in the sum.
- Per hex i: T_w is tons taken this week, U_w is restored this week (Flow / Mend / Tend that writes restore).
- L's **this week** line is **this disk only**. Sanctuary 1/1 means T_w^S=1, U_w^S=1.
- House sum: T_w^House = sum T_w over hexes that have a file (Sanctuary, Heartwood, Depths; Threshold only if it has its own file). Same for U_w.

Accepted pass on #319 / `b76b7646` (yard 1/1, House 1 tons · 2 restored):

- Sanctuary 1 / 1
- Heartwood 0 / 1
- House 1 / 2

L shows both lines so the yard is not mistaken for the House:

- `this week · 1 tons · 1 restored` → current hex
- `House week · 1 tons · 2 restored` → the sum

Not in the sum: live RAM that never flushed, gamma leak (still 0), Market / lethal hexes, other Houses, shards, or peers.

**Q** stays recipes. **L** is the ledger.

Steam Offline 1.0 is those four rooms plus that one add. Title Online stays grey. Tag `11c577e`. Floor `2163551`. No listen. No γ leak. No new sim.

## Later named slices (not this PR)

| Slice | Job | Note |
|---|---|---|
| **U1** | writable user-dir persist | on main |
| **U2** | Continue loads last hex | on main; Play still boots Sanctuary |
| **U3.5** | leak tick | isolation stays γ = 0 until that named law |
| **Market** | HOLD | not shipped; not in the House sum |
| Comfort | Places fat-tap; Depths tend node | later, not U13 |

## Refuse

- Blocking Steam Offline on a public MMO / shard / listen path.
- Lighting Title Online. Binding `0.0.0.0`. Claiming loopback as the SKU.
- Five Sanctuary clones. Heartwood / Depths spawned on Sanctuary. A fifth place. Market as shipped. Title Online on.
- Coupling lethal / seeds / tons across hexes. Leak tick in U0.
- Filling `HOUR_TWO_PLAYTEST` minutes / OS / GPU / Time.
- Creating a Steam partner account from a bot.
- Cutting or retagging `playable-preview` from this stamp.
- Certification / AGSi warranty / legal-product claims. xAI endorsement.

## Related

`STEAM_CHECKLIST` · `PREVIEW_CHECKLIST` · `PLACES_BIBLE` · `GENSHARE` · `LAUNCH_UX` · `PROTOCOL` · `STRANGER_LOOP` · `SETTLED_DOOR_CLARITY` · `HOUR_TWO_PLAYTEST` (minutes stay blank).

**Thunder locked in.** Yoi ⚡
