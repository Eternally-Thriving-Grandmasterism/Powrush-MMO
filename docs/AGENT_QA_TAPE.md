# AGENT_QA_TAPE.md — machine PASS/FAIL roll-up

**CARD Q4 HOUR-TWO-TAPE** · tip `40e6692f` · design tick · not a Cargo bump  
**Named by:** [`NEXT_NAMED_CARDS.md`](NEXT_NAMED_CARDS.md) §1 · court [`AGENT_AUTONOMY_COURT_2026-09-22.md`](AGENT_AUTONOMY_COURT_2026-09-22.md) §3 CARD Q4  
**Model seat:** [`CURSOR_GROK_MODEL_SEAT.md`](CURSOR_GROK_MODEL_SEAT.md) — Grok 4.7 High · Hands cook only  
**Contact:** info@Rathor.ai · Independent of xAI. No certification / AGSi warranty.

Machine column only. Cite for tape content: [`FIRST_HOUR_PLAYTEST.md`](FIRST_HOUR_PLAYTEST.md) · [`HOUR_TWO_PLAYTEST.md`](HOUR_TWO_PLAYTEST.md) · `client/src/lib.rs` (`q0_*` · `q1_*`).  
Do **not** fill human Report blanks. No minutes. No OS/GPU.

**Bin:** NEVER for Online / Steam / `.glb`. Title Online stays grey. Floor `2163551`. Tag `playable-preview` `11c577e`. Workspace `21.88.0`.

**Core gate:**

```bash
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
```

---

## q0_* on tip (`client/src/lib.rs`)

Oracle eight + greys already on tip. PASS when `--lib` green. Do not rebuild Q0.

| id | Beat | Result |
|---|---|---|
| `q0_title_play_continue_settings_online_grey` | Play / Continue / Settings · Online grey | PASS |
| `q0_new_soul_light_sealed_continue_dress` | new soul = light · sealed Continue = dress | PASS |
| `q0_wrong_door_decline_still_light` | wrong door / decline = still light (S2) | PASS |
| `q0_each_people_landing_s1_f7_aftermath_line` | each People landing + S1/F7 aftermath | PASS |
| `q0_f1_stance_four_values_garden_no_stance` | F1 four stances · garden = no stance | PASS |
| `q0_f2_window_only_if_open_trade_ghost_lots_offline` | F2 window only if Open-trade · ghost lots offline | PASS |
| `q0_f3_hostile_take_allowed_f9_nevc_label_no_lockout` | F3 Hostile Take · F9 NEVC label · no lockout | PASS |
| `q0_f4_dress_stays_serve_other_well_offline` | F4 dress stays · serve other well offline | PASS |
| `q0_online_stays_grey` | Online grey support | PASS |
| `q0_place_id_stays_three` | PlaceId stays three | PASS |

---

## eight q1_* from Q1b #486 (`client/src/lib.rs`)

Stranger-hour machine beats. Cite [`FIRST_HOUR_PLAYTEST.md`](FIRST_HOUR_PLAYTEST.md). PASS when `--lib` green. Do not rebuild.

| id | Beat | Result |
|---|---|---|
| `q1_tap_e_take_not_equal_hold_e_tend` | tap-E take ≠ hold-E tend | PASS |
| `q1_satchel_nonempty_after_take` | satchel non-empty after take | PASS |
| `q1_r1_flow_restores_tired_well` | R then 1 flow restore | PASS |
| `q1_r2_reserve_nonzero_confirm` | R then 2 non-zero reserve | PASS |
| `q1_h_hides_card_well_words_remain` | H hides card · well words remain | PASS |
| `q1_lived_tick_roundtrip` | lived tick round-trip | PASS |
| `q1_dual_hud_labels_gate_and_quality` | dual HUD gate + quality labels | PASS |
| `q1_title_online_still_grey` | Title Online still grey | PASS |

---

## Hour-2 machine verbs already in `--lib`

Cite fail verbs in [`HOUR_TWO_PLAYTEST.md`](HOUR_TWO_PLAYTEST.md). Already walked on tip. Do not rebuild U0–U8 / H2-TAB/Q/L/RESUME. Human OS / GPU / minutes on that Report stay **blank**.

| Verb | Result |
|---|---|
| Tab without allocate stays Peace | PASS |
| Q in Peace does not found a House | PASS |
| L in Peace does not open Ledger | PASS |
| visitor E = *Not your charter* | PASS |
| resume skips WASD | PASS |

---

## Roll-up

| Family | Result |
|---|---|
| `q0_*` on tip | PASS |
| eight `q1_*` (#486) | PASS |
| Hour-2 `--lib` verbs (five) | PASS |

Hands DARK. 0 meshes · 0 `client/**` · 0 new crates · 1 new docs file (`docs/AGENT_QA_TAPE.md`).  
STEAM-SHORT-DESC banked until Q4 merges — do not touch `docs/STEAM_INTEGRATION.md` here.  
#459 closed. Online grey. After merge HOLD. Next only if Core names PATHS.
