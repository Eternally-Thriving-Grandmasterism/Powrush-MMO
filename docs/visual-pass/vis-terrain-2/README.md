# VIS-TERRAIN-2

Abandoned under Core RT5a (`#738` comment 6672801454) after one measured retry. The runtime file matches the base. No painted ground remains.

Base: `4c461cc486fa8cac801c49bc4b2067e1145de72c`.
Code commit: `1ec09b7771b2f0af3f9811f0d501771006d7f943`. That commit restores `client/src/sky_backdrop.rs` to the base. The PR body records the branch head. The head adds only this evidence directory.

The earlier render `020f89521591abe213c5c7fd28e1b3af782a8630` (branch head `b07a5a410a87adf7ba2588981af402943d0878df`) painted every preset with `PainterlyRole::Earth` and no height ramp. Hands COURT YELLOW 6672790733 flagged the shipped Medium median at +2.545 ms (87494 µs versus 84949 µs, n=29). RT5a ruled that breach stands.

RT5a P1 was compiled and measured, then not committed. Mobile, Low, and Medium spawned `BackdropShade::Plain(lit(VALLEY_EARTH, 0.95))`. High and Ultra spawned `painterly_material(PainterlyRole::Earth, VALLEY_EARTH, preset)`. No height ramp. `terrain_plan` still chose the mesh. A preset test checked plain below High and `PainterlyRole::Earth` on High and Ultra, with hue `VALLEY_EARTH` and roughness 0.95 on both paths. `cargo test -p powrush-client --lib sky_backdrop::tests::` on that uncommitted tree: 21 passed, 0 failed, 1 ignored.

Both measurement runs used the ignored harness `sky_backdrop::tests::vp_sky_2_lavapipe_low_and_medium_yard` on lavapipe (`VK_ICD_FILENAMES` pointed at `/usr/share/vulkan/icd.d/lvp_icd.json`, `WGPU_BACKEND=vulkan`), the same camera and seed as #972 and #973. A frame-gap print was compiled only for those two runs and is not in this tree. Steady samples drop the first 20 Medium gaps and the last 3. n=29. One run each.

- Baseline, exact base: median 85636 µs, mean 86196 µs, min 81531 µs, max 92205 µs.
- P1: median 87458 µs, mean 88016 µs, min 83758 µs, max 91968 µs.
- Median delta: +1822 µs (+1.822 ms). Mean delta: +1820 µs.

+1.822 ms is past the +0.5 ms bound, so RT5a P2 drops the card.

Low and Medium pixels did match. SHA-256:

- `before-low.png` and `after-low.png`: `c82c6745de10c629fc00e84163657365aeec29ec73d5b59daf7e0d3fb0f674ae`
- `before-medium.png` and `after-medium.png`: `120cd522517c13266ac0ad6aa1b2c192213b40d5a21da305689a386d749aa853`

Those are the base harness frames. The P1 Low frame and the P1 Medium frame hashed the same, including the baseline recapture in this session. The frames are byte-equivalent. The Medium median was not. These figures are lavapipe software-frame gaps.

`after-low.png` and `after-medium.png` are the plain frames above. They replace the Earth-shader stills from `b07a5a41`, which this head does not render.
