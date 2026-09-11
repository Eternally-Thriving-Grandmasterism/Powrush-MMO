# NPC_SCHEDULE_SPEC.md — schedule bible for immersion ladder rung E

**Contact:** info@Rathor.ai  
**Independent of xAI.** No certification / AGSi warranty / legal-product claims.

**Docs-only. GREEN-DOCS.** Core Approved autonomous foundations. Hands POST-MERGE after `#367` D4 allocate @ `459ffcf0`. The next ladder step is `GDD_IMMERSION_REVISION` §3 **rung E only**. This file is the NPC schedule / work-loop law for that rung. It adds no verb, Place, HUD plate, sim write, socket, Online claim, mesh, or person roster. **Title Online stays grey.** Tag `11c577e` does not move.

> ### CONFIRM BEFORE BUILD
> **Schedule law, not a Hands work order.** No mesh, anim pack, crime meter, ownership flag, combat pose, second HUD, setting, or `client/**` edit follows from these sentences. A later Hands slice starts only after this docs merge **and** Core / Canon names a **YELLOW SLICE** with **exact paths**. Until then every schedule polish is **unbuilt**. This PR does not edit the client. This PR invents no mesh and claims no client paths as edited.

Parent law (cross-linked, **not** restated — this file is **not** a second bible): `GDD_IMMERSION_REVISION` §2.4 Oblivion map + §3 rung E · `STUDIO_ARCHITECTURE_ORDER` step 5 · `PERSON_READ_SPEC` · `SETTLED_DOOR_CLARITY` · `CREDIT_RESERVE_LOGISTICS` · `PLACE_CLARITY_HOUR` · `PLACE_CLARITY_WALK` · `PLACES_BIBLE` · `FUN_WITHOUT_WOW` · `PLAYER_FEEL_METRICS` · `OFFLINE_SKU` · `INPUT_CANON` · `SIM_AND_HAND_CANON`.

Hands beat lore. If a still frame cannot name **person at a known post + same sacred five + hour still finishes without them**, the rung has not landed.

---

## 1. Purpose

Oblivion’s gift is a **clock you can catch people on**: sleep, wake, walk to work, greet, go home (`GDD_IMMERSION_REVISION` §2.4). Powrush already has a day, well moods, four posts, and the sacred five. Rung E asks persons to *use that day*, not to become a second product.

Three reads, without a second HUD:

1. **Schedules on the existing day** — people can be found at the same post twice. The clock is the day already turning in `living_day` (240 s loop, night flag, Depths stay night). Schedules ride that day. They do not invent Wall-clock as game truth (`STUDIO_ARCHITECTURE_ORDER` layer 3).
2. **Work loops** — the person is *doing the job of the Place* with the **same sacred five** at the **same posts**: **Tend / Take / Flow / Reserve / Mend** at Heartwood Wards, Threshold pipe, Depths landing, Sanctuary well. Same verb, same tick, no privileged NPC-only action.
3. **Greet by Place / mood** — short spoken lines on the **one HUD**, hideable by **H**, tied to Place + well mood (Idle / Glowing / Tended / Resting / Stressed). Existing voice-direction docs are dress; this file orders no new audio assets.

**Hour-finish bar (non-negotiable):** the hour still finishes if every person is removed. Persons never write the house book and never gate a Place. A schedule that can strand the hour does not ship.

Rungs A–D already make Place, person, door, and credit chain readable (`#355` · `#360` · `#361`/`#362` · `#363`–`#367`). Schedules wait on **B + D** for that reason (`STUDIO_ARCHITECTURE_ORDER` step 5). Unreadable bodies on a gold Market are not this rung.

This is **Presentation** of presence on a day that already exists. Presentation never writes \(F\), never locksteps pose, never lets a person become sim truth the shared tick does not own.

---

## 2. Core Approved scope

| In | Out |
|---|---|
| `GDD_IMMERSION_REVISION` §3 **rung E** only | Rung F, Sky / Net, art-pack freestyle |
| Schedules + work loops on the **existing** day | A new calendar, raid lockout, Wall-clock as \(F\) |
| Same sacred five at same posts | NPC-only verbs, person combat, theft loops |
| Greet by Place + well mood on the one HUD | Dialogue tree, crime meter, fame/infamy bar |
| Hour finishes with **zero** persons | Persons writing the house book · Place gates |
| After B + D (person-read + credit logistics) | Skip ladder · E before B or D |

Post-`#367` **does not** confirm a Hands path, a mesh order, an NPC roster, or a Quellorian cargo dump. Core / Canon still names the YELLOW SLICE with exact paths before cargo.

---

## 3. Candidate modules only

Verified on GitHub `main` tip `459ffcf0` (2026-09-11, Hands `#367` D4 allocate) before citing. These are **candidates for a later confirmed Hands slice**, not files claimed by this docs pack. This PR does not edit them.

| Candidate | What already lives there (read, not rewrite) |
|---|---|
| `client/src/living_day.rs` | `LivingDay` phase + night. ~240 s loop. Ambient brightness. Depths stay night. The day schedules must ride — not a second clock, not Wall-clock as \(F\). |
| `client/src/living_ecology.rs` | `BiomeFeel`, `PersistentWeb`, climate dress, deer / ecology props. Place mood the greet may rhyme with. Ecology props are **not** persons, **not** owned inventory, **not** a fence. |
| `client/src/human_presence.rs` | Local stacked-capsule person-read (`H-2026-09-11-B`). FixedUpdate locomotion, stop-on-release, follow camera. Later scheduled persons must **rhyme** with this presentation. Today this file is the **local** body, not an NPC roster. |

A later slice verifies the live path, claims **only** what it needs, names any asset budget on its own ballot, and runs the Core gate. No YELLOW SLICE, no build from this file alone.

---

## 4. What “schedule” and “work loop” mean (still-frame bar)

Hour 1 does **not** require Radiant AI, owned beds, or a town simulation. It requires a stranger, looking at stills, to answer: *is that person doing the Place’s job?* *can I find them at this post again?* *if they vanished, could I still finish the hour?*

| Read | Pass | Fail |
|---|---|---|
| **Same post twice** | A person at a Ward / pipe / landing / well is nameable as *at work here*, and a later still can show the same post occupied on the same day. | Cutscene spawn. Random crowd. Peer-ghost theatre. Fake twelve Online. |
| **Same verbs** | The work is Tend / Take / Flow / Reserve / Mend, Place-honest (Threshold pipe still Tend-not-Take; Depths restore-not-Take). | NPC-only spell, combat, steal, fence, sell. |
| **Greet** | One short line on the existing HUD, hideable by **H**, that changes with Place + well mood. | Dialogue wall, crime meter, infamy, quest tracker. |
| **Hour without them** | Play / Continue / allocate / Settled+book / Places still complete with every person removed. | Quest strand, Place locked behind an NPC, book written by a person, schedule crash. |

Radiant ownership of objects and beds **transforms** to occupy/prefer **existing** dress (posts, shelf, lamp, landing) only. Preferences never write the house book. Fame/infamy **transforms** to standing + climate: a tired well and a cooler greeting, not a crime meter (`GDD_IMMERSION_REVISION` §2.4).

---

## 5. Acceptance — Core still-frame + hour-finish bar

Bots invent **no** cargo playtest and **no** fake screenshots. After a named Hands slice exists, Core posts stills. Until then this section is the **bar**, not a claim that the bar is met.

Door for the walk: `./scripts/play-offline.sh` or `cargo run -p powrush-client` with `POWRUSH_NET` off. Title Online grey in any Title frame.

| Shot | Expect |
|---|---|
| **Title** | Play / Continue / Settings · Online grey. No race lobby. No Market. |
| **Person at post** | At least one Place post (Ward / pipe / landing / well) shows a person doing a sacred-five verb, readable at default camera distance. |
| **Greet / mood** | Place + well mood still on the one climate slab; any greeting hushes with **H**. |
| **Hour-finish without persons** | Same stranger loop completes when every scheduled person is removed: allocate, Settled+book, Places, week on **L**. |
| **After H** | Guidance hushes; Place + mood + post remain nameable without a second HUD. |

### Fail → no next feel

| Fail | Meaning |
|---|---|
| **Hour stranded** | Removing persons blocks allocate, Settled+book, Places, or the week bill. |
| **Ownership economy** | Pickpocket, fence, owned-item currency, theft loop. |
| **Person combat** | Persons fight each other or the stranger as the schedule. |
| **Book / Place gate** | A person writes the house book or gates a Place. |
| **Second HUD / F-row** | A roster, timetable, or crime plate papers over presence. |
| **Online / Market / gold** | Title Online lit, Market row, gold word as score. |

Otherwise: **HOLD**. No OFFER NEXT for meshes until Core / Canon names the YELLOW SLICE with exact paths.

---

## 6. Explicit NON-goals / REFUSE

| Parked | Why |
|---|---|
| **Ownership economy · theft · fence · crime meter** | Oblivion stolen-item collapse refused (`GDD_IMMERSION_REVISION` §2.4). Credit stays repair-rights. |
| **Person combat** | Peace is the hour. Persons never break Peace, never fight, never remove a Place’s verb. |
| **Schedule that strands the hour** | Acceptance risk, not a later bugfix. Hour finishes with zero persons. |
| **Persons writing the house book** | Book is Settled + Embassy law, not NPC persist. |
| **Place gates** | Places stay Settled+book disk rooms (`SETTLED_DOOR_CLARITY`). No NPC key. |
| **Title Online / sockets / listen / public bind** | Grey until steward `online yes` on a HOLD ticket. Default binary opens zero sockets. |
| **Market · gold** | Market HOLD. Credit, not gold. Not in the House sum. |
| **Second HUD · F-row** | One HUD: satchel **I** · ledger **L** · climate slab · one card · Q faces. |
| **Art-pack freestyle · Quellorian mesh cargo** | No new mesh set, no texture pack, no animation library, no Codex/Imagine dump into the yard. Named art slice only, after YELLOW SLICE. |
| **Rung F · Sky / Net** | Place dress last; Sky only on `online yes`. |
| **WelcomeBack blank** | Parked own ticket. This rung does not fix it. |

Walked U0–U8 (and related README stamps) stay spent. Do not rebuild them.

---

## 7. Next Hands

Order:

1. This docs pack merges.
2. Core / Canon names **YELLOW SLICE** with **exact paths** (from the candidate table, not a freelance tree). Confirm-before-build remains in force until that card exists.
3. Feature branch, one slice, ballot filled. **This PR does not edit `client/**`.**
4. Core gate: `cargo test -p shared -p rsil-identity` and `cargo test -p powrush-client --lib`. Never `--workspace` as product-green.
5. Human stills against §5, including the **hour-finish without persons** bar. Bots invent no shots.

No CONFIRMED YELLOW SLICE, no build. A bot that cargos a mesh, a crime meter, or an Online listen from this file is out of seat (AGENTS.md).

---

## Refuse

`client/**` · `shared/**` · `server/**` · `data/**` · scripts · art / new assets · `Cargo.toml` · sockets / `POWRUSH_NET=on` · Title Online light · `0.0.0.0` · public bind · mesh / anim freestyle · Quellorian mesh cargo · race lobby · Market as shipped · gold · ownership economy · theft · fence · crime meter · person combat · schedule that strands the hour · persons writing the house book · Place gates · Brood Spire on Sanctuary · fifth Place · WelcomeBack blank as this slice · second HUD · F-row · rung F · Sky / Net from this file · `playable-preview` retag · invented Hour-two minutes · invented playtests or screenshots · Ra-Thor driving WASD / Use · certification / AGSi warranty / legal-product claims · xAI endorsement.

## Related

`GDD_IMMERSION_REVISION` · `STUDIO_ARCHITECTURE_ORDER` · `PERSON_READ_SPEC` · `SETTLED_DOOR_CLARITY` · `CREDIT_RESERVE_LOGISTICS` · `PLAYER_FEEL_METRICS` · `PLACE_CLARITY_HOUR` · `PLACE_CLARITY_WALK` · `PLACES_BIBLE` · `FUN_WITHOUT_WOW` · `OFFLINE_SKU` · `INPUT_CANON` · `SIM_AND_HAND_CANON` · `PATSAGI_MERGE_COURT`.

**Thunder locked in.** Yoi ⚡
