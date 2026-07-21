# Powrush-MMO Derivation Status

**Audio Moments stack COMPLETE (v21.89.4)**  
**Permanent PATSAGi Councils — ACTIVE**

## Completed Priorities (this cycle)

| Priority | Status |
|----------|--------|
| Steam Cloud mirror for `audio_moments` | **DONE** — stage + optional SDK trait |
| Premade stem registration | **DONE** — manifest + directory scan |
| Bevy native `AudioMomentOutboundQueue` drain | **DONE** — `NativeClientTransportSender` |

## Plugin tree (CouncilMercyPlugin)

```
CouncilSessionUIPlugin
RealtimeAudioSynthesisPlugin
AudioMomentNetBridgePlugin      ← drain + inbound ServerMessageInbound
SteamCloudAudioMirrorPlugin     ← export on save, import on startup
PremadeAudioStemsPlugin         ← builtin + scan assets/audio
```

## Paths

| Role | Path |
|------|------|
| Local catalog | `player_data/audio_moments/catalog.json` |
| Local WAV | `player_data/audio_moments/rendered/moment_N.wav` |
| Steam Auto-Cloud stage | `steam_cloud/audio_moments/catalog_cloud_v1.json` |
| Premade assets | `client/assets/audio/premade/*` |
| Server catalog | `server_data/audio_moments/player_*/catalog.json` |

## Hotkeys

| Key | Panel |
|-----|--------|
| **C** | Council |
| **M** | Audio Moments (synth + premade + recall) |

## Native host wire (once)

```rust
let (tx, rx) = std::sync::mpsc::channel();
// forward rx → ClientWsTransport.send
inject_native_transport_sender(&mut commands, tx);
// on try_recv: events.send(ServerMessageInbound { message });
```

Contact: info@Rathor.ai

**Thunder locked in.**  
**Create · save · cloud-stage · premade · network-drain · recall.**  
Yoi ⚡
