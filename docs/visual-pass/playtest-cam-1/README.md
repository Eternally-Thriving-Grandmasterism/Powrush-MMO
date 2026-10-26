# PLAYTEST-CAM-1: raw look (CARD CAM-RAW)

## What changed
- New persisted setting `raw_look` (default off) in `shared/local_settings.rs`, copied into `LocalLookFeel` by `from_settings`.
- With `raw_look` on, `follow_camera` skips the follow-lerp look smoothing (`1 - (1 - 0.12)^(dt * 60)`). `look_follow_t` returns 1.0, so look is 1:1. The test `look_follow_t_raw_skips_smoothing` checks this.
- Orbit input was already raw per-pixel and is unchanged. The pitch clamp is kept.
- `mouse_look` now defaults on. A missing field loads on; a stored `false` still loads off.

## Why there is no before/after image
The change only shows while the mouse is moving. There is no input-replay harness on the box, so before and after stills would look the same. No images or frame-times are committed.

## Check by hand
1. Run the client in the yard with mouse look on.
2. With `raw_look` off, move the mouse: the camera eases toward the orbit target.
3. Set `"raw_look": true` in `powrush_settings.json`, then move the mouse again: the camera lands on the orbit target the same frame, with no follow smoothing. The pitch clamp still applies.
