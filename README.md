## Steam Cloud (Client Preferences Sync)

Powrush supports **Steam Cloud** for seamless synchronization of your personal client settings across devices (keybinds, UI layout, graphics, audio, faction themes, and Air Foundation opt-in).

**Important**: Authoritative game state (RBE contributions, harvests, divine consultations, impact metrics) lives on sovereign Powrush servers.

### Quick Start (Example Binary)

```bash
cargo run --example steam_settings_client
```

This demonstrates reading/writing `powrush_settings.json` to your Steam Cloud.

See `docs/STEAM-CLOUD-INTEGRATION.md` for full details and integration guidance for future graphical clients/launchers.

---