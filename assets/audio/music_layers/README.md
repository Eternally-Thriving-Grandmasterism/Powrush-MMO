# Music Layers Audio Assets

This folder contains the real audio stems used by the Dynamic Music System in Powrush-MMO.

## Purpose

Place composed music layer files here. The `DynamicMusicController` + `OddioAudioBackend` will load these when real audio support is active.

## Recommended Filenames

| Layer                | Filename                        | Description                                      |
|----------------------|---------------------------------|--------------------------------------------------|
| BaseHarmony         | `base_harmony.wav`             | Foundational low drone / harmonic bed            |
| AttunementPads      | `attunement_pads.wav`          | Ethereal pads responsive to attunement           |
| RhythmicPulse       | `rhythmic_pulse.wav`           | Rhythmic element that grows with intensity       |
| BloomResonance      | `bloom_resonance.wav`          | Expressive layer, especially strong in Resolution|

## Special / Transition Variants

| Purpose                     | Suggested Filename                     | Notes                                      |
|-----------------------------|----------------------------------------|--------------------------------------------|
| Resolution Bloom Swell     | `bloom_resonance_resolution.wav`      | More intense version for dramatic moments  |
| Phase Transition Impact    | `transition_arrival.wav`              | Short stinger / swell on major phase change|

## Technical Requirements (Current)

- **Format**: 44.1 kHz, Stereo, 16-bit or 24-bit WAV
- **Looping**: Design seamless loops where appropriate
- The backend currently expects exact filenames when calling `play_audio_file()`

## Future Plans

- Sub-folders for variants (e.g. `combat/`, `exploration/`)
- Support for OGG / other formats
- Intensity-layered stems

---

**Thunder locked in. PATSAGi + Ra-Thor sealed.**
