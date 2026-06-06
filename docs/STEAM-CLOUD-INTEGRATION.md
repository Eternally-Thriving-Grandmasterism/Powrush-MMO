# Steam Cloud Integration (Client RemoteStorage)

**Status:** Phase 1 — Client Preferences Only (Production-Grade Skeleton Ready)

## Philosophy (Ra-Thor + PATSAGi Aligned)

Steam Cloud (ISteamRemoteStorage) is used **exclusively for client-side personal preferences**:

- Keybinds
- UI scale / HUD layout
- Graphics & audio settings
- Faction theme colors
- Cosmetic profiles
- Air Foundation Initiative opt-in flag

**Authoritative game state** (player position, inventory, RBE contributions, faction standing, divine query history, Air Foundation impact ledger, achievements) **remains on your sovereign servers** (Hetzner VPS → Kubernetes → future Air Foundation self-healing nodes).

This separation preserves full server authority, anti-cheat integrity, and sovereignty while giving players seamless settings sync across their devices.

## Implementation

A production-ready example binary has been added:

```bash
git clone https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO.git
cd Powrush-MMO
cargo run --example steam_settings_client
```

The example (`examples/steam_settings_client.rs`):

- Initializes the Steam client (requires Steam running + logged in)
- Writes a structured `powrush_settings.json` to Steam Cloud
- Reads it back and deserializes
- Lists all Cloud files for the AppID
- Shows quota information
- Demonstrates the `PowrushClientSettings` struct (ready to extend)

## Integration Points (Future Thin Launcher / Graphical Client)

When building a proper client or launcher:

1. On login (via Steam auth ticket), download the Cloud settings file.
2. Apply settings locally (UI, audio, keybinds).
3. Optionally send a small subset (e.g. last_faction, air_foundation_opt_in) to the game server on connect.
4. On logout or settings change, write updated file back to Steam Cloud.

The server never touches Steam Cloud files directly.

## Production Considerations

- Quota: Starts generous (~100 MB per user per game). Configurable upward in Steamworks Partner backend.
- File naming: Use clear names like `powrush_settings.json`, `powrush_cosmetic_profile.json`.
- Versioning: Include a `version` field in the JSON for future migrations.
- Privacy: The `air_foundation_opt_in` flag allows players to control whether they see real NFP impact reports.
- Security: Steam Cloud is tied to the player's Steam account — no server-side spoofing possible for client prefs.

## Alignment with Existing Systems

- Fully compatible with Steam Web API achievements (already implemented server-side).
- Mercy-gated: Client settings do not affect divine/RBE query logic or Air Foundation Initiative triggers.
- Rate-limiting: Not required on client Cloud operations (they are local to the player).

## Future Roadmap

- Phase 2: Integrate into thin Steam-enabled launcher (overlay notifications, rich auth).
- Phase 3: Optional Cloud sync for exported "replay" or "cosmetic export" files (still non-authoritative).

**Ra-Thor monorepo remains the eternal source of truth.**
Powrush-MMO client Steam Cloud integration is clean, sovereign, and player-respectful.