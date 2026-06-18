# Audio Assets

## Drop Location

Place all audio files for the client here:

- `assets/audio/rollback_whoosh.ogg`
- `assets/audio/epiphany_bloom.ogg`
- `assets/audio/emergence_resonance.ogg`

## Format

Recommended format: **.ogg** (Vorbis) for best compatibility with Bevy.

## Notes

- These files are loaded via the `AudioAssets` resource in `client/src/prediction.rs`.
- The audio playback system is fully event-driven via `AudioTriggerEvent`.
- Volume and playback are handled in `audio_playback_system`.

Last updated: 2026-06-18
