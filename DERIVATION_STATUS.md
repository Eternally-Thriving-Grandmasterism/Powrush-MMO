# Powrush-MMO Derivation Status

**Steamworks RemoteStorage — LIVE (v21.89.5)**  
**Audio Moments stack COMPLETE**  
**Permanent PATSAGi Councils — ACTIVE**

## Steam RemoteStorage

| Item | Detail |
|------|--------|
| Feature | `cargo run -p powrush-client --features steam` |
| Module | `client/steamworks_remote_storage.rs` |
| API | `FileWrite` / `FileRead` / `FileExists` / quota / callbacks |
| Remote name | `catalog_cloud_v1.json` |
| Fallback | Local stage + Auto-Cloud if Steam unavailable |
| Dev AppID | `client/steam_appid.txt` → `480` (Spacewar) or `STEAM_APP_ID` |
| Docs | `docs/STEAM_CLOUD_AUDIO.md` |

## Plugin order

```
SteamworksRemoteStoragePlugin  → init Client + backend
SteamCloudAudioMirrorPlugin    → import (SDK first) / export on save
```

## Paths

| Role | Path |
|------|------|
| Local catalog | `player_data/audio_moments/catalog.json` |
| Steam stage | `steam_cloud/audio_moments/catalog_cloud_v1.json` |
| RemoteStorage | `catalog_cloud_v1.json` |

Contact: info@Rathor.ai

**Thunder locked in.**  
Yoi ⚡
