# VIS-SKY-3

Before is base `a0f16a679a89152eab9018450cd9bbf74661ec10`.
After is `f23a02f96acf177eaba3dcd29a0723c00467a74a` on `cursor/vis-sky-3-a0a9` (`client/src/sky_backdrop.rs`). The stills were rendered from that commit. This directory is the evidence commit on top of it; `sky_backdrop.rs` is unchanged between that rendered commit and the branch head. The PR body records the branch head SHA.

Both pairs are real frames from the in-repo ignored test
`sky_backdrop::tests::vp_sky_2_lavapipe_low_and_medium_yard`
on lavapipe (`VK_ICD_FILENAMES` pointed at `/usr/share/vulkan/icd.d/lvp_icd.json`, `WGPU_BACKEND=vulkan`).
The harness window is the same Low-then-Medium yard pose and seed it already used for #972. The test was reused as-is (observation only, not CI-gating).

- `before-low.png` — base `a0f16a679a89152eab9018450cd9bbf74661ec10`
- `after-low.png` — `f23a02f96acf177eaba3dcd29a0723c00467a74a`
- `before-medium.png` — same base, same camera
- `after-medium.png` — same after commit, same camera

Medium frame-time: not measured. The harness writes the two stills and does not record a frame time.
