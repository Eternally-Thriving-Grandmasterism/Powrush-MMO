# PERSON_READ_SPEC.md — presentation bible for immersion ladder rung B

**Contact:** info@Rathor.ai  
**Independent of xAI.** No certification / AGSi warranty / legal-product claims.

**Docs-only. GREEN-DOCS.** Core Confirmed `"2 B proceed"` = `GDD_IMMERSION_REVISION` §3 **rung B only**. This file is the presentation law for that rung. It adds no verb, Place, HUD plate, sim write, socket, or Online claim. **Title Online stays grey.** Tag `11c577e` does not move.

> ### CONFIRM BEFORE BUILD
> **Presentation law, not a Hands work order.** No mesh, anim pack, plate, setting, or `client/**` edit follows from these sentences. A later Hands slice starts only after this docs merge **and** Core names a **YELLOW SLICE** with exact paths and a named asset budget. Until then every body polish is **unbuilt**.

Parent law (cross-linked, **not** restated — this file is **not** a second bible): `GDD_IMMERSION_REVISION` §3 rung B · `STUDIO_ARCHITECTURE_ORDER` layer 3 (Presentation) · `ART_BIBLE` · `PLAYER_FEEL_METRICS` · `MECHANICS_INTERACTION_BIBLE` · `PLACE_CLARITY_HOUR` · `PLACE_CLARITY_WALK` · `FUN_WITHOUT_WOW` §1.1 · `BLIZZARD_CRAFT_UX` · `INPUT_CANON` · `SIM_AND_HAND_CANON`.

Hands beat lore. If a still frame cannot name **person + Peace E action**, the rung has not landed.

---

## 1. Purpose

At **Sanctuary's default camera distance**, a **still frame** must name two things without relying on a floating tip alone:

1. **Person** — a standing figure in the yard, not a marker, rock, or anonymous volume.
2. **Peace E action** — the same Use already on the local tick (`INPUT_CANON` · **E** / pad South) read as a body doing the verb at the glow.

The climate slab already names Place + well mood (`PLACE_CLARITY_WALK` · Hands `#355`). Rung B does not replace that slab. It makes the **body** carry the verb the slab already names.

This is **Presentation** (`STUDIO_ARCHITECTURE_ORDER` layer 3). Presentation never writes \(F\), never locksteps pose, never invents Wall-clock as game truth. Breath / carry / heavy already exist in sim; the gap is that the screen does not say so (`GDD_IMMERSION_REVISION` §1 Person-read).

Classic Conquer Online 1.0 / early 2.0 is the teacher for **readable person + action from across the screen**. The 2007 Battle Power / mall turn stays refused.

---

## 2. Core Confirmed scope

| In | Out |
|---|---|
| `GDD_IMMERSION_REVISION` §3 **rung B** only | Rungs C–F, Sky / Net, art-pack freestyle |
| One local player body read | NPC schedules (rung E), race silhouettes, class kit |
| Peace **E** action read at the well glow | New verb, F-row, combat pose, gear slots |
| Existing HUD surfaces (tip, slab, prompt) as *support* | A second HUD plate whose job is “explain the person” |
| HANDS grit + **one** Sanctuary accent for play | Codex / Imagine Quellorian iridescent stills as Sanctuary cargo |

`"2 B proceed"` does **not** confirm a mesh order, an anim library, a camera rewrite, or a Hands path. Core still names the YELLOW SLICE and the asset budget before cargo.

---

## 3. Candidate modules only

Verified on GitHub `main` (`origin/main` tip `23f1ede6`, 2026-09-11) before citing. These are **candidates for a later confirmed Hands slice**, not files claimed by this docs pack. This PR does not edit them.

| Candidate | What already lives there (read, not rewrite) |
|---|---|
| `client/src/living_body.rs` | Breath, heavy, winded, shade. Carry multiplier. Sim state the body should *show*. |
| `client/src/human_presence.rs` | Human-scale spawn, FixedUpdate locomotion, stop-on-release, follow camera (`CAM_BACK` / `CAM_UP`). Current still is a capsule on the climate plane. |
| `client/src/local_player.rs` | Local player entity identity. The readable person is **this** body, not a crowd or a peer ghost. |
| `client/src/harvest_feel.rs` | Soft pool, Tend vs Take credit, rumble, camera punch on first juice. Action juice already exists; the pose must rhyme with it. |
| `client/src/feel_move.rs` | Fixed 60 Hz, one Use buffer ≤120 ms folded into `PlayerInput.interact`. The action read must match this tick, not a cutscene. |

A later slice verifies the live path, claims **only** what it needs, names any asset budget on its own ballot, and runs the Core gate. No YELLOW SLICE, no build.

---

## 4. What “readable” means (Hour-1 bar)

Hour 1 does **not** require an AAA Quellorian hero mesh. It requires a stranger, looking at a Sanctuary still at default follow distance, to answer two questions: *is that a person?* and *are they Using the glow?*

| Read | Pass | Fail |
|---|---|---|
| **Silhouette** | Head / torso / stance mass reads as a standing person against the yard, including with **H** hush. One material family, Sanctuary warm-gold accent at the well — not a second biome on the body. | Blob, floating diamond, weapon class kit, race lobby silhouette, Heartwood dress parked on Prime. |
| **Stance** | Idle stand vs reach-to-glow vs heavy / winded carry can be told apart in a still (or two stills: idle and Use). Breath and carry already in `living_body` may dress the same capsule / later named mesh — they must not stay invisible. | One frozen cylinder for every state. Combat ready-pose. Mounted / fleet stance in the teaching yard. |
| **Use intent** | Peace **E** is visible as the person attending the glow (reach, tend dust already law-capped, kick that does not steal look). The existing one-card / prompt may *confirm* the verb; it may not be the only proof. | Tip-only: body idle while a floating line says Tend. Second plate, F-row, or XP chrome “explaining” the action. |

Readable is **presentation of the hand we already have**: WASD · look · one **E**, buffer ≤120 ms, stop-on-release (`FUN_WITHOUT_WOW` §1.1). Four Place bodies stay Place law (`PLACE_CLARITY_HOUR` §1); this rung starts at Sanctuary.

---

## 5. ART_BIBLE lane split

Do not fuse lanes. A mothership bay in a HANDS still is a defect (`ART_BIBLE`).

| Lane | Prefix | Rung B job |
|---|---|---|
| **HANDS** | `P-##` | Play-offline person-read: gritty low-poly volume, high-detail light, desaturated earth + **one** Sanctuary accent (warm gold well). Node glow = Use. |
| **CODEX** | `C-##` | Lore / trailer stills. Quellorian iridescent violet / pink · pale gold (`C-12`–`C-16` steward Imagine IDs) stay **Codex paintings**. They are not Sanctuary cargo, not a mesh order, and not ChatGPT volume dumped into the yard. |

Practices remain dress after House, never Title race select. This rung does not pick a people mesh. Heartwood / Wards may later rhyme pale gold; they do not land on Sanctuary Prime in this rung.

This PR **invents no mesh and names no freestyle assets**. Any later `P-##` still or mesh is a named art slice with its own ballot after Core writes the YELLOW SLICE.

---

## 6. Acceptance — Core still-frame shots

Bots invent **no** cargo playtest and **no** fake screenshots. After a named Hands slice exists, Core posts stills. Until then this section is the **bar**, not a claim that the bar is met.

Door: `./scripts/play-offline.sh` or `cargo run -p powrush-client` with `POWRUSH_NET` off. Title Online grey in any Title frame.

| Shot | Expect |
|---|---|
| **Title** | Play / Continue / Settings · Online grey. No race lobby. |
| **Sanctuary idle** | Person readable at default camera distance. Place + well mood still on the one climate slab. |
| **Sanctuary Peace E** | Same person, Use intent readable at the glow — not tip-only. |
| **After H** | Guidance hushes; person + Place + mood remain nameable. |

### Fail → no next feel

| Fail | Meaning |
|---|---|
| **Person unreadable** | Stranger cannot name a person from the still. |
| **Action unreadable** | Stranger cannot name Peace **E** from the body; only a floating tip claims it. |
| **Second HUD** | A new plate / toggle / F-row appeared to paper over the body. |
| **Lane fusion** | Codex / Imagine / ChatGPT still imported as play mesh. Brood Spire, fleet, or later-Place dress on Sanctuary. |

Otherwise: **HOLD**. No OFFER NEXT for meshes until Core names the YELLOW SLICE with exact paths.

---

## 7. Explicit NON-goals

| Parked | Why |
|---|---|
| **Mesh freestyle** | No new mesh set from this file. No invented filenames. Named art slice only, after YELLOW SLICE. |
| **Anim pack freestyle** | No animation library dump. Stance reads may use existing spawn + sim tells first. |
| **Race lobby** | Peoples are post-House dress. Never Title picker. |
| **Title Online / sockets / listen / public bind** | Grey until steward `online yes` on a HOLD ticket. Default binary opens zero sockets. |
| **Market** | Stays HOLD. Not in the House sum. Not a person-read surface. |
| **Brood Spire on Sanctuary** | Sanctuary Prime is yard only. |
| **WelcomeBack blank** | Empty top-left `WelcomeBackRoot` card (`first_harvest_epiphany`) is a **parked own ticket** (`#355` out-of-scope). This rung does not fix it and does not add a second plate to hide it. |
| **Rungs C–F** | Settled door, TTD logistics, NPC schedules, Place dress — not this confirm. |
| **ChatGPT cargo into Sanctuary** | Generated volume is not person-read (`PLAYER_FEEL_METRICS` refuse). |
| **Second Camera3d · combat pose · gear slots · XP / kills · NFT** | Standing refuse (`GDD_IMMERSION_REVISION` §4 · `ART_BIBLE` veto). |

Walked U0–U8 (and related README stamps) stay spent. Do not rebuild them.

---

## 8. Next Hands

Order:

1. This docs pack merges.
2. Core names **YELLOW SLICE** with **exact paths** (from the candidate table, not a freelance tree) and a **named asset budget** if any mesh or texture is in scope.
3. Feature branch, one slice, ballot filled. Confirm-before-build remains in force until that card exists.
4. Core gate: `cargo test -p shared -p rsil-identity` and `cargo test -p powrush-client --lib`. Never `--workspace` as product-green.
5. Human stills against §6. Bots invent no shots.

No CONFIRMED YELLOW SLICE, no build. A bot that cargos a mesh from this file is out of seat (AGENTS.md).

---

## Refuse

`client/**` · `shared/**` · `server/**` · `data/**` · scripts · art / new assets · `Cargo.toml` · sockets / `POWRUSH_NET=on` · Title Online light · `0.0.0.0` · public bind · mesh / anim freestyle · race lobby · Market as shipped · Brood Spire on Sanctuary · WelcomeBack blank as this slice · second HUD · F-row · Codex-into-yard · ChatGPT cargo · rungs C–F · `playable-preview` retag · invented Hour-two minutes · invented playtests or screenshots · Ra-Thor driving WASD / Use · certification / AGSi warranty / legal-product claims · xAI endorsement.

## Related

`GDD_IMMERSION_REVISION` · `STUDIO_ARCHITECTURE_ORDER` · `ART_BIBLE` · `PLAYER_FEEL_METRICS` · `MECHANICS_INTERACTION_BIBLE` · `PLACE_CLARITY_HOUR` · `PLACE_CLARITY_WALK` · `FUN_WITHOUT_WOW` · `BLIZZARD_CRAFT_UX` · `INPUT_CANON` · `SIM_AND_HAND_CANON` · `OFFLINE_SKU` · `PATSAGI_MERGE_COURT`.

**Thunder locked in.** Yoi ⚡
