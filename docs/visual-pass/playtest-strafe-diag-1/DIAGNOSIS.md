# PLAYTEST-STRAFE-DIAG-1 — strafe shimmer diagnosis (docs only)

Base: `main` @ `d6c6db1d8dcafbe5228774995854ecd86921fce9`. No `client/**`, `shared/**`,
host, divine-module, or `Cargo.*` change. All line numbers are on that base.

## Symptom

Playtester report: while strafing left/right (A/D) under the first-hour follow
camera, the scene and/or the character shimmers / jitters.

## Evidence basis (honesty note)

- **No live motion capture and no frame-time trace were produced.** The box has
  no input-replay / scripted-input harness, so the character cannot be made to
  strafe without live keyboard input (same limitation as PLAYTEST-CAM-1). The
  repo has `FrameTimeDiagnosticsPlugin` + `LogDiagnosticsPlugin` logging FPS /
  FRAME_TIME (`client/src/main.rs:11, 25-29, 96`), but that only logs averages
  of a live session; it cannot drive a strafe.
- No images, numbers, or frame-times in this doc are measured. Everything below
  is from reading source and the Bevy schedule wiring, cited by file:line.

## Schedule map (what runs where)

| System | File:line | Schedule | Clock |
|---|---|---|---|
| `snapshot_move_keys` | `client/src/human_presence.rs:664-669` | PreUpdate | — |
| `apply_locomotion` (writes `SoftPresence.position`) | `human_presence.rs:688`, body `1145-1245` (`position += velocity * dt` at `1234`) | **FixedUpdate** | `Res<Time<Fixed>>` (`1147`) |
| Fixed rate | `client/src/feel_move.rs:15` (`SIM_HZ = 60.0`), inserted at `feel_move.rs:35` | — | 60 Hz |
| `sync_body` (copies position into the body `Transform`) | `human_presence.rs:1272-1282` (`tf.translation = presence.position` at `1282`) | Update (chain `672-686`) | — |
| `follow_camera` (camera translation + `look_at` the body) | `human_presence.rs:1628-1700`, registered `684` | Update, chained after `sync_body` | variable `Res<Time>` (`1617-1621`, dt at `1646`) |
| `pull_camera_from_walls` | `human_presence.rs:686` | Update, `.after(follow_camera)` | variable |
| `glance_toward_nearest_node` | `client/src/first_hour_camera.rs:56-58` | **PostUpdate, `.before(TransformSystems::Propagate)`** | variable (`161`) |
| glance slerp | `first_hour_camera.rs:195` (`cam.rotation.slerp(desired, blend)`) | — | — |

No `overstep`, `Interpolation`, or `TransformInterpolation` symbol exists anywhere
in `client/src` (rg, zero hits).

## Candidate A — camera-follow vs movement / Propagate ordering

- `follow_camera` reads `SoftPresence.position` (a resource), **not** the body's
  `Transform`/`GlobalTransform` (`human_presence.rs:1678-1688`). Every write to
  that resource (FixedUpdate, which runs inside `RunFixedMainLoop` before
  `Update`) has already happened by the time `follow_camera` runs in Update. So
  the camera does **not** sample a stale/pre-propagate target transform.
- `sync_body` and `follow_camera` are in the same `.chain()` (`672-686`), so body
  and camera both see the same position in the same frame.
- The glance (`first_hour_camera.rs:56-58`) writes only camera rotation, in
  PostUpdate before Propagate, so it lands in the same frame's GlobalTransform —
  correct ordering. It is also **cancelled while A/D is held**: `moving()` checks
  `KeyA`/`KeyD` (`first_hour_camera.rs:68-78`), `look_intent` → reset at
  `150-158`. The glance cannot be active during a strafe.
- Verdict: **ordering is not the cause.** The L57-58 note is correct and harmless.

## Candidate B — fixed (60 Hz) movement vs variable-rate camera, no interpolation  ← MOST LIKELY

- Position integrates only on FixedUpdate ticks at 60 Hz (`feel_move.rs:15,35`;
  `human_presence.rs:688, 1234`).
- Rendering/camera run once per display frame with variable `dt`
  (`human_presence.rs:1646`; `follow_lerp_t` at `1599-1609`, `FOLLOW_LERP_AT_60
  = 0.12` at `163`).
- `sync_body` copies the **raw last-tick** position to the body every frame
  (`1282`) with no fixed-step overstep interpolation (no such code exists).
- Consequence (standard Bevy fixed-timestep aliasing): on any frame rate that is
  not an exact 60 Hz lock (e.g. 75/120/144 Hz monitors, or 60 Hz with frame-time
  jitter), some frames run 0 fixed ticks and some run 2. The body therefore moves
  in uneven steps (0, 1, or 2 × `velocity/60`) per rendered frame, while the
  camera translation eases smoothly with a dt-weighted lerp (`1689-1690`).
  The body jitters against the camera; and because `follow_camera` also calls
  `cam.look_at(look)` on the stepped position every frame (`1691`), the **view
  direction** snaps by a small yaw each step. On a sideways (A/D) strafe the
  target moves perpendicular to the view axis, which is exactly the direction in
  which a position step becomes a visible yaw step — so the whole scene shimmers
  sideways. Forward/back (W/S) moves along the view axis and mostly shows as a
  barely visible distance change, which matches a report that is specific to strafe.
- Modern preset strafe keeps the body facing camera-forward (`sync_body`
  `1287-1290`), so rotation does not mask the lateral step.
- Confidence: high from code alone; not yet proven by capture.

## Candidate C — TAA / shadow temporal shimmer

Per preset (Low / Medium / High):

| Preset | AA (`local_settings.rs:250-256`) | Sun shadows (`climate_plane.rs:1055-1084`) |
|---|---|---|
| Low (and Mobile) | FXAA | **off** (`SUN_SHADOW_OFF`) |
| Medium | SMAA (`SMAA_PRESET = Medium`, `259`) | on, 1024², 2 cascades, 48 m |
| High | SMAA | on, 2048², 4 cascades, 96 m |
| Ultra | TAA (`245`) | as High |

- TAA is only on Ultra, so it cannot explain shimmer at Low/Med/High. All world
  cameras are `Msaa::Off` (`main.rs:121`, `ui_above_world.rs:114, 144-147`).
- Medium/High cascaded shadows (`cascade_shadow_for`, `climate_plane.rs:1091-1100`,
  first cascade 12 m) have no texel-snapping/stabilisation code in the repo (rg
  for `stabiliz|snap` in `climate_plane.rs`: no hit). Cascades are fit to the
  camera frustum, so a laterally moving/yawing camera can crawl shadow edges.
  This is a plausible **secondary** shimmer on Medium/High only.
- `rbe_simulation.rs:421-428` also forces `shadows_enabled = true` with
  `Hardware2x2` filtering on a light in that sim; worth checking whether it
  touches the yard sun (it would override Low's "off" plan). Not resolved here.
- Discriminator: Low has shadows off and FXAA (no temporal AA). If the shimmer
  reproduces on **Low**, C is excluded and B stands alone.

## Named cause

**Most likely: B — player locomotion integrates on FixedUpdate at 60 Hz with no
render-side interpolation, while `sync_body` and `follow_camera` run per
variable display frame and `look_at` the raw stepped position.** The lateral
step aliasing shows as a sideways shimmer of the scene on A/D. A (ordering) is
ruled out by the code. C (cascade shadow crawl) may add shimmer on Medium/High
but cannot explain Low.

## Files / systems a fix card would touch

- `client/src/human_presence.rs`
  - `SoftPresence` (`~521`): keep previous + current fixed-tick position.
  - `apply_locomotion` (`1145-1245`): store previous position before `+= velocity * dt` (`1234`).
  - `sync_body` (`1272-1282`) and `follow_camera` (`1628-1692`, desired/look at
    `1678-1688`): use a render position `prev.lerp(curr, Time<Fixed>::overstep_fraction())`
    instead of raw `presence.position`.
- Possibly `client/src/feel_move.rs` (`SIM_HZ`, `15/35`) only if Core picks a rate change rather than interpolation (not recommended).
- Secondary, only if Medium/High still shimmer after B: `client/src/climate_plane.rs`
  (`cascade_shadow_for` `1091`, `SUN_SHADOW_*` `1055-1076`) and check `rbe_simulation.rs:415-430`.

## What would prove it (not producible on this box)

1. A scripted-input / replay harness that holds A (or D) for N seconds, so a
   strafe can be rendered headlessly.
2. A per-frame log of: render dt, fixed ticks run this frame, `SoftPresence.position.x/z`,
   camera yaw. Expected if B: frames with 0 or 2 ticks line up with yaw steps.
3. Same strafe at Low vs Medium/High (and at a 60 Hz cap vs uncapped): B predicts
   shimmer on every preset that disappears at a stable 60 Hz lock; C predicts
   Medium/High only.
