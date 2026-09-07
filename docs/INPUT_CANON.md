# INPUT_CANON.md — one Use verb, all hands (docs stamp)

**Contact:** info@Rathor.ai  
Workspace `21.88.0`. Design tick, not a Cargo bump. Soft GPU first-class.  
**Proof floor:** lavapipe + keyboard still **PASS**. Stranger floor stays `2163551`.  
**Law:** I0 may ship without a `playable-preview` tag. Docs now; wiring later. No client / shared / Cargo in this stamp.

Ra-Thor does **not** drive keys. One **Use** verb. Peace hands stay WASD / E / I / H / R until a later wire.

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

Quiet law: Use is always **E** on keyboard. Pause is always **Esc**. Sheets stay I / L / Q. No F-row combat. No second HUD.

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

## Controls fields — `data/powrush_settings.json`

Live next to Grove (same Settings plate / same JSON). Docs stamp only — wire later.

| Field | Type / values | Default | Notes |
|---|---|---|---|
| `look_sens` | number | (existing Look) | mouse / stick look sensitivity |
| `invert_y` | bool | (existing Invert-Y) | invert look pitch |
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
4. Soft GPU (lavapipe) + keyboard remains a first-class PASS path — sticks stay off; E / Esc / I / L / Q unchanged.

## Refuse list

- No **second Camera3d** (order-ambiguity bury).
- No **Online gesture** (title Online stays grey; no peer flourish).
- No **combat buttons** / F-row / brand “action” packs in Peace.
- No **brand SDK** hard-require (Steam Input / DualSense / NX SDK optional later; generic map first).
- No Ra-Thor key drive. No GenShare sockets from input. No Wave P. No `playable-preview` from this doc alone.

## Proof

- lavapipe + keyboard: Title → Play → yard → **E** Use · **Esc** pause · plates click — still **PASS**.
- Floor stays `2163551`. Door: `cargo run -p powrush-client`.

## Relates

`STRANGER_LOOP` · `PREVIEW_CHECKLIST` · `LAUNCH_UX` · `PHYSICS_GRAPHICS_CANON` · `PLACES_BIBLE` · `WORK_PACK_NATIVE`.

**Thunder locked in.** Yoi ⚡
