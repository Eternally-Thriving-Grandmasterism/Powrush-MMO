# Steam Cloud — Audio Moment Catalog

Powrush-MMO syncs **recipe catalogs** (not bulk PCM) through Steam Cloud so players can recall moments across machines.

## Two layers

| Layer | Path / API | When |
|-------|------------|------|
| **Local stage** | `steam_cloud/audio_moments/catalog_cloud_v1.json` | Always (Auto-Cloud fallback) |
| **RemoteStorage SDK** | `ISteamRemoteStorage::FileWrite("catalog_cloud_v1.json")` | `--features steam` + Steam running |

## Build

```bash
# With Steamworks SDK linked
cargo run -p powrush-client --features steam

# Optional app id override (dev)
export STEAM_APP_ID=480   # Spacewar for testing; replace with real AppID
```

Place `steam_appid.txt` next to the binary (repo includes `client/steam_appid.txt` with `480` for Spacewar testing).

## Steamworks Partner setup

1. **Steamworks App Admin → Steam Cloud**
   - Enable Steam Cloud for the app
   - Byte quota: default 1 GB is plenty (catalog JSON is small)
2. **Optional Auto-Cloud** (for the stage file without SDK):
   - Root: App Install Directory (or appropriate platform root)
   - Subdirectory: `steam_cloud/audio_moments`
   - Pattern: `*`
3. Publish the cloud settings change

## Runtime behavior

**Export** (after any audio moment save):
1. Write local stage JSON
2. If Steam Cloud enabled for account + app → `FileWrite("catalog_cloud_v1.json", bytes)`
3. Quota checked before write

**Import** (startup):
1. Prefer `FileRead` from RemoteStorage when available
2. Else read local stage file
3. Merge any moments missing from the local catalog

**Callbacks:** `SingleClient::run_callbacks()` is pumped every frame while Steam is initialized.

## What is NOT uploaded

- Rendered WAV under `player_data/audio_moments/rendered/` (regenerated from recipes)
- Premade game assets under `assets/audio/` (ship with the build)

## API surface (code)

- Trait: `SteamCloudBackend` in `client/steam_cloud_audio_mirror.rs`
- Impl: `SteamRemoteStorageBackend` in `client/steamworks_remote_storage.rs`
- Plugin order: `SteamworksRemoteStoragePlugin` → `SteamCloudAudioMirrorPlugin`

Contact: info@Rathor.ai
