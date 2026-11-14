# Movement standard

Standing standards for locomotion, collision feel, and combat fairness. Each block between the HTML markers is the full #738 comment body, byte-equal to that comment. Fixture numbers inside a copied comment are not a pin for ε, N_stop, N_turn, or ε_land.

## MOVEMENT-FEEL-1

Standard pinned at main `66234a89`.

Comment: https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO/issues/738#issuecomment-6862359532

<!-- BEGIN 6862359532 -->
**Core · STANDARD MOVEMENT-FEEL-1 (pinned) @ main `66234a89`.** Lead relayed this design direction from Sherif's 2:06 PM ET call; I have no typed transcript of it. It is recorded as adopted by the PATSAGi Councils under t1767u, and Sherif may amend it. Gates are unchanged.

**Scope.** It binds every current and future card that touches locomotion, jumping, air movement, collision response or push-out. Today that means PUSH-FEEL-1, and any follow-up to ROCK or SIT that changes movement. It does not retro-fail merged cards; gaps become new cards.

**Target:** Conquer Online style, but more refined and modern. The four points:
1. **Precise jumps.** The landing point is predictable from the takeoff input, and the player lands where they aimed.
2. **Instant control on landing.** The player can turn on a dime as they land.
3. **No floaty slide.** No ground carry-over in the old run direction after landing or stopping.
4. **Air momentum, WoW-like.** Takeoff momentum carries through the air, and mid-air steering is limited, never a free reversal. Control comes back fully on the landing tick.

How 1 and 4 fit together: the trajectory is committed at takeoff, so it is predictable, and any steering in the air is bounded.

**Testable assertions every movement card must cover, as fixed-timestep fixtures on the real systems:**
- **A1 Landing stop.** With no input on landing, horizontal ground velocity is at or below ε within N_stop fixed ticks. There is no residual velocity in the takeoff direction after that.
- **A2 Landing turn.** With an opposite or perpendicular input on landing, facing and velocity reach the new direction within N_turn ticks. There is no arc that drifts the old way.
- **A3 Air control bound.** With a full opposite input held for a whole jump, the horizontal velocity change stays at or below the air_control cap. The landing point stays within D_air of the takeoff projection.
- **A4 Jump precision.** The same takeoff state and input give the same landing point within ε_land, so it is deterministic. The landing point equals the analytic projection from takeoff velocity and gravity, within ε_land.
- **A5 No slide on stop.** Releasing input on the ground brings speed to at most ε within N_stop ticks, on flat ground and on gentle slopes.
- **A6 Collision response.** Push-out never adds tangential velocity that persists after contact ends. This ties ROCK push-out to A1/A5.

**Constants.** N_stop, N_turn, the air_control cap, D_air, ε and ε_land are not invented here. Clerk pins them from the current movement constants in Lead's gap map. A card that changes a constant names it in its CUT. Each fixture must be red on main wherever the gap exists, and every card still keeps clean-boot and Q2 unchanged.

**PUSH-FEEL-1:** its CUT gains A6 and A5 if its diff touches movement or contact. The caps hold; anything bigger needs a re-ruling after Clerk's re-anchor.

Next to act: Lead - the movement gap map with the affected cards; Clerk - pin constants and A1–A6 anchors in the PUSH re-anchor.
<!-- END 6862359532 -->

## MOVEMENT-FEEL-1 amendments

Two amendments from the same call. They replace bounded air steering, the A3 air cap, `AIR_ACCEL ≤2.4` and D_air.

Comment: https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO/issues/738#issuecomment-6862475803

<!-- BEGIN 6862475803 -->
**Core · AMEND MOVEMENT-FEEL-1 (standard 6862359532) and MOVE-FEEL-1 (ruling 6862461352) @ `66234a89`.** Lead relayed two amendments from Sherif's call, at 2:11 and 2:12 PM ET. They are recorded as council-adopted under t1767u, and Sherif may amend them again. They **replace** "bounded air steering", the A3 air cap, `AIR_ACCEL ≤2.4` and D_air.

**Standard, as amended:**
1. **Air control is zero.** After takeoff, input held during the jump never changes horizontal velocity. The trajectory is set by takeoff momentum, WoW-style.
2. **Crisp takeoff aim.** On the takeoff tick, horizontal velocity takes the direction held on that tick, at the current target speed (walk or sprint). No leftover run direction is blended in. With no input held, current horizontal velocity carries.
3. **Crisp landing.** Input on the landing tick acts on that same tick. There is no recovery lockout, no landing lag and no wind-up.
4. **No floaty slide, and not stiff.** No heavy friction, and no ground carry-over in the old direction.

The skill lives in the takeoff choice and in landing control. A click or aim jump target is a possible future card only, and needs a council ruling first.

**MOVE-FEEL-1 acceptance, which replaces A1–A6** (`human_presence.rs` only, ≤+300/−25, jump lock kept):
- **M1 (a) Zero air control.** For every WASD pattern held during the jump, horizontal velocity at landing equals horizontal velocity at takeoff, within 1e-4, with no collision in between.
- **M2 (b) Analytic landing.** The landing point equals the takeoff projection (horizontal takeoff velocity × airtime, with airtime = 2·JUMP/GRAVITY on flat ground) within ε_land = 0.01 m, and repeated runs give the identical point.
- **M3 (c) Same-tick landing input.** Input on the landing tick changes velocity on that tick. `LANDING_RECOVERY_SECS` (:148) can no longer ignore input or force a brake.
- **M4 Takeoff aim.** Sprint east, then press north on the takeoff tick: the takeoff velocity points north, its east component is 0 within 1e-4, and its magnitude is the target speed. A second fixture covers no input at takeoff, where current horizontal velocity carries.
- **M5 Landing 180, N_turn.** Land moving east with west held: velocity points west on the landing tick, and reaches ≥90% of target speed within 3 ticks. Clerk may tighten this with a code-tied reason.
- **M6 No lockout or lag.** No tick after landing where held input is ignored, and no added friction beyond MOVE_DECEL.
- **M7 Stop (A5, flat ground).** From full sprint with no input, speed ≤0.05 within 7 ticks, on the ground or after landing.
- **M8 Push-out (A6).** Trunk and rock push-out removes contact-normal velocity, with no tangential residual drift after the contact ends.
- **M9 Jump lock.** Holding Space never auto-hops.

Every M fixture must be red on main where the gap exists, and clean-boot and Q2 stay unchanged.

**Order is unchanged:** Lane A runs MISSION, MOVE-FEEL-1, PUSH, EMBASSY, LINEAGE, TRAIL.

Next to act: Clerk - MOVE-FEEL-1 PRE-PATH against M1–M9.
<!-- END 6862475803 -->

## COMBAT-FAIR, and the later MOVE-FEEL-1 amend

This is the only later MOVE-FEEL amendment on #738 after 6862475803. The same comment pins COMBAT-FAIR and also opens with a CI verdict. The body is the whole comment.

Comment: https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO/issues/738#issuecomment-6862567059

<!-- BEGIN 6862567059 -->
**Core · exact-main CI 66234a89 (ROCK-COLLIDE-1 #992): GREEN, 6/6 push checks SUCCESS** (validate/court aside): Core, Core + Q2 one-frame, clean-boot, offline-build ubuntu-24.04, offline-build windows-latest, update_release_draft. CI-only verdict.

**Core · STANDARD COMBAT-FAIR (pinned, beside MOVEMENT-FEEL-1).** Lead relayed this from Sherif's 2:12 PM ET call. It is recorded as council-adopted under t1767u, and Sherif may amend it. It binds every combat-related card.

Fair means predictable, readable and skill-based. A combat card's acceptance must show, by fixture where it can:
- **CF1 Readable outcome.** Each hit or miss has a cause the player can see: range, arc, timing or blocking.
- **CF2 No hidden one-shots.** No single enemy hit removes more than a declared fraction of full health unless it was telegraphed. The card states the fraction.
- **CF3 Telegraph.** Every enemy attack has a visible wind-up of at least a stated number of ticks before damage.
- **CF4 Avoidable.** Every enemy attack can be avoided by movement or a defensive action inside its telegraph window. There is no unavoidable damage.
- **CF5 No stunlock.** Control effects have a cap or immunity window, and the player always gets an input window back.

The thresholds are pinned per card from real code. No combat work starts unless Lead's read-only map finds a gap that blocks Early Access.

**Core · MOVE-FEEL-1: Clerk's AMEND accepted, and the card is CUT @ `66234a89`** (PRE-PATH /workspace/reports/xmas-lane-bc/clerk-prepath-move-feel-66234a89.md).
- **Scope:** `client/src/human_presence.rs` only. Production ≤+60/−20, tests ≤+240. No new physics constants.
- **Anchors:** the air path :1361-1369 skips `approach_horizontal`. The landing lockout :148 / :1356-1358 is removed, while the jump lock at :1381-1384 and :3490 stays. Integration is at :1377 and :1408-1409. Push-out :1410-1417 and :1437-1446 change for M8 only.
- **M2 (amended):** the landing point equals a discrete replay of the integrator that actually runs (39 ticks), within ε_land 0.01, plus a repeated-run identity check.
- **M5 (amended):** on the landing tick, velocity equals the held target speed within 1e-4, using the same snap as M4.
- M1, M3, M4, M6 and M8 must fail on main. M7 and M9 are regression fixtures, green on main.
- Hands may cook now, but the PR opens only when the Lane A slot frees, after MISSION squashes, from tip.

**Core · N2 provisional caps accepted:**
- offline-build.yml ≤+50/−10, shrinking to ≤+45/−8 after NOTICES-1a
- JSON ≤+10/−10
- steam_partner_config.rs ≤+8/−6
- steam_abundance_mirror.rs ≤+8/−6
- docs ≤+15/−15
- tests ≤+40

Clerk re-runs PATHS once 1a lands, and N2 is CUT after that.

**Core · #994 STORY-COPY-2:** its own diff over #993 is `title_screen.rs` +72/−3. Production is within ≤+8/−4: the heading becomes "4 · Story (your own words)", and the preview drops "AI assist". Tests ≤+70 are accepted. Copy is a council pick that Sherif may veto. No votes count until it is rebased onto tip after #993 and AGE-GATE-1 land, per the `title_screen.rs` order.

Next to act: Hands - cook MOVE-FEEL-1 now and open it after MISSION; Lead's agent - #993 CI.
<!-- END 6862567059 -->

## PHYS-REAL

Standing principle pinned next to MOVEMENT-FEEL-1 and COMBAT-FAIR.

Comment: https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO/issues/738#issuecomment-6863026191

<!-- BEGIN 6863026191 -->
**CORE — PHYS-REAL standing principle (pinned next to MOVEMENT-FEEL-1 6862359532 and COMBAT-FAIR 6862567059)**

Source: Sherif's own words, Lead transcript position 7102 (t1994u, 2026-11-13 2:38 PM ET), verified by Core: *"Add things like Half-life 2 which allow players to feel things out in a way which feels physically realistic all the time, even when sci-fi"*. Adopted by the PATSAGi Councils under Sherif's typed delegation (t1767u). Sherif may veto or amend.

**Standard (PR1–PR5)**
- **PR1 Weight:** objects and bodies move as if they have believable mass and momentum.
- **PR2 Consistent forces:** pushes, impacts and gravity act the same way every time, on every surface.
- **PR3 Shape-true collision:** what blocks you matches what you see. Nothing ghosts through solid objects, and nothing invisible blocks you.
- **PR4 Consistent sci-fi rules:** sci-fi effects follow fixed rules the player can learn.
- **PR5 Feel-out:** players can probe the world and get physically plausible feedback.

**What it applies to**
- MOVE-FEEL-1 (CUT @66234a89): no scope change. M1–M9 already cover PR1/PR2 for movement. M8 (push-out clears contact-normal velocity) is the PR3 hook.
- PUSH-FEEL-1 and every world-interaction card: acceptance must name the PR items it touches.
- Known suspects from the earlier playtest: rock collision and trunk-only tree collision. Classification comes from Lead's read-only code map.

**Hold:** no new physics card is CUT until Lead's map lands with its EA-blocking list and post-EA list. MOVE-FEEL-1 and MISSION-RESPONSE-1 continue as ruled. This hold blocks only new physics scope, not the queue (t1948u).

Next to act: Lead (PHYS-REAL code map, read-only).
<!-- END 6863026191 -->

## Constant table

Quoted from `client/src/human_presence.rs` at `670b9e1fb92dceb2889b26034880f63345caf8b3`. ε, N_stop, N_turn and ε_land are **UNPINNED**. This table does not give them a value.

| Symbol or const | Value quoted from code | `client/src/human_presence.rs` |
|---|---|---|
| `STAND` | `0.90` | :139 |
| `WALK` | `3.4` | :140 |
| `SPRINT` | `5.4` | :141 |
| `GRAVITY` | `18.0` | :142 |
| `JUMP` | `5.6` | :143 |
| `JUMP_BUFFER_SECS` | `0.12` | :145 |
| `LANDING_RECOVERY_SECS` | `0.15` | :148 |
| `FOLLOW_LERP_AT_60` | `0.12` | :170 |
| `PLAYER_TRUNK_RADIUS` | `0.3` | :176 |
| `MOVE_ACCEL` | `28.0` | :178 |
| `MOVE_DECEL` | `48.0` | :180 |
| Air horizontal | unchanged while airborne; ground uses `approach_horizontal` | :1360–:1371 |
| Takeoff snap | wish on that tick, or carried horizontal velocity if no wish | :1410–:1413 |
| Landing snap | wish on the landing tick when a wish is held | :1439–:1442 |
| Push-out | drop the inward part of velocity; keep the tangent | :1456–:1470 |
| Fixed step | module states FixedUpdate @ 60 Hz | :5 |
| ε | **UNPINNED** | no value in this file |
| N_stop | **UNPINNED** | no value in this file |
| N_turn | **UNPINNED** | no value in this file |
| ε_land | **UNPINNED** | no value in this file |

Copied card text may name a fixture tolerance. That does not pin ε, N_stop, N_turn, or ε_land.
