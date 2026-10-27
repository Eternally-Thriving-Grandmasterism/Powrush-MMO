# PLAYTEST-UI-BENCH-1: HUD / UI / onboarding benchmark

Docs only. No client, shared, host, divine-module, Cargo or CI change. Online stays grey.

Base: main `acd16717` (after #776 PLAYTEST-CAM-1 and #777).

## 0. Ground rules for this doc

- Every claim about a reference game has an inline link. Every link was fetched on 2026-10-26 (America/Toronto) and resolved. The full list is in [SOURCES.md](SOURCES.md).
- **UNVERIFIED** marks anything we could not confirm from a source we could open. We did not fill gaps from memory.
- These are patterns only, written in our own words. No art, assets or quoted text were copied.
- Powrush slab names come from [`docs/visual-pass/HUD_SLAB_MAP.md`](../HUD_SLAB_MAP.md) ("the map", built at main `a51f7ee5`). There is no `docs/HUD_SLAB_MAP.md` at the docs root. The map is the only copy, and [`docs/design/UI_LAYOUT_SYSTEM.md`](../../design/UI_LAYOUT_SYSTEM.md) cites it too. We added no slabs.
- Facts about Powrush from the map:
  - There is **no action-bar slab, no target or unit-frame slab, no minimap slab and no tooltip slab** in the loaded client. `ability_bar.rs` and `onboarding_ui.rs` exist but are not declared in `lib.rs`, so they never spawn (map, section 1).
  - The nearest existing slabs are the ones used in the tables below.

Verdict key: **adopt** = take the pattern as is. **adapt-Powrush-style** = keep the idea but fit it to Powrush's tone and care/abundance loop (no combat-first framing). **skip** = do not take it.

## 1. Action bars and cooldowns

What the references do:
- **FFXIV.** Hotbars hold actions, items, emotes and macros. A lock stops icons being dragged off by accident, and the layout is saved per class ([game manual](https://na.finalfantasyxiv.com/game_manual/view/)). The bar's shape can be changed ([hotbar shape](https://na.finalfantasyxiv.com/uiguide/know/know-hb/hotbar_shape.html)). The recast (cooldown) timer can be moved from a small corner position to a larger centred one ([recast timer](https://na.finalfantasyxiv.com/uiguide/know/know-hb/hotbar_recasttimes.html)).
- **RuneScape 3.** An optional cooldown countdown sits on each ability, either centred or top-right. Ability queueing lets you press an ability that is still cooling down, and it fires once ready ([RS Wiki: Action Bar settings](https://runescape.wiki/w/Settings/Combat_%26_Action_Bar/Action_Bar)). The action bar can be minimised but not closed, and up to 5 bars can show ([RS Wiki: Interface](https://runescape.wiki/w/Interface)).
- **WoW.** Action Bars 1-8 can each be moved in Edit Mode ([Warcraft Wiki: Edit Mode](https://warcraft.wiki.gg/wiki/Edit_Mode)). Dragonflight added "Press and Hold Casting", which repeats a cast while a key is held ([Blizzard: Accessibility Updates in Dragonflight](https://news.blizzard.com/en-us/article/23876525/accessibility-updates-in-dragonflight)).
- **ESO.** By default five skill slots sit on 1-5 and the ultimate on R. All keys can be remapped ([UESP: Controls](https://en.uesp.net/wiki/Online:Controls)).
- GW2, Albion, Diablo IV, Valheim: we did not verify their action-bar or cooldown details this pass (**UNVERIFIED**).

| pattern | nearest Powrush slab | verdict | reason |
|---|---|---|---|
| Large, readable cooldown number on the slot (FFXIV centred recast, RS3 centred timer) | none loaded (`ability_bar.rs` is not declared). The verbs live in `LivingPracticeStrip` / `FirstSessionGuidanceStrip` | adapt-Powrush-style | Powrush has few verbs. Show a "ready in N" readout on the practice strip, not a 12-slot bar. |
| Bar lock against accidental drag (FFXIV) | n/a | skip (for now) | Nothing is draggable yet. Revisit if a bar ships. |
| Input queueing for an action on cooldown (RS3) | `LivingPracticeStrip` | adopt | It cuts the "pressed too early, nothing happened" friction, and it is invisible to new players. |
| Press-and-hold repeat (WoW) | `WorldCarePromptRoot` / `CareCycleStrip` (Tend/Flow verbs) | adopt | It eases repetitive tend actions and is an accessibility win. |

## 2. Target / unit frames

- **FFXIV.** A target bar shows what is targeted, with a second bar for what that target is engaging. There is a separate focus-target bar, with a keybind to set the focus target and another to swap to it ([game manual](https://na.finalfantasyxiv.com/game_manual/view/)).
- **RS3.** A target-info panel floats above the target, showing name, weakness icon, colour-coded level, life bar and status icons. It can be unlocked and moved. A coloured ring under the feet shows the current target and anything attacking you ([RS Wiki: Interface](https://runescape.wiki/w/Interface)).
- **WoW.** Player, Target and Focus frames are all movable in Edit Mode ([Edit Mode](https://warcraft.wiki.gg/wiki/Edit_Mode)). "Action Targeting" auto-targets enemies you face, and a minimum character-name size keeps in-world names readable, fading them with distance instead of shrinking them ([Dragonflight accessibility](https://news.blizzard.com/en-us/article/23876525/accessibility-updates-in-dragonflight)).
- **Albion (mobile, Queen Patch 11).** A glow plus an arrow mark the selected target ([changelog](https://albiononline.com/changelog/QUEEN%20PATCH%2011)).
- **GW2.** Action Camera has a crosshair with four states (no target, target out of range, in range of some skill, in range of skill 1) and right-click target lock ([GW2: Introducing the Action Camera](https://www.guildwars2.com/en/news/introducing-the-action-camera/)).
- **Diablo IV.** Players can turn on outlines for players, enemies, objects and NPCs, each in a colour they choose ([Blizzard: Combatting Demons with Accessibility](https://news.blizzard.com/en-us/article/23954932/combatting-demons-with-accessibility-in-diablo-iv)).

| pattern | nearest Powrush slab | verdict | reason |
|---|---|---|---|
| Full target/focus unit frame (FFXIV, WoW) | none | skip | Powrush is not tab-target combat. A frame would add a combat-first slab with nothing to fill it. |
| In-world marker on the thing you will interact with (Albion glow + arrow, WoW interact icon) | `WorldCarePromptRoot` (care prompt) + world-space marker | adapt-Powrush-style | Show which node a Tend will hit, using a soft world ring, not a HUD frame. |
| Crosshair states that tell you whether an action will land (GW2) | none (world/camera) | adapt-Powrush-style | Use a two-state "in reach / out of reach" cue for care nodes. This fits mouse-look ON. |
| Player-chosen highlight colours (D4) | none | adopt (as an option) | This is a colourblind and low-vision win. See follow-up PLAYTEST-UI-OPTIONS-1. |
| Minimum name size with distance fade (WoW) | `ClimateNameRoot`, `PeerPresenceRoot` | adopt | It is a cheap readability floor for place and peer names. |

## 3. Quest / objective tracker

- **FFXIV.** The Duty List shows active quests and their objectives on the HUD. Clicking a title opens the journal, and clicking an objective shows it on the map. A separate Main Scenario Guide points to the next story quest and can be switched off ([game manual](https://na.finalfantasyxiv.com/game_manual/view/)). The UI guide also has entries for hiding the Duty List during some content ([UI guide index](https://na.finalfantasyxiv.com/uiguide/know/)).
- **WoW.** The Objective Tracker is one of the Edit Mode movable frames ([Edit Mode](https://warcraft.wiki.gg/wiki/Edit_Mode)). Blizzard listed "Quest Tracker" as planned further HUD work ([Dragonflight HUD and UI Revamp](https://news.blizzard.com/en-us/article/23841481/world-of-warcraft-dragonflight-hud-and-ui-revamp)).
- **ESO.** One key cycles the tracked active quest, and another opens the Journal ([UESP: Controls](https://en.uesp.net/wiki/Online:Controls)).
- **Diablo IV (Vessel of Hatred).** Auto-Pin drops a map pin on the selected quest and draws a route on the minimap ([Blizzard: A Crucible for All](https://news.blizzard.com/en-us/article/24139219/a-crucible-for-allaccessibility-features-in-vessel-of-hatred)).
- **Albion (mobile).** A compact panel switches between party and quest, and there is a compact Destiny Board tracker ([changelog](https://albiononline.com/changelog/QUEEN%20PATCH%2011)).
- **OSRS.** The quest list can be sorted and filtered, with a text-size choice and custom colours for each status ([OSRS Wiki: Settings](https://oldschool.runescape.wiki/w/Settings)). In RS3 the quest log uses red, yellow and green for not started, in progress and complete ([RS Wiki: Interface](https://runescape.wiki/w/Interface)).

| pattern | nearest Powrush slab | verdict | reason |
|---|---|---|---|
| One always-visible "next step" line (FFXIV Main Scenario Guide) | `FirstSessionGuidanceStrip` | adopt | It already exists in spirit. Keep it as one line with one next step. |
| Click/press an objective to see it on the map (FFXIV) | `JourneyEchoRoot` (J echo), `PlacesRoot` | adapt-Powrush-style | Link journal entries to a Places or compass pin, without a full world map. |
| Cycle the tracked goal with one key (ESO) | `JourneyEchoRoot` | adopt | It is cheap, keyboard-friendly and needs no new slab. |
| Auto-pin the selected goal and route the compass to it (D4) | `CompassSlabRoot` | adapt-Powrush-style | The compass slab exists. Point it at the active Journey goal. |
| Status shown by colour alone (OSRS/RS3 red/yellow/green) | `JourneyEchoRoot` | adapt-Powrush-style | Pair each colour with an icon or word, because colour alone fails colourblind players ([GAG](https://gameaccessibilityguidelines.com/ensure-no-essential-information-is-conveyed-by-a-colour-alone/)). |

## 4. Minimap and compass

- **FFXIV.** The minimap shows nearby landmarks and quest spots, red dots for enemies and blue dots for allies, and a time-of-day marker on its rim. Clicking it opens the full map ([game manual](https://na.finalfantasyxiv.com/game_manual/view/)).
- **WoW.** The minimap is movable, and Edit Mode can resize it and turn on rotation ([Edit Mode](https://warcraft.wiki.gg/wiki/Edit_Mode)). The Dragonflight revamp made the minimap and health bar "more prominent" and removed clutter ([HUD and UI Revamp](https://news.blizzard.com/en-us/article/23841481/world-of-warcraft-dragonflight-hud-and-ui-revamp)).
- **RS3.** The minimap can be square or circular. A compass on it shows camera facing and snaps the camera north when clicked ([RS Wiki: Interface](https://runescape.wiki/w/Interface)).
- **OSRS.** There is a minimap zoom level, scroll-to-zoom, and an option to hide icons when zoomed out ([OSRS Wiki: Settings](https://oldschool.runescape.wiki/w/Settings)).
- **Albion (mobile).** The minimap can be hidden with a long press. The player icon was enlarged, and road colours in steppe zones were changed for readability ([changelog](https://albiononline.com/changelog/QUEEN%20PATCH%2011)).
- **Diablo IV.** An optional compass ring around the player points at the placed pin, and the player picks the arrow colour. Optional spatial-audio pings also lead to the pin ([A Crucible for All](https://news.blizzard.com/en-us/article/24139219/a-crucible-for-allaccessibility-features-in-vessel-of-hatred)).
- ESO's on-screen compass bar: our UESP page for it returned 404 (**UNVERIFIED**, dropped).

| pattern | nearest Powrush slab | verdict | reason |
|---|---|---|---|
| Full minimap with enemy/ally dots (FFXIV) | none | skip (for now) | It adds a corner slab and combat framing. The map shows 27 HUD slabs already competing for space (UI_LAYOUT_SYSTEM section 1). |
| Compass that points at the chosen goal, with player-picked colour (D4) | `CompassSlabRoot` | adopt | The slab exists. Give it a goal arrow, and a colour option in UI-OPTIONS-1. |
| Audio ping toward the goal (D4) | none (audio) | adapt-Powrush-style | Use a soft care-tone ping. This is accessibility for low-vision players. |
| Click the compass to snap the camera to a cardinal direction (RS3) | `CompassSlabRoot` | adopt | It is a one-line orientation help for new players with mouse-look ON. |
| Hide the map/compass with one gesture (Albion) | `CompassSlabRoot` | adopt | It respects players who want a clean screen. |

## 5. Element / item tooltips

- **OSRS.** It has toggles for mouseover tooltips, "+N options" in tooltips, prayer and special-attack tooltips, and remaining XP in the stats tooltip ([OSRS Wiki: Settings](https://oldschool.runescape.wiki/w/Settings)).
- **RS3.** Tooltips were reworked so they no longer block buttons. They gained "+ x options" for elements with several right-click actions. Item tooltips show charges, passive effects and level requirements ([RS Wiki: Interface, update history](https://runescape.wiki/w/Interface)).
- **WoW.** "HUD Tooltip" is a movable element in Edit Mode advanced options ([Edit Mode](https://warcraft.wiki.gg/wiki/Edit_Mode)).
- **Diablo IV.** A built-in screen reader reads menu text, including gear and shop items ([Combatting Demons with Accessibility](https://news.blizzard.com/en-us/article/23954932/combatting-demons-with-accessibility-in-diablo-iv)).
- FFXIV/GW2/ESO/Albion/Valheim tooltip specifics: **UNVERIFIED** this pass.

| pattern | nearest Powrush slab | verdict | reason |
|---|---|---|---|
| Tooltip on satchel items: what it is, what it is for, what it unlocks | `SatchelRoot`, `PickupFlashRoot` | adopt | This is the core readability gap for the first hour. |
| "+N more actions" hint (OSRS/RS3) | `SatchelRoot`, `WorldCarePromptRoot` | adopt | It tells players a context menu exists, and it is copy-only. |
| Tooltips must never cover the button you are aiming at (RS3 fix) | all | adopt | This is a layout rule for HUD-LAYOUT-1. |
| Movable tooltip anchor (WoW) | n/a | skip | It is power-user only. A fixed rule beats a setting. |
| Screen-reader text for menus (D4) | `SettingsStubRoot`, `SatchelRoot` | adapt-Powrush-style | It is a long-term accessibility goal and too big for this pass. Note it in UI-OPTIONS-1. |

## 6. First-hour tutorial hints

- **Valheim.** The raven Hugin gives short hints that fire on events: picking up items, crafting a hammer, hoe or pickaxe, getting cold, being overweight, dying, entering a dangerous biome ([Valheim DB: Hugin & Munin](https://www.valheimdatabase.com/hugin-munin/)). This is a third-party database, not an official source.
- **RS3.** On arrival the interface is only the action bar and an options ribbon. Other panels are added as the tutorial goes on ([RS Wiki: Interface](https://runescape.wiki/w/Interface)). UI tutorials for the bank, shop and similar screens moved to play only when the player asks via a help button. Players who say they have played recently get guidance hints switched off automatically (RS Wiki: Interface, update history, 16 Aug 2021 and 18 Mar 2020).
- **OSRS.** Since 26 Mar 2025, settings are adjusted from the player's answer on Tutorial Island: Beginner, Returning or Experienced ([OSRS Wiki: Settings](https://oldschool.runescape.wiki/w/Settings)).
- **FFXIV.** "Active Help" keeps a list of every tutorial already seen, so it can be re-read ([game manual](https://na.finalfantasyxiv.com/game_manual/view/)).
- **WoW.** Edit Mode is usually turned off before level 10 on the starting island (Exile's Reach) ([Edit Mode](https://warcraft.wiki.gg/wiki/Edit_Mode)).
- **Albion (mobile).** It added intro tutorial screens for the joystick control scheme ([changelog](https://albiononline.com/changelog/QUEEN%20PATCH%2011)).
- GW2 hearts and onboarding: the GW2 wiki was down for maintenance when fetched (**UNVERIFIED**, dropped).

| pattern | nearest Powrush slab | verdict | reason |
|---|---|---|---|
| Event-triggered one-shot hints (Valheim) | `FirstSessionGuidanceStrip`, `WhisperRoot` | adopt | Powrush already has guidance and a whisper. Fire hints on the first harvest, first Tend and first well. |
| Reveal HUD slabs gradually (RS3) | all class-5 slabs (see UI_LAYOUT_SYSTEM) | adopt | It lowers first-minute clutter and matches existing `when it shows` gating in the map. |
| Ask "new / returning / experienced" once and tune hints (OSRS, RS3) | `TitleRoot` flow / `SettingsStubRoot` | adapt-Powrush-style | Ask one question in the title or name flow, with no quiz. |
| Re-readable hint log (FFXIV Active Help) | `JourneyEchoRoot` | adopt | Players miss hints, so let them re-read. |
| Lock layout editing in the first hour (WoW) | n/a | adopt (when a layout editor exists) | It prevents a broken HUD before players know the defaults. |

## 7. Input settings: sensitivity, raw input, smoothing, keybinds

- **WoW.** Raw mouse input is an option for mouselook only, with no effect on the cursor. It reads the device directly, bypassing OS acceleration. A community thread notes most players turn acceleration off in games ([WoWInterface: rawMouseEnable](https://www.wowinterface.com/forums/showthread.php?t=58836)). This is a forum, not an official source. WoW's default value for raw input: **UNVERIFIED**.
- **GW2.** Action Camera keeps mouse-look always on while it is enabled. It turns off automatically in dialogue and menus, and it has its own toggle keybind ([Introducing the Action Camera](https://www.guildwars2.com/en/news/introducing-the-action-camera/)). Before Action Camera, players held the right mouse button to mouse-look (same source). A 2015 camera update added FOV, horizontal and vertical offset, collision sensitivity, zoom sensitivity and a camera-only "Restore Defaults" ([Get Ready: Camera Changes](https://www.guildwars2.com/en/news/get-ready-camera-changes-are-coming/)).
- **ESO.** The mouse is the look/turn control by default (mouse-look is the default movement model), and all controls can be remapped ([UESP: Controls](https://en.uesp.net/wiki/Online:Controls)).
- **RS3.** Its Controls tab covers keybinds and camera sensitivity. Most keybinds are rebindable with one modifier, and rebinding a key already in use unbinds the old action with a warning ([RS Wiki: Interface](https://runescape.wiki/w/Interface)).
- **OSRS.** It has options such as middle-mouse camera control, single-button mode and scroll-to-zoom ([OSRS Wiki: Settings](https://oldschool.runescape.wiki/w/Settings)).
- **FFXIV.** It has a dedicated Keybind menu and Configuration Sharing to copy settings between characters ([game manual](https://na.finalfantasyxiv.com/game_manual/view/)).
- **Diablo IV.** Full remapping on keyboard/mouse and controller, plus toggles in place of hold ([Can I Play That? preview](https://caniplaythat.com/2023/05/23/diablo-iv-accessibility-detailed-by-blizzard/)). Cursor size is small, medium or large ([Combatting Demons with Accessibility](https://news.blizzard.com/en-us/article/23954932/combatting-demons-with-accessibility-in-diablo-iv)).
- **Albion.** Quick Settings Presets save graphics, audio and control settings, and a keybind can switch between them ([Dev Talk: UI Improvements](https://albiononline.com/news/dev-talk-horizons-ui)).
- Guidance: [GAG: allow controls to be remapped](https://gameaccessibilityguidelines.com/allow-controls-to-be-remapped-reconfigured/), [GAG: adjustable sensitivity](https://gameaccessibilityguidelines.com/include-an-option-to-adjust-the-sensitivity-of-controls/).
- Camera follow-smoothing defaults in any reference game: **UNVERIFIED**. No source we opened states a default smoothing value.

| pattern | nearest Powrush slab | verdict | reason |
|---|---|---|---|
| Raw input as an opt-in for mouse-look only (WoW) | `SettingsStubRoot` | adopt (already shipped: `raw_look`, #776) | It matches. See section 9. |
| Mouse-look on by default, with auto-release in menus/dialogue (GW2 Action Camera, ESO) | `SettingsStubRoot` + any modal (`PlacesRoot`, `SatchelRoot`, `MercySoftRoot`) | adopt | The default is on (#776). Confirm the cursor frees itself whenever a modal opens. |
| Sensitivity slider + invert Y | `SettingsStubRoot` | adopt | It is standard, and the GAG guideline asks for it. |
| Full keybind remap with conflict warning (RS3) | `SettingsStubRoot` | adopt | Remapping is an accessibility baseline. |
| Camera-only "Restore Defaults" (GW2) | `SettingsStubRoot` | adopt | It is a safety net for new raw/look toggles. |
| Settings presets switchable by key (Albion) | n/a | skip (for now) | Too few settings yet to justify it. |

## 8. UI scale, colourblind and readability

- **WoW.** Edit Mode layouts can be named, saved, copied and shared, and a layout is remembered per spec ([HUD and UI Revamp](https://news.blizzard.com/en-us/article/23841481/world-of-warcraft-dragonflight-hud-and-ui-revamp)). There is an FOV option for motion sickness, a minimum name size, "Reduce Camera Motion" and "Alternate Full Screen Effects" ([Dragonflight accessibility](https://news.blizzard.com/en-us/article/23876525/accessibility-updates-in-dragonflight)).
- **FFXIV.** HUD Layout mode drags, resizes, hides and sets transparency per element, and layouts can be copied between slots ([HUD layout step by step](https://na.finalfantasyxiv.com/uiguide/know/know-hud/hud-layout.html)).
- **Albion (Horizons).** Its HUD Editor sets position, scale and opacity of nearly every element, with export and import of layouts ([Dev Talk: UI Improvements](https://albiononline.com/news/dev-talk-horizons-ui)).
- **ESO.** A "Custom Scale" slider under Video > Interface scales both fonts and UI ([ESO Support: font size](https://help.elderscrollsonline.com/app/answers/detail/a_id/28891/~/how-do-i-increase-the-font-size-in-game%3F)).
- **OSRS.** It has an interface scaling slider (Steam client, resizable mode) and a toggle that swaps bright colour fades for black ([OSRS Wiki: Settings](https://oldschool.runescape.wiki/w/Settings)).
- **RS3.** It has a transparency slider, an Accessibility tab, and chat font sizes 10-22. Its adrenaline bar colour was changed to help colourblind players ([RS Wiki: Interface](https://runescape.wiki/w/Interface)).
- **Diablo IV.** Over 50 accessibility features. Text comes in 3 sizes, with subtitles on by default and adjustable in colour, size and background ([Combatting Demons with Accessibility](https://news.blizzard.com/en-us/article/23954932/combatting-demons-with-accessibility-in-diablo-iv)). There are colourblind filters for protanopia, deuteranopia and tritanopia with an intensity slider. The reviewer found the highlight option more useful than the filters ([Can I Play That? review](https://caniplaythat.com/2023/07/18/diablo-iv-accessibility-review/)).
- WoW colourblind filter details: Blizzard's support article would not render for our fetcher (**UNVERIFIED**, not cited).
- Guidance: [GAG: allow interfaces to be rearranged](https://gameaccessibilityguidelines.com/allow-interfaces-to-be-rearranged/), [GAG: no info by colour alone](https://gameaccessibilityguidelines.com/ensure-no-essential-information-is-conveyed-by-a-colour-alone/).

| pattern | nearest Powrush slab | verdict | reason |
|---|---|---|---|
| One global UI scale slider (ESO, OSRS) | all slabs; setting in `SettingsStubRoot` | adopt | It is the biggest readability win per line of code. It needs the registry (UI_LAYOUT_SYSTEM) so slabs do not collide when scaled. |
| Text size S/M/L (D4) | all text slabs | adopt | Simpler than a free slider for text. |
| Per-element drag/resize editor (WoW, FFXIV, Albion) | all | adapt-Powrush-style | Start with the presets already proposed in UI_LAYOUT_SYSTEM (classic / minimal / management). A free editor comes later, if ever. |
| Colourblind full-screen filter (D4) | n/a | skip | The reviewer found filters doubtful. Prefer the next row. |
| Player-picked highlight and accent colours + never colour-only (D4, GAG) | `CompassSlabRoot`, `CareCycleStrip`, `WatchStripRoot` | adopt | It works for all colourblind types. |
| Reduce camera motion / bright flashes (WoW, OSRS) | `ThrivingToastRoot`, `HarvestPulseRoot`, VFX | adopt | Comfort option. It pairs with VP-VFX caps. |
| Opacity per slab (FFXIV, Albion, RS3) | all class-5 slabs | adapt-Powrush-style | Use one global HUD opacity first. |

## 9. CAM-1 check: mouse-look ON + optional raw 1:1 look (#776)

What shipped, from `docs/visual-pass/playtest-cam-1/README.md` and the #776 diff: `mouse_look` defaults on. `raw_look` defaults off. When `raw_look` is on, the follow-lerp smoothing (`1-(1-0.12)^(dt*60)`) is skipped, so look is 1:1. Orbit input was already raw per-pixel, and the pitch clamp is kept.

Compared with the references:
- **Mouse-look on by default.** ESO uses the mouse for look/turn by default ([UESP](https://en.uesp.net/wiki/Online:Controls)). GW2 found players holding right-mouse all the time and shipped Action Camera so mouse-look stays on ([GW2](https://www.guildwars2.com/en/news/introducing-the-action-camera/)). GW2 Action Camera is a toggle, not GW2's default; we could not verify its default state (**UNVERIFIED**). **Verdict: Powrush matches the action-MMO end of best practice.** The GW2 detail to copy is auto-release of mouse-look in dialogue, menus and panels. Check that Powrush frees the cursor in `SettingsStubRoot`, `PlacesRoot`, `SatchelRoot`, `NameHouseRoot` and the soft panels. If it does not, that is a CAM follow-up, not this card.
- **Raw input opt-in.** WoW offers raw mouse as an option that applies only to mouselook and bypasses OS acceleration ([WoWInterface thread](https://www.wowinterface.com/forums/showthread.php?t=58836)). **Verdict: matches.** Powrush's raw toggle is also look-only. Its default (off) could not be compared, because WoW's default is **UNVERIFIED**.
- **Smoothing on by default, raw drops it.** No reference source we opened states a camera-smoothing default (**UNVERIFIED**). Keeping light smoothing on by default for new players, with a raw 1:1 opt-out for precise players, is a reasonable pattern. We cannot cite it as industry practice. GAG asks for adjustable sensitivity ([GAG](https://gameaccessibilityguidelines.com/include-an-option-to-adjust-the-sensitivity-of-controls/)). Powrush still needs a visible slider and an invert-Y option (UI-OPTIONS-1).
- **Gap.** `raw_look` is today set by editing `powrush_settings.json` (CAM-1 README, "Check by hand"). The references expose camera and input options in-menu (GW2 camera options, RS3 Controls tab). Recommend surfacing `raw_look` and sensitivity in the Esc plate (`SettingsStubRoot`) in PLAYTEST-UI-OPTIONS-1.

## 10. Proposed follow-up cards (all OUT OF SCOPE here)

1. **PLAYTEST-HUD-COPY-1 (copy/labels only).** Tooltip text for satchel items, "+N more actions" hints, a paired word/icon for every colour-coded status, and one-line next-step wording in `FirstSessionGuidanceStrip`. **Strictly copy/labels: no anchor, width, z-index, scale or layout change.**
2. **PLAYTEST-HUD-LAYOUT-1.** Implement the slab registry and presets from `docs/design/UI_LAYOUT_SYSTEM.md`, plus a gradual reveal of class-5 slabs in the first hour (RS3), a "tooltips never cover their button" rule, and a compass hide gesture.
3. **PLAYTEST-UI-OPTIONS-1.** In-menu settings on `SettingsStubRoot`: mouse sensitivity, invert Y, `raw_look` toggle (surface #776), camera Restore Defaults, global UI scale, text size S/M/L, HUD opacity, highlight/accent colour pickers, reduce camera motion and flash. A screen reader is noted as a long-term goal.
4. **PLAYTEST-TRACKER-1.** Point `CompassSlabRoot` at the active Journey goal (D4 auto-pin pattern). Add a one-key cycle for the tracked goal (ESO), a re-readable hint log in `JourneyEchoRoot` (FFXIV Active Help) and an optional audio ping.
5. **PLAYTEST-ONBOARD-HINTS-1.** Event-triggered one-shot hints (Valheim pattern) via `WhisperRoot` / `FirstSessionGuidanceStrip`, plus one "new / returning" question that tunes hint volume (OSRS/RS3).
6. **PLAYTEST-PRACTICE-READY-1.** A "ready in N" cooldown readout and input queueing on `LivingPracticeStrip` (FFXIV/RS3), and press-and-hold repeat for Tend (WoW).
7. **PLAYTEST-CAM-RELEASE-1** (only if the check in section 9 fails). Auto-release mouse-look whenever a modal or soft panel opens (GW2).

Deliberately not proposed: a minimap slab, target/unit frames, and a full per-element HUD editor (see skips above).
