# PLAYTEST_AUDIT_2026-09-09 — docs-only code check (B7 tip)

**Contact:** info@Rathor.ai  
Independent of xAI. No certification / AGSi warranty / legal-product claims.

**Base:** `c56b261e0aa00a0f9aad4aa118ea4d164f18c43a` — B7: tend the Depths Peace node (#329)  
**Tag stays:** `playable-preview` `11c577e`  
**Floor stays:** `2163551`  
**Workspace stays:** `21.88.0` (not a Cargo bump)

This is a **docs-only** audit. It is **not** a human GPU walk, **not** a lavapipe play, **not** invented Hour-two minutes, **not** a Ra-Thor drive, **not** a Title Online light. Checks are from code + Core tests on this SHA. `docs/HOUR_TWO_PLAYTEST.md` minutes stay blank. `docs/FIRST_HOUR_PLAYTEST.md` OS / GPU / Time stay blank.

No `client/**` or `shared/**` edits in this slice. Assertions already exist for the six doors; this stamp cites them.

---

## Core counts (this SHA)

Commands run (not `cargo test --workspace`):

```bash
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
```

| crate | result |
|---|---|
| `rsil-identity` | **5 passed**; 0 failed; 0 ignored |
| `shared` | **311 passed**; 0 failed; 0 ignored |
| `powrush-client --lib` | **202 passed**; 0 failed; 0 ignored |

Product-green for this slice is those two Core commands only.

---

## Six checks (code, not play)

| # | Claim | Verdict | file:fn |
|---|---|---|---|
| 1 | Default keys still WASD / E / I / H / R when settings are default | **PASS** | `shared/local_settings.rs:peace_keys_are_default` (defaults W/S/A/D + `default_key_use` E, `default_key_satchel` I, `default_key_hide` H, `default_key_allocate` R); default-path bypass `client/src/input.rs:apply_peace_keyboard` (returns immediately when `peace_keys_are_default()`) |
| 2 | After H, well sentence + token still exist | **PASS** | `client/src/climate_visible.rs:well_state_sentence` with `well_state_token` Idle `•` / Glowing `○` / Tended `✓` / Resting `—` / Stressed `!`; `climate_slab_should_show` keeps a well in range when guidance is hidden; test `hidden_guidance_keeps_each_well_state_sentence_visible` |
| 3 | Wards notice is `WARDS_NOTICE`; E is dress; book not written | **PASS** | `shared/heartwood_wards.rs:visit_wards` + `WARDS_NOTICE` (`"Wards · E tend · Well · Grove · Ember"`); `WardDress::apply` Look/Tend; test `look_and_tend_keep_book_places_and_sanctuary_return` (`house` unchanged). Client E is tend dress only: `client/src/heartwood_wards.rs:use_heartwood_wards` (does not load/write the house pack) |
| 4 | `depths_near` claims Use; restore writes `powrush_hex_depths.json`; tons and satchel stay flat | **PASS** | `client/src/depths_landing.rs:mark_depths_node_near` sets `FirstHarvestEpiphany.depths_near`; `restore_depths_hex` writes via `write_hex_named` (`hex_file_name(PlaceId::Depths)` = `powrush_hex_depths.json`); test `depths_use_restores_its_hex_file_without_take_or_stock` (tons unchanged, `satchel_count` unchanged) |
| 5 | Title Online stub still disabled | **PASS** | `shared/hex_listen.rs:PowrushNet::title_online_enabled` always `false`; `shared/net_mode.rs:NetMode::title_online_enabled` always `false`; Title spawn `client/src/title_screen.rs` `ONLINE_STUB_LABEL` with `enabled: false`; `shared/title_house_proof.rs:online_row_is_honest_disabled` |
| 6 | `harvest_use_is_claimed` includes threshold \| wards \| depths | **PASS** | `client/src/first_harvest_epiphany.rs:harvest_use_is_claimed` = `threshold_near \|\| wards_near \|\| depths_near`. Tests: `wards_claim_use_as_dress_not_take`, `depths_claims_use_as_restore_not_take`. Threshold flag from `client/src/heartwood_lip.rs` (`epiphany.threshold_near = in_reach`) |

---

## One stranger-flow risk (honest)

On Heartwood, the climate slab prefers Wards over a well: `wards_line.or(well_line)` in `client/src/climate_visible.rs`. A Ward post radius (`WARD_USE_RADIUS`) that also overlaps a well will show `WARDS_NOTICE`, claim Use (`wards_near` → `harvest_use_is_claimed`), and hide the well Idle/Glowing/Tended/Resting/Stressed sentence. That is a code overlap, not a GPU anecdote. A stranger who still treats every glow as harvest E can press E and get dress instead of take/tend. Not theatrical; not a Title Online issue.

---

## Title Online

**Do not light Title Online.** This audit does **not** recommend enabling it, binding, listening, or promoting the Title row. Stub stays `"Online — off (no listen)"`, grey, `title_online_enabled` false on Off / Localhost / Online label. `PROTOCOL.md`: client does not author `n_online`. Default `POWRUSH_NET=off`.

---

## Next door (Steward names later)

Parked, not this PR:

- **P2 slab priority** — well state sentence vs Wards notice when both are in range, **or**
- **Depths slab line** `"Depths Peace · restored"` — today that string is `DepthsPeaceTend.last_line` + an info log after `restore_depths_hex`; `climate_visible` does not read that resource.

Steward names which door. Agents do not freelance it here.

---

## Also read (law, not replayed)

- `docs/FIRST_HOUR_PLAYTEST.md` — five-minute script; this stamp does not fill OS / GPU / Time.
- `docs/OFFLINE_SKU.md` — Steam Offline first; Title Online off (no listen).
- `docs/PROTOCOL.md` — presence is houses only; no client-authored `n_online`.
- `docs/PRACTICE_NOT_VERB.md` — E Use / I satchel / H hide / R allocate stay species-agnostic.

---

## Refuse

- Hour-two minutes invented.
- Human GPU / lavapipe walk claimed.
- Ra-Thor driving keys.
- Title Online on. Public bind. Listen. `0.0.0.0`.
- Retag `playable-preview`. Floor bump. Workspace Cargo bump.
- `client/**` / `shared/**` edits in this slice.
