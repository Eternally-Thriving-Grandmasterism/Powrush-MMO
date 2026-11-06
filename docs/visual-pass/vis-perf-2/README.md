# VIS-PERF-2 measurement evidence

RP7 court evidence only. This folder is one measurement of the contained yard. It is not a ship vote, not a Dual, not GREEN, and not ready. No optimization and no ablation were committed or measured. Title Online stays grey. No OFFER NEXT.

Contact: info@Rathor.ai. Independent of xAI.

## What was measured

One process, four settled windows, fixed renderer and world, no rebuilt entities and no compile between windows:

`startup 100` → `preroll 48` → `off1 29` → `on1_warm 20` → `on1 29` → `off2 29` → `on2_warm 20` → `on2 29` → one tail frame that writes these files.

Warm frames and startup and preroll are not in the settled rankings. Screenshots were not taken. OFF/ON flips only the span-sink buffer. `RenderDiagnosticsPlugin`, the subscriber, the chrome layer, and the renderer stay up for the whole run. That common overhead is in every window and is not claimed as sink overhead.

The scene is the existing ignored yard `vp_sky_2_lavapipe_low_and_medium_yard`: realm 0, conifer seed `0x12345678`, the same camera (`Transform::from_xyz(0.0, 3.25, 6.4).looking_at(Vec3::new(0.0, 5.2, -28.0), Vec3::Y)`), plate and glow, `PainterlyPlugin`, and `SkyBackdropPlugin`. It is not the whole game. `PowrushClientBundle`, `VegetationPlugin`, `NetModePlugin`, and the gameplay plugins are not loaded. Graphics preset is Medium from frame 0. Session net is `PowrushNet::Off` inserted as `SessionNetMode`. `NetModePlugin` is not added. `title_online_enabled` is false. `opens_listen_socket` is false. Fresh user dir, empty at the end of the run.

`PipelinedRenderingPlugin` is disabled at setup for the whole run, not toggled between OFF and ON. Bevy's system spans are created with `parent: None`, and worker threads do not inherit the tracing parent, so a global schedule stack is only unambiguous while one schedule is active. That disable is a harness constraint. It is not an optimization and it is not sink overhead.

## Machine

- Base commit: `4c461cc486fa8cac801c49bc4b2067e1145de72c`
- OS: PRETTY_NAME="Ubuntu 24.04.5 LTS" | NAME="Ubuntu" | VERSION_ID="24.04" | VERSION="24.04.5 LTS (Noble Numbat)"
- uname: Linux cursor 6.12.113+ #1 SMP PREEMPT_DYNAMIC Thu Nov  5 16:04:51 UTC 2026 x86_64 x86_64 x86_64 GNU/Linux
- CPU: Intel(R) Xeon(R) Processor (`nproc` 4)
- rustc: rustc 1.99.1 (58cf8f9c3 2026-10-13)
- cargo: cargo 1.99.1 (5f94df478 2026-08-27)
- Profile: `cargo test` dev (`unoptimized + debuginfo`), not release
- Bevy 0.20.1, wgpu 30.0.1, tracing-subscriber 0.3.23, tracing-chrome 0.7.2
- Display: `:1`, `WAYLAND_DISPLAY` unset
- Vulkan ICD file: `/usr/share/vulkan/icd.d/lvp_icd.json` (`library_path` `libvulkan_lvp.so`)
- `WGPU_BACKEND=vulkan`
- Adapter: name=llvmpipe (LLVM 20.1.2, 256 bits) vendor=65541 device=0 type=Cpu backend=Vulkan driver=llvmpipe driver_info=Mesa 25.2.8-0ubuntu0.24.04.4 (LLVM 20.1.2) pci=
- The ICD requested is lavapipe. wgpu reports the adapter name `llvmpipe (LLVM 20.1.2, 256 bits)`, driver `llvmpipe`, device type `Cpu`, backend `Vulkan`. That name string is the adapter name this lavapipe ICD returns.
- Timestamp query: True. Inside passes: True. Inside encoders: True. Pipeline statistics: True. Timestamp period ns: 1.0.
- Requested inner size: 1920x1080 physical, scale_factor_override 1.0
- Requested present mode: Immediate (non-Fifo)
- Observed window, all Last samples: first `physical=1900x1068 logical=1900x1068 scale=1 present=Immediate`; last `physical=1920x1080 logical=1920x1080 scale=1 present=Immediate`; modes ['Immediate']; scales ['1']
- Frame 0 (startup, no gap row) was 1900×1068 Immediate. Every gap from frame 1 through frame 304, including all 116 counted gaps, is 1920×1080 Immediate. Logical size matched physical size. Scale factor was 1. No Fifo fallback was logged.
- Presets seen: ['Medium']
- Scene counts snapshotted from frame 1 and unchanged: sky 13, cameras 1, meshes 15. Change note: None.
- Net: SessionNetMode inserted from PowrushNet::Off. NetModePlugin was not added. No listen. Label `offline · one human · one machine`. offline true. title_online false. opens_listen false.
- User dir: `/tmp/vis-perf-2-user-4982`. Entries after the run: [].
- Command: `cd /agent/repos/Powrush-MMO && VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json WGPU_BACKEND=vulkan TRACE_CHROME=/tmp/vis-perf-2-chrome.json DISPLAY=:1 cargo test -p powrush-client --lib sky_backdrop::tests::vis_perf_2_contained_yard -- --ignored --exact --nocapture`
- Args recorded inside the test: `/agent/repos/Powrush-MMO/target/debug/deps/powrush_client-72532980b1e90c44 sky_backdrop::tests::vis_perf_2_contained_yard --ignored --exact --nocapture`

Bevy log lines kept by the observer:

- INFO bevy_render::renderer: message=AdapterInfo { name: "llvmpipe (LLVM 20.1.2, 256 bits)", vendor: 65541, device: 0, device_type: Cpu, device_pci_bus_id: "", driver: "llvmpipe", driver_info: "Mesa 25.2.8-0ubuntu0.24.04.4 (LLVM 20.1.2)", backend: Vulkan, subgroup_min_size: 8, subgroup_max_size: 8, transient_saves_memory: Some(false), limit_bucket: None }
- WARN bevy_render::renderer: message=The selected adapter is using a driver that only supports software rendering. This is likely to be very slow. See https://bevy.org/learn/errors/b0006/

Full env notes are in `machine.json`.

## Capture method

enter Instant to exit Instant; bevy system spans are long-lived and re-entered, so tracing on_close is the process-lifetime drop and is not the per-invocation sample.

global stack of bevy_ecs schedule spans; PipelinedRenderingPlugin disabled at setup so only one schedule is active while workers run.

system spans are created with parent: None, so tracing parent id is expected null.

span vec push only while sink is open; RenderDiagnosticsPlugin and the subscriber stay up for all four windows.

trace, trace_chrome (includes bevy debug), chrome layer, RenderDiagnosticsPlugin, subscriber callbacks, and diagnostic sample copy on OFF and ON are not claimed as sink overhead.

DiagnosticMeasurement.time is the PreUpdate sync stamp, not the GPU execution time.

(path, measurement_ns from that stamp); a cached stamp is not a second invocation.

0 when the stamp is at or after the previous observer instant; 1 when the stamp predates it.

Unsupported GPU or pipeline counters are absent paths, stored as no sample, not as 0. `measured_zero` is true only when the value is present and actually 0. Non-finite values would be JSON null. This run had no null diagnostic values and no measured zeros.

`trace_chrome` does not register `RenderDiagnosticsPlugin`. The harness adds that plugin once, in `finish`/`build` at setup. Pass CPU and GPU numbers come from `DiagnosticsStore` in `Last`, after `PreUpdate` `sync_diagnostics`. A cached `(path, measurement.time)` is not written again.

System rankings key `(app bucket, schedule, system name)`. Bucket `main` is the app stack value `main`. Bucket `render` is any other app label containing `render`. This run had no `other` bucket. Main and RenderApp are not added together. System wall times overlap across threads and are not summed into a frame gap.

Percentile is nearest-rank `ceil(0.95*n)` 1-based. Median is the middle sample, or the mean of the two middle samples when the count is even. Rank sort is median descending, then key.

The live recorder is the `VisPerfLayer` subscriber installed once through `LogPlugin.custom_layer`. The chrome layer also ran for the whole process because `trace_chrome` was on. The chrome file is not in this commit.

- Chrome path: `/tmp/vis-perf-2-chrome.json`
- Chrome bytes: 376690735
- Chrome sha256: `534694da9542800e0b42f0d4b792b0980dc909597ac225f449cff3c20fab7fbc`

No new `[dependencies]` were added. `tracing`, `serde`, and `serde_json` were already direct client dependencies. `tracing-subscriber` 0.3.23 and `tracing-chrome` 0.7.2 were pulled by the temporary Bevy features `trace` and `trace_chrome` and left the tree when `Cargo.lock` was restored. The only `client/Cargo.toml` edit was those two feature names, after `sysinfo_plugin`.

## Hashes

Base file sha256, matching the working tree after restore. Git blobs are `git rev-parse HEAD:path` at `4c461cc486fa8cac801c49bc4b2067e1145de72c`. `Cargo.lock` is gitignored and has no HEAD blob.

| path | sha256 | git blob |
| --- | --- | --- |
| `client/Cargo.toml` | `1d05da30739bcb1e4956c84631b2a295add52963efaa60617aadb54de4e572b8` | `036a6aa33d80ef61309f3ef5380df8bc1e07e1fa` |
| `client/src/sky_backdrop.rs` | `16d6b5f50bce2ba327809e2fb388cb7c5be61fa133875e623a7b3c24dadb3b76` | `1edc824a57b37308fab37093cfd29c3acc4e5b38` |
| `client/src/main.rs` | `5a8275fa36ae1fa13a2f70a346320d2e03554f5fbe973168442e6f3ce21bcb85` | `47b7b248b509630578cb25d8fe95ff9ef0532cf8` |
| `client/src/lib.rs` | `a39d0180e7db640ccc4bf20dd3d3e3fc7b2f3ab7a22df499b2173220dce6e692` | `e6efc1ae961410588449495ab8c1ee9da64ee8d7` |
| `Cargo.toml` | `50f9b1bcd4909fbc15a46eb60d07f2ef3fa9832987e2b5b3911277f54391f8a5` | `5d9a8f956faeb348f5f8cfac4c416547601cf9d7` |
| `Cargo.lock` | `5b7de4c54b281895431b33bd5eb08274856979e1dca7a22d7b6746fc13379f49` | not in HEAD |

Temporary observer, present only while the pair ran:

| blob | sha256 | git hash-object |
| --- | --- | --- |
| instrumented `client/Cargo.toml` | `02b8ea4c643c7d88e8340908def1077a31748e3a07fb60e856e11345093de5d1` | `6223f2fb3445330ce06662d9ce3d4672ab3bdc5b` |
| instrumented `client/src/sky_backdrop.rs` | `f2396c353e6be7932fe55b46bd93dcce5895ea3673c72e798263b289ee405555` | `9ba06144f6246fb38b8b7e9c8f4cd8efbd52ddaa` |
| observer source slice below | `cc51066c5992926ff725d5c87a87eaa859b29f52351bca6c6a4d4c4f43c87b14` | `1fd8e06a2bee8c0755fb70edd1eab717234c0e23` |
| `Cargo.lock` during capture | `296d8f2664f1eff202f362e1eff75ac74437a584be880a21ee79f3744978c896` | `a25f7cf741a91b19fd4d65967edda9a729d7d603` |

After the pair, `git checkout -- client/Cargo.toml client/src/sky_backdrop.rs` and the saved base `Cargo.lock` were copied back. `git diff --exit-code 4c461cc486fa8cac801c49bc4b2067e1145de72c -- client shared host powrush-divine-module Cargo.toml` exited 0. There is no `divine-module` path. `Cargo.lock` sha256 matches the base file above. The restored gate is recorded at the end of this note after `cargo test -p shared -p rsil-identity` and `cargo test -p powrush-client --lib` on that tree, ordinary default, no ignored tests.

An earlier complete pair, from before gap rows carried physical size, was deleted and was not spliced into these files. The files here are one later process.

## Files

| file | sha256 |
| --- | --- |
| `summary.json` | `bcf96e652d7404891b5c9c0c07efbad0048c373658abffddfe7ea4731c52ec38` |
| `machine.json` | `ca9140fde91fc0677219ea0c680f8c89bf0d416f9c9a177330254d8688cb746b` |
| `frame_gaps.csv` | `38d7a7caa28ca83e6f3eff18a6421a6952f2dbf5109383fdd1a53a108dc95b42` |
| `diagnostics.jsonl` | `bd6ed4807ec6a7959dde1a58bad7530e9f7930fe242cf251b23a5c7612575e4e` |
| `spans.jsonl` | `3bbb6311e86481fcf0b018dba4d1d62b11af89b453a7c58443b66e91dcdcc6b3` |

Raw counts: span records 186883, diagnostic records 3744, gap records 304. `summary.json` holds the full system and pass rankings. The tables below are the distinct outcomes, not a substitute for that file.

`docs/profiling/run_perf_probe.sh` was not added. The observer is not in the restored tree, so a launcher would not run. The command is the one in Machine.

## Frame gaps

Same probe on every window: `Last` observer `Instant` minus the previous `Last` observer `Instant`. Unit milliseconds. The first frame has no gap. Later gaps include the boundary from the previous phase. Counted rows are the 29 settled frames only.

| window | count | median | mean | min | max | p95 |
| --- | --- | --- | --- | --- | --- | --- |
| off1 | 29 | 151.590731 | 155.01101744827588 | 143.503311 | 174.193253 | 173.150413 |
| on1 | 29 | 153.044522 | 154.02882058620688 | 146.341078 | 180.809786 | 168.614862 |
| off2 | 29 | 148.881367 | 150.93550117241375 | 142.021722 | 171.290449 | 170.951506 |
| on2 | 29 | 151.613322 | 152.14324268965518 | 144.641459 | 167.709214 | 160.136916 |

Adjacent deltas (later minus earlier), same statistics:

| delta | median | mean | min | max |
| --- | --- | --- | --- | --- |
| on1_minus_off1 | 1.4537909999999954 | -0.9821968620690029 | 2.8377670000000137 | 6.616533000000004 |
| off2_minus_on1 | -4.163154999999989 | -3.0933194137931252 | -4.319355999999999 | -9.519337000000007 |
| on2_minus_off2 | 2.7319549999999992 | 1.2077415172414305 | 2.6197369999999864 | -3.5812349999999924 |
| off2_minus_off1 | -2.7093639999999937 | -4.075516275862128 | -1.4815889999999854 | -2.9028040000000033 |

The ON minus OFF median is the same scale as the pre/post OFF drift. These deltas are the frame probe under a sink that is recording spans. They are not a pass ablation and not a product hotspot.

## CPU system rankings

Counted `system` spans with a schedule span still open: on1 23649/23649, on2 23616/23616. Tracing parent id was 0 on on1 and 0 on on2, which matches `parent: None`. Warm attribution: 16274/16274. Span exits during the sink with no recorded enter: 1309. Those misses are not samples. Non-finite diagnostics: 0.

Distinct systems: main 274 on1 and 274 on2; render 408 on1 and 408 on2; other 0 and 0.

Top 15 main order identical on1 vs on2: False. Top 15 render order identical: False.

Thread overlap on counted ON frames: 58 frames had more than one system-span thread. Thread count per such frame median 3.0, min 3.0, max 3.0. Not summed into the frame gap.

`powrush_client::sky_backdrop::tests::vis_perf_observe` is the harness `Last` system. It is in the ranking because it is a real `bevy_ecs` system span during the ON sink. It is not a product system. Its ON samples include the evidence copy. OFF frames still run it, but the sink is closed so those invocations are not in the ON tables.

### Main, on1

| rank | system | schedule | count | median ms | mean ms | p95 ms |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | `bevy_time::fixed::run_fixed_main_schedule` | `RunFixedMainLoop` | 29 | 4.34064 | 4.379322172413793 | 5.525548 |
| 2 | `powrush_client::sky_backdrop::tests::vis_perf_observe` | `Last` | 29 | 2.596599 | 2.678144724137931 | 3.10369 |
| 3 | `bevy_pbr::prepass::update_mesh_previous_global_transforms` | `PreUpdate` | 29 | 0.404774 | 0.4810910689655171 | 0.815555 |
| 4 | `bevy_pbr::material::check_entities_needing_specialization<bevy_pbr::extended_material::ExtendedMaterial<bevy_pbr::pbr_material::StandardMaterial, powrush_client::painterly_shader::PainterlyExtension>>` | `PostUpdate` | 29 | 0.358027 | 0.3620830689655172 | 0.564351 |
| 5 | `bevy_transform::systems::mark_dirty_trees` | `PostUpdate` | 29 | 0.262961 | 0.27113134482758616 | 0.32078 |
| 6 | `bevy_camera::visibility::calculate_bounds` | `PostUpdate` | 29 | 0.258616 | 0.2781476896551725 | 0.440744 |
| 7 | `bevy_light::check_dir_light_mesh_visibility` | `PostUpdate` | 29 | 0.209357 | 0.23610120689655173 | 0.36232 |
| 8 | `bevy_camera::visibility::check_visibility_cpu_culling` | `PostUpdate` | 29 | 0.190234 | 0.19911317241379312 | 0.284364 |
| 9 | `bevy_camera::visibility::reset_view_visibility` | `PostUpdate` | 29 | 0.133798 | 0.13308 | 0.164607 |
| 10 | `bevy_pbr::material::check_entities_needing_specialization<bevy_pbr::pbr_material::StandardMaterial>` | `PostUpdate` | 29 | 0.129381 | 0.14301206896551721 | 0.274238 |
| 11 | `bevy_light::check_point_light_mesh_visibility` | `PostUpdate` | 29 | 0.123929 | 0.12824313793103448 | 0.158896 |
| 12 | `bevy_camera::visibility::mark_newly_hidden_entities_invisible` | `PostUpdate` | 29 | 0.104598 | 0.11562655172413792 | 0.177205 |
| 13 | `bevy_transform::systems::sync_simple_transforms` | `PostUpdate` | 29 | 0.101618 | 0.10455455172413794 | 0.136575 |
| 14 | `bevy_picking::events::pointer_events` | `PreUpdate` | 29 | 0.080498 | 0.0834045172413793 | 0.11276 |
| 15 | `bevy_pbr::material::mark_meshes_as_changed_if_their_materials_changed<bevy_pbr::pbr_material::StandardMaterial>` | `PostUpdate` | 29 | 0.080059 | 0.08526231034482759 | 0.105783 |

### Main, on2

| rank | system | schedule | count | median ms | mean ms | p95 ms |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | `bevy_time::fixed::run_fixed_main_schedule` | `RunFixedMainLoop` | 29 | 3.939641 | 4.087213034482759 | 4.856469 |
| 2 | `powrush_client::sky_backdrop::tests::vis_perf_observe` | `Last` | 29 | 2.720407 | 2.7292470689655173 | 3.057822 |
| 3 | `bevy_pbr::material::check_entities_needing_specialization<bevy_pbr::extended_material::ExtendedMaterial<bevy_pbr::pbr_material::StandardMaterial, powrush_client::painterly_shader::PainterlyExtension>>` | `PostUpdate` | 29 | 0.369489 | 0.3663993103448276 | 0.501617 |
| 4 | `bevy_pbr::prepass::update_mesh_previous_global_transforms` | `PreUpdate` | 29 | 0.350296 | 0.4408666206896552 | 0.798587 |
| 5 | `bevy_transform::systems::mark_dirty_trees` | `PostUpdate` | 29 | 0.274162 | 0.27792986206896547 | 0.315093 |
| 6 | `bevy_camera::visibility::calculate_bounds` | `PostUpdate` | 29 | 0.268139 | 0.27329686206896553 | 0.37942 |
| 7 | `bevy_light::check_dir_light_mesh_visibility` | `PostUpdate` | 29 | 0.208732 | 0.287448275862069 | 0.383319 |
| 8 | `bevy_camera::visibility::check_visibility_cpu_culling` | `PostUpdate` | 29 | 0.197812 | 0.2089806896551724 | 0.284482 |
| 9 | `bevy_pbr::material::check_entities_needing_specialization<bevy_pbr::pbr_material::StandardMaterial>` | `PostUpdate` | 29 | 0.139767 | 0.15328293103448273 | 0.214494 |
| 10 | `bevy_camera::visibility::reset_view_visibility` | `PostUpdate` | 29 | 0.130049 | 0.13221879310344828 | 0.15584 |
| 11 | `bevy_light::check_point_light_mesh_visibility` | `PostUpdate` | 29 | 0.123555 | 0.12594006896551724 | 0.143493 |
| 12 | `bevy_camera::visibility::mark_newly_hidden_entities_invisible` | `PostUpdate` | 29 | 0.107363 | 0.11486837931034483 | 0.180838 |
| 13 | `bevy_transform::systems::sync_simple_transforms` | `PostUpdate` | 29 | 0.100264 | 0.100974 | 0.127182 |
| 14 | `bevy_picking::events::pointer_events` | `PreUpdate` | 29 | 0.090149 | 0.0903620344827586 | 0.109694 |
| 15 | `bevy_ui::picking_backend::ui_picking` | `PreUpdate` | 29 | 0.084142 | 0.08006503448275862 | 0.100137 |

### Render, on1

| rank | system | schedule | count | median ms | mean ms | p95 ms |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | `bevy_render::run_render_schedule` | `RenderRecovery` | 29 | 127.888345 | 129.44967579310347 | 136.040999 |
| 2 | `bevy_render::renderer::render_system` | `Render` | 29 | 106.842862 | 107.9089162413793 | 113.073527 |
| 3 | `bevy_core_pipeline::schedule::submit_pending_command_buffers` | `RenderGraph` | 29 | 100.931174 | 88.712001 | 107.139635 |
| 4 | `bevy_core_pipeline::schedule::camera_driver` | `RenderGraph` | 29 | 3.885619 | 3.9384401724137934 | 4.473436 |
| 5 | `bevy_pbr::cluster::gpu::prepare_clusters_for_gpu_clustering` | `Render` | 29 | 2.38312 | 2.4115469655172417 | 2.72847 |
| 6 | `bevy_pbr::render::gpu_preprocess::prepare_preprocess_bind_groups` | `Render` | 29 | 2.279258 | 2.3878822068965517 | 3.186923 |
| 7 | `bevy_render::batching::gpu_preprocessing::write_binned_instance_buffers<bevy_pbr::render::light::Shadow, bevy_pbr::render::mesh::MeshPipeline>` | `Render` | 29 | 1.933615 | 2.0028257586206895 | 2.599162 |
| 8 | `bevy_render::batching::gpu_preprocessing::write_binned_instance_buffers<bevy_core_pipeline::core_3d::Opaque3d, bevy_pbr::render::mesh::MeshPipeline>` | `Render` | 29 | 1.055151 | 1.1337606206896553 | 1.413228 |
| 9 | `bevy_extract::extract_plugin::apply_extract_commands` | `Render` | 29 | 1.006642 | 1.0251057586206895 | 1.225717 |
| 10 | `bevy_render::batching::gpu_preprocessing::write_indirect_parameters_buffers` | `Render` | 29 | 0.930862 | 0.9538815172413793 | 1.087704 |
| 11 | `bevy_pbr::render::mesh::extract_meshes_for_gpu_building` | `ExtractSchedule` | 29 | 0.919503 | 0.9190726206896552 | 1.245272 |
| 12 | `bevy_render::batching::gpu_preprocessing::write_batched_instance_buffers<bevy_pbr::render::mesh::MeshPipeline>` | `Render` | 29 | 0.91692 | 0.8466749310344825 | 1.073651 |
| 13 | `bevy_pbr::render::light::per_view_shadow_pass<false>` | `Core3d` | 29 | 0.787349 | 0.8348884482758621 | 1.204122 |
| 14 | `bevy_pbr::render::light::prepare_lights` | `Render` | 29 | 0.529965 | 0.5412756896551724 | 0.6395 |
| 15 | `bevy_pbr::render::mesh::prepare_mesh_bind_groups` | `Render` | 29 | 0.46814 | 0.47498575862068965 | 0.570422 |

### Render, on2

| rank | system | schedule | count | median ms | mean ms | p95 ms |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | `bevy_render::run_render_schedule` | `RenderRecovery` | 29 | 127.002146 | 128.3000912413793 | 136.530934 |
| 2 | `bevy_render::renderer::render_system` | `Render` | 29 | 106.385719 | 107.6802896551724 | 114.961733 |
| 3 | `bevy_core_pipeline::schedule::submit_pending_command_buffers` | `RenderGraph` | 29 | 100.649889 | 95.07028499999998 | 108.97896 |
| 4 | `bevy_core_pipeline::schedule::camera_driver` | `RenderGraph` | 29 | 3.802746 | 3.8650772758620677 | 4.242403 |
| 5 | `bevy_pbr::cluster::gpu::prepare_clusters_for_gpu_clustering` | `Render` | 29 | 2.366363 | 2.385342482758621 | 2.644451 |
| 6 | `bevy_pbr::render::gpu_preprocess::prepare_preprocess_bind_groups` | `Render` | 29 | 2.18981 | 2.210832689655173 | 2.372995 |
| 7 | `bevy_render::batching::gpu_preprocessing::write_binned_instance_buffers<bevy_pbr::render::light::Shadow, bevy_pbr::render::mesh::MeshPipeline>` | `Render` | 29 | 1.95374 | 1.965596965517241 | 2.200908 |
| 8 | `bevy_render::batching::gpu_preprocessing::write_binned_instance_buffers<bevy_core_pipeline::core_3d::Opaque3d, bevy_pbr::render::mesh::MeshPipeline>` | `Render` | 29 | 1.065327 | 1.0804250000000002 | 1.211702 |
| 9 | `bevy_extract::extract_plugin::apply_extract_commands` | `Render` | 29 | 1.039788 | 1.0488696206896553 | 1.173168 |
| 10 | `bevy_pbr::render::mesh::extract_meshes_for_gpu_building` | `ExtractSchedule` | 29 | 0.905455 | 0.9001706896551727 | 1.314589 |
| 11 | `bevy_render::batching::gpu_preprocessing::write_indirect_parameters_buffers` | `Render` | 29 | 0.900061 | 0.9178834137931033 | 1.02088 |
| 12 | `bevy_render::batching::gpu_preprocessing::write_batched_instance_buffers<bevy_pbr::render::mesh::MeshPipeline>` | `Render` | 29 | 0.897479 | 0.8761312758620693 | 0.975663 |
| 13 | `bevy_pbr::render::light::per_view_shadow_pass<false>` | `Core3d` | 29 | 0.777143 | 0.803937448275862 | 0.924632 |
| 14 | `bevy_pbr::render::light::prepare_lights` | `Render` | 29 | 0.544851 | 0.5563816206896552 | 0.629983 |
| 15 | `bevy_pbr::render::mesh::prepare_mesh_bind_groups` | `Render` | 29 | 0.466854 | 0.4725976551724138 | 0.517965 |

The two main orders differ, and the two render orders differ. The complete 274 and 408 rows, with min and max, are `summary.json` keys `cpu_systems.on1` and `cpu_systems.on2`. Rank movement between the two ON windows is not an ablation.

## Render pass rankings

GPU status: `PRESENT`. Counted `elapsed_gpu` samples with a value: 928. Measured zeros: 0. Absence would have been zero samples, not a stored 0. GPU timestamps were present on this lavapipe ICD, so this evidence does not ask Hands or Core for an O2 ablation authorization.

CPU pass rank and GPU pass rank are separate. A whole pass is not a draw. Unit is milliseconds, which is the diagnostic suffix.

CPU `elapsed_cpu` order:

- off1: `render/per_view_shadow_pass`, `render/main_opaque_pass_3d`, `render/clustering`, `render/uniform_allocation`, `render/early_mesh_preprocessing`, `render/bin_unpacking`, `render/upscaling`, `render/auxiliary_views`
- on1: `render/per_view_shadow_pass`, `render/main_opaque_pass_3d`, `render/clustering`, `render/uniform_allocation`, `render/early_mesh_preprocessing`, `render/bin_unpacking`, `render/upscaling`, `render/auxiliary_views`
- off2: `render/per_view_shadow_pass`, `render/main_opaque_pass_3d`, `render/clustering`, `render/uniform_allocation`, `render/early_mesh_preprocessing`, `render/bin_unpacking`, `render/upscaling`, `render/auxiliary_views`
- on2: `render/per_view_shadow_pass`, `render/main_opaque_pass_3d`, `render/clustering`, `render/uniform_allocation`, `render/bin_unpacking`, `render/early_mesh_preprocessing`, `render/upscaling`, `render/auxiliary_views`

GPU `elapsed_gpu` order:

- off1: `render/main_opaque_pass_3d`, `render/per_view_shadow_pass`, `render/upscaling`, `render/uniform_allocation`, `render/clustering`, `render/bin_unpacking`, `render/early_mesh_preprocessing`, `render/auxiliary_views`
- on1: `render/main_opaque_pass_3d`, `render/per_view_shadow_pass`, `render/upscaling`, `render/uniform_allocation`, `render/clustering`, `render/bin_unpacking`, `render/early_mesh_preprocessing`, `render/auxiliary_views`
- off2: `render/main_opaque_pass_3d`, `render/per_view_shadow_pass`, `render/upscaling`, `render/uniform_allocation`, `render/clustering`, `render/bin_unpacking`, `render/early_mesh_preprocessing`, `render/auxiliary_views`
- on2: `render/main_opaque_pass_3d`, `render/per_view_shadow_pass`, `render/upscaling`, `render/uniform_allocation`, `render/clustering`, `render/bin_unpacking`, `render/early_mesh_preprocessing`, `render/auxiliary_views`

CPU order is not the GPU order. On the CPU ranking, `render/per_view_shadow_pass` leads and `render/main_opaque_pass_3d` is second in every settled window. On the GPU ranking, `render/main_opaque_pass_3d` leads and `render/per_view_shadow_pass` is second in every settled window. on1 CPU swaps `early_mesh_preprocessing` above `bin_unpacking` relative to on2; the GPU order is the same in on1 and on2.

### elapsed_cpu

| window | rank | pass | count | median | mean | min | max | p95 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| off1 | 1 | `render/per_view_shadow_pass` | 29 | 0.71134 | 0.7390455172413793 | 0.642551 | 0.912658 | 0.876769 |
| off1 | 2 | `render/main_opaque_pass_3d` | 29 | 0.25995199999999996 | 0.2572709310344828 | 0.207429 | 0.329137 | 0.299954 |
| off1 | 3 | `render/clustering` | 29 | 0.124768 | 0.12301648275862069 | 0.09028800000000001 | 0.188021 | 0.15375999999999998 |
| off1 | 4 | `render/uniform_allocation` | 29 | 0.081497 | 0.08394358620689656 | 0.060659000000000005 | 0.12161 | 0.11872100000000001 |
| off1 | 5 | `render/early_mesh_preprocessing` | 29 | 0.040093 | 0.0427294827586207 | 0.033711000000000005 | 0.072011 | 0.06216800000000001 |
| off1 | 6 | `render/bin_unpacking` | 29 | 0.031010999999999997 | 0.037730103448275855 | 0.026208000000000002 | 0.06179899999999999 | 0.056304999999999994 |
| off1 | 7 | `render/upscaling` | 29 | 0.027713 | 0.028220620689655175 | 0.023473 | 0.035117 | 0.034375 |
| off1 | 8 | `render/auxiliary_views` | 29 | 0.023621999999999997 | 0.024786689655172417 | 0.020380000000000002 | 0.039459 | 0.036975 |
| on1 | 1 | `render/per_view_shadow_pass` | 29 | 0.743606 | 0.7843522413793104 | 0.65962 | 1.233503 | 1.119776 |
| on1 | 2 | `render/main_opaque_pass_3d` | 29 | 0.24514300000000003 | 0.24949900000000003 | 0.210834 | 0.35935900000000004 | 0.349367 |
| on1 | 3 | `render/clustering` | 29 | 0.09752200000000001 | 0.10031137931034481 | 0.084522 | 0.126804 | 0.126622 |
| on1 | 4 | `render/uniform_allocation` | 29 | 0.075103 | 0.07607717241379308 | 0.059095 | 0.098143 | 0.097735 |
| on1 | 5 | `render/early_mesh_preprocessing` | 29 | 0.050589 | 0.047535862068965526 | 0.034375 | 0.076554 | 0.060696 |
| on1 | 6 | `render/bin_unpacking` | 29 | 0.032314999999999997 | 0.03725437931034482 | 0.027229999999999997 | 0.061388000000000005 | 0.058628 |
| on1 | 7 | `render/upscaling` | 29 | 0.030198000000000003 | 0.03258379310344828 | 0.026798 | 0.050444 | 0.045880000000000004 |
| on1 | 8 | `render/auxiliary_views` | 29 | 0.022677 | 0.023056931034482756 | 0.019701999999999997 | 0.0336 | 0.033089 |
| off2 | 1 | `render/per_view_shadow_pass` | 29 | 0.7371530000000001 | 0.7542922413793103 | 0.635437 | 0.9805809999999999 | 0.964854 |
| off2 | 2 | `render/main_opaque_pass_3d` | 29 | 0.239092 | 0.24943824137931034 | 0.199354 | 0.381398 | 0.328537 |
| off2 | 3 | `render/clustering` | 29 | 0.111761 | 0.11151844827586205 | 0.081355 | 0.139934 | 0.132628 |
| off2 | 4 | `render/uniform_allocation` | 29 | 0.07947 | 0.08402375862068966 | 0.060025999999999996 | 0.13001000000000001 | 0.113708 |
| off2 | 5 | `render/early_mesh_preprocessing` | 29 | 0.039428000000000005 | 0.042960344827586205 | 0.033685 | 0.0776 | 0.063786 |
| off2 | 6 | `render/bin_unpacking` | 29 | 0.032263 | 0.03840124137931036 | 0.026624000000000002 | 0.059242 | 0.053338 |
| off2 | 7 | `render/upscaling` | 29 | 0.028839 | 0.029803758620689656 | 0.024797 | 0.044227 | 0.036504999999999996 |
| off2 | 8 | `render/auxiliary_views` | 29 | 0.022334999999999997 | 0.02292962068965517 | 0.019093000000000002 | 0.032006 | 0.029514 |
| on2 | 1 | `render/per_view_shadow_pass` | 29 | 0.7383179999999999 | 0.7899835172413793 | 0.670643 | 1.351449 | 1.1310230000000001 |
| on2 | 2 | `render/main_opaque_pass_3d` | 29 | 0.260283 | 0.258085275862069 | 0.21363600000000002 | 0.312227 | 0.304147 |
| on2 | 3 | `render/clustering` | 29 | 0.11049400000000001 | 0.10959996551724138 | 0.072541 | 0.19645 | 0.125585 |
| on2 | 4 | `render/uniform_allocation` | 29 | 0.067547 | 0.07230906896551724 | 0.056341 | 0.10886599999999999 | 0.101996 |
| on2 | 5 | `render/bin_unpacking` | 29 | 0.048374999999999994 | 0.045433172413793105 | 0.027755000000000002 | 0.10816500000000001 | 0.064628 |
| on2 | 6 | `render/early_mesh_preprocessing` | 29 | 0.041898 | 0.044514413793103454 | 0.034092 | 0.06996999999999999 | 0.061724999999999995 |
| on2 | 7 | `render/upscaling` | 29 | 0.029656 | 0.030410551724137935 | 0.026146 | 0.037997 | 0.036634 |
| on2 | 8 | `render/auxiliary_views` | 29 | 0.024790000000000003 | 0.025178068965517244 | 0.020487 | 0.037338 | 0.031560000000000005 |

### elapsed_gpu

| window | rank | pass | count | median | mean | min | max | p95 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| off1 | 1 | `render/main_opaque_pass_3d` | 29 | 61.509905 | 62.765700827586215 | 58.420504 | 70.306013 | 69.221317 |
| off1 | 2 | `render/per_view_shadow_pass` | 29 | 27.878311 | 29.16563613793104 | 25.963637 | 35.882799 | 35.558156 |
| off1 | 3 | `render/upscaling` | 29 | 4.888784 | 4.970714448275862 | 4.176696 | 6.691254 | 6.013288 |
| off1 | 4 | `render/uniform_allocation` | 29 | 0.973365 | 1.0007202068965517 | 0.947253 | 1.136319 | 1.131723 |
| off1 | 5 | `render/clustering` | 29 | 0.52597 | 0.5588274137931035 | 0.49086 | 0.765999 | 0.682789 |
| off1 | 6 | `render/bin_unpacking` | 29 | 0.31922 | 0.32167368965517246 | 0.287972 | 0.36488 | 0.36311 |
| off1 | 7 | `render/early_mesh_preprocessing` | 29 | 0.187161 | 0.19486217241379308 | 0.168475 | 0.324962 | 0.229545 |
| off1 | 8 | `render/auxiliary_views` | 29 | 0.060641 | 0.10986824137931037 | 0.047267 | 0.849275 | 0.234786 |
| on1 | 1 | `render/main_opaque_pass_3d` | 29 | 60.269738 | 61.37701989655172 | 57.990406 | 76.739274 | 64.205698 |
| on1 | 2 | `render/per_view_shadow_pass` | 29 | 28.249062 | 28.506472137931034 | 24.739795 | 33.916098 | 30.749375 |
| on1 | 3 | `render/upscaling` | 29 | 4.762156 | 4.928487862068964 | 4.325823 | 7.052988 | 5.88105 |
| on1 | 4 | `render/uniform_allocation` | 29 | 0.97447 | 1.0430292758620687 | 0.934145 | 1.466177 | 1.441155 |
| on1 | 5 | `render/clustering` | 29 | 0.507568 | 0.5258780344827586 | 0.4688 | 0.734589 | 0.603837 |
| on1 | 6 | `render/bin_unpacking` | 29 | 0.300452 | 0.31229393103448283 | 0.28202 | 0.378334 | 0.377882 |
| on1 | 7 | `render/early_mesh_preprocessing` | 29 | 0.181972 | 0.19232279310344827 | 0.171351 | 0.365855 | 0.261902 |
| on1 | 8 | `render/auxiliary_views` | 29 | 0.060576 | 0.09929579310344827 | 0.04599 | 0.506945 | 0.282082 |
| off2 | 1 | `render/main_opaque_pass_3d` | 29 | 61.48668 | 61.62533065517241 | 57.295119 | 69.35419 | 66.643897 |
| off2 | 2 | `render/per_view_shadow_pass` | 29 | 28.091321 | 28.968645137931038 | 25.789445 | 39.994784 | 34.925911 |
| off2 | 3 | `render/upscaling` | 29 | 4.586599 | 4.854274172413794 | 4.108988 | 9.14241 | 5.930761 |
| off2 | 4 | `render/uniform_allocation` | 29 | 0.96776 | 0.9891558965517243 | 0.943465 | 1.157174 | 1.101953 |
| off2 | 5 | `render/clustering` | 29 | 0.5236 | 0.5624475172413791 | 0.481708 | 0.900256 | 0.814084 |
| off2 | 6 | `render/bin_unpacking` | 29 | 0.313285 | 0.3146340689655172 | 0.277712 | 0.362166 | 0.343302 |
| off2 | 7 | `render/early_mesh_preprocessing` | 29 | 0.181662 | 0.18554955172413795 | 0.173001 | 0.23228 | 0.213568 |
| off2 | 8 | `render/auxiliary_views` | 29 | 0.063304 | 0.09876020689655171 | 0.044644 | 0.307114 | 0.229501 |
| on2 | 1 | `render/main_opaque_pass_3d` | 29 | 60.104445 | 60.83727637931034 | 57.89129 | 66.615923 | 65.850363 |
| on2 | 2 | `render/per_view_shadow_pass` | 29 | 28.493959 | 28.81184648275862 | 26.182077 | 34.585572 | 31.409836 |
| on2 | 3 | `render/upscaling` | 29 | 4.688515 | 4.78520896551724 | 4.177658 | 6.063361 | 5.771587 |
| on2 | 4 | `render/uniform_allocation` | 29 | 0.968722 | 1.0348807241379312 | 0.944733 | 1.877752 | 1.383411 |
| on2 | 5 | `render/clustering` | 29 | 0.50348 | 0.5298788965517242 | 0.477112 | 0.794423 | 0.740694 |
| on2 | 6 | `render/bin_unpacking` | 29 | 0.307364 | 0.3083715172413793 | 0.281311 | 0.356846 | 0.337519 |
| on2 | 7 | `render/early_mesh_preprocessing` | 29 | 0.18213 | 0.18573044827586208 | 0.171151 | 0.221018 | 0.203008 |
| on2 | 8 | `render/auxiliary_views` | 29 | 0.06123 | 0.10169596551724139 | 0.045358 | 0.231385 | 0.228047 |

## Other diagnostic paths

Every counted path is in `diagnostic_path_counts`. Metrics other than `elapsed_cpu` and `elapsed_gpu` are pipeline or load counters, unit count, reported only where a sample existed. `started_load_count` has no slash, so its metric field is empty and its pass field is the whole path. It is included, not dropped.

| metric | pass | on1 median | on1 min | on1 max | same median all four windows |
| --- | --- | --- | --- | --- | --- |
| `(path has no suffix)` | `started_load_count` | 124.0 | 124.0 | 124.0 | True |
| `clipper_invocations` | `render/main_opaque_pass_3d` | 66652.0 | 66652.0 | 66652.0 | True |
| `clipper_primitives_out` | `render/main_opaque_pass_3d` | 7484.0 | 7484.0 | 7484.0 | True |
| `compute_shader_invocations` | `render/bin_unpacking` | 1728.0 | 1728.0 | 1728.0 | True |
| `compute_shader_invocations` | `render/early_mesh_preprocessing` | 640.0 | 640.0 | 640.0 | True |
| `compute_shader_invocations` | `render/uniform_allocation` | 6912.0 | 6912.0 | 6912.0 | True |
| `fragment_shader_invocations` | `render/main_opaque_pass_3d` | 2745365.0 | 2745365.0 | 2745365.0 | True |
| `vertex_shader_invocations` | `render/main_opaque_pass_3d` | 74113.0 | 74113.0 | 74113.0 | True |

Each of those rows has count 29 in each settled window. Shader and primitive counters are the values the diagnostics store returned. They were not estimated.

Age and lag live on each `diagnostics.jsonl` row. This pair's diagnostic `age_frames` values are all 0. `lag_ns` is the observer `Last` instant minus `DiagnosticMeasurement.time`.

## Status

`summary.json` status is `ok`. Problems: []. GPU `PRESENT`.

## Restored gate

Restored tree, ordinary default, no `--ignored`:

- `cargo test -p shared -p rsil-identity` exited 0. `rsil-identity` 5 passed. `shared` 497 passed.
- `cargo test -p powrush-client --lib` exited 0. 1358 passed, 0 failed, 4 ignored. The ignored tests are the ones already ignored on the base. The temporary VIS-PERF-2 test is not in this tree.

## Repeatable observer

Paste the following inside `client/src/sky_backdrop.rs` module `tests` (that module already has `use super::*`). Add Bevy features `trace` and `trace_chrome` for the run only. Restore both files and `Cargo.lock` afterwards. The test is `#[ignore]`.

```rust
    // TEMPORARY VIS-PERF-2 observer. Removed before the evidence commit.
    // Subscriber and RenderDiagnosticsPlugin stay installed for the whole run.
    // OFF/ON flips only the span sink. Pass diagnostics stay on in every window.

    use std::collections::{BTreeMap, BTreeSet, HashSet};
    use std::io::{BufWriter, Write};
    use std::path::{Path, PathBuf};
    use std::sync::{Mutex, OnceLock};
    use std::time::Instant;

    use bevy::diagnostic::DiagnosticsStore;
    use bevy::ecs::message::MessageWriter;
    use bevy::log::tracing_subscriber::layer::Context;
    use bevy::log::tracing_subscriber::registry::LookupSpan;
    use bevy::log::tracing_subscriber::Layer;
    use bevy::render::render_resource::WgpuFeatures;
    use bevy::window::{PresentMode, PrimaryWindow, WindowResolution};
    use serde::Serialize;
    use tracing::field::{Field, Visit};
    use tracing::span::{Attributes, Id};
    use tracing::Subscriber;

    const STARTUP_FRAMES: u64 = 100;
    const PREROLL_FRAMES: u64 = 48;
    const SETTLED_FRAMES: u64 = 29;
    const WARM_FRAMES: u64 = 20;

    fn phase_at(frame: u64) -> (&'static str, bool, bool, u64) {
        const STEPS: &[(&str, u64, bool, bool)] = &[
            ("startup", STARTUP_FRAMES, false, false),
            ("preroll", PREROLL_FRAMES, false, false),
            ("off1", SETTLED_FRAMES, true, false),
            ("on1_warm", WARM_FRAMES, false, true),
            ("on1", SETTLED_FRAMES, true, true),
            ("off2", SETTLED_FRAMES, true, false),
            ("on2_warm", WARM_FRAMES, false, true),
            ("on2", SETTLED_FRAMES, true, true),
        ];
        let mut start = 0u64;
        for (name, len, counted, sink) in STEPS {
            if frame < start + len {
                return (*name, *counted, *sink, frame - start);
            }
            start += *len;
        }
        ("tail", false, false, frame.saturating_sub(start))
    }

    fn emit_phase(phase: &str) -> bool {
        matches!(
            phase,
            "off1" | "on1_warm" | "on1" | "off2" | "on2_warm" | "on2"
        )
    }

    struct SpanSlot {
        name: Option<String>,
    }

    struct NameVisit<'a>(&'a mut Option<String>);

    impl Visit for NameVisit<'_> {
        fn record_str(&mut self, field: &Field, value: &str) {
            if field.name() == "name" {
                *self.0 = Some(value.to_string());
            }
        }

        fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
            if field.name() == "name" && self.0.is_none() {
                *self.0 = Some(format!("{value:?}"));
            }
        }
    }

    struct MsgVisit<'a>(&'a mut String);

    impl Visit for MsgVisit<'_> {
        fn record_str(&mut self, field: &Field, value: &str) {
            if !self.0.is_empty() {
                self.0.push(' ');
            }
            self.0.push_str(field.name());
            self.0.push('=');
            self.0.push_str(value);
        }

        fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
            if !self.0.is_empty() {
                self.0.push(' ');
            }
            self.0.push_str(field.name());
            self.0.push('=');
            self.0.push_str(&format!("{value:?}"));
        }
    }

    #[derive(Clone, Serialize)]
    struct SpanRec {
        frame: u64,
        phase: String,
        phase_index: u64,
        counted: bool,
        meta: String,
        target: String,
        field_name: String,
        schedule: String,
        app: String,
        thread_name: String,
        thread_id: String,
        span_id: u64,
        parent_id: Option<u64>,
        parent_meta: String,
        elapsed_ns: u64,
    }

    #[derive(Clone, Serialize)]
    struct DiagRec {
        frame: u64,
        phase: String,
        phase_index: u64,
        counted: bool,
        path: String,
        pass: String,
        metric: String,
        suffix: String,
        value: Option<f64>,
        measured_zero: bool,
        measurement_ns: u64,
        lag_ns: u64,
        age_frames: u32,
    }

    #[derive(Clone, Serialize)]
    struct GapRec {
        frame: u64,
        phase: String,
        phase_index: u64,
        counted: bool,
        gap_ns: u64,
        physical_w: u32,
        physical_h: u32,
        present_mode: String,
    }

    struct SceneCounts {
        sky: usize,
        cameras: usize,
        meshes: usize,
    }

    struct WinSeen {
        frames: u64,
        min_w: u32,
        max_w: u32,
        min_h: u32,
        max_h: u32,
        modes: BTreeSet<String>,
        scales: BTreeSet<String>,
        logical: BTreeSet<String>,
        first: String,
        last: String,
    }

    struct RunState {
        armed: bool,
        frozen: bool,
        frame_next: u64,
        frame: u64,
        phase: String,
        phase_index: u64,
        counted: bool,
        sink: bool,
        last_calls: u64,
        schedule_stack: Vec<String>,
        app_stack: Vec<String>,
        entered: std::collections::HashMap<u64, Instant>,
        spans: Vec<SpanRec>,
        diags: Vec<DiagRec>,
        gaps: Vec<GapRec>,
        seen: HashSet<(String, u64)>,
        prev_observe: Option<Instant>,
        logs: Vec<String>,
        span_misses: u64,
        abort: Option<String>,
        scene: Option<SceneCounts>,
        scene_note: Option<String>,
        win: Option<WinSeen>,
        adapter: Option<String>,
        features: Option<String>,
        timestamp_period_ns: Option<f32>,
        flag_timestamp_query: Option<bool>,
        flag_timestamp_inside_passes: Option<bool>,
        flag_timestamp_inside_encoders: Option<bool>,
        flag_pipeline_statistics: Option<bool>,
        presets: BTreeSet<String>,
        nan_diags: u64,
        notes: BTreeMap<String, String>,
    }

    impl RunState {
        fn new() -> Self {
            Self {
                armed: false,
                frozen: false,
                frame_next: 0,
                frame: 0,
                phase: "before".into(),
                phase_index: 0,
                counted: false,
                sink: false,
                last_calls: 0,
                schedule_stack: Vec::new(),
                app_stack: Vec::new(),
                entered: std::collections::HashMap::new(),
                spans: Vec::new(),
                diags: Vec::new(),
                gaps: Vec::new(),
                seen: HashSet::new(),
                prev_observe: None,
                logs: Vec::new(),
                span_misses: 0,
                abort: None,
                scene: None,
                scene_note: None,
                win: None,
                adapter: None,
                features: None,
                timestamp_period_ns: None,
                flag_timestamp_query: None,
                flag_timestamp_inside_passes: None,
                flag_timestamp_inside_encoders: None,
                flag_pipeline_statistics: None,
                presets: BTreeSet::new(),
                nan_diags: 0,
                notes: BTreeMap::new(),
            }
        }
    }

    static STATE: OnceLock<Mutex<RunState>> = OnceLock::new();
    static T0: OnceLock<Instant> = OnceLock::new();

    fn state() -> &'static Mutex<RunState> {
        STATE.get().expect("vis-perf state")
    }

    fn t0() -> Instant {
        *T0.get().expect("vis-perf t0")
    }

    fn lock_state() -> std::sync::MutexGuard<'static, RunState> {
        state().lock().unwrap_or_else(|err| err.into_inner())
    }

    struct VisPerfLayer;

    fn vis_perf_layer(_app: &mut App) -> Option<bevy::log::BoxedLayer> {
        Some(Box::new(VisPerfLayer))
    }

    fn stack_kind(meta: &str, target: &str) -> Option<&'static str> {
        if target != "bevy_app::sub_app" && !target.starts_with("bevy_ecs::schedule") {
            return None;
        }
        match meta {
            "schedule" if target.starts_with("bevy_ecs::schedule") => Some("schedule"),
            "main app" if target == "bevy_app::sub_app" => Some("main"),
            "sub app" if target == "bevy_app::sub_app" => Some("sub"),
            _ => None,
        }
    }

    impl<S> Layer<S> for VisPerfLayer
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
            let mut name = None;
            attrs.record(&mut NameVisit(&mut name));
            if let Some(span) = ctx.span(id) {
                span.extensions_mut().insert(SpanSlot { name });
            }
        }

        fn on_record(&self, id: &Id, values: &tracing::span::Record<'_>, ctx: Context<'_, S>) {
            let mut name = None;
            values.record(&mut NameVisit(&mut name));
            if name.is_none() {
                return;
            }
            if let Some(span) = ctx.span(id) {
                if let Some(slot) = span.extensions_mut().get_mut::<SpanSlot>() {
                    if slot.name.is_none() {
                        slot.name = name;
                    }
                } else {
                    span.extensions_mut().insert(SpanSlot { name });
                }
            }
        }

        fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
            let Some(span) = ctx.span(id) else {
                return;
            };
            let meta = span.metadata().name();
            let target = span.metadata().target();
            let field_name = span
                .extensions()
                .get::<SpanSlot>()
                .and_then(|slot| slot.name.clone())
                .unwrap_or_default();
            let now = Instant::now();
            let mut run = lock_state();
            run.entered.insert(id.into_u64(), now);
            if run.armed && meta == "update" && target == "bevy_app::sub_app" {
                let frame = run.frame_next;
                run.frame_next = frame.saturating_add(1);
                let (name, counted, sink, index) = phase_at(frame);
                run.frame = frame;
                run.phase = name.to_string();
                run.phase_index = index;
                run.counted = counted;
                run.sink = sink;
            }
            match stack_kind(meta, target) {
                Some("schedule") => run.schedule_stack.push(field_name),
                Some("main") => run.app_stack.push("main".into()),
                Some("sub") => run.app_stack.push(if field_name.is_empty() {
                    "sub app".into()
                } else {
                    field_name
                }),
                _ => {}
            }
        }

        fn on_exit(&self, id: &Id, ctx: Context<'_, S>) {
            let Some(span) = ctx.span(id) else {
                return;
            };
            let meta = span.metadata().name().to_string();
            let target = span.metadata().target().to_string();
            let field_name = span
                .extensions()
                .get::<SpanSlot>()
                .and_then(|slot| slot.name.clone())
                .unwrap_or_default();
            let parent = span.parent().map(|parent| {
                (
                    parent.id().into_u64(),
                    parent.metadata().name().to_string(),
                )
            });
            let now = Instant::now();
            let mut run = lock_state();
            let entered = run.entered.remove(&id.into_u64());
            match stack_kind(&meta, &target) {
                Some("schedule") => {
                    run.schedule_stack.pop();
                }
                Some("main") | Some("sub") => {
                    run.app_stack.pop();
                }
                _ => {}
            }
            if !run.armed || run.frozen || !run.sink {
                return;
            }
            let Some(entered) = entered else {
                run.span_misses = run.span_misses.saturating_add(1);
                return;
            };
            let schedule = run
                .schedule_stack
                .last()
                .cloned()
                .unwrap_or_default();
            let app = run.app_stack.last().cloned().unwrap_or_default();
            let (parent_id, parent_meta) = match parent {
                Some((pid, pmeta)) => (Some(pid), pmeta),
                None => (None, String::new()),
            };
            let thread = std::thread::current();
            let rec = SpanRec {
                frame: run.frame,
                phase: run.phase.clone(),
                phase_index: run.phase_index,
                counted: run.counted,
                meta,
                target,
                field_name,
                schedule,
                app,
                thread_name: thread.name().unwrap_or("").to_string(),
                thread_id: format!("{:?}", thread.id()),
                span_id: id.into_u64(),
                parent_id,
                parent_meta,
                elapsed_ns: now.saturating_duration_since(entered).as_nanos() as u64,
            };
            run.spans.push(rec);
        }

        fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<'_, S>) {
            let mut msg = String::new();
            event.record(&mut MsgVisit(&mut msg));
            let target = event.metadata().target();
            let interesting = msg.contains("PresentMode")
                || msg.contains("present mode")
                || msg.contains("Adapter")
                || msg.contains("lavapipe")
                || msg.contains("llvmpipe")
                || target.contains("bevy_render::renderer");
            if !interesting || msg.is_empty() {
                return;
            }
            let line = format!("{} {target}: {msg}", event.metadata().level());
            let mut run = lock_state();
            if run.logs.len() < 80 {
                run.logs.push(line);
            }
        }
    }

    fn split_path(path: &str) -> (String, String) {
        match path.rfind('/') {
            Some(idx) => (path[..idx].to_string(), path[idx + 1..].to_string()),
            None => (path.to_string(), String::new()),
        }
    }

    fn out_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../docs/visual-pass/vis-perf-2")
    }

    fn sha256_file(path: &Path) -> String {
        let output = std::process::Command::new("sha256sum")
            .arg(path)
            .output()
            .expect("sha256sum");
        String::from_utf8_lossy(&output.stdout)
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_string()
    }

    fn git_blob(path: &Path) -> String {
        let output = std::process::Command::new("git")
            .args(["hash-object", "--"])
            .arg(path)
            .output()
            .expect("git hash-object");
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    fn cmd_out(cmd: &str, args: &[&str]) -> String {
        match std::process::Command::new(cmd).args(args).output() {
            Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
            Err(err) => format!("{cmd} failed: {err}"),
        }
    }

    fn lock_pkg_version(pkg: &str) -> String {
        let lock = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../Cargo.lock");
        let Ok(text) = std::fs::read_to_string(lock) else {
            return String::new();
        };
        let needle = format!("name = \"{pkg}\"");
        let mut lines = text.lines();
        while let Some(line) = lines.next() {
            if line.trim() == needle {
                if let Some(next) = lines.next() {
                    if let Some(rest) = next.trim().strip_prefix("version = \"") {
                        return rest.trim_end_matches('"').to_string();
                    }
                }
            }
        }
        String::new()
    }

    fn cpu_model() -> String {
        let Ok(text) = std::fs::read_to_string("/proc/cpuinfo") else {
            return String::new();
        };
        text.lines()
            .find_map(|line| line.strip_prefix("model name"))
            .and_then(|rest| rest.split(':').nth(1))
            .unwrap_or("")
            .trim()
            .to_string()
    }

    struct EnvGuard {
        key: &'static str,
        prev: Option<String>,
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.prev {
                Some(value) => std::env::set_var(self.key, value),
                None => std::env::remove_var(self.key),
            }
        }
    }

    fn set_env(key: &'static str, value: &str) -> EnvGuard {
        let prev = std::env::var(key).ok();
        std::env::set_var(key, value);
        EnvGuard { key, prev }
    }

    fn stats_f64(values: &[f64]) -> serde_json::Value {
        if values.is_empty() {
            return serde_json::json!({
                "count": 0,
                "mean": null,
                "median": null,
                "p95": null,
                "min": null,
                "max": null,
            });
        }
        let mut sorted = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let n = sorted.len();
        let mean = sorted.iter().sum::<f64>() / n as f64;
        let median = if n % 2 == 1 {
            sorted[n / 2]
        } else {
            (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
        };
        let rank = ((0.95 * n as f64).ceil() as usize).clamp(1, n) - 1;
        serde_json::json!({
            "count": n,
            "mean": mean,
            "median": median,
            "p95": sorted[rank],
            "min": sorted[0],
            "max": sorted[n - 1],
            "p95_method": "nearest-rank ceil(0.95*n) 1-based",
        })
    }

    fn num(value: &serde_json::Value, key: &str) -> Option<f64> {
        value.get(key).and_then(|v| v.as_f64())
    }

    fn sub_stat(later: &serde_json::Value, earlier: &serde_json::Value) -> serde_json::Value {
        let keys = ["mean", "median", "min", "max"];
        let mut obj = serde_json::Map::new();
        for key in keys {
            let delta = match (num(later, key), num(earlier, key)) {
                (Some(a), Some(b)) => serde_json::json!(a - b),
                _ => serde_json::Value::Null,
            };
            obj.insert(key.to_string(), delta);
        }
        serde_json::Value::Object(obj)
    }

    fn app_bucket(app: &str) -> &'static str {
        if app == "main" {
            "main"
        } else if app.to_ascii_lowercase().contains("render") {
            "render"
        } else {
            "other"
        }
    }

    struct RankRow {
        key: String,
        app: String,
        schedule: String,
        name: String,
        bucket: &'static str,
        samples: Vec<f64>,
    }

    fn system_rows(spans: &[SpanRec], phase: &str) -> Vec<RankRow> {
        let mut grouped: BTreeMap<(String, String, String), Vec<f64>> = BTreeMap::new();
        for span in spans {
            if span.phase != phase || !span.counted || span.meta != "system" {
                continue;
            }
            let ms = span.elapsed_ns as f64 / 1_000_000.0;
            grouped
                .entry((span.app.clone(), span.schedule.clone(), span.field_name.clone()))
                .or_default()
                .push(ms);
        }
        let mut rows = Vec::new();
        for ((app, schedule, name), samples) in grouped {
            let bucket = app_bucket(&app);
            let key = format!("{bucket}|{app}|{schedule}|{name}");
            rows.push(RankRow {
                key,
                app,
                schedule,
                name,
                bucket,
                samples,
            });
        }
        rows.sort_by(|a, b| {
            let am = median_only(&a.samples);
            let bm = median_only(&b.samples);
            bm.partial_cmp(&am)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.key.cmp(&b.key))
        });
        rows
    }

    fn median_only(values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        let mut sorted = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let n = sorted.len();
        if n % 2 == 1 {
            sorted[n / 2]
        } else {
            (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
        }
    }

    fn rows_json(rows: &[RankRow], bucket: &str) -> serde_json::Value {
        let mut out = Vec::new();
        let mut rank = 1u64;
        for row in rows.iter().filter(|row| row.bucket == bucket) {
            let stats = stats_f64(&row.samples);
            out.push(serde_json::json!({
                "rank": rank,
                "system": row.name,
                "schedule": row.schedule,
                "app": row.app,
                "unit": "ms",
                "whole_invocation_wall_time": true,
                "stats": stats,
            }));
            rank += 1;
        }
        serde_json::Value::Array(out)
    }

    fn top_names(rows: &[RankRow], bucket: &str, n: usize) -> Vec<String> {
        rows.iter()
            .filter(|row| row.bucket == bucket)
            .take(n)
            .map(|row| format!("{} @ {}", row.name, row.schedule))
            .collect()
    }

    fn pass_rows(diags: &[DiagRec], phase: &str, metric: &str) -> Vec<(String, Vec<f64>)> {
        let mut grouped: BTreeMap<String, Vec<f64>> = BTreeMap::new();
        for diag in diags {
            if diag.phase != phase || !diag.counted || diag.metric != metric {
                continue;
            }
            if let Some(value) = diag.value {
                grouped.entry(diag.pass.clone()).or_default().push(value);
            }
        }
        let mut rows: Vec<(String, Vec<f64>)> = grouped.into_iter().collect();
        rows.sort_by(|a, b| {
            median_only(&b.1)
                .partial_cmp(&median_only(&a.1))
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.0.cmp(&b.0))
        });
        rows
    }

    fn pass_json(rows: &[(String, Vec<f64>)], unit: &str) -> serde_json::Value {
        let mut out = Vec::new();
        for (rank, (pass, samples)) in rows.iter().enumerate() {
            out.push(serde_json::json!({
                "rank": rank + 1,
                "pass": pass,
                "unit": unit,
                "whole_pass_not_draw": true,
                "stats": stats_f64(samples),
            }));
        }
        serde_json::Value::Array(out)
    }

    fn gap_values(gaps: &[GapRec], phase: &str) -> Vec<f64> {
        gaps.iter()
            .filter(|gap| gap.phase == phase && gap.counted)
            .map(|gap| gap.gap_ns as f64 / 1_000_000.0)
            .collect()
    }

    fn write_jsonl(path: &Path, rows: &[impl Serialize]) -> std::io::Result<()> {
        let file = std::fs::File::create(path)?;
        let mut writer = BufWriter::new(file);
        for row in rows {
            serde_json::to_writer(&mut writer, row).map_err(std::io::Error::other)?;
            writer.write_all(b"\n")?;
        }
        writer.flush()
    }

    fn write_gaps(path: &Path, gaps: &[GapRec]) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        writeln!(
            file,
            "frame,phase,phase_index,counted,gap_ns,gap_ms,physical_w,physical_h,present_mode"
        )?;
        for gap in gaps {
            writeln!(
                file,
                "{},{},{},{},{},{:.6},{},{},{}",
                gap.frame,
                gap.phase,
                gap.phase_index,
                gap.counted,
                gap.gap_ns,
                gap.gap_ns as f64 / 1_000_000.0,
                gap.physical_w,
                gap.physical_h,
                gap.present_mode
            )?;
        }
        Ok(())
    }

    fn system_attribution(spans: &[SpanRec], phase: &str) -> (u64, u64, u64) {
        let mut total = 0u64;
        let mut with_schedule = 0u64;
        let mut with_parent = 0u64;
        for span in spans {
            if span.phase != phase || span.meta != "system" {
                continue;
            }
            if phase.ends_with("_warm") || span.counted {
                total += 1;
                if !span.schedule.is_empty() {
                    with_schedule += 1;
                }
                if span.parent_id.is_some() {
                    with_parent += 1;
                }
            }
        }
        (total, with_schedule, with_parent)
    }

    fn placeholder_names(spans: &[SpanRec]) -> bool {
        spans.iter().any(|span| {
            span.meta == "system" && span.field_name.contains("Enable the debug feature")
        })
    }

    fn finish_report(run: &RunState, abort: Option<String>) {
        let dir = out_dir();
        let _ = std::fs::create_dir_all(&dir);
        if let Some(message) = abort {
            let body = serde_json::json!({
                "status": "blocker",
                "error": message,
                "frame": run.frame,
                "phase": run.phase,
                "span_records": run.spans.len(),
                "diag_records": run.diags.len(),
                "gap_records": run.gaps.len(),
                "span_misses": run.span_misses,
                "logs": run.logs,
                "notes": run.notes,
            });
            let _ = std::fs::write(
                dir.join("blocker.json"),
                serde_json::to_string_pretty(&body).unwrap_or_else(|_| "{}".into()),
            );
            let _ = std::fs::write(dir.join("status.txt"), format!("blocker\n{message}\n"));
            let _ = write_jsonl(&std::path::PathBuf::from("/tmp/vis-perf-2-debug-spans.jsonl"), &run.spans);
            let _ = write_jsonl(&std::path::PathBuf::from("/tmp/vis-perf-2-debug-diags.jsonl"), &run.diags);
            return;
        }

        let phases = ["off1", "on1", "off2", "on2"];
        let mut gap_stats = serde_json::Map::new();
        for phase in phases {
            gap_stats.insert(phase.to_string(), stats_f64(&gap_values(&run.gaps, phase)));
        }
        let gap_stats = serde_json::Value::Object(gap_stats);
        let deltas = serde_json::json!({
            "on1_minus_off1": sub_stat(&gap_stats["on1"], &gap_stats["off1"]),
            "off2_minus_on1": sub_stat(&gap_stats["off2"], &gap_stats["on1"]),
            "on2_minus_off2": sub_stat(&gap_stats["on2"], &gap_stats["off2"]),
        });
        let off_drift = sub_stat(&gap_stats["off2"], &gap_stats["off1"]);

        let on1_rows = system_rows(&run.spans, "on1");
        let on2_rows = system_rows(&run.spans, "on2");
        let cpu = serde_json::json!({
            "unit": "ms",
            "identity": "bevy_ecs system span field `name` plus schedule span field `name`",
            "main_vs_render_separated": true,
            "overlap": "System span wall times may overlap across threads. They are not summed into a frame gap.",
            "on1": {
                "main": rows_json(&on1_rows, "main"),
                "render": rows_json(&on1_rows, "render"),
                "other": rows_json(&on1_rows, "other"),
            },
            "on2": {
                "main": rows_json(&on2_rows, "main"),
                "render": rows_json(&on2_rows, "render"),
                "other": rows_json(&on2_rows, "other"),
            },
            "top15_main_same_order": top_names(&on1_rows, "main", 15) == top_names(&on2_rows, "main", 15),
            "top15_render_same_order": top_names(&on1_rows, "render", 15) == top_names(&on2_rows, "render", 15),
            "top15_main_on1": top_names(&on1_rows, "main", 15),
            "top15_main_on2": top_names(&on2_rows, "main", 15),
            "top15_render_on1": top_names(&on1_rows, "render", 15),
            "top15_render_on2": top_names(&on2_rows, "render", 15),
        });

        let mut pass = serde_json::Map::new();
        let mut other_metrics = serde_json::Map::new();
        for phase in phases {
            let cpu_rows = pass_rows(&run.diags, phase, "elapsed_cpu");
            let gpu_rows = pass_rows(&run.diags, phase, "elapsed_gpu");
            pass.insert(
                phase.to_string(),
                serde_json::json!({
                    "elapsed_cpu": pass_json(&cpu_rows, "ms"),
                    "elapsed_gpu": pass_json(&gpu_rows, "ms"),
                    "elapsed_gpu_status": if gpu_rows.is_empty() { "ABSENT" } else { "PRESENT" },
                }),
            );
            let mut metric_names: BTreeSet<String> = BTreeSet::new();
            for diag in &run.diags {
                if diag.phase == phase && diag.counted && diag.metric != "elapsed_cpu" && diag.metric != "elapsed_gpu" {
                    metric_names.insert(diag.metric.clone());
                }
            }
            let mut metric_obj = serde_json::Map::new();
            for metric in metric_names {
                let rows = pass_rows(&run.diags, phase, &metric);
                metric_obj.insert(metric, pass_json(&rows, "count"));
            }
            other_metrics.insert(phase.to_string(), serde_json::Value::Object(metric_obj));
        }

        let gpu_samples = run
            .diags
            .iter()
            .filter(|diag| diag.counted && diag.metric == "elapsed_gpu" && diag.value.is_some())
            .count();
        let gpu_zeros = run
            .diags
            .iter()
            .filter(|diag| diag.counted && diag.metric == "elapsed_gpu" && diag.measured_zero)
            .count();
        let gpu_status = if gpu_samples == 0 { "ABSENT" } else { "PRESENT" };

        let mut path_counts: BTreeMap<String, BTreeMap<String, u64>> = BTreeMap::new();
        for diag in &run.diags {
            if !diag.counted {
                continue;
            }
            *path_counts
                .entry(diag.phase.clone())
                .or_default()
                .entry(diag.path.clone())
                .or_default() += 1;
        }

        let mut threads: BTreeMap<(String, u64), BTreeSet<String>> = BTreeMap::new();
        for span in &run.spans {
            if span.counted && span.meta == "system" {
                threads
                    .entry((span.phase.clone(), span.frame))
                    .or_default()
                    .insert(span.thread_id.clone());
            }
        }
        let widths: Vec<f64> = threads.values().map(|set| set.len() as f64).collect();
        let multi = threads.values().filter(|set| set.len() > 1).count();

        let (on1_total, on1_sched, on1_parent) = system_attribution(&run.spans, "on1");
        let (on2_total, on2_sched, on2_parent) = system_attribution(&run.spans, "on2");
        let (warm_total, warm_sched, _) = system_attribution(&run.spans, "on1_warm");

        let mut problems = Vec::new();
        for phase in phases {
            let gaps = run.gaps.iter().filter(|gap| gap.phase == phase && gap.counted).count();
            if gaps != SETTLED_FRAMES as usize {
                problems.push(format!("{phase} frame gaps {gaps} != {SETTLED_FRAMES}"));
            }
            let cpu_n = run.diags.iter().filter(|d| d.phase == phase && d.counted && d.metric == "elapsed_cpu" && d.value.is_some()).count();
            if cpu_n == 0 {
                problems.push(format!("{phase} has no render elapsed_cpu samples"));
            }
        }
        if on1_total == 0 || on2_total == 0 {
            problems.push("settled ON windows recorded no system spans".into());
        }
        if on1_total > 0 && on1_sched * 2 < on1_total {
            problems.push(format!("on1 schedule attribution {on1_sched}/{on1_total}"));
        }
        if on2_total > 0 && on2_sched * 2 < on2_total {
            problems.push(format!("on2 schedule attribution {on2_sched}/{on2_total}"));
        }
        if placeholder_names(&run.spans) {
            problems.push("system names are the bevy debug-feature placeholder".into());
        }
        if run.presets.iter().any(|preset| preset != "Medium") {
            problems.push(format!("preset set changed: {:?}", run.presets));
        }
        if let Some(win) = &run.win {
            if win.max_w < 100 || win.max_h < 100 {
                problems.push(format!("window stayed tiny {}x{}", win.max_w, win.max_h));
            }
        } else {
            problems.push("no window sample".into());
        }
        if let Some(note) = &run.scene_note {
            if note.contains("sky") || note.contains("camera") {
                problems.push(note.clone());
            }
        }

        let problem_text = problems.join("; ");
        let status = if problems.is_empty() { "ok" } else { "blocker" };
        let summary = serde_json::json!({
            "card": "VIS-PERF-2",
            "status": status,
            "problems": problems,
            "gpu_status": gpu_status,
            "gpu_elapsed_samples": gpu_samples,
            "gpu_measured_zero_samples": gpu_zeros,
            "gpu_absence_is_not_zero": true,
            "frame_gap_ms": gap_stats,
            "adjacent_deltas_ms": deltas,
            "off_pre_post_drift_ms": off_drift,
            "cpu_systems": cpu,
            "render_passes": pass,
            "other_pass_metrics": other_metrics,
            "diagnostic_path_counts": path_counts,
            "system_span_attribution": {
                "on1_warm_total_with_schedule": [warm_total, warm_sched],
                "on1_total_with_schedule_with_tracing_parent": [on1_total, on1_sched, on1_parent],
                "on2_total_with_schedule_with_tracing_parent": [on2_total, on2_sched, on2_parent],
                "span_enter_misses": run.span_misses,
                "nan_diagnostics_skipped_as_numbers": run.nan_diags,
            },
            "thread_overlap": {
                "system_span_thread_count_per_frame": stats_f64(&widths),
                "frames_with_more_than_one_thread": multi,
                "not_summed_into_frame": true,
            },
            "scene": {
                "sky": run.scene.as_ref().map(|scene| scene.sky),
                "cameras": run.scene.as_ref().map(|scene| scene.cameras),
                "meshes": run.scene.as_ref().map(|scene| scene.meshes),
                "note": run.scene_note,
            },
            "presets_seen": run.presets,
            "window": run.win.as_ref().map(|win| serde_json::json!({
                "frames": win.frames,
                "min_w": win.min_w,
                "max_w": win.max_w,
                "min_h": win.min_h,
                "max_h": win.max_h,
                "modes": win.modes,
                "scales": win.scales,
                "logical": win.logical,
                "first": win.first,
                "last": win.last,
            })),
            "method": {
                "span_elapsed": "enter Instant to exit Instant; bevy system spans are long-lived and re-entered, so tracing on_close is the process-lifetime drop and is not the per-invocation sample",
                "schedule": "global stack of bevy_ecs schedule spans; PipelinedRenderingPlugin disabled at setup so only one schedule is active while workers run",
                "tracing_parent": "system spans are created with parent: None, so tracing parent id is expected null",
                "diagnostic_time": "DiagnosticMeasurement.time is the PreUpdate sync stamp, not the GPU execution time",
                "dedupe": "(path, measurement_ns from that stamp); a cached stamp is not a second invocation",
                "age_frames": "0 when the stamp is at or after the previous observer instant; 1 when the stamp predates it",
                "sink": "span vec push only while sink is open; RenderDiagnosticsPlugin and the subscriber stay up for all four windows",
                "common_overhead": "trace, trace_chrome (includes bevy debug), chrome layer, RenderDiagnosticsPlugin, subscriber callbacks, and diagnostic sample copy on OFF and ON are not claimed as sink overhead",
                "screenshots": "none",
            },
        });

        let machine = serde_json::json!({
            "notes": run.notes,
            "adapter": run.adapter,
            "features_debug": run.features,
            "timestamp_period_ns": run.timestamp_period_ns,
            "timestamp_query": run.flag_timestamp_query,
            "timestamp_query_inside_passes": run.flag_timestamp_inside_passes,
            "timestamp_query_inside_encoders": run.flag_timestamp_inside_encoders,
            "pipeline_statistics_query": run.flag_pipeline_statistics,
            "logs": run.logs,
            "span_records": run.spans.len(),
            "diag_records": run.diags.len(),
            "gap_records": run.gaps.len(),
        });

        let summary_ok = std::fs::write(
            dir.join("summary.json"),
            serde_json::to_string_pretty(&summary).unwrap_or_else(|_| "{}".into()),
        );
        let machine_ok = std::fs::write(
            dir.join("machine.json"),
            serde_json::to_string_pretty(&machine).unwrap_or_else(|_| "{}".into()),
        );
        let gaps_ok = write_gaps(&dir.join("frame_gaps.csv"), &run.gaps);
        let spans_ok = write_jsonl(&dir.join("spans.jsonl"), &run.spans);
        let diags_ok = write_jsonl(&dir.join("diagnostics.jsonl"), &run.diags);
        if summary_ok.is_err() || machine_ok.is_err() || gaps_ok.is_err() || spans_ok.is_err() || diags_ok.is_err() {
            let _ = std::fs::write(
                dir.join("status.txt"),
                "blocker\nfailed to write evidence files\n",
            );
            return;
        }
        if status == "ok" {
            let _ = std::fs::remove_file(dir.join("blocker.json"));
            let _ = std::fs::write(dir.join("status.txt"), "ok\n");
        } else {
            let _ = std::fs::write(
                dir.join("blocker.json"),
                serde_json::to_string_pretty(&summary).unwrap_or_else(|_| "{}".into()),
            );
            let _ = std::fs::write(
                dir.join("status.txt"),
                format!("blocker\n{problem_text}\n"),
            );
        }
    }

    fn vis_perf_observe(
        mut exit: MessageWriter<bevy::app::AppExit>,
        windows: Query<&Window, With<PrimaryWindow>>,
        sky: Query<&SkyBackdropPart>,
        cameras: Query<&Camera>,
        meshes: Query<&Mesh3d>,
        settings: Res<LocalSettingsState>,
        store: Res<DiagnosticsStore>,
        adapter: Option<Res<bevy::render::renderer::RenderAdapterInfo>>,
        device: Option<Res<bevy::render::renderer::RenderDevice>>,
        queue: Option<Res<bevy::render::renderer::RenderQueue>>,
        pipelined: Option<Res<bevy::render::pipelined_rendering::RenderAppChannels>>,
    ) {
        let now = Instant::now();
        let sky_n = sky.iter().count();
        let cam_n = cameras.iter().count();
        let mesh_n = meshes.iter().count();
        let preset = format!("{:?}", settings.inner.graphics_preset);
        let mut window_line = None;
        let mut window_now = (0u32, 0u32, String::new());
        if let Ok(window) = windows.single() {
            window_now = (
                window.physical_width(),
                window.physical_height(),
                format!("{:?}", window.present_mode),
            );
            window_line = Some(format!(
                "physical={}x{} logical={}x{} scale={} present={:?}",
                window.physical_width(),
                window.physical_height(),
                window.width(),
                window.height(),
                window.scale_factor(),
                window.present_mode
            ));
        }
        let adapter_line = adapter.as_ref().map(|info| {
            format!(
                "name={} vendor={} device={} type={:?} backend={:?} driver={} driver_info={} pci={}",
                info.name,
                info.vendor,
                info.device,
                info.device_type,
                info.backend,
                info.driver,
                info.driver_info,
                info.device_pci_bus_id
            )
        });
        let feature_line = device.as_ref().map(|device| {
            let features = device.features();
            (
                features.contains(WgpuFeatures::TIMESTAMP_QUERY),
                features.contains(WgpuFeatures::TIMESTAMP_QUERY_INSIDE_PASSES),
                features.contains(WgpuFeatures::TIMESTAMP_QUERY_INSIDE_ENCODERS),
                features.contains(WgpuFeatures::PIPELINE_STATISTICS_QUERY),
                format!("{features:?}"),
            )
        });
        let period = queue.as_ref().map(|queue| queue.get_timestamp_period());
        let pipelined_on = pipelined.is_some();

        let mut fresh = Vec::new();
        for diag in store.iter() {
            let path = diag.path().to_string();
            let suffix = diag.suffix.to_string();
            let (pass, metric) = split_path(&path);
            for measurement in diag.measurements() {
                let measurement_ns = measurement.time.saturating_duration_since(t0()).as_nanos() as u64;
                fresh.push((
                    path.clone(),
                    pass.clone(),
                    metric.clone(),
                    suffix.clone(),
                    measurement_ns,
                    measurement.value,
                    measurement.time,
                ));
            }
        }

        let mut abort_now = None;
        let mut finalize = false;
        {
            let mut run = lock_state();
            run.last_calls = run.last_calls.saturating_add(1);
            run.presets.insert(preset);
            if pipelined_on && run.abort.is_none() {
                run.abort = Some(
                    "PipelinedRenderingPlugin still inserted RenderAppChannels; schedule stack would race across the render thread".into(),
                );
            }
            if run.armed && run.last_calls >= 5 && run.frame.saturating_add(1) != run.last_calls && run.abort.is_none() {
                run.abort = Some(format!(
                    "update span frame counter desynced from Last: frame={} last_calls={}",
                    run.frame, run.last_calls
                ));
            }
            let previous_observe = run.prev_observe;
            if let Some(prev) = previous_observe {
                let gap_ns = now.saturating_duration_since(prev).as_nanos() as u64;
                let rec = GapRec {
                    frame: run.frame,
                    phase: run.phase.clone(),
                    phase_index: run.phase_index,
                    counted: run.counted,
                    gap_ns,
                    physical_w: window_now.0,
                    physical_h: window_now.1,
                    present_mode: window_now.2.clone(),
                };
                run.gaps.push(rec);
            }
            run.prev_observe = Some(now);

            if let Some(line) = window_line {
                let (w, h, scale, mode, logical) = windows.single().ok().map(|window| {
                    (
                        window.physical_width(),
                        window.physical_height(),
                        format!("{}", window.scale_factor()),
                        format!("{:?}", window.present_mode),
                        format!("{}x{}", window.width(), window.height()),
                    )
                }).unwrap_or((0, 0, String::new(), String::new(), String::new()));
                match &mut run.win {
                    Some(win) => {
                        win.frames += 1;
                        win.min_w = win.min_w.min(w);
                        win.max_w = win.max_w.max(w);
                        win.min_h = win.min_h.min(h);
                        win.max_h = win.max_h.max(h);
                        win.modes.insert(mode);
                        win.scales.insert(scale);
                        win.logical.insert(logical);
                        win.last = line;
                    }
                    None => {
                        let mut modes = BTreeSet::new();
                        modes.insert(mode);
                        let mut scales = BTreeSet::new();
                        scales.insert(scale);
                        let mut logicals = BTreeSet::new();
                        logicals.insert(logical);
                        run.win = Some(WinSeen {
                            frames: 1,
                            min_w: w,
                            max_w: w,
                            min_h: h,
                            max_h: h,
                            modes,
                            scales,
                            logical: logicals,
                            first: line.clone(),
                            last: line,
                        });
                    }
                }
            }
            if run.adapter.is_none() {
                run.adapter = adapter_line;
            }
            if let Some((tq, tp, te, ps, debug)) = feature_line {
                if run.features.is_none() {
                    run.flag_timestamp_query = Some(tq);
                    run.flag_timestamp_inside_passes = Some(tp);
                    run.flag_timestamp_inside_encoders = Some(te);
                    run.flag_pipeline_statistics = Some(ps);
                    run.features = Some(debug);
                }
            }
            if run.timestamp_period_ns.is_none() {
                run.timestamp_period_ns = period;
            }
            let scene = SceneCounts {
                sky: sky_n,
                cameras: cam_n,
                meshes: mesh_n,
            };
            if run.scene.is_none() && run.frame >= 1 {
                run.scene = Some(SceneCounts { ..scene });
            } else if let Some(first) = &run.scene {
                if (first.sky != scene.sky || first.cameras != scene.cameras) && run.scene_note.is_none() {
                    run.scene_note = Some(format!(
                        "sky/camera changed after frame 1: sky {}->{} camera {}->{} mesh {}->{}",
                        first.sky, scene.sky, first.cameras, scene.cameras, first.meshes, scene.meshes
                    ));
                    run.abort = run.scene_note.clone();
                } else if first.meshes != scene.meshes && run.scene_note.is_none() {
                    run.scene_note = Some(format!(
                        "mesh count changed {}->{}; sky {} and camera {} stayed",
                        first.meshes, scene.meshes, scene.sky, scene.cameras
                    ));
                }
            }

            let phase = run.phase.clone();
            let phase_index = run.phase_index;
            let counted = run.counted;
            let frame = run.frame;
            if emit_phase(&phase) {
                for (path, pass, metric, suffix, measurement_ns, value, stamped) in fresh {
                    if !run.seen.insert((path.clone(), measurement_ns)) {
                        continue;
                    }
                    let finite = value.is_finite();
                    if !finite {
                        run.nan_diags = run.nan_diags.saturating_add(1);
                    }
                    let age_frames = match previous_observe {
                        Some(prev) if stamped < prev => 1,
                        _ => 0,
                    };
                    run.diags.push(DiagRec {
                        frame,
                        phase: phase.clone(),
                        phase_index,
                        counted,
                        path,
                        pass,
                        metric,
                        suffix,
                        value: if finite { Some(value) } else { None },
                        measured_zero: finite && value == 0.0,
                        measurement_ns,
                        lag_ns: now.saturating_duration_since(stamped).as_nanos() as u64,
                        age_frames,
                    });
                }
            } else {
                for (path, _, _, _, measurement_ns, _, _) in fresh {
                    run.seen.insert((path, measurement_ns));
                }
            }

            if phase == "on1_warm" && phase_index + 1 == WARM_FRAMES && run.abort.is_none() {
                let (total, with_schedule, _) = system_attribution(&run.spans, "on1_warm");
                if total == 0 {
                    run.abort = Some(
                        "no bevy_ecs `system` spans during on1_warm; trace span capture failed".into(),
                    );
                } else if placeholder_names(&run.spans) {
                    run.abort = Some(
                        "system span names are `<Enable the debug feature to see the name>`; trace_chrome debug names were not applied".into(),
                    );
                } else if with_schedule * 2 < total {
                    run.abort = Some(format!(
                        "schedule attribution failed during on1_warm: {with_schedule}/{total} system spans had an open schedule span"
                    ));
                }
                let cpu = run.diags.iter().filter(|d| d.metric == "elapsed_cpu" && d.value.is_some()).count();
                if cpu == 0 && run.abort.is_none() {
                    run.abort = Some(
                        "no render/.../elapsed_cpu diagnostics by the end of on1_warm; RenderDiagnosticsPlugin readback produced none".into(),
                    );
                }
            }

            if run.frame % 10 == 0 || phase_index == 0 {
                eprintln!(
                    "vis-perf-2 frame={} phase={} idx={} sink={} spans={} diags={} gaps={}",
                    run.frame,
                    run.phase,
                    run.phase_index,
                    run.sink,
                    run.spans.len(),
                    run.diags.len(),
                    run.gaps.len()
                );
                let _ = std::fs::write(
                    "/tmp/vis-perf-2-status.txt",
                    format!(
                        "frame={} phase={} idx={} sink={} spans={} diags={} gaps={}\n",
                        run.frame, run.phase, run.phase_index, run.sink, run.spans.len(), run.diags.len(), run.gaps.len()
                    ),
                );
            }

            if run.phase == "tail" || run.abort.is_some() || run.frame > 400 {
                if run.frame > 400 && run.abort.is_none() {
                    run.abort = Some(format!("frame budget exceeded at {}", run.frame));
                }
                run.frozen = true;
                run.sink = false;
                abort_now = run.abort.clone();
                finalize = true;
            }
        }
        if finalize {
            let run = lock_state();
            finish_report(&run, abort_now);
            exit.write(bevy::app::AppExit::Success);
        }
    }

    #[test]
    #[ignore = "VIS-PERF-2 lavapipe measurement; temporary"]
    fn vis_perf_2_contained_yard() {
        let _user_lock = crate::test_env::lock();
        T0.set(Instant::now()).ok();
        STATE.set(Mutex::new(RunState::new())).ok();
        let out = out_dir();
        std::fs::create_dir_all(&out).expect("evidence dir");
        for name in [
            "status.txt",
            "blocker.json",
            "machine.json",
            "summary.json",
            "frame_gaps.csv",
            "spans.jsonl",
            "diagnostics.jsonl",
        ] {
            let _ = std::fs::remove_file(out.join(name));
        }

        let user_dir = std::env::temp_dir().join(format!("vis-perf-2-user-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&user_dir);
        std::fs::create_dir_all(&user_dir).expect("fresh user dir");
        let chrome = std::path::PathBuf::from("/tmp/vis-perf-2-chrome.json");
        let _ = std::fs::remove_file(&chrome);
        let icd = "/usr/share/vulkan/icd.d/lvp_icd.json";
        let _env = [
            set_env("POWRUSH_USER_DIR", &user_dir.display().to_string()),
            set_env("POWRUSH_NET", "off"),
            set_env("XDG_DATA_HOME", &user_dir.display().to_string()),
            set_env("VK_ICD_FILENAMES", icd),
            set_env("WGPU_BACKEND", "vulkan"),
            set_env("TRACE_CHROME", chrome.to_str().unwrap_or("/tmp/vis-perf-2-chrome.json")),
        ];

        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let sky = manifest.join("src/sky_backdrop.rs");
        let cargo_toml = manifest.join("Cargo.toml");
        let lock = manifest.join("../Cargo.lock");
        let workspace = manifest.join("..");
        let mut notes = BTreeMap::new();
        notes.insert("base_commit".into(), "4c461cc486fa8cac801c49bc4b2067e1145de72c".into());
        notes.insert("scene".into(), "contained yard; vp_sky_2_lavapipe_low_and_medium_yard composition".into());
        notes.insert("realm".into(), "0".into());
        notes.insert("conifer_seed".into(), format!("{:#x}", super::CONIFER_SEED));
        notes.insert("preset".into(), "Medium from frame 0".into());
        notes.insert("requested_inner".into(), "1920x1080 physical, scale_factor_override 1.0".into());
        notes.insert("requested_present_mode".into(), "Immediate (non-Fifo)".into());
        notes.insert("plugins".into(), "DefaultPlugins minus Audio, stock Winit, PipelinedRendering; plus run_on_any_thread Winit, RenderDiagnosticsPlugin, PainterlyPlugin, SkyBackdropPlugin".into());
        notes.insert("not_loaded".into(), "PowrushClientBundle, VegetationPlugin, NetModePlugin, gameplay plugins".into());
        notes.insert("net".into(), "SessionNetMode inserted from PowrushNet::Off; NetModePlugin not added; no listen".into());
        notes.insert("pipelined_rendering".into(), "disabled at setup for the whole run so the schedule stack is unambiguous".into());
        notes.insert("screenshots".into(), "not taken".into());
        notes.insert("startup_frames".into(), STARTUP_FRAMES.to_string());
        notes.insert("preroll_frames".into(), PREROLL_FRAMES.to_string());
        notes.insert("warm_frames".into(), WARM_FRAMES.to_string());
        notes.insert("settled_frames".into(), SETTLED_FRAMES.to_string());
        notes.insert("windows".into(), "off1 -> on1_warm -> on1 -> off2 -> on2_warm -> on2 -> tail".into());
        notes.insert("user_dir".into(), user_dir.display().to_string());
        notes.insert("cwd".into(), cmd_out("pwd", &[]));
        notes.insert("rustc".into(), cmd_out("rustc", &["--version"]));
        notes.insert("cargo".into(), cmd_out("cargo", &["--version"]));
        notes.insert("uname".into(), cmd_out("uname", &["-a"]));
        notes.insert("cpu".into(), cpu_model());
        notes.insert("nproc".into(), cmd_out("nproc", &[]));
        notes.insert("os_release".into(), std::fs::read_to_string("/etc/os-release").unwrap_or_default().lines().take(4).collect::<Vec<_>>().join(" | "));
        notes.insert("bevy_version".into(), lock_pkg_version("bevy"));
        notes.insert("tracing_chrome_version".into(), lock_pkg_version("tracing-chrome"));
        notes.insert("tracing_subscriber_version".into(), lock_pkg_version("tracing-subscriber"));
        notes.insert("wgpu_version".into(), lock_pkg_version("wgpu"));
        notes.insert("temp_sky_sha256".into(), sha256_file(&sky));
        notes.insert("temp_sky_git_blob".into(), git_blob(&sky));
        notes.insert("temp_client_cargo_sha256".into(), sha256_file(&cargo_toml));
        notes.insert("temp_client_cargo_git_blob".into(), git_blob(&cargo_toml));
        notes.insert("temp_lock_sha256".into(), sha256_file(&lock));
        if let Ok(text) = std::fs::read_to_string("/tmp/vis-perf-2-base-sha256.txt") {
            notes.insert("base_sha256_file".into(), text);
        }
        if let Ok(cmd) = std::fs::read_to_string("/tmp/vis-perf-2-command.txt") {
            notes.insert("command".into(), cmd.trim().to_string());
        }
        notes.insert(
            "args".into(),
            std::env::args().collect::<Vec<_>>().join(" "),
        );
        for key in [
            "DISPLAY",
            "WAYLAND_DISPLAY",
            "VK_ICD_FILENAMES",
            "WGPU_BACKEND",
            "WGPU_ADAPTER_NAME",
            "WGPU_VALIDATION",
            "RUST_LOG",
            "TRACE_CHROME",
            "POWRUSH_NET",
            "POWRUSH_USER_DIR",
            "XDG_DATA_HOME",
        ] {
            notes.insert(
                format!("env_{key}"),
                std::env::var(key).unwrap_or_else(|_| "<unset>".into()),
            );
        }

        let net = crate::net_mode::SessionNetMode::from_powrush_net(shared::hex_listen::PowrushNet::Off);
        notes.insert("net_mode_label".into(), net.mode.label().to_string());
        notes.insert("net_offline".into(), net.mode.is_offline_default().to_string());
        notes.insert("title_online".into(), net.title_online_enabled().to_string());
        notes.insert("opens_listen".into(), net.opens_listen_socket().to_string());

        let mut settings = shared::local_settings::LocalSettings::default();
        settings.set_graphics_preset(GraphicsPreset::Medium);
        let mut log_plugin = bevy::log::LogPlugin::default();
        log_plugin.custom_layer = vis_perf_layer;
        log_plugin.level = bevy::log::Level::INFO;

        let mut app = App::new();
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1920, 1080).with_scale_factor_override(1.0),
                        title: "vis-perf-2".into(),
                        present_mode: PresentMode::Immediate,
                        decorations: false,
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(log_plugin)
                .disable::<bevy::audio::AudioPlugin>()
                .disable::<bevy::winit::WinitPlugin>()
                .disable::<bevy::render::pipelined_rendering::PipelinedRenderingPlugin>(),
        )
        .add_plugins({
            let mut winit = bevy::winit::WinitPlugin::default();
            winit.run_on_any_thread = true;
            winit
        })
        .add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin)
        .insert_resource(SoftPlayerRealm { current: Some(0) })
        .insert_resource(LocalSettingsState {
            inner: settings,
            dirty: false,
        })
        .insert_resource(net)
        .insert_resource(GlobalAmbientLight {
            color: Color::srgb(0.70, 0.80, 1.0),
            brightness: 250.0,
            ..default()
        })
        .insert_resource(ClearColor(crate::climate_plane::sky_for(
            Some(0),
            GraphicsPreset::Medium,
        )))
        .add_plugins(crate::painterly_mat::PainterlyPlugin)
        .add_plugins(SkyBackdropPlugin)
        .add_systems(Last, vis_perf_observe);

        let plate = {
            let mut meshes = app.world_mut().resource_mut::<Assets<Mesh>>();
            meshes.add(Cuboid::new(1.15, 1.45, 0.08))
        };
        let glow = {
            let mut meshes = app.world_mut().resource_mut::<Assets<Mesh>>();
            meshes.add(Sphere::new(0.42))
        };
        let readable = {
            let mut materials = app.world_mut().resource_mut::<Assets<StandardMaterial>>();
            materials.add(StandardMaterial {
                base_color: Color::srgb(0.93, 0.91, 0.84),
                perceptual_roughness: 0.45,
                ..default()
            })
        };
        let well = {
            let mut materials = app.world_mut().resource_mut::<Assets<StandardMaterial>>();
            materials.add(StandardMaterial {
                base_color: Color::srgb(0.86, 0.66, 0.29),
                emissive: LinearRgba::new(6.0, 2.4, 0.4, 1.0),
                ..default()
            })
        };
        app.world_mut().spawn((
            Camera3d::default(),
            Camera {
                order: 0,
                ..default()
            },
            crate::climate_plane::WORLD_TONEMAPPING,
            crate::climate_plane::world_color_grading(),
            Transform::from_xyz(0.0, 3.25, 6.4).looking_at(Vec3::new(0.0, 5.2, -28.0), Vec3::Y),
            bevy::render::view::Msaa::Off,
            super::yard_distance_fog(GraphicsPreset::Medium),
        ));
        app.world_mut().spawn((
            crate::climate_plane::SANCTUARY_SUN.bundle(),
            Name::new("SanctuarySun"),
        ));
        app.world_mut().spawn((
            Mesh3d(plate),
            MeshMaterial3d(readable),
            Transform::from_xyz(-2.4, 1.35, 3.4).looking_at(Vec3::new(0.0, 3.25, 6.4), Vec3::Y),
        ));
        app.world_mut().spawn((
            Mesh3d(glow),
            MeshMaterial3d(well),
            Transform::from_xyz(0.35, 0.7, 1.2),
        ));

        {
            let mut run = lock_state();
            run.notes = notes;
            run.schedule_stack.clear();
            run.app_stack.clear();
            run.armed = true;
        }
        let _ = workspace;
        app.run();

        let status = std::fs::read_to_string(out.join("status.txt"))
            .unwrap_or_else(|err| panic!("VIS-PERF-2 wrote no status: {err}"));
        if let Ok(meta) = std::fs::metadata(&chrome) {
            let mut machine: serde_json::Value = std::fs::read_to_string(out.join("machine.json"))
                .ok()
                .and_then(|text| serde_json::from_str(&text).ok())
                .unwrap_or_else(|| serde_json::json!({}));
            if let Some(obj) = machine.as_object_mut() {
                obj.insert(
                    "chrome_trace_bytes".into(),
                    serde_json::json!(meta.len()),
                );
                obj.insert(
                    "chrome_trace_path".into(),
                    serde_json::json!(chrome.display().to_string()),
                );
                obj.insert(
                    "user_dir_entries".into(),
                    serde_json::json!(
                        std::fs::read_dir(&user_dir)
                            .map(|entries| {
                                entries
                                    .filter_map(|e| e.ok())
                                    .map(|e| e.file_name().to_string_lossy().to_string())
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default()
                    ),
                );
            }
            let _ = std::fs::write(
                out.join("machine.json"),
                serde_json::to_string_pretty(&machine).unwrap_or_else(|_| "{}".into()),
            );
        }
        assert_eq!(
            status.lines().next().unwrap_or(""),
            "ok",
            "VIS-PERF-2 blocker:\n{status}"
        );
    }
```
