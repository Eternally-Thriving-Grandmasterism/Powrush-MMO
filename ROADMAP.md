## v16.0 — RBE Economy Core (Resource Nodes + Harvesting) — IN PROGRESS
- ResourceNode struct with position, type, remaining, regen
- ClientMessage::HarvestResource + ServerMessage::ResourceUpdate
- PATSAGi Council validate_harvest hook (sustainable amounts)
- Authoritative tick regen + broadcast on harvest
- Integrated with InterestManager culling (nodes visible as entities)
- Mercy gate on HarvestResource
- Next: Player inventory / global abundance tracking, RBE trading, node respawn, PATSAGi abundance rituals

(Previous v15.9 polish merged)