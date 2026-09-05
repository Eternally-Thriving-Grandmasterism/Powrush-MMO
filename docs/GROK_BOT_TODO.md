# GROK_BOT_TODO.md

Agentic follow-up.
Do not rewrite harvest_feel.rs or rbe_allocate_choice.rs.
Surgical hooks only. No server. No Ra-Thor path dep.

Repo: https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO
Branch: feat/hour-two-card-23.2.30
Workspace: 21.88.0
Design tick: 23.2.30

## Already done (do not redo)

- First hour A+B + Workstream D (PR 209–215)
- Hour-two door + pack (PR 216–217 / 23.2.28–23.2.29)
- handle_interact_harvest stays at 16 Bevy 0.14 params

## This slice

Same card teaches Tab · Q · L · Settled. No new HUD. No Embassy.

```bash
cargo test -p powrush-client --lib first_session_guidance
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
```

## After 23.2.30 is green

- Merge
- Do not add Embassy / Crownstone / Sylvaris until a human times hour two

## Do not do in this pass

- Steam / WebXR / k8s / payments
- Unpark server/ simulation/ host/
- New HUD
- Rewrite harvest_feel.rs or rbe_allocate_choice.rs
- Path-dep on Ra-Thor
