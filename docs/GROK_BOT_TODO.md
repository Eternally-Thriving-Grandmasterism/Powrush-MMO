# GROK_BOT_TODO.md

Tip `db05095` — TODO9 on main (#419). This PR is a **design tick**, not a Cargo bump.

**Canon pack:** [`docs/AGENT_COMPLETION_PACK_v23.2.md`](AGENT_COMPLETION_PACK_v23.2.md)  
**Player-experience canon (review only, 2026-09-11):** [`docs/GROK_BOT_PLAYER_EXPERIENCE_BRIEF.md`](GROK_BOT_PLAYER_EXPERIENCE_BRIEF.md)

Hands HOLD. One card in flight. Bot 1 Hands executes Hands. Canon vote-only on Hands. Title Online stays grey.

## Closed / landed (do not rebuild)

Court 2026-09-11–12. Do not re-litigate. Do not rebuild.

- **A0** — Closed. #316 stays closed. Do not revive. Do not merge dirty.
- **B1** — Landed #322 / `c4529542`. Persist `reduced_motion` + `rumble` beside Grove.
- **B2** — Landed #323 / `0e4b89db`. Well captions as words (Idle / Glowing / Tended / Resting / Stressed) even when H hid the card.
- **B3** — Landed #376 / `80962065`. Colorblind well tokens (shapes + word). Do not rewrite `harvest_feel`.
- **B4** — Landed on tip (INPUT_CANON remaps + Settings rows). Do not re-litigate. B4 remaps remain landed.
- **B5** — Landed/stamp. Practice≠Verb law already on main at [`docs/PRACTICE_NOT_VERB.md`](PRACTICE_NOT_VERB.md); `sprint_mode` live on Controls (stick|trigger|key) — no second Peace verb; E/I/H/R untouched. Do not invent Hands client paths for B5.
- **B6** — Closed. PLACES-FAT #407 / `b33c1a6`. Places deck + Confirm/Back ≥44dp. Wave B Comfort B1–B7 all closed.
- **B7** — Landed on tip. Depths Peace tend (restore, not Take). Do not re-litigate.
- **Place dress** — F0 docs landed #374 / `ff97b505` (`PLACE_DRESS_SPEC`). F1–F4 Hands on `climate_plane.rs` landed #377–#380 (`f34c6e98` … `bfaa455e`).
- **Temper** — T0 docs landed (#370). T1–T5 landed #381–#385 (`shared/temper` · fabricator · human_inventory · mercy_harvest_nodes).
- **MERCY Persona** — P0 docs landed (#372). P1–P5 landed #386–#390. Tip before FLAG was `7e450d1` (P5).
- **FLAG-TEMPER** — Landed H-2026-09-12 #391 / `5d4b23c`. `TEMPER_LOOP_ENABLED=true` on `shared/temper.rs` + fabricator honesty.
- **FLAG-PERSONA** — Landed H-2026-09-12 #393 / `c2dd402`. `PERSONA_CREATOR_ENABLED=true` on `shared/persona.rs` + `client/src/title_screen.rs`. `ONLINE_PICKER_ENABLED` and `STEWARD_ONLINE_YES` stay false. Title Online grey.
- **REMAP** — Live on Controls tab (Peace rebind + conflict *taken by…*; `shared/local_settings` keys; soft_play/input). Do not rebuild as a separate Hands card. B4 remaps remain landed.
- **PAUSE-TABS** — Landed H-2026-09-12 #395 / `dc2c156`. Esc pause → Comfort · Controls · Guide tabs on `client/src/title_screen.rs`. Guide one-sentence Peace stranger loop. Online grey.
- **ECONOMY court** — docs [`OFFLINE_ECONOMY_COURT.md`](OFFLINE_ECONOMY_COURT.md) (H-2026-09-12-ECONOMY). Offline teacher banked. Market HOLD.
- **PLACES-DOOR** — Landed H-2026-09-12 #398 / `417a4fa`. Esc Places on tabbed pause plate opens Sanctuary·Heartwood·Threshold·Depths after Settled+book (`hex_travel.rs` + `title_screen.rs`). Does not only dismiss pause. Online grey.
- **COMFORT-PRESETS** — Landed H-2026-09-12 #399 / `b2b6e37`. Esc Comfort Graphics Low · Medium (default) · High; persist beside Grove (`title_screen.rs` + `shared/local_settings.rs`). Online grey.
- **MESH-BUDGET** — docs [`MESH_QUALITY_BUDGET.md`](MESH_QUALITY_BUDGET.md) (H-2026-09-12-MESH-BUDGET). Low/Med/High mesh tiers + first-launch Comfort banner law.
- **MESH-LOD** — Landed H-2026-09-12 #402 / `6b62721`. GraphicsPreset→mesh LOD + Comfort banner.
- **EARTH-CLIMATE** — Landed H-2026-09-12 #403 / `a284cd0b`. PlaceMood weather beds + FlowWeather + WeatherFidelity. Hands QA bank (no PR) may run offline. Do not retag `playable-preview` `11c577e`.
- **PLACES-OVERLAY** — Landed H-2026-09-12 #405 / `01bfdba1`. Comfort banner hide while Places open.
- **PLACES-CLICK** — Landed H-2026-09-12 #406 / `b555386`. Places row opens four-room plate (z+2 · SettingsStub hidden). Places×4 weather re-QA GREEN after PLACES-CLICK (no PR; Online grey; real frames).
- **PLACES-FAT / B6** — Landed H-2026-09-12 #407 / `b33c1a6`. Places deck + Confirm/Back ≥44dp.
- **TODO6** — Landed H-2026-09-12 #408 / `625632f`. Places×4 / Wave-B stamp before Wave C.
- **WAVE-C1** — Landed H-2026-09-12 #409 / `92102b5`. Shared WELL_GLOW_DECAY 0.55 on Peace wells / WeekFeelGlow (`climate_visible.rs` · `skirmish_well.rs`). No harvest_feel rewrite.
- **WAVE-C2** — Landed H-2026-09-12 #410 / `5dacdd3`. Depths quieter bed BED_GAIN_DEPTHS=0.06; Mute kills; no ALSA/cpal (`client/src/peace_audio.rs` · `shared/peace_audio.rs`).
- **WAVE-C3** — Landed H-2026-09-12 #411 / `f319a24`. Heartwood lamp hush BED_GAIN_HEARTWOOD=0.04; Mute; no ALSA (same peace_audio PATHS).
- **WAVE-C4** — Landed H-2026-09-12 #412 / `70f14b3`. First Play boot quiet: welcome_glow_from_line / welcome_glow 0 unless hour_two_welcome_reward; no XP sparkle; particles.rs not wired into lib (`hour_two_resume.rs` · `first_harvest_epiphany.rs`). Wave C C1–C4 closed.
- **TODO7** — Landed H-2026-09-12 #413 / `8961b29`. Wave C C1–C4 tip stamp.
- **PACK-STAMP** — Landed H-2026-09-12 #414 / `4584afa`. AGENT_COMPLETION_PACK tip+§14 Next aligned.
- **TRAILER-BANK** — Landed H-2026-09-12 #415 / `0deee8d`. Imagine Trailer Drive inventory in IMAGINE_TRAILER_PACK (refs only).
- **JOY-LAW** — Landed H-2026-09-12 #416 / `9385e94`. docs/JOY_WITHOUT_MALL.md rarity+social bank.
- **DRIVE-BANK** — Landed H-2026-09-12 #417 / `221fcaf`. docs/DRIVE_POWRUSH_BANK.md Powrush Drive inventory (Buildings mesh-ref not cargo; UI Drive dark).
- **TODO8** — Landed H-2026-09-12 #418 / `11af6da`. Tip stamp TODO7/PACK/TRAILER/JOY/DRIVE; Next Hour-two · MESH-PERSONA · Wave E.
- **Phase B QA** — Banked H-2026-09-13 no-PR @ tip `221fcaf` (client same on main through TODO8). Stranger-loop offline: Title/Play Online grey · Walk/well · E Tend · I inspect · Places×4 · Places plate · Comfort L/M/H = GREEN; R Flow allocate = AMBER (surplus cue reachable; Reserve stayed 0.0 — no banked confirm). Roll-up AMBER · no RED · Hands code HOLD. Reserve-cue fix only on Core CARD (exact PATHS) — do not invent Hands PATHS.
- **TODO9** — Landed H-2026-09-13 #419 / `db05095`. Tip stamp TODO8 `11af6da` + Phase B AMBER bank.
- **Preview QA** — Banked H-2026-09-13 no-PR @ tip `db05095`. Stranger-loop offline + real frames: Title/Play Online grey · Walk/well · E Tend · I inspect · Places×4+plate · Comfort L/M/H = GREEN; R Flow+Reserve = AMBER (surplus/flow ok; Reserve stayed 0.0 after R+2). Roll-up AMBER · no RED · Hands code HOLD. Feel: clarity strong; AAA gap = Reserve banked confirm — Core CARD only (exact PATHS).

Immersion / logistics waves A–E already spent earlier in court (docs + Hands on the lived ladder). Do not reopen.

## Next (ordered)

1. **Hour-two / Steam** — Core/human. Minutes stay blank for the human.
2. **MESH-PERSONA** — optional only with Core ASSET BUDGET + exact PATHS (Buildings Astra Medium sheets are refs in DRIVE_POWRUSH_BANK).
3. **Wave E Offline 1.0 seal** — human preview / Steam copy (≠ retag).

Phase B/Preview AMBER Reserve = Core CARD only (not a Hands freestyle).

Hands HOLD. One card in flight. Title Online stays grey. Floor `2163551`. Tag `11c577e`. Workspace `21.88.0`.

## Refuses this turn

Online freestyle · Market · AH · gold · NFT · drones · Quellorian dump · Cargo · sockets · Always-allow · invent screenshots · Title race select · Sky until online yes · live Earth API · invent Hands PATHS · binary Drive dump · reopen Wave C · Reserve-cue freestyle.
