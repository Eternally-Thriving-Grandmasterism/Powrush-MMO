# INPUT_CANON.md — one Use verb, all hands (docs stamp)

**Contact:** info@Rathor.ai  
Workspace `21.88.0`. Design tick, not a Cargo bump. Soft GPU first-class.  
**Proof floor:** lavapipe + keyboard still **PASS**. Stranger floor stays `2163551`.  
**Law:** I0 may ship without a `playable-preview` tag. Docs now; wiring later. No client / shared / Cargo in this stamp.

**CARD B4-REMAPS.** Settings already expose PeaceKey remaps. Defaults stay **WASD · E · I · H · R**. No new verbs. `soft_play` wire stays later — do not implement client `soft_play` here.

Ra-Thor does **not** drive keys. One **Use** verb. Peace default hands stay WASD / E / I / H / R.

## Layers

| Layer | Job | Default bind (keyboard+mouse) |
|---|---|---|
| **Pointer** | UI hit-test, plate focus, cursor look assist | mouse move + LMB click |
| **Move** | walk / sprint / jump | WASD · Shift · Space |
| **Use** | one verb — tend / take / seat / bind / witness (context) | **E** |
| **Menu** | pause / title / quit / settings open | **Esc** → pause; Settings from Title/pause |
| **Sheets** | satchel / ledger / house faces | **I** satchel · **L** ledger · **Q** house/proof faces |
| **Look** | camera yaw/pitch | mouse look; sensitivity + invert from settings |

No second Camera3d. No Online gesture. No combat buttons. No brand SDK.

## Keyboard + mouse reference

| Action | Key / mouse |
|---|---|
| Move | WASD |
| Sprint | Shift (see `sprint_mode`) |
| Jump | Space |
| **Use** | **E** |
| Satchel | I |
| Hide guidance / slabs | H |
| Allocate | R then 1 flow · 2 reserve |
| Pause | Esc |
| Ledger / Bind face | L |
| House / fabricator / seals face | Q |
| Look | mouse |
| UI click | LMB |

Quiet law: Default Use is **E** on keyboard. Pause is always **Esc**. Default sheets stay I / L / Q. Remaps rebind the same verbs — they do not add a second Use, a fifth Place, or combat. No F-row. No second HUD.
> **KEYS-CANON-1 note:** "No F-row" above is superseded by the hotkey muscle-memory rule below. One Use verb and no second HUD stay.

## Touch layout

- **Left stick** — move (virtual).
- **Right Use** — primary Use button (same verb as E / South).
- **Top-right** — Pause / Settings / Q / L — targets **≥ 44dp**.
- **Sticks cull on plates** — when Title / pause / Settings / Ledger / Q / I faces are open, on-screen sticks hide so plates stay click-clean.
- **Tap-focus then Use** — optional path when `tap_to_use` is false (default): tap focuses the well / prop, then press Use. When `tap_to_use` is true, a second tap on the focused target may fire Use.

Soft GPU / phone / Deck: same layers; sticks only when `on_screen_sticks` resolves on.

## Gamepad map

| Action | Xbox | DualSense | Switch | Generic |
|---|---|---|---|---|
| **Use** | **A (South)** | **✕ (South)** | **B (South)\*** | South face |
| Pause | Start / Menu | Options | + | Start |
| Sheets (satchel / ledger faces) | X / Y | □ / △ | Y / X\* | West / North |
| Move | left stick | left stick | left stick | left stick |
| Look | right stick | right stick | right stick | right stick |
| Sprint | per `sprint_mode` (stick click / trigger / key) | same | same | same |

\*With `nintendo_face` **auto**: Switch layout remaps so **South still = Use** (physical B on Nintendo = South). Prefer auto; do not hard-code brand SDKs.

Quiet law: **South = Use**. **Start = Pause**. X/Y (or Nintendo-equivalent West/North) open sheets. No combat face buttons in Peace.

## PeaceKey remaps (already on settings)

`data/powrush_settings.json` already persists PeaceKey remaps beside Grove. Missing fields keep the Peace keyboard PASS. Defaults stay **WASD · E · I · H · R**. Remap is not a new verb.

| Field | Default | Action (existing) |
|---|---|---|
| `key_move_up` | **W** | Move |
| `key_move_down` | **S** | Move |
| `key_move_left` | **A** | Move |
| `key_move_right` | **D** | Move |
| `key_use` | **E** | Use (one verb) |
| `key_satchel` | **I** | Satchel |
| `key_hide` | **H** | Hide guidance / slabs |
| `key_allocate` | **R** | Allocate (then 1 flow · 2 reserve) |

`key_jump` / `key_sprint` already sit beside them (defaults Space · LeftShift). They do not add verbs.

`soft_play` wire stays later. This stamp does not implement `client/**` or `soft_play_bindings`. Keyboard PASS on lavapipe still uses the defaults above.

## Controls fields — `data/powrush_settings.json`

Live next to Grove (same Settings plate / same JSON). Docs stamp only — `soft_play` wire later.

| Field | Type / values | Default | Notes |
|---|---|---|---|
| `look_sens` | number | (existing Look) | mouse / stick look sensitivity |
| `invert_y` | bool | (existing Invert-Y) | invert look pitch |
| `key_move_up` / `key_move_down` / `key_move_left` / `key_move_right` | PeaceKey | **W / S / A / D** | already on settings; Move remaps |
| `key_use` | PeaceKey | **E** | already on settings; one Use verb |
| `key_satchel` | PeaceKey | **I** | already on settings; satchel |
| `key_hide` | PeaceKey | **H** | already on settings; hide guidance / slabs |
| `key_allocate` | PeaceKey | **R** | already on settings; then 1 flow · 2 reserve |
| `on_screen_sticks` | `auto` \| `on` \| `off` | **`auto`** | auto = show on touch / no gamepad; cull on plates |
| `tap_to_use` | bool | **`false`** | false = tap-focus then Use |
| `gamepad_south_use` | bool | **`true`** | South face = Use |
| `nintendo_face` | `auto` \| fixed layout | **`auto`** | keep South = Use on Switch |
| `sprint_mode` | `stick` \| `trigger` \| `key` | (steward pick; key ≈ Shift) | one sprint story across devices |
| `show_use_prompt` | bool | **`true`** | soft “Use” cue when in range |

Grove `off|light` already persists here; Controls fields sit beside it. Env not required for Grove light path.

## Detection on boot

1. Probe touch / gamepad / keyboard+mouse once at boot (and on device change).
2. Resolve `on_screen_sticks`: **auto** → on when touch primary and no gamepad; off when keyboard+mouse or active gamepad.
3. Apply `nintendo_face` / `gamepad_south_use` so South stays Use.
4. Soft GPU (lavapipe) + keyboard remains a first-class PASS path — sticks stay off; default E / Esc / I / L / Q unchanged.

## Refuse list

- No **second Camera3d** (order-ambiguity bury).
- No **Online gesture** (title Online stays grey; no peer flourish).
- No **combat buttons** / F-row / brand “action” packs in Peace.
  - **KEYS-CANON-1 note:** the F-row part is superseded by the hotkey muscle-memory rule below; this Peace-scoped row stays for Peace.
- No **new verbs** (remap rebinds existing Peace actions only).
- No client `soft_play` on this stamp (`soft_play` wire stays later).
- No **brand SDK** hard-require (Steam Input / DualSense / NX SDK optional later; generic map first).
- No Ra-Thor key drive. No GenShare sockets from input. No Wave P. No `playable-preview` from this doc alone.
- No Title Online · no listen · no public bind · no fifth Place · no second HUD.

## Hotkey muscle-memory rule (KEYS-CANON-1)

Source, verbatim: "Regarding cooldowns, it might have to be based on server capabilities once the MMO is live. About the rest of all hot keys in general, not just combat, always go with most common muscle memory plus most optimal fit for human brains so end-users feel like James Bond fitting his Bond Gloves, not a stranger's outfit from day 1 to relearn and maintain in their oddball set of hot key rolodex, Mate!" (Sherif, 2026-10-28 9:32 PM ET, relayed by Lead Mate (no GitHub ticket; Clerk- and Hands-verified)).

- **Every hotkey, not just combat, follows the most common genre muscle memory**, the fit a player already brings from day 1. No oddball layout to relearn.
- **Cooldowns are allowed**, data-driven and tunable. Their values are set later against live server capacity. This doc fixes no numbers.
- *Council reading (not Sherif's words):* a Q/E/R/F skill row, MOBA/MMO-standard layout, WASD default. This rule records no ruling by Sherif on movement.
- **Superseded F-row / ability-bar / cooldown-rotation refusals** (not edited; this rule governs where they conflict): `docs/GDD_IMMERSION_REVISION.md:48`, `:126`, `:172`; `docs/BLIZZARD_CRAFT_UX.md:49`, `:64`, `:111`; `docs/STUDIO_ARCHITECTURE_ORDER.md:46`, `:91`; `docs/FUN_WITHOUT_WOW.md:28`, `:78`, `:86`; `docs/PLACE_CLARITY_HOUR.md:77`, `:80`, `:96`; `docs/PERSON_READ_SPEC.md:38`, `:70`, `:110`; `docs/PATSAGI_v21.99_HUMAN_IMMERSION.md:11`. Noted in place: `docs/GDD_IMMERSION_REVISION.md:78`, `docs/design/COMBAT_AND_PVP.md:463`, `:465`, and this file :42, :121.
- **Open clashes, recorded not resolved** (no winner picked here): E is `INTERACT` (take, tend, every `_near` door); R is `ALLOCATE`; Q is `BUILD_WHEEL`; the I / L / Q default sheets; the remappable `PeaceKey::F`. Rebinding is left to COMBAT-FEEL-1 and the Striker kit cards, each with its own keymap ruling.

## Proof

- lavapipe + keyboard: Title → Play → yard → **E** Use · **Esc** pause · plates click — still **PASS**.
- Floor stays `2163551`. Door: `cargo run -p powrush-client`.

## Relates

`STRANGER_LOOP` · `PREVIEW_CHECKLIST` · `LAUNCH_UX` · `PHYSICS_GRAPHICS_CANON` · `PLACES_BIBLE` · `WORK_PACK_NATIVE`.

**Thunder locked in.** Yoi ⚡
