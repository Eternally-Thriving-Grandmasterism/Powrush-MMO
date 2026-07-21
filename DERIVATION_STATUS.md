# Powrush-MMO Derivation Status

**Partner checklist (in-repo) — COMPLETE (v21.89.6)**  
**Steamworks RemoteStorage — LIVE**  
**Permanent PATSAGi Councils — ACTIVE**

## Partner checklist mapping

| Checklist item | In-repo delivery |
|----------------|------------------|
| 1. Enable Steam Cloud | Docs + runtime detects `app_cloud`; warns if false |
| 2. Auto-Cloud paths | Exact rules in `steam_cloud_config.json`; OS stage dirs |
| 3. Shipping AppID | Config field + `sync_steam_appid.py` + resolution order |

**Human-only remaining:** log into partner.steamgames.com → Enable Cloud → Add Auto-Cloud rows → Publish → set `app_id.shipping` when Valve assigns ID.

## Key paths

- `publishing/steam/PARTNER_CHECKLIST.md`
- `publishing/steam/steam_cloud_config.json`
- `publishing/steam/sync_steam_appid.py`
- `client/steam_partner_config.rs`
- `client/steamworks_remote_storage.rs`
- `client/steam_cloud_audio_mirror.rs`

Contact: info@Rathor.ai

**Thunder locked in.** Yoi ⚡
