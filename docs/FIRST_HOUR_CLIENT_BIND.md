# FIRST_HOUR_CLIENT_BIND.md

Do not land this as a giant client rewrite. Bind the existing keys to `shared::climate_node`.

## Map

| Key | Call |
| --- | --- |
| E on nearest glowing/tended node | `LivedHour::tend(node_id)` |
| I | show `hour.satchel.takes` |
| R then 1 | `hour.allocate(AllocKind::Flow)` |
| R then 2 | `hour.allocate(AllocKind::Reserve)` |
| H | hide the guidance card only |
| quit / resume | write/read `data/powrush_lived_tick.json` via `LivedHour::to_json` / `from_json` |

## Feel hooks already named in README

- Take: glow + camera punch + first-take rumble
- No-take on Resting/Stressed: no rumble, short world sentence
- Flow success: nearest tired node brightens
- Reserve success: satchel loses one take, repair-rights count +1, field unchanged

## Feature gate

Keep this on the default client path.
Do not enable `full_rbe` or a Ra-Thor path dep to play the hour.
