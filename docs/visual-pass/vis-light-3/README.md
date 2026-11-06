# VIS-LIGHT-3

P1. The Medium environment-map fill was measured once and is not in this tree. `client/src/ultra_env.rs` and `client/src/sky_backdrop.rs` match base `4c461cc486fa8cac801c49bc4b2067e1145de72c`. This README is the only evidence path.

## Gate

Lavapipe software-renderer frame gaps, not device FPS. Same accounting as #974: 52 raw Medium gaps per side, drop the first 20 and the last 3, n=29 settled. Median is the sorted settled sample at index 14. Mean is `(sum + 14) / 29` integer division (half up).

| side | median µs | mean µs | min µs | max µs |
| --- | ---: | ---: | ---: | ---: |
| base (Medium plan `None`) | 112385 | 113822 | 105281 | 134773 |
| candidate (Medium plan `Some(120)`) | 149404 | 152207 | 136961 | 183267 |

Median delta **+37019 µs (+37.019 ms)**. Mean delta **+38385 µs (+38.385 ms)**. The ship bound is **+300 µs (+0.3 ms)**. The fill does not ship.

## Pair

One `cargo test -p powrush-client --lib sky_backdrop::tests::vp_sky_2_lavapipe_low_and_medium_yard -- --ignored --nocapture --test-threads=1` process. The test returned ok in 36.12s. No second pair.

`VK_ICD_FILENAMES` and `VK_DRIVER_FILES` were `/usr/share/vulkan/icd.d/lvp_icd.json`. `WGPU_BACKEND=vulkan`. `DISPLAY=:1`. `RUST_LOG=error` on both sides. The harness window request is unchanged: 1280×800, title `vp-sky-2`. The captured stills are 1267×792, and the base still matches the #974 base Medium still byte for byte.

Camera, unchanged from the harness: `Transform::from_xyz(0.0, 3.25, 6.4).looking_at(Vec3::new(0.0, 5.2, -28.0), Vec3::Y)`. Realm `Some(0)`. Conifer seed `0x1234_5678`. Ridge rng `0x9E37_79B9`. `Msaa::Off`. Sanctuary sun, yard fog, and clear colour follow the existing Low-then-Medium switch.

Order inside that one app, with no world rebuild, no renderer reinit, and no compile between sides:

1. Low preset, 100-frame warmup, Low still (not saved).
2. Switch to Medium (fog and clear colour updated once, as the harness already did). `UltraEnvMediumObserve.candidate` stays false.
3. 48 Medium frames, then `before-medium.png`.
4. 52 base frame gaps.
5. Flip `candidate` to true. Next gaps are the candidate window.
6. 52 candidate frame gaps.
7. `after-medium.png`.

A frame gap is the wall time between consecutive `drive_sky_probe` calls. Screenshots are outside both 52-gap windows.

The harness asserted, and the passing test means these held: at base gaps 1 and 52 the camera had no `GeneratedEnvironmentMapLight` and no `EnvironmentMapLight`, and `graphics_preset` was Medium with the flag false. At candidate gaps 20 and 52 the camera had one `GeneratedEnvironmentMapLight` at intensity 120.0 and one `EnvironmentMapLight`, preset still Medium, flag true.

## Settled or not

Candidate raw gaps 0..19 contain the compile spikes (max **762751 µs**). The settled candidate max is **183267 µs**. Settled ranges do not overlap: base max 134773 < candidate min 136961.

Head-8 vs tail-8 settled medians: base 115051 then 113505 (−1546 µs); candidate 153961 then 149735 (−4226 µs). The candidate slope is small beside the +37019 µs shift, and the high samples sit in the middle of the window rather than only at its start. The pair is the steady cost after those prefix spikes. It was not rerun.

## Samples

Base raw (52), in order:

```
98550 117716 113485 113826 116777 119517 119326 115878 114572 123583 108886 122509 107602 113689 119478 111853 114674 113623 113302 113582 119117 116490 105281 112194 115051 118880 113828 114107 116497 134773 117551 107595 108646 107595 108255 110011 109699 108104 111141 122853 110184 120194 121681 111055 108886 113505 112385 111471 113812 109662 108809 112782
```

Base settled (raw index 20 through 48):

```
119117 116490 105281 112194 115051 118880 113828 114107 116497 134773 117551 107595 108646 107595 108255 110011 109699 108104 111141 122853 110184 120194 121681 111055 108886 113505 112385 111471 113812
```

Candidate raw (52), in order:

```
119180 108750 140389 463233 139221 689037 762751 651007 600698 173451 172291 152909 622965 145918 139223 148464 163500 154558 176879 159332 145241 153961 155547 139064 149147 147964 170413 164793 162739 182505 183267 173072 149404 142141 142173 138865 147957 149689 158348 136961 142978 141164 144602 149735 139747 146502 150441 152596 152992 159953 152219 157758
```

Candidate settled (raw index 20 through 48):

```
145241 153961 155547 139064 149147 147964 170413 164793 162739 182505 183267 173072 149404 142141 142173 138865 147957 149689 158348 136961 142978 141164 144602 149735 139747 146502 150441 152596 152992
```

## Mechanism

`UltraEnvMediumObserve`. Temporary `cfg(test)` resource `UltraEnvMediumObserve { candidate: bool }`, inserted by this harness before `app.run()` with `candidate: false`.

In that binary, `ultra_env_for(Medium)` always returned `Some(MEDIUM_ENV_INTENSITY)` (`120.0`). `sync_ultra_env` then applied `medium_observe_plan` only when the live preset was Medium and the resource was present:

- `candidate == false` → plan `None` (base `ultra_env_for`)
- `candidate == true` → plan `Some(MEDIUM_ENV_INTENSITY)` (the same value `ultra_env_for` returns)

Mobile, Low, High, and Ultra ignored the resource. No resource (the unit tests) left `ultra_env_for` unchanged. The flag is not in the shipping build and is not in this commit. No public setting, load path, world, or camera change.

The harness also added the existing `UltraEnvPlugin` for the run so `sync_ultra_env` was in the schedule. That addition was reverted with the rest of `sky_backdrop.rs`.

## Hashes

Base commit `4c461cc486fa8cac801c49bc4b2067e1145de72c`.

| file | role | git blob | sha256 |
| --- | --- | --- | --- |
| `client/src/ultra_env.rs` | base, restored in this commit | `b46ac13e5637bf5ea0e55cb0a598a397bda8e912` | `3c55dce98cee5f927a5384aab32aa19fdbab97e50b97b3aa8b13fc514044a11e` |
| `client/src/sky_backdrop.rs` | base, restored in this commit | `1edc824a57b37308fab37093cfd29c3acc4e5b38` | `16d6b5f50bce2ba327809e2fb388cb7c5be61fa133875e623a7b3c24dadb3b76` |
| `client/src/ultra_env.rs` | temporary measurement source | `922a01f07435ab49b4eb32e76f0124d220feec69` | `8ab95436d8efe1b3e06cfc3523f937bf3c65daa7b8c127fcc922ecc934beb161` |
| `client/src/sky_backdrop.rs` | temporary measurement source | `f01b7d98c7621b0397463917aa7c1b7b9a04481c` | `ec89a4387014ca09a2b5d06d8b8fb52813ad836a861a83e05290bd86f19d8333` |
| `client/src/ultra_env.rs` | clean candidate, not compiled as its own binary, not committed | `228cf76b8e6a309c7a525814361d022744557046` | `ba3a724ac4fc9bf97c2576926ee1cc9172446f7449074f1c99074652f2f20ba8` |

The measured process was the temporary pair. Its `ultra_env_for(Medium)` was already `Some(120)`; the flag only replaced the Medium plan inside `sync_ultra_env`. The clean candidate blob is the same function with the flag removed. It is recorded so the candidate source can be compared. It is not this commit.

After restore, `git hash-object` on both runtime files matches the base blobs above. `git grep -F UltraEnvMediumObserve` and `git grep -F MEDIUM_FRAME_US` over the tree are empty. `git grep -F MEDIUM_ENV_INTENSITY -- client` is empty.

## Stills

Not in the tree. P1 keeps this README only.

- `before-medium.png` sha256 `120cd522517c13266ac0ad6aa1b2c192213b40d5a21da305689a386d749aa853` (923173 bytes). This is the #974 base Medium still, same hash.
- `after-medium.png` sha256 `3600f45cbfc41d91f2dbdfb3b318ddbf4b46cfb6865850fbce4961566a9b7b52` (935039 bytes).

Both are 1267×792. 486130 of 1003464 pixels differ. Mean absolute difference per channel is 1.2792. Max per-pixel channel sum is 25. Same yard, small shading change.

## Tests on the restored tree

`cargo test -p shared -p rsil-identity` — exit 0. `rsil-identity` 5 passed. `shared` 497 passed. Doc-tests 0. Compile 14.75s.

`cargo test -p powrush-client --lib` — exit 0. 1358 passed, 0 failed, 4 ignored (not run). Compile 45.10s. Tests 1.80s.

The lavapipe test was not part of that lib run.

Online grey. No OFFER NEXT.
