# GROK_BOT_TODO.md

Agentic follow-up. Human landed the first-hour bind on phone.
Do not rewrite harvest_feel.rs or rbe_allocate_choice.rs.
Surgical hooks only. Keep Peace hour solo. No server. No Ra-Thor path dep.

Repo: https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO
Branch: feat/onboarding-card-23.2.24
Workspace: 21.88.0
Design tick: 23.2.24

## Already done (do not redo)

- shared/climate_node.rs
- shared prelude export of LivedHour types
- client/src/lived_hour_bind.rs
- pub mod lived_hour_bind
- LivedHourEconomyPlugin → LivedHourBindPlugin
- CHANGELOG 23.2.19 … 23.2.23
- harvest_feel::note_lived_hour_take + HarvestFeelPlugin::sync_lived_hour_take
- first_harvest_epiphany handle_interact_harvest stays at 16 Bevy 0.14 params
- rbe_allocate_choice commit_allocate → LivedHourBind::allocate
- PR 209 merged (hands hook)
- PR 210 merged (skirmish well 23.2.22)
- PR 211 merged (climate visible 23.2.23)
- climate_id on MercyHarvestNode (1 Sanctuary / 2 Verdant / 3 Horizon)
- ClimateVisiblePlugin paints Idle / Glowing / Tended / Resting / Stressed
- LivedHourBind focus_id + 1 Hz tick persist
- FirstSessionGuidance is the single onboarding card (walk · tend · satchel · allocate)

## TODO 3 — verify (run on merge)

```bash
cargo test -p shared climate_node
cargo test -p powrush-client --lib first_session_guidance
cargo test -p powrush-client --lib climate_visible
cargo test -p powrush-client --lib lived_hour_bind
cargo test -p powrush-client --lib
```

Manual 5-minute check: docs/FIRST_HOUR_PLAYTEST.md
Card says walk. Walk. Card says glow. E. Card says I. I. Card says R. R 1. H hides. Field still speaks.

## After 23.2.24 is green

- Merge this PR
- Next first-hour fruit from COMPLETION_BRIEF A: accessibility remappable keys (H + contrast landed; keys stay WASD/E/I/H/R)
- Workstream B: scripted mercy-restore vs extract-only climates (shared tests already cover transitions)
- Do not start second-hour charter/frontier work until explicitly requested

## Do not do in this pass

- Steam / WebXR / k8s / payments
- Unpark server/ simulation/ host/
- Charter / Frontier / Ledger / Embassy / Crownstone
- New HUD
- Rewrite harvest_feel.rs or rbe_allocate_choice.rs
- Path-dep on Ra-Thor
