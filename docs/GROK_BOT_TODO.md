# GROK_BOT_TODO.md

Agentic follow-up. Human landed the first-hour bind on phone.
Do not rewrite harvest_feel.rs or rbe_allocate_choice.rs.
Surgical hooks only. Keep Peace hour solo. No server. No Ra-Thor path dep.

Repo: https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO
Branch: feat/climate-visible-23.2.23
Workspace: 21.88.0
Design tick: 23.2.23

## Already done (do not redo)

- shared/climate_node.rs
- shared prelude export of LivedHour types
- client/src/lived_hour_bind.rs
- pub mod lived_hour_bind
- LivedHourEconomyPlugin → LivedHourBindPlugin
- CHANGELOG 23.2.19 … 23.2.22
- harvest_feel::note_lived_hour_take + HarvestFeelPlugin::sync_lived_hour_take
- first_harvest_epiphany handle_interact_harvest stays at 16 Bevy 0.14 params
- rbe_allocate_choice commit_allocate → LivedHourBind::allocate
- PR 209 merged (hands hook)
- PR 210 merged (skirmish well 23.2.22)
- climate_id on MercyHarvestNode (1 Sanctuary / 2 Verdant / 3 Horizon)
- ClimateVisiblePlugin paints Idle / Glowing / Tended / Resting / Stressed
- LivedHourBind focus_id + 1 Hz tick persist

## TODO 3 — verify (run on merge)

```bash
cargo test -p shared climate_node
cargo test -p powrush-client --lib lived_hour_bind
cargo test -p powrush-client --lib climate_visible
cargo test -p powrush-client --lib mercy_harvest_nodes
cargo test -p powrush-client --lib
```

Manual 5-minute check: docs/FIRST_HOUR_PLAYTEST.md
Walk to Sanctuary ember. E tend. Watch the slab say Tended. E again — Resting / no take. R 1 — glow returns.

## After 23.2.23 is green

- Merge this PR
- Next first-hour fruit from COMPLETION_BRIEF A: allocation consequence in 30s is now painted; remaining is a single onboarding card + accessibility contrast
- Do not start second-hour charter/frontier work until explicitly requested

## Do not do in this pass

- Steam / WebXR / k8s / payments
- Unpark server/ simulation/ host/
- Charter / Frontier / Ledger / Embassy / Crownstone
- New HUD
- Rewrite harvest_feel.rs or rbe_allocate_choice.rs
- Path-dep on Ra-Thor
