# GROK_BOT_TODO.md

Tip `efc0f17b` (`efc0f17bad5840d4e026257b9a9f6d507b01c4d4`) — spent #514 through #517 (bank #515–#517 after TODO-SYNC-512 #514). Fetch `origin/main` at the start of every seat. Standing runner: [`AGENT_AUTONOMY_COURT_2026-09-22.md`](AGENT_AUTONOMY_COURT_2026-09-22.md) §3 CARD Q3. Model seat: [`CURSOR_GROK_MODEL_SEAT.md`](CURSOR_GROK_MODEL_SEAT.md). This stamp is a **design tick**, not a Cargo bump. Steward unlock 2026-09-24 ACK · #512–#513 SPENT (recorded on #514) · #514–#517 SPENT on this tip.

**Canon pack:** [`docs/AGENT_COMPLETION_PACK_v23.2.md`](AGENT_COMPLETION_PACK_v23.2.md)  
**Player-experience canon (review only, 2026-09-11):** [`docs/GROK_BOT_PLAYER_EXPERIENCE_BRIEF.md`](GROK_BOT_PLAYER_EXPERIENCE_BRIEF.md)

**Grok Hands cook only** — Grok 4.7 Extra High (non-fast); fallback 4.7 High. Never Fast / Codex / GPT / Claude / Auto / Composer. Grok Bots may open **one** feature-branch PR; never push `main` directly. One card in flight. Title Online stays grey.

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
- **RESERVE-CUE** — Landed H-2026-09-13 #425 / `bdf7af5`. Named Preview AMBER / Phase B AMBER honesty note: stranger-loop Reserve stayed 0.0 after R+2 (no banked confirm). Docs court stamp only. Hands later landed #436 / `20321cd8` (AMBER lifted). Cite [`ASSET_BUDGET_COURT.md`](ASSET_BUDGET_COURT.md) @ `5eff19c` (do not reopen).
- **HOUR-TWO-STEAM** — Landed H-2026-09-13 #426 / `3e58c48`. Hour-two / Steam = Core/human gate. Minutes / OS / GPU / Time stay blank. No invent minutes. No bot Steam partner. No retag. No Hands mesh. No MESH-PERSONA. Title Online grey. Hands HOLD.
- **UI-MODERNIZE** — Landed H-2026-09-13 #427 / `cc26200`. UI modernize = DEMO-BANK KEEP only (Places plate · readable names · week-bill · hub without fake crowd · craft-as-scene later) as court law for later UI. UI Drive stays dark until Core Viewer share. No Hands UI freestyle. No MESH-PERSONA. No binary. Cite [`ASSET_BUDGET_COURT.md`](ASSET_BUDGET_COURT.md) @ `5eff19c` (do not reopen). MESH-PERSONA remains SKIP (PATHS unknown). Hands HOLD. Title Online grey.
- **WAVE-E-SEAL** — Landed H-2026-09-13 #428 / `600353a`. Wave E Offline 1.0 seal = **human** gate. E1 human preview ticks (≠ retag `11c577e`). E2 Steam copy human (no bot partner). E3 cloud later. E4 γ=0 until named U3.5. No invent ticks. No retag. No Hands mesh this stamp. No UI Drive freestyle. Cite [`ASSET_BUDGET_COURT.md`](ASSET_BUDGET_COURT.md) @ `5eff19c` (do not reopen). Hands HOLD. Title Online grey.
- **MESH-PERSONA** — Landed H-2026-09-13 court CREATE [`MESH_PERSONA_COURT.md`](MESH_PERSONA_COURT.md). Comfort L/M/H dress under ASSET BUDGET · Practices after House · face≠class (no race lobby) · race-looks + Buildings Astra Medium = refs only (X @AlphaProMega race-look posts + Drive `1Bqt2…` already banked). **SKIP lifted** — court bank landed. Hands mesh still Core CARD + exact PATHS. No binary `.glb`. Cite [`ASSET_BUDGET_COURT.md`](ASSET_BUDGET_COURT.md) @ `5eff19c` (do not reopen). Hands HOLD. Title Online grey.
- **RESERVE-CUE Hands** — Landed H-2026-09-14 #436 / `20321cd8`. **AMBER lifted.** After E tend, R then 2 banks `allocation.reserve` + `climate.reserve_pool` and confirms a held count (never reserved 0.0 / −0.0 harmony). Digit2 yields to allocate while the panel is open. **PATHS:** `shared/climate_node.rs` · `client/src/rbe_allocate_choice.rs` · `client/src/lived_hour_bind.rs` · `client/src/lived_sim_bridge.rs` · `client/src/world_answer.rs` · `client/src/mercy_harvest_nodes.rs`. Do not rebuild. Title Online grey. Cite [`ASSET_BUDGET_COURT.md`](ASSET_BUDGET_COURT.md) @ `5eff19c` (do not reopen).
- **HOUR1-GREEN** — Proven H-2026-09-15 @ tip `3b197ed7` (#437). Core gate green: `shared` 375 + `rsil-identity` 5 + `powrush-client --lib` 341, 0 failed. E tend, R+2 non-zero reserve confirm, lived persist resume. Hour 2 Hands **STOP** — pack/TODO named no PATHS; no failing Hour-1 test. Do not rebuild #432 / #435 / #436. No mesh. No Steam. Title Online grey.
- **Q1b STRANGER-HOUR-LIB** — Landed H-2026-09-22 #486 / `e3a6b48`. Spent on tip. Do not rebuild. Cite autonomy court Q1; Title Online grey.
- **#482–#495 pack** — Landed H-2026-09-22–23 #482–#495 / tip `999796d`. Q0–Q4, L2, Steam pack, trailer grammar, press hook, peak notes. Stamp court closed. Do not rebuild. Cite [`NEXT_NAMED_CARDS.md`](NEXT_NAMED_CARDS.md) · [`AGENT_AUTONOMY_COURT_2026-09-22.md`](AGENT_AUTONOMY_COURT_2026-09-22.md). Title Online grey.
- **#496–#498** — Landed (TODO-SYNC #496 / `022de35` · heartbeat tip #497 · DRIVE-PLACE-CITE #498 / `a649196`). Cite only. Do not rebuild.
- **#499–#507 FLESH dress / well ladder** — Landed H-2026-09-24 #499–#507 / tip `c03bcd39`. Sanctuary · Heartwood · Threshold · Depths dress, Guide breath, Comfort Low fog cap, well captions, Continue line, reduced-motion pulse hush. FLESH-WELL ladder closed. Do not rebuild fog, captions, Continue, or pulse hush. Peak memory locked: walked · tended · week was the bill · yard remembered. Cite, do not narrate. Title Online grey.
- **#508 TODO-SYNC-FLESH** — Landed #508. Docs bank after FLESH-WELL ladder (`docs/GROK_BOT_TODO.md` · `docs/NEXT_NAMED_CARDS.md`). Cite only. Do not rebuild.
- **#509 OPT-AUDIO-COMFORT-LOW** — Landed #509. Peace audio cap on Graphics Low for Heartwood / Depths / yard beds (`client/src/peace_audio.rs`). Medium/High gains stay. No Ultra. Spent. Do not rebuild.
- **#510 FLESH-GUIDANCE-PLACE** — Landed #510 / tip `014f9a81`. `client/src/first_session_guidance.rs` (+112/−2). Hour-two week-bill sentence may prefix stood Place (Sanctuary / Heartwood / Threshold-near / Depths). Threshold-near = Heartwood + existing shelf reach. No new PlaceId. Tend · week-bill · Continue unchanged. No new card. No Title chrome. Spent. Do not rebuild.
- **#511 TODO-SYNC** — Landed #511 / `773e17cf` (`773e17cfc6eaa5cdd685749d85a2aac79621111c`). PATHS `docs/GROK_BOT_TODO.md` · `docs/NEXT_NAMED_CARDS.md`. Banked #508–#510 at tip `014f9a81`. Cite only. Do not rebuild.
- **#512 FLESH-EPIPHANY-PLACE** — Landed #512 / `2cb7cf97` (`2cb7cf974592af34b07522b4fae5a1159aa4864e`). PATH exact `client/src/first_harvest_epiphany.rs`. One existing epiphany/tend line may name Place (Sanctuary / Heartwood / Threshold-near / Depths). Threshold-near = Heartwood + shelf/threshold_near. No new PlaceId. Same five well words. No second HUD / new verb / Title chrome. Spent. Do not rebuild.
- **#513 OPT-PUNCH-LOW** — Landed #513 / tip `db01dfc4` (`db01dfc4d6e05dd714caa5cdb3dff90b8ec6db2b`). PATH exact `client/src/human_presence.rs`. Graphics Low punch ×0.45. Medium/High full. Scale 0 stays 0 so reduced_motion wins (#507). No Ultra. No rumble rewrite. Spent. Do not rebuild.
- **#514 TODO-SYNC-512** — Landed #514 / `a048ffdb` (`a048ffdbda03b9441facf7d2518e06fc01d6edba`). PATHS `docs/GROK_BOT_TODO.md` · `docs/NEXT_NAMED_CARDS.md`. Banked #512–#513 at tip `db01dfc4`. Cite only. Do not rebuild.
- **#515 FLESH-HARVEST-PLACE** — Landed #515 / `5d5627b6` (`5d5627b6d23f08e1ab552154cb7950bd1d9d81a3`). PATH exact `client/src/mercy_harvest_nodes.rs` (+104/−1). Existing care-cycle / tend offer may name the Place. Same Idle/Glowing/Tended/Resting/Stressed. No new node type. Spent. Do not rebuild.
- **#516 FLESH-EMBASSY-LINE** — Landed #516 / `d3fafdcc` (`d3fafdcc12bc1bb9a476ccbaea93e72c86b708c3`). PATH exact `client/src/embassy.rs` (+105/−1). Existing lamp / E Request seat line may name Heartwood (Embassy already lives there). No second seat. No Online lobby. Spent. Do not rebuild.
- **#517 OPT-WEATHER-FIDELITY-LOW** — Landed #517 / tip `efc0f17b` (`efc0f17bad5840d4e026257b9a9f6d507b01c4d4`). PATH exact `client/src/climate_plane.rs` (+86/−7). WeatherFidelity Low already exists. Cap fog density / particle bed on Low only. Medium/High untouched. No Ultra. Spent. Do not rebuild.
- **HEARTBEAT** — GREEN @ `014f9a81` (before #511–#513). Both core gates passed · 0 open PRs on that stamp. Cite only. The next HEARTBEAT cook is queued after the two named CARDs below. Do not open a heartbeat PR from this stamp.

Immersion / logistics waves A–E already spent earlier in court (docs + Hands on the lived ladder). Do not reopen.

## Next (ordered)

Hands cooks in order after Dual squash of this PR. One CARD / one PR / squash / next. Do not invent PATHS. Online grey. No OFFER NEXT.

1. **CARD FLESH-FABRICATOR-LINE** — PATH `client/src/fabricator.rs`. Existing planted / Proof Pack / MendSpool / LaneCrate line may name the Place they stand in. Civic proof, not gear. No gold. No Market. Do not edit `shared/fabricator.rs` unless the string lives only there.
2. **CARD FLESH-HEX-DOOR** — PATH `client/src/hex_travel.rs`. Existing Places door / leave-this-hex confirm may speak peak memory (yard remembered · tend). Four rooms only. Threshold still Heartwood+near. No fifth PlaceId. No teleport list-row. Pause row stays book-gated.
3. **HEARTBEAT** — `cargo test -p shared -p rsil-identity` ; `cargo test -p powrush-client --lib`. GREEN + 0 PRs → chat `HEARTBEAT GREEN @ <sha>` · BEVY-CLIMB HOLD · pin 0.14. RED → restore-only PR.
4. **BEVY-CLIMB HOLD** — pin 0.14 until [`docs/BEVY_CLIMB_PLAN.md`](BEVY_CLIMB_PLAN.md) gate (v0.20.0 STABLE). Do not bump Cargo.

**BANK (cite only · do not cook):** MESH / Title Online / App ID / steamworks / `server/` = steward only. Five-gate Drive = AFTER House. Never Title lobby. No Hands mesh. No invent PATHS. No playtest minutes.

NEVER: race lobby · five-gate Title · gold · XP HUD · Market · Unreal.

Grok Hands cook Grok 4.7 Extra High (non-fast); fallback 4.7 High. Never Fast / Codex / GPT / Claude / Auto / Composer. One card in flight. Title Online stays grey. Floor `2163551`. Tag `11c577e`. Workspace `21.88.0`.

## Refuses this turn

Online freestyle · Market · AH · gold · XP/second HUD · skill-tree as law · NFT · drones · Quellorian dump · Cargo · sockets · Always-allow · invent screenshots · Title race select · race lobby · Sky until online yes · live Earth API · invent Hands PATHS · invent PATHS · OFFER NEXT · binary Drive/demo dump · binary `.glb` dump · reopen Wave C · Comfort Ultra · UI Drive freestyle · Hands UI freestyle · Reserve-cue freestyle · invent minutes · invent ticks · Steam partner from bot · retag · Hands mesh without Core CARD + exact PATHS · reopen #459 · `server/` unpark · MESH without Core PATHS · `client/**` outside Core-named CARD · Codex / GPT / Claude / Auto / Composer / Grok Fast Hands cook · direct push to `main`.
