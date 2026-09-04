# GROK_BOT_TODO.md

Agentic follow-up. Human landed the first-hour bind on phone.
Do not rewrite harvest_feel.rs or rbe_allocate_choice.rs.
Surgical hooks only. Keep Peace hour solo. No server. No Ra-Thor path dep.

Repo: https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO
Branch: feat/lived-hour-hands-hook-23.2.21
Workspace: 21.88.0
Design tick landed: 23.2.21

## Already done (do not redo)

- shared/climate_node.rs
- shared prelude export of LivedHour types
- client/src/lived_hour_bind.rs
- pub mod lived_hour_bind
- LivedHourEconomyPlugin → LivedHourBindPlugin
- CHANGELOG 23.2.19, 23.2.20, 23.2.21
- docs: COMPLETION_BRIEF, PARKED_SURFACES, FIRST_HOUR_PLAYTEST,
  RBE_FIRST_HOUR, SECOND_HOUR_CHARTER_FRONTIER, FIRST_HOUR_CLIENT_BIND
- harvest_feel::note_lived_hour_take + HarvestFeelPlugin::sync_lived_hour_take
- first_harvest_epiphany handle_interact_harvest stays at 16 Bevy 0.14 params
  (LivedHour take is mirrored from SoftRbePool.harvests, not a 17th ResMut)
- rbe_allocate_choice commit_allocate → LivedHourBind::allocate

## TODO 3 — verify (run on merge)

```bash
cargo test -p shared climate_node
cargo test -p powrush-client --lib lived_hour_bind
cargo test -p powrush-client --lib harvest_feel
cargo test -p powrush-client --lib rbe_allocate_choice
cargo test -p powrush-client --lib
```

Manual 5-minute check: docs/FIRST_HOUR_PLAYTEST.md

## After 209 is green

- Merge PR 209 into main
- Rebase PR 208 (feat/skirmish-well-23.2.19) onto main
- Do not start second-hour charter/frontier work until 208 is merged or explicitly requested

## Do not do in this pass

- Steam / WebXR / k8s / payments
- Unpark server/ simulation/ host/
- Charter / Frontier / Ledger / Embassy / Crownstone
- New HUD
- Rewrite harvest_feel.rs or rbe_allocate_choice.rs
- Path-dep on Ra-Thor
