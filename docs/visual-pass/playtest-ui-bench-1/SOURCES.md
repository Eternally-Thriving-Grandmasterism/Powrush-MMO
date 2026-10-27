# PLAYTEST-UI-BENCH-1: sources

All URLs fetched 2026-10-26 (America/Toronto) and resolved. Some wiki and publisher hosts block plain `curl` with 403 (warcraft.wiki.gg, oldschool.runescape.wiki, runescape.wiki, albiononline.com). Those were confirmed with a browser-grade fetch that returned the page content.

| # | URL | type | supports |
|---|---|---|---|
| 1 | https://warcraft.wiki.gg/wiki/Edit_Mode | wiki | WoW Edit Mode: movable action bars 1-8, player/target/focus frames, objective tracker, minimap size/rotation, HUD tooltip; disabled before level 10 on Exile's Reach; layout sharing |
| 2 | https://news.blizzard.com/en-us/article/23841481/world-of-warcraft-dragonflight-hud-and-ui-revamp | official | WoW HUD revamp: clutter reduction, more prominent minimap and health bar, saved/shared layouts per spec, interact key, action targeting, quest tracker planned |
| 3 | https://news.blizzard.com/en-us/article/23876525/accessibility-updates-in-dragonflight | official | WoW press-and-hold casting, FOV option, interact key, action targeting, minimum character-name size with distance fade, reduce camera motion, alternate full-screen effects |
| 4 | https://www.wowinterface.com/forums/showthread.php?t=58836 | community forum | WoW raw mouse is a mouselook-only option that bypasses OS acceleration (non-official) |
| 5 | https://na.finalfantasyxiv.com/game_manual/view/ | official | FFXIV hotbar lock and per-class save; target, focus-target and enmity bars; Duty List; Main Scenario Guide; minimap dots and time; Active Help; Keybind; Configuration Sharing |
| 6 | https://na.finalfantasyxiv.com/uiguide/know/ | official | FFXIV UI guide index (hide Duty List, recast-timer and hotbar topics) |
| 7 | https://na.finalfantasyxiv.com/uiguide/know/know-hud/hud-layout.html | official | FFXIV HUD Layout: drag, resize, hide, transparency, copy layouts |
| 8 | https://na.finalfantasyxiv.com/uiguide/know/know-hb/hotbar_shape.html | official | FFXIV hotbar shape changes |
| 9 | https://na.finalfantasyxiv.com/uiguide/know/know-hb/hotbar_recasttimes.html | official | FFXIV larger, centred recast timer option |
| 10 | https://www.guildwars2.com/en/news/introducing-the-action-camera/ | official | GW2 Action Camera: always-on mouse-look while enabled, auto-off in dialogue/menus, toggle keybind, four crosshair states, RMB target lock |
| 11 | https://www.guildwars2.com/en/news/get-ready-camera-changes-are-coming/ | official | GW2 camera options: FOV, horizontal/vertical offset, collision and zoom sensitivity, camera Restore Defaults |
| 12 | https://en.uesp.net/wiki/Online:Controls | wiki | ESO default controls: mouse look/turn, skills 1-5 + R ultimate, quest cycle T, journal J, all remappable |
| 13 | https://help.elderscrollsonline.com/app/answers/detail/a_id/28891/~/how-do-i-increase-the-font-size-in-game%3F | official support | ESO Custom Scale slider scales font and UI |
| 14 | https://albiononline.com/news/dev-talk-horizons-ui | official | Albion HUD Editor (position/scale/opacity, import/export), Quick Settings Presets on a keybind |
| 15 | https://albiononline.com/changelog/QUEEN%20PATCH%2011 | official | Albion mobile: target glow and arrow, compact party/quest view, Destiny Board tracker, minimap hide and larger player icon, road colour readability, joystick tutorial screens |
| 16 | https://oldschool.runescape.wiki/w/Settings | wiki | OSRS tooltips toggles, quest-list sort/filter/text size/colours, minimap zoom, interface scaling, colourful-fade toggle, camera controls, Tutorial Island experience answer tunes settings |
| 17 | https://runescape.wiki/w/Interface | wiki | RS3 movable/resizable windows, action bar cannot close, square/circle minimap, compass snaps north, target info panel and rings, keybind remap with conflict warning, tutorial reveals UI gradually, tooltip and colourblind adrenaline-bar fixes, hints off for experienced players |
| 18 | https://runescape.wiki/w/Settings/Combat_%26_Action_Bar/Action_Bar | wiki | RS3 cooldown timer overlay (centre/top-right), ability queueing |
| 19 | https://news.blizzard.com/en-us/article/23954932/combatting-demons-with-accessibility-in-diablo-iv | official | D4: 50+ accessibility features, 3 text sizes, cursor sizes, subtitles, highlight colours, screen reader |
| 20 | https://news.blizzard.com/en-us/article/24139219/a-crucible-for-allaccessibility-features-in-vessel-of-hatred | official | D4 Auto-Pin with minimap route, compass ring with colour choice, audio navigation pings |
| 21 | https://caniplaythat.com/2023/07/18/diablo-iv-accessibility-review/ | review | D4 colourblind filters (3 types + intensity), reviewer doubts filters and prefers highlights, HUD font scaling |
| 22 | https://caniplaythat.com/2023/05/23/diablo-iv-accessibility-detailed-by-blizzard/ | review | D4 full remap on KBM and controller, hold-to-toggle options |
| 23 | https://www.valheimdatabase.com/hugin-munin/ | fan database | Valheim Hugin event-triggered hints (pickup, crafting, cold, encumbrance, death, biome entry) |
| 24 | https://gameaccessibilityguidelines.com/allow-interfaces-to-be-rearranged/ | guideline | rearrangeable UI |
| 25 | https://gameaccessibilityguidelines.com/ensure-no-essential-information-is-conveyed-by-a-colour-alone/ | guideline | never colour alone |
| 26 | https://gameaccessibilityguidelines.com/allow-controls-to-be-remapped-reconfigured/ | guideline | remappable controls |
| 27 | https://gameaccessibilityguidelines.com/include-an-option-to-adjust-the-sensitivity-of-controls/ | guideline | adjustable sensitivity |

## Tried and dropped (did not resolve or could not be read)

- `wiki.guildwars2.com/*` (Options, Mini_map, Heart, Tutorial, Colorblind_mode): the wiki served a maintenance page, so no GW2 tooltip, minimap, hearts or onboarding claims were made.
- `ffxiv.consolegameswiki.com/*`, `na.finalfantasyxiv.com/uiguide/hud/`: 404.
- `en.uesp.net/wiki/Online:Settings`, `Online:Compass`: 404.
- `valheim.fandom.com/*`: 403 to every fetcher. No Valheim settings, UI-scale or sensitivity claims were made.
- `support.blizzard.com/en/article/000362342` (WoW accessibility settings): JS-only, so its content could not be read. Not cited.
- `elderscrollsonline.com/en-us/newplayerguide/accessibility`: age gate. Not cited.
- `help.elderscrollsonline.com/.../a_id/56111`: 401. Not cited.
- `wiki.albiononline.com/*`: 403. Not cited.
- `warcraft.wiki.gg/wiki/CVar_rawMouseEnable`: 404. The forum thread (#4) was used instead.
