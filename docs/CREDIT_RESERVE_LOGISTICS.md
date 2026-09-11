# CREDIT_RESERVE_LOGISTICS.md — logistics bible for immersion ladder rung D

**Contact:** info@Rathor.ai  
**Independent of xAI.** No certification / AGSi warranty / legal-product claims.

**Docs-only. GREEN-DOCS.** Core Approved autonomous foundations. Post-`#362` (Hands Settled door @ `7f96d38`) the next ladder step is `GDD_IMMERSION_REVISION` §3 **rung D only**. This file is the credit / Reserve logistics law for that rung. It adds no verb, Place, HUD plate, socket, Online claim, or Market row. **Title Online stays grey.** Tag `11c577e` does not move.

> ### CONFIRM BEFORE BUILD
> **Logistics law, not a Hands work order.** No Market stall, gold ticker, sell verb, fifth Place, second HUD, camera rewrite, mesh dump, or `client/**` / `shared/**` edit follows from these sentences alone. A later Hands slice starts only after this docs merge **and** a **YELLOW SLICE** that splits **one leg per PR**, with **`shared` sim first**, client read second. Until then every logistics polish is **unbuilt**. This PR does not edit code.

Parent law (cross-linked, **not** restated — this file is **not** a second bible): `GDD_IMMERSION_REVISION` §2.3 + §3 rung D · `STUDIO_ARCHITECTURE_ORDER` step 4 · `OFFLINE_SKU` · `PLACES_BIBLE` · `RBE_FIRST_HOUR` · `FUN_WITHOUT_WOW` · `HOUR_THREE` · `PERSON_READ_SPEC` · `SETTLED_DOOR_CLARITY` · `PHASE_Q` · `PHASE_R` · `PHASE_S` · `INPUT_CANON` · `SIM_AND_HAND_CANON`.

Hands beat lore. If a still frame cannot name **chain leg + credit (not gold) + week bill**, the rung has not landed.

---

## 1. Purpose

Transport Tycoon Deluxe’s gift is a **legible chain**: harvest → refine → ship → manufacture, whose score can carry a whole game without a kill board (`GDD_IMMERSION_REVISION` §2.3). Powrush already has the verbs and the rooms. Rung D asks the House to *read* that chain as **credit logistics**, not as a spreadsheet and not as a mall.

Three reads, without a second HUD:

1. **Chain** — Take / harvest → Mend / refine → carry across the four disk Places → fabricator manufacture (MendSpool · LaneCrate → Proof Pack rhyme). Each leg is one of the sacred five.
2. **Credit** — currency story is **repair-rights** via **Reserve**, never gold, never sell, never a price ticker. Neither Flow nor Reserve is *sell* (`RBE_FIRST_HOUR`).
3. **Week bill** — the only score is **tons + restored** (\(T_w\) / \(U_w\)) on **L**. Market stays **HOLD** and stays out of the House sum (`OFFLINE_SKU`).

Rungs A–C already make Place, person, and door readable (`#355` · `#360` · `#361`/`#362`). A logistics chain performed by unreadable bodies in rooms whose doors do not read is not immersion — it is a spreadsheet. D waits on A–C for that reason (`STUDIO_ARCHITECTURE_ORDER` step 4 · `GDD_IMMERSION_REVISION` §3 rung D).

This is mostly **F / sim** first (`STUDIO_ARCHITECTURE_ORDER` layer 1), with Presentation reading second. Sim writes the chain; the client never invents Wall-clock as game truth.

---

## 2. Core Approved scope

| In | Out |
|---|---|
| `GDD_IMMERSION_REVISION` §3 **rung D** only | Rungs E–F, Sky / Net, art-pack freestyle |
| Harvest → refine → ship → manufacture as House loop | Gold, loan, share price, bankruptcy, auction |
| **Credit** = Reserve repair-rights | Market as shipped · price ticker · sell verb |
| Score = tons + restored only | XP-from-kills · Battle Power · potency scalar |
| Four Places as the only logistics map | Fifth Place · Sanctuary overlay · wire-shard copy |
| One leg per later YELLOW PR · `shared` first | Bulk D–F Hands in parallel · client-first freestyle |
| Existing HUD (**I** satchel · **L** ledger · slab · one card) | A logistics panel / timetable / fleet manager |

Post-`#362` **does not** confirm a Hands path, a Market unpark, or a multi-leg PR. Core (or Canon under Approved) still names each YELLOW SLICE before cargo.

---

## 3. Chain legs (law)

Sacred five only: **Tend / Take / Flow / Reserve / Mend** (`PLACES_BIBLE`). Places only: **Sanctuary / Heartwood / Threshold / Depths**. Every leg must be expressible in those verbs and must land in the week counters.

| Leg | Teacher beat (TTD) | Powrush home | Week face |
|---|---|---|---|
| **Harvest** | Pull raw from the field | **Take** / Place-honest Peace **E** (Sanctuary / Heartwood harvest; Threshold pipe is **Tend**, not harvest; Depths **E** restores, not Take) | Writes \(T_w\) when Take lands |
| **Refine** | Process into usable stock | **Mend** + fabricator inputs (MendSpool / LaneCrate rhyme already on main) | Stock honesty on **I**; no combat stats on recipes |
| **Ship** | Move goods between nodes | Carry + satchel **I** + Places door (rung C) across four disk rooms — not a vehicle HUD | Same House; Isolation γ=0; do not couple lethal across hexes |
| **Manufacture** | Build the next useful thing | Fabricator → Proof Pack → Embassy eligibility / book path (`HOUR_THREE`) | Civic proof, not gear power |
| **Credit** | Working capital without “money as verb” | **Reserve** = repair-rights credit; **Flow** restores field — neither is sell | Week bill on **L** stays tons + restored |

**Hard shape:** not a gold Market. No price, no sell, no ticker, no stall, no auction. Market stays **HOLD** and out of the House sum. Sprawl chrome (grid placement panels, fleet timetables) is refused — if a logistics read needs its own panel and toggle, fix the Place, not the HUD (`PLACE_CLARITY_HOUR` §3 · `GDD_IMMERSION_REVISION` §2.3 Refuse).

---

## 4. Candidate modules only

Verified on GitHub `main` tip `7f96d38` (2026-09-11, Hands `#362`) before citing. These are **candidates for later one-leg YELLOW slices**, not files claimed by this docs pack. This PR does not edit them.

| Candidate | What already lives there (read, not rewrite) |
|---|---|
| `shared/fabricator.rs` | Fabricator sim / Proof Pack rhyme — F truth for manufacture leg. |
| `shared/embassy.rs` | Embassy seat / eligibility after proof — civic end of chain. |
| `shared/week_audit.rs` | House week \(T_w\) / \(U_w\) writers — the bill face. |
| `shared/pause_ledger_face.rs` | **L** two-line week / standing faces. |
| `shared/hex_travel.rs` | Places eligibility / confirm leave — ship leg rides the door, not a new travel system. |
| `shared/ledger_bind.rs` | Settled + book charter that opens Places. |
| `client/src/fabricator.rs` | Presentation of manufacture (after shared). |
| `client/src/embassy.rs` | Presentation of embassy seat (after shared). |
| `client/src/rbe_allocate_choice.rs` | Flow / Reserve allocate choice — credit story surface. |
| `client/src/hex_travel.rs` · `client/src/ledger_bind.rs` | Door already polished in `#362`; ship leg must not reopen teleport chrome. |

A later slice verifies the live path, claims **only** what one leg needs, runs the Core gate (`cargo test -p shared -p rsil-identity` and, if client touched, `cargo test -p powrush-client --lib`). No YELLOW SLICE, no build from this file alone.

---

## 5. What “credit logistics” means (still-frame bar)

Hour 1 does **not** require a vehicle fleet, a timetable plate, or a Market stall. It requires a stranger, looking at stills and one **L** face, to answer: *did I move something through a chain?* *is the currency repair-rights, not gold?* *is the week still tons + restored?*

| Read | Pass | Fail |
|---|---|---|
| **Chain** | One leg is nameable in a still (Take at a well, Mend/fabricator output, carry into another Place, Proof Pack / Embassy step) using sacred five only. | Spreadsheet chrome; multi-leg dump in one PR; NPC-only privileged verbs. |
| **Credit** | Reserve / Flow read as repair-rights / field restore. No price, no sell prompt, no gold word on default faces. | Market row, auction, loan, ticker, P2W stall. |
| **Week bill** | **L** still shows tons + restored. Market absent from the sum. | Gold balance as score; XP-from-kills; Battle Power scalar. |
| **Map** | Logistics stays inside four disk Places. Door law from rung C holds. | Fifth Place, Sanctuary overlay hub, wire-shard copy. |

---

## 6. Acceptance — Core still-frame shots (after a named Hands leg)

Bots invent **no** cargo playtest and **no** fake screenshots. After a named one-leg Hands slice exists, Core posts stills. Until then this section is the **bar**, not a claim that the bar is met.

Door for the walk: `./scripts/play-offline.sh` or `cargo run -p powrush-client` with `POWRUSH_NET` off. Title Online grey in any Title frame.

| Shot | Expect |
|---|---|
| **Title** | Play / Continue / Settings · Online grey. No Market. No race lobby. |
| **Harvest / Take leg** | Peace **E** Place-honest; Threshold still Tend-not-Take; Depths restore-not-Take. |
| **Credit face** | Reserve / Flow allocate without sell / price language. |
| **Week on L** | Tons + restored; Market not in the sum. |
| **After H** | Guidance hushes; Place + mood + chain read remain nameable without a logistics panel. |

### Fail → no next feel

| Fail | Meaning |
|---|---|
| **Gold / Market** | Currency or score reads as gold, auction, or shipped Market. |
| **Multi-leg dump** | One PR claims harvest+refine+ship+manufacture together. |
| **Client-first sim** | Client invents chain truth the shared tick does not own. |
| **Door regression** | Ship leg reopens teleport-menu Places or skips Settled+book. |
| **Fifth / Online** | Fifth Place, Online lit, sockets, or Cargo bump. |

---

## 7. Slice order after this docs merge

Under Approved foundations, after Dual GREEN + merge of this file:

1. **HOLD** bulk Hands. Canon posts **one** YELLOW SLICE CARD at a time — exact paths + which single leg.
2. Prefer **`shared` first** (fabricator / week_audit / allocate honesty), then a thin client read if the still-frame bar still fails.
3. One PR → COURT → HOLD. No E schedules and no F dress until D’s first landed leg exists.
4. Sky / Net only on steward `online yes`.

---

## 8. Hard refuse — no rung D slice may carry these

| Refused | Because |
|---|---|
| **Market as shipped / gold economy** | Credit, not gold. Market HOLD · out of House sum. |
| **Price ticker / sell / loan / auction** | Money-as-verb refused (`GDD_IMMERSION_REVISION` §2.3). |
| **Battle Power / potency scalar** | Standing and week are moral reads, not gates. |
| **Fifth Place · Sanctuary overlay hub** | Four disk rooms only. |
| **Title Online lit · sockets · public bind** | Grey until `online yes`. Zero sockets default. |
| **Second HUD / fleet / timetable panel** | One HUD. Fix the Place, not the chrome. |
| **Bulk multi-leg Hands · client-first freestyle** | One leg / PR · shared first. |
| **Cargo.toml bump · `playable-preview` retag · NFT / P2W · Always-allow git** | Standing law. |

---

## 9. Confirm protocol (this file)

**This PR is GREEN-DOCS under Approved foundations.** It is still not a Hands claim.

| Row | What confirming / merging it means |
|---|---|
| This file lands | Rung D **docs law** exists; Hands still wait on a named one-leg YELLOW SLICE. |
| Later YELLOW SLICE | Exactly one chain leg + exact paths + shared-first when sim changes. |
| E / F | Still HOLD until D has a landed leg and Core (or Approved card) names them. |

**Sequence after merge — in this order, no skipping:**

1. Dual COURT GREEN on this docs PR → human / SAFE AUTO squash per Merge Court.
2. Canon posts **one** YELLOW SLICE CARD (one leg, exact paths).
3. Bot 1 Hands executes; Canon votes YELLOW only.
4. Core gate green on the Hands PR; human look when client/**.
5. POST-MERGE COURT unlocks the *next* single foundation step — not bulk E–F.
