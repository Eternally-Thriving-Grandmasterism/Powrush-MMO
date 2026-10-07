# HUD slab map

Facts only. Every absolutely positioned UI node spawned by a plugin that `client/src/lib.rs` loads, at main `a51f7ee5`. Source: `client/src/*.rs`.

## 1. Which modules are loaded

- `client/src/lib.rs` declares the module tree (`pub mod ...`, L8-L74) and `PowrushClientBundle::build` (L137-L203) adds one plugin per module with a UI spawn. `client/src/main.rs` adds `PowrushClientBundle` (L69) and only diagnostics plugins besides.
- Absolute nodes were found by searching every declared module for `PositionType::Absolute` (and any other `position_type` use: none). Every spawn function in the table is registered in a `Startup` system of its plugin.
- Not declared in `lib.rs`, so left out: `ability_bar.rs`, `faction_diplomacy_ui.rs`, `inventory_ui.rs`, `my_mercy_journey_panel.rs`, `onboarding_ui.rs`, `player_progress_ui.rs`, `rbe_ui_feedback.rs`, `realm_travel_panel.rs`, `treaty_negotiation_ui.rs`, `ui.rs` (all contain `PositionType::Absolute`, none is a module of the crate).
- Parked per `docs/GROK_BOT_TODO.md` L190 / `docs/NEXT_NAMED_CARDS.md` L581 (DO-NOT-FREESTYLE list): `war_week.rs`, `crownstone.rs`, `coop_voice.rs`. All three are declared and their plugins are added in `lib.rs` (L144, L149, L150), so their slabs spawn. `war_week.rs` and `crownstone.rs` are tagged `[parked]`; `coop_voice.rs` is tagged `[parked list]` because the docs list it but it also carries the co-op voice slab. `docs/LIVE_MOUTH_AUDIT.md` L25 calls `species_redemption.rs` HELD (Crownstone); it is not tagged.
- `foundation_lattice.rs` is loaded but `toggle_lattice_panel` sets `panel_open = false` every frame (L112-L118), so its slab is never visible (`[never shown]`).
- The green box in `climate_plane.rs` L4129 is inside an `#[ignore]` test (`high_plate_over_world_and_ultra_volumetric_still_renders`, 320x180 window), not spawned by the running client (`[Q2 probe]`).

Notation: `Global N` = `ZIndex::Global(N)`. `default` = no `ZIndex` set. Offsets are Bevy `Val::Px` unless a `%` is shown. Centred slabs use `left: 50%` plus a negative `margin-left` of half the width.

## 2. Absolute nodes

| # | file:line | slab | anchor | horizontal placement | width | z-index | when it shows |
|---|-----------|------|--------|----------------------|-------|---------|---------------|
| 1 | `title_screen.rs:1331` | TitleRoot (Title door dimmer) | top/left unset (0,0 default); height 100% | full width, left unset | 100% x 100%, padding 24 (TITLE_SAFE_INSET) | Global 120 (LIVED_UI_Z_TITLE) | `sync_title_visibility`: shown when `LaunchDoor::Title` |
| 2 | `title_screen.rs:1447` | ComfortGraphicsBannerRoot | top 10 | centred: left 50% + margin-left -260 | 520 | Global 131 (PAUSE + 1) | `sync_comfort_graphics_banner`: hidden if `comfort_graphics_banner_dismissed`; shown on `LaunchDoor::Title`, or when pause settings open and Places closed |
| 3 | `title_screen.rs:1609` | SettingsStubRoot (Esc pause plate) | top 1% | centred: left 50% + margin-left -210 | 420; max-height 98% | Global 130 (LIVED_UI_Z_PAUSE) | `label.settings_open` (Esc / Start / key 3 in yard; Settings button or Digit3 on Title) and Places plate closed, door not NameHouse/HouseDress, persona closed |
| 4 | `title_screen.rs:1813` | NameHouseRoot (dimmer) | full viewport (no offsets) | full width | 100% x 100% | Global 140 | `sync_name_house_visibility`: `LaunchDoor::NameHouse` |
| 5 | `title_screen.rs:1884` | HouseDressRoot (dimmer) | full viewport (no offsets) | full width | 100% x 100% | Global 141 | `sync_house_dress_visibility`: `LaunchDoor::HouseDress` |
| 6 | `title_screen.rs:2045` | PersonaCreatorRoot (dimmer) | full viewport (no offsets) | full width | 100% x 100% | Global 142 | `sync_persona_creator_visibility`: `persona.open && PERSONA_CREATOR_ENABLED` (const is true) |
| 7 | `hex_travel.rs:449` | PlacesRoot (four-room Places plate) | top 18% | centred: left 50% + margin-left -200 | 400 | Global 132 (PLACES_PLATE_Z = PAUSE + 2) | `sync_places_visibility`: `PlacesPlate.open` (opened from the Esc pause Places row) |
| 8 | `vertical_factory.rs:77` | FactorySlabRoot | top 16 | centred: left 50% + margin-left -260 | 520 | default | `update_factory_slab`: `hex != Peace || door ready || pack.complete` |
| 9 | `coop_voice.rs:44` | VoiceSlabRoot (co-op voice) [parked list] | bottom 88 | centred: left 50% + margin-left -280 | 560 | default | `update_voice_slab`: `charter_skin_live && sash_open` (G toggles, never in Peace) |
| 10 | `infra_spill.rs:69` | SpillSlabRoot | top 52 | left 16 | 520 | default | `update_spill_slab`: `witness.visible_on(hex) && charter_skin_live` |
| 11 | `ledger_bind.rs:98` | LedgerSlabRoot (ledger sash) | bottom 16 | left 16 | 560 | Global 125 (LIVED_UI_Z_LEDGER) | `update_ledger_slab`: `sash_open` (L toggles off Peace; also LivedUiPlate) |
| 12 | `fabricator.rs:75` | FabSlabRoot | top 88 | centred: left 50% + margin-left -260 | 520 | default | `update_fab_slab`: `hour.complete && charter_skin_live && factory tutorial_complete` |
| 13 | `embassy.rs:66` | EmbassySlabRoot | top 124 | right 16 | 420 | default | `update_embassy_slab`: `!heartwood_stub && hour.complete && charter_skin_live && lamp_live` |
| 14 | `war_week.rs:43` | WarSlabRoot (war week chart) [parked] | bottom 52 | right 16 | 420 | default | `update_war_slab`: `charter_skin_live && sash_open` (Tab toggles; not in Peace) |
| 15 | `crownstone.rs:44` | CrownstoneSlabRoot [parked] | top 164 | right 16 | 420 | default | `update_crownstone_slab`: `hour_three_complete && charter_skin_live && embassy.seated` |
| 16 | `species_redemption.rs:45` | RedemptionSlabRoot | top 204 | right 16 | 420 | default | `update_redemption_slab`: `charter_skin_live && stone.witnessed` |
| 17 | `hybrid_matrix.rs:45` | HybridSlabRoot | top 244 | right 16 | 420 | default | `update_hybrid_slab`: `charter_skin_live && redemption.events > 0` |
| 18 | `compass.rs:40` | CompassSlabRoot | bottom 92 | right 16 | 420 | default | `update_compass_slab`: `yard.last.is_some()` (compass tell at live warrant W 20 / 60) |
| 19 | `skirmish_well.rs:108` | WellSlabRoot | bottom 132 | left 16 | 420 | default | `update_well_slab`: `near_first_well(presence)` (within CONTEST_REACH of well 0) |
| 20 | `touch_controls.rs:69` | TouchOverlayRoot (transparent hit root) | full viewport (no offsets) | full width | 100% x 100% | Global 124 (LEDGER - 1) | `sync_touch_overlay_visibility`: not culled (title / pause / ledger / inventory / Q-hint cull it) and on-screen sticks resolve true (setting or last pointer = Touch) |
| 21 | `touch_controls.rs:89` | TouchStickZone | bottom 24 | left 24 | 120 x 120 | child of TouchOverlayRoot (Global 124) | same as TouchOverlayRoot |
| 22 | `touch_controls.rs:119` | TouchUseBtn (via spawn_overlay_btn L182) | bottom 36 | right 28 | 44 x 44 (TOUCH_HIT_MIN) | child of TouchOverlayRoot (Global 124) | same as TouchOverlayRoot |
| 23 | `touch_controls.rs:133` | TouchPauseBtn (via spawn_overlay_btn L182) | top 24 | right 24 | 44 x 44 | child of TouchOverlayRoot (Global 124) | same as TouchOverlayRoot |
| 24 | `touch_controls.rs:145` | TouchQBtn (via spawn_overlay_btn L182) | top 76 (24 + 44 + 8) | right 24 | 44 x 44 | child of TouchOverlayRoot (Global 124) | same as TouchOverlayRoot |
| 25 | `touch_controls.rs:157` | TouchLBtn (via spawn_overlay_btn L182) | top 128 (24 + 2 x 52) | right 24 | 44 x 44 | child of TouchOverlayRoot (Global 124) | same as TouchOverlayRoot |
| 26 | `first_session_guidance.rs:792` | FirstSessionGuidanceStrip | bottom 72 | centred: left 50% + margin-left -260 | 520 | default | `update_guidance_visibility`: `LaunchDoor::InYard && guidance.active && !dismissed && !bind.guidance_hidden` (H hides); spawned Visible |
| 27 | `first_harvest_epiphany.rs:306` | WorldCarePromptRoot | bottom 128 | centred: left 50% + margin-left -230 | 460 | default | `update_world_care_prompt`: `world_care_prompt_visible` (not dismissed; nodes in range, or prompt window, or nodes exist before first harvest); spawned Visible |
| 28 | `first_harvest_epiphany.rs:342` | HarvestPulseRoot | top 118 | centred: left 50% + margin-left -280 | 560 | default | `update_harvest_pulse`: `now < pulse_until && pulse_line non-empty` |
| 29 | `first_harvest_epiphany.rs:378` | WelcomeBackRoot | top 16 | left 16 | 380 | default | `welcome_visible`: `welcome_until` set, `welcome_shown`, before deadline, no E interact since (LivedUiPlate) |
| 30 | `mercy_harvest_nodes.rs:390` | CareCycleStrip | bottom 128 | centred: left 50% + margin-left -280 | 560 | default | `update_care_cycle_strip`: `CareCycleOffer.active` (Idle after tend) |
| 31 | `climate_visible.rs:291` | ClimateStateRoot | bottom 176 | left 16 | 420 | default | `update_climate_state_slab`: speech/notice conditions (threshold, wards, depths lines; guidance-hidden rules); exact gate not summarised |
| 32 | `human_inventory.rs:120` | WatchStripRoot | bottom 16 | left 16 | 340 | default | `update_watch_strip`: `first_harvest_lived || harvests > 0` |
| 33 | `human_inventory.rs:154` | SatchelRoot (I satchel) | bottom 22% | left 16 | 300 | Global 125 (LIVED_UI_Z_LEDGER) | `update_satchel`: `inv.open` (I toggles); also LivedUiPlate |
| 34 | `human_inventory.rs:215` | PickupFlashRoot | top 38% | centred: left 50% + margin-left -180 | 360 | default | `update_pickup_flash`: `now < pickup_until && pickup_line non-empty` |
| 35 | `local_sovereign_session.rs:81` | SovereignBannerRoot | top 52 | centred: left 50% + margin-left -260 | 520 | default | `update_banner`: `!dismissed && now < banner_until`; spawned Visible |
| 36 | `living_practice_loop.rs:202` | LivingPracticeStrip | bottom 72 | centred: left 50% + margin-left -320 | 640 | default | `update_practice_visibility`: `practice.active && !dismissed && !(guidance.active && !guidance.dismissed)` (P toggles) |
| 37 | `rbe_allocate_choice.rs:226` | AllocatePanelRoot | bottom 140 | centred: left 50% + margin-left -260 | 520 | default | `update_allocate_visibility` (`allocate.panel_open`; R toggles, Esc closes) |
| 38 | `thriving_moments.rs:151` | ThrivingToastRoot | top 48 | centred: left 50% + margin-left -310 | 620 | default | `update_toast_ui`: `moments.current.is_some()` |
| 39 | `human_soft_panels.rs:63` | MercySoftRoot (M journey) | top 10% | right 2% | 360; max-height 320 | default | `panels.mercy_open` (M toggles) |
| 40 | `human_soft_panels.rs:116` | RealmSoftRoot (Z realm) | top 18% | left 2% | 300 | default | `panels.realm_open` (Z toggles) |
| 41 | `first_whisper.rs:56` | WhisperRoot | top 28% | centred: left 50% + margin-left -210 | 420 | default | `update_whisper`: `clock.showing && now < clock.until` |
| 42 | `abundance_journey_echo.rs:220` | JourneyEchoRoot (J echo) | top 12% | left 2% | 360; max-height 280 | default | `echo.panel_open` (J toggles) |
| 43 | `lattice_flow_share.rs:93` | PeerPresenceRoot | bottom 16 | right 16 | 280 | default | `update_peer_presence_chip`: `share.chip_visible && last_peer.is_some()` (U ingests a peer) |
| 44 | `climate_plane.rs:1145` | ClimateNameRoot (place-name chip) | top 18 | centred: left 50% + margin-left -140 | 280 | default | not stated (no visibility toggle found; spawned with default Visibility) |
| 45 | `foundation_lattice.rs:60` | FoundationLatticeRoot [never shown] | top 14% | right 2% | 380; max-height 500 | default | `panels.panel_open`; `toggle_lattice_panel` sets `panel_open = false` every frame, so never shown |
| 46 | `climate_plane.rs:4129` | green test box (light-bloom-1 probe) [Q2 probe] | top 8 | left 8 | 120 x 36 | default | not stated; inside `#[test] #[ignore]` `high_plate_over_world_and_ultra_volumetric_still_renders` (320x180 lavapipe window), never spawned by the game |

Row count: 46 (45 spawned by the game, 1 test probe).

Child nodes of these roots (text, buttons inside flex columns) are not absolute and are not listed. Rows 21-25 are children of row 20 (`TouchOverlayRoot`) and take the same visibility.

## 3. Height and width model

Widths and offsets are coded. Heights are content-driven, so two estimates are used.
- Plate height = 2 x border + 2 x vertical padding + lines x 1.2 x font px. Text wrap is not measured: no render was run.
- Model A: the number of lines the code forces with `\n` (one line for single-string slabs; ledger settled face 5 lines: house label, yard week, house week, lethal sign, sash).
- Model B: Model A plus one wrapped line on every text-driven slab (ledger 6 lines). Strings such as `War week · N tons · N restored · score N · hex gone green` (about 62 chars) and `Embassy lamp waits on the Proof Pack · civic seat after manufacture` (67 chars) exceed one line of the 390 px inner width of a 420 px plate at 14 px (about 46 chars at 0.6 em per glyph), so B is the likelier case for those.
- Fixed: touch stick 120x120, touch buttons 44x44 (`TOUCH_HIT_MIN`), place-name chip 1 line, allocate panel 117 (A) / 135 (B) (2 text lines + button row), satchel 219 (A) / 235 (B) (9 body lines at 13.5 px + header + footer), Places plate assumed 400 (title, body, four room buttons at 44 px min, Back), Realm panel assumed 170, Mercy / Journey echo panels take their `max-height` (320 / 280), Settings plate takes `max-height` 98% of the window (top 1%).
- Line height 1.2 x font px: 14 px slabs are 35 px (A) / 52 px (B) tall; guidance 48 / 69; practice 46 / 64; care strip 43 / 62; ledger 102 / 119.
- `text_scale` (local settings) rescales font px at runtime on several slabs (for example `guidance_card_font_px`: 17 x scale, clamped 11-22). Widths and offsets do not change. Estimates use scale 1.0.

## 4. Overlapping pairs

Rectangles are in window pixels, origin top-left. Pairs listed are those whose rectangles share positive area. Full-viewport layers are in 4.5. `Yes` in the last column means no code found that excludes the pair: both slabs have their own independent show condition (table above). `No:` gives the code that excludes it. Slabs hidden by their own condition are listed anyway.

### 4.1 1024x640

Model A: 130 overlapping pairs (126 can be visible together, 4 cannot). Model B adds 41 more (171 in total).

Model A pairs:

| # | slab 1 | slab 2 | overlapping region | visible together |
|---|--------|--------|--------------------|------------------|
| 1 | ComfortGraphicsBannerRoot | SettingsStubRoot | x 302-722, y 10-44 | Yes |
| 2 | ComfortGraphicsBannerRoot | FactorySlabRoot | x 252-772, y 16-44 | Yes |
| 3 | ComfortGraphicsBannerRoot | WelcomeBackRoot | x 252-396, y 16-44 | Yes |
| 4 | ComfortGraphicsBannerRoot | ClimateNameRoot | x 372-652, y 18-44 | Yes |
| 5 | SettingsStubRoot | PlacesRoot | x 312-712, y 115-515 | No: `settings_visible_with_places` = `settings_open && !places_open` |
| 6 | SettingsStubRoot | FactorySlabRoot | x 302-722, y 16-51 | Yes |
| 7 | SettingsStubRoot | VoiceSlabRoot | x 302-722, y 517-552 | Yes |
| 8 | SettingsStubRoot | SpillSlabRoot | x 302-536, y 52-87 | Yes |
| 9 | SettingsStubRoot | LedgerSlabRoot | x 302-576, y 522-624 | Yes |
| 10 | SettingsStubRoot | FabSlabRoot | x 302-722, y 88-123 | Yes |
| 11 | SettingsStubRoot | EmbassySlabRoot | x 588-722, y 124-159 | Yes |
| 12 | SettingsStubRoot | WarSlabRoot | x 588-722, y 553-588 | Yes |
| 13 | SettingsStubRoot | CrownstoneSlabRoot | x 588-722, y 164-199 | Yes |
| 14 | SettingsStubRoot | RedemptionSlabRoot | x 588-722, y 204-239 | Yes |
| 15 | SettingsStubRoot | HybridSlabRoot | x 588-722, y 244-279 | Yes |
| 16 | SettingsStubRoot | CompassSlabRoot | x 588-722, y 513-548 | Yes |
| 17 | SettingsStubRoot | WellSlabRoot | x 302-436, y 473-508 | Yes |
| 18 | SettingsStubRoot | FirstSessionGuidanceStrip | x 302-722, y 520-568 | Yes |
| 19 | SettingsStubRoot | WorldCarePromptRoot | x 302-722, y 475-512 | Yes |
| 20 | SettingsStubRoot | HarvestPulseRoot | x 302-722, y 118-160 | Yes |
| 21 | SettingsStubRoot | WelcomeBackRoot | x 302-396, y 16-58 | Yes |
| 22 | SettingsStubRoot | CareCycleStrip | x 302-722, y 469-512 | Yes |
| 23 | SettingsStubRoot | ClimateStateRoot | x 302-436, y 429-464 | Yes |
| 24 | SettingsStubRoot | WatchStripRoot | x 302-356, y 586-624 | Yes |
| 25 | SettingsStubRoot | SatchelRoot | x 302-316, y 280-499 | Yes |
| 26 | SettingsStubRoot | PickupFlashRoot | x 332-692, y 243-280 | Yes |
| 27 | SettingsStubRoot | SovereignBannerRoot | x 302-722, y 52-91 | Yes |
| 28 | SettingsStubRoot | LivingPracticeStrip | x 302-722, y 522-568 | Yes |
| 29 | SettingsStubRoot | AllocatePanelRoot | x 302-722, y 383-500 | Yes |
| 30 | SettingsStubRoot | ThrivingToastRoot | x 302-722, y 48-88 | Yes |
| 31 | SettingsStubRoot | MercySoftRoot | x 644-722, y 64-384 | Yes |
| 32 | SettingsStubRoot | RealmSoftRoot | x 302-320, y 115-285 | Yes |
| 33 | SettingsStubRoot | WhisperRoot | x 302-722, y 179-227 | Yes |
| 34 | SettingsStubRoot | JourneyEchoRoot | x 302-380, y 77-357 | Yes |
| 35 | SettingsStubRoot | ClimateNameRoot | x 372-652, y 18-49 | Yes |
| 36 | PlacesRoot | FabSlabRoot | x 312-712, y 115-123 | Yes |
| 37 | PlacesRoot | EmbassySlabRoot | x 588-712, y 124-159 | Yes |
| 38 | PlacesRoot | CrownstoneSlabRoot | x 588-712, y 164-199 | Yes |
| 39 | PlacesRoot | RedemptionSlabRoot | x 588-712, y 204-239 | Yes |
| 40 | PlacesRoot | HybridSlabRoot | x 588-712, y 244-279 | Yes |
| 41 | PlacesRoot | CompassSlabRoot | x 588-712, y 513-515 | Yes |
| 42 | PlacesRoot | WellSlabRoot | x 312-436, y 473-508 | Yes |
| 43 | PlacesRoot | WorldCarePromptRoot | x 312-712, y 475-512 | Yes |
| 44 | PlacesRoot | HarvestPulseRoot | x 312-712, y 118-160 | Yes |
| 45 | PlacesRoot | CareCycleStrip | x 312-712, y 469-512 | Yes |
| 46 | PlacesRoot | ClimateStateRoot | x 312-436, y 429-464 | Yes |
| 47 | PlacesRoot | SatchelRoot | x 312-316, y 280-499 | Yes |
| 48 | PlacesRoot | PickupFlashRoot | x 332-692, y 243-280 | Yes |
| 49 | PlacesRoot | AllocatePanelRoot | x 312-712, y 383-500 | Yes |
| 50 | PlacesRoot | MercySoftRoot | x 644-712, y 115-384 | Yes |
| 51 | PlacesRoot | RealmSoftRoot | x 312-320, y 115-285 | Yes |
| 52 | PlacesRoot | WhisperRoot | x 312-712, y 179-227 | Yes |
| 53 | PlacesRoot | JourneyEchoRoot | x 312-380, y 115-357 | Yes |
| 54 | FactorySlabRoot | WelcomeBackRoot | x 252-396, y 16-51 | Yes |
| 55 | FactorySlabRoot | ThrivingToastRoot | x 252-772, y 48-51 | Yes |
| 56 | FactorySlabRoot | ClimateNameRoot | x 372-652, y 18-49 | Yes |
| 57 | VoiceSlabRoot | LedgerSlabRoot | x 232-576, y 522-552 | Yes |
| 58 | VoiceSlabRoot | CompassSlabRoot | x 588-792, y 517-548 | Yes |
| 59 | VoiceSlabRoot | FirstSessionGuidanceStrip | x 252-772, y 520-552 | Yes |
| 60 | VoiceSlabRoot | LivingPracticeStrip | x 232-792, y 522-552 | Yes |
| 61 | SpillSlabRoot | WelcomeBackRoot | x 16-396, y 52-58 | Yes |
| 62 | SpillSlabRoot | SovereignBannerRoot | x 252-536, y 52-87 | Yes |
| 63 | SpillSlabRoot | ThrivingToastRoot | x 202-536, y 52-87 | Yes |
| 64 | SpillSlabRoot | JourneyEchoRoot | x 20-380, y 77-87 | Yes |
| 65 | LedgerSlabRoot | TouchStickZone | x 24-144, y 522-616 | No: `sync_touch_overlay_visibility` culls the touch overlay when the ledger sash, inventory, pause settings, Places (or title door) is open |
| 66 | LedgerSlabRoot | FirstSessionGuidanceStrip | x 252-576, y 522-568 | Yes |
| 67 | LedgerSlabRoot | WatchStripRoot | x 16-356, y 586-624 | Yes |
| 68 | LedgerSlabRoot | LivingPracticeStrip | x 192-576, y 522-568 | Yes |
| 69 | FabSlabRoot | HarvestPulseRoot | x 252-772, y 118-123 | Yes |
| 70 | FabSlabRoot | SovereignBannerRoot | x 252-772, y 88-91 | Yes |
| 71 | FabSlabRoot | MercySoftRoot | x 644-772, y 88-123 | Yes |
| 72 | FabSlabRoot | RealmSoftRoot | x 252-320, y 115-123 | Yes |
| 73 | FabSlabRoot | JourneyEchoRoot | x 252-380, y 88-123 | Yes |
| 74 | EmbassySlabRoot | TouchLBtn | x 956-1000, y 128-159 | Yes |
| 75 | EmbassySlabRoot | HarvestPulseRoot | x 588-792, y 124-159 | Yes |
| 76 | EmbassySlabRoot | MercySoftRoot | x 644-1004, y 124-159 | Yes |
| 77 | WarSlabRoot | TouchUseBtn | x 952-996, y 560-588 | Yes |
| 78 | WarSlabRoot | FirstSessionGuidanceStrip | x 588-772, y 553-568 | Yes |
| 79 | WarSlabRoot | LivingPracticeStrip | x 588-832, y 553-568 | Yes |
| 80 | WarSlabRoot | PeerPresenceRoot | x 728-1008, y 587-588 | Yes |
| 81 | CrownstoneSlabRoot | TouchLBtn | x 956-1000, y 164-172 | Yes |
| 82 | CrownstoneSlabRoot | MercySoftRoot | x 644-1004, y 164-199 | Yes |
| 83 | CrownstoneSlabRoot | WhisperRoot | x 588-722, y 179-199 | Yes |
| 84 | RedemptionSlabRoot | MercySoftRoot | x 644-1004, y 204-239 | Yes |
| 85 | RedemptionSlabRoot | WhisperRoot | x 588-722, y 204-227 | Yes |
| 86 | HybridSlabRoot | PickupFlashRoot | x 588-692, y 244-279 | Yes |
| 87 | HybridSlabRoot | MercySoftRoot | x 644-1004, y 244-279 | Yes |
| 88 | CompassSlabRoot | FirstSessionGuidanceStrip | x 588-772, y 520-548 | Yes |
| 89 | CompassSlabRoot | LivingPracticeStrip | x 588-832, y 522-548 | Yes |
| 90 | WellSlabRoot | TouchStickZone | x 24-144, y 496-508 | Yes |
| 91 | WellSlabRoot | WorldCarePromptRoot | x 282-436, y 475-508 | Yes |
| 92 | WellSlabRoot | CareCycleStrip | x 232-436, y 473-508 | Yes |
| 93 | WellSlabRoot | SatchelRoot | x 16-316, y 473-499 | Yes |
| 94 | WellSlabRoot | AllocatePanelRoot | x 252-436, y 473-500 | Yes |
| 95 | TouchStickZone | WatchStripRoot | x 24-144, y 586-616 | Yes |
| 96 | TouchStickZone | SatchelRoot | x 24-144, y 496-499 | No: `sync_touch_overlay_visibility` culls the touch overlay when the ledger sash, inventory, pause settings, Places (or title door) is open |
| 97 | TouchUseBtn | PeerPresenceRoot | x 952-996, y 587-604 | Yes |
| 98 | TouchPauseBtn | MercySoftRoot | x 956-1000, y 64-68 | Yes |
| 99 | TouchQBtn | MercySoftRoot | x 956-1000, y 76-120 | Yes |
| 100 | TouchLBtn | MercySoftRoot | x 956-1000, y 128-172 | Yes |
| 101 | FirstSessionGuidanceStrip | LivingPracticeStrip | x 252-772, y 522-568 | No: `update_practice_visibility` hides the practice strip while `guidance.active && !guidance.dismissed`; the guidance strip needs that same condition to show |
| 102 | WorldCarePromptRoot | CareCycleStrip | x 282-742, y 475-512 | Yes |
| 103 | WorldCarePromptRoot | SatchelRoot | x 282-316, y 475-499 | Yes |
| 104 | WorldCarePromptRoot | AllocatePanelRoot | x 282-742, y 475-500 | Yes |
| 105 | HarvestPulseRoot | MercySoftRoot | x 644-792, y 118-160 | Yes |
| 106 | HarvestPulseRoot | RealmSoftRoot | x 232-320, y 118-160 | Yes |
| 107 | HarvestPulseRoot | JourneyEchoRoot | x 232-380, y 118-160 | Yes |
| 108 | WelcomeBackRoot | SovereignBannerRoot | x 252-396, y 52-58 | Yes |
| 109 | WelcomeBackRoot | ThrivingToastRoot | x 202-396, y 48-58 | Yes |
| 110 | WelcomeBackRoot | ClimateNameRoot | x 372-396, y 18-49 | Yes |
| 111 | CareCycleStrip | SatchelRoot | x 232-316, y 469-499 | Yes |
| 112 | CareCycleStrip | AllocatePanelRoot | x 252-772, y 469-500 | Yes |
| 113 | ClimateStateRoot | SatchelRoot | x 16-316, y 429-464 | Yes |
| 114 | ClimateStateRoot | AllocatePanelRoot | x 252-436, y 429-464 | Yes |
| 115 | SatchelRoot | AllocatePanelRoot | x 252-316, y 383-499 | Yes |
| 116 | SatchelRoot | RealmSoftRoot | x 20-316, y 280-285 | Yes |
| 117 | SatchelRoot | JourneyEchoRoot | x 20-316, y 280-357 | Yes |
| 118 | PickupFlashRoot | MercySoftRoot | x 644-692, y 243-280 | Yes |
| 119 | PickupFlashRoot | JourneyEchoRoot | x 332-380, y 243-280 | Yes |
| 120 | SovereignBannerRoot | ThrivingToastRoot | x 252-772, y 52-88 | Yes |
| 121 | SovereignBannerRoot | MercySoftRoot | x 644-772, y 64-91 | Yes |
| 122 | SovereignBannerRoot | JourneyEchoRoot | x 252-380, y 77-91 | Yes |
| 123 | AllocatePanelRoot | MercySoftRoot | x 644-772, y 383-384 | Yes |
| 124 | ThrivingToastRoot | MercySoftRoot | x 644-822, y 64-88 | Yes |
| 125 | ThrivingToastRoot | JourneyEchoRoot | x 202-380, y 77-88 | Yes |
| 126 | ThrivingToastRoot | ClimateNameRoot | x 372-652, y 48-49 | Yes |
| 127 | MercySoftRoot | WhisperRoot | x 644-722, y 179-227 | Yes |
| 128 | RealmSoftRoot | WhisperRoot | x 302-320, y 179-227 | Yes |
| 129 | RealmSoftRoot | JourneyEchoRoot | x 20-320, y 115-285 | Yes |
| 130 | WhisperRoot | JourneyEchoRoot | x 302-380, y 179-227 | Yes |

Pairs added by Model B only:

| # | slab 1 | slab 2 | overlapping region (B heights) | visible together |
|---|--------|--------|--------------------------------|------------------|
| 1 | ComfortGraphicsBannerRoot | SpillSlabRoot | x 252-536, y 52-59 | Yes |
| 2 | ComfortGraphicsBannerRoot | SovereignBannerRoot | x 252-772, y 52-59 | Yes |
| 3 | ComfortGraphicsBannerRoot | ThrivingToastRoot | x 252-772, y 48-59 | Yes |
| 4 | PlacesRoot | VoiceSlabRoot | x 312-712, y 500-515 | Yes |
| 5 | PlacesRoot | LedgerSlabRoot | x 312-576, y 505-515 | Yes |
| 6 | PlacesRoot | FirstSessionGuidanceStrip | x 312-712, y 499-515 | Yes |
| 7 | PlacesRoot | LivingPracticeStrip | x 312-712, y 504-515 | Yes |
| 8 | FactorySlabRoot | SpillSlabRoot | x 252-536, y 52-68 | Yes |
| 9 | FactorySlabRoot | SovereignBannerRoot | x 252-772, y 52-68 | Yes |
| 10 | FactorySlabRoot | MercySoftRoot | x 644-772, y 64-68 | Yes |
| 11 | VoiceSlabRoot | WarSlabRoot | x 588-792, y 536-552 | Yes |
| 12 | VoiceSlabRoot | WellSlabRoot | x 232-436, y 500-508 | Yes |
| 13 | VoiceSlabRoot | WorldCarePromptRoot | x 282-742, y 500-512 | Yes |
| 14 | VoiceSlabRoot | CareCycleStrip | x 232-792, y 500-512 | Yes |
| 15 | SpillSlabRoot | FabSlabRoot | x 252-536, y 88-104 | Yes |
| 16 | LedgerSlabRoot | WellSlabRoot | x 16-436, y 505-508 | Yes |
| 17 | LedgerSlabRoot | WorldCarePromptRoot | x 282-576, y 505-512 | Yes |
| 18 | LedgerSlabRoot | CareCycleStrip | x 232-576, y 505-512 | Yes |
| 19 | FabSlabRoot | EmbassySlabRoot | x 588-772, y 124-140 | Yes |
| 20 | FabSlabRoot | ThrivingToastRoot | x 252-772, y 88-106 | Yes |
| 21 | EmbassySlabRoot | CrownstoneSlabRoot | x 588-1008, y 164-176 | Yes |
| 22 | WarSlabRoot | CompassSlabRoot | x 588-1008, y 536-548 | Yes |
| 23 | CrownstoneSlabRoot | RedemptionSlabRoot | x 588-1008, y 204-216 | Yes |
| 24 | CrownstoneSlabRoot | HarvestPulseRoot | x 588-792, y 164-179 | Yes |
| 25 | RedemptionSlabRoot | HybridSlabRoot | x 588-1008, y 244-256 | Yes |
| 26 | RedemptionSlabRoot | PickupFlashRoot | x 588-692, y 243-256 | Yes |
| 27 | HybridSlabRoot | WhisperRoot | x 588-722, y 244-248 | Yes |
| 28 | CompassSlabRoot | WorldCarePromptRoot | x 588-742, y 496-512 | Yes |
| 29 | CompassSlabRoot | CareCycleStrip | x 588-792, y 496-512 | Yes |
| 30 | CompassSlabRoot | AllocatePanelRoot | x 588-772, y 496-500 | Yes |
| 31 | WellSlabRoot | FirstSessionGuidanceStrip | x 252-436, y 499-508 | Yes |
| 32 | WellSlabRoot | ClimateStateRoot | x 16-436, y 456-464 | Yes |
| 33 | WellSlabRoot | LivingPracticeStrip | x 192-436, y 504-508 | Yes |
| 34 | FirstSessionGuidanceStrip | WorldCarePromptRoot | x 282-742, y 499-512 | Yes |
| 35 | FirstSessionGuidanceStrip | CareCycleStrip | x 252-772, y 499-512 | Yes |
| 36 | FirstSessionGuidanceStrip | AllocatePanelRoot | x 252-772, y 499-500 | Yes |
| 37 | WorldCarePromptRoot | ClimateStateRoot | x 282-436, y 456-464 | Yes |
| 38 | WorldCarePromptRoot | LivingPracticeStrip | x 282-742, y 504-512 | Yes |
| 39 | CareCycleStrip | ClimateStateRoot | x 232-436, y 450-464 | Yes |
| 40 | CareCycleStrip | LivingPracticeStrip | x 232-792, y 504-512 | Yes |
| 41 | PickupFlashRoot | WhisperRoot | x 332-692, y 243-248 | Yes |

### 4.2 1280x800

Model A: 89 overlapping pairs (86 can be visible together, 3 cannot). Model B adds 37 more (126 in total).

Model A pairs:

| # | slab 1 | slab 2 | overlapping region | visible together |
|---|--------|--------|--------------------|------------------|
| 1 | ComfortGraphicsBannerRoot | SettingsStubRoot | x 430-850, y 10-44 | Yes |
| 2 | ComfortGraphicsBannerRoot | FactorySlabRoot | x 380-900, y 16-44 | Yes |
| 3 | ComfortGraphicsBannerRoot | WelcomeBackRoot | x 380-396, y 16-44 | Yes |
| 4 | ComfortGraphicsBannerRoot | ClimateNameRoot | x 500-780, y 18-44 | Yes |
| 5 | SettingsStubRoot | PlacesRoot | x 440-840, y 144-544 | No: `settings_visible_with_places` = `settings_open && !places_open` |
| 6 | SettingsStubRoot | FactorySlabRoot | x 430-850, y 16-51 | Yes |
| 7 | SettingsStubRoot | VoiceSlabRoot | x 430-850, y 677-712 | Yes |
| 8 | SettingsStubRoot | SpillSlabRoot | x 430-536, y 52-87 | Yes |
| 9 | SettingsStubRoot | LedgerSlabRoot | x 430-576, y 682-784 | Yes |
| 10 | SettingsStubRoot | FabSlabRoot | x 430-850, y 88-123 | Yes |
| 11 | SettingsStubRoot | EmbassySlabRoot | x 844-850, y 124-159 | Yes |
| 12 | SettingsStubRoot | WarSlabRoot | x 844-850, y 713-748 | Yes |
| 13 | SettingsStubRoot | CrownstoneSlabRoot | x 844-850, y 164-199 | Yes |
| 14 | SettingsStubRoot | RedemptionSlabRoot | x 844-850, y 204-239 | Yes |
| 15 | SettingsStubRoot | HybridSlabRoot | x 844-850, y 244-279 | Yes |
| 16 | SettingsStubRoot | CompassSlabRoot | x 844-850, y 673-708 | Yes |
| 17 | SettingsStubRoot | WellSlabRoot | x 430-436, y 633-668 | Yes |
| 18 | SettingsStubRoot | FirstSessionGuidanceStrip | x 430-850, y 680-728 | Yes |
| 19 | SettingsStubRoot | WorldCarePromptRoot | x 430-850, y 635-672 | Yes |
| 20 | SettingsStubRoot | HarvestPulseRoot | x 430-850, y 118-160 | Yes |
| 21 | SettingsStubRoot | CareCycleStrip | x 430-850, y 629-672 | Yes |
| 22 | SettingsStubRoot | ClimateStateRoot | x 430-436, y 589-624 | Yes |
| 23 | SettingsStubRoot | PickupFlashRoot | x 460-820, y 304-341 | Yes |
| 24 | SettingsStubRoot | SovereignBannerRoot | x 430-850, y 52-91 | Yes |
| 25 | SettingsStubRoot | LivingPracticeStrip | x 430-850, y 682-728 | Yes |
| 26 | SettingsStubRoot | AllocatePanelRoot | x 430-850, y 543-660 | Yes |
| 27 | SettingsStubRoot | ThrivingToastRoot | x 430-850, y 48-88 | Yes |
| 28 | SettingsStubRoot | WhisperRoot | x 430-850, y 224-272 | Yes |
| 29 | SettingsStubRoot | ClimateNameRoot | x 500-780, y 18-49 | Yes |
| 30 | PlacesRoot | HarvestPulseRoot | x 440-840, y 144-160 | Yes |
| 31 | PlacesRoot | PickupFlashRoot | x 460-820, y 304-341 | Yes |
| 32 | PlacesRoot | AllocatePanelRoot | x 440-840, y 543-544 | Yes |
| 33 | PlacesRoot | WhisperRoot | x 440-840, y 224-272 | Yes |
| 34 | FactorySlabRoot | WelcomeBackRoot | x 380-396, y 16-51 | Yes |
| 35 | FactorySlabRoot | ThrivingToastRoot | x 380-900, y 48-51 | Yes |
| 36 | FactorySlabRoot | ClimateNameRoot | x 500-780, y 18-49 | Yes |
| 37 | VoiceSlabRoot | LedgerSlabRoot | x 360-576, y 682-712 | Yes |
| 38 | VoiceSlabRoot | CompassSlabRoot | x 844-920, y 677-708 | Yes |
| 39 | VoiceSlabRoot | FirstSessionGuidanceStrip | x 380-900, y 680-712 | Yes |
| 40 | VoiceSlabRoot | LivingPracticeStrip | x 360-920, y 682-712 | Yes |
| 41 | SpillSlabRoot | WelcomeBackRoot | x 16-396, y 52-58 | Yes |
| 42 | SpillSlabRoot | SovereignBannerRoot | x 380-536, y 52-87 | Yes |
| 43 | SpillSlabRoot | ThrivingToastRoot | x 330-536, y 52-87 | Yes |
| 44 | LedgerSlabRoot | TouchStickZone | x 24-144, y 682-776 | No: `sync_touch_overlay_visibility` culls the touch overlay when the ledger sash, inventory, pause settings, Places (or title door) is open |
| 45 | LedgerSlabRoot | FirstSessionGuidanceStrip | x 380-576, y 682-728 | Yes |
| 46 | LedgerSlabRoot | WatchStripRoot | x 16-356, y 746-784 | Yes |
| 47 | LedgerSlabRoot | LivingPracticeStrip | x 320-576, y 682-728 | Yes |
| 48 | FabSlabRoot | HarvestPulseRoot | x 380-900, y 118-123 | Yes |
| 49 | FabSlabRoot | SovereignBannerRoot | x 380-900, y 88-91 | Yes |
| 50 | FabSlabRoot | MercySoftRoot | x 894-900, y 88-123 | Yes |
| 51 | FabSlabRoot | JourneyEchoRoot | x 380-386, y 96-123 | Yes |
| 52 | EmbassySlabRoot | TouchLBtn | x 1212-1256, y 128-159 | Yes |
| 53 | EmbassySlabRoot | HarvestPulseRoot | x 844-920, y 124-159 | Yes |
| 54 | EmbassySlabRoot | MercySoftRoot | x 894-1254, y 124-159 | Yes |
| 55 | WarSlabRoot | TouchUseBtn | x 1208-1252, y 720-748 | Yes |
| 56 | WarSlabRoot | FirstSessionGuidanceStrip | x 844-900, y 713-728 | Yes |
| 57 | WarSlabRoot | LivingPracticeStrip | x 844-960, y 713-728 | Yes |
| 58 | WarSlabRoot | PeerPresenceRoot | x 984-1264, y 747-748 | Yes |
| 59 | CrownstoneSlabRoot | TouchLBtn | x 1212-1256, y 164-172 | Yes |
| 60 | CrownstoneSlabRoot | MercySoftRoot | x 894-1254, y 164-199 | Yes |
| 61 | RedemptionSlabRoot | MercySoftRoot | x 894-1254, y 204-239 | Yes |
| 62 | RedemptionSlabRoot | WhisperRoot | x 844-850, y 224-239 | Yes |
| 63 | HybridSlabRoot | MercySoftRoot | x 894-1254, y 244-279 | Yes |
| 64 | HybridSlabRoot | WhisperRoot | x 844-850, y 244-272 | Yes |
| 65 | CompassSlabRoot | FirstSessionGuidanceStrip | x 844-900, y 680-708 | Yes |
| 66 | CompassSlabRoot | LivingPracticeStrip | x 844-960, y 682-708 | Yes |
| 67 | WellSlabRoot | TouchStickZone | x 24-144, y 656-668 | Yes |
| 68 | WellSlabRoot | WorldCarePromptRoot | x 410-436, y 635-668 | Yes |
| 69 | WellSlabRoot | CareCycleStrip | x 360-436, y 633-668 | Yes |
| 70 | WellSlabRoot | AllocatePanelRoot | x 380-436, y 633-660 | Yes |
| 71 | TouchStickZone | WatchStripRoot | x 24-144, y 746-776 | Yes |
| 72 | TouchUseBtn | PeerPresenceRoot | x 1208-1252, y 747-764 | Yes |
| 73 | TouchQBtn | MercySoftRoot | x 1212-1254, y 80-120 | Yes |
| 74 | TouchLBtn | MercySoftRoot | x 1212-1254, y 128-172 | Yes |
| 75 | FirstSessionGuidanceStrip | LivingPracticeStrip | x 380-900, y 682-728 | No: `update_practice_visibility` hides the practice strip while `guidance.active && !guidance.dismissed`; the guidance strip needs that same condition to show |
| 76 | WorldCarePromptRoot | CareCycleStrip | x 410-870, y 635-672 | Yes |
| 77 | WorldCarePromptRoot | AllocatePanelRoot | x 410-870, y 635-660 | Yes |
| 78 | HarvestPulseRoot | MercySoftRoot | x 894-920, y 118-160 | Yes |
| 79 | HarvestPulseRoot | JourneyEchoRoot | x 360-386, y 118-160 | Yes |
| 80 | WelcomeBackRoot | SovereignBannerRoot | x 380-396, y 52-58 | Yes |
| 81 | WelcomeBackRoot | ThrivingToastRoot | x 330-396, y 48-58 | Yes |
| 82 | CareCycleStrip | AllocatePanelRoot | x 380-900, y 629-660 | Yes |
| 83 | ClimateStateRoot | SatchelRoot | x 16-316, y 589-624 | Yes |
| 84 | ClimateStateRoot | AllocatePanelRoot | x 380-436, y 589-624 | Yes |
| 85 | SovereignBannerRoot | ThrivingToastRoot | x 380-900, y 52-88 | Yes |
| 86 | SovereignBannerRoot | MercySoftRoot | x 894-900, y 80-91 | Yes |
| 87 | ThrivingToastRoot | MercySoftRoot | x 894-950, y 80-88 | Yes |
| 88 | ThrivingToastRoot | ClimateNameRoot | x 500-780, y 48-49 | Yes |
| 89 | RealmSoftRoot | JourneyEchoRoot | x 26-326, y 144-314 | Yes |

Pairs added by Model B only:

| # | slab 1 | slab 2 | overlapping region (B heights) | visible together |
|---|--------|--------|--------------------------------|------------------|
| 1 | ComfortGraphicsBannerRoot | SpillSlabRoot | x 380-536, y 52-59 | Yes |
| 2 | ComfortGraphicsBannerRoot | SovereignBannerRoot | x 380-900, y 52-59 | Yes |
| 3 | ComfortGraphicsBannerRoot | ThrivingToastRoot | x 380-900, y 48-59 | Yes |
| 4 | FactorySlabRoot | SpillSlabRoot | x 380-536, y 52-68 | Yes |
| 5 | FactorySlabRoot | SovereignBannerRoot | x 380-900, y 52-68 | Yes |
| 6 | VoiceSlabRoot | WarSlabRoot | x 844-920, y 696-712 | Yes |
| 7 | VoiceSlabRoot | WellSlabRoot | x 360-436, y 660-668 | Yes |
| 8 | VoiceSlabRoot | WorldCarePromptRoot | x 410-870, y 660-672 | Yes |
| 9 | VoiceSlabRoot | CareCycleStrip | x 360-920, y 660-672 | Yes |
| 10 | SpillSlabRoot | FabSlabRoot | x 380-536, y 88-104 | Yes |
| 11 | SpillSlabRoot | JourneyEchoRoot | x 26-386, y 96-104 | Yes |
| 12 | LedgerSlabRoot | WellSlabRoot | x 16-436, y 665-668 | Yes |
| 13 | LedgerSlabRoot | WorldCarePromptRoot | x 410-576, y 665-672 | Yes |
| 14 | LedgerSlabRoot | CareCycleStrip | x 360-576, y 665-672 | Yes |
| 15 | FabSlabRoot | EmbassySlabRoot | x 844-900, y 124-140 | Yes |
| 16 | FabSlabRoot | ThrivingToastRoot | x 380-900, y 88-106 | Yes |
| 17 | EmbassySlabRoot | CrownstoneSlabRoot | x 844-1264, y 164-176 | Yes |
| 18 | WarSlabRoot | CompassSlabRoot | x 844-1264, y 696-708 | Yes |
| 19 | CrownstoneSlabRoot | RedemptionSlabRoot | x 844-1264, y 204-216 | Yes |
| 20 | CrownstoneSlabRoot | HarvestPulseRoot | x 844-920, y 164-179 | Yes |
| 21 | RedemptionSlabRoot | HybridSlabRoot | x 844-1264, y 244-256 | Yes |
| 22 | CompassSlabRoot | WorldCarePromptRoot | x 844-870, y 656-672 | Yes |
| 23 | CompassSlabRoot | CareCycleStrip | x 844-920, y 656-672 | Yes |
| 24 | CompassSlabRoot | AllocatePanelRoot | x 844-900, y 656-660 | Yes |
| 25 | WellSlabRoot | FirstSessionGuidanceStrip | x 380-436, y 659-668 | Yes |
| 26 | WellSlabRoot | ClimateStateRoot | x 16-436, y 616-624 | Yes |
| 27 | WellSlabRoot | SatchelRoot | x 16-316, y 616-624 | Yes |
| 28 | WellSlabRoot | LivingPracticeStrip | x 320-436, y 664-668 | Yes |
| 29 | FirstSessionGuidanceStrip | WorldCarePromptRoot | x 410-870, y 659-672 | Yes |
| 30 | FirstSessionGuidanceStrip | CareCycleStrip | x 380-900, y 659-672 | Yes |
| 31 | FirstSessionGuidanceStrip | AllocatePanelRoot | x 380-900, y 659-660 | Yes |
| 32 | WorldCarePromptRoot | ClimateStateRoot | x 410-436, y 616-624 | Yes |
| 33 | WorldCarePromptRoot | LivingPracticeStrip | x 410-870, y 664-672 | Yes |
| 34 | CareCycleStrip | ClimateStateRoot | x 360-436, y 610-624 | Yes |
| 35 | CareCycleStrip | LivingPracticeStrip | x 360-920, y 664-672 | Yes |
| 36 | SovereignBannerRoot | JourneyEchoRoot | x 380-386, y 96-108 | Yes |
| 37 | ThrivingToastRoot | JourneyEchoRoot | x 330-386, y 96-106 | Yes |

### 4.3 Ledger against the bottom column

The ledger plate (`ledger_bind.rs` L98-L101) is `left 16`, `width 560`, `bottom 16`, so it spans x 16-576. Vertical overlap with a slab anchored `bottom b` starts when the ledger height exceeds `b - 16` px. Ledger height is 102 px (A) / 119 px (B).

| slab | x range at 1024 | x overlap with ledger at 1024 | x range at 1280 | x overlap with ledger at 1280 | ledger height needed for vertical overlap |
|------|-----------------|-------------------------------|-----------------|-------------------------------|-------------------------------------------|
| VoiceSlabRoot (bottom 88) | 232-792 | 232-576 | 360-920 | 360-576 | > 72 px |
| WarSlabRoot (bottom 52) | 588-1008 | none | 844-1264 | none | > 36 px |
| CompassSlabRoot (bottom 92) | 588-1008 | none | 844-1264 | none | > 76 px |
| WellSlabRoot (bottom 132) | 16-436 | 16-436 | 16-436 | 16-436 | > 116 px |
| TouchStickZone (bottom 24) | 24-144 | 24-144 | 24-144 | 24-144 | > 8 px |
| TouchUseBtn (bottom 36) | 952-996 | none | 1208-1252 | none | > 20 px |
| FirstSessionGuidanceStrip (bottom 72) | 252-772 | 252-576 | 380-900 | 380-576 | > 56 px |
| WorldCarePromptRoot (bottom 128) | 282-742 | 282-576 | 410-870 | 410-576 | > 112 px |
| CareCycleStrip (bottom 128) | 232-792 | 232-576 | 360-920 | 360-576 | > 112 px |
| ClimateStateRoot (bottom 176) | 16-436 | 16-436 | 16-436 | 16-436 | > 160 px |
| WatchStripRoot (bottom 16) | 16-356 | 16-356 | 16-356 | 16-356 | > 0 px |
| SatchelRoot (bottom 22%) | 16-316 | 16-316 | 16-316 | 16-316 | > 124.8 px at 1024x640, > 160 px at 1280x800 (bottom 22%) |
| LivingPracticeStrip (bottom 72) | 192-832 | 192-576 | 320-960 | 320-576 | > 56 px |
| AllocatePanelRoot (bottom 140) | 252-772 | 252-576 | 380-900 | 380-576 | > 124 px |
| PeerPresenceRoot (bottom 16) | 728-1008 | none | 984-1264 | none | > 0 px |

### 4.4 Stacked columns: spacing against height

Offsets between neighbouring slabs in one column, and the height at which they touch:

| column | slabs (offset) | gap between anchors | overlap starts when the upper slab is taller than |
|--------|----------------|---------------------|---------------------------------------------------|
| right, top-anchored | embassy 124 / crownstone 164 / redemption 204 / hybrid 244 | 40 px each | 40 px (A: 35 px, B: 52 px) |
| right, bottom-anchored | war 52 / compass 92 | 40 px | 40 px (A: 35, B: 52) |
| right, bottom-anchored | peer chip 16 / war 52 | 36 px | 36 px (chip A: 37, B: 52) |
| centre top | thriving toast 48 / sovereign banner 52 | 4 px | 4 px |
| centre top | factory 16 / toast 48 / sovereign 52 / fab 88 / pulse 118 | 32 / 4 / 36 / 30 px | gap to the next slab below |
| centre bottom | guidance 72 / practice 72 | 0 | same anchor and width class; mutually exclusive in code |
| centre bottom | voice 88 / guidance 72 / practice 72 | 16 px | 16 px (guidance A: 48, practice A: 46, voice A: 35) |
| centre bottom | care strip 128 / world-care prompt 128 | 0 | same anchor, widths 560 and 460 |
| centre bottom | allocate 140 / care strip and prompt 128 | 12 px | 12 px |
| left bottom | ledger 16 / watch strip 16 | 0 | same corner, widths 560 and 340 |
| left bottom | well 132 / climate state 176 | 44 px | 44 px (A: 35, B: 52) |

### 4.5 Full-viewport layers

These nodes are `width 100%`, `height 100%` with no offsets, so each one covers every slab above at every window size.

| node | z | shown when |
|------|---|-----------|
| TitleRoot (`title_screen.rs:1331`) | Global 120 (LIVED_UI_Z_TITLE) | `sync_title_visibility`: shown when `LaunchDoor::Title` |
| NameHouseRoot (`title_screen.rs:1813`) | Global 140 | `sync_name_house_visibility`: `LaunchDoor::NameHouse` |
| HouseDressRoot (`title_screen.rs:1884`) | Global 141 | `sync_house_dress_visibility`: `LaunchDoor::HouseDress` |
| PersonaCreatorRoot (`title_screen.rs:2045`) | Global 142 | `sync_persona_creator_visibility`: `persona.open && PERSONA_CREATOR_ENABLED` (const is true) |
| TouchOverlayRoot (`touch_controls.rs:69`) | Global 124 (LEDGER - 1) | `sync_touch_overlay_visibility`: not culled (title / pause / ledger / inventory / Q-hint cull it) and on-screen sticks resolve true (setting or last pointer = Touch) |

- Each of the five layers overlaps the other 40 rows in section 2 (all rows except the five layers and the test probe; 40 x 5 = 200 pairs at any resolution), and the five layers overlap each other (10 pairs).
- Title, NameHouse and HouseDress are driven by the single `LaunchDoor` enum, so they are never visible together. Persona (`persona.open`) is independent of `LaunchDoor`: not stated. The touch overlay is culled on `LaunchDoor::Title / NameHouse / HouseDress`, pause settings, Places, ledger, inventory and Q-hint.
- Stacking: Title 120, touch overlay 124, ledger and satchel 125, Settings 130, Comfort banner 131, Places 132, NameHouse 140, HouseDress 141, Persona 142. Nodes with no `ZIndex` draw under every `Global` node.

## 5. Counts

| resolution | Model A pairs | Model B extra pairs | Model B total |
|------------|---------------|---------------------|---------------|
| 1024x640 | 130 | 41 | 171 |
| 1280x800 | 89 | 37 | 126 |

Not determined: actual wrapped line counts (no render, string lengths only for a few slabs); runtime `text_scale` effects; exact show gates for `climate_visible.rs` (several speech sources) and `vertical_factory.rs` door-ready state; whether Persona can open on the Title door.
