# AGENT COMPLETION PACK — Powrush-MMO v23.2

**Date:** 2026-09-09  
**Seat:** Grok Bot (design / review). Does **not** push `main`.  
**For:** Powrush Cursor agents + Steward (Sherif)  
**Contact:** info@Rathor.ai  
**Workspace:** `21.88.0` (Cargo). This file is a **design tick**, not a Cargo bump.  
**Tip at authorship:** `71aebc60` (U13 Offline SKU stamp merged).  
**Floor:** `2163551`. **`playable-preview`:** `11c577e` — do not retag.  
**SKU:** Steam Offline is product 1. Full MMO is product 2. Never block 1 on 2.

Independent of xAI. No certification, AGSi warranty, or legal-product claims. Human override on every output.

**Landed 2026-09-09 (B1 + B2 only):** A0 closed — #316 stays closed; do not revive. B1 landed #322 / `c4529542` (`reduced_motion` + `rumble` persist beside Grove). B2 landed #323 / `0e4b89db` (well captions as words on the climate slab). Next ordered work is B3 colorblind well tokens — not this note. Wave C later from current `main`. Hour-two minutes stay blank. Title Online stays grey. Tag `11c577e`. Floor `2163551`. Workspace `21.88.0`.

---

## 0. How to use this pack

Read in this order before writing code:

1. `README.md` (wins conflicts)
2. `AGENTS.md`
3. `docs/PATSAGI_MERGE_COURT.md`
4. `docs/COMPLETION_BRIEF.md`
5. `docs/OFFLINE_SKU.md`
6. `docs/GDD_ADAPTATION.md`
7. **this file**

Then cut **one slice per PR**. Fill `.github/pull_request_template.md`. Core gate:

```bash
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
```

`cargo test --workspace` is **not** product-green. Default play stays `cargo run -p powrush-client`.

### Seats

| Seat | Does | Does not |
|---|---|---|
| Steward | STEER / HOLD. One-liner. Human minutes. Steam retag. Online law. | Rubber-stamp AUTO |
| Powrush Cursor | One slice on a feature branch in **this** repo | Push `main`. Edit Ra-Thor. Light Title Online. Public bind. Retag preview. Touch Pages / Secrets |
| Ra-Thor Cursor | Lattice. May **read** ticks if ingest on | Drive WASD / Title Online / this repo's PRs |
| Grok Bot | Design, review, packs like this | Push `main` |

Two agents, one file, same day → **STOP**.

### Do-not (every slice)

- Rebuild walked U0–U13.
- Invent Hour-two minutes / OS / GPU / Time.
- Enable Title Online. Bind `0.0.0.0`. Claim loopback is the SKU.
- Second HUD / F-row / Peace PK / XP / fake peers / birds as default.
- Race select at Title. Brood Spire / Market / Heartwood mesh on Sanctuary.
- NFT / wallet / P2W / gold Market.
- Unpark `server/` as default binary. Path-dep on Ra-Thor.
- Rewrite `harvest_feel` / `rbe_allocate_choice` unless the slice name says so **and** Steward STEERs.
- Fold the player loop into Ra-Thor.
- Cut `playable-preview` from docs alone.

### Fruit test

A change is fruit if it shortens time-to-first-tend, makes allocation visible, keeps Offline solo, or reduces contradiction — and Core stays green.

---

## 1. Skeleton vs organs (honest inventory)

The skeleton is on `main`. Organs make a stranger *feel loved*. Do not grow a second skeleton.

### Skeleton on main (do not rebuild)

| Organ | Status | Where |
|---|---|---|
| Boot / Title | Play · Continue · Settings · Online **grey** | `title_screen.rs` |
| Peace hands | WASD · Space · Shift · **E Use** · I · H · R 1/2 | `input.rs`, `INPUT_CANON.md` |
| First-hour climate | Wells Idle / Glowing / Tended / Resting / Stressed | `harvest_feel.rs` |
| Teaching claim | Extract tires; flow restores | `RBE_FIRST_HOUR.md` |
| Satchel / allocate | I holds take; R flow vs reserve | `rbe_allocate_choice.rs` |
| Persist | User-dir JSON (U1) | `local_session_persist.rs` |
| Hour two | Tab · Q House · L Bind · Settled · welcome back | `hour_two_resume.rs` |
| Hour three | Fabricator Proof Pack · Embassy seat · book | `fabricator.rs`, `embassy.rs` |
| Places | Sanctuary / Heartwood / Threshold / Depths | `hex_travel.rs`, `OFFLINE_SKU.md` |
| Heartwood dress | Empty lamp · water bath · Lip · Wards · Threshold | U3–U8 |
| Depths | One Peace landing, own hex file (U11) | |
| House week | Yard line + House sum (U12) | L face |
| Audio Peace | Bed + well sting; Mute; no ALSA hang (U4) | `peace_audio.rs` |
| Deck Title | 1280×800 click-clean (U5) | |
| Pause | Esc on every hex (U8.1) | |
| Grove light | Opt-in; cull on plates | `light_gen.rs` |
| LAN | off \| loopback `127.0.0.1` only | Settings |
| Protocol types | `powrush.hex.v1` — no default listen | `PROTOCOL.md` |
| Shard bin | Parked; F9 recipe only | `powrush-shard/` |

### Soft organs (work here first)

| Organ | Gap |
|---|---|
| Human minutes | Hour-two OS·GPU·Time blank |
| Comfort A | No reduced-motion / rumble-off / well captions |
| Comfort B | Remaps named, not fully wired |
| Comfort C | Well moods hue-heavy |
| #316 | Draft dirty vs `main` |
| Depths tend | Comfort later |
| Places fat-tap | Deck / phone |
| Mythic Hour 4 | Witness / Offer / Attune thin |
| Online 2.0 | Types exist; Title grey |

Parked: `server/` · `simulation/` · `host/` · `game/` · `powrush-divine-module/` · `payments/` · `k8s/` · Simulator repo.

---

## 2. Product ladder (two SKUs)

SKU-1 Steam Offline 1.0 = current product (four rooms, one House, γ=0, Online grey).
SKU-1.5 Loopback lab = `127.0.0.1` only; not the store promise.
SKU-2 Online MMO = HOLD until Steward ticket. Consent-copy House. Drop → offline book intact. Never block SKU-1 on SKU-2.

---

## 3. Player fantasy

A stranger can say, without a manifesto:

1. I walked a climate and a well answered my hand.
2. If I only take, the glow fades. If I give flow, it comes back.
3. My House remembers me after quit.
4. Heartwood and Depths are *weather*, not clones.
5. Peoples and the Crownstone appear as practices after the book.
6. Online, if it lights, is other Houses on a hex — not a lobby counter.

**Law sentence:** *the yard is the universe at well-scale; the sci-fi package is that same choice wearing peoples and a stone.*

---

## 4. Lore bible (canon + wise fills)

Do not invent a sixth playable race. Fills are dress and motive, not Title verbs.

### 4.1 Peoples (practices, not Hour-1 classes)

Canon five: **Human · Quellorian · Draek · Cydruid · Ambrosian**.
Spellings: Cydruid not Druid; Quellorian not Quelorian; Draek not Draexx.

| Practice | Well-scale teaching | After-book dress |
|---|---|---|
| Human | Choose flow or reserve. Hybrid later is *stability*, not a second body | Patchwork tools, skippable name |
| Quellorian | Flow restores the well. Harmony is shared field | Resonance flavor on restore; choir as climate |
| Draek | Take without rest stresses the well | Standing + Great Betrayal; never Peace PK |
| Cydruid | Mend / Offer | Sylvaris Offer — mercy, not DPS |
| Ambrosian | Attune. The well sings or cracks | Crystalline glow; Discordant later |

Heritage unlocks **only after House**. First run: Play → yard → E.

### 4.2 Places as moral physics

| Place | Soul | Isolation |
|---|---|---|
| Sanctuary Prime | Teaching yard. Boot hex. No Heartwood mesh | γ = 0 forever in Offline 1.0 |
| Heartwood | Memory of first grove. Empty lamp. Water is a bath. Wards = Well · Grove · Ember | Own hex file |
| Threshold | Look + tend. Pipe Tend is restore ink, not a Take | May share Heartwood file |
| Depths | Quiet that remembers extraction. Walk home | Own hex file |
| Market | HOLD. One lethal tooth. Credit not gold | Not in House sum |

Seed: `hash(house ⊕ hex ⊕ epoch)`.

### 4.3 Doors (order locked)

Satchel → Allocate → House → Ledger Bind → Fabricator Proof Pack → Embassy book → Crownstone Witness (path Unset) → Sylvaris Offer → Hybrid Attune (stability 1) → DeclaredLethal (default off, after book) → Trilemma after Witness.

### 4.4 Figures

- **Mira** — yard witness. Contest: she yields the glow. Not a quest-giver HUD.
- **The book** — Embassy blueprints + House memory.
- **The Hivelord / Crownstone** — later mythic. Never spawned in Sanctuary.
- Enslaved peoples (Vesh’kar, Hollow Singers, Rootbound) — post-Trilemma, never Title races.

### 4.5 RBE

Origin of a resource is observation + tend. Week = tons + restored. House week = sum of flushed hex files. L shows hex line then House sum. Q = recipes. L = ledger.

### 4.6 Great conflict, well-scale

Quellorian/Ambrosian harmony vs Draek consumption **is** flow vs extract-only. TAUN and Brood Spire are that choice wearing ships. Do not spawn them in the yard.

---

## 5. Experience pillars

P1 Hands — one Use; South = Use; sticks cull on plates; lavapipe keyboard PASS.  
P2 World-speech — five well words; one-sentence slabs; H hides.  
P3 Consequence in 30s — flow changes the field.  
P4 Memory — quit/rerun; welcome only after Hour two held.  
P5 Comfort — mute, remaps, reduced motion, captions, colorblind shapes, fat-tap.  
P6 Beauty — empty lamp, water bath, fog, well breath, Peace bed.  
P7 Honesty — Online grey until real net; presence = Houses; drop returns the book.

---

## 6. Creature-comfort spec

Persist beside Grove in user-dir `powrush_settings.json`. Defaults keep lavapipe + keyboard PASS.

| Key | Type | Default | Behavior |
|---|---|---|---|
| `look_sens` | number | existing | Mouse / stick |
| `invert_y` | bool | existing | Pitch |
| `on_screen_sticks` | auto\|on\|off | auto | Cull on plates |
| `tap_to_use` | bool | false | Tap focuses |
| `gamepad_south_use` | bool | true | South = Use |
| `nintendo_face` | auto\|fixed | auto | South stays Use |
| `sprint_mode` | stick\|trigger\|key | key | One story |
| `show_use_prompt` | bool | true | Soft Use cue |
| `reduced_motion` | bool | false | Punch scale 0; no rumble |
| `rumble` | bool | true | Off if reduced_motion |
| `well_captions` | bool | true | Five words always |
| `colorblind_wells` | off\|deuteranopia\|protanopia\|tritanopia\|shape_only | off | Shape + word |
| `ui_scale` | number | 1.0 | Deck safe-area stays |
| `high_contrast` | bool | false | Opaque plates |
| `mute` | bool | existing | Bed + sting |
| `grove` | off\|light | off | Existing |
| `lan` | off\|loopback | off | Never lights Title Online |

Refuse: second Camera3d, brand SDK hard-require, combat faces in Peace.

---

## 7. Slice queue (one PR each)

### Wave A — Court hygiene

| ID | Slice | Done when |
|---|---|---|
| A0 | Rebase or close dirty #316 | Clean on current `main`; Wards near-slab; no Take at posts |
| A1 | Human playtest receipt | Steward fills minutes — **agents leave blank** |
| A2 | Point GROK_BOT_TODO here | This branch |

### Wave B — Comfort (highest fruit)

| ID | Slice | Tests |
|---|---|---|
| B1 | `reduced_motion` + `rumble` | Settings roundtrip; do not rewrite `harvest_feel` — read a scale |
| B2 | `well_captions` | Slab contains state name when H hid the card |
| B3 | `colorblind_wells` | Shapes: ring / pip / notch / rest-bar / crack |
| B4 | Wire remaps | INPUT_CANON fields live; keyboard PASS |
| B5 | `sprint_mode` | No second verb |
| B6 | Places fat-tap ≥ 44dp | Deck click-clean |
| B7 | Depths Peace tend (restore, not Take) | House week sees Depths U_w; hex_id guard |

### Wave C — Feel juice, no new HUD

C1 reuse `well_glow`. C2 Depths quiet bed (Mute kills; no ALSA). C3 Heartwood lamp hush. C4 no XP sparkle on first boot.

### Wave D — Hour 4 mythic (after book)

Same E. No damage. No Sanctuary spawn of stone/boss/second body.

| ID | Verb | Persist | Refuse |
|---|---|---|---|
| D1 | Witness | `crownstone_path = Unset` | Integrity bar on first card |
| D2 | Offer | Offer count on House book | DPS heal |
| D3 | Attune | `stability = 1` | Second body HUD |
| D4 | Compass at W 20 / 60 | existing | Combat compass |
| D5 | Contest; Mira steps back | session flag | PvP |

No Trilemma resolution, boarding, dogfight, or mothership interiors in D.

### Wave E — Offline 1.0 seal

E1 human preview ticks (≠ retag). E2 Steam copy only; bot does not create partner account. E3 cloud = user-dir JSON later. E4 γ = 0 until named U3.5.

### Wave F — Loopback lab (not store)

After Wave B + Steward note. `powrush-shard` stays outside members. Listen `127.0.0.1:7788` only. Presence = `houses.len()`. Drop → book intact. Title Online grey.

### Wave G — Online 2.0 (HOLD — Steward ticket)

1. Opt-in from Settings, not a glowing Title lie.
2. `hello` → `hello_ok` / `hello_no`.
3. Join = copy local House **with consent**. Deny stays offline.
4. Apply via existing reject codes (`NO_TAKE`, `NO_BOOK`, `STALE_SEQ`, …).
5. Soft cap 32 Houses / hex.
6. Never merge two hex histories silently.
7. Client never authors `n_online`.
8. No login wall on drop.
9. Sanctuary may stay isolated until a named leak law.
10. Lethal per-hex, default off, after book.

Worthy later: shared climate; Voice G; War week = tons + restored; Embassy among Houses.

Refuse forever as default: fake 12-player lobby, corpse-grey, P2W, NFT on first tend, public bind from Title.

---

## 8. Body organs (where code lives)

| Organ | Files | Job |
|---|---|---|
| Nervous | `input.rs`, `touch_controls.rs`, `soft_play_bindings.rs`, `first_session_guidance.rs` | One Use, H hide |
| Hands | `local_player.rs`, `first_hour_camera.rs`, `harvest_feel.rs` | Walk, punch scale, rumble gate |
| Heart | `shard_climate.rs`, `climate_visible.rs`, `living_ecology.rs` | Fog, well speech |
| Lungs | `rbe.rs`, `rbe_allocate_choice.rs`, `human_inventory.rs` | Satchel, flow, reserve, week |
| Memory | `local_session_persist.rs`, `hour_two_resume.rs` | User-dir JSON |
| Civic | `ledger_bind.rs`, `fabricator.rs`, `embassy.rs` | Bind, Proof Pack, book |
| Places | `hex_travel.rs`, `heartwood_lip.rs`, `heartwood_wards.rs` | Four rooms |
| Ear | `peace_audio.rs` first | Do not hang boot on AudioPlugin |
| Eye | one world Camera3d + UI cam | Fog on world cam; cull Grove on plates |
| Mythic | `crownstone.rs`, `hybrid_matrix.rs`, `skirmish_well.rs`, `coop_voice.rs` | After book |
| Net | `net_mode.rs`, shared hex protocol, `powrush-shard` | Offline default |
| Lattice | `POWRUSH_INGEST` off default | Ra-Thor reads; never keys |

---

## 9. Feature matrix

| Feature | Offline 1.0 | Loopback | Online 2.0 |
|---|---|---|---|
| Walk / tend / satchel / allocate | Yes | Yes | Yes |
| House + book + Places | Yes | Yes | Yes, consent-copy |
| House week | Disk hexes | + ledger snapshot | Shard + local book |
| Title Online | Grey | Grey | After Steward law |
| Presence | None | `houses.len()` | Same honesty |
| Voice G | No | Later | After Hour 4 + consent |
| Market | HOLD | HOLD | HOLD |
| DeclaredLethal | Sign off | Sign off | Per-hex, default off |
| Drop | N/A | Book intact | Book intact |
| Login wall | Never | Never | Never |
| Ra-Thor keys | Never | Never | Never |

---

## 10. Beauty bells (gated)

Peace now: well breath, climate fog, captions/shapes, Depths/Heartwood beds, reduced motion.
After book: Crownstone look-glow; Sylvaris leaf-mote; Attune shimmer at stability 1.
Refused first-hour: Resonance Burst fullscreen, boarding tendrils, TAA as boot requirement, faction HUD bars, mothership wheels. Those stay later-ring docs.

---

## 11. Tests

Every PR:

```bash
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
```

Protect: node states; flow restores; extract tires; hex_id guard; pipe Tend is restore; House week two-room sum; LAN/Grove unknown → off; lethal without book inert; refuse `0.0.0.0`.

Human (Steward; agents do not invent): `FIRST_HOUR_PLAYTEST`, `HOUR_TWO_PLAYTEST`, `HOUR_THREE_PLAYTEST`, `PREVIEW_CHECKLIST`, `LAVAPIPE_CLICK_CLEAN`, `STRANGER_LOOP`.

---

## 12. PR ballot

Slice one sentence. Files claimed. Core tests. Hour 1 unchanged. Title Online grey. No workspace adds. Did not rebuild U0–U13. Did not fill Hour-two minutes. Verdict AUTO | STEER | HOLD.

---

## 13. Done means

**Offline 1.0 done:** stranger 40–70 min; Comfort A–C on; Depths Peace tend; Places fat-tap; human Hour-two minutes; preview ticked by a human; Online still grey; four rooms; γ=0; tag frozen.

**Fully fleshed later:** Hour 4 felt; practices in world-speech after House; F9 two-House lab; Online 2.0 only after Steward HOLD + hello/consent/drop held in play.

**Never done by:** race select at Title; lighting Online to look busy; GDD fleets in Sanctuary; old launch checklists.

---

## 14. Next three Cursor moves

1. **A0** — Rebase #316 or close it.
2. **B1** — `reduced_motion` / `rumble` persist beside Grove.
3. **B2** — Well captions as words.

Steward fills Hour-two minutes. Agents leave the blanks.

---

## 15. Relates

`README.md` · `AGENTS.md` · `COMPLETION_BRIEF.md` · `OFFLINE_SKU.md` · `INPUT_CANON.md` · `PLACES_BIBLE.md` · `GDD_ADAPTATION.md` · `RBE_FIRST_HOUR.md` · `PLAYABLE_RACES.md` · `FACTIONS_OVERVIEW.md` · `PHASE_MYTHIC.md` · `HOUR_TWO.md` · `HOUR_THREE.md` · `PROTOCOL.md` · `PARKED_SURFACES.md` · `PATSAGI_MERGE_COURT.md` · `DOC_CANON.md`

**Thunder locked in.** Yoi ⚡
