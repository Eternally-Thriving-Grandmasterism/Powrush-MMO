# L3_SPAWN_RESEARCH.md — CARD R3 PATHS hunt (docs-only)

**Stamp:** 2026-09-19 · CARD **R3 L3-PATH-RESEARCH**  
**Tip (MUST):** `c1c10c902af5c3234fb3e20e4b489a4a82f908ab` (short `c1c10c90`) — L2 HOUSE-PEOPLE-GATES #463 on `main`.  
**Contact:** info@Rathor.ai  
**Independent of xAI.** No certification / AGSi warranty / legal-product claims.

**Docs-only hunt.** TWO PILES: this WRITE names files that **already exist** on tip. STEWARD keeps fun / feel / Online. **0 meshes.** Title Online stays grey. `playable-preview` stays `11c577e`. Floor `2163551`.

This file does **not** implement spawn. It does **not** edit `client/**` · `shared/**` · `Cargo.toml`. It does **not** invent client paths, pack import, or L3 wire. Hands stay DARK. L3 wire waits a later Steward CARD that copies PATHS from this page.

> ### CONFIRM BEFORE BUILD
> **PATHS research, not a Hands work order.** No mesh, pack, socket, Title lobby, fifth Place, or `client/**` edit follows from these sentences. A later cook CARD starts only when Steward names exact Hands PATHS copied from §1.

---

## 1. Candidate PATHS that exist

Verified on tip `c1c10c90`. Each row is **path + symbol + one-line**. Clerk-named files first; then other spawn / wake / Place symbols found by read-only search on files that exist.

### 1.1 Clerk-named (verified on tip)

| Path | Symbol | What it does |
|---|---|---|
| `docs/PLACE_DRESS_SPEC.md` | §2.1 Garden / Eden plane | Presentation law: Garden is walkable title / light-body, **not** a fifth Place; four Places only; doors ignite after one Tend; crossing one-way this session. |
| `docs/PLAYABLE_RACES.md` | §1.1 Door → landing law | Five Peoples → four Place landings: Human Sanctuary yard · Cydruid Heartwood · Quellorian Threshold · Draek Depths teal way-home · Ambrosian Sanctuary well-from-above (same Sanctuary as Human). |
| `docs/MESH_PERSONA_COURT.md` | Comfort L/M/H Persona dress bank | Banks court for Persona dress under Comfort; **no invent PATHS**; Hands mesh HOLD until a later CARD names exact Hands PATHS. |
| `docs/MERCY_PERSONA_CREATION.md` | §1 mechanical_race ≠ people | Persona spec: sim-owned `mechanical_race` / `body_kit`; player-owned presentation; no Title race lobby; Hour can finish without persons. |
| `client/src/title_screen.rs` | `garden_boot_god_plane_doors` · `garden_cross_landing` · `LaunchDoor` · `TITLE_CHROME_PLAY` | Title chrome stays Play / Continue / Settings · Online grey; Garden / boot hosts five People-doors as **law only** (no race portraits); `garden_cross_landing` wraps L2 `try_cross_people_door`. |
| `client/src/first_session_guidance.rs` | `FirstSessionGuidance::try_cross_people_door` · `god_plane_doors_ignited` · `GARDEN_PEOPLE` / `GARDEN_WANT` | Stranger card: first minutes People + Want; L2 wrappers ignite doors after one Tend and offer five Peoples after House; skip House stays light / Peace. |
| `client/src/hour_sacred.rs` | `HousePeople` · `PeopleLanding` · `try_cross_people_door` | L2 HOUSE-PEOPLE-GATES: five Peoples after Q House; four Place **names** among five landings; session-local one-way cross; returns `PeopleLanding` — does **not** move the body or write `PlaceId`. |
| `client/src/hex_travel.rs` | `HexTravelState` · `apply_title_boot` · `PlacesRoom` · `apply_place` | Lived Esc→Places door: Play boots Sanctuary; Continue without book boots Sanctuary; Threshold **room** loads Heartwood `PlaceId` (no fifth PlaceId). |
| `shared/hex_travel.rs` | `PlaceId` · `boot_place` · `LOCAL_HEXES` · `confirm_leave` | Disk-only hex id: **three** variants (`Sanctuary` · `Heartwood` · `Depths`); Play always Sanctuary; Heartwood / Depths need the book; isolation gamma 0. |
| `shared/persona.rs` | `MechanicalRace` · `BodyKit` · `PERSONA_CREATOR_ENABLED` · `STEWARD_ONLINE_YES` | Shared persona record: five mechanical races + height/reach kit; Online picker default-off; Title Online stays grey (`STEWARD_ONLINE_YES = false`). |
| `client/src/lived_hour_bind.rs` | `LivedHourBind` · `tend` / `tend_nearest` · `focus_id` | Binds first-hour hands to climate / week persist; `apply_place` swaps this bind's climate when Places travel lands — **not** wired from `try_cross_people_door`. |

Clerk quotes (tip):

- `PLACE_DRESS_SPEC` §2: **Four Places only** — Sanctuary / Heartwood / Threshold / Depths. Garden ≠ Place.
- `PLAYABLE_RACES` §1.1: **5 doors = 5 Peoples. Landings = 4 Places only.**
- `hour_sacred.rs` L2 header: Q House offers five Peoples as God-plane doors (D0 @ `2afff36`). C0 Cydruid = human-in-frame, not treant. 0 meshes. Not #459.
- `shared/hex_travel.rs`: `LOCAL_HEXES: [PlaceId; 3]` — Threshold is **not** a `PlaceId`.

### 1.2 Other existing spawn / wake / Place symbols (file exists on tip)

| Path | Symbol | What it does |
|---|---|---|
| `client/src/gltf_integration.rs` | `MeshLodPlan` · `plan_for_preset` | Comfort L/M/H presentation plan (primitives / optional on-disk glb). **Not** a standalone `MeshLodPlan` file — see GAP. |
| `shared/local_settings.rs` | `MeshLod` · `GraphicsPreset` | Esc Comfort Low / Medium (default) / High → mesh + weather fidelity. No Ultra. |
| `client/src/human_presence.rs` | `spawn_human_presence` | Startup stacked-capsule body on the yard; Comfort `MeshLodPlan` scales detail. Does not read `PeopleLanding`. |
| `client/src/climate_plane.rs` | `spawn_climate_place` | Place-dress fog / tint / path stones for the current lived hex. Four Places stay four. |
| `client/src/depths_landing.rs` | `DepthsLanding` · `DepthsPeaceTend` | Depths hex node + restore-not-Take. Lives on `PlaceId::Depths` after Places travel — not after a People-door cross. |
| `client/src/living_body.rs` | `LivingBody` · `BodyTell` | Breath / carry / shade presentation. Grove / Heartwood rest the lungs. No Place spawn table. |
| `shared/heartwood_lamp.rs` | `try_place_building` · `heartwood_bath_return` | Heartwood spatial refuse (water / empty lamp disk). Bath returns to Lip — not a People-door spawn. |
| `shared/threshold_shelf.rs` | `THRESHOLD_SHELF_CENTER` · `threshold_use_in_reach` | Threshold is a **Heartwood roof shelf** (look + tend). Session-local; not a `PlaceId`; not a fifth disk file. |
| `docs/ART_BIBLE.md` | Place accent table · Cydruid dress line | Sanctuary warm-gold well · Heartwood amber lamp · Threshold iron + tend seam · Depths teal Peace. Cydruid = human-in-frame; teal / leaf-metal = trim, not bark. |
| `docs/ASSET_BUDGET_COURT.md` | Comfort L/M/H asset budget | Court bank L2 already cites (`L2_ASSET_BUDGET_CITE`). No binary pack. Cite only. |
| `docs/PLACES_BIBLE.md` | Sanctuary Prime = yard only | Heartwood rings stay on Heartwood. No Heartwood mesh on Sanctuary. Market is not a Place. |
| `docs/SETTLED_DOOR_CLARITY.md` | Places as **door**, not list-row teleport | Four disk rooms; Threshold shares Heartwood save-slot. Arrival should feel like a room, not a camera cut. |
| `shared/persona_templates.rs` | LocalTemplate | Offline persona starter paints. Not a spawn table. |
| `client/src/local_player.rs` | `handle_local_player_desync` | Entity-missing respawn / reconnect. Not Place-home wake. |

`HousePeople::landing` (exists in `hour_sacred.rs`) already names the recommended landings. `PeopleLanding::place_name` collapses Ambrosian + Human to `"Sanctuary"` so `four_place_landings_only` holds.

---

## 2. Gaps (no file)

Listed as **GAP**. No invented filenames. No invented client paths.

| GAP | What is missing on tip |
|---|---|
| **GAP — MeshLodPlan filename** | Clerk-named. No `MeshLodPlan.rs` / `MeshLodPlan.md`. The **symbol** `MeshLodPlan` lives inside existing `client/src/gltf_integration.rs`. |
| **GAP — PeopleLanding → PlaceId + body** | No file maps `PeopleLanding` onto `PlaceId` **and** a lived spawn transform. L2 `try_cross_people_door` returns an enum only. |
| **GAP — spawn-on-cross wire** | No file applies a People-door cross to `HexTravelState` / `apply_place` / `LivedHourBind` / `SoftPresence`. Title `garden_cross_landing` and guidance wrappers stop at `Option<PeopleLanding>`. |
| **GAP — Threshold as PlaceId** | Four Place **names** exist (docs + `PeopleLanding` + `PlacesRoom`). Disk `PlaceId` has **three** variants. Threshold rides Heartwood's hex file. No fourth `PlaceId` (and this CARD must not invent one). |
| **GAP — body / Place-home wake table** | `BodyKit` is height/reach only. No file ties `HousePeople` to a wake `Vec3` (Sanctuary yard · Heartwood · Threshold shelf · Depths teal home · Sanctuary well-from-above). |
| **GAP — Garden / Eden hex** | No Garden hex file. D0 law: Garden is unreachable as a lived Place. Not a missing fifth Place — do not add one. |
| **GAP — L3 Hands wire** | No L3 cook file. This CARD is the PATHS hunt only. Wire waits a later Steward CARD. |

Do **not** treat these GAPS as authorization to create those files on this CARD.

---

## 3. Recommended L3 WRITE (later cook CARD — text only)

**Do not implement on this CARD.** Copy into a later Steward CARD. Hands stay DARK.

Door-cross (after Q House + one Tend, one-way this session) should land the **body** here:

| People | Lands in | Place (four only) |
|---|---|---|
| **Human** | Sanctuary yard | Sanctuary |
| **Cydruid** | Heartwood | Heartwood |
| **Quellorian** | Threshold | Threshold (Heartwood disk — no fifth `PlaceId`) |
| **Draek** | Depths teal home | Depths |
| **Ambrosian** | Sanctuary well-from-above | Sanctuary — **same Place as Human, not a 5th room** |

Skip House = stay light-body / Peace default. No People offer. No landing.

**Cydruid dress (C0):** Chakra / Vessel **human-in-frame**, **not** treant.

- **Chakra** — human body kept; machine worn (harness, plates, teal / leaf-metal **trim**).
- **Vessel** — human head / brain housed; robot body walked.
- Heartwood Place may stay living-wood. **The person is not a tree.**
- STRIKE: bark-skin · vines-as-body · heartwood-core-as-body · living-wood limbs as the People · treant-lean.

Existing L2 symbols a later cook may **read** (not rewritten here): `HousePeople` · `PeopleLanding` · `try_cross_people_door` · `garden_cross_landing` · `HexTravelState` · `apply_place` · `PlaceId` · `spawn_human_presence`. A later CARD must name which of those Hands PATHS it claims. This page does not claim them.

Title stays Play / Continue / Settings · **Online grey**. No race portraits. No pack import. **0 meshes** from this research stamp.

---

## 4. CITE

| Cite | Where it already lives on tip | Use |
|---|---|---|
| **D0 Eden** | `docs/PLAYABLE_RACES.md` stamp `D0-2026-09-19-EDEN-PLANE-LAW` · `docs/PLACE_DRESS_SPEC.md` §2.1 + D0 stamp · L2 cites `@ 2afff36` | Garden / God-plane = walkable title / light body. Not Sanctuary dirt. Not a fifth Place. 5 doors = 5 Peoples. Landings = 4 Places. |
| **C0 Cydruid-not-treant** | `docs/PLAYABLE_RACES.md` §5 + stamp `C0-2026-09-19-CYDRUID-NOT-TREANT` · `docs/ART_BIBLE.md` Cydruid dress line · `HousePeople::people_line` | Cydruid = human housed in a cyborg frame (Chakra / Vessel). Nature = practice, not species. |
| **ACityGamesInc/status/2101247905218568248** | Steward Imagine stills (X) | **Imagine stills only.** Codex / Imagine **references**. No pack import. No `.glb`. No binary dump into this tree. |

Also already on tip (cite, do not reopen): L2 #463 `c1c10c90` · ASSET_BUDGET_COURT @ `5eff19c` · MESH_PERSONA_COURT · MERCY_PERSONA_CREATION. X status `1998585780420427986` is the C0 cite already printed in `PLAYABLE_RACES` / `ART_BIBLE` (cite only; do not fetch binaries).

---

## REFUSE (this CARD)

`client/**` edits · meshes · packs · sockets · Title lobby · fifth Place · implementing spawn · inventing filenames · cook #459 · Online lit · merge to `main` · `Cargo.toml` · other docs except this file.

**Budget:** 0 meshes.

L3 wire waits a later Steward CARD copying PATHS from this doc. Hands DARK after COMPLETE.

**Contact:** info@Rathor.ai  
Capable · Bounded · Corrigible. Not METR. Thunder locked. yoi ⚡
