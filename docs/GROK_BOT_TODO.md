# GROK_BOT_TODO.md

Agentic follow-up.
Do not rewrite harvest_feel.rs or rbe_allocate_choice.rs.
Surgical hooks only. No server. No Ra-Thor path dep.

Repo: https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO
Branch: feat/hour-two-resume-23.2.31
Workspace: 21.88.0
Design tick: 23.2.31

## Already done (do not redo)

- First hour A+B + Workstream D (PR 209–215)
- Hour-two door + pack + card (PR 216–218 / 23.2.28–23.2.30)
- handle_interact_harvest stays at 16 Bevy 0.14 params

## This slice

Resume: welcome slab speaks Hour two held. Card skips to held if pack.complete.
No Embassy.

```bash
cargo test -p powrush-client --lib first_session_guidance
cargo test -p powrush-client --lib first_harvest
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
```

## After 23.2.31 is green

- Merge
- Do not add Embassy / Crownstone / Sylvaris until a human times hour two

## Do not do in this pass

- Steam / WebXR / k8s / payments
- Unpark server/ simulation/ host/
- New HUD
- Rewrite harvest_feel.rs or rbe_allocate_choice.rs
- Path-dep on Ra-Thor
