# UI layout system (design)

CARD UI-LAYOUT-DESIGN-1 — step 1 of the UI layout epic. Design only. Base: `main` at `027c9802`.

This doc designs a HUD layout system that ends HUD overlaps: a central anchor registry (step 2), a Reset UI button plus three tested presets (step 3), and an Unlock / Edit mode (step 4). It changes no code.

Labels used below:

- **FACT** — read from [`docs/visual-pass/HUD_SLAB_MAP.md`](../visual-pass/HUD_SLAB_MAP.md) ("the map") or from the code at `027c9802`, with the line cited.
- **RECOMPUTED** — produced with the map's own overlap method (section 1.4) on map and code numbers. The check script is scratch and not in the tree.
- **PROPOSAL** — a design choice for steps 2–4. Every offset, gap, rank and file list marked this way is a proposal, not a fact.
- **Qn** — an open question (section 9). Anything the code does not prove is a Qn, not a fact.

## 1. Sources

### 1.1 The map

FACT. The map lists 46 absolute UI nodes at `main` `a51f7ee5`: 45 spawned by the game plus row 46, the green light-bloom-1 test box (`[Q2 probe]`). Row numbers below are the map's section 2 row numbers. Heights come from map section 3 (Model A: forced lines; Model B: Model A plus one wrapped line). Overlaps come from map section 4 (rectangles in window pixels, origin top-left, positive shared area; "visible together = Yes" when no code excludes the pair).

The map was merged in #676 (`47dbc43c`). Two PRs landed after its `a51f7ee5` base, and both are on `027c9802`.

### 1.2 Landed after the map, checked against the code

| PR | change | code at `027c9802` | map row |
|----|--------|--------------------|---------|
| #677 (`6092b9bd`) | voice slab `bottom: Val::Px(88.0)` → `144.0` | `client/src/coop_voice.rs` L45 `bottom: Val::Px(144.0)`. Locked by `voice_above_ledger1_slab_bottom_clears_ledger` (L184–L204), which asserts `Val::Px(144.0)` and `px > 135.0` (ledger bottom 16 + Model B 119) | row 9 (the map still says bottom 88) |
| #678 (`027c9802`) | world-care prompt yields while the care strip is active | `client/src/first_harvest_epiphany.rs` L727 adds `care: Option<Res<CareCycleOffer>>`. The `show` value at L732–L738 ends `&& !care.is_some_and(\|c\| c.active)`. Locked by `care_prompt_yield_1_hides_when_care_cycle_active` (L1198–L1235) | rows 27 / 30, map 4.1 A pair 102, 4.2 A pair 76, map 4.4 "care strip 128 / world-care prompt 128" |

FACT, line drift: the #678 import change adds two lines, so rows 27, 28 and 29 now sit at `first_harvest_epiphany.rs:308`, `:344` and `:380` (the map says `:306`, `:342` and `:378`). Every other `PositionType::Absolute` line in the map still matches at `027c9802`. Since `a51f7ee5`, only `coop_voice.rs` and `first_harvest_epiphany.rs` changed under `client/src`.

Other facts this design relies on:

- `client/src/ui_above_world.rs` L36–L40: `LIVED_UI_Z_TITLE = 120`, `LIVED_UI_Z_LEDGER = 125`, `LIVED_UI_Z_PAUSE = 130`. `client/src/hex_travel.rs` L70: `PLACES_PLATE_Z = LIVED_UI_Z_PAUSE + 2`. `client/src/touch_controls.rs` L19: `TOUCH_HIT_MIN = 44.0`. `client/src/title_screen.rs` L206: `TITLE_SAFE_INSET = 24.0`.
- Separate open flags: ledger `LedgerYard.sash_open` (L key, `ledger_bind.rs` L159), voice `VoiceYard.sash_open` (G key `soft_play_bindings::SASH`, `coop_voice.rs` L102), war `sash_open` (Tab key `CHART`, `war_week.rs` L85). Allocate `panel_open` (R, `rbe_allocate_choice.rs` L399–L408; auto-opens at L388–L389). `mercy_open` / `realm_open` (M / Z, `human_soft_panels.rs` L169–L172). Journey `panel_open` (J, `abundance_journey_echo.rs` L310). No code makes any two of these exclusive.
- Root markers that are not `pub`: `VoiceSlabRoot` (`coop_voice.rs` L24), `CareCycleStrip` (`mercy_harvest_nodes.rs` L284), `RedemptionSlabRoot` (`species_redemption.rs` L25), `HybridSlabRoot` (`hybrid_matrix.rs` L25). A registry cannot reach these roots without editing their files.
- Only one test pins a slab offset: `coop_voice.rs` L196 (`assert_eq!(bottom, Val::Px(144.0))`).

### 1.3 Baseline at `027c9802`

RECOMPUTED with voice at bottom 144 and the #678 exclusion added:

| resolution | Model A pairs | visible together | excluded by code | Model B extra | Model B total | Model B visible together |
|------------|---------------|------------------|------------------|---------------|---------------|--------------------------|
| 1024x640 | 133 | 128 | 5 | 36 | 169 | 164 |
| 1280x800 | 90 | 86 | 4 | 33 | 123 | 119 |

Against the map (130 / 171 and 89 / 126), voice at 144 drops its pairs with the ledger, compass, guidance and practice (and war under Model B). Under Model A it gains pairs with the care strip, world-care prompt, allocate, well and climate state at both sizes, plus satchel and Places at 1024x640. Under Model B the care strip, prompt, well and (at 1024x640) Places pairs already existed at bottom 88. Care strip against world-care prompt is now "No" (#678).

### 1.4 Method check

RECOMPUTED. A re-implementation of map sections 3 and 4 reproduces the map's tables at `a51f7ee5` exactly: 130 Model A + 41 Model B pairs at 1024x640, and 89 + 37 at 1280x800, with the same pair names. It matches when rectangle edges are rounded to whole pixels. Without rounding, one extra Model B pair appears at 1024x640: guidance (Model B top 499.0) and satchel (bottom 499.2) share 0.2 px, and the map does not list them. Every count in this doc uses that same rounding.

### 1.5 Why geometry alone cannot end the overlaps

RECOMPUTED. At Model B, the 27 HUD slabs that a layout can move (section 2.4, role HUD) cover 927,840 px² at 1024x640. The whole 1024x640 window is 655,360 px², and the window inside a 16 px margin is 603,136 px². The ledger and satchel add 137,140 px² more. So no placement of coded widths can keep every co-showable pair apart. The registry needs explicit rules for which slabs share a spot and which one shows. #678 is the precedent: two slabs on one anchor, and code that hides one.

### 1.6 LAW bounds that bind this design

Five peoples live in four Places: Human Sanctuary yard; Ambrosian Sanctuary well-from-above (the same Place, not a fifth); Cydruid Heartwood; Quellorian Threshold; Draek Depths. The Places plate (row 7) keeps its four rooms and is not in any preset or in edit mode.

- No Title race select. No fifth Place. Bevy stays pinned at 0.14 (`client/Cargo.toml` L17). Online stays grey.
- No padding or UI scale changes: the registry moves offsets and, in edit mode, width only. `text_scale` is never written.
- E and Q never call `try_cross_people_door` (`hour_sacred.rs` L294). Edit mode binds neither E nor Q (section 6).
- Parked items get no anchors and no preset rows: `treaty_*`, `war_week` (row 14), `crownstone` (row 15), `faction*_ui`. Of these, only rows 14 and 15 spawn. `coop_voice` (row 9) is **not** parked: live LAW wins over the old DO-NOT-FREESTYLE list in `docs/GROK_BOT_TODO.md` L190, which the map's row 9 tag followed.

## 2. Anchor registry model

### 2.1 The anchor record (PROPOSAL)

One central table owns every HUD position. Slab modules keep their own show logic and stop hard-coding offsets.

| field | meaning |
|-------|---------|
| `id` | stable name, for example `ACTION_BAR` |
| `corner` | `top-left`, `top-centre`, `top-right`, `bottom-left`, `bottom-centre`, `bottom-right` (edge midpoints use `centre` on the free axis) |
| `offset` | px from the named edges: `x` from the left or right edge (unused when centred, which keeps the map's `left: 50%` + negative half-width margin), `y` from the top or bottom edge |
| `width` | anchor width = the widest occupant's coded width (no occupant is resized) |
| `height budget` | the tallest occupant's Model B height, rounded up to a whole px |
| `z band` | `HUD` (no `ZIndex`, the map's `default`), or a fixed band from 2.2 |
| `class` | the anchor's highest-priority (lowest-numbered) occupant class (2.3) |
| `occupants` | slab ids in rank order |
| `share` | `solo`, `code-exclusive`, `yield`, or `push` (2.3) |

Each occupant takes the anchor's corner and offset with its own coded width and its content-driven height. So its rectangle sits inside the anchor's rectangle.

### 2.2 Z bands

FACT (map 4.5 and the constants in 1.2), with roles as PROPOSAL:

| band | nodes (map rows) | in the registry | in presets | in edit mode |
|------|------------------|-----------------|------------|--------------|
| HUD (no `ZIndex`) | 27 slabs (2.4) | yes | yes | yes (with class limits, section 6) |
| Global 120 Title | TitleRoot (1) | no | no | no |
| Global 124 touch | TouchOverlayRoot (20), stick / buttons (21–25) | fixed keep-out rects | same in every preset | no |
| Global 125 Ledger | Ledger (11), Satchel (33) | fixed rects | same in every preset | no |
| Global 130 Pause | Settings plate (3) | no | no | no |
| Global 131 | Comfort banner (2) | fixed entry (height unmeasured, section 8) | no | no |
| Global 132 | Places plate (7) | no | no | no |
| Global 140–142 doors | NameHouse (4), HouseDress (5), Persona (6) | no | no | no |

### 2.3 Classes and sharing rules (PROPOSAL)

Slab class, highest first:

| class | meaning | slabs (map row) |
|-------|---------|-----------------|
| 1 panel | opened by the player's own key | Voice (9, G), Allocate (37, R; also auto-opens), Mercy (39, M), Realm (40, Z), Journey (42, J) |
| 2 action prompt | tells the player what E does now | CareStrip (30), CarePrompt (27) |
| 3 tutor | first-session strips | Guidance (26, H hides), Practice (36, P toggles) |
| 4 toast | timed | Pulse (28), Welcome (29), Pickup (34), Sovereign (35), Thriving (38), Whisper (41) |
| 5 status | shows while a state holds | Factory (8), Spill (10), Fab (12), Embassy (13), Redemption (16), Hybrid (17), Compass (18), Well (19), ClimateState (31), Watch (32), Peer (43), PlaceName (44) |

Rules:

- **R1, code-exclusive.** Pairs the code already excludes may share an anchor: Guidance / Practice (map pair 101), CarePrompt / CareStrip (#678), touch stick / Ledger and touch stick / Satchel (map pairs 65, 96), Settings / Places (map pair 5).
- **R2, yield (generalises #678).** In a shared anchor, at most one occupant shows: the highest class first, then the highest rank. A yielded slab is hidden only. Its state, timers and keys keep running, exactly like the prompt under #678.
- **R3, push.** Class-1 panels never yield. When several share one anchor, opening one closes the others through each panel's own close path, the same flag its key toggles (`VoiceYard.sash_open`, `allocate.panel_open`, `mercy_open`, `realm_open`, `echo.panel_open`). This follows the WoW default UI, where opening a side panel can close the one already in its slot. Q2.
- **R4, cover.** A class-4 or class-5 slab yields (R2 semantics) while a visible class-1 slab's rectangle overlaps its own. Classes 2 and 3 are never covered. Cover is the only way two anchors may overlap: class-2 and class-3 anchors overlap nothing (except by R1), and two class-1 anchors never overlap. Q3.
- **R5, modal yield.** The HUD band yields while the Pause plate (row 3) or the Places plate (row 7) is open, or the launch door is not `InYard` (rows 1, 4, 5), or Persona (row 6) is open. The Ledger band and the touch band are not touched. Q5, Q6.

Fixed rects, the same in every preset (FACT: map coded anchors, Model B heights):

| rect | map row | anchor | 1024x640 | 1280x800 |
|------|---------|--------|----------|----------|
| Ledger | 11 | bottom 16, left 16, 560 × 119 | 16-576 × 505-624 | 16-576 × 665-784 |
| Satchel | 33 | bottom 22%, left 16, 300 × 235 | 16-316 × 264-499 | 16-316 × 389-624 |
| TouchStick | 21 | bottom 24, left 24, 120 × 120 | 24-144 × 496-616 | 24-144 × 656-776 |
| TouchUse | 22 | bottom 36, right 28, 44 × 44 | 952-996 × 560-604 | 1208-1252 × 720-764 |
| TouchPause / TouchQ / TouchL | 23–25 | top 24 / 76 / 128, right 24, 44 × 44 | 956-1000 × 24-172 | 1212-1256 × 24-172 |

Rects are written `x0-x1 × y0-y1` in window px. PROPOSAL, keep-out offsets: HUD anchors use `right 76` above y 180 (touch column right 24 + `TOUCH_HIT_MIN` 44 + an 8 px gap) and `right 80` near TouchUse (right 28 + 44 + 8). The gap between anchors is 8 px and the edge margin is 16 px, the most common edge offset in the map. Q20.

Heights used for budgets (Model B):

| slab | width | A / B height | source |
|------|-------|--------------|--------|
| 14 px slabs: Factory, Spill, Fab, Embassy, Redemption, Hybrid, Compass, Well, ClimateState, Voice | 520 / 420 / 560 | 35 / 52 | map 3 ("14 px slabs are 35 (A) / 52 (B)") |
| Guidance / Practice / CareStrip | 520 / 640 / 560 | 48 / 69, 46 / 64, 43 / 62 | map 3 |
| Allocate | 520 | 117 / 135 | map 3 |
| Mercy / Journey / Realm | 360 / 360 / 300 | 320 / 280 / 170 | map 3 (max-heights; Realm assumed) |
| PlaceName | 280 | 31 / 31 | map 3 (one line), map rect y 18-49 |
| Peer | 280 | 37 / 52 | map 4.4 |
| Comfort | 520 | 34 / 49 | map 4.1 A pair 1 (y 10-44); 4.1 B-only pair 1 puts its bottom at y 59 |
| Thriving / Sovereign / Whisper | 620 / 520 / 420 | 40 / 58, 39 / 56, 48 / 69 | map 4.1 / 4.2 rects |
| CarePrompt | 460 | 37.2 / 56.4 | code: border 1, padding 8, font 16 (`first_harvest_epiphany.rs` L313, L316, L331), map 3 formula; A matches map y 475-512 |
| Pulse | 560 | 41.6 / 60.8 | code: border 1.2, padding 10, font 16 (L349, L352, L367); A matches map y 118-160 |
| Welcome | 380 | 42.2 / 58.4 | code: border 1, padding 12, font 13.5 (L384, L385, L401); A matches map y 16-58 |
| Watch | 340 | 37.6 / 53.2 | code: border 1, padding 10, `WATCH_STRIP_FONT_BASE` 13 (`human_inventory.rs` L70); A matches map y 586-624 |
| Pickup | 360 | 37.2 / 56.4 | code: border 1, padding 8, `PICKUP_FLASH_FONT_BASE` 16 (`human_inventory.rs` L72); A matches map y 243-280 |

### 2.4 Existing slabs mapped onto the registry

Coded anchors are FACT (map section 2 at `027c9802`). Role and step 2 action are PROPOSAL.

| row | root (short name) | file:line @ `027c9802` | coded anchor, width | z | registry role | step 2 |
|-----|-------------------|------------------------|---------------------|---|---------------|--------|
| 1 | TitleRoot | `title_screen.rs:1331` | full viewport | 120 | Title door: out | none |
| 2 | ComfortGraphicsBannerRoot (Comfort) | `title_screen.rs:1447` | top 10, centred, 520 | 131 | fixed modal entry | none |
| 3 | SettingsStubRoot (Settings) | `title_screen.rs:1609` | top 1%, centred, 420 | 130 | Pause: out | none |
| 4 | NameHouseRoot | `title_screen.rs:1813` | full viewport | 140 | door: out | none |
| 5 | HouseDressRoot | `title_screen.rs:1884` | full viewport | 141 | door: out | none |
| 6 | PersonaCreatorRoot | `title_screen.rs:2045` | full viewport | 142 | door: out | none |
| 7 | PlacesRoot (Places) | `hex_travel.rs:449` | top 18%, centred, 400 | 132 | Places plate: out | none |
| 8 | FactorySlabRoot (Factory) | `vertical_factory.rs:77` | top 16, centred, 520 | HUD | class 5 | joins as-is |
| 9 | VoiceSlabRoot (Voice) | `coop_voice.rs:44` | bottom 144 (#677), centred, 560 | HUD | class 1 | **moves** |
| 10 | SpillSlabRoot (Spill) | `infra_spill.rs:69` | top 52, left 16, 520 | HUD | class 5 | joins as-is |
| 11 | LedgerSlabRoot (Ledger) | `ledger_bind.rs:98` | bottom 16, left 16, 560 | 125 | fixed rect | none |
| 12 | FabSlabRoot (Fab) | `fabricator.rs:75` | top 88, centred, 520 | HUD | class 5 | joins as-is |
| 13 | EmbassySlabRoot (Embassy) | `embassy.rs:66` | top 124, right 16, 420 | HUD | class 5 | joins as-is |
| 14 | WarSlabRoot | `war_week.rs:43` | bottom 52, right 16, 420 | HUD | **parked: no anchor** | none |
| 15 | CrownstoneSlabRoot | `crownstone.rs:44` | top 164, right 16, 420 | HUD | **parked: no anchor** | none |
| 16 | RedemptionSlabRoot (Redemption) | `species_redemption.rs:45` | top 204, right 16, 420 | HUD | class 5; HELD (`docs/GROK_BOT_TODO.md` L178), private marker | stays coded (Q9) |
| 17 | HybridSlabRoot (Hybrid) | `hybrid_matrix.rs:45` | top 244, right 16, 420 | HUD | class 5; HELD (L178), private marker | stays coded (Q9) |
| 18 | CompassSlabRoot (Compass) | `compass.rs:40` | bottom 92, right 16, 420 | HUD | class 5 | joins as-is |
| 19 | WellSlabRoot (Well) | `skirmish_well.rs:108` | bottom 132, left 16, 420 | HUD | class 5 | joins as-is |
| 20 | TouchOverlayRoot | `touch_controls.rs:69` | full viewport | 124 | touch band: out | none |
| 21–25 | TouchStick, TouchUse, TouchPause, TouchQ, TouchL | `touch_controls.rs:89`, `:182` (rows 22–25) | fixed rects above | 124 | keep-out rects | none |
| 26 | FirstSessionGuidanceStrip (Guidance) | `first_session_guidance.rs:792` | bottom 72, centred, 520 | HUD | class 3 | **moves** |
| 27 | WorldCarePromptRoot (CarePrompt) | `first_harvest_epiphany.rs:308` | bottom 128, centred, 460 | HUD | class 2 | **moves** |
| 28 | HarvestPulseRoot (Pulse) | `first_harvest_epiphany.rs:344` | top 118, centred, 560 | HUD | class 4 | joins as-is |
| 29 | WelcomeBackRoot (Welcome) | `first_harvest_epiphany.rs:380` | top 16, left 16, 380 | HUD | class 4 | joins as-is |
| 30 | CareCycleStrip (CareStrip) | `mercy_harvest_nodes.rs:390` | bottom 128, centred, 560 | HUD | class 2 | **moves** |
| 31 | ClimateStateRoot (ClimateState) | `climate_visible.rs:291` | bottom 176, left 16, 420 | HUD | class 5 | joins as-is |
| 32 | WatchStripRoot (Watch) | `human_inventory.rs:120` | bottom 16, left 16, 340 | HUD | class 5 | joins as-is |
| 33 | SatchelRoot (Satchel) | `human_inventory.rs:154` | bottom 22%, left 16, 300 | 125 | fixed rect | none |
| 34 | PickupFlashRoot (Pickup) | `human_inventory.rs:215` | top 38%, centred, 360 | HUD | class 4 | joins as-is |
| 35 | SovereignBannerRoot (Sovereign) | `local_sovereign_session.rs:81` | top 52, centred, 520 | HUD | class 4 | joins as-is |
| 36 | LivingPracticeStrip (Practice) | `living_practice_loop.rs:202` | bottom 72, centred, 640 | HUD | class 3 | **moves** |
| 37 | AllocatePanelRoot (Allocate) | `rbe_allocate_choice.rs:226` | bottom 140, centred, 520 | HUD | class 1 | **moves** |
| 38 | ThrivingToastRoot (Thriving) | `thriving_moments.rs:151` | top 48, centred, 620 | HUD | class 4 | joins as-is |
| 39 | MercySoftRoot (Mercy) | `human_soft_panels.rs:63` | top 10%, right 2%, 360 | HUD | class 1 | joins as-is |
| 40 | RealmSoftRoot (Realm) | `human_soft_panels.rs:116` | top 18%, left 2%, 300 | HUD | class 1 | joins as-is |
| 41 | WhisperRoot (Whisper) | `first_whisper.rs:56` | top 28%, centred, 420 | HUD | class 4 | joins as-is |
| 42 | JourneyEchoRoot (Journey) | `abundance_journey_echo.rs:220` | top 12%, left 2%, 360 | HUD | class 1 | joins as-is |
| 43 | PeerPresenceRoot (Peer) | `lattice_flow_share.rs:93` | bottom 16, right 16, 280 | HUD | class 5 | joins as-is |
| 44 | ClimateNameRoot (PlaceName) | `climate_plane.rs:1145` | top 18, centred, 280 | HUD | class 5 | joins as-is |
| 45 | FoundationLatticeRoot | `foundation_lattice.rs:60` | never shown | HUD | no anchor | none |
| 46 | green test box `[Q2 probe]` | `climate_plane.rs:4129` | test only | — | no anchor | none |

HUD role count: 27 (rows 8–10, 12–13, 16–19, 26–32 and 34–44). Two of them (16, 17) cannot join while HELD.

### 2.5 Banked overlaps that fold into step 2

RECOMPUTED at `027c9802` (Model A / Model B rects):

| banked overlap | 1024x640 | 1280x800 | map reference |
|----------------|----------|----------|---------------|
| Voice@144 × CareStrip | A 232-792 × 469-496; B 232-792 × 450-496 | A 360-920 × 629-656; B 360-920 × 610-656 | new since #677 |
| Voice@144 × CarePrompt | A 282-742 × 475-496; B 282-742 × 456-496 | A 410-870 × 635-656; B 410-870 × 616-656 | new since #677 |
| Guidance × Ledger | A 252-576 × 522-568 | A 380-576 × 682-728 | 4.1 A pair 66, 4.2 A pair 45 |
| Practice × Ledger | A 192-576 × 522-568 | A 320-576 × 682-728 | 4.1 A pair 68, 4.2 A pair 47 |
| Allocate@140 × CareStrip / CarePrompt / Voice | A 252-772 × 469-500 / 282-742 × 475-500 / 252-772 × 461-496 | A 380-900 × 629-660 / 410-870 × 635-660 / 380-900 × 621-656 | 4.1 A pairs 112, 104; map 4.4 "allocate 140 / care strip and prompt 128" |
| Allocate@140, its other pairs | Well, ClimateState, Satchel, Mercy (A); plus Compass, Guidance (B) | Well, ClimateState (A); plus Compass, Guidance (B) | 4.1 A pairs 94, 114, 115, 123; B pairs 30, 36 |

PROPOSAL, step 2 moves exactly six slabs into a right-side dock above the ledger (these anchors match the classic preset, except Allocate):

| row | slab | from (coded) | to (PROPOSAL) | 1024x640 B before → after | 1280x800 B after |
|-----|------|--------------|---------------|---------------------------|------------------|
| 26 | Guidance | bottom 72, centred | `ACTION_BAR` bottom 144, right 16 (rank 3) | 252-772 × 499-568 → 488-1008 × 427-496 | 744-1264 × 587-656 |
| 36 | Practice | bottom 72, centred | `ACTION_BAR` (rank 4) | 192-832 × 504-568 → 368-1008 × 432-496 | 624-1264 × 592-656 |
| 30 | CareStrip | bottom 128, centred | `ACTION_BAR` (rank 1) | 232-792 × 450-512 → 448-1008 × 434-496 | 704-1264 × 594-656 |
| 27 | CarePrompt | bottom 128, centred | `ACTION_BAR` (rank 2) | 282-742 × 456-512 → 548-1008 × 440-496 | 804-1264 × 600-656 |
| 9 | Voice | bottom 144, centred | `VOICE` bottom 221, right 16 | 232-792 × 444-496 → 448-1008 × 367-419 | 704-1264 × 527-579 |
| 37 | Allocate | bottom 140, centred | `ALLOCATE_DOCK` bottom 281, right 16 | 252-772 × 365-500 → 488-1008 × 224-359 | 744-1264 × 384-519 |

The dock is right-anchored because of a FACT: at 1024 wide, a centred slab wider than 392 px shares x with the satchel column (x 16-316), and the ledger fills x 16-576 below y 505. `ACTION_BAR` needs one new R2 yield: Guidance and Practice hide while CareStrip or CarePrompt shows (Q4).

RECOMPUTED step 2 result (visible-together pairs, against 1.3):

| resolution | model | before → after | removed | added |
|------------|-------|----------------|---------|-------|
| 1024x640 | A | 128 → 113 | 21 | Allocate–Hybrid, Allocate–Pickup, ClimateState–Practice, Guidance–Places, Places–Practice, Practice–Well |
| 1024x640 | B | 164 → 136 | 34 | Allocate–Hybrid, Allocate–Pickup, Allocate–Redemption, Allocate–Whisper, ClimateState–Practice, Mercy–Voice |
| 1280x800 | A | 86 → 69 | 17 | none |
| 1280x800 | B | 119 → 90 | 31 | Allocate–Mercy, Places–Voice |

Every named banked pair in the table above is gone at both resolutions under both models. A search over allocate anchors (left / right / centre, top / bottom, offsets 8–496 in 4 px steps) found no place that adds nothing, given the other five moves. The added pairs fold into step 3.

## 3. Presets

### 3.1 What "zero" means here

Method: map section 4 exactly (1.4), at 1024x640 and 1280x800, Model A and Model B, Model B budgets per 2.3.

"Always-co-showing" is read as the map's "visible together = Yes": the pair overlaps and nothing excludes it. Exclusions allowed: R1 code, R2 / R3 same anchor, R4 cover, R5 modal yield. Every preset slab is counted against every other preset slab and against the fixed rects in 2.3.

Preconditions for the zero results below:

1. Steps 2 and 3 implement R2–R5 (PROPOSAL; R1 is already code).
2. Rows 16 and 17 join the registry, which needs a lift of their HOLD (Q9). If they stay at their coded anchors, see 3.5.
3. Model B is a fair height estimate (Q10).

Not counted and listed in 3.5: the parked rows 14 and 15 at their coded rects, and the fixed modal-band pairs that no preset moves.

### 3.2 Classic MMO (WoW-style, loosely)

Idea: toasts across the top centre (zone and warning text). Status on the top left (the player-frame corner). The place name top right where a minimap's zone text sits, with a quest-tracker column under it. The action bar above the ledger (the ledger stays bottom left, where a chat box sits). One window slot for the key-opened panels. Small chips in the bottom-right corner.

#### classic anchors (PROPOSAL)

| anchor | corner | offset | width | height budget | class | occupants, rank order (map row) | 1024x640 | 1280x800 |
|---|---|---|---|---|---|---|---|---|
| `TOP_TOAST` | top-centre | centred, y 16 | 620 | 69 | 4 | Pulse (28), Pickup (34), Thriving (38), Sovereign (35), Whisper (41), Welcome (29) | 202-822 × 16-85 | 330-950 × 16-85 |
| `LEFT_STATUS_1` | top-left | x 16, y 93 | 520 | 52 | 5 | Factory (8), Fab (12), Spill (10) | 16-536 × 93-145 | 16-536 × 93-145 |
| `LEFT_STATUS_2` | top-left | x 16, y 153 | 420 | 52 | 5 | ClimateState (31), Well (19) | 16-436 × 153-205 | 16-436 × 153-205 |
| `PLACE_NAME` | top-right | x 76, y 93 | 280 | 31 | 5 | PlaceName (44) | 668-948 × 93-124 | 924-1204 × 93-124 |
| `TRACKER_1` | top-right | x 16, y 180 | 420 | 52 | 5 | Embassy (13) | 588-1008 × 180-232 | 844-1264 × 180-232 |
| `TRACKER_2` | top-right | x 16, y 240 | 420 | 52 | 5 | Redemption (16) | 588-1008 × 240-292 | 844-1264 × 240-292 |
| `TRACKER_3` | top-right | x 16, y 300 | 420 | 52 | 5 | Hybrid (17) | 588-1008 × 300-352 | 844-1264 × 300-352 |
| `TRACKER_4` | top-right | x 16, y 360 | 420 | 52 | 5 | Compass (18) | 588-1008 × 360-412 | 844-1264 × 360-412 |
| `VOICE` | bottom-right | x 16, y 221 | 560 | 52 | 1 | Voice (9) | 448-1008 × 367-419 | 704-1264 × 527-579 |
| `ACTION_BAR` | bottom-right | x 16, y 144 | 640 | 69 | 2 | CareStrip (30), CarePrompt (27), Guidance (26), Practice (36) | 368-1008 × 427-496 | 624-1264 × 587-656 |
| `WINDOW` | top-right | x 76, y 16 | 520 | 320 | 1 | Allocate (37), Mercy (39), Journey (42), Realm (40) | 428-948 × 16-336 | 684-1204 × 16-336 |
| `CORNER_WATCH` | bottom-right | x 80, y 76 | 340 | 54 | 5 | Watch (32) | 604-944 × 510-564 | 860-1200 × 670-724 |
| `CORNER_PEER` | bottom-right | x 80, y 16 | 280 | 52 | 5 | Peer (43) | 664-944 × 572-624 | 920-1200 × 732-784 |

#### classic slab-by-anchor placement

| row | slab | anchor | rank | class | 1024x640 (Model B) | 1280x800 (Model B) |
|---|---|---|---|---|---|---|
| 8 | Factory | `LEFT_STATUS_1` | 1 | 5 | 16-536 × 93-145 | 16-536 × 93-145 |
| 9 | Voice | `VOICE` | 1 | 1 | 448-1008 × 367-419 | 704-1264 × 527-579 |
| 10 | Spill | `LEFT_STATUS_1` | 3 | 5 | 16-536 × 93-145 | 16-536 × 93-145 |
| 12 | Fab | `LEFT_STATUS_1` | 2 | 5 | 16-536 × 93-145 | 16-536 × 93-145 |
| 13 | Embassy | `TRACKER_1` | 1 | 5 | 588-1008 × 180-232 | 844-1264 × 180-232 |
| 16 | Redemption | `TRACKER_2` | 1 | 5 | 588-1008 × 240-292 | 844-1264 × 240-292 |
| 17 | Hybrid | `TRACKER_3` | 1 | 5 | 588-1008 × 300-352 | 844-1264 × 300-352 |
| 18 | Compass | `TRACKER_4` | 1 | 5 | 588-1008 × 360-412 | 844-1264 × 360-412 |
| 19 | Well | `LEFT_STATUS_2` | 2 | 5 | 16-436 × 153-205 | 16-436 × 153-205 |
| 26 | Guidance | `ACTION_BAR` | 3 | 3 | 488-1008 × 427-496 | 744-1264 × 587-656 |
| 27 | CarePrompt | `ACTION_BAR` | 2 | 2 | 548-1008 × 440-496 | 804-1264 × 600-656 |
| 28 | Pulse | `TOP_TOAST` | 1 | 4 | 232-792 × 16-77 | 360-920 × 16-77 |
| 29 | Welcome | `TOP_TOAST` | 6 | 4 | 322-702 × 16-74 | 450-830 × 16-74 |
| 30 | CareStrip | `ACTION_BAR` | 1 | 2 | 448-1008 × 434-496 | 704-1264 × 594-656 |
| 31 | ClimateState | `LEFT_STATUS_2` | 1 | 5 | 16-436 × 153-205 | 16-436 × 153-205 |
| 32 | Watch | `CORNER_WATCH` | 1 | 5 | 604-944 × 511-564 | 860-1200 × 671-724 |
| 34 | Pickup | `TOP_TOAST` | 2 | 4 | 332-692 × 16-72 | 460-820 × 16-72 |
| 35 | Sovereign | `TOP_TOAST` | 4 | 4 | 252-772 × 16-72 | 380-900 × 16-72 |
| 36 | Practice | `ACTION_BAR` | 4 | 3 | 368-1008 × 432-496 | 624-1264 × 592-656 |
| 37 | Allocate | `WINDOW` | 1 | 1 | 428-948 × 16-151 | 684-1204 × 16-151 |
| 38 | Thriving | `TOP_TOAST` | 3 | 4 | 202-822 × 16-74 | 330-950 × 16-74 |
| 39 | Mercy | `WINDOW` | 2 | 1 | 588-948 × 16-336 | 844-1204 × 16-336 |
| 40 | Realm | `WINDOW` | 4 | 1 | 648-948 × 16-186 | 904-1204 × 16-186 |
| 41 | Whisper | `TOP_TOAST` | 5 | 4 | 302-722 × 16-85 | 430-850 × 16-85 |
| 42 | Journey | `WINDOW` | 3 | 1 | 588-948 × 16-296 | 844-1204 × 16-296 |
| 43 | Peer | `CORNER_PEER` | 1 | 5 | 664-944 × 572-624 | 920-1200 × 732-784 |
| 44 | PlaceName | `PLACE_NAME` | 1 | 5 | 668-948 × 93-124 | 924-1204 × 93-124 |

Cover (R4) anchors: at 1024x640, `WINDOW` covers `TOP_TOAST`, `LEFT_STATUS_1`, `LEFT_STATUS_2`, `PLACE_NAME` and `TRACKER_1`–`TRACKER_3`, and `VOICE` covers `TRACKER_4`. At 1280x800, `WINDOW` covers `TOP_TOAST`, `PLACE_NAME` and `TRACKER_1`–`TRACKER_3`.

### 3.3 Minimal / immersive

Idea: the fewest anchors, the centre of the screen left to the world. One toast slot (the place name shows when no toast does). One status slot on the right edge. The same action bar as classic. One window slot for every key-opened panel. One corner chip.

#### minimal anchors (PROPOSAL)

| anchor | corner | offset | width | height budget | class | occupants, rank order (map row) | 1024x640 | 1280x800 |
|---|---|---|---|---|---|---|---|---|
| `TOP_TOAST` | top-centre | centred, y 16 | 620 | 69 | 4 | Pulse (28), Pickup (34), Thriving (38), Sovereign (35), Whisper (41), Welcome (29), PlaceName (44) | 202-822 × 16-85 | 330-950 × 16-85 |
| `EDGE_STATUS` | top-right | x 16, y 180 | 520 | 52 | 5 | Factory (8), Fab (12), Spill (10), Embassy (13), Redemption (16), Hybrid (17), Compass (18), Well (19), ClimateState (31) | 488-1008 × 180-232 | 744-1264 × 180-232 |
| `ACTION_BAR` | bottom-right | x 16, y 144 | 640 | 69 | 2 | CareStrip (30), CarePrompt (27), Guidance (26), Practice (36) | 368-1008 × 427-496 | 624-1264 × 587-656 |
| `WINDOW` | top-right | x 76, y 16 | 560 | 320 | 1 | Voice (9), Allocate (37), Mercy (39), Journey (42), Realm (40) | 388-948 × 16-336 | 644-1204 × 16-336 |
| `CORNER` | bottom-right | x 80, y 16 | 340 | 54 | 5 | Watch (32), Peer (43) | 604-944 × 570-624 | 860-1200 × 730-784 |

#### minimal slab-by-anchor placement

| row | slab | anchor | rank | class | 1024x640 (Model B) | 1280x800 (Model B) |
|---|---|---|---|---|---|---|
| 8 | Factory | `EDGE_STATUS` | 1 | 5 | 488-1008 × 180-232 | 744-1264 × 180-232 |
| 9 | Voice | `WINDOW` | 1 | 1 | 388-948 × 16-68 | 644-1204 × 16-68 |
| 10 | Spill | `EDGE_STATUS` | 3 | 5 | 488-1008 × 180-232 | 744-1264 × 180-232 |
| 12 | Fab | `EDGE_STATUS` | 2 | 5 | 488-1008 × 180-232 | 744-1264 × 180-232 |
| 13 | Embassy | `EDGE_STATUS` | 4 | 5 | 588-1008 × 180-232 | 844-1264 × 180-232 |
| 16 | Redemption | `EDGE_STATUS` | 5 | 5 | 588-1008 × 180-232 | 844-1264 × 180-232 |
| 17 | Hybrid | `EDGE_STATUS` | 6 | 5 | 588-1008 × 180-232 | 844-1264 × 180-232 |
| 18 | Compass | `EDGE_STATUS` | 7 | 5 | 588-1008 × 180-232 | 844-1264 × 180-232 |
| 19 | Well | `EDGE_STATUS` | 8 | 5 | 588-1008 × 180-232 | 844-1264 × 180-232 |
| 26 | Guidance | `ACTION_BAR` | 3 | 3 | 488-1008 × 427-496 | 744-1264 × 587-656 |
| 27 | CarePrompt | `ACTION_BAR` | 2 | 2 | 548-1008 × 440-496 | 804-1264 × 600-656 |
| 28 | Pulse | `TOP_TOAST` | 1 | 4 | 232-792 × 16-77 | 360-920 × 16-77 |
| 29 | Welcome | `TOP_TOAST` | 6 | 4 | 322-702 × 16-74 | 450-830 × 16-74 |
| 30 | CareStrip | `ACTION_BAR` | 1 | 2 | 448-1008 × 434-496 | 704-1264 × 594-656 |
| 31 | ClimateState | `EDGE_STATUS` | 9 | 5 | 588-1008 × 180-232 | 844-1264 × 180-232 |
| 32 | Watch | `CORNER` | 1 | 5 | 604-944 × 571-624 | 860-1200 × 731-784 |
| 34 | Pickup | `TOP_TOAST` | 2 | 4 | 332-692 × 16-72 | 460-820 × 16-72 |
| 35 | Sovereign | `TOP_TOAST` | 4 | 4 | 252-772 × 16-72 | 380-900 × 16-72 |
| 36 | Practice | `ACTION_BAR` | 4 | 3 | 368-1008 × 432-496 | 624-1264 × 592-656 |
| 37 | Allocate | `WINDOW` | 2 | 1 | 428-948 × 16-151 | 684-1204 × 16-151 |
| 38 | Thriving | `TOP_TOAST` | 3 | 4 | 202-822 × 16-74 | 330-950 × 16-74 |
| 39 | Mercy | `WINDOW` | 3 | 1 | 588-948 × 16-336 | 844-1204 × 16-336 |
| 40 | Realm | `WINDOW` | 5 | 1 | 648-948 × 16-186 | 904-1204 × 16-186 |
| 41 | Whisper | `TOP_TOAST` | 5 | 4 | 302-722 × 16-85 | 430-850 × 16-85 |
| 42 | Journey | `WINDOW` | 4 | 1 | 588-948 × 16-296 | 844-1204 × 16-296 |
| 43 | Peer | `CORNER` | 2 | 5 | 664-944 × 572-624 | 920-1200 × 732-784 |
| 44 | PlaceName | `TOP_TOAST` | 7 | 5 | 372-652 × 16-47 | 500-780 × 16-47 |

Cover (R4) anchors at both resolutions: `WINDOW` covers `TOP_TOAST` and `EDGE_STATUS`.

### 3.4 Management / builder (OpenTTD / Transport Tycoon Deluxe-style, loosely)

Idea: a toolbar of chips across the top. An advisor strip under the toolbar for prompts and tutor lines. A list column of status rows on the right. One floating window slot for the key-opened panels. The news ticker above the ledger, near the bottom edge, where those games keep a status bar.

#### management anchors (PROPOSAL)

| anchor | corner | offset | width | height budget | class | occupants, rank order (map row) | 1024x640 | 1280x800 |
|---|---|---|---|---|---|---|---|---|
| `TOOLBAR_PLACE` | top-left | x 16, y 16 | 280 | 31 | 5 | PlaceName (44) | 16-296 × 16-47 | 16-296 × 16-47 |
| `TOOLBAR_WATCH` | top-left | x 304, y 16 | 340 | 54 | 5 | Watch (32) | 304-644 × 16-70 | 304-644 × 16-70 |
| `TOOLBAR_PEER` | top-left | x 652, y 16 | 280 | 52 | 5 | Peer (43) | 652-932 × 16-68 | 652-932 × 16-68 |
| `ADVISOR` | top-centre | centred, y 78 | 640 | 69 | 2 | CareStrip (30), CarePrompt (27), Guidance (26), Practice (36) | 192-832 × 78-147 | 320-960 × 78-147 |
| `LIST_1` | top-right | x 16, y 180 | 520 | 52 | 5 | Factory (8), Fab (12), Spill (10) | 488-1008 × 180-232 | 744-1264 × 180-232 |
| `LIST_2` | top-right | x 16, y 240 | 420 | 52 | 5 | Embassy (13), Redemption (16), Hybrid (17) | 588-1008 × 240-292 | 844-1264 × 240-292 |
| `LIST_3` | top-right | x 16, y 300 | 420 | 52 | 5 | Compass (18) | 588-1008 × 300-352 | 844-1264 × 300-352 |
| `LIST_4` | top-right | x 16, y 360 | 420 | 52 | 5 | Well (19), ClimateState (31) | 588-1008 × 360-412 | 844-1264 × 360-412 |
| `WINDOW` | top-left | x 324, y 155 | 560 | 320 | 1 | Voice (9), Allocate (37), Mercy (39), Journey (42), Realm (40) | 324-884 × 155-475 | 324-884 × 155-475 |
| `NEWS_TICKER` | bottom-right | x 16, y 144 | 620 | 69 | 4 | Pulse (28), Pickup (34), Thriving (38), Sovereign (35), Whisper (41), Welcome (29) | 388-1008 × 427-496 | 644-1264 × 587-656 |

#### management slab-by-anchor placement

| row | slab | anchor | rank | class | 1024x640 (Model B) | 1280x800 (Model B) |
|---|---|---|---|---|---|---|
| 8 | Factory | `LIST_1` | 1 | 5 | 488-1008 × 180-232 | 744-1264 × 180-232 |
| 9 | Voice | `WINDOW` | 1 | 1 | 324-884 × 155-207 | 324-884 × 155-207 |
| 10 | Spill | `LIST_1` | 3 | 5 | 488-1008 × 180-232 | 744-1264 × 180-232 |
| 12 | Fab | `LIST_1` | 2 | 5 | 488-1008 × 180-232 | 744-1264 × 180-232 |
| 13 | Embassy | `LIST_2` | 1 | 5 | 588-1008 × 240-292 | 844-1264 × 240-292 |
| 16 | Redemption | `LIST_2` | 2 | 5 | 588-1008 × 240-292 | 844-1264 × 240-292 |
| 17 | Hybrid | `LIST_2` | 3 | 5 | 588-1008 × 240-292 | 844-1264 × 240-292 |
| 18 | Compass | `LIST_3` | 1 | 5 | 588-1008 × 300-352 | 844-1264 × 300-352 |
| 19 | Well | `LIST_4` | 1 | 5 | 588-1008 × 360-412 | 844-1264 × 360-412 |
| 26 | Guidance | `ADVISOR` | 3 | 3 | 252-772 × 78-147 | 380-900 × 78-147 |
| 27 | CarePrompt | `ADVISOR` | 2 | 2 | 282-742 × 78-134 | 410-870 × 78-134 |
| 28 | Pulse | `NEWS_TICKER` | 1 | 4 | 448-1008 × 435-496 | 704-1264 × 595-656 |
| 29 | Welcome | `NEWS_TICKER` | 6 | 4 | 628-1008 × 438-496 | 884-1264 × 598-656 |
| 30 | CareStrip | `ADVISOR` | 1 | 2 | 232-792 × 78-140 | 360-920 × 78-140 |
| 31 | ClimateState | `LIST_4` | 2 | 5 | 588-1008 × 360-412 | 844-1264 × 360-412 |
| 32 | Watch | `TOOLBAR_WATCH` | 1 | 5 | 304-644 × 16-69 | 304-644 × 16-69 |
| 34 | Pickup | `NEWS_TICKER` | 2 | 4 | 648-1008 × 440-496 | 904-1264 × 600-656 |
| 35 | Sovereign | `NEWS_TICKER` | 4 | 4 | 488-1008 × 440-496 | 744-1264 × 600-656 |
| 36 | Practice | `ADVISOR` | 4 | 3 | 192-832 × 78-142 | 320-960 × 78-142 |
| 37 | Allocate | `WINDOW` | 2 | 1 | 324-844 × 155-290 | 324-844 × 155-290 |
| 38 | Thriving | `NEWS_TICKER` | 3 | 4 | 388-1008 × 438-496 | 644-1264 × 598-656 |
| 39 | Mercy | `WINDOW` | 3 | 1 | 324-684 × 155-475 | 324-684 × 155-475 |
| 40 | Realm | `WINDOW` | 5 | 1 | 324-624 × 155-325 | 324-624 × 155-325 |
| 41 | Whisper | `NEWS_TICKER` | 5 | 4 | 588-1008 × 427-496 | 844-1264 × 587-656 |
| 42 | Journey | `WINDOW` | 4 | 1 | 324-684 × 155-435 | 324-684 × 155-435 |
| 43 | Peer | `TOOLBAR_PEER` | 1 | 5 | 652-932 × 16-68 | 652-932 × 16-68 |
| 44 | PlaceName | `TOOLBAR_PLACE` | 1 | 5 | 16-296 × 16-47 | 16-296 × 16-47 |

Cover (R4) anchors: at 1024x640, `WINDOW` covers `LIST_1`–`LIST_4` and `NEWS_TICKER`. At 1280x800, `WINDOW` covers `LIST_1`–`LIST_4`.

### 3.5 Results

RECOMPUTED, visible-together overlapping pairs among the 27 HUD slabs, the Ledger band and the touch rects (preconditions in 3.1):

| preset | resolution | Model A | Model B | excluded: code / same anchor / cover | HUD vs modal plates, excluded by R5 (A / B) |
|--------|------------|---------|---------|--------------------------------------|---------------------------------------------|
| classic | 1024x640 | **0** | **0** | 4 / 29 / 38 | 56 / 57 |
| classic | 1280x800 | **0** | **0** | 3 / 29 / 26 | 32 / 37 |
| minimal | 1024x640 | **0** | **0** | 4 / 72 / 62 | 56 / 56 |
| minimal | 1280x800 | **0** | **0** | 3 / 72 / 51 | 38 / 39 |
| management | 1024x640 | **0** | **0** | 4 / 36 / 40 (B: 41) | 52 / 53 |
| management | 1280x800 | **0** | **0** | 3 / 36 / 6 | 34 / 35 |

Every occupant fits its anchor (width and Model B height), and every anchor lies inside both windows. Model A rects sit inside Model B rects on the same anchor, so the Model B zero implies the Model A zero.

Not counted, and the same for all three presets:

- **Fixed modal-band pairs** that no preset moves (RECOMPUTED). At 1024x640 there are 4 under Model A: Comfort–Settings, Ledger–Settings, Satchel–Settings, Places–Satchel. Model B adds Ledger–Places, for 5. At 1280x800 there are 2 under both models: Comfort–Settings, Ledger–Settings. Settings–Places is excluded by code (map pair 5). Q6, Q7.
- **Parked rows at their coded rects** (rows 14, 15; no anchors). Classic has 8 pairs: Crownstone with Embassy, Journey, Mercy, Realm, TouchL, and War with Peer, TouchUse, Watch. Minimal has 16 and management has 10 at 1024x640 and 6 at 1280x800. Crownstone–TouchL and War–TouchUse are map pairs 81 and 77, already present before any preset. Q8.
- **If rows 16 and 17 stay HELD at their coded anchors** (top 204 / top 244, right 16), these visible-together pairs remain. Classic: 5 (A) / 6 (B) at both resolutions. Minimal: 11 / 12 at both. Management: 12 / 15 at 1024x640 and 4 / 7 at 1280x800. Redemption–Hybrid is one of them under Model B (map 4.1 B pair 25, 4.2 B pair 21). Q9.

## 4. Reset contract

### 4.1 One named default

PROPOSAL. Reset UI always restores `classic` (3.2), named in code as the single default preset. Q1 asks the steward to confirm or pick another of the three. Reset never reads the saved layout, so a bad save cannot break it.

- **From the Esc pause plate** (a new row; the plate itself does not move), Reset applies `classic` at once and writes a save of `base: classic` with no overrides.
- **In edit mode**, Reset stages `classic`, and Save commits it (section 6).

### 4.2 "Safe" means all of these

PROPOSAL, checked at 1024x640, at 1280x800 and at the current window:

- **S1.** Every registry HUD slab is in exactly one anchor. No parked, never-shown or probe row (14, 15, 45, 46) appears.
- **S2.** Every anchor lies inside the window with the 16 px margin.
- **S3.** Every occupant's coded width ≤ the anchor width, and its Model B height ≤ the anchor's height budget.
- **S4.** Zero visible-together pairs by the map's method (1.4) under Model A and Model B. Exclusions come only from R1–R5.
- **S5.** No anchor meets a fixed rect (Ledger, Satchel, touch), except where R1 excludes the pair.
- **S6.** No class-1, 2 or 3 anchor is covered or hidden, and only class-1 panels push.
- **S7.** Global z nodes (rows 1–7, 11, 20–25, 33) keep their coded `Style`.
- **S8.** Reset is idempotent: applying it twice gives the same `Style` on every root.

### 4.3 How the step 3 tests prove it

PROPOSAL. All tests are `--lib` tests in the step 3 files.

- **T1 (pure).** For each preset × {1024x640, 1280x800} × {A, B}, run the overlap method over a registry const table of coded widths and the Model B heights in 2.3. Assert S2–S5 with zero pairs.
- **T2 (pure).** The Reset target is `classic`, and `classic` is in T1's preset list.
- **T3 (pure).** S1 and S6 over the preset tables.
- **T4 (App, `MinimalPlugins`).** Same pattern as `voice_above_ledger1_slab_bottom_clears_ledger`: spawn the slab Startup systems with the registry, apply a custom layout, send Reset, and assert every root's `Style` offsets and width equal `classic`'s. Run Reset again and assert nothing changes (S8).
- **T5 (App).** Missing, unparsable, wrong-schema and stale saves each load `classic` (section 5).
- **T6 (App).** Same pattern as `care_prompt_yield_1_hides_when_care_cycle_active`. For each shared anchor, two occupants both wanting to show leaves only the top one `Visible`. The yielded slab's own state (for example `allocate.panel_open`) is unchanged.
- **T7 (App).** Rows 1–7, 11, 20–25 and 33 keep their coded `Style` after Reset and after each preset switch (S7).
- **Render check, outside the unit tests.** Screenshots of each preset at both sizes, attached to the step 3 PR, confirm that Model B holds (Q10).

## 5. Saving layouts (design only)

FACT, precedent: `shared/local_settings.rs` persists to `data/powrush_settings.json` (`SETTINGS_PATH`, L17) with `schema: "powrush_settings_v1"` (`SETTINGS_SCHEMA`, L18). Fields carry `#[serde(default)]`, `from_json` clamps (L958–L965), and `load_or_default` falls back to `Self::default()` on read or parse failure (L967–L972), through `shared::user_persist::read_named` / `write_named`.

PROPOSAL, saved layout shape:

| field | meaning |
|-------|---------|
| `schema` | `"powrush_hud_layout_v1"`; a new value for any breaking change |
| `registry_rev` | integer the registry bumps whenever slab ids, classes or preset tables change |
| `base` | `classic`, `minimal` or `management` |
| `anchors` | overrides only, each `{ id, corner, x, y, width, hidden }` for an anchor of `base`; anchors not listed keep `base` values |

Edit mode moves anchors, not single slabs between anchors (Q21), so a layout is always "preset + anchor overrides".

PROPOSAL, load and fallback. A failure never edits the file; the file changes only on the next Save or Reset.

- **F1.** Missing or unreadable: `classic`.
- **F2.** Parse error: `classic`, plus a one-line notice on the Esc pause plate.
- **F3.** Unknown or newer `schema`: `classic`, file kept.
- **F4.** `registry_rev` differs: drop every override and keep `base` if it is a known preset (stale save).
- **F5.** Unknown anchor id or `base`, a non-finite number, or a width outside 6.2: whole layout falls back to `base` (or `classic` if `base` is unknown). No partial apply.
- **F6.** After load, run S1–S6 at the current window and at both test sizes. Any failure: `base`, or `classic` if `base` fails too.
- **F7.** On window resize, re-run S2–S4. A failure applies `base` for the session without overwriting the save.

Where the file lives is Q16.

## 6. Edit mode rules

PROPOSAL throughout.

### 6.1 Unlock

- An "Edit HUD" row on the Esc pause plate opens edit mode. The pause plate closes, and R5 stops hiding the HUD while edit mode is up.
- Every editable anchor draws a frame of its budget rect with its id, even when no occupant is showing.
- The edit toolbar holds Save, Cancel and Reset.
- Esc means Cancel. No new hotkey is added, and **E and Q are not read in edit mode**, so edit mode can never reach `try_cross_people_door`. Gameplay input during edit mode is Q17.

### 6.2 Drag and resize

- **Drag** moves the whole anchor. On release, the corner is re-picked from the screen third the anchor's centre falls in, so the anchor keeps its place when the window grows. Offsets are re-measured from the new edges.
- **Snap** to the 16 px margin when within 16 px of an edge. Snap to another anchor's edge plus the 8 px gap when within 8 px.
- **Width**:
  - Minimum = the anchor's coded width (its widest occupant). Shrinking below coded widths is Q18, because wrap is not measured.
  - Maximum = min(640, window width − 32). 640 is the widest coded HUD slab (Practice, row 36).
  - Every button inside keeps at least `TOUCH_HIT_MIN` 44 px, because nothing narrows.
- **Height** is not editable. It stays content-driven inside the Model B budget, and the max-height panels keep 320 (Mercy) and 280 (Journey).
- **No padding, border, font or `text_scale` change.**

### 6.3 Show or hide

- Class 4 and 5 anchors can be hidden.
- Class 1 and 2 anchors cannot: the panel's own key is its show / hide, and the action prompt is needed to play.
- Class 3 cannot either, because Guidance already has H and Practice has P.
- Hiding is R2 semantics: the occupants' state and keys keep running.

### 6.4 Save

- Save is enabled only when S1–S6 pass at the current window and at both test sizes. Otherwise the conflicting frames are outlined and Save stays disabled.
- Save writes `base` plus overrides (section 5).
- Cancel restores the last saved layout. Reset stages `classic`.

### 6.5 Every panel keeps working wherever it is placed

- The registry writes only these `Style` fields on a slab root: `top` / `bottom` / `left` / `right` / `margin.left` (centred anchors) and `width`. It never touches children, `Visibility` logic, `FocusPolicy`, `ZIndex` or text.
- Each slab's systems keep running, and its keys do not change: G, R, M, J, Z, P, H, L, I, U, Tab. Layout never changes input routing. For example, allocate owns Digit2 while open (`rbe_allocate_choice.rs` L342–L343) and voice "nay" is Digit2 (`coop_voice.rs` L111). Both stay as they are.
- Buttons inside a moved slab still receive `Interaction`, because Bevy UI hit-testing follows the computed layout. A step 4 test moves Allocate and presses its button at the new rect.
- Yield, cover and modal yield only hide. Push uses the panel's own close flag.

### 6.6 Stays out of edit mode

- The Title door (row 1, z 120) and the door dimmers (rows 4–6, z 140–142).
- The Pause plate (row 3, z 130), the Comfort banner (row 2, z 131) and the Places plate (row 7, z 132).
- The Ledger band (rows 11 and 33, z 125).
- The touch band (rows 20–25, z 124; Q14).
- Parked rows 14 and 15, never-shown row 45, and probe row 46.
- Title Online stays grey, and nothing in edit mode adds a listen or bind.

## 7. Slice plan, steps 2–4

One PR per step. **The file sets below are a PROPOSAL. Core rules the real PATHS when each card is named.** Each step's gate is `cargo test -p shared -p rsil-identity`, then `cargo test -p powrush-client --lib`.

### Step 2 — central HUD anchor registry

Scope: the registry table and R1 / R2. All 25 non-HELD HUD slabs join with their coded positions (no visual change), except the six moves in 2.5.

PROPOSAL files:

- New `client/src/hud_anchor_registry.rs`.
- `client/src/lib.rs` (module and plugin).
- The six movers: `coop_voice.rs`, `first_session_guidance.rs`, `living_practice_loop.rs`, `mercy_harvest_nodes.rs`, `first_harvest_epiphany.rs` (which also carries rows 28 and 29 as-is), `rbe_allocate_choice.rs`.
- The as-is joiners: `vertical_factory.rs`, `infra_spill.rs`, `fabricator.rs`, `embassy.rs`, `compass.rs`, `skirmish_well.rs`, `climate_visible.rs`, `human_inventory.rs` (rows 32, 34 only), `local_sovereign_session.rs`, `thriving_moments.rs`, `human_soft_panels.rs`, `first_whisper.rs`, `abundance_journey_echo.rs`, `lattice_flow_share.rs`, `climate_plane.rs` (row 44 only).
- All of these are under `client/src/`.
- Not touched: `species_redemption.rs`, `hybrid_matrix.rs` (HELD), `war_week.rs`, `crownstone.rs` (parked), and the band owners `title_screen.rs`, `hex_travel.rs`, `ledger_bind.rs`, `touch_controls.rs` and `foundation_lattice.rs`.
- Core may split the as-is joiners into their own card.

Tests:

- The table equals the coded numbers for the joiners.
- The six moves land as in 2.5.
- A pure overlap test shows the named banked pairs at zero (Model B, both sizes).
- An `ACTION_BAR` yield test.
- `voice_above_ledger1_slab_bottom_clears_ledger` (`coop_voice.rs` L184–L204) moves to the new anchor and keeps its "clears ledger top 135" assertion.

The Comfort banner entry is not sized until the render in section 8 nit 2.

### Step 3 — Reset UI plus three presets

Scope: R3–R5, the three preset tables, Reset, and saving the chosen preset.

PROPOSAL files:

- `client/src/hud_anchor_registry.rs`.
- A new `client/src/hud_presets.rs`, or presets kept in the registry file.
- `client/src/title_screen.rs` (Reset and preset rows on the Esc pause plate; the plate keeps its anchor).
- `shared/local_settings.rs` (a preset id with a serde default).
- Push close hooks in `rbe_allocate_choice.rs`, `human_soft_panels.rs`, `abundance_journey_echo.rs` and `coop_voice.rs`.

Tests: T1–T7 (4.3), plus the render check.

### Step 4 — Unlock / Edit mode

Scope: section 6, and saving overrides (section 5).

PROPOSAL files:

- New `client/src/hud_edit_mode.rs`.
- `client/src/lib.rs`.
- `client/src/hud_anchor_registry.rs` (overrides).
- `client/src/title_screen.rs` (the "Edit HUD" row).
- The save: either `shared/local_settings.rs` or a new `shared/hud_layout.rs` (plus `shared/lib.rs`). Q16.

Tests:

- Drag snaps to edges.
- Width clamps to 6.2.
- Save is disabled on overlap.
- Cancel restores.
- F1–F7.
- Allocate's button is pressed at a moved rect (6.5).
- E and Q are ignored in edit mode.

## 8. Map nits (recorded here; the map is not edited)

1. **L8 Startup sentence.** The map's L8 says "Every spawn function in the table is registered in a `Startup` system of its plugin." Row 46 should be left out of that sentence. Its node (`climate_plane.rs:4129`) is inside `mod tests` (L1650–L1651), in the `#[ignore]` test `high_plate_over_world_and_ultra_volumetric_still_renders` (L4055–L4056), and is not spawned by any plugin's `Startup`. Map section 1 already tags it `[Q2 probe]`.
2. **Comfort banner Dismiss height is unmeasured.** The map's 34 px (Model A, y 10-44) fits one 13 px text line (2 + 16 + 15.6). The banner row (`title_screen.rs` L1447–L1506) also holds the Dismiss button: padding 10 / 6, 1 px border, 12 px "Dismiss" text, `column_gap` 10, `SpaceBetween`. The copy's width (and so its wrap) depends on that button's width, which is not measured. A render is needed before step 2 gives the Comfort entry a height budget.

## 9. Open questions

1. **Q1.** Which preset is the Reset default? `classic` is proposed.
2. **Q2.** Is push (R3) acceptable in `WINDOW` anchors? Allocate auto-opens (`rbe_allocate_choice.rs` L388–L389), so it would close an open Mercy, Journey or Realm panel. In minimal and management, Voice shares `WINDOW` too.
3. **Q3.** Is cover (R4) acceptable? A toast's timer runs while it is hidden, so a toast can expire unseen.
4. **Q4.** May Guidance and Practice yield to CareStrip and CarePrompt in `ACTION_BAR` / `ADVISOR`? This is a new yield beyond #678, and step 2 needs it.
5. **Q5.** Is modal yield (R5) acceptable: the HUD band hidden under Pause, Places, the non-`InYard` doors and Persona?
6. **Q6.** Are the Ledger band's pairs with Settings and Places intended z cover? `ui_above_world.rs` L37 calls the ledger "under pause". Or should the Ledger band yield under Pause too?
7. **Q7.** Is the Comfort banner over the Settings plate (map pair 1; Global 131 over 130) intended? Its height also needs nit 2's render.
8. **Q8.** Parked rows 14 and 15 still spawn (`lib.rs` L149–L150), and their show gates are reachable. They get no anchors, so their pairs with preset slabs stay banked (3.5). Hiding them would touch parked files. Leave them, or HOLD ticket?
9. **Q9.** Rows 16 and 17 are HELD (`docs/GROK_BOT_TODO.md` L178) and their markers are private. The zero results need them to join the registry. Lift the HOLD for a marker-only edit, or accept the leftover pairs in 3.5?
10. **Q10.** Model B is an estimate (map section 3: no render). Who runs the render at both sizes before step 3 locks the budgets? Also, `text_scale` > 1 grows text (guidance clamps 11–22 px per map section 3), which can exceed the Model B budgets. How should budgets treat `text_scale`, given no UI scale changes?
11. **Q11.** PlaceName (row 44) has no visibility toggle found (map). Presets treat it as always visible. Confirm.
12. **Q12.** ClimateState's (row 31) exact gate is not summarised (map). Presets treat it as independent of every other slab. Confirm.
13. **Q13.** Can Persona (row 6) open on the Title door? The map leaves this undetermined. It is out of edit mode either way.
14. **Q14.** Should the touch band (rows 20–25) stay fixed and out of edit mode, with the keep-out offsets in 2.3? Should touch be culled during edit mode?
15. **Q15.** What happens below 1024x640? No preset is proven there.
16. **Q16.** Where does the saved layout live: a field in `data/powrush_settings.json`, or its own named file through `shared::user_persist` (a new path)?
17. **Q17.** Does the yard keep running in edit mode, and does movement input (WASD) stay live?
18. **Q18.** May widths shrink below the coded width once a render measures wrap?
19. **Q19.** The rank orders inside shared anchors (toasts, status) are proposals. Is fixed rank right, or should the latest toast win?
20. **Q20.** Confirm the 8 px gap, the 16 px margin, the touch-clear offsets `right 76` / `right 80` and the y 180 start under the touch column.
21. **Q21.** May edit mode move a single slab out of its shared anchor? Version 1 moves anchors only.
22. **Q22.** May a later preset move the Ledger band (rows 11, 33)? Here it is fixed in every preset.
