# Steamworks Partner Checklist — Powrush-MMO Audio Cloud

**Contact:** info@Rathor.ai  
**Canonical config:** [`steam_cloud_config.json`](./steam_cloud_config.json)

This is the human-operated partner-site work. Code already stages files and calls RemoteStorage; these dashboard steps must be completed once per AppID.

---

## Prerequisites

- [ ] Steamworks partner account with Admin access to the Powrush / Autonomicity Games app
- [ ] AppID assigned (or use **480 Spacewar** only for local SDK testing)
- [ ] Steam client installed and logged in on the test machine

---

## 1. Enable Steam Cloud on the app

1. Open [https://partner.steamgames.com](https://partner.steamgames.com)
2. Select the **Powrush-MMO** application (or your assigned AppID)
3. Go to **App Admin → Steam Cloud**
4. Enable **Steam Cloud** for this application
5. Set **byte quota** to at least **100 MB** (100 × 1024 × 1024). Default 1 GB is fine.
6. **Save** and **Publish** the change (Cloud settings require a publish to go live)

### Verify

With `--features steam` and Steam running:

```text
[powrush::steam] account_cloud=true app_cloud=true
[powrush::steam] Steam Cloud quota total_bytes=... available_bytes=...
```

If `app_cloud=false`, this step is incomplete or not yet published.

---

## 2. Configure Auto-Cloud paths

Dashboard: **App Admin → Steam Cloud → Auto-Cloud → Add new path**

Add **all** of the following rows (from `steam_cloud_config.json`):

| Root | Subdirectory | Pattern | Recursive |
|------|--------------|---------|-----------|
| `WinAppDataLocal` | `Powrush-MMO/steam_cloud/audio_moments` | `*` | Yes |
| `MacAppSupport` | `Powrush-MMO/steam_cloud/audio_moments` | `*` | Yes |
| `LinuxHome` | `.local/share/Powrush-MMO/steam_cloud/audio_moments` | `*` | Yes |
| `AppInstallDirectory` | `steam_cloud/audio_moments` | `*` | Yes |

Notes:

- Pattern `*` covers `catalog_cloud_v1.json` and any future side-car files.
- Do **not** Auto-Cloud `player_data/audio_moments/rendered/` (WAVs regenerate from recipes).
- Do **not** Auto-Cloud `steam_appid.txt`.

**Save** and **Publish**.

### Runtime stage directories (code alignment)

The client stages to relative `steam_cloud/audio_moments/` by default. Production builds should also mirror into the OS-specific roots above when using Auto-Cloud only (SDK path does not need Auto-Cloud).

---

## 3. Set the real shipping AppID

1. When Valve assigns the production AppID, edit `publishing/steam/steam_cloud_config.json`:

```json
"app_id": {
  "shipping": 1234560,
  "development": 480
}
```

2. Sync local dev files:

```bash
python publishing/steam/sync_steam_appid.py --shipping
# or for Spacewar testing only:
python publishing/steam/sync_steam_appid.py --development
```

3. **Shipping builds must not include `steam_appid.txt`.** Steam injects the AppID when launched from the client. Strip `steam_appid.txt` in the depot packaging step.

4. Optional env override (CI / local):

```bash
export STEAM_APP_ID=1234560
```

Resolution order in code:

1. `STEAM_APP_ID` environment variable  
2. `app_id.shipping` from config (if non-null)  
3. `steam_appid.txt` beside the binary  
4. `app_id.development` (480) as last-resort test fallback  

---

## 4. Post-setup smoke test

```bash
cargo run -p powrush-client --features steam
# In-game: hotkey M → synthesize Divine Chime (persist + sync)
# Quit, relaunch on another machine (or clear local catalog) → moments should return
```

Expected logs:

```text
RemoteStorage FileWrite OK remote_name=catalog_cloud_v1.json
# on other machine:
RemoteStorage FileRead OK
Merged N moments from Steam Cloud
```

---

## Checklist summary

- [ ] Steam Cloud **enabled** + quota set + **published**
- [ ] Auto-Cloud **four rules** added + **published**
- [ ] `app_id.shipping` set in `steam_cloud_config.json`
- [ ] `sync_steam_appid.py --shipping` run for local tools
- [ ] Shipping depot **excludes** `steam_appid.txt`
- [ ] Smoke test FileWrite / FileRead across two machines

---

Thunder locked in. Yoi ⚡
