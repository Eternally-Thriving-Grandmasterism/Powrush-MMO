# CARD Q2 CAPTURE-IF-PATH

**Named 2026-09-22 by steward.** #483 Q1 COMPLETE (merged). Q2 HOLD lifts.

id: Q2
phase: P0
bin: NEVER for Online / ffmpeg movies / golden gameplay stills / Imagine as proof
next: HOLD (then L2-REPLAY or Q1b only if Core names it)

## PATHS

Exist on tip:

- `.cursor/run-client-headless.sh`

One new file this CARD names:

- `.github/workflows/agent-headless-qa.yml`

Optional cite (do not require to merge):

- `docs/AGENT_QA_TAPE.md` (create if missing; pass/fail only)

## Finish line

CI job on pull_request + push to main:

1. Install Xvfb + lavapipe/vulkan ICD + scrot (or imagemagick import).
2. Run `.cursor/run-client-headless.sh` with a timeout (e.g. 45s).
3. PASS if process started and one frame file exists under `/tmp/powrush-q2.png` (or workflow artifact).
4. FAIL if the window never appears / process dies before the frame.
5. Do **not** compare pixels to a golden screenshot of gameplay.
6. Do **not** claim lavapipe = Steam Deck / real GPU feel.
7. Title Online stays grey. `POWRUSH_NET=off`.

Core gate still required on the same PR:
`cargo test -p shared -p rsil-identity` and `cargo test -p powrush-client --lib`.

## Refuse

ffmpeg trailer · Imagine stills as QA · invent minutes · .glb · listen · 0.0.0.0 · retag `playable-preview` · OFFER NEXT · reopen #459
