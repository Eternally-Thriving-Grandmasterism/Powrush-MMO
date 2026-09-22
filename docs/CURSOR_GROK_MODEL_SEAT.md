# CURSOR GROK MODEL SEAT — 2026-09-22

**Steward:** remaining Cursor team/account credits for non-Grok models are spent. **5x Grok usage** is what is left for ~2–3 weeks. Contact info@Rathor.ai.

Cursor cloud agents (the ones that opened #482 / #483) stay allowed. They must pick a **Grok-family** model. They must not pick Codex, GPT, Claude/Fable, or Auto-that-routes-off-Grok.

## Pick (in order)

| Job | Model in Cursor | Effort | Why |
|---|---|---|---|
| Hands cook (Q2 workflow, L2-REPLAY, lib tests) | **Grok 4.7** | **high** | Live as of 2026-09-21. Longer self-check. Same price band as 4.6. |
| Fallback if 4.7 missing in the picker | **Grok 4.6** | **high** (named default) | Long agent, instruction-follow. Model id `grok-4.6`. |
| Clerk vote / one COURT line | Grok 4.7 or 4.6 | **medium** | Do not spend xhigh on a vote. |
| Merge-conflict on `persona.rs` / `hour_sacred.rs` only | Grok 4.7 or 4.6 | **xhigh** once | Then drop back to high. |

Do **not** enable Grok Fast / 2x-price variant unless the steward writes `fast yes`. Fast burns the 5x pool twice as hard.

Composer 2.5 sits in Cursor’s first-party pool with Grok. Still **not** default. Clerk may use it only for a one-line vote if Grok 4.7/4.6 are missing from the picker. Hands never cooks on Composer.

## Forbidden (burns the empty pool)

- Codex / Codex 2.5 / any OpenAI Codex harness
- GPT-5.x / Sol / o-series
- Claude / Fable / Anthropic
- Cursor Auto if it can route off Grok
- A second parallel cloud agent on the same CARD

Codex is **not** Grok family. Do not “or Codex” this seat.

## How to launch (Hands Bot 1)

1. Fetch `origin/main`.
2. Open Cursor Agent / cloud agent on **this repo only**.
3. Model = Grok 4.7 High (else Grok 4.6 High).
4. Paste the named CARD (`docs/Q2_CAPTURE_IF_PATH_CARD.md` now) + `AGENTS.md` + this file.
5. One PR then HOLD. Do not start a second agent.

Core / Clerk do not need a cloud agent for a COURT line. Type the vote in group chat.

## Still true

Title Online grey. No OFFER NEXT. No invent PATHS. #459 closed. Q2 PATHS already named.
