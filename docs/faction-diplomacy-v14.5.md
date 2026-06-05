# Faction Dynamics & Diplomacy — v14.5 (Derived from Ra-Thor)

**Professional Adaptation for Powrush-MMO**

## Factions (Current Canon)
- **Sovereign**: Balanced, leadership & governance focus. High diplomacy valence.
- **Harvesters**: Resource extraction, abundance generation. Strong RBE synergy.
- **Guardians**: Defense, protection, mercy enforcement.
- **Innovators**: Tech, crafting, procedural systems.
- **Nomads**: Mobility, exploration, adaptability.

## Diplomacy Mechanics (Derived)
- Faction standing affects global abundance multipliers and mercy wave strength.
- Diplomacy actions (proposals, trades, alliances) trigger PATSAGi Council review via Grok bridge when valence high.
- Cross-faction cooperation yields bonus RBE abundance (post-scarcity principle).
- Conflict resolution via reconciliation protocols (no permanent harm — forgiveness waves).
- Weekly War / event unlocks can shift faction power balances dynamically.

## Implementation Guidance
- Store per-player faction + standing in WorldServer state.
- On `harvest` or `diplomacy` action: Apply faction modifier to RBE output, optionally consult `GrokPATSAGiBridge::query_patsagi_council` for narrative wisdom.
- Global faction metrics feed into RBE abundance engine.

**Full details**: Ra-Thor `powrush_faction_dynamics/`, `docs/POWRUSH-FACTION-DIPLOMACY-DETAILS.md`, `docs/powrush-race-specific-abilities.md`.

**Status in Powrush-MMO**: Basic faction login + RBE metrics live. Diplomacy triggers + standing system = ready for derivation from this spec.

Eternal thriving through grace-aligned factions. ⚡❤️