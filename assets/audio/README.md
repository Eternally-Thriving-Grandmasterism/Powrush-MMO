# Audio Assets

## U4 Peace yard

Git tracks two audio files in this folder:

- Bed (loop): `assets/audio/peace_yard_bed.ogg` — quiet two-partial drone.
- Well sting: `assets/audio/peace_well_sting.ogg` — soft short triad on existing well Use.

The lived mixer loads only those two (`shared/peace_audio.rs` `BED_ASSET` and `STING_ASSET`). Heartwood and Depths reuse the yard bed at a lower gain. Generate both with `bash scripts/gen_peace_yard_audio.sh`.

Mute is the existing pause/Settings row. A missing file must never block Use or hang boot.

Recommended format: **.ogg** (Vorbis) for Bevy.

Contact: info@Rathor.ai
