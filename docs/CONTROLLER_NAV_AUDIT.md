# CONTROLLER-NAV-AUDIT-1

main `e4d1f70531b57a5fe93b7cf320a69703cc6ec9da`. SOURCE-ONLY unless TEST-PINNED. Adopted by the PATSAGi Councils under Sherif's typed delegation; Sherif may veto or amend.

The verdict is the pad route. FULL: every pad act on that surface is reachable. PARTIAL: the pad can do some of those acts. MOUSE-ONLY: the acts are pointer presses. DEAD END: the pad cannot leave. NO PAD ROUTE: no pad button reaches the verb. A dash means that act is not on the surface. Each action cell is keyboard / pad. A named `--lib` test is TEST-PINNED. Every other cell is a source read at this main. Every row below is TEST-PINNED or SOURCE-ONLY.

## Table 1 — menus and surfaces

| Surface | Open kb / pad | Navigate kb / pad | Activate kb / pad | Close kb / pad | Verdict | Cite | Evidence |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Age gate | boot / boot | Tab, arrows / D-pad | Enter / South | — / — | FULL | `client/src/age_gate.rs:100-112` | TEST-PINNED `pad_only_boot_clears_age_gate_then_plays` |
| Title | boot / boot | Digit1 or Enter Play, Digit2 Continue, Digit3 Settings; Persona, Credits, and Quit are pointer / D-pad on live rows; Online has no row | those shortcuts, else pointer / South on the focused row | Esc closes Settings / East leaves Settings; East skips Persona | FULL | `client/src/title_screen.rs:2667-2804`; `client/src/launch_menu.rs:26-38`, `:95-134` | TEST-PINNED `pad_focus_reaches_every_live_row_and_never_online`; `credits_opens_licences_page_zero_then_backs_out_to_title` |
| Credits / Licences | pointer Credits row / South on that row | pointer Prev and Next / D-pad scrolls; LeftTrigger and RightTrigger page | pointer / those triggers | Esc / East | FULL | `client/src/licences_screen.rs:119-151`; `client/src/title_screen.rs:2756-2760` | TEST-PINNED `east_backs_out_and_dpad_scrolls`; `every_section_renders_and_prev_next_wrap`; `credits_opens_licences_page_zero_then_backs_out_to_title` |
| Settings | Digit3 on Title / South on the Settings row | Tab, Shift-Tab, `[` `]` / none once Settings is open | pointer rows, sprint-mode cycle included / none | Esc / East | PARTIAL | `client/src/title_screen.rs:2788-2792`, `:3241-3272`, `:4210-4312`; `client/src/launch_menu.rs:114-120` | SOURCE-ONLY |
| Pause | Esc or Digit3 / Start | same tab keys / none on InYard | pointer Resume, Title, Quit, Places / none | Esc / Start; Places open steps back to Pause | PARTIAL | `client/src/title_screen.rs:2796-2800`, `:2966-2986`, `:3045-3083`, `:4584-4601`; `client/src/launch_menu.rs:110-111`; `client/src/hex_travel.rs:598-619` | SOURCE-ONLY |
| Persona | pointer Persona row / South on that row | typing / none | Enter steps; Preview Enter commits; pointer cycles options / none | Esc skip / East skip | PARTIAL | `client/src/title_screen.rs:5155-5301`; `client/src/launch_menu.rs:115-116`; `shared/persona.rs:28` | SOURCE-ONLY |
| Ledger | L; no-op in Peace / North, same gate | Digit3, E, or Digit1 when the sash is open and charter skin is live / none | those keys / none | L / North | PARTIAL | `client/src/ledger_bind.rs:145-167`, `:169-216`; `client/src/input.rs:436-438` | SOURCE-ONLY |
| Fabricator | Q / West | the press is the step / the press is the step | plant, advance, or proof pack / same press | — / — | PARTIAL | `client/src/vertical_factory.rs:107-136`; `client/src/fabricator.rs:226-241`; `client/src/input.rs:430-434` | SOURCE-ONLY |
| Guidance card | default `active` / same | — / — | — / — | KeyH / none | PARTIAL | `client/src/first_session_guidance.rs:476-481`, `:1191-1201` | SOURCE-ONLY |
| Heartwood plaque | E while near / South while near | — / — | toggle / toggle | E, or leaving reach / South | FULL | `client/src/heartwood_plaque.rs:63-68` | TEST-PINNED `kh_p_pad_south_opens_plaque` |
| Places | pointer Places row when settled and the book are both held / none | pointer rooms / none | pointer confirm / none | Esc or pointer Back / Start steps back to Pause | MOUSE-ONLY | `client/src/hex_travel.rs:598-675`; `shared/hex_travel.rs:389-390`; `client/src/title_screen.rs:4592-4596` | SOURCE-ONLY |
| HUD edit | pointer HUD-tab edit button / none | pointer drag / none | pointer toolbar / none | Esc or pointer Cancel / Start is cleared while edit is active | MOUSE-ONLY | `client/src/hud_edit_mode.rs:168-309` | TEST-PINNED `edit_mode_ignores_pad_use_sheet_q_and_start` |
| NameHouse | InYard, once `HourSacred.complete`, while the house is unresolved / same | typing / none | Enter or pointer Confirm / none | Esc or pointer Skip / none | DEAD END | `client/src/title_screen.rs:2905-2936`, `:4589-4591`, `:4781-4884`; `client/src/launch_menu.rs:110-111` | SOURCE-ONLY |
| HouseDress | after the name resolves, while seals are unresolved / same | — / — | pointer seals, heritage, and rename; Enter confirms / none | Esc or pointer Skip / none | DEAD END | `client/src/title_screen.rs:2938-2943`, `:4901-4907`, `:4966-5046` | SOURCE-ONLY |

NameHouse and HouseDress share the pad gates: `yard_pause_step` returns `None` off InYard, and pad nav returns off Title.

## Table 2 — in-world verbs

| Verb | Open kb / pad | Navigate kb / pad | Activate kb / pad | Close kb / pad | Verdict | Cite | Evidence | FIRST10 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Satchel | I toggles / none | — / — | — / — | I / none | NO PAD ROUTE | `client/src/human_inventory.rs:235-243`; `client/src/soft_play_bindings.rs:28`; `client/src/first_session_guidance.rs:686`, `:1277-1278` | SOURCE-ONLY | B6, C5 |
| Allocate R/1/2 | R when eligible, or the auto offer / none | — / — | Digit1 flow, Digit2 reserve, pointer buttons / none | R or Esc / none | NO PAD ROUTE | `client/src/rbe_allocate_choice.rs:376-397`, `:487-507`; `client/src/soft_play_bindings.rs:30` | SOURCE-ONLY | B7, C6, C9 |
| Care-cycle offer | raised after idle-after-tend / same | — / — | Digit1, Digit2 / none | Esc or Digit3 / none | NO PAD ROUTE | `client/src/mercy_harvest_nodes.rs:507-517`, `:551-621` | SOURCE-ONLY | — |
| G | KeyG when charter skin is live; Peace keeps the sash closed / none | — / — | E or Digit1 aye, Digit2 nay / none | KeyG / none | NO PAD ROUTE | `client/src/coop_voice.rs:83-112`; `client/src/soft_play_bindings.rs:34` | SOURCE-ONLY | — |
| M/Z | M or F2, and Z or F3 / none | Digit1–5 while the realm panel is open / none | those digits / none | the same keys / none | NO PAD ROUTE | `client/src/human_soft_panels.rs:156-186`; `client/src/soft_play_bindings.rs:26-27`, `:45-46` | SOURCE-ONLY | — |
| H | — / — | — / — | KeyH dismisses / none | — / — | NO PAD ROUTE | `client/src/first_session_guidance.rs:1191-1201` | SOURCE-ONLY | B8, C7 |
| J | KeyJ toggles / none | — / — | — / — | KeyJ / none | NO PAD ROUTE | `client/src/abundance_journey_echo.rs:312-318`; `client/src/soft_play_bindings.rs:21` | SOURCE-ONLY | — |
| K | KeyK toggles close-up when the gate allows / none | — / — | — / — | KeyK / none | NO PAD ROUTE | `client/src/human_presence.rs:2183-2208` | SOURCE-ONLY | — |
| Tab | Tab when the hour-two door is ready / none | — / — | — / — | — / — | NO PAD ROUTE | `client/src/hour_sacred.rs:942-951`, `:1000-1013` | SOURCE-ONLY | — |
| Slots 3–4 | Digit3 writes `Some(2)`, Digit4 writes `Some(3)` / none (`RightTrigger` writes `Some(1)`) | — / — | strike is `Some(0)`; plate is `Some(1)`; Digit3 also toggles yard pause / none | — / — | NO PAD ROUTE | `client/src/input.rs:381-390`; `client/src/combat_core.rs:253-254`, `:590`; `client/src/title_screen.rs:2799-2800` | SOURCE-ONLY | — |
| Sprint | Shift / none while mode is `key` | — / — | Shift / LeftThumb in `stick`, LeftTrigger2 in `trigger` | release / release | PARTIAL | `shared/local_settings.rs:537-539`; `client/src/input.rs:418-422`, `:478-487` | TEST-PINNED `pp1_s7_other_pad_buttons_unchanged_and_rb_sets_no_other_verb` | B1 |
| Tend / extract | — / — | — / — | tap `resolve_take`, hold `resolve_tend` / South tap and hold via `interact` and `interact_held` | — / — | FULL | `client/src/first_harvest_epiphany.rs:572-665`; `client/src/input.rs:393-410` | TEST-PINNED `first_gamepad_use_hold_sets_interact_held` | B4, B5, C3, C4, C9 |

FIRST10 row ids are from `docs/FIRST10_AUDIT.md` at this main. B6 and C5 are the satchel rows.

## Conflicts

Record only. `docs/INPUT_CANON.md:61` names West / North as the satchel / ledger faces. `client/src/input.rs:433` maps West to `sheet_q`. `client/src/input.rs:436-438` maps North to `sheet_l`.

Record only. `shared/local_settings.rs:537-539` returns `"key"` as the sprint-mode default.

## Open items

`HourSacred.complete` is set by `try_mark_hour_two_held` when charter skin is live, the witness was seen, and the ledger contract is Settled (`client/src/hour_sacred.rs:976-984`), from `stamp_complete` (`client/src/ledger_bind.rs:227-241`). `persist_pack` writes that same predicate back onto the resource (`client/src/hour_sacred.rs:847-854`; `shared/hour_two.rs:58-63`). Boot `load_or_peace` copies a disk pack (`client/src/hour_sacred.rs:817-824`). NameHouse then reads the flag (`client/src/title_screen.rs:2911-2936`). These lines carry no minute count.

Ledger inner keys, once the sash is open and charter skin is live, are Digit3, E, and Digit1 (`client/src/ledger_bind.rs:169-216`). The L / North branch returns before those keys (`:165-167`). Fabricator and factory Q / West are the step itself (`client/src/fabricator.rs:226-241`; `client/src/vertical_factory.rs:107-136`). Those handlers have no focus walk.

The care-cycle offer stays up until `dismiss` (Esc or Digit3) or `choose` (Digit1 or Digit2) (`client/src/mercy_harvest_nodes.rs:158-166`, `:551-621`). Production code has no elapsed-time clear.

## Follow-up groups

D, A, B, C, F, E, G.
