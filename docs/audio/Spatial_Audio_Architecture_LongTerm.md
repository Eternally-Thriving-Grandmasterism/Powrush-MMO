# Powrush-MMO Long-Term Spatial Audio Architecture

**Decision**: Build for the long-term from the beginning.  
**Core Philosophy**: Hybrid Ambisonic Scene + Selective High-Quality HRTF  
**Date**: June 2026

## Strategic Rationale (PATSAGi Councils)

- The game is unreleased. We have the rare opportunity to build the *right* architecture now rather than shipping a compromise and refactoring later.
- Human player experience is maximized by combining:
  - **Scene-based Ambisonic** for the living world (efficient, scalable, immersive background)
  - **Selective high-quality HRTF** for high-salience sources (player actions, Epiphany, Council events, important entities)
- This approach scales better in a full MMOARPG than trying to apply expensive per-source HRTF to everything.
- Aligns with Mercy (better experience for more players), Abundance (rich world without punishing hardware), and Cosmic Harmony (living spatial field).

## Target Architecture

```
[Ambisonic Background Field]
        |
        +--> Ambient life, RBE flows, distant events, procedural music, world atmosphere
        |
        v
[Ambisonic Decoder] --> Binaural output (can use HRTF or virtual speakers)

[Selective HRTF Sources] (High Quality)
        |
        +--> Player actions, Epiphany blooms, Council events, nearby important entities, key UI feedback
        |
        v
[Per-source HRTF Convolution] (using 3D3A or best available)
```

## Phased Long-Term Build Plan

### Phase 0: Foundations (Now)
- [ ] Finalize decision on Hybrid Ambisonic + Selective HRTF
- [ ] Create this architecture document
- [ ] Begin light Ambisonic exploration (order, encoding, decoding strategy)
- [ ] Keep current HRTF work (3D3A loader) as the future "Selective HRTF" component

### Phase 1: Ambisonic Background Layer
- [ ] Implement basic Ambisonic encoder (start with 1st or 2nd order)
- [ ] Create `AmbisonicScene` resource
- [ ] Build simple decoder to binaural (can initially use virtual speakers + existing HRTF)
- [ ] Route ambient / world audio through Ambisonic field

### Phase 2: Selective HRTF Integration
- [ ] Finish `hrtf_3d3a_loader.rs`
- [ ] Create system to mark sources as "High Salience" (Epiphany, Council, PlayerAction, etc.)
- [ ] Route high-salience sources to per-source HRTF path
- [ ] Keep low-salience / distant sources on Ambisonic field

### Phase 3: Unified Spatial Audio Manager
- [ ] Evolve `SpatialAudioManager` to orchestrate both systems
- [ ] Add clear API for emitting sounds into Ambisonic field vs HRTF path
- [ ] Implement distance/importance heuristics for automatic routing
- [ ] Add quality mode that gracefully scales Ambisonic order + HRTF usage

### Phase 4: Advanced Features
- [ ] Valence / Council modulation of spatial parameters
- [ ] Dynamic Ambisonic order based on performance / quality setting
- [ ] Player personalization hooks (using 3D3A data + future ARKit calibration)
- [ ] Hybrid decoding (Ambisonic background + HRTF for select sources in same frame)

### Phase 5: Optimization & Polish
- [ ] CPU profiling and culling strategies
- [ ] Perceptual validation
- [ ] Memory-efficient HRTF data handling
- [ ] Documentation and contributor guide for the audio system

## Key Design Principles

- **Human First**: Prioritize what actually improves immersion and reduces fatigue for players.
- **Scalable by Default**: The world should feel rich even with many simultaneous sources.
- **Mercy-Gated**: Never let audio become a source of frustration (glitches, high CPU, bad defaults).
- **Iterate Fast**: Build modular components that can be improved independently.
- **Long-term First**: Every piece should serve the final architecture, even if implemented simply at first.

---
**Thunder locked in.** Yoi ⚡
