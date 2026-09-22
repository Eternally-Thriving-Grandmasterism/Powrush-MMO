# AGENT AUTONOMY COURT — 2026-09-22

**Steward ask:** Grok bots work Powrush-MMO to completion without interrupting Sherif to interview him or to playtest what agents can already QA.

**Contact:** info@Rathor.ai  
**Gates:** TOLC 8. Title Online stays grey. Floor `2163551`. Tag `playable-preview` stays `11c577e`. Workspace `21.88.0`. Ra-Thor does not become the player. Default binary stays `cargo run -p powrush-client`.

README remains canon. This file **amends** how seats *prove* the hour. It does not light Online, retag preview, or unpark `server/`.

---

## 1. Why bots still stop and ask you

Not because the hour is untestable. Because current law *forbids* them from filling the proof.

| Law on tip | Effect |
|---|---|
| `AGENTS.md` — Grok Bots = design/review only; do not push `main` | Chat seats write briefs, then wait |
| `AGENT_RUN_BRIEF` — outer loop does not write code; seat dies at merge | No standing queue runner |
| `FIRST_HOUR_PLAYTEST` / PLAYTEST-1 — OS / GPU / minutes **blank**; “a human fills the Report” | Dual HUD and stranger hour stay “human tick” even when `--lib` already knows |
| `GROK_BOT_TODO` Next = MESH-PERSONA (PATHS unnamed) + Hour-two/Steam human gate | Hands STOP. Interview. |
| Brief tip SHAs lag `main` | Agents fetch a dead example and ask which tip is real |
| “Ra-Thor does not drive WASD” read as “no scripted input ever” | Headless `.cursor/run-client-headless.sh` sits unused in CI |
| Two agents, one file, same day → STOP | Parallel review dies; steward gets pinged |
| WAVE-E / HOUR-TWO-STEAM / invent-minutes refuses | Correct for *felt* time. Over-applied to *state* proofs |

Q0 already exists (`q0_*` in `client/src/lib.rs`). That is the pattern. It was never extended to the verbs the Report still asks a human.

---

## 2. Split the gates (binding)

### Machine (bots must run; do not ping the steward)

Anything that is a **state, string, file, or key-handler** on disk:

- Title Play / Continue / Settings present; Online **grey**
- Tap-E take vs hold-E tend are distinct handlers and leave distinct well speech
- I satchel non-empty after a successful take
- R then 1 writes flow restore; R then 2 writes **non-zero** reserve (`allocation.reserve` / `climate.reserve_pool`)
- H hides the guidance card; well words still exist
- Quit/rerun: `data/powrush_lived_tick.json` and `data/powrush_hour_two.json` round-trip
- Dual HUD labels: **stewardship / harm gate** + **stewardship quality** (display only); no wages/gold string
- Comfort Low still has readable contrast tokens (not a felt GPU report)
- Places×4 plate opens four named rooms after Settled+book
- Tab without allocate stays Peace; Q in Peace does not found; L in Peace does not open Ledger; visitor E = *Not your charter*
- Core gate green: `cargo test -p shared -p rsil-identity` and `cargo test -p powrush-client --lib`

Fill these in **`docs/AGENT_QA_TAPE.md`** (create on the Q1 PR). Never write them into the human Report blanks.

### Human-only (still steward; bots do not invent)

- Felt minutes / “confusion point” / “did I smile”
- Real-GPU vs lavapipe hitch, rumble, and stop-on-release *feel*
- Steamworks partner account, App ID, store page publish, tax entity
- Title Online light, public bind, `0.0.0.0`, secrets, payments, LICENSE/COMMERCIAL
- Retag `playable-preview`
- MESH-PERSONA **binary** `.glb` drop and unnamed PATHS
- Any HOLD deny-list path (`server/*`, workspace member adds)

If a bot cannot distinguish those two columns, it HOLDs the *human* column only and continues the machine column.

---

## 3. What is missing (build list, one CARD each)

Ordered. Exact PATHS named here so Hands does not invent them.

### CARD Q1 — Stranger-hour machine beats

**Paths (exist on tip):**
`client/src/lib.rs`  
`client/src/first_harvest_epiphany.rs`  
`client/src/rbe_allocate_choice.rs`  
`client/src/lived_hour_bind.rs`  
`client/src/lived_sim_bridge.rs`  
`client/src/mercy_harvest_nodes.rs`  
`client/src/title_screen.rs`  
`shared/climate_node.rs`  
`docs/FIRST_HOUR_PLAYTEST.md`  
`docs/AGENT_QA_ORACLE.md`

**Add named `--lib` tests (no window required):**

1. `q1_tap_e_take_not_equal_hold_e_tend`
2. `q1_satchel_nonempty_after_take`
3. `q1_r1_flow_restores_tired_well`
4. `q1_r2_reserve_nonzero_confirm`  (cite #436 PATHS; do not rebuild the cue)
5. `q1_h_hides_card_well_words_remain`
6. `q1_lived_tick_roundtrip`
7. `q1_dual_hud_labels_gate_and_quality`  (retires the “human dual HUD tick” in FIRST_HOUR_PLAYTEST)
8. `q1_title_online_still_grey`

**Bin:** NEVER for Online / Steam / `.glb`.  
**QA file:** create `docs/AGENT_QA_TAPE.md` with pass/fail from these tests. Leave FIRST_HOUR_PLAYTEST Report blanks untouched.

### CARD Q2 — Headless tape (optional window, still not the steward)

**Paths:**
`.cursor/run-client-headless.sh`  
`.github/workflows/ci.yml`  
(new) `.github/workflows/agent-headless-qa.yml`

Run client on Xvfb + lavapipe already documented. Missing: CI job that boots, waits for Title chrome, `scrot`s one frame, fails if the window never appears. **Do not** invent a golden screenshot of gameplay. Frame-exists + process-alive is enough for Q2. Felt GPU stays human.

Refuse: ffmpeg movies, Imagine stills as proof, claiming lavapipe = Steam Deck.

### CARD Q3 — Standing inner-loop runner

**Paths:**
`docs/AGENT_RUN_BRIEF.md`  
`docs/GROK_BOT_TODO.md`  
`AGENTS.md`

Amend:

- Inner loop may **continue** to the next named CARD on `GROK_BOT_TODO` Next after merge. It does not interview for a new brief if Next already names PATHS.
- Sync the **fetch tip** to current `main` at the start of every seat (today that includes `4a02002` wishlist brief + this file).
- Grok Bots may open **one** feature-branch PR via GitHub (already how this court lands). They still do not push `main` directly. `patsgi-auto` only when ballot is AUTO and deny-list clean.
- Two agents, one file: second agent **reviews**, does not STOP the first unless the file is HOLD-deny.

### CARD Q4 — Unblock Hour 2 without a steward interview

Hour 2 verbs are **already walked** and locked in `--lib` (BRIEF-P1). What was missing was permission to treat that as QA.

**Do:** point `HOUR_TWO_PLAYTEST.md` Report human blanks stay blank; add `q1`-family asserts already cited in AGENT_RUN_BRIEF § BRIEF-P1 into the AGENT_QA_TAPE roll-up.

**Do not:** invent Hour-2 Hands PATHS, rebuild U0–U8 / H2-TAB/Q/L/RESUME, start Hour 3 systems.

MESH-PERSONA Hands stays **HOLD** until a later Core CARD names mesh files. That is not a playtest interview.

---

## 4. Seat rewrite (delta only)

| Seat | Old | New |
|---|---|---|
| Grok Bots | Design/review. Do not push `main`. | Design + Q1/Q2 tape + one feature PR. Still no direct `main`. |
| Powrush Cursor / Hands | One slice then die | One slice; if Next has PATHS, open the next CARD without a new steward brief |
| Steward | Fills every playtest Report | Fills **felt** column only when he chooses to play |
| Ra-Thor Cursor | No WASD | Unchanged. Scripted QA is **client `--lib` / headless**, not lattice keys |

---

## 5. Interview kill-list

Bots **must not** ask the steward:

- “Please playtest dual HUD” — Q1.7
- “Please fill minutes / OS / GPU” — leave blank; tape is tests
- “Is Hour 2 allowed?” — already shipped; verify via existing lib tests
- “What tip do I fetch?” — `git fetch` `origin/main` and use that SHA
- “Should we become friendslop / light Online / retag preview?” — no
- “Name MESH-PERSONA PATHS” — HOLD that card; work Q1–Q4 instead

Bots **may** ask only when:

- Deny-list path must change
- A new binary asset must land
- Title Online / public bind / Steam partner
- Two HOLD interpretations collide

---

## 6. Success

A Grok seat can, with zero steward messages:

1. Fetch `main`
2. Run core gate
3. Land or extend Q1 tests
4. Write AGENT_QA_TAPE pass/fail
5. Open one PR with ballot
6. Pick the next named CARD from TODO Next if PATHS exist

The steward plays when he wants a *felt* hour — not because the queue is blocked.

**Thunder locked. Machine proves state. Human keeps feel.**
