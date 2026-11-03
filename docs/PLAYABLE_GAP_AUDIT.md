# Playable gap audit

**Card:** PLAYABLE-GAP-AUDIT-1
**Base:** `fe4bc9b1` (main, CARD OFFLINE-BUILD-4 #708)
**Kind:** docs only. This file is the only change.

Evidence is a file:line opened on this stamp. A figure that is not on that line is unmeasured. The last column of the gap table is a suggested name. Nothing in this file is queued, and nothing is marked spent.

## 1. Human — Title to the end of the offline hour

The player process is `client/src/main.rs`. It opens a window titled `Powrush-MMO — first hour` (`client/src/main.rs:47-49`) and adds `PowrushClientBundle` (`client/src/main.rs:67-69`). The bundle plugin list is `client/src/lib.rs:142-212`. The library modules are `client/src/lib.rs:8-79`. `client/src/gltf_integration.rs` is on that graph through `#[path]` at `client/src/title_screen.rs:168-169`, `client/src/human_presence.rs:68-70`, and `client/src/climate_plane.rs:133-135`.

The loading overlay spawns on startup unless `POWRUSH_Q2_FRAME` is set (`client/src/loading_screen.rs:43-48`, `client/src/loading_screen.rs:236`).

The Play, Continue, and Settings strings are `client/src/title_screen.rs:223-225`. The title plate spawns those three buttons (`client/src/title_screen.rs:1441-1443`). The hint under them reads `1 Play · 2 Continue · 3 Settings · Esc from yard opens pause` (`client/src/title_screen.rs:1452`). Digit1 or Enter runs Play (`client/src/title_screen.rs:2394-2400`). Digit2 runs Continue when `persist_present` is set (`client/src/title_screen.rs:2401-2416`). Digit3 toggles Settings (`client/src/title_screen.rs:2417-2418`). Those clicks are `title_button_clicks` (`client/src/title_screen.rs:2308`, `client/src/title_screen.rs:2327-2361`), registered on `TitleScreenPlugin` (`client/src/title_screen.rs:1276`). Naming, dress, and pause systems on that plugin are `client/src/title_screen.rs:1279-1287` and `client/src/title_screen.rs:1345`.

Play calls `s3_play_boot` (`client/src/title_screen.rs:298-306`), which calls `apply_title_boot` with `BootKind::Play` (`client/src/hex_travel.rs:230-240`). That boot place is Sanctuary (`shared/hex_travel.rs:441-443`). The door becomes `InYard` (`client/src/title_screen.rs:2303-2306`). Continue calls `s3_continue_boot` when `persist_present` is set (`client/src/title_screen.rs:2340-2353`). If the hour-two roster's first entry has a place, that call applies the place (`client/src/title_screen.rs:321-323`). Otherwise it calls `apply_title_boot` with `BootKind::Continue` (`client/src/title_screen.rs:325`). `boot_place` for Continue is Sanctuary when the book flag is absent (`shared/hex_travel.rs:444-446`).

In the yard, `handle_player_input` copies the keyboard and the first gamepad into `PlayerInput` (`client/src/input.rs:324-372`, `client/src/input.rs:387-426`). Move keys are W A S D and the arrows (`client/src/input.rs:336-348`). The key constants are `client/src/soft_play_bindings.rs:11-18` and `client/src/soft_play_bindings.rs:27-37` (Space jump, E interact, Shift sprint, I satchel, H hide, R allocate, Tab chart, L ledger, Q build). `apply_locomotion` moves `SoftPresence` from that vector on the fixed step (`client/src/human_presence.rs:548`, `client/src/human_presence.rs:759-806`). `WALK` is `3.4` (`client/src/human_presence.rs:75`). Sprint uses `SPRINT` `5.4` when `input.sprint` is set (`client/src/human_presence.rs:76`, `client/src/human_presence.rs:769-772`). A grounded jump sets vertical velocity to `JUMP` `5.6` (`client/src/human_presence.rs:78`, `client/src/human_presence.rs:797-798`). Mouse and touch update `LastPointerKind` (`client/src/input.rs:271-285`). The touch overlay starts hidden (`client/src/touch_controls.rs:76`). `sync_touch_overlay_visibility` shows it from `overlay_sticks_visible` (`client/src/touch_controls.rs:252`, `client/src/input.rs:483-485`).

The guidance card starts on `MoveAround` (`client/src/first_session_guidance.rs:471`). The prompt strings are `client/src/first_session_guidance.rs:382-400`. Tracking is registered at `client/src/first_session_guidance.rs:782-783`. Advance rules are `client/src/first_session_guidance.rs:657-684`.

Holding a move key adds `delta_seconds * 6.0` to the card meter (`client/src/first_session_guidance.rs:1032-1042`). The card leaves `MoveAround` when that meter passes `4.0` (`client/src/first_session_guidance.rs:662`). `near_glow` follows `NearbyMercyNode.in_range`, and the approach step also passes when the meter passes `12.0` (`client/src/first_session_guidance.rs:663`, `client/src/first_session_guidance.rs:1049-1050`).

E is the Use key (`client/src/soft_play_bindings.rs:12`). Holding it for `TEND_HOLD` `0.42` while a node is in range runs tend (`client/src/first_harvest_epiphany.rs:49`, `client/src/first_harvest_epiphany.rs:525-526`, `client/src/first_harvest_epiphany.rs:591-598`, `client/src/first_harvest_epiphany.rs:684-726`). Releasing before that runs take (`client/src/first_harvest_epiphany.rs:601-608`). The system is registered at `client/src/first_harvest_epiphany.rs:298`. A buffered Use sets `PlayerInput.interact` (`client/src/feel_move.rs:48-52`). That flag takes on the same frame (`client/src/first_harvest_epiphany.rs:571-577`).

A raised take or tend count is written into the lived hour by `sync_lived_hour_from_peace_use` (`client/src/mercy_harvest_nodes.rs:306`, `client/src/mercy_harvest_nodes.rs:691-731`). That calls `sync_lived_hour_use` (`client/src/lived_sim_bridge.rs:123-133`), which calls `tend_nearest` (`client/src/lived_hour_bind.rs:327-329`) and then `persist` (`client/src/lived_hour_bind.rs:313`). `focus_id` is the nearby node's `climate_id` (`client/src/climate_visible.rs:173-184`). The harvest module states that id maps onto the lived-hour node ids (`client/src/mercy_harvest_nodes.rs:5`).

I toggles the satchel (`client/src/human_inventory.rs:251-259`). R toggles allocate when surplus is already noted (`client/src/rbe_allocate_choice.rs:396-407`). Digit1 commits flow and Digit2 commits reserve (`client/src/rbe_allocate_choice.rs:506-524`). `LivedHourBind::allocate` persists (`client/src/lived_hour_bind.rs:335-357`). Tab after that allocate calls `try_ridge_tab` (`client/src/hour_sacred.rs:999-1013`, `client/src/hour_sacred.rs:942-950`). Q plants a house only once the hex is off Peace (`client/src/vertical_factory.rs:111-124`, `client/src/hour_sacred.rs:953-956`). The same Q key also reaches `handle_fab_q`, which calls `try_craft_proof_pack` (`client/src/fabricator.rs:210-228`). L toggles the ledger only off Peace (`client/src/ledger_bind.rs:140-145`, `client/src/ledger_bind.rs:148-161`). E calls `try_request_embassy_seat` (`client/src/embassy.rs:184-189`). Peace, or an hour that is not complete, returns without a seat (`client/src/embassy.rs:153-155`). H dismisses the card (`client/src/first_session_guidance.rs:1004-1013`). Those yard systems are registered at `client/src/human_inventory.rs:102`, `client/src/rbe_allocate_choice.rs:213-216`, `client/src/hour_sacred.rs:934`, `client/src/vertical_factory.rs:69`, `client/src/fabricator.rs:67`, `client/src/ledger_bind.rs:87`, and `client/src/embassy.rs:58`.

`HourTwoHeld` and `HourThreeHeld` stay put inside `advance_if_ready` (`client/src/first_session_guidance.rs:671`, `client/src/first_session_guidance.rs:674`). After `free_since` passes `6.0` the card moves on (`client/src/first_session_guidance.rs:1113-1118`). The `FreeExploration` prompt is `E tend the well · harm stays optional` (`client/src/first_session_guidance.rs:399`). After `free_since` passes `8.0` the card calls `dismiss` (`client/src/first_session_guidance.rs:1119-1121`). `dismiss` sets `dismissed` and clears `active` (`client/src/first_session_guidance.rs:495-497`). The door stays `InYard`. That dismiss is the end of the guidance card.

When `hour.complete` is true in the yard, the door becomes `NameHouse` and the house file is written (`client/src/title_screen.rs:2499-2530`). The draft takes the characters at `client/src/title_screen.rs:3898-3906`. Enter confirms and Escape skips (`client/src/title_screen.rs:3937-3938`). The hour pack writes when its resources change (`client/src/hour_sacred.rs:1030-1062`).

Esc toggles the pause plate (`client/src/title_screen.rs:2560-2578`). The plate line is `the yard is waiting` (`client/src/title_screen.rs:220`). Resume, Title, and Quit are `client/src/title_screen.rs:1840-1842`. The same Esc press persists the lived hour (`client/src/shard_climate.rs:42`, `client/src/shard_climate.rs:158-161`). Title on that plate calls `return_yard_to_title` (`client/src/title_screen.rs:2667`), which writes the house file and calls `bind.persist` (`client/src/title_screen.rs:2684-2697`). Quit sends `AppExit` (`client/src/title_screen.rs:2671-2678`). Digit3 opens that same pause from the yard (`client/src/title_screen.rs:2428-2429`).

The Places row is shown when settled and the book are both true, the pause plate is open, and the door is `InYard` (`client/src/hex_travel.rs:560-571`, `shared/hex_travel.rs:387-389`).

`LivedHourBind::persist` writes climate, standing, week, and the tick (`client/src/lived_hour_bind.rs:174-180`, `client/src/lived_hour_bind.rs:193-232`). The path constants are `client/src/lived_hour_bind.rs:33-36`. When `POWRUSH_INGEST` is on, the tick write uses the ingest helper (`client/src/lived_hour_bind.rs:215`, `shared/lived_tick_ingest.rs:25-31`). The local session file is a second write, on Update, when the pool, realm, harvest, web, or bond changed (`client/src/local_session_persist.rs:20`, `client/src/local_session_persist.rs:125-128`, `client/src/local_session_persist.rs:174-188`).

## 2. AI — what a script can drive

These are the environment reads on the player process. `#[cfg(test)]` reads are omitted. Reads inside Gap 1 are omitted here and named there.

- `POWRUSH_Q2_FRAME` is read at `client/src/loading_screen.rs:47-48`. When it is set, the loading overlay returns before it spawns (`client/src/loading_screen.rs:236`).
- `POWRUSH_NET` is read by `parse_powrush_net` (`shared/hex_listen.rs:132-133`), used for the default session (`client/src/net_mode.rs:44-46`), and read again in `sync_lan_door_from_settings` (`client/src/net_mode.rs:112-115`). `scripts/play-offline.sh:12-13` and `.cursor/run-client-headless.sh:19-21` set the variable to `off`.
- `POWRUSH_USER_DIR` is the override name (`shared/user_persist.rs:28`) and the read (`shared/user_persist.rs:94-98`). Client saves go through `persist_path`, including `client/src/local_session_persist.rs:74-75`.
- `XDG_DATA_HOME` and `HOME` are the Linux directory reads when that override is empty (`shared/user_persist.rs:131-141`).
- `POWRUSH_GEN` is read at `shared/powrush_gen.rs:49-50`. `light_gen_enabled` calls that read, and `light_gen_enabled_with` calls `light_gen_enabled` (`shared/powrush_gen.rs:54-55`, `shared/powrush_gen.rs:66-67`). `LightGenDoor::default` calls `from_settings_grove` (`client/src/light_gen.rs:65-66`), which calls `light_gen_enabled_with` (`client/src/light_gen.rs:75-76`).
- `POWRUSH_INGEST` is read at `shared/lived_tick_ingest.rs:25-31` and consulted from `client/src/lived_hour_bind.rs:215`.
- `POWRUSH_AUDIO` is the name at `shared/peace_audio.rs:44` and the read at `shared/peace_audio.rs:242-250`. Callers in the player graph are `client/src/peace_audio.rs:73`, `client/src/main.rs:60`, and `client/src/mercy_harvest_nodes.rs:753`.

`scripts/play-offline.sh:4` defines `q1` / `--script-run` as a machine six-landing pass (no interactive walk). The branch is `scripts/play-offline.sh:101-103`. The function runs `cargo test -p shared -p rsil-identity` and `cargo test -p powrush-client --lib` (`scripts/play-offline.sh:50-64`), greps the lib log (`scripts/play-offline.sh:66-72`), prints the door line at `scripts/play-offline.sh:83` (`no cargo run` and `no WASD`), and exits. That mode is a test pass. It is not a scripted walk. Any other first argument falls through to `exec cargo run -p powrush-client` (`scripts/play-offline.sh:105-107`).

Headless boot is `.cursor/run-client-headless.sh`. The usage comment is `.cursor/run-client-headless.sh:7-12`. The script exports `POWRUSH_NET=off` (`.cursor/run-client-headless.sh:19-21`), `DISPLAY` from `POWRUSH_DISPLAY` (`.cursor/run-client-headless.sh:23`, `.cursor/run-client-headless.sh:36`), `VK_ICD_FILENAMES` (`.cursor/run-client-headless.sh:38-47`), and `WGPU_BACKEND` (`.cursor/run-client-headless.sh:54`). Those display variables are shell exports. No `env::var` or `env::var_os` call in `client/src` or `shared` reads `WGPU_BACKEND`, `VK_ICD_FILENAMES`, `POWRUSH_DISPLAY`, or `DISPLAY`. With `POWRUSH_Q2_FRAME` empty, the script execs `cargo run -p powrush-client` (`.cursor/run-client-headless.sh:59-61`). With the variable set, it starts that binary (`.cursor/run-client-headless.sh:66`), waits until a window title contains `Powrush` (`.cursor/run-client-headless.sh:98`), grabs one PNG (`.cursor/run-client-headless.sh:134`), and exits 0 (`.cursor/run-client-headless.sh:150-151`). The trap at `.cursor/run-client-headless.sh:69-75` kills the client. `main` always builds a window and has no script argument (`client/src/main.rs:34-72`).

A script cannot walk, tend, and save one full hour with no human. Input in the bundle is the keyboard, the gamepad, and the pointer kind above. There is no action file and no environment variable that holds W A S D or E. `--script-run` never starts the client. The Q2 headless path starts the client, writes one PNG, and the shell exits. Tend, allocate, and `LivedHourBind::persist` wait on the hands in section 1. `dismiss` writes no file. Pause Quit sends `AppExit` and does not call `persist`.

## Gap 1 (not compiled, cite-only)

`client/src/lib.rs:8-79` names the library modules. `client/src/main.rs` is the binary. `client/src/gltf_integration.rs` is the `#[path]` file in section 1. Every other Rust file under `client/src` is outside that graph. Those files are not features of `PowrushClientBundle`. `client/src/localization.rs:140` reads `LANG`; the file is in the list below, so the player process does not run that read.

```text
client/src/ability_bar.rs
client/src/ambisonics_engine.rs
client/src/anisotropic_filtering.rs
client/src/app.rs
client/src/behavioral_tracking.rs
client/src/bevy_ecs_scheduling.rs
client/src/binaural_ambisonics_decoder.rs
client/src/chromatic_aberration.rs
client/src/config.rs
client/src/council_bloom_feedback.rs
client/src/council_trial_ui.rs
client/src/council_ui.rs
client/src/debug/testing_harness.rs
client/src/delta_compression.rs
client/src/divine_whispers.rs
client/src/dynamic_events_ui.rs
client/src/dynamic_music.rs
client/src/egui_settings_panel.rs
client/src/epiphany_scenario_wiring.rs
client/src/example_gpu_material.rs
client/src/faction.rs
client/src/faction_diplomacy_ui.rs
client/src/faction_reputation_ui.rs
client/src/fmod_audio.rs
client/src/fracture.rs
client/src/fundsp_audio.rs
client/src/gpu/infrastructure_culling.rs
client/src/gpu/mod.rs
client/src/gpu/staging_buffer.rs
client/src/gpu/visual_materials.rs
client/src/gpu_simulation/mod.rs
client/src/gpu_simulation/resources.rs
client/src/gpu_simulation/state.rs
client/src/gpu_simulation/sync.rs
client/src/higher_order_ambisonics.rs
client/src/inventory_replication.rs
client/src/inventory_ui.rs
client/src/local_player.rs
client/src/localization.rs
client/src/motion_blur.rs
client/src/multiplayer_web_deepening.rs
client/src/my_mercy_journey_panel.rs
client/src/networking.rs
client/src/oddio_backend.rs
client/src/onboarding.rs
client/src/onboarding_chronicle.rs
client/src/onboarding_ui.rs
client/src/particles.rs
client/src/player_progress_ui.rs
client/src/prediction.rs
client/src/rbe.rs
client/src/rbe_client_sync.rs
client/src/rbe_client_ui_sync.rs
client/src/rbe_education_ui.rs
client/src/rbe_engine.rs
client/src/rbe_simulation.rs
client/src/rbe_ui_feedback.rs
client/src/realm_travel_panel.rs
client/src/render.rs
client/src/replication.rs
client/src/server_message_dispatcher.rs
client/src/settings.rs
client/src/shadow_render_node.rs
client/src/ships/mod.rs
client/src/simulation_integration.rs
client/src/spatial_audio.rs
client/src/spectator_legacy_thread_viz.rs
client/src/ssr_render_node.rs
client/src/steam_integration.rs
client/src/steamworks_integration.rs
client/src/systems.rs
client/src/taa_compute_node.rs
client/src/taa_reprojection.rs
client/src/taa_resources.rs
client/src/treaty_negotiation_ui.rs
client/src/ui.rs
client/src/ui_utils.rs
client/src/velocity_prepass.rs
client/src/visual/development_resonance.rs
client/src/visual/infrastructure_resonance.rs
client/src/webxr_bootstrap.rs
client/src/world.rs
client/src/world_simulation/data_collection.rs
client/src/world_simulation/mirror_score.rs
client/src/world_simulation/mod.rs
```

### ability_bar.rs audit (ABILITY-BAR-AUDIT-1)

Facts at `6abac1c4`, cite-only. Nothing in this subsection is a decision.

- **Not compiled:** `client/src/lib.rs` declares no `ability_bar` module, and the file is listed above.
- **Old Bevy API:** the client pins `bevy = "0.20"` (`client/Cargo.toml:26`). The file uses `NodeBundle` (`client/src/ability_bar.rs:55`, `:74`, `:111`), `TextBundle` with `Text::from_section` and `TextStyle` (`:95-96`), `add_event` (`:42`), `EventReader` (`:183`) and `time.delta_seconds()` (`:133`).
- **Missing font:** `:97` loads `fonts/Inter-Bold.ttf`. `client/assets/fonts` holds only FiraSans-Regular, FiraSans-Bold and OFL.txt. The live UI font is `UI_FONT_REGULAR` (`client/src/ui_font.rs:17`).
- **Stub input:** `handle_ability_input` (`:156-180`) reads Digit1–4 (`:162-165`), prints a line (`:173`) and sets a local cooldown (`:174`). It sends nothing.
- **No producer:** `AbilityCooldownUpdate` (`:30`) is read at `:182-183`, and no compiled client code sends it. The server has its own type (`server/src/combat/mod.rs:141`).
- **Double bind:** `client/src/input.rs:380-390` already maps Digit1–4 to `PlayerInput.ability_slot` (`client/src/input.rs:31`).
- **Placement:** absolute, bottom 20, left 50% (`:57-59`), outside the live HUD registry.

**Overlap with the live seams**

- **Home:** `ID_ABILITY_TIP` (`client/src/hud_anchor_registry.rs:133`, id at `:100`) is the live data-only slot for ABILITY-TOOLTIP-SHOW-1. The file's own bar root (`client/src/ability_bar.rs:53-70`) duplicates that role.
- **Text:** `slot_tooltip` (`client/src/combat_core.rs:201`) is the live slot text, and only slot 0 has a kit (`slot_strikes`, `:195`). The file shows only a slot number (`client/src/ability_bar.rs:96`).
- **Only in this file:** a per-slot cooldown model and overlay fill (`client/src/ability_bar.rs:9-14`, `:128`, `:142-154`). Live combat has no cooldown, and the tooltip test refuses the word "cooldown" (`client/src/combat_core.rs:739`). Porting that overlay under option (a) would add a mechanic, not a display, so it needs its own card and ruling. It is named here as a gap, not proposed.
- **Colours:** its palette (`client/src/ability_bar.rs:66`, `:82`, `:99`) is not the title palette, and no ratio has been measured. Any ratio would use `hud_contrast_ratio` (`client/src/hud_presets.rs:702`) with a named pair.

**Options (listed neutrally; none is chosen)**

- (a) Port the useful parts into ABILITY-TOOLTIP-SHOW-1 on the live seams, rewritten for Bevy 0.20.
- (b) Keep the file as reference, uncompiled, as today.
- (c) Retire the file. Deleting a file needs Sherif's own yes.

Out of scope: `server/src/ascension/ui/ability_bars.rs` is a separate server module (`server/src/ascension/mod.rs:22`).

## Combat presence audit (COMBAT-PRESENCE-AUDIT-1)

Facts at `10d71e6d`, cite-only, from Sherif's playtester's report that no combat was recognised. Nothing in this section is a fix.

**Headline: on default Play, combat is not reachable before hour three is complete.**

**Path to the first enemy**
- **Boot:** Play always lands in Sanctuary (`shared/hex_travel.rs:440-443`), and so does Continue without the book (`:444-446`). The client passes `book = hour.hour_three_complete` (`client/src/hex_travel.rs:239-242`).
- **Enemies live in Heartwood only:** `predator_present` is `place == PlaceId::Heartwood` (`client/src/combat_core.rs:213-215`), and `in_reach` needs it (`:217-220`). The droid sits at the Threshold shelf, `DROID_HOME` [0, 13] (`client/src/droid.rs:12-13`), and is gated by the same `predator_present` (`droid.rs:41`, `:135`).
- **The door is gated:** the Places list is live only when settled and the book is held (`shared/hex_travel.rs:387-390`, enforced at `client/src/hex_travel.rs:647-651`). `apply_place` sends Heartwood back to Sanctuary without the book (`client/src/hex_travel.rs:254-257`). The book is the fabricator pack unlocked and the embassy seated (`shared/hour_two.rs:71`).
- **The People-door route has no live caller found:** the landing chain (`try_cross_people_door_land` `client/src/hex_travel.rs:294`, `try_unsealed_light_people_door_land` `:326`, `garden_cross_landing` `client/src/title_screen.rs:426`, `garden_cross_people_land` `client/src/first_session_guidance.rs:574`, `:767`) is reached only from test code (`hex_travel.rs:1571`, `:2010`, `:2060` after `#[cfg(test)]` `:807`; `title_screen.rs:5520` after `#[cfg(test)]` `:5256`; `first_session_guidance.rs:1639` after `:1208`).
- **No signpost found:** the only Heartwood want line is `HEARTWOOD_WANT` "Heartwood Wards · Tend" (`client/src/first_session_guidance.rs:163`), which names Heartwood once you are there. No Sanctuary line pointing at Heartwood or its enemies was found by grep. Time to first enemy is not measured.

**Count and spawn**
- Two client-local enemies: one predator, `[NAME TBD]` with 200 health (`client/src/combat_core.rs:34-35`), and one droid, `[DROID NAME TBD]` (`client/src/droid.rs:20`), never saved (`droid.rs:2`).
- The predator is a capsule primitive spawned hidden at `PREDATOR_HOME` [-18, -18] (`combat_core.rs:42`, `:373-379`). It shows only while present and alive (`:539-541`), and respawns after 30 s (`:49`).

**What a fight looks like**
- **Telegraphs:** the lunge is 0.6 s and hits 15 (`combat_core.rs:37-38`), and the droid shot has a 0.5 s tell and hits 25 (`droid.rs:16-19`). The tell is a ring of hidden primitive dots (`combat_core.rs:593-599`) with the text "Brace, it is about to lunge" (`:564`) or "Look out, the droid is about to fire" (`droid.rs:22`). The text shows only on Sworn and Unbroken, which hide the ring (`combat_core.rs:589-590`, `client/src/difficulty.rs:36-37`).
- **Hit feedback (visual):** a landed hit gives a 0.10 s colour flash and a 12% scale pulse on the predator (`combat_core.rs:134`, `:159-166`, `:635-641`), a camera kick of 0.35 on a hit and 0.6 on the kill (`:135-137`, `:624`), and a gamepad rumble (`:625`).
- **Hit feedback (text):** the pulse line reads "`[NAME TBD]` 185 / 200 · +10 combat XP" or "Missed" or "is down" (`combat_core.rs:318-328`), at 10 XP per landed hit (`:40`). The droid pays 0 XP (`droid.rs:2`). Sworn and Unbroken drop the HP number (`difficulty.rs:47-50`).
- **Respawn and clear:** the respawn tell is a hidden pillar at `PREDATOR_HOME` (`combat_core.rs:405-421`), and a steady amber disc marks the cleared state (`:423-426`).
- **Defeat:** the player is reseated with full health (`combat_core.rs:545-546`, `:309-315`) and the pulse reads "You wake at the Heartwood entrance" (`:341-343`, `:549`).

**What a fight sounds like**
- Three audio files are tracked: `assets/audio/peace_yard_bed.ogg`, `assets/audio/peace_well_sting.ogg` and `assets/music/epic_dark_ambient.ogg`. The music file is not wired (`client/assets/audio/README.md:9`).
- The only combat sound is `play_tell` (`combat_core.rs:556-562`), which plays the peace `STING_ASSET` (`shared/peace_audio.rs:30`) on a predator tell (`combat_core.rs:587-588`) and a droid tell (`droid.rs:149-150`).
- There is no strike, hit, defeat or respawn sound. The other live `AudioPlayer` spawns are the peace bed and sting (`client/src/peace_audio.rs:158`, `:192`, `:210`) and the harvest sting (`client/src/mercy_harvest_nodes.rs:804`). `client/src/rbe_ui_feedback.rs:74` and `:96` are in a file that `client/src/lib.rs` does not declare.

**Gaps (suggested card names only; Core ranks them)**

| ID | Gap | Evidence | Suggested card |
|----|-----|----------|----------------|
| CP-01 | Combat is not reachable on default Play before hour three plus a Places pick. | `shared/hex_travel.rs:443`; `shared/hex_travel.rs:388-390`; `shared/hour_two.rs:71` | COMBAT-REACH-1 |
| CP-02 | Nothing in Sanctuary points toward Heartwood or its enemies. | `client/src/first_session_guidance.rs:163` | COMBAT-SIGNPOST-1 |
| CP-03 | The only combat sound is the reused peace sting on a tell; there is no strike, hit or defeat sound. | `client/src/combat_core.rs:556-562`; `client/src/droid.rs:150` | COMBAT-FEEL-SFX-1 |
| CP-04 | Both enemies are unnamed primitives (a capsule and dots). The player now feels being hit (camera kick, gamepad rumble and a lower thud, once per landed lunge), and the creature leans in over the existing wind-up (COMBAT-FEEL-LOOK-1). The name and model stay open for the councils and Sherif. | `client/src/combat_core.rs:40`; `client/src/combat_core.rs:399-411`; `client/src/droid.rs:20` | COMBAT-FEEL-LOOK-1 |
| CP-05 | The lunge ring blinked and the tell sting chimed on every 0.6 s loop while in reach, so the tell read as flicker. Now the ring stays steady while in reach, the sting and the Sworn/Unbroken text play once per reach entry and re-arm when the player leaves reach, and the lean still marks each lunge. | `client/src/combat_core.rs`, `lunge_tell`, `tell_arm` | TELL-CADENCE-1 |
| CP-06 | The player's own health showed nowhere. Now a hurt line "You {left} / 200" shows on the shared Pulse line when the slot is free (idle or already a hurt line), never on Sworn or Unbroken, and the wake line still wins on defeat. | `client/src/combat_core.rs`, `hurt_line`, `hurt_pulse`, `predator_lunge` | HURT-READOUT-1 |
| CP-07 | The droid's tell sting chimed on every 1.25 s shot cycle while in reach. Now it plays once per reach entry and re-arms when the player leaves reach, the droid goes down or the player is defeated; the ground ring still shows each 0.5 s shot tell. | `client/src/droid.rs`, `droid_tick`, `tell_arm` | DROID-TELL-CADENCE-1 |
| CP-08 | A droid shot hurt the player with no cue. Now it gives the predator's hurt path: the same kick, rumble and hurt sting, and the "You {left} / 200" line on a free Pulse slot only, never on Sworn or Unbroken; a held wake line still wins. | `client/src/droid.rs`, `droid_tick`, `hurt_pulse` | DROID-HURT-READOUT-1 |

No capture: this audit adds no image (Hands standing rule, no media or binaries).

## UI noise audit (UI-NOISE-AUDIT-1)

Facts at `87031c99`, cite-only, from Sherif's playtester's report that the UI "reads like spam windows". Nothing in this section is a fix. Sizes are model heights, not enforced: `height_b` is "Not written onto `Style`" (`client/src/hud_anchor_registry.rs:244`). Times are seconds of app clock (`time.elapsed_secs_f64()`).

**Auto-open candidates on fresh default Play (Sanctuary)**

| ID | Panel | What opens it | How long | Where · size (model height, not enforced) |
|----|-------|---------------|----------|-------------------------------------------|
| U1 | Sovereign banner | Shows while not dismissed and app time < `banner_until` (`client/src/local_sovereign_session.rs:156`), default 8.5 s (`:26`, `:52`). Not gated on the yard door. Dismissed on any move, the first harvest or guidance dismiss (`:139-146`). | Until app time 8.5 s or the first move | Top-centre, top 52, 520 × 56 (`hud_anchor_registry.rs:452-462`) |
| U2 | Guidance strip | `active` defaults true on MoveAround (`client/src/first_session_guidance.rs:473-475`); shows in the yard, hides while a glow is in range (`:966-975`). First line "WASD walk · Space jump · Shift sprint" (`:384`). | Until the objective chain ends or is dismissed | ACTION_BAR, bottom-right, 520 × 69 (`hud_anchor_registry.rs:117-123`) |
| U3 | Pulse strip | Shows while `now < pulse_until` (`client/src/first_harvest_epiphany.rs:778`). | 2.0, 2.2 or 2.4 s (`:588`, `:636`, `:534`); 4.2 s on a harvest (`:45`, `:668`, `:725`); combat cues 4.2 s (`client/src/combat_core.rs:47`) | Top-centre, top 118, 560 × 60.8 (`hud_anchor_registry.rs:387-397`) |
| U4 | Whisper | Once, when `first_harvest_lived` turns true (`client/src/first_whisper.rs:94-103`). | 5.2 s (`:20`, `:118`) | Centre, top 28%, 420 × 69 (`hud_anchor_registry.rs:504-514`) |
| U5 | Climate slab and danger line | The slab always shows (`client/src/climate_visible.rs:504-514`). On each place entry, boot included, the text is the danger line (`:361-365`). | Slab: always. Danger line: 5.2 s (`:363`) | Bottom-left, bottom 176, 420 × 52 (`hud_anchor_registry.rs:413-423`) |
| U6 | Thriving toast | Queued per kind, one at a time (`client/src/thriving_moments.rs:115-124`); the first harvest fires `FirstMercyHarvest` (`client/src/first_harvest_epiphany.rs:663`). | 5 s each (`thriving_moments.rs:123`) | Top-centre, top 48, 620 × 58 (`hud_anchor_registry.rs:465-475`) |
| U7 | Satchel watch and pickup | Watch shows once the first harvest is lived (`client/src/human_inventory.rs:315`); pickup flashes per harvest (`:288`). | Watch: from then on. Pickup: 1.8 s (`:288`) | Watch bottom-left, 340 × 53.2 (`hud_anchor_registry.rs:426-436`); pickup centre, top 38%, 360 × 56.4 (`:439-449`) |
| U8 | Practice card | On FreeExploration, or guidance dismissed with 1 or more harvests (`client/src/living_practice_loop.rs:134-142`). | While active (`:281`) | ACTION_BAR, 640 × 64 (`hud_anchor_registry.rs:124-130`) |
| U9 | Care offer | After an idle tend (`client/src/mercy_harvest_nodes.rs:513-528`); shows while `offer.active` (`:644`). | Until the offer clears | ACTION_BAR, 560 × 62 (`hud_anchor_registry.rs:103-109`) |
| U10 | Place-name chip | Spawned with no `Visibility` and no writer (`client/src/climate_plane.rs:1470-1502`), so it is always on. | Always | Top-centre, top 18, 280 × 31 (`hud_anchor_registry.rs:543-553`) |

Conditional, not proven on for Sanctuary minute 0-10: Compass (`client/src/compass.rs:102`, needs a stored tell), Factory (`client/src/vertical_factory.rs:178`, non-Peace hex or a ready pack), Embassy (`client/src/embassy.rs:202-205`, needs `hour.complete`) and Spill (`client/src/infra_spill.rs:134`). Not on fresh Play: Welcome-back needs `hour.complete` or a loaded echo (`client/src/first_harvest_epiphany.rs:426-429`). Opt-in by key: Satchel I (`client/src/human_inventory.rs:241`), whose first-open line is held out of the pickup flash (`:244-246`, `:512`).

**First 60 s after Play (only cited triggers; order within one frame is unordered)**
- **At Play (P):** U10 and the U5 slab are on, U5 shows the danger line for 5.2 s, and U2 shows the WASD line. U1 is also on only if Play came before app time 8.5 s; that depends on how long the player sat on the title.
- **First move (player-dependent):** U1 is dismissed (`local_sovereign_session.rs:139-146`).
- **First harvest (player-dependent, no cited time):** in the same pass, U6 (5 s), U3 (4.2 s), U4 (5.2 s), the U7 pickup (1.8 s) and the U7 watch turn on, unordered. With U5 and U10 that is up to 7 text boxes at once. U2 is hidden only while the glow is in range (`first_session_guidance.rs:966`).
- **After that (player-dependent):** U9 can raise after an idle tend, and U8 once FreeExploration arrives.

**Overlap facts**
- The premise of UN-02 was stale. The six top-centre positions (PlaceName, Factory, Thriving, Sovereign, Fab, Pulse at `hud_anchor_registry.rs:543`, `:283`, `:465`, `:452`, `:309`, `:387`) are coded positions only. Boot always applies a preset, with Classic as the fallback (`local_settings.rs:484-490`), and every preset gives top-centre one `TOP_TOAST` Yield anchor (`hud_presets.rs:281`) whose occupants yield through `r2_yields_to` (`:744`).
- The first harvest also dismisses U1 (`local_sovereign_session.rs:144`), so U1 does not outlast the harvest burst; the frame order between the two systems is unordered.

**Gaps (suggested card names only; Core ranks them)**

| ID | Gap | Evidence | Suggested card |
|----|-----|----------|----------------|
| UN-01 | The first harvest opens up to five panels in one pass (Thriving, Pulse, Whisper, Pickup, Watch). | `client/src/first_harvest_epiphany.rs:663-668`; `client/src/first_whisper.rs:97-103`; `client/src/human_inventory.rs:288`, `:315` | UI-CALM-1 |
| UN-02 | Stale premise: the live presets already yield top-centre through one `TOP_TOAST` anchor. The real gap was that a yielded Thriving toast kept losing its 5 s while hidden; HUD-TOP-YIELD-1 pauses that clock while it is yielded. | `client/src/hud_presets.rs:281`; `client/src/thriving_moments.rs:123` | HUD-TOP-YIELD-1 |
| UN-03 | Stale premise: R5 (`client/src/hud_presets.rs:1915-1926`) already hides the Sovereign banner on Title, NameHouse and HouseDress. The real defect was the clock: 8.5 s of app time from boot could run out in the title flow. SOVEREIGN-YARD-GATE-1 counts only visible time (InYard, not yielded, not paused) and ignores intent before the yard. | `client/src/local_sovereign_session.rs:26`, `update_banner` | SOVEREIGN-YARD-GATE-1 |
| UN-04 | The place name showed twice all session: the place chip and the climate slab were both always on. Now the chip shows for `HOLD_SECS` (5.2 s) of visible yard time (InYard, not paused, not yielded) on boot and on each real place change, then hides; the climate slab still carries the place name and stays on. | `client/src/climate_plane.rs:1470-1503`, `update_climate_chip`; `client/src/climate_visible.rs:504-515` | HUD-QUIET-REST-1 |
| UN-05 | Places and Persona hid the Sovereign banner (R5) while its 8.5 s clock kept running, so opening Places early could spend the banner unseen. The clock now holds while either is open and resumes from where it was; HUD edit mode, which keeps the banner visible, still counts. | `client/src/local_sovereign_session.rs:172`, `update_banner`; `client/src/hud_presets.rs:1916-1926` | SOVEREIGN-MODAL-HOLD-1 |

No capture: this audit adds no image (Hands standing rule, no media or binaries).

## Gap table

One row per gap. `runtime` means a fix would touch `client/**`, `shared/**`, or the engine. `docs-only` is everything else. Size is one PR. PG-01 to PG-07 are spent (PR and short sha in each row, cites re-seated at `403ba3b9`). PG-08 is spent in part and stays open for its residue.

| ID | Area | Gap | Evidence | PATHS | Size | Kind | Proposed card name |
|----|------|-----|----------|-------|------|------|--------------------|
| PG-01 | Scripted hour | **Spent:** #721 `377c495`. The lived input system reads the keyboard and the first gamepad. `main` takes no arguments. The bundle has no action file or environment variable that walks or holds E. | Closed: `client/src/main.rs:35-36`; `client/src/main.rs:58-59`; `client/src/input.rs:96-111` | `client/src/input.rs`, `client/src/main.rs` | M | runtime | SCRIPT-HOUR-DRIVE-1 |
| PG-02 | Scripted hour | **Spent:** #722 `a94c885`, via `--script-walk`; `--script-run` stays a machine pass by design. `q1` / `--script-run` is a machine six-landing pass: two `cargo test` commands, a log grep, then exit. The mode never starts the client. | Closed: `scripts/play-offline.sh:105-117`; by design: `scripts/play-offline.sh:4`, `:101` | `scripts/play-offline.sh`, `client/src/input.rs` | M | runtime | SCRIPT-RUN-WALK-1 |
| PG-03 | Scripted hour | **Spent:** #724 `4dcbb50`. Headless Q2 starts the windowed client, writes one PNG, and the shell exits. The client has no headless argument and does not tend or save the hour on that path. | Closed: `.cursor/run-client-headless.sh:63-73` | `.cursor/run-client-headless.sh`, `client/src/loading_screen.rs`, `client/src/input.rs` | M | runtime | Q2-HOUR-DRIVE-1 |
| PG-04 | Scripted hour | **Spent:** #720 `30233d3`. `PlayerInput.interact` is a just-pressed edge, and `handle_interact_harvest` treats that flag as a take. Hold-to-tend reads the keyboard for `TEND_HOLD`. | Closed: `client/src/input.rs:405-410`; `client/src/first_harvest_epiphany.rs:524`; `client/src/first_harvest_epiphany.rs:594` (`TEND_HOLD` `:49`) | `client/src/input.rs`, `client/src/first_harvest_epiphany.rs` | S | runtime | SCRIPT-HOLD-TEND-1 |
| PG-05 | Human hour | **Spent:** #712 `3280351`. The guidance walk meter adds `delta_seconds * 6.0` while a move key is held and advances past `4.0`. Body walk speed is the separate constant `WALK` `3.4`. The card does not read `SoftPresence` position. | Closed: `client/src/first_session_guidance.rs:1044-1048`; `client/src/first_session_guidance.rs:1060` | `client/src/first_session_guidance.rs` | S | runtime | GUIDANCE-WALK-BODY-1 |
| PG-06 | Save | **Spent:** #710 `cf371d0`, plus #711 `26d7e87` (window close). Pause Quit sends `AppExit`. Pause Title calls `bind.persist`. Esc persists in `persist_climate_on_escape`. Digit3 opens pause without that Esc press. | Closed: `client/src/title_screen.rs:3051-3052`; `client/src/title_screen.rs:3059`; `client/src/title_screen.rs:3070`; Digit3 pause `client/src/title_screen.rs:2753` | `client/src/title_screen.rs` | S | runtime | QUIT-HOUR-PERSIST-1 |
| PG-07 | Save | **Spent:** #713 `ce2b73c`. The guidance card ends by setting `dismissed` and clearing `active`. That function writes no file, so a script has no end-of-card save to wait on. | Closed: `client/src/first_session_guidance.rs:1182-1188` | `client/src/first_session_guidance.rs` | S | runtime | HOUR-CARD-END-1 |
| PG-08 | Docs | **Spent in part:** #725 `c2ca7ae` added `## Scripted run` (`docs/FIRST_HOUR_PLAYTEST.md:91`). **Still open:** `docs/FIRST_HOUR_PLAYTEST.md:43` still calls `--script-run` "the headless door", echoed at `:59` and `:87` (banked as FIRST-HOUR-DOOR-WORD-1, not queued). The first-hour playtest calls `q1` / `--script-run` the headless door and names the `cargo test` lines. The shell mode never starts the client. The window boot is `.cursor/run-client-headless.sh`. | Open: `docs/FIRST_HOUR_PLAYTEST.md:43`, `:59`, `:87`; closed in part: `docs/FIRST_HOUR_PLAYTEST.md:91` | `docs/FIRST_HOUR_PLAYTEST.md` | S | docs-only | SCRIPT-RUN-DOC-1 |
