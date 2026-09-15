# AGENTS.md — Powrush-MMO seats and slices

**Contact:** [info@Rathor.ai](mailto:info@Rathor.ai)

Independent of xAI. No certification or warranty claims. Do not email anyone from this repo’s agent seats.

Law for anyone coding, reviewing, or steering this tree. Point: [`docs/PATSAGI_MERGE_COURT.md`](docs/PATSAGI_MERGE_COURT.md). Fill the PR ballot in [`.github/pull_request_template.md`](.github/pull_request_template.md).

README **Agent landing** is canon for walked slices: U0–U8 (and related stamps listed there), do not rebuild those. Do not continue HOLD PRs.

## Inner-loop brief

Cursor / PATSAGi inner-loop agents start at [`docs/AGENT_RUN_BRIEF.md`](docs/AGENT_RUN_BRIEF.md).
The outer loop does not write code. This seat codes one named slice on a feature branch.
BRIEF-P1 locks walked Hour 2/3 fail verbs in `--lib` tests; a human fills OS / GPU / minutes.
Title Online stays grey. `playable-preview` stays `11c577e`. Floor `2163551`.
Ra-Thor does not drive WASD. Powrush-MMO-Simulator is a third repo.
Do not rebuild U0–U8 or H-2026-09-15-H2-TAB / Q / L / RESUME / H3-FAB / H3-SEAT.
Core gate: `cargo test -p shared -p rsil-identity` then `cargo test -p powrush-client --lib`.
Default play: `cargo run -p powrush-client`. Contact: [info@Rathor.ai](mailto:info@Rathor.ai).

## Seats

| Seat | Does | Does not |
|------|------|----------|
| **Steward (Sherif)** | Steers STEER and **HOLD**. One-liner, then continue or stop. | Rubber-stamp AUTO. Merge law changes without a HOLD ticket. |
| **Powrush Cursor** | Codes **this** repo on a feature branch. One slice per PR. Fills the ballot. | Push `main`. Edit Ra-Thor. Enable Title Online. Add a listen or public bind. Retag `playable-preview`. Touch Pages / Deployments / Secrets. |
| **Ra-Thor Cursor** | Lattice only (Ra-Thor repo). May **read** `data/powrush_lived_tick.json`. | Drive WASD. Drive Title Online. Drive this repo’s slice PRs. Drive keys. |
| **Grok Bots** | Design and review. | Push `main`. |

Steward steers HOLD. Steward does not rubber-stamp AUTO.

Two agents, one file, same day → **STOP**.

## Core gate

Product-green for a slice is **only**:

```bash
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
```

Do **not** treat `cargo test --workspace` as product-green. Do not unpark `server/` as the default binary. Default play stays `cargo run -p powrush-client`.

## Slice order (one slice per PR)

1. Stranger loop.
2. Hour 2 reachability without breaking Hour 1.
3. Hour 3 only after an Hour 2 playtest note.
4. Local persist.
5. Net foundations: Offline default / LoopbackDev / Online grey; **no public listen**.
6. Real Online only on a steward **HOLD** ticket.

Feature branch. Fill the ballot. Label `patsgi-auto` only when the court verdict is AUTO.

## Do not

- Continue HOLD PRs.
- Freelance a new product.
- Enable Title Online. Add a listen or public bind.
- Widen the deny-list or auto-merge HOLD paths.
- Change `Cargo.toml` workspace members / default bins.
- Retag `playable-preview`.
- Touch GitHub Pages, Deployments, or Secrets.
