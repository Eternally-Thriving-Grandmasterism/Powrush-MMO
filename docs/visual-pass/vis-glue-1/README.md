# VIS-GLUE-1

Before is base `48af3578e4721256367f5076aba4f26cdfc94653` (painted meshes on `StandardMaterial`).
After is this branch (`ExtendedMaterial` and `client/assets/shaders/painterly.wgsl`).

Both pairs are real frames from the in-repo ignored test
`sky_backdrop::tests::vp_sky_2_lavapipe_low_and_medium_yard`
on lavapipe (`VK_ICD_FILENAMES` pointed at `lvp_icd.json`, `WGPU_BACKEND=vulkan`).
The harness window is the same Low-then-Medium yard pose it already used.

- `before-low.png`
- `after-low.png`
- `before-medium.png`
- `after-medium.png`

Medium frame-time: not measured. The harness writes the two stills and does not record a frame time.
