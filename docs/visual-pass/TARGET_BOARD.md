# TARGET_BOARD.md — VP-TARGET-0

Source: Sherif Botros, 2026-10-23 1:47 AM ET: *"Override the art bible and push the game toward the painted site look"*; follow-up *"Maximally."* — maximal beauty **and** maximal playability.

No binaries committed. Plates are referenced by URL only; they are look targets, never shipped assets (no art reuse across sites).

## Reference plates (powrush.com)
| Plate | URL |
|---|---|
| Quellorian waterfall city (hero) | https://powrush.com/images/art/home-quellorian-waterfall-city-1500.webp |

Add further plates by name + URL as cards cite them.

## Target goals
- **Palette:** lively valley colour, sky blues, golden-hour warms, one bold accent per place (earth tone: open question — site "muted earth" vs bible "no brown/beige earth").
- **Golden-hour light:** low warm sun, long soft shadows, warm/cool split.
- **Aerial haze:** distance desaturates to blue; behind plates, never through type.
- **Layered blue mountains:** 3+ silhouette depth bands.
- **Terraces:** stepped, readable terrain shelves.
- **Water:** falls, pools, reflective surfaces ("water is a bath").
- **Dense vegetation:** layered foliage, canopy breakup.
- **Warm lamps:** emissive points against cool dusk; glow = Use.

## Presets
- **Low / Medium:** strict frame-time budgets (budgets to be set per card), gameplay readability first; no effect may obscure plates, Use glow or Mute-safety.
- **High / Ultra:** beauty as far as Bevy 0.20 allows — SSAO, SSR, TAA, volumetric/atmospheric fog, environment-map lighting, dense vegetation. Ultra is added by **VP-ULTRA-1**.

## Ordered card plan
Acceptance for every card: before/after **Low + Medium** shots in `docs/visual-pass/<card>/` plus a side-by-side vs a named plate (High/Ultra shots too where the card touches them). **Every render card needs Core's vote.**

| # | Card | Scope | Beauty/ms rank (est.) |
|---|---|---|---|
| 1 | VP-AA-1 | anti-aliasing baseline (FXAA/SMAA Low–Med) | 4 |
| 2 | VP-AIR-1 | aerial perspective / distance haze | 1 |
| 3 | VP-LIGHT-1 | golden-hour sun, shadows, tonemapping/grade | 2 |
| 4 | VP-AO-1 | ambient occlusion (SSAO High+) | 8 |
| 5 | VP-SKY-2 | sky gradient + layered blue mountain bands | 3 |
| 6 | VP-MAT-1 | painterly materials/palette | 5 |
| 7 | VP-VEG-1 | dense vegetation (instanced, LOD by preset) | 10 |
| 8 | VP-TERRAIN-1 | terraces, terrain shaping | 7 |
| 9 | VP-WATER-1 | water, falls, reflections (SSR High+) | 9 |
| 10 | VP-GLOW-1 | warm lamps, bloom, Use glow | 6 |
| 11 | VP-CHAR-1 | character dress/material pass | 11 |
| 12 | VP-VFX-1 | capped Tend/Flow VFX | 12 |
| 13 | VP-ULTRA-1 | Ultra preset: SSAO, SSR, TAA, volumetric fog, env lighting, max vegetation (after quick wins) | 13 |

**Beauty per millisecond ranking** (estimate; re-rank with measured GPU ms after each card): AIR-1 > LIGHT-1 > SKY-2 > AA-1 > MAT-1 > GLOW-1 > TERRAIN-1 > AO-1 > WATER-1 > VEG-1 > CHAR-1 > VFX-1 > ULTRA-1. Cheap fullscreen/lighting changes first; geometry- and screen-space-heavy work later and gated to High/Ultra.
