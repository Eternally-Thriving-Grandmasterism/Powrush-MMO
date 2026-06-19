# Council System Architecture

**Version**: v20  
**Model**: Hybrid (Local from start + Earned Regional/Global depth)

## Overview

The Council system is the central governance layer of Powrush-MMO. It allows players and agents to participate in meaningful decision-making that shapes realms and the broader metaverse.

## Hybrid Model (Approved by PATSAGi Councils)

- **Local Council**: Available from the very beginning on every server/realm.
- **Regional Council**: Unlocked through collective metrics (mercy, harmony, participation).
- **Global / PATSAGi-level**: Highest tier, earned through exceptional redemptive history and large-scale cooperation.

## Local Council (Current Focus)

### Responsibilities
- Handle local realm proposals
- Basic deliberation and voting
- Issue simple persistent decisions
- Feed into `CouncilDeliberationInput` for InterRealmDiplomacyEvent

### Core Components (Planned)
- `CouncilProposal`
- `CouncilSession`
- `CouncilDecision` (persistent effects)
- `CouncilState` resource

### Progression Hooks
- Metrics tracked: Average mercy, harmony, participation rate, successful redemptive resolutions
- Thresholds will unlock Regional Council capabilities

## Integration Points

| System                    | How Council Interacts                              |
|---------------------------|----------------------------------------------------|
| InterRealmDiplomacyEvent  | Provides `CouncilDeliberationInput`                |
| GraceBlessing             | Can trigger blessings after major council decisions|
| RBE Economy               | Future: Council can influence resource policies    |
| LegacyJournal             | All major decisions and proposals are recorded     |
| World Simulation          | Future: Council decisions can affect biomes        |

## Future Evolution

- Richer proposal types
- Multi-level council hierarchy
- Spectator participation in council sessions
- Persistent world effects from passed proposals
