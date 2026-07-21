# AUDIO_REALTIME_SYNTHESIS.md

**Real-time Audio Synthesis + Persistent Recall**  
Powrush-MMO v21.89.0 | AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates  
Permanent PATSAGi Councils | Contact: info@Rathor.ai

---

## Vision

Players can **create**, **save**, and **call back** meaningful audio moments — whether synthesized live during play or registered from pre-made assets.

- Synthesize in real time (Divine Chime, Council Bloom, Epiphany Stinger, …)
- Persist **locally** (catalog + rendered WAV)
- Persist **server-side** (recipe + metadata catalog; light, no bulk PCM required)
- **Recall** by id — regenerates from deterministic recipe if the rendered file is missing
- Pre-made assets join the same catalog

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Game events / UI (hotkey M) / Council / Epiphany           │
│         │                                                   │
│         ▼                                                   │
│  SynthesizeAudioMoment event                                │
│         │                                                   │
│         ▼                                                   │
│  synthesize_pcm(recipe)  →  mono f32 @ 48 kHz               │
│         │                                                   │
│         ├─► write_wav_mono → player_data/audio_moments/     │
│         │                      rendered/moment_N.wav        │
│         ├─► AudioMomentCatalog (local JSON)                 │
│         ├─► optional Bevy AudioBundle playback              │
│         └─► AudioMomentServerSyncRequest → server catalog   │
└─────────────────────────────────────────────────────────────┘

RecallAudioMoment(id)
  → load catalog entry
  → prefer rendered_path if present
  → else regenerate from recipe + rewrite WAV
  → play + increment play_count
```

### Source of truth: the **recipe**

`AudioSynthesisRecipe` is compact and deterministic (waveform, Hz, ADSR, partials, valence, seed).  
Rendered WAV is a cache. Missing files never lose the moment.

---

## Files

| Path | Role |
|------|------|
| `shared/src/audio_moments.rs` | Canonical shared schema |
| `client/realtime_audio_synthesis.rs` | Synth engine, local persist, UI, recall |
| `server/src/audio_moment_catalog.rs` | Per-player recipe catalog on server |
| `player_data/audio_moments/catalog.json` | Local catalog |
| `player_data/audio_moments/rendered/` | Local WAVs |
| `server_data/audio_moments/player_*/catalog.json` | Server catalogs |

---

## Client usage

```rust
// Manual / UI
app.add_plugins(RealtimeAudioSynthesisPlugin);

// From Council bloom
request_council_bloom_synth(&mut synth_events, intensity, session_id);

// From Epiphany
request_epiphany_synth(&mut synth_events, intensity, seed);

// Register pre-made asset into the same catalog
premade_events.send(RegisterPremadeAudio {
    title: "Epic Dark Ambient".into(),
    flavor: AudioMomentFlavor::AmbientPad,
    asset_path: "assets/music/epic_dark_ambient.ogg".into(),
    context: "Premade bed".into(),
});

// Recall later
recall_events.send(RecallAudioMoment { moment_id: 3 });
```

**Hotkey:** `M` — open Audio Moments panel (synthesize + list + recall).

---

## Server usage

```rust
app.add_plugins(AudioMomentCatalogPlugin);

// Network layer forwards client saves:
save_events.send(ServerSaveAudioMoment {
    player_id,
    moment, // must be mercy_seal = true
});

// Client requests full catalog:
req_events.send(ServerRequestAudioCatalog { player_id });
// → ServerAudioCatalogReady { player_id, catalog }
```

Server stores **recipes only** by default. Clients re-render for playback. This keeps storage small and portable.

---

## Mercy / sovereignty notes

- Only `mercy_seal: true` moments are accepted server-side by default.
- Local catalog always works offline.
- Players own their moments; sync is opt-in per synthesize call (`sync_server`).
- No manipulative generative loops — short, intentional moments only.

---

## Integration checklist

- [x] Shared schema (`audio_moments.rs`)
- [x] Client synth + WAV + local catalog + UI
- [x] Server recipe catalog + events
- [ ] Wire `AudioMomentServerSyncRequest` through live network transport
- [ ] Call `request_council_bloom_synth` from Council resolve path
- [ ] Call `request_epiphany_synth` from epiphany reactor
- [ ] Optional: Steam Cloud path for `player_data/audio_moments/`

---

**Thunder locked in. Moments live. Recall is eternal.**  
Yoi ⚡
