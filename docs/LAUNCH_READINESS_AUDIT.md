# Launch readiness gap audit

**Card:** AUDIT-LAUNCH-READINESS-1
**Base:** `6db11a7f` (main, DOCS-CYDRUID-STANDING-1 #600)
**Kind:** docs only. This file is the only change.

Shipped MMO, RPG, and ARPG games are reference points for the questions below (a save that survives a crash, a first hour a stranger can finish, a hit the hands can feel, a reason to come back). They are not evidence. This file does not name a quality tier for Powrush.

Evidence is a file:line cite, a CI run, or a command this seat ran, with the output written below. Any FPS, load-time, memory, or crash figure that is not in that output is **unmeasured**.

## Cited launch docs

Pointers only. The sentences live in those files.

- `LAUNCH-CHECKLIST.md:1-6` — banner says historical; the status line still says `100% LAUNCH WORTHY`.
- `docs/LAUNCH-CHECKLIST.md:12-16` and `:39-43` — a second checklist, v19.2.9, with remaining boxes still open, and no historical banner in the file.
- `LAUNCH_SCENARIO_SIMULATION.md:1-8` — stub. Full body is an old commit. README is named as canon.
- `docs/LAUNCH_UX.md:19-24` and `:45-46` — Title flow and the Online row. Online stays grey.
- `docs/FIRST_LAUNCH_UI_SCALE.md:26-38` and `:95` — text-scale presets, and a line that client wiring is still parked.
- `docs/PLAYTEST_AUDIT_2026-09-09.md:49-51` — slab overlap note from an older tip.
- `docs/LIVE_MOUTH_AUDIT.md:47` — says the lived-tick bridge appends JSON.

`docs/DOC_CANON.md:15-21` and `docs/COMPLETION_BRIEF.md:15` already say the old “100% launch worthy” files are historical. `README.md:9` and `README.md:17` say this tree is not a launch candidate.

## 1. Stability, crashes and save integrity

The lived client is `cargo run -p powrush-client` (`client/src/main.rs:9-37`). `PowrushClientBundle` is the plugin list in `client/src/lib.rs:93-157`.

House save and the local session file already use a temp file, a rename, and a `.bak` that is copied only when the live file parses (`shared/house_name.rs:374-387`, `client/src/local_session_persist.rs:105-119`). The shared helper most other JSON uses does not. `shared/user_persist.rs:56-62` is `write_named`: create the directory, then `fs::write` the whole file.

`LivedHourBind::persist` writes the lived tick, shard climate, shard standing, and week audit through that helper, and it ignores each error (`client/src/lived_hour_bind.rs:158-181`). A stop between those four writes can leave them from different moments. Each write can also tear, because `fs::write` replaces the file in place.

The journey echo uses the same direct write (`client/src/abundance_journey_echo.rs:85-96`). On the lavapipe boot in [Measurements](#measurements-this-seat), the log contains seven `journey echo saved` lines from `2026-10-02T20:27:38.733249Z` through `2026-10-02T20:27:40.766300Z`. The system marks the file dirty whenever `RbeAllocateChoice` is changed (`client/src/abundance_journey_echo.rs:154-173`). That boot had not allocated.

`client/src/lived_sim_bridge.rs:155-167` does not write the tick. It returns after a comment that `LivedHourBind::persist` owns the path. `docs/LIVE_MOUTH_AUDIT.md:47` still says that bridge appends a JSON line.

There is no panic hook in `client/` or `shared/`. Command and result are in Measurements. A panic ends the process with whatever those files last managed to write.

`client/src/` has 153 tracked `.rs` files (`git ls-files 'client/src/*.rs' | wc -l`). `client/src/lib.rs` has 66 `mod` lines, including `mod tests`. Files such as `client/src/inventory_ui.rs`, `client/src/taa_reprojection.rs`, `client/src/settings.rs`, `client/src/fracture.rs`, and `client/src/ability_bar.rs` are not those modules, and nothing in `client/src` pulls them in with `#[path]`. `LAUNCH-CHECKLIST.md:21-24` and `:33` describe inventory UI, TAA, SSR, and motion blur as already complete. The lived plugin list does not add them. `docs/LAUNCH-CHECKLIST.md` is a second, older list.

CI on `6db11a7f`:

- Actions run [37044293770](https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO/actions/runs/37044293770) `Core first hour` — success. Lib tests only (`.github/workflows/ci.yml:35-40`).
- Actions run [37044293737](https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO/actions/runs/37044293737) `Agent headless QA` — success. One PNG. The pass line is `Q2 PASS: frame exists at /tmp/powrush-q2.png (30410 bytes). No golden compare.` The same log’s test lines include `5 passed`, `468 passed`, and `679 passed`, each with `0 measured`. The workflow boots, grabs one frame, and exits (`.github/workflows/agent-headless-qa.yml:60-79`). It does not quit, reload a save, or print a frame time.
- Actions run [37044292176](https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO/actions/runs/37044292176) `.github/workflows/sync-from-ra-thor.yml` — `conclusion: failure`, `jobs: []`, and `gh run view --log-failed` returned `log not found`. The file on this commit is `workflow_dispatch` only (`.github/workflows/sync-from-ra-thor.yml:5-6`). That failure is not a client crash count. LR-07 is retired by #673 at 8b8fa141; Ra-Thor contributes via PRs and there is no cross-repo sync.

This seat’s lavapipe boot stayed up until the seat killed pid 3349. Crash count for a play session: **unmeasured**.

## 2. Onboarding and the first hour

The teaching card is one chain from walk through Hour three, then `FreeExploration` (`client/src/first_session_guidance.rs:392-407`). That last prompt is `this hex admits harm · optional` (`client/src/first_session_guidance.rs:375`). A test locks that phrase (`client/src/first_session_guidance.rs:1196-1198`). The card does not name a next yard action after the book.

Human OS, GPU, and minutes are still blank:

- `docs/FIRST_HOUR_PLAYTEST.md:9`
- `docs/HOUR_TWO_PLAYTEST.md:9`, `:83`, and `:87`
- `docs/HOUR_THREE_PLAYTEST.md:15`

This seat did not fill them. A later card is a human writing those blanks. An agent does not invent the numbers.

`docs/PLAYTEST_AUDIT_2026-09-09.md:51` says the climate slab uses `wards_line.or(well_line)` and hides the well sentence. Current code keeps the well and the Wards notice on one line (`client/src/climate_visible.rs:416-424`). The test at `client/src/climate_visible.rs:1043-1044` expects both.

## 3. Combat feel

`client/src/input.rs:10` says the lived input has no combat face. `shared/ledger_bind.rs:4` says DeclaredLethal is an opt-in ledger clause, not a combat key. This audit does not propose a combat face.

What the hands do feel is harvest: a camera kick and a pad rumble (`client/src/harvest_feel.rs:27-28` and `:91`). The well contest is a hold (`client/src/skirmish_well.rs:22`, `HOLD_SECS` 6.0). Win and loss change slab words and a color pulse (`client/src/skirmish_well.rs:3` and `:206-209`). That path does not set `SoftRbePool.kick` and does not send a rumble.

## 4. Progression and loot

The satchel face shows house, week, and vitality / harmony / joy (`client/src/human_inventory.rs:409-428`). It can append a temper line when `FabricatorYard.fab.last_tempered` is set (`client/src/human_inventory.rs:415-419`).

`shared/fabricator.rs:199-204` can craft a Tend Hook. The client care-cycle sets `CareCycleChoice::TemperTool` on Digit1 (`client/src/mercy_harvest_nodes.rs:588-591`) and does not call `craft_tend_hook`. A search of `client/` for `craft_tend_hook` and `apply_temper` returned no matches. Distill-on-Digit2 only runs when `last_tempered` is already `Some` (`client/src/mercy_harvest_nodes.rs:595-598`). In the lived bundle that field stays empty, so the satchel temper line stays empty.

## 5. The content loop

`GuidanceObjective::next` stops on `FreeExploration` (`client/src/first_session_guidance.rs:404-407`). Section 2 covers that sentence.

The practice loop’s last surface stays on itself (`client/src/living_practice_loop.rs:59-65`). `try_activate_from_guidance` returns when `principle_sealed` is set (`client/src/living_practice_loop.rs:117-118`). The sealed prompt is `You carried the same principle across three climates. Sovereign exploration continues.` (`client/src/living_practice_loop.rs:53-54`). There is no later surface in that match.

## 6. UI/UX and accessibility

Settings has a Text scale step. Default is `1.0`, minimum `0.85`, maximum `1.35`, step `0.05` (`shared/local_settings.rs:34-37` and `:490`). The button wraps from the max back to the minimum (`shared/local_settings.rs:878-887`). The label is `Text scale · {number}` (`client/src/title_screen.rs:2883-2884`).

`docs/FIRST_LAUNCH_UI_SCALE.md:32-38` names Comfort, Standard, and Compact, and asks the first-launch pick to be the larger one. Those three names are not in `shared/local_settings.rs`. The same doc says client wiring is parked (`docs/FIRST_LAUNCH_UI_SCALE.md:95`) and that Hands will persist the scale later (`docs/FIRST_LAUNCH_UI_SCALE.md:28`). The button and `data` persist already exist. Title rows and the garden-want line read `text_scale` (`client/src/title_screen.rs:2236` and `:3173`). The yard plates spawn at fixed sizes: climate slab `14.0` (`client/src/climate_visible.rs:285`), guidance card `17.0` (`client/src/first_session_guidance.rs:796`), satchel title `14.0` (`client/src/human_inventory.rs:164`). `scaled_font` is defined for pause and settings rows (`client/src/local_settings.rs:125-127`). `client/src/first_session_guidance.rs:1584` mentions `text_scale` in a test, not in the card spawn.

Colorblind well shapes and 44px touch targets are already in the bundle (`client/src/climate_visible.rs:550`, `client/src/touch_controls.rs:18-19`). This audit does not reopen them.

No `accesskit` string exists in the tracked `.rs` or `.toml` files. A screen reader would be a new crate. That sits with crate bumps in the steward list. No card.

## 7. Art and audio

The lived body is capsules and spheres (`client/src/human_presence.rs:582-588`). `git ls-files '*.glb' '*.gltf'` printed `0`. Place separation in the lived client is fog and color (`client/src/climate_plane.rs:25-46`). New authored meshes are a steward blocker. No card.

`docs/PLACE_DRESS_SPEC.md:7-9` still says the dress is unbuilt and that no client edit follows from the file. `client/src/climate_plane.rs:25-46` describes Sanctuary, Heartwood, Threshold, and Depths dress as already in this file.

The mixer loads two assets (`shared/peace_audio.rs:28-36`): `audio/peace_yard_bed.ogg` and `audio/peace_well_sting.ogg`. Heartwood and Depths reuse that bed at a lower gain (`shared/peace_audio.rs:34-37`). `client/src` does not mention `epic_dark_ambient`. `git ls-files` tracks that ogg and the two peace files only.

`assets/audio/README.md:12-20` names `mercy_harvest_sting.ogg`, place variants, `rollback_whoosh.ogg`, `epiphany_bloom.ogg`, and `emergence_resonance.ogg`. `client/assets/audio/README.md:9-15` names a premade set under `client/assets/audio/premade/`. `git ls-files` does not track those paths (command in Measurements).

## 8. Performance

Frame time is **unmeasured**. The client log from this seat and from CI run 37044293737 does not print FPS. Both logs warn that the adapter is software rendering (`llvmpipe` here; the CI line is `The selected adapter is using a driver that only supports software rendering. This is likely to be very slow.`).

The headless gate treats a non-empty PNG as pass (`.github/workflows/agent-headless-qa.yml:75-79`). Byte count of that PNG is 30410. That is not a frame time.

This seat’s debug process RSS is in Measurements. It is a debug binary on llvmpipe, not a release package and not a player GPU. Release memory and player-GPU frame time are **unmeasured**.

G0 light gen logged `OFF` on that boot. That matches the default in `README.md:25`. No card.

## Measurements (this seat)

### Panic hook

```text
rg -n "panic::set_hook|std::panic::set_hook" client shared --glob '*.rs' || echo 'NO panic::set_hook in client/ or shared/'
```

```text
NO panic::set_hook in client/ or shared/
```

### Tracked audio and meshes

```text
git ls-files '*.glb' '*.gltf' | wc -l
```

```text
0
```

```text
wc -c assets/audio/peace_yard_bed.ogg assets/audio/peace_well_sting.ogg assets/music/epic_dark_ambient.ogg
```

```text
15034 assets/audio/peace_yard_bed.ogg
 4296 assets/audio/peace_well_sting.ogg
 1803 assets/music/epic_dark_ambient.ogg
21133 total
```

```text
git ls-files --error-unmatch assets/audio/mercy_harvest_sting.ogg assets/audio/mercy_harvest_sting_sanctuary.ogg assets/audio/mercy_harvest_sting_verdant.ogg assets/audio/mercy_harvest_sting_horizon.ogg assets/audio/rollback_whoosh.ogg assets/audio/epiphany_bloom.ogg assets/audio/emergence_resonance.ogg
```

```text
error: pathspec 'assets/audio/mercy_harvest_sting.ogg' did not match any file(s) known to git
error: pathspec 'assets/audio/mercy_harvest_sting_sanctuary.ogg' did not match any file(s) known to git
error: pathspec 'assets/audio/mercy_harvest_sting_verdant.ogg' did not match any file(s) known to git
error: pathspec 'assets/audio/mercy_harvest_sting_horizon.ogg' did not match any file(s) known to git
error: pathspec 'assets/audio/rollback_whoosh.ogg' did not match any file(s) known to git
error: pathspec 'assets/audio/epiphany_bloom.ogg' did not match any file(s) known to git
error: pathspec 'assets/audio/emergence_resonance.ogg' did not match any file(s) known to git
```

`command -v scrot` and `command -v import` both failed on this machine (`scrot not installed`, `import not installed`). This seat did not write a local PNG.

### Lavapipe boot of the existing debug binary

Debug binary already on disk: `target/debug/powrush-client` (`wc -c` → `848839880`). Not a release build. `POWRUSH_NET=off`. `WGPU_BACKEND=vulkan`. `VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json`. Xvfb `:99` at 1280x800. The seat started the binary, waited until `xwininfo` showed `Powrush`, sampled `/proc/<pid>/status`, then killed that pid.

```text
binary bytes: 848839880
client_pid 3349
window_up_ns 1028527018
window_up=1
VmPeak:	 1590672 kB
VmSize:	 1590672 kB
VmHWM:	  192964 kB
VmRSS:	  192964 kB
--- /proc status +2s ---
VmPeak:	 1932904 kB
VmSize:	 1924784 kB
VmHWM:	  421020 kB
VmRSS:	  421020 kB
```

Log lines from that process:

```text
INFO bevy_diagnostic::system_information_diagnostics_plugin::internal: SystemInfo { os: "Linux 24.04 Ubuntu", kernel: "6.12.94+", cpu: "Intel(R) Xeon(R) Processor", core_count: "4", memory: "15.6 GiB" }
INFO bevy_render::renderer: AdapterInfo { name: "llvmpipe (LLVM 20.1.2, 256 bits)", vendor: 65541, device: 0, device_type: Cpu, driver: "llvmpipe", driver_info: "Mesa 25.2.8-0ubuntu0.24.04.4 (LLVM 20.1.2)", backend: Vulkan }
WARN bevy_render::renderer: The selected adapter is using a driver that only supports software rendering. This is likely to be very slow. See https://bevyengine.org/learn/errors/b0006/
INFO powrush::gen: Sanctuary Prime · G0 light gen OFF (Settings Grove · light or POWRUSH_GEN=light) mode="off"
INFO bevy_winit::system: Creating new window "Powrush-MMO — first hour"
```

`rg -c "journey echo saved" /tmp/powrush-boot.log` was `7`, timestamps as in section 1. The process was still running at the second sample. FPS: **unmeasured**.

### Not run here

`cargo test` was not run in this seat. The counts above are from Actions run 37044293737 on `6db11a7f`. This commit does not change Rust.

## Steward blockers (Sherif decides)

Listed only. No cards. Online stays grey.

- **Online play.** `docs/LAUNCH_UX.md:45-46`. Title row stays off. No listen and no public bind from this audit.
- **Steam and App ID.** `docs/OFFLINE_SKU.md:22`. `client/src/steam_integration.rs:10` and `:112-114` describe an App ID and a commented production plugin. That file is not a module in `client/src/lib.rs`. `client/Cargo.toml` keeps `steamworks` optional. The lived bundle does include `SteamAbundanceMirrorPlugin` (`client/src/lib.rs:146`), which staged local directories on the boot above. No App ID is set here.
- **MESH (new 3D assets).** `git ls-files` shows zero `.glb` / `.gltf`. `docs/PLACE_DRESS_SPEC.md:23` budgets 0 meshes from that stamp. New meshes wait on the steward.
- **Bevy version climb.** `client/Cargo.toml:17` is `bevy` `0.14`. This audit does not change it.
- **Payments.** `docs/PARKED_SURFACES.md:7` parks `payments/`. `README.md:51` leaves payments out of the first hour.
- **Crate bumps.** Workspace members stay `shared`, `crates/rsil-identity`, and `client` (`Cargo.toml:2-5`). No version bumps. A screen-reader crate is in this bucket.

## Gap table

One row per gap. PATHS were checked with `git ls-files` on `6db11a7f`. Size is one PR. `runtime` means `client/**`, `shared/**`, or engine. `docs-only` is everything else. Performance is under stability only when it is a crash or a torn save; the frame-time row is with UI/UX.

| ID | Area | Gap | Evidence | PATHS | Size | Kind | Proposed card name |
|----|------|-----|----------|-------|------|------|--------------------|
| LR-01 | Stability, crashes and save integrity | `write_named` replaces a persist file with one `fs::write` and keeps no `.bak`. | `shared/user_persist.rs:56-62` | `shared/user_persist.rs` | S | runtime | SAVE-NAMED-ATOMIC-1 |
| LR-02 | Stability, crashes and save integrity | Lived tick, climate, standing, and week are four ignored writes, so a stop between them leaves a mixed set. | `client/src/lived_hour_bind.rs:158-181` | `client/src/lived_hour_bind.rs` | S | runtime | SAVE-HOUR-SET-1 |
| LR-03 | Stability, crashes and save integrity | An idle boot rewrote the journey file seven times in about two seconds through a direct `fs::write`. | `client/src/abundance_journey_echo.rs:154-173`; boot log in Measurements | `client/src/abundance_journey_echo.rs` | S | runtime | JOURNEY-SAVE-QUIET-1 |
| LR-04 | Stability, crashes and save integrity | A panic has no local hook or crash log in the client or shared crates. | Measurements: `NO panic::set_hook in client/ or shared/` | `client/src/main.rs` | S | runtime | CRASH-LOG-HOOK-1 |
| LR-05 | Stability, crashes and save integrity | Two launch checklists disagree, and the root one still calls inventory UI and TAA complete while those files are outside the lived plugin list. | `LAUNCH-CHECKLIST.md:6` and `:33`; `docs/LAUNCH-CHECKLIST.md:39-43`; `client/src/lib.rs:93-157` | `LAUNCH-CHECKLIST.md`, `docs/LAUNCH-CHECKLIST.md` | S | docs-only | LAUNCH-CHECKLIST-ONE-1 |
| LR-06 | Stability, crashes and save integrity | Headless QA passes when one PNG exists and does not quit and load the lived tick. | Actions run 37044293737; `.github/workflows/agent-headless-qa.yml:75-79` | `.github/workflows/agent-headless-qa.yml`, `.cursor/run-client-headless.sh` | S | docs-only | Q2-RELOAD-GATE-1 |
| LR-07 | Stability, crashes and save integrity | Retired by #673 at 8b8fa141. The Ra-Thor sync workflow run on this commit failed with no jobs and no log. | Actions run 37044292176; `.github/workflows/sync-from-ra-thor.yml:5-6` | `.github/workflows/sync-from-ra-thor.yml` | S | docs-only | SYNC-WORKFLOW-RETIRE-1 (#673), spent |
| LR-08 | Stability, crashes and save integrity | The mouth audit says the lived-tick bridge appends JSON, and that function returns without writing. | `docs/LIVE_MOUTH_AUDIT.md:47`; `client/src/lived_sim_bridge.rs:155-167` | `docs/LIVE_MOUTH_AUDIT.md` | S | docs-only | LIVE-MOUTH-TICK-1 |
| LR-09 | Onboarding and the first hour | OS, GPU, and minutes on the hour playtest notes are still blank. | `docs/FIRST_HOUR_PLAYTEST.md:9`; `docs/HOUR_TWO_PLAYTEST.md:83`; `docs/HOUR_THREE_PLAYTEST.md:15` | `docs/FIRST_HOUR_PLAYTEST.md`, `docs/HOUR_TWO_PLAYTEST.md`, `docs/HOUR_THREE_PLAYTEST.md` | S | docs-only | PLAYTEST-FELT-HUMAN-1 |
| LR-10 | Onboarding and the first hour | After Hour three the card’s next sentence is “this hex admits harm · optional”, which names no yard action. | `client/src/first_session_guidance.rs:375` and `:404-407` | `client/src/first_session_guidance.rs` | S | runtime | GUIDANCE-AFTER-BOOK-1 |
| LR-11 | Onboarding and the first hour | The 2026-09-09 playtest audit still says a Wards line replaces the well line. | `docs/PLAYTEST_AUDIT_2026-09-09.md:51`; `client/src/climate_visible.rs:416-424` | `docs/PLAYTEST_AUDIT_2026-09-09.md` | S | docs-only | PLAYTEST-AUDIT-REFRESH-1 |
| LR-12 | Combat feel | A well contest ends as slab words and a tint, while the harvest kick and rumble stay on the harvest path. | `client/src/skirmish_well.rs:206-209`; `client/src/harvest_feel.rs:27-28` | `client/src/skirmish_well.rs`, `client/src/harvest_feel.rs` | S | runtime | CONTEST-IMPACT-1 |
| LR-13 | Progression and loot | Digit1 records a Temper choice, and the client never calls `craft_tend_hook`, so the satchel temper line stays empty. | `client/src/mercy_harvest_nodes.rs:588-598`; `shared/fabricator.rs:199-204` | `client/src/mercy_harvest_nodes.rs`, `client/src/fabricator.rs`, `shared/fabricator.rs` | M | runtime | TEMPER-CRAFT-CALL-1 |
| LR-14 | UI/UX and accessibility | Text scale is a 0.85–1.35 step that defaults to 1.0, with no Comfort, Standard, or Compact pick. | `shared/local_settings.rs:34-37`; `docs/FIRST_LAUNCH_UI_SCALE.md:32-38` | `shared/local_settings.rs`, `client/src/title_screen.rs` | M | runtime | UI-TEXT-PRESET-1 |
| LR-15 | UI/UX and accessibility | The text-scale value updates settings rows and does not change the climate slab, guidance card, or satchel fonts. | `client/src/climate_visible.rs:285`; `client/src/first_session_guidance.rs:796`; `client/src/human_inventory.rs:164` | `client/src/climate_visible.rs`, `client/src/first_session_guidance.rs`, `client/src/human_inventory.rs` | M | runtime | UI-SCALE-SLABS-1 |
| LR-16 | UI/UX and accessibility | The first-launch scale doc still says the client wiring is parked. | `docs/FIRST_LAUNCH_UI_SCALE.md:28` and `:95`; `client/src/title_screen.rs:2883-2884` | `docs/FIRST_LAUNCH_UI_SCALE.md` | S | docs-only | UI-SCALE-DOC-1 |
| LR-17 | UI/UX and accessibility | The lived client does not log frame time, so FPS on this seat and in the one-frame CI run is unmeasured. | Actions run 37044293737; Measurements boot log | `client/src/main.rs` | S | runtime | FRAME-TIME-LOG-1 |
| LR-18 | Art and audio | Audio readmes name ogg and wav files that git does not track, while the lived mixer loads two tracked ogg files. | `assets/audio/README.md:12-20`; `shared/peace_audio.rs:28-30`; Measurements `git ls-files --error-unmatch` | `assets/audio/README.md`, `client/assets/audio/README.md` | S | docs-only | AUDIO-CATALOG-MATCH-1 |
| LR-19 | Art and audio | The place-dress spec still says the dress is unbuilt, while the climate plane file already describes four dressed places. | `docs/PLACE_DRESS_SPEC.md:7-9`; `client/src/climate_plane.rs:25-46` | `docs/PLACE_DRESS_SPEC.md` | S | docs-only | PLACE-DRESS-STATUS-1 |
| LR-20 | The content loop | After the third practice surface the loop stays sealed and does not offer another step. | `client/src/living_practice_loop.rs:53-65` and `:117-118` | `client/src/living_practice_loop.rs` | S | runtime | PRACTICE-AFTER-SEAL-1 |
