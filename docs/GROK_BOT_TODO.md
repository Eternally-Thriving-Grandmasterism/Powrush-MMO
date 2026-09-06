# GROK_BOT_TODO.md

Agentic follow-up.
Do not rewrite harvest_feel.rs or rbe_allocate_choice.rs.
Surgical hooks only. No server. No Ra-Thor path dep.

Repo: https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO
Workspace: 21.88.0
Live design tip: 23.2.32 (`798352fe`)

## Already done (do not redo)

- First hour A+B + Workstream D (PR 209–215)
- Hour-two door + pack + card + resume + welcome wire (PR 216–220 / 23.2.28–23.2.32)
- handle_interact_harvest stays at 16 Bevy 0.14 params

## Now (PATSAGi)

1. Human runs `docs/HOUR_TWO_PLAYTEST.md` and drops the Report.
2. Until that Report exists: **no new Bevy systems**. Docs-only maps are allowed.
3. After the Report: wire Hour three per `docs/HOUR_THREE.md` (Proof Pack → Embassy seat → held). Merge only if Core green.

## Do not do until Hour-two Report

- Embassy / Crownstone / Sylvaris / Hybrid / Compass as new hours
- Steam / WebXR / k8s / payments
- Unpark server/ simulation/ host/
- New HUD
- Rewrite harvest_feel.rs or rbe_allocate_choice.rs
- Path-dep on Ra-Thor
