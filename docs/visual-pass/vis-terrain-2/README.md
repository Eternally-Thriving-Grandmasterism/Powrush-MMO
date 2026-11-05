# VIS-TERRAIN-2

Before is base `4c461cc486fa8cac801c49bc4b2067e1145de72c`.
After is `020f89521591abe213c5c7fd28e1b3af782a8630` (`client/src/sky_backdrop.rs`). The stills were rendered from that commit. This directory is the evidence commit on top of it; `sky_backdrop.rs` is unchanged between that rendered commit and the branch head. The PR body records the branch head SHA.

Both pairs are real frames from the in-repo ignored test
`sky_backdrop::tests::vp_sky_2_lavapipe_low_and_medium_yard`
on lavapipe (`VK_ICD_FILENAMES` pointed at `/usr/share/vulkan/icd.d/lvp_icd.json`, `WGPU_BACKEND=vulkan`).
The harness window is the same Low-then-Medium yard pose and seed it already used for #972 and #973. The test was reused as-is (observation only, not CI-gating). These stills were rendered with no frame timer in the harness.

- `before-low.png` — base `4c461cc486fa8cac801c49bc4b2067e1145de72c`
- `after-low.png` — `020f89521591abe213c5c7fd28e1b3af782a8630`
- `before-medium.png` — same base, same camera
- `after-medium.png` — same after commit, same camera

The terrace is `PainterlyRole::Earth` with authored `VALLEY_EARTH`. Earth is one stop, so the fragment ramp is white. The height ramp was measured, then cut.

Medium frame-time was measured. The same harness printed the gap between updates during the Medium phase. That print was compiled only for the measurement runs and is not in the render commit or this evidence commit. Steady samples drop the first 20 gaps (preset switch and pipeline compile) and the last 3 (screenshot). n=29. One lavapipe run each.

- Plain ground, base: median 84949 µs, mean 85592 µs.
- Earth plus height ramp, not shipped: median 88724 µs, mean 88788 µs. Median delta versus plain: +3.775 ms.
- Earth shader with the ramp cut, shipped: median 87494 µs, mean 88048 µs. Median delta versus plain: +2.545 ms.

The ramp run was past +0.5 ms, so the relief was cut and the painterly ground shader kept. The shipped shader is still slower than the plain ground on this lavapipe pass. These figures are software-frame gaps, not a device rate.

Pixel check of these stills: the sky window and the ridge window match byte for byte on Low and on Medium. The open court shifts about one 8-bit level (red 124.16 to 123.30, green 117.44 to 117.54, blue stays 0). The changed pixels stay on the terrace.
