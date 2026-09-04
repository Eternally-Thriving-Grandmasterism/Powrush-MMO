# GROK_BOT_TODO.md

Agentic follow-up. Human landed the first-hour bind on phone.
Do not rewrite harvest_feel.rs or rbe_allocate_choice.rs.
Surgical hooks only. Keep Peace hour solo. No server. No Ra-Thor path dep.

Repo: https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO
Branch: feat/teaching-climates-23.2.25
Workspace: 21.88.0
Design tick: 23.2.25

## Already done (do not redo)

- shared/climate_node.rs + climate_script.rs
- PR 209–212 merged (hooks, well, climate visible, onboarding card)
- FirstSessionGuidance is the single onboarding card
- handle_interact_harvest stays at 16 Bevy 0.14 params

## TODO 3 — verify (run on merge)

```bash
cargo test -p shared climate_script
cargo test -p powrush-client --lib climate_script
cargo test -p powrush-client --lib climate_visible
cargo test -p powrush-client --lib
```

Manual: E twice on Sanctuary — slab *extract left it tired*. R 1 — *flow restored the well*.

## After 23.2.25 is green

- Merge this PR
- Remaining A: remappable E/I/R (sealed — would rewrite harvest_feel / rbe_allocate_choice)
- Workstream D hygiene
- Do not start second-hour charter/frontier work until explicitly requested

## Do not do in this pass

- Steam / WebXR / k8s / payments
- Unpark server/ simulation/ host/
- Charter / Frontier / Ledger / Embassy / Crownstone
- New HUD
- Rewrite harvest_feel.rs or rbe_allocate_choice.rs
- Path-dep on Ra-Thor
