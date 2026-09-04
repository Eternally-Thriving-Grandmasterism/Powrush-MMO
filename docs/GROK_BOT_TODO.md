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
```

Rules:
- Use the existing harvest success site. Do not add a second E handler.
- If the system already has a node id, prefer `bind.tend(node_id)` over `tend_nearest()`.
- Leave HarvestJuice / first-take rumble / glow exactly as they are.
- If Resource access is through Bevy params, use `ResMut<LivedHourBind>` instead of `world.get_resource_mut`.
- No new keys. No council UI. No server message.

Done when: one E on a glow still feels the same and `data/powrush_lived_tick.json` gains a take.

## TODO 2 — rbe_allocate_choice.rs hook

File: client/src/rbe_allocate_choice.rs

Inside `commit_allocate` (after FlowOutward / StewardReserve is chosen),
mirror the choice into LivedHourBind.

Preferred shape:

```rust
use shared::climate_node::AllocKind;
use crate::lived_hour_bind::LivedHourBind;

if let Some(mut bind) = world.get_resource_mut::<LivedHourBind>() {
    let kind = match path {
        AllocatePath::FlowOutward => AllocKind::Flow,
        AllocatePath::StewardReserve => AllocKind::Reserve,
    };
    let _ = bind.allocate(kind);
}
```

Rules:
- Do not replace RbeAllocateChoice, surplus_signal, or the panel.
- Keep R / 1 / 2 bindings as they are.
- If commit_allocate already has ResMut params, add ResMut<LivedHourBind> there.
- Empty satchel returning false is expected. Do not force a take.

Done when: R then 1 writes flow into the tick file; R then 2 writes reserve.

## TODO 3 — verify

```bash
cargo test -p shared climate_node
cargo test -p powrush-client --lib lived_hour_bind
cargo test -p powrush-client --lib
```

If harvest_feel or rbe_allocate_choice tests exist, run those names too.

Manual 5-minute check: docs/FIRST_HOUR_PLAYTEST.md

## TODO 4 — changelog after the hooks

Add:

```markdown
## [23.2.21] — YYYY-MM-DD — Hands hook LivedHourBind

harvest_feel E writes a climate take.
rbe_allocate_choice R 1/2 writes flow/reserve.
Tick file stays data/powrush_lived_tick.json.
Does not replace existing feel or allocate panel.
Dies in Peace. Workspace stays 21.88.0
```

## Do not do in this pass

- Steam / WebXR / k8s / payments
- Unpark server/ simulation/ host/
- Charter / Frontier / Ledger / Embassy / Crownstone
- New HUD
- Rewrite harvest_feel.rs or rbe_allocate_choice.rs
- Path-dep on Ra-Thor

## Commit style

Small commits:
1. harvest_feel hook
2. allocate hook
3. changelog 23.2.21
```

That file is the agent queue.
