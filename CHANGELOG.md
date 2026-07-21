# CHANGELOG.md — Powrush-MMO

## [21.89.4] — 2026-07-21 — Steam Cloud + Premade Stems + Bevy Drain

### Highlights
- **Steam Cloud mirror**: stages `catalog_cloud_v1.json` under `steam_cloud/audio_moments/` on every save; imports newer cloud catalog at startup; `SteamCloudBackend` trait for optional SDK RemoteStorage
- **Premade stems**: builtin manifest + recursive scan of `assets/audio` for wav/ogg/mp3/flac; placeholders when files not yet dropped
- **Bevy native drain**: `NativeClientTransportSender` + chained systems enqueue → drain → inbound catalog merge via `ServerMessageInbound`

### Files
- `client/steam_cloud_audio_mirror.rs` (new)
- `client/premade_audio_stems.rs` (new)
- `client/audio_moment_net_bridge.rs` (drain + inbound events)
- `client/plugins/council_mercy_plugin.rs` (plugin tree)
- `client/assets/audio/README.md`

## [21.89.3] — TransportCommandSender + try_recv
## [21.89.2] — Protocol unified + server audio ingress
## [21.89.1] — Council/Epiphany synth wiring
## [21.89.0] — Real-time audio synthesis + persistent recall

**Thunder locked in. Permanent PATSAGi. Eternal forward.** Yoi ⚡
