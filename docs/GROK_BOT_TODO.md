# GROK_BOT_TODO.md

Agentic follow-up. Human landed the first-hour bind on phone.
Do not rewrite harvest_feel.rs or rbe_allocate_choice.rs.
Surgical hooks only. Keep Peace hour solo. No server. No Ra-Thor path dep.

Repo: https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO
Branch: main
Workspace: 21.88.0
Design tick after this work: 23.2.21

## Already done (do not redo)

- shared/climate_node.rs
- shared prelude export of LivedHour types
- client/src/lived_hour_bind.rs
- pub mod lived_hour_bind
- LivedHourEconomyPlugin → LivedHourBindPlugin
- CHANGELOG 23.2.19 and 23.2.20
- docs: COMPLETION_BRIEF, PARKED_SURFACES, FIRST_HOUR_PLAYTEST,
  RBE_FIRST_HOUR, SECOND_HOUR_CHARTER_FRONTIER, FIRST_HOUR_CLIENT_BIND

## TODO 1 — harvest_feel.rs hook

File: client/src/harvest_feel.rs

After a successful first-hour E / tend (the existing first-take juice path),
call LivedHourBind without changing camera punch or rumble.

Preferred shape:

```rust
if let Some(mut bind) = world.get_resource_mut::<crate::lived_hour_bind::LivedHourBind>() {
    let _ = bind.tend_nearest();
}
