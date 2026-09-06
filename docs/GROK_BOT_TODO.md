# GROK_BOT_TODO.md

Agentic follow-up.
Do not rewrite harvest_feel.rs or rbe_allocate_choice.rs.
Surgical hooks only. No server. No Ra-Thor path dep.

Repo: https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO
Branch: feat/hour-two-welcome-wire-23.2.32
Workspace: 21.88.0
Design tick: 23.2.32

## Already done (do not redo)

- First hour A+B + Workstream D (PR 209–215)
- Hour-two door + pack + card + resume skip (PR 216–219 / 23.2.28–23.2.31)
- handle_interact_harvest stays at 16 Bevy 0.14 params

## This slice

Wire `welcome_line` into the existing WelcomeBack slab.
When `HourSacred.complete`, the slab speaks the held-yard sentence.
No Embassy.

```bash
cargo test -p powrush-client --lib first_harvest
cargo test -p powrush-client --lib hour_two_resume
cargo test -p powrush-client --lib first_session_guidance
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
```

## After 23.2.32 is green

- Merge
- Stop. Human times hour two (docs/HOUR_TWO_PLAYTEST.md).
- Do not add Embassy / Crownstone / Sylvaris until that timing note exists.

## Do not do in this pass

- Steam / WebXR / k8s / payments
- Unpark server/ simulation/ host/
- New HUD
- Rewrite harvest_feel.rs or rbe_allocate_choice.rs
- Path-dep on Ra-Thor
