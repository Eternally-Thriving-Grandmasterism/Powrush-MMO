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

## Gap table

One row per gap. `runtime` means a fix would touch `client/**`, `shared/**`, or the engine. `docs-only` is everything else. Size is one PR. Suggested names are not queued and not spent.

| ID | Area | Gap | Evidence | PATHS | Size | Kind | Proposed card name |
|----|------|-----|----------|-------|------|------|--------------------|
| PG-01 | Scripted hour | The lived input system reads the keyboard and the first gamepad. `main` takes no arguments. The bundle has no action file or environment variable that walks or holds E. | `client/src/input.rs:336-348`; `client/src/input.rs:350-366`; `client/src/main.rs:34` | `client/src/input.rs`, `client/src/main.rs` | M | runtime | SCRIPT-HOUR-DRIVE-1 |
| PG-02 | Scripted hour | `q1` / `--script-run` is a machine six-landing pass: two `cargo test` commands, a log grep, then exit. The mode never starts the client. | `scripts/play-offline.sh:4`; `scripts/play-offline.sh:50-64`; `scripts/play-offline.sh:83`; `scripts/play-offline.sh:101-103` | `scripts/play-offline.sh`, `client/src/input.rs` | M | runtime | SCRIPT-RUN-WALK-1 |
| PG-03 | Scripted hour | Headless Q2 starts the windowed client, writes one PNG, and the shell exits. The client has no headless argument and does not tend or save the hour on that path. | `.cursor/run-client-headless.sh:59-66`; `.cursor/run-client-headless.sh:150-151`; `client/src/loading_screen.rs:47-48` | `.cursor/run-client-headless.sh`, `client/src/loading_screen.rs`, `client/src/input.rs` | M | runtime | Q2-HOUR-DRIVE-1 |
| PG-04 | Scripted hour | `PlayerInput.interact` is a just-pressed edge, and `handle_interact_harvest` treats that flag as a take. Hold-to-tend reads the keyboard for `TEND_HOLD`. | `client/src/input.rs:387-398`; `client/src/first_harvest_epiphany.rs:571-577`; `client/src/first_harvest_epiphany.rs:525`; `client/src/first_harvest_epiphany.rs:591` | `client/src/input.rs`, `client/src/first_harvest_epiphany.rs` | S | runtime | SCRIPT-HOLD-TEND-1 |
| PG-05 | Human hour | The guidance walk meter adds `delta_seconds * 6.0` while a move key is held and advances past `4.0`. Body walk speed is the separate constant `WALK` `3.4`. The card does not read `SoftPresence` position. | `client/src/first_session_guidance.rs:662`; `client/src/first_session_guidance.rs:1041-1042`; `client/src/human_presence.rs:75`; `client/src/human_presence.rs:780-806` | `client/src/first_session_guidance.rs` | S | runtime | GUIDANCE-WALK-BODY-1 |
| PG-06 | Save | Pause Quit sends `AppExit`. Pause Title calls `bind.persist`. Esc persists in `persist_climate_on_escape`. Digit3 opens pause without that Esc press. | `client/src/title_screen.rs:2671-2678`; `client/src/title_screen.rs:2695-2697`; `client/src/shard_climate.rs:158-161`; `client/src/title_screen.rs:2428-2429` | `client/src/title_screen.rs` | S | runtime | QUIT-HOUR-PERSIST-1 |
| PG-07 | Save | The guidance card ends by setting `dismissed` and clearing `active`. That function writes no file, so a script has no end-of-card save to wait on. | `client/src/first_session_guidance.rs:495-497`; `client/src/first_session_guidance.rs:1119-1121` | `client/src/first_session_guidance.rs` | S | runtime | HOUR-CARD-END-1 |
| PG-08 | Docs | The first-hour playtest calls `q1` / `--script-run` the headless door and names the `cargo test` lines. The shell mode never starts the client. The window boot is `.cursor/run-client-headless.sh`. | `docs/FIRST_HOUR_PLAYTEST.md:43`; `scripts/play-offline.sh:4`; `scripts/play-offline.sh:83`; `.cursor/run-client-headless.sh:7` | `docs/FIRST_HOUR_PLAYTEST.md` | S | docs-only | SCRIPT-RUN-DOC-1 |
