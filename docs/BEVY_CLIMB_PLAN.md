# BEVY CLIMB PLAN — banked 2026-09-23

**Councils:** Truth, Service, Abundance. Contact: info@Rathor.ai.
**Lived pin today:** `bevy = "0.14"` on `client/Cargo.toml` (also parked `server/`, `simulation/`, `host/`).
**Do not treat this file as permission to bump Cargo this week.**

Title Online grey. Floor `2163551`. Tag `playable-preview` `11c577e`. Workspace `21.88.0`.
Model seat: Grok 4.7 High / 4.6 High only (`docs/CURSOR_GROK_MODEL_SEAT.md`).

## Decision (locked)

| When | Do |
|---|---|
| **NOW** | Stay on Bevy **0.14**. Heartbeat core tests. TODO-SYNC if still stale. No engine port. |
| **NOT NOW** | Do not bump to 0.19.1 (layover tax). Do not bump to **0.20.0-rc.*** (input regressions). |
| **LATER** | When **Bevy 0.20.0 stable** exists on crates.io (tag `v0.20.0`, not `v0.20.0-rc.N`), open **CARD BEVY-CLIMB**. |
| **NEVER from bots** | Unreal / Unity / Godot rewrite. Bevy 1.0 wait-forever. Lighting Title Online to justify a renderer. |

There is no “Bevy 0.2”. The next minor is **0.20**.

## How bots detect “time is appropriate” (no steward ping required)

Clerk or Hands may check, at most once per heartbeat day:

```
# crates.io Bevy versions — look for exact 0.20.0 (not rc)
# GitHub tag: https://github.com/bevyengine/bevy/releases/tag/v0.20.0
```

**OPEN BEVY-CLIMB only if all are true:**

1. crates.io / GitHub show **`v0.20.0`** as a non-pre-release.
2. Tip `client/Cargo.toml` still says `bevy = "0.14"` (or 0.14.x).
3. Core gate is green on tip (`shared` + `rsil-identity` + `powrush-client --lib`).
4. 0 open feature PRs in flight (heartbeat-fix PR may finish first).
5. Footer model is Grok 4.7/4.6 High — not Codex / GPT / Claude / RC-chasing Auto.

If 0.20 is still **rc** or only 0.19.x exists → chat `BEVY-CLIMB HOLD · pin 0.14` · no PR.

## CARD BEVY-CLIMB (future PATHS — named now so you do not invent)

**First PR PATHS only:**

- `client/Cargo.toml`
- `Cargo.lock` (as produced)
- compile/API fixes **inside `client/src/**` required by official guides**

**Parked crates stay 0.14 until client gate is green on 0.20:**
`server/Cargo.toml` · `simulation/Cargo.toml` · `host/Cargo.toml` · `powrush-divine-module/Cargo.toml`
Second CARD may lift those after Dual GREEN on the client climb.

**Walk official guides in order** (do not skip to 0.20 docs only):

- 0.14 → 0.15 → 0.16 → 0.17 → 0.18 → 0.19 → 0.20
- https://bevyengine.org/learn/migration-guides/

**Success for the climb PR:**

```
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
```

All existing `q0_*` / `q1_*` pass. Title Online still grey. E / I / H / R / 1 / 2 verbs unchanged. Welcome-back slab unchanged. No new Place. No BSN rewrite of Title. No Solari-as-default. No mesh dump.

One PR then HOLD. Dual YELLOW until ready + Core SUCCESS → GREEN → squash.

## NOW queue (applicable today)

1. If `docs/GROK_BOT_TODO.md` tip is still `#486` / MESH-as-Next → **CARD TODO-SYNC** (`docs/NEXT_NAMED_CARDS.md`).
2. Else **STANDING HEARTBEAT** — run the two cargo test lines; GREEN chat or restore-only PR.
3. Re-read this file. If 0.20.0 stable is missing → do not climb.

## HOLD until steward names extra PATHS

MESH-PERSONA `.glb` · P1 rooms · F10 · Title Online · App ID · `steamworks` crate enable · `server/` unpark · Unreal/Unity/Godot.

## After 0.20 climb (even later)

Only then consider Comfort High GPU-driven path / named mesh PATHS. Still not a mothership-bay Hands cargo. Trailer stays Codex / Drive.
