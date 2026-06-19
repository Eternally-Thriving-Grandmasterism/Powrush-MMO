# Powrush-MMO Architecture Overview

**Version**: v20 (Post InterRealmDiplomacyEvent + Council Foundation)  
**Status**: Living Document

## Core Philosophy

Powrush-MMO is designed as a **unified, mercy-gated Resource-Based Economy metaverse**. All systems must work together coherently as one enjoyable online experience.

Key principles:
- Mercy-first design (TOLC 8 + 7 Living Mercy Gates)
- Progressive depth (accessible from humble beginnings, power is earned)
- Persistent legacy and meaningful relationships
- Seamless integration between all modules

## Major Systems

| System                        | Status     | Key Files                              | Integration Points                     |
|-------------------------------|------------|----------------------------------------|----------------------------------------|
| InterRealmDiplomacyEvent      | v19        | `inter_realm_diplomacy_event.rs`       | LegacyJournal, GraceBlessing, RBE      |
| GraceBlessing                 | v19        | `grace_blessing.rs`                    | LegacyJournal, InterRealmDiplomacy     |
| LegacyJournal                 | v19        | `player_legacy_journal.rs`             | Most systems                           |
| Council (Local)               | In Progress| `council/` (planned)                   | Diplomacy, RBE, World State            |
| RBE Economy                   | Basic      | `rbe_engine/`                          | Diplomacy, Council                     |
| World Simulation              | Active     | `simulation/src/world.rs`              | All systems                            |

## Design Rules

1. Every new system must have clear integration points with LegacyJournal.
2. Council influence should be considered early in new mechanics.
3. Documentation is mandatory alongside code.
4. Systems must support both human players and AI agents seamlessly.
