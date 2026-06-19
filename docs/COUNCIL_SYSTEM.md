# Council System Architecture (Updated v20.1)

**Model**: Hybrid (Local from start + Earned Regional/Global depth)

## Local Council

### Core Components
- `CouncilProposal`
- `CouncilSession` (with `run_deliberation()`)
- `CouncilDecision` + `CouncilDecisions` resource

### Deliberation & Voting
- Proposals move through Draft → Deliberating → Passed/Rejected
- Resolution influenced by vote count and average participant mercy
- Simple but extensible logic ready for archetype and council influence

### Integration
- `get_council_deliberation_input()` provides data to `InterRealmDiplomacyEvent`
- Passed decisions stored persistently and can apply world effects

## Progression
Local Council is available immediately. Regional and Global tiers unlock based on collective mercy, harmony, and redemptive outcomes.
