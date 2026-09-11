# STUDIO_ARCHITECTURE_ORDER.md — layer cake + perfect order of operations

**Contact:** info@Rathor.ai  
**Independent of xAI.** No certification / AGSi warranty / legal-product claims.

**Docs-only.** GREEN-DOCS. No Cargo bump. No sockets / listen / public bind. No `client/**` · `shared/**` · `server/**` · `data/**` · scripts · art / assets · `SLICE_LOG`. **Title Online stays grey.** Tag `11c577e` does not move.

**Offer stamp:** `O-2026-09-10-4` — studio architecture order. Seat: Grok Bot 3 — Canon / design / review. Does not push `main`. Does not drive the play floor.

> ### CONFIRM BEFORE BUILD
> **This file is ops law for order + layers, not a work order.** Hands implement only after Core writes `CONFIRMED §3 rung A` (or a matching SLICE CARD). Until then every rung is **unbuilt**. `GDD_IMMERSION_REVISION` §3 owns rung *detail*; this file owns **order + layers**. Cross-link, do not replace, the teachers below.

Cross-link (not restated, not a second bible): `docs/SIM_AND_HAND_CANON.md` · `docs/OFFLINE_SKU.md` · `docs/FUN_WITHOUT_WOW.md` · `docs/GDD_IMMERSION_REVISION.md` · `docs/GDD_ADAPTATION.md` · `docs/PLACE_CLARITY_HOUR.md` · `docs/PLACE_CLARITY_WALK.md` · `docs/FIRST_LAUNCH_UI_SCALE.md` · `docs/NET_OFFLINE_CONTRACT.md` · `docs/ARCHITECTURE.md` (older overview — **this file is ops law**).

Hands beat lore. Council freeze for Powrush **Offline 1.0**, as if SC2 / WoW-calibre studios mapped their gifts onto this SKU.

---

## 1. Layer cake (SC2 studio gift)

Four layers. Do not collapse them. Do not let Presentation or Sky own game truth.

| Layer | Job | Truth |
|---|---|---|
| **1. F / sim** | Deterministic tick in `shared`: stocks, wells, week, verbs. \(S_{n+1} = F(S_n, C_n)\). | Game truth lives here. |
| **2. Hand** | **FixedUpdate @ 60 Hz**, one Use, buffer ≤120 ms, **stop-on-release**. | Finger ack this tick (`SIM_AND_HAND_CANON`). Ice-skate / eaten E / Title hitch stay local hand bugs. |
| **3. Presentation** | Camera, audio, UI, mesh **reads**. | Never writes \(F\). Never locksteps pose. Never invents **Wall-clock as game truth**. |
| **4. Sky** | Online / digests **later**. | Title Online **grey** until steward `online yes`. Wire later = commands / digests, never poses (`NET_OFFLINE_CONTRACT`). |

Presentation and Sky must **not** lockstep pose or invent Wall-clock as game truth.

---

## 2. WoW Vanilla / TBC gift (keep / transform / refuse)

Short table. Detail lives in `FUN_WITHOUT_WOW` and `GDD_IMMERSION_REVISION` §2.2.

| Verdict | Gift | Powrush Offline 1.0 |
|---|---|---|
| **Keep** | Zone / hub identity | One material per Place; room nameable with **H** off. |
| **Keep** | Profession-like craft loops **as feel** | Tend / Mend + fabricator rhyme; recipes teach craft / mend / civic proof — no combat stats. |
| **Transform** | Zone world + hub + craft economy | **Four Places** + **Settled + book** door + **RBE credit** logistics (tons + restored; Reserve = repair-rights). |
| **Refuse** | Auction gold | Credit, not gold. Market stays HOLD. |
| **Refuse** | Race lobby at character select | Peoples are post-House dress. Never Title picker. |
| **Refuse** | Raid lockout / reset week | Week is a **bill**, not a calendar that owns you. |
| **Refuse** | Ability bars / F-row | One Use verb. One HUD. |
| **Refuse** | Title Online | Grey until `online yes` / NET-OK. |

---

## 3. Perfect order of operations

This file owns **order**. Rung lore and candidate paths: `GDD_IMMERSION_REVISION` §3. Each rung waits on the previous being confirmed and, where Hands are involved, landed. No skip.

| Step | Name | Ops sentence |
|---|---|---|
| **0** | **Floor green** | Spent fail-beats stay spent. Do not rebuild walked U0–U8 (and related README stamps). Floor `2163551`. Tag `11c577e` does not move. |
| **1** | **Rung A** | Climate slab readable + UI scale wire. **FIRST build.** Walk fail = blank slab (`PLACE_CLARITY_WALK`). |
| **2** | **Rung B** | Person body + action read (CO gift). **YELLOW.** Named asset budget. No art-pack freestyle. |
| **3** | **Rung C** | Settled + book Places door. Four rooms on disk, not a teleport menu. |
| **4** | **Rung D** | TTD guild logistics: **`shared` sim first**, client read second, **one leg / PR**. Credit, not gold. |
| **5** | **Rung E** | Oblivion-grade schedules **after B + D**. Hour still finishes if every NPC is removed. |
| **6** | **Rung F** | Place dress / hub identity. **Still four Places.** No fifth. |
| **7** | **Sky / Net** | Only on steward **`online yes`** / NET-OK. No public listen. Loopback does not light Title. |

Do not skip the ladder. A bot that builds a later rung without Core confirm on the earlier ones is out of seat.

---

## 4. Begin-build gate

Implement starts **only** when Core writes **`CONFIRMED §3 rung A`** or a matching **SLICE CARD**.

**First Hands slice** = climate slab blank on the listed path (`client/src/climate_visible.rs` is a **candidate** — this file does not edit it). Also named in `GDD_IMMERSION_REVISION` §3 rung A: `client/src/local_settings.rs` · `client/src/settings.rs`.

That first slice is **YELLOW**: no SAFE AUTO-MERGE, no art pack, no mesh dump, no `SLICE_LOG` freelance. Ballot filled. Core gate later (when Hands exist): `cargo test -p shared -p rsil-identity` and `cargo test -p powrush-client --lib`. This offer does **not** run Cargo.

---

## 5. Hard refuse

No rung, no slice, no confirm carries these:

- **Battle Power** / power scalar
- Cash-shop power / mall
- Auction gold
- Race lobby
- Sockets / listen / public bind / Title Online lit from docs
- Brood Spire on Sanctuary
- XP-from-kills
- Second HUD / F-row
- **Skip ladder** (no B before A, no E before B+D, no Sky before `online yes`)
- Universe dump in one PR
- Fifth Place · Market as shipped · `Cargo.toml` edits · `playable-preview` retag · Ra-Thor driving WASD

---

## Related

`SIM_AND_HAND_CANON` · `OFFLINE_SKU` · `FUN_WITHOUT_WOW` · `GDD_IMMERSION_REVISION` · `GDD_ADAPTATION` · `PLACE_CLARITY_HOUR` · `PLACE_CLARITY_WALK` · `FIRST_LAUNCH_UI_SCALE` · `NET_OFFLINE_CONTRACT` · `ARCHITECTURE.md` · `PATSAGI_MERGE_COURT` · `AGENTS.md`.
