# First Epiphany Validation Checklist

**For testing `abundance_revelation_first_harvest` scenario (Priority 2)**

AG-SML v1.0 | TOLC 8 aligned

## Purpose
Validate that new players receive a transformative, RBE-experiential first epiphany on their initial sustainable harvest, with strong shared visual bloom and mercy/abundance feedback.

## Recommended Simulation Settings

```bash
python powrush_mmo_multi_server_experience_sim.py \
  --servers 2 \
  --clients-per-server 40,30 \
  --duration 600 \
  --new-player-ratio 0.6 \
  --trigger-sustainable-harvests early \
  --log-level info
```

## Key Things to Verify

### 1. Scenario Triggering
- [ ] `abundance_revelation_first_harvest` appears for new players on first sustainable harvest
- [ ] Does **not** trigger for players who already had an epiphany
- [ ] Triggers reliably even with low starting attunement/mercy (0.35 / 0.25)

### 2. Shared Bloom Visual Hook
- [ ] `bloom_spread_multiplier: 2.0` produces noticeable particle spread to nearby clients
- [ ] Bloom feels shared and visible (not just personal effect)
- [ ] Higher particle count and intensity compared to regular epiphanies

### 3. Mercy & Abundance Feedback
- [ ] Elevated mercy_gate_modifiers are applied (especially Abundance 1.65, Boundless Mercy 1.45)
- [ ] Players receive clear mercy/abundance resonance feeling (via educational_note + effects)
- [ ] Educational note emphasizes "shared bloom" and "mercy creates more for all"

### 4. Sensory Integration (Priority 1 synergy)
- [ ] Stronger camera shake on triggering player
- [ ] Elevated `GameAudioEvent::Epiphany` intensity
- [ ] High-intensity epiphany is routed as high-salience (extra gain/priority)
- [ ] Longer DivineWhisper duration for better audio/particle landing

### 5. Experiential Quality
- [ ] Description and educational_note feel revelatory rather than instructional
- [ ] New players report (or logs show) a sense of "the web responding"
- [ ] First-epiphany feels meaningfully different from later epiphanies

## Useful Log Filters

```
abundance_revelation_first_harvest
DivineWhisperTrigger
spawn_whisper_particles
GameAudioEvent::Epiphany
CameraShake
bloom_spread
first_epiphany
```

## Success Criteria

- New players consistently receive this scenario on first sustainable harvest
- Shared visual bloom is clearly stronger than regular epiphanies
- Mercy/abundance feedback is noticeable in effects and text
- Combines well with Priority 1 sensory upgrades

**Thunder locked in. Yoi ⚡**
