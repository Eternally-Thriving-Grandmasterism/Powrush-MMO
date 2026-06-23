# HRTF Database Comparison: 3D3A vs MIT KEMAR

**Date**: June 2026  
**Context**: Powrush-MMO Audio Engine — Choosing primary high-quality HRTF source

## Overview

| Aspect                    | **3D3A Lab (Princeton)**                  | **MIT KEMAR**                          | Winner for Powrush-MMO      |
|---------------------------|-------------------------------------------|----------------------------------------|-----------------------------|
| **Subjects**              | 32+ human subjects                        | 1 mannequin head (KEMAR)               | 3D3A                        |
| **Data Types**            | Measured + Numerically Computed HRTFs     | Measured only                          | 3D3A                        |
| **3D / Anthropometric**   | High-quality 3D head + torso scans        | None                                   | 3D3A                        |
| **License**               | CC-BY-4.0 (clear attribution required)    | Public domain / very permissive        | MIT KEMAR (simpler)         |
| **Diversity**             | Good variety of head/ear shapes           | Single "average" head                | 3D3A                        |
| **Personalization Potential** | High (with 3D scans)                   | Very Low                               | 3D3A                        |
| **Current Usage**         | Not yet integrated                        | Current baseline (mit_kemar)           | -                           |
| **Maturity / Stability**  | Modern and actively maintained            | Extremely well-known and stable        | MIT KEMAR (for fallback)    |
| **Game Suitability**      | Excellent for premium High quality mode   | Excellent safe default                 | Hybrid approach             |

## Detailed Comparison

### 1. Subject Diversity & Realism
- **3D3A**: Real human subjects with natural variation in head size, pinna shape, etc. Much closer to actual player anatomy.
- **MIT KEMAR**: Single standardized mannequin. Good average case but can cause front/back confusion or poor elevation for many listeners.

### 2. Data Quality & Completeness
- **3D3A**: Offers both measured HRTFs and high-quality numerically computed versions from 3D scans. Very rich dataset.
- **MIT KEMAR**: Classic high-quality measurements. Still excellent, but limited to one morphology.

### 3. Future-Proofing (Personalization)
- **3D3A**: Includes 3D head/torso scans → strong foundation for future anthropometric or ML-based personalization.
- **MIT KEMAR**: No supporting 3D data. Not suitable as a base for personalization.

### 4. Licensing & Legal
- **3D3A**: CC-BY-4.0 → Must include attribution. Clear and acceptable for games.
- **MIT KEMAR**: Extremely permissive. Easier for quick integration and fallback.

## Recommended Strategy for Powrush-MMO

**Hybrid Approach (Best of Both Worlds):**

- **Default / Fallback** → Keep using **MIT KEMAR** (current `mit_kemar` path) as the safe, high-quality default for all players.
- **High Quality Mode** → Adopt **3D3A** as the premium HRTF set when the player selects High spatial quality.
- **Long-term** → Use 3D3A’s 3D scan data as the foundation for optional personalization features (via ARKit or anthropometric profile).

This gives every player excellent spatial audio immediately, while giving dedicated players a meaningfully better experience in High quality mode and opening the door for future personalization.

## Hybrid Implementation Steps

### Phase A: Foundation (Current State)
- [x] Keep existing `HrtfImpulseResponses` resource and loading system
- [x] MIT KEMAR remains the default loaded set
- [x] `SpatialQuality` enum already exists in `SpatialAudioManager`

### Phase B: 3D3A Loader Development
- [ ] Complete `game/hrtf_3d3a_loader.rs` (stub already created)
- [ ] Implement parsing of 3D3A HRTF data into `HrtfImpulseResponses` format
- [ ] Support loading both measured and computed versions
- [ ] Add function to load a curated high-quality subset

### Phase C: Quality Mode Integration
- [ ] In `SpatialAudioManager::set_spatial_quality()`:
  - When `High` is selected → attempt to load 3D3A set
  - If loading fails → gracefully fall back to MIT KEMAR + log warning
- [ ] Add a `current_hrtf_set: HrtfSet` enum (`MitKemar`, `ThreeD3A`)
- [ ] Expose which set is currently active (for UI/debug)

### Phase D: UI & Player Control
- [ ] Add setting: "Spatial Audio Quality" (Low / Medium / High)
- [ ] When High is selected, show "Using 3D3A HRTF Set" indicator (optional)
- [ ] Add proper attribution for 3D3A in audio credits screen

### Phase E: Optimization & Validation
- [ ] Memory profiling (HRTF data is large — consider lazy loading or streaming)
- [ ] Perceptual A/B testing between MIT KEMAR and 3D3A
- [ ] Ensure zero audio glitches when switching quality modes at runtime
- [ ] Document how to expand the 3D3A subject pool later

### Phase F: Future Personalization (Optional)
- [ ] Use 3D3A 3D scan data as base for anthropometric matching
- [ ] Integrate with ARKit calibration flow (planned)
- [ ] Allow players to select "Best Match" from 3D3A subjects based on simple profile

## Next Steps
- Complete Phase 1 evaluation of 3D3A data
- Begin Phase 2: Loader development in `game/hrtf_3d3a_loader.rs`
- Decide on exact subject subset to integrate first

---
**Thunder locked in.** Yoi ⚡
