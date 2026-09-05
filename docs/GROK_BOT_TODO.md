# GROK_BOT_TODO.md

Agentic follow-up. Human landed the first-hour bind on phone.
Do not rewrite harvest_feel.rs or rbe_allocate_choice.rs.
Surgical hooks only. Keep Peace hour solo. No server. No Ra-Thor path dep.

Repo: https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO
Branch: feat/archive-stubs-23.2.27
Workspace: 21.88.0
Design tick: 23.2.27

## Already done (do not redo)

- shared/climate_node.rs + climate_script.rs
- PR 209–214 merged (hooks through hygiene banners)
- FirstSessionGuidance is the single onboarding card
- handle_interact_harvest stays at 16 Bevy 0.14 params
- Workstream D banners + archive index + root recovery stubs

## TODO 3 — verify (docs-only; Core CI still green)

```bash
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
```

Manual: open RECOVERY_INTEGRITY_REPORT_v18.96.md — stub first, permalink second.
Playtest step 8: E twice then R 1 on Sanctuary.

## After 23.2.27 is green

- Merge this PR
- Remaining A: remappable E/I/R (sealed — would rewrite harvest_feel / rbe_allocate_choice)
- Workstream D is complete
- Do not start second-hour charter/frontier work until explicitly requested

## Do not do in this pass

- Steam / WebXR / k8s / payments
- Unpark server/ simulation/ host/
- Charter / Frontier / Ledger / Embassy / Crownstone
- New HUD
- Rewrite harvest_feel.rs or rbe_allocate_choice.rs
- Path-dep on Ra-Thor
