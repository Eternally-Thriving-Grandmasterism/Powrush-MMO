# Stability batch — spent on main

**Tip read:** `1edf721` after #645. Audit file `docs/LAUNCH_READINESS_AUDIT.md` still describes `6db11a7f`. Do not re-cook these cards.

## Spent

- **LR-01 SAVE-NAMED-ATOMIC-1.** `shared/user_persist.rs` `write_named` copies a good live file to `.bak`, writes a sibling `.tmp`, then `fs::rename`. A torn `.tmp` does not replace the live file. `read_named` falls back to `.bak`.
- **LR-02 SAVE-HOUR-SET-1.** `client/src/lived_hour_bind.rs` `write_hour_set` writes climate, standing, week, then the tick through `write_named`. A failed file is warned. The rest still run. Tick is the commit marker, not a new schema.
- **LR-03 JOURNEY-SAVE-QUIET-1.** `client/src/abundance_journey_echo.rs` writes only when the serialized journey changes, through `write_named`. An idle boot does not create the file.
- **LR-04 CRASH-LOG-HOOK-1.** `client/src/main.rs` sets a panic hook before `App::new()` and appends `crash.log` in the user dir. The name is not `powrush_*`.
- **LR-08 LIVE-MOUTH-TICK-1.** `docs/LIVE_MOUTH_AUDIT.md` already says `write_lived_tick` writes nothing and `LivedHourBind::persist` owns the tick path.

## Still refused

- **LR-07.** Retired: #673 deleted `.github/workflows/sync-from-ra-thor.yml` at 8b8fa141. Ra-Thor contributes through PRs into this repo with no cross-repo sync.

## Not this stamp

Playtest minutes, Online, Steam, meshes, Bevy, payments, and a combat face stay uncooked. Online grey.

Peak memory: walked · tended · week was the bill · yard remembered
