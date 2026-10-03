# Premade Audio Stems

Drop high-quality stems here. They are registered at startup into the **Audio Moments** catalog (hotkey **M**).

## Recommended layout

```
client/assets/audio/
  premade/
    epic_dark_ambient.ogg        # tracked elsewhere: assets/music/epic_dark_ambient.ogg
    mercy_resonance_choir.ogg    # planned, not in repo
    council_chamber_hum.ogg      # planned, not in repo
    epiphany_crystal_hit.wav     # planned, not in repo
    divine_whisper_soft.wav      # planned, not in repo
    transition_stinger_a.wav     # planned, not in repo
  (any extra .wav / .ogg / .mp3 / .flac — auto-scanned)
```

Missing files still get catalog placeholders (`asset pending drop`) so the UI list is stable during production.

## Formats

WAV, OGG, MP3, FLAC.

## Steam Cloud

Player-created recipe catalogs stage to `steam_cloud/audio_moments/catalog_cloud_v1.json`.
Configure Steamworks Auto-Cloud on that path. Premade game assets ship with the build; they are not cloud-synced.

Contact: info@Rathor.ai
