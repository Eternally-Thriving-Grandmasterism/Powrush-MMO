# GROK_BOT_TODO.md

Tip `600353a` — WAVE-E-SEAL spent on main (#428). This PR is a **design tick**, not a Cargo bump.

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
- **TODO10** — Landed H-2026-09-13 #420 / `f9f8880`. Tip stamp TODO9 + Preview QA AMBER bank.
- **DEMO-BANK** — Landed H-2026-09-13 #421 / `85b461b`. docs/DEMO_PREALPHA_BANK.md YouTube 68GY59nuf8M inventory + KEEP/REFUSE (Places plate · readable names · week-bill death · hub without fake crowd · craft-as-scene later; REFUSE gold/XP/second HUD/skill-tree/class lobby/Market/sockets/Link-in-Chat). Refs only; no binaries.
- **TODO11** — Landed H-2026-09-13 #422 / `089be3b`. Tip stamp DEMO-BANK `85b461b` + DEMO KEEP/REFUSE court.
- **ASSET-BUDGET** — Landed H-2026-09-13 #423 / `5eff19c`. docs/ASSET_BUDGET_COURT.md WoW/SC2-style Comfort L/M/H budget law (readable silhouette · one graphics plate · procedural first · offline fair). MESH-PERSONA still Core CARD+PATHS. Refs only; no binary cargo. Cite only this tick — do not reopen.
- **TODO12** — Landed H-2026-09-13 #424 / `4183c8c`. Tip stamp ASSET-BUDGET `5eff19c` + ASSET-BUDGET court.
- **RESERVE-CUE** — Landed H-2026-09-13 #425 / `bdf7af5`. Named Preview AMBER / Phase B AMBER honesty note: stranger-loop Reserve stayed 0.0 after R+2 (no banked confirm). Docs court stamp only. Fix remains Core CARD + exact PATHS. Hands HOLD. Cite [`ASSET_BUDGET_COURT.md`](ASSET_BUDGET_COURT.md) @ `5eff19c` (do not reopen).
- **HOUR-TWO-STEAM** — Landed H-2026-09-13 #426 / `3e58c48`. Hour-two / Steam = Core/human gate. Minutes / OS / GPU / Time stay blank. No invent minutes. No bot Steam partner. No retag. No Hands mesh. No MESH-PERSONA. Title Online grey. Hands HOLD.
- **UI-MODERNIZE** — Landed H-2026-09-13 #427 / `cc26200`. UI modernize = DEMO-BANK KEEP only (Places plate · readable names · week-bill · hub without fake crowd · craft-as-scene later) as court law for later UI. UI Drive stays dark until Core Viewer share. No Hands UI freestyle. No MESH-PERSONA. No binary. Cite [`ASSET_BUDGET_COURT.md`](ASSET_BUDGET_COURT.md) @ `5eff19c` (do not reopen). MESH-PERSONA remains SKIP (PATHS unknown). Hands HOLD. Title Online grey.
- **WAVE-E-SEAL** — Landed H-2026-09-13 #428 / `600353a`. Wave E Offline 1.0 seal = **human** gate. E1 human preview ticks (≠ retag `11c577e`). E2 Steam copy human (no bot partner). E3 cloud later. E4 γ=0 until named U3.5. No invent ticks. No retag. No Hands mesh this stamp. No UI Drive freestyle. Cite [`ASSET_BUDGET_COURT.md`](ASSET_BUDGET_COURT.md) @ `5eff19c` (do not reopen). Hands HOLD. Title Online grey.
- **MESH-PERSONA** — Landed H-2026-09-13 court CREATE [`MESH_PERSONA_COURT.md`](MESH_PERSONA_COURT.md). Comfort L/M/H dress under ASSET BUDGET · Practices after House · face≠class (no race lobby) · race-looks + Buildings Astra Medium = refs only (X @AlphaProMega race-look posts + Drive `1Bqt2…` already banked). **SKIP lifted** — court bank landed. Hands mesh still Core CARD + exact PATHS. No binary `.glb`. Cite [`ASSET_BUDGET_COURT.md`](ASSET_BUDGET_COURT.md) @ `5eff19c` (do not reopen). Hands HOLD. Title Online grey.

Immersion / logistics waves A–E already spent earlier in court (docs + Hands on the lived ladder). Do not reopen.

## Next (ordered)

1. **MESH-PERSONA Hands mesh** — court bank landed (this CREATE). Hands mesh still Core CARD + exact PATHS. Do not invent PATHS. Cite [`MESH_PERSONA_COURT.md`](MESH_PERSONA_COURT.md) · [`ASSET_BUDGET_COURT.md`](ASSET_BUDGET_COURT.md) @ `5eff19c` (do not reopen). Buildings / race-looks remain refs only.
2. **Hour-two / Steam** — Core/human gate (stamp #426 spent). Minutes / OS / GPU / Time stay blank for the human. Bot does not fill Report. Bot does not create a Steam partner account.
3. **Wave E Offline 1.0 seal** — **human** gate (#428 spent). E1 preview ticks stay blank (≠ retag `playable-preview` / `11c577e`). E2 Steam copy human (no bot partner). E3 cloud later. E4 γ=0 until named U3.5. Bot does not tick. Hands HOLD after merge.

**Reserve-cue** — named Phase B / Preview AMBER gap. Stranger-loop Reserve stayed 0.0 after R+2 (no banked confirm). Fix remains Core CARD + exact PATHS. Hands HOLD. Not a Hands freestyle.
**UI-MODERNIZE** court law: later UI cites DEMO-BANK KEEP only (Places plate · readable names · week-bill · hub without fake crowd · craft-as-scene later). UI Drive dark until Core Viewer share. Hands UI dark until Core CARD + exact PATHS. No Hands UI freestyle. No invent PATHS. MESH-PERSONA court bank landed (Hands mesh still Core CARD + exact PATHS).

Hands HOLD. One card in flight. Title Online stays grey. Floor `2163551`. Tag `11c577e`. Workspace `21.88.0`.

## Refuses this turn

Online freestyle · Market · AH · gold · XP/second HUD · skill-tree as law · NFT · drones · Quellorian dump · Cargo · sockets · Always-allow · invent screenshots · Title race select · race lobby · Sky until online yes · live Earth API · invent Hands PATHS · invent PATHS · OFFER NEXT · binary Drive/demo dump · binary `.glb` dump · reopen Wave C · Comfort Ultra · UI Drive freestyle · Hands UI freestyle · Reserve-cue freestyle · invent minutes · invent ticks · Steam partner from bot · retag · Hands mesh without Core CARD + exact PATHS.
