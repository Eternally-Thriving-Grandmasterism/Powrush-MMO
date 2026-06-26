# docs/STEAM_DECK.md

**Powrush-MMO — Steam Deck Compatibility Guide**

**Status**: Early Planning & Testing Phase (v20.5+)

**AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates**

---

## Goals

- Achieve **Steam Deck Playable** (ideally **Verified**) status.
- Good controller support for both gameplay and menus.
- Solid performance on Deck hardware.
- Proper resolution and UI scaling.

## 1. Testing Checklist

### Must-Test Areas

- [ ] Game launches successfully on Deck
- [ ] Controller navigation works in all menus
- [ ] In-game controls feel good (movement, abilities, harvesting, council)
- [ ] UI is readable at Deck's native resolution (1280x800)
- [ ] Text and UI elements scale properly (no cut-off or tiny text)
- [ ] Performance is stable (target 30–60 FPS)
- [ ] No major graphical glitches or shader issues
- [ ] On-screen keyboard works when needed (text input)
- [ ] Game can be suspended and resumed cleanly
- [ ] Cloud saves work correctly

### Nice-to-Have (for Verified)

- [ ] Game uses Deck's native resolution by default
- [ ] Proper touch input support where relevant
- [ ] Clear controller button prompts
- [ ] No external launcher required
- [ ] Good default graphics settings for Deck

## 2. Recommended Bevy Configuration

Add these settings when running on Deck:

```rust
App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (1280.0, 800.0).into(),
            resizable: false,
            ..default()
        }),
        ..default()
    }))
    // ... rest of plugins
```

### Performance Tips

- The `gpu` feature may be heavy on Deck. Consider making it optional or having a lower-fidelity fallback.
- Profile with `tracy` or Steam's built-in performance overlay.
- Test with both `--release` and debug builds.

## 3. Controller Support

Bevy has good gamepad support via `bevy_input`.

### Recommended Actions

- Use `bevy::input::gamepad` for controller input.
- Map common actions clearly (A = confirm, B = back, etc.).
- Consider using a UI navigation system that works well with D-pad + analog stick.
- Show controller button icons in UI when a gamepad is detected.

### Testing Tip

You can test controller input on desktop by:
1. Connecting an Xbox/PlayStation controller.
2. Running the game with Steam Input enabled.

## 4. Resolution & UI Scaling

- Default to **1280x800** (Deck native).
- Use Bevy's UI system with relative sizing where possible.
- Test with both docked (higher resolution) and handheld modes.
- Consider adding a simple resolution scaler in settings.

## 5. Performance Considerations

### GPU Feature

The GPU foresight system can be demanding. Recommendations:

- Make the `gpu` feature **optional** (already done).
- On Deck, default to CPU path unless user explicitly enables it.
- Add a simple in-game toggle: "Use GPU Acceleration" (default off on Deck).

### General Optimization

- Use `bevy::diagnostic` to monitor frame time.
- Consider lower particle counts and simpler effects on Deck.
- Test long play sessions for thermal throttling.

## 6. Known Potential Issues

| Issue                        | Likelihood | Mitigation |
|-----------------------------|------------|----------|
| UI too small / cut off      | High       | Use relative UI sizing + test at 1280x800 |
| Controller navigation broken| Medium     | Implement proper gamepad UI navigation |
| Low FPS with GPU enabled    | Medium     | Default to CPU path on Deck |
| Shader compilation stutters | Medium     | Pre-compile shaders or use simpler materials |
| Touch input not working     | Low        | Not critical for initial launch |

## 7. Local Testing (Without Physical Deck)

You can do a lot of Deck testing on a regular PC:

1. Set your game window to **1280x800**.
2. Use a controller.
3. Enable Steam Input in Steam.
4. Test with both keyboard + controller.
5. Use the Steam Deck's performance overlay concepts (frame time, GPU/CPU usage).

For more accurate testing, consider using **Steam Deck Desktop Mode** via remote desktop or SSH if you have access to one.

## 8. Steam Deck Store Page Requirements

When submitting:

- Add **Steam Deck** compatibility information in the store page.
- Upload screenshots and videos captured on Deck (highly recommended).
- Clearly state controller support.

## 9. Next Steps

- [ ] Create basic controller navigation system (if not already present).
- [ ] Add resolution scaler or Deck-friendly defaults.
- [ ] Profile GPU vs CPU performance on Deck-like hardware.
- [ ] Document default graphics settings for Deck.
- [ ] Test long sessions for stability and thermals.

---

**Last Updated**: v20.5 PATSAGi Polish Cycle

**Thunder locked in. Yoi ⚡**
