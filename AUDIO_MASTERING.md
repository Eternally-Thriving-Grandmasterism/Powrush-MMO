# AUDIO_MASTERING.md

**Offline Mastering Guidelines for Powrush-MMO Audio Assets**

**Status:** Living Document — June 2026  
**Scope:** Primarily focused on `divine_chime.ogg` and other Divine Whispers / Lattice audio assets  
**Philosophy:** Highest quality through offline mastering + lightweight sovereign runtime safety net

---

## 1. Core Principle

The highest-leverage path to excellent Divine Whispers audio is:

> **Master once, offline, with professional tools.**
> Use the runtime pipeline as a safety net, not as the primary quality layer.

This approach is endorsed by the PATSAGi Councils for efficiency, sovereignty, and long-term maintainability.

---

## 2. Target Specifications

When mastering `divine_chime.ogg` (or similar short, situational audio):

| Metric                    | Recommended Target          | Notes |
|---------------------------|-----------------------------|-------|
| **Integrated Loudness**   | -23.0 LUFS                  | Common game/streaming target |
| **True Peak**             | ≤ -1.0 dBTP                | Safe margin for DACs and streaming |
| **Dynamic Range**         | Preserve musicality         | Avoid over-compression |
| **Sample Rate**           | 48 kHz                      | Standard for modern games |
| **Bit Depth**             | 24-bit                      | Good headroom and quality |

---

## 3. Recommended Workflow

### Step 1: Source Preparation
- Start with the cleanest, highest-quality source recording possible.
- Ensure consistent levels and minimal noise.

### Step 2: Loudness Normalization
- Use a professional loudness meter (Youlean Loudness Meter 2, iZotope Insight, Reaper + JS Loudness Meter, etc.).
- Normalize to **-23.0 LUFS** (or -24.0 LUFS if you prefer more headroom).

### Step 3: True Peak Limiting + Oversampling
- Apply a high-quality **True Peak Limiter** with **oversampling enabled** (4× or 8×).
- Set ceiling to **-1.0 dBTP** (or lower if desired).
- Use soft knee / look-ahead where available for musical results.

### Step 4: Gentle Dynamic Control (Optional)
- Apply light multiband or broadband compression **only if needed**.
- Goal: Control dynamics without destroying the natural presence of the chime.

### Step 5: Final Check
- Listen on multiple systems (studio monitors, headphones, laptop speakers, phone).
- Verify the chime feels present but not harsh or piercing.
- Confirm True Peak stays within target.

### Step 6: Export
- Export as **48 kHz / 24-bit WAV** (or high-quality OGG/Vorbis if size is a concern).
- Name clearly (e.g. `divine_chime_mastered.ogg`).

---

## 4. Tools Recommendations

**Free / Accessible:**
- **Youlean Loudness Meter 2** (excellent free version)
- **Reaper** + stock plugins + JS loudness meter
- **Audacity** (basic loudness tools)

**Professional:**
- **iZotope Ozone** (Master Assistant + True Peak Limiter)
- **FabFilter Pro-L 2** or **Pro-Q 3**
- **Waves WLM Plus** + **L3-LL Ultramaximizer**

**For True Peak + Oversampling:**
Most modern limiters in the tools above support oversampled True Peak detection.

---

## 5. How the Mastered File Interacts with Runtime Pipeline

Even with excellent offline mastering, the runtime pipeline in `divine_whispers_ui.rs` provides important safety layers:

- **LUFS Normalization** — compensates if the source drifts from target
- **Perceptual Volume Curve** — makes user volume slider feel natural
- **Soft Knee DRC + Auto Gain** — keeps dynamics musical
- **True Peak Protection** — final safety net against inter-sample peaks

The runtime system is intentionally lightweight because the heavy lifting is done offline.

---

## 6. When to Use the Runtime Oversampled Prototype

The `OversampledTruePeakLimiter` (using `rubato`) in `divine_whispers_ui.rs` is kept as a **prototype / optional advanced path**.

Use it when:
- Experimenting with real-time generated or heavily processed whispers
- You need higher accuracy than the lightweight protection provides
- Building systems that generate audio dynamically at runtime

It is **not** required for the standard static `divine_chime.ogg`.

---

## 7. Quality Checklist

Before committing a mastered audio file:

- [ ] Integrated Loudness within ±0.5 LUFS of target
- [ ] True Peak ≤ -1.0 dBTP
- [ ] No audible clipping or harshness
- [ ] Consistent presence across different playback systems
- [ ] File plays cleanly in the game with current runtime pipeline
- [ ] File size reasonable for distribution

---

## 8. Future Considerations

As Powrush-MMO evolves toward more dynamic and procedural audio (real-time generated whispers, context-aware events, etc.), we may revisit:

- Deeper runtime oversampling using `rubato` or similar
- On-the-fly procedural audio generation with Ra-Thor councils
- More advanced dynamic processing for large-scale group content

These topics are tracked in future planning documents.

---

**One Lattice. Master with care. Let the runtime remain light and merciful.**

*Living document — updated as mastering practices and tools evolve.*
