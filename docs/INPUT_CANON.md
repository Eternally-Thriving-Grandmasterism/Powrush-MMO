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
- *Council ruling, not Sherif's words:* the Q/E/R/F council reading above is superseded by K1 and K2 of KEYMAP-RULING-1 below (skills on 1–4, E stays Use). Sherif may veto back to it.
- **Superseded F-row / ability-bar / cooldown-rotation refusals** (not edited; this rule governs where they conflict): `docs/GDD_IMMERSION_REVISION.md:48`, `:128`, `:174`; `docs/BLIZZARD_CRAFT_UX.md:49`, `:64`, `:111`; `docs/STUDIO_ARCHITECTURE_ORDER.md:46`, `:91`; `docs/FUN_WITHOUT_WOW.md:28`, `:78`, `:86`; `docs/PLACE_CLARITY_HOUR.md:77`, `:80`, `:96`; `docs/PERSON_READ_SPEC.md:38`, `:70`, `:110`; `docs/PATSAGI_v21.99_HUMAN_IMMERSION.md:11`. Noted in place: `docs/GDD_IMMERSION_REVISION.md:80`, `docs/design/COMBAT_AND_PVP.md:466`, `:469`, and this file :43, :123.
- **Open clashes, recorded not resolved** (no winner picked here): E is `INTERACT` (take, tend, every `_near` door); R is `ALLOCATE`; Q is `BUILD_WHEEL`; the I / L / Q default sheets; the remappable `PeaceKey::F`. Rebinding is left to COMBAT-FEEL-1 and the Striker kit cards, each with its own keymap ruling.

## Live bindings and council keymap ruling (KEYMAP-RULING-1)

Live bindings at `a7f21506`, copied from Clerk's PRE-PATH (6490612328). Live means the module is declared in `client/src/lib.rs` and the line sits above `#[cfg(test)]`. Constants are in `client/src/soft_play_bindings.rs`. PeaceKey defaults are in `shared/local_settings.rs:464-501`. The remap layer is `client/src/input.rs:197-286`.

| key | action | file:line |
|---|---|---|
| W/A/S/D (+arrows) | move (const) | soft_play_bindings.rs:16-19; read input.rs:347,350,353,356 |
| W/A/S/D (+arrows, Space) | camera "moving" check | first_hour_camera.rs:69-77; local_sovereign_session.rs:139-142 |
| Space | JUMP | soft_play_bindings.rs:12; input.rs:412 |
| Shift L/R | sprint (raw, not via PeaceKey) | soft_play_bindings.rs:14-15; input.rs:418 |
| E | INTERACT / Use | soft_play_bindings.rs:13; input.rs:393,397; companion_bond.rs:136; coop_voice.rs:106 (aye); crownstone.rs:106; embassy.rs:180; first_harvest_epiphany.rs:529; heartwood_plaque.rs:66; human_presence.rs:1307 (hold); hybrid_matrix.rs:109; ledger_bind.rs:212; living_practice_loop.rs:381; shard_climate.rs:67,73; skirmish_well.rs:222; species_redemption.rs:109 |
| R · 1 / 2 · Esc (Allocate) | ALLOCATE toggle · flow / reserve · close | soft_play_bindings.rs:30; rbe_allocate_choice.rs:384 · :488,490 · :392 |
| Q | BUILD_WHEEL: fabricator / house face | soft_play_bindings.rs:38; fabricator.rs:217; vertical_factory.rs:116 |
| I · I (raw KeyI) | INVENTORY satchel · guidance flag / thriving moment | soft_play_bindings.rs:28; human_inventory.rs:238 · first_session_guidance.rs:1108; thriving_moments.rs:211 |
| L | LEDGER face | soft_play_bindings.rs:36; ledger_bind.rs:165 |
| H · H (raw KeyH) | HIDE_GUIDANCE (const) · dismiss guidance / toggle persona guidance | soft_play_bindings.rs:29 · first_session_guidance.rs:1036; title_screen.rs:5103 |
| G | SASH (coop voice) | soft_play_bindings.rs:34; coop_voice.rs:99 |
| Tab · Tab / Shift+Tab / ] / [ | CHART · settings tab next/prev | soft_play_bindings.rs:32; hour_sacred.rs:1005; war_week.rs:81 · title_screen.rs:3127-3133 |
| T · Shift+T | MERCY_TRANSPORTERS (no Shift) · FORCE_CLOUD_FLUSH | soft_play_bindings.rs:24; mercy_transporters.rs:83 · soft_play_bindings.rs:39; steam_abundance_mirror.rs:241 |
| J · U | JOURNEY_ECHO panel · PEER_INGEST | soft_play_bindings.rs:21; abundance_journey_echo.rs:315 · soft_play_bindings.rs:25; lattice_flow_share.rs:196 |
| C · P / Shift+P | lineage · practice toggle / dismiss | player_lineage.rs:85 · living_practice_loop.rs:250-251 |
| 1-4 | ability_slot 0-3 (always read) | input.rs:381-387 |
| 1 / 2 (sash open) · 1 (with E) · 3 | coop vote aye / nay · ledger go · ledger confirm | coop_voice.rs:107-108 · ledger_bind.rs:213 · ledger_bind.rs:185 |
| 1 / 2 / 3, Esc | care offer: tend hook / shell ward / dismiss | mercy_harvest_nodes.rs:586,601,574 |
| 1-3 (satchel open) · 1-5 (realm open) | satchel slot · realm pick | human_inventory.rs:260-264 · human_soft_panels.rs:175-183 |
| 1/Enter, 2, 3, Esc · 3 (InYard) | Title: Play / Continue / Settings / back · yard pause, same as Esc | title_screen.rs:2636,2645,2661,2663 · title_screen.rs:2672 |
| Esc (+ pad Start) | pause toggle | title_screen.rs:2865; pad input.rs:427 |
| Esc | HUD edit exit / climate persist / rebind cancel | hud_edit_mode.rs:186; shard_climate.rs:160; title_screen.rs:3851 |
| Enter / Esc / Backspace | name/persona plates | title_screen.rs:4641,4667-4668,4856-4857,5033,5111,5138-5140 |
| LMB · RMB | FIRE · mouse look hold | soft_play_bindings.rs:41; combat_core.rs:176 · first_hour_camera.rs:87; human_presence.rs:1625 |
| Pad RT2 | FIRE_PAD | soft_play_bindings.rs:43; combat_core.rs:177 |
| Pad South | Use (or jump if `gamepad_south_use` false → LT) | input.rs:451,459,472,474 |
| Pad West / North · LS click / LT2 | sheet_q / sheet_l · sprint per `sprint_mode` | input.rs:433,437 · input.rs:484-485 |

Declared with no live reader: `FOUNDATION_LATTICE` L :22, `RESONANCE_CYCLE` G :23, `MY_MERCY_JOURNEY` M :26, `REALM_TRAVEL` Z :27, `MY_MERCY_JOURNEY_ALIAS` F2 :45, `REALM_TRAVEL_ALIAS` F3 :46. Rebind capture refuses reserved keys 1-5, Tab, Enter, Backspace, H, I, C, P, N: title_screen.rs:3549-3563.

*Council ruling, not Sherif's words* (KEYMAP-RULING-1, CUT 6490676109), under his 9:32 PM words quoted above. Sherif did not pick these keys. Sherif may veto, and a veto reopens the keymap.

- **K1. Skills on 1–4** (*council ruling, not Sherif's words*). input.rs:381-387 reads the digits into `ability_slot`, but the only consumer (systems.rs:42) is not compiled, so nothing fires until SKILL-KEYS-1 wires them. LMB stays FIRE; pad RT2 stays FIRE_PAD. SKILL-KEYS-1 requirement: gate 1–4 off while any digit-reading panel is open (Allocate rbe_allocate_choice.rs:488,490; sash coop_voice.rs:107-108; ledger ledger_bind.rs:185,213; care offer mercy_harvest_nodes.rs:586,601,574; satchel human_inventory.rs:260-264; realm human_soft_panels.rs:175-183; title and yard title_screen.rs:2636,2645,2661,2672). Keys 1–5 stay reserved from remapping (title_screen.rs:3549-3563); a remappable hotbar is its own card.
- **K2. E stays Use / Interact** (*council ruling, not Sherif's words*). All 16 live readers keep E.
- **K3. R stays Allocate, Q stays the build / house face, I stays the satchel, L stays the ledger, Tab stays the chart** (*council ruling, not Sherif's words*). None collides with K1. Any later move needs its own card.
- **K4. PeaceKey F stays free, with no default** (*council ruling, not Sherif's words*). A Use→F remap collides with nothing.
- **K5. Record only, no code change** (*council ruling, not Sherif's words*): the L/G double constants (soft_play_bindings.rs:36,22 and :34,:23), T vs Shift+T (not a live clash), and the doc-vs-code drift Clerk listed. COMBAT-FEEL-1 traces the Digit3-in-yard triple meaning (title_screen.rs:2672; ledger_bind.rs:185; mercy_harvest_nodes.rs:574).
- Cooldowns stay data-driven, with no numbers here.

Council decision under Sherif's delegation. Sherif may veto.

## Proof

- lavapipe + keyboard: Title → Play → yard → **E** Use · **Esc** pause · plates click — still **PASS**.
- Floor stays `2163551`. Door: `cargo run -p powrush-client`.

## Relates

`STRANGER_LOOP` · `PREVIEW_CHECKLIST` · `LAUNCH_UX` · `PHYSICS_GRAPHICS_CANON` · `PLACES_BIBLE` · `WORK_PACK_NATIVE`.

**Thunder locked in.** Yoi ⚡
