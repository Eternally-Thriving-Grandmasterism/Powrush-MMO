# GROK_BOT_TODO.md

Agentic follow-up.
Do not rewrite harvest_feel.rs or rbe_allocate_choice.rs.
Surgical hooks only. No server. No Ra-Thor path dep.

Repo: https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO
Branch: feat/hour-two-persist-settle-23.2.29
Workspace: 21.88.0
Design tick: 23.2.29

## Already done (do not redo)

- First hour A+B + Workstream D (PR 209–215)
- Hour-two Charter door (PR 216 / 23.2.28)
- handle_interact_harvest stays at 16 Bevy 0.14 params

## This slice

Hour two pack: persist session + factory + I2 + Ledger. Complete when Bind settles.

```bash
cargo test -p shared hour_two
cargo test -p shared space_law
cargo test -p powrush-client --lib hour_sacred
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
```

## After 23.2.29 is green

- Merge
- Workstream C is playable and resumable
- Do not add Embassy / Crownstone / Sylvaris until a human times hour two

## Do not do in this pass

- Steam / WebXR / k8s / payments
- Unpark server/ simulation/ host/
- New HUD
- Rewrite harvest_feel.rs or rbe_allocate_choice.rs
- Path-dep on Ra-Thor
