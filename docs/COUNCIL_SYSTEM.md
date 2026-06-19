# Council System Architecture

**Model**: Hybrid (Local Council from start + Earned Regional/Global depth)

## Local Council

### Components
- CouncilProposal
- CouncilSession (with run_deliberation)
- CouncilDecision + CouncilDecisions resource

### Deliberation
- Simple but mercy-influenced resolution logic
- Ready for future expansion with archetype weighting and reputation

### Integration
- Provides CouncilDeliberationInput to InterRealmDiplomacyEvent
- Decisions can apply persistent world effects

## Next Steps
- Expand proposal types
- Add voting from agents/players
- Unlock Regional and Global tiers based on metrics
