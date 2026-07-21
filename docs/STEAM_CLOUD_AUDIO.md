# Steam Cloud — Audio Moment Catalog

**Partner dashboard steps:** [`publishing/steam/PARTNER_CHECKLIST.md`](../publishing/steam/PARTNER_CHECKLIST.md)  
**Canonical config:** [`publishing/steam/steam_cloud_config.json`](../publishing/steam/steam_cloud_config.json)

Powrush-MMO syncs **recipe catalogs** (not bulk PCM) through Steam Cloud.

## Two layers

| Layer | Path / API | When |
|-------|------------|------|
| **Auto-Cloud stage** | OS-specific `…/Powrush-MMO/steam_cloud/audio_moments/catalog_cloud_v1.json` + portable `steam_cloud/audio_moments/` | Always |
| **RemoteStorage SDK** | `FileWrite("catalog_cloud_v1.json")` | `--features steam` + Steam running + app cloud enabled |

## AppID resolution

1. `STEAM_APP_ID` env  
2. `app_id.shipping` in config (when set)  
3. `steam_appid.txt`  
4. `app_id.development` (480 Spacewar)

```bash
python publishing/steam/sync_steam_appid.py --development
python publishing/steam/sync_steam_appid.py --shipping   # after shipping ID is set
cargo run -p powrush-client --features steam
```

**Never ship `steam_appid.txt` in production depots.**

## Partner checklist (summary)

1. **Enable Steam Cloud** on the app + set quota + **Publish**
2. **Auto-Cloud** four rules (Win/Mac/Linux/InstallDir) + **Publish**
3. Set **`app_id.shipping`** and sync / strip for shipping builds

See the full checklist for exact dashboard clicks and verification logs.

## Runtime

- Export on every audio moment save (stage + optional RemoteStorage)
- Import on startup (RemoteStorage preferred, else stage files)
- Callbacks pumped every frame when Steam is live
- Checklist readiness logged: `config✓ · shipping_appid· · steam_init· · app_cloud· · account_cloud·`

Contact: info@Rathor.ai
