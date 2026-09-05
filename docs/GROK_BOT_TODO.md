# GROK_BOT_TODO.md

Agentic follow-up. Human landed the first-hour bind on phone.
Do not rewrite harvest_feel.rs or rbe_allocate_choice.rs.
Surgical hooks only. Keep Peace hour solo. No server. No Ra-Thor path dep.

Repo: https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO
Branch: feat/hygiene-archive-23.2.26
Workspace: 21.88.0
Design tick: 23.2.26

## Already done (do not redo)

- shared/climate_node.rs + climate_script.rs
- PR 209–213 merged (hooks, well, climate visible, onboarding card, teaching climates)
- FirstSessionGuidance is the single onboarding card
- handle_interact_harvest stays at 16 Bevy 0.14 params
- Workstream D banners + docs/archive/README.md (this slice)

## TODO 3 — verify (docs-only; Core CI still green)

```bash
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
```

Manual: open README, then LAUNCH-CHECKLIST — banner first, verdict second.

## After 23.2.26 is green

- Merge this PR
- Remaining A: remappable E/I/R (sealed — would rewrite harvest_feel / rbe_allocate_choice)
- Remaining D: physical move of root recovery blobs into docs/archive/ (optional)
- Do not start second-hour charter/frontier work until explicitly requested

## Do not do in this pass

- Steam / WebXR / k8s / payments
- Unpark server/ simulation/ host/
- Charter / Frontier / Ledger / Embassy / Crownstone
- New HUD
- Rewrite harvest_feel.rs or rbe_allocate_choice.rs
- Path-dep on Ra-Thor
