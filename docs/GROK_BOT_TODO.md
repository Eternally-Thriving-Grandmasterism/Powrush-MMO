# GROK_BOT_TODO.md

Agentic follow-up.
Do not rewrite harvest_feel.rs or rbe_allocate_choice.rs.
Surgical hooks only. No server. No Ra-Thor path dep.

Repo: https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO
Branch: feat/hour-two-charter-door-23.2.28
Workspace: 21.88.0
Design tick: 23.2.28

## Already done (do not redo)

- First hour A+B + Workstream D (PR 209–215)
- handle_interact_harvest stays at 16 Bevy 0.14 params

## This slice

Hour two door: allocate → Tab ridge → Q House → spill witness → L Bind/Escort.

```bash
cargo test -p shared space_law
cargo test -p powrush-client --lib hour_sacred
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
```

## After 23.2.28 is green

- Merge
- Workstream C item 1+2 become playable
- Do not add Embassy / Crownstone / Sylvaris until hour two is timed

## Do not do in this pass

- Steam / WebXR / k8s / payments
- Unpark server/ simulation/ host/
- New HUD
- Rewrite harvest_feel.rs or rbe_allocate_choice.rs
- Path-dep on Ra-Thor
