# In-game radio — stations, licence manifest, and future station slots (design)

**Contact:** [info@Rathor.ai](mailto:info@Rathor.ai)

CARD AUDIO-RADIO-DESIGN-1 — the first AUDIO card. Design only. Base: `main` at `382c74d4`.

Docs only: a proposal, not a work order. This file adds no client, shared, host or Cargo code, and no audio files. Nothing here lands in the game until Core names a slice. Title Online stays grey. Everything is offline-bundled; there is **no live internet streaming** in this design. Any purchase (paid packs, no-attribution licences, premium catalogues) is Sherif's call.

The radio's unlock (a quest with combat or Engineering-profession paths, account-bound) is designed separately in AUDIO-RADIO-QUEST-DESIGN-1; this file does not design it.

Labels used below:

- **FACT** — read from the repo at `382c74d4` (file and line cited) or from a source's own public page on 2026-10-26 (URL cited).
- **PROPOSAL** — a design choice. Every field, path, gain, rank and name marked this way is a proposal, not a fact.
- **UNVERIFIED** — a claim this card could not confirm on the source's own page on 2026-10-26. It comes from Lead Mate's research file (§1.1) or is an open gap. Treat it as unknown.
- **PROPOSED · HELD** — player-facing copy (station names). Held for Sherif's OK; nothing ships before it.
- **Qn** — an open question (§9).

No certification or legal claims are made here. Terms below are "as published on the source's page, read on 2026-10-26". Terms change; the manifest (§4) stores a dated snapshot per track for that reason.

---

## 0. Source

> In-game radio for Powrush-MMO: genre stations with funny, symbolic names that players tune into, playing licensed royalty-free music (e.g. Karl Casey / White Bat Audio), later adding procedurally generated and possibly premium music; the same sources may serve the Ra-Thor animated series.
>
> — Sherif, 2026-10-26 8:53 PM ET (relayed)

Core's card on COUNCIL BOARD issue #738 (comment 6431133983) sets the bounds: station model (§2–§3), a per-track licence manifest with a CI check designed but not wired (§4–§5), future procedural and premium slots (§6), honour each source's terms (§1, §7), cite real terms only, and rank station names with the creative-stuck approach, held for Sherif's OK (§8).

Standing rules this file keeps (FACT, councils' audio decision 2026-10-14): music is **one shared audio pipeline** across Sherif's products (Ra-Thor⚡️Living Thunder first, Powrush-MMO reuses it). Never name real artists or tracks in AI prompts. No health claims (no "relaxing", "healing", "focus-boosting" copy on any station).

---

## 1. Sources and their terms

### 1.1 Inputs

- Lead Mate's research: `/workspace/lead-mate/audio-rnd/royalty-free-sources-2026-10-26.md` (box file, not in the repo). Read-only research; nobody was contacted.
- This card re-read each source's own public page on 2026-10-26. Where the re-read confirmed a term, it is marked FACT with the URL. Where the page could not be read or did not say, it is UNVERIFIED.

Key finding carried from Lead Mate (and confirmed on every page re-read below): **no source page says anything about "radio", "jukebox" or "music player" use.** An in-game station is a place where the music is the main thing on offer, so any custom licence that bans standalone distribution needs written confirmation before its tracks enter a station (§7.2). Asking a source is an outbound email: Sherif's call.

### 1.2 Source table

"Radio fit" is this card's PROPOSAL, derived from the cited terms. It is not a legal opinion.

| # | Source (genres) | Terms as published, read 2026-10-26 | Credit line (exact) | Radio fit (PROPOSAL) |
|---|---|---|---|---|
| S1 | **White Bat Audio — Karl Casey** (synthwave, darksynth, horror, metal) · [FAQ](https://whitebataudio.com/pages/faq) | FACT: allowed in "films, animations, YouTube videos, live streams, video games and podcasts"; monetized use allowed. FACT: "the soundtrack CANNOT be sold or distributed separately from the film or game … cannot be uploaded to Spotify (or other audio streaming services)". FACT: bans "music projects" (sampling into songs, vocals over, re-releasing under another title). FACT: "No AI is used" in production. Radio/jukebox use: **not addressed** on the page. Formats: UNVERIFIED. Paid pack prices (€9–€129): UNVERIFIED by this card. | `Music by Karl Casey @ White Bat Audio` (song title optional) | **Pending written confirmation** for a player-tuned station (§7.2). Allowed in non-radio game/animation use with credit. |
| S2 | **Kevin MacLeod — incompetech.com** (orchestral, jazz, horror, world, many) · [FAQ](https://incompetech.com/music/royalty-free/faq.html) | FACT: CC BY 4.0; video games "most commonly" credit on a "Credits" screen in settings; the credit must not be obscured. FACT: changes allowed, but "You MUST make it clear in the credits which parts are yours". FACT: paid Standard License where credit is impossible. FACT: ISRC numbers published per track. AI status: UNVERIFIED. Formats: UNVERIFIED. | `Title Kevin MacLeod (incompetech.com)` / `Licensed under Creative Commons: By Attribution 4.0` / `https://creativecommons.org/licenses/by/4.0/` (replace *Title*) | **Fit** under CC BY 4.0. |
| S3 | **Eric Matyas — soundimage.org** (game loops, sci-fi, fantasy, puzzle, horror) · [attribution](https://soundimage.org/attribution-info/), [licence](https://soundimage.org/sample-page/) | FACT: credit must be "in your actual game" (store/info pages are "not sufficient"). FACT: licence page grants a "worldwide, royalty-free, non-sublicensable, non-exclusive, irrevocable license" and states "use … in media that is obscene or pornographic is prohibited". FACT: $30 (USD) per track non-attribution licence. "No A.I." statement and Ogg/MP3 formats: from Lead Mate's homepage read, UNVERIFIED by this card. | Only his music: `Music by Eric Matyas` / `www.soundimage.org`. Mixed with others: list each of his track titles, then `by Eric Matyas` / `www.soundimage.org` | **Fit**; credit in-game (the radio credits screen satisfies "in your actual game" — PROPOSAL). |
| S4 | **Scott Buckley** (cinematic orchestral, ambient, epic) · [FAQ](https://www.scottbuckley.com.au/library/faq/) | FACT: free commercial use with credit (CC BY). FACT: refuses use in albums released to streaming services ("I will also not grant a license for this purpose"). FACT: remixes are **not** CC BY. FACT: Smart Content ID claims uncredited YouTube videos. AI status and formats: UNVERIFIED. | At least `Scott Buckley` (PROPOSAL: `Music by Scott Buckley — CC BY 4.0 — https://creativecommons.org/licenses/by/4.0/`) | **Fit** (library tracks only; never his remixes). |
| S5 | **Alexander Nakarada — creatorchords.com** (epic fantasy, medieval, folk, ambient) · [licensing](https://creatorchords.com/licensing-info/) | FACT: CC BY 4.0 unless an extended licence is bought. FACT: the Personal License "covers … games, commercial projects". FACT: Smart Content ID on YouTube/Facebook/Instagram scans descriptions. FACT (via Lead Mate's linked notice, not re-read): do not use the old serpentsoundstudios.com URL. AI status: UNVERIFIED. | `Music: [song] by Alexander Nakarada https://creatorchords.com Licensed under Creative Commons Attribution 4.0 International (CC BY 4.0) https://creativecommons.org/licenses/by/4.0` | **Fit** under CC BY 4.0. |
| S6 | **HeatleyBros** (chiptune/8-bit, game, chill) · [site](https://heatleybros.com/) | FACT: "Free to use with credit"; "credit HeatleyBros with a song link"; "shipping in indie games since 2012". UNVERIFIED: the exact "IN-GAME SONG LINK" wording, whether commercial games still need a paid licence (an older third-party note said so), radio use, formats. | UNVERIFIED (Lead Mate: `Music By HeatleyBros` plus a song or channel link) | **Pending written confirmation.** |
| S7 | **Purple Planet Music** (jazz, funk, world, folk, rock, EDM, orchestral, horror) · [using our music](https://www.purple-planet.com/using-our-music) | UNVERIFIED: the page did not render readable terms for this card. Lead Mate reports free use only for games of ≤5,000 digital copies, a paid licence beyond that, and credit `Music: www.purple-planet.com`. | UNVERIFIED | **Pending** (copy cap would bind a live MMO; licence page not reviewed). |
| S8 | **Pixabay Music** (all genres, many uploaders) · [licence summary](https://pixabay.com/service/license-summary/) | FACT: free, attribution not required, modification allowed. FACT: prohibited to "sell or distribute Content … on a Standalone basis", meaning "no creative effort has been applied … substantially the same form". FACT: "only the full Content License is legally binding" (full text not read: UNVERIFIED). Per-track AI origin and Content ID registration: UNVERIFIED (many uploaders). | Not required (PROPOSAL: credit anyway, per uploader) | **Not for stations** until written confirmation: an unaltered track on a station is close to "standalone". Higher risk. |
| S9 | **Epidemic Sound** (premium, all genres) · [single-track licence PDF](https://www.epidemicsound.com/staticfiles/legacy/20/documents/SingleTrackLicensesV8.pdf) | FACT (single-track licence): "you may not enable or allow any interactivity with the Licensed Work(s). As an example, players of the game may not be allowed to control the playback". FACT: only game versions produced within a year of the licence are covered. Scale/Enterprise plan terms: UNVERIFIED. | `Artist / Title / courtesy of Epidemic Sound` (Lead Mate; UNVERIFIED by this card) | **Blocked** for player-controlled radio under the single-track licence. |
| S10 | **Tabletop Audio** (fantasy/sci-fi ambiences) · [about](https://tabletopaudio.com/about.html) | UNVERIFIED: page not re-read. Lead Mate reports no clear game licence (monetized use needs a negotiated fee or Patreon). | UNVERIFIED | **Blocked** (not licensable for this per Lead Mate). |
| — | FreePD | Site closed (Lead Mate) | — | Excluded. |

Shared legal text: [CC BY 4.0 legal code](https://creativecommons.org/licenses/by/4.0/legalcode.en) (FACT: grants an irrevocable licence; attribution must be given "in any reasonable manner based on the medium", and you must "indicate if You modified the Licensed Material"; it bars adding "downstream restrictions" on the licensed material).

### 1.3 What the terms mean for the design (PROPOSAL)

1. **Start with the CC BY 4.0 composers** (S2–S5). Their terms put no limit on how the music is used inside the game as long as credit is given and modifications are marked.
2. **S1 White Bat Audio** is the named example, and its no-separate-distribution rule is binding on the whole radio (§7.1). Its tracks wait for written confirmation of in-game station use before they enter a station (§7.2). Until then they may be used in non-radio game scenes and the Ra-Thor series with credit, if Core names such a card.
3. **S6, S7, S8** wait for written confirmation (or a reviewed paid licence). **S9 single-track** and **S10** are blocked for stations.
4. Downloads come from the source's own site or its official Bandcamp/paid packs. Never rip from YouTube or other streams (Lead Mate step 4).

---

## 2. Station model

### 2.1 What exists today (FACT)

- Master mute only: `shared/local_settings.rs` L328–L330 (`mute: bool`), applied as `MasterMuteGain` (`client/src/local_settings.rs` L5). There is no music or SFX volume slider.
- Lived music is a quiet generated Peace-yard bed and well sting: `shared/peace_audio.rs` L28–L41 (`BED_ASSET`, `STING_ASSET`, gains 0.04–0.28); `assets/audio/README.md` says the mixer loads only those two files and "A missing file must never block Use or hang boot."
- `assets/config/adaptive_audio.ron` holds ducking biases (`music_ducking_bias: 1.15`, `sfx_ducking_bias: 0.9`, …). Its loader lives in `game/src/audio/adaptive_layering.rs`, and `game/` is a parked crate with no `Cargo.toml` (root `Cargo.toml` members comment). So those biases are **not** live in the first hour.
- There is no Credits screen in the client today (no `credits` UI in `client/src`).

### 2.2 Stations (PROPOSAL)

A station is a named, genre-themed playlist of offline-bundled tracks.

| Field | Meaning |
|---|---|
| `id` | Stable key, e.g. `synthwave` |
| `display_name` | Player-facing copy. **PROPOSED · HELD** until Sherif's OK (§8) |
| `genre` | One genre per station |
| `kind` | `curated` (now) · `procedural` (future, §6.1) · `premium` (future, §6.2) |
| `tracks` | Manifest track ids (§4). A track id may sit on only one station (keeps credits and counts simple) |
| `status` | `live` only when every track is `approved` in the manifest; otherwise the station is hidden |

Playback (PROPOSAL):

- Shuffle with a no-repeat window (the last `min(5, n−1)` tracks are not picked again). Seed per session; nothing is synced between players (Online grey).
- Tracks stream from local disk inside the game's asset pack. No network call, ever, in this design.
- One station plays at a time. Turning the radio on fades the adaptive/Peace bed out over 1.5 s and back in when the radio is off, so two musics never play together.
- A missing or unreadable track is skipped silently with a log line; it never blocks Use, boot or input (same law as `assets/audio/README.md`).
- Tracks play unmodified by default. Any edit (loop cut, fade, EQ beyond normal mixing) is recorded in the manifest's `modified` fields (CC BY 4.0 and incompetech both require modifications to be marked).

### 2.3 Tune-in UI (PROPOSAL)

- **Radio panel**: a small dial strip listing stations by display name, with Prev / Next station, Play/Stop, and a Radio volume slider. Keyboard: no new default binding; reachable from the pause menu and from the radio device once unlocked (unlock is AUDIO-RADIO-QUEST-DESIGN-1).
- **Station switch** plays a 0.4 s soft static burst (a generated sound, not a licensed track) and then the new station.
- **Now-playing toast**: on each new track, a 4 s toast shows the track title and the source's credit line exactly as written in the manifest. This doubles as attribution at the point of use. Reduced-motion setting turns the slide-in into a fade.
- The panel is anchored through the HUD anchor registry proposed in [UI_LAYOUT_SYSTEM](UI_LAYOUT_SYSTEM.md); it adds no free absolute node.

### 2.4 Volume and ducking (PROPOSAL)

- New setting `radio_volume` (0.0–1.0, default 0.6) beside `mute` in `powrush_settings.json`, plus `radio_station` (last station id) and `radio_on` (default false). Master mute silences the radio too.
- Effective gain = `radio_volume × station_trim × duck × (mute ? 0 : 1)`, with `station_trim` from the manifest's per-track `loudness_trim_db` so tracks from different sources sit at a similar level.
- Ducking against world SFX: when a priority sound plays, the radio ducks and recovers.

| Trigger | Duck to | Attack | Release |
|---|---|---|---|
| Combat hit / lethal warning / boss cue | 35 % | 80 ms | 1.2 s |
| Voice line or NPC dialogue | 25 % | 120 ms | 0.8 s after the line ends |
| UI confirm / well sting | 70 % | 40 ms | 0.4 s |
| Pause menu open | 50 % | 200 ms | 0.3 s |

- If the parked `adaptive_audio.ron` biases are revived, `music_ducking_bias` scales these targets; until then the table is the whole rule.

### 2.5 Credits screen (PROPOSAL)

- **Settings → Credits → Music**, also opened by an "i" button on the radio panel.
- Generated from the manifest, never hand-typed. Grouped by source, then by station; every track listed by title with its exact `credit_line` and licence name/URL, and a "modified: …" note where `modified = true`.
- Shows tracks that are in the build, including any not on a station (so non-radio uses are credited too).
- The same generator emits a plain-text `CREDITS-MUSIC.txt` for the store page and the Ra-Thor series end credits (shared pipeline).

---

## 3. Offline bundling (PROPOSAL)

- Files live under a single proposed root, `assets/radio/<station_id>/<track_id>.ogg` (Vorbis, matching `assets/audio/README.md`). **No audio file enters the repo under this card.** The folder is created only by the first named asset card.
- Files ship inside the game's asset pack. There is no "Open music folder", no export, and no file-browser link from the radio UI (§7.1).
- Source masters (WAV/FLAC from the source's own site or paid pack) stay off-repo in the asset store named by that card; only the encoded game file and its hash are tracked.

---

## 4. Licence manifest

One file lists every bundled track. Proposed path: `assets/radio/licence_manifest.toml` (created by the first asset card, not here). TOML is proposed because it is diff-friendly and allows comments. FACT: `shared/Cargo.toml` L18–L19 has `serde` and `serde_json` but no TOML parser, so the asset card either adds a `toml` dependency or ships the same schema as `licence_manifest.json` (Q8).

### 4.1 Schema (PROPOSAL)

`[[source]]` holds per-source policy so tracks inherit the source's rules; `[[track]]` holds per-track facts.

| Table.field | Type | Required | Meaning |
|---|---|---|---|
| `source.id` | string | yes | e.g. `white_bat_audio`, `incompetech` |
| `source.name` | string | yes | Display name |
| `source.licence_id` | string | yes | `CC-BY-4.0`, `WBA-FAQ`, `PIXABAY-CL`, … |
| `source.licence_url` | URL | yes | The source's own licence/FAQ page |
| `source.snapshot_date` | date | yes | ISO date the terms were read |
| `source.snapshot_sha256` | hex | yes | Hash of the saved snapshot text (snapshots kept off-repo with the masters; Q3) |
| `source.credit_template` | string | yes | Exact credit wording with `{title}` placeholder |
| `source.station_use` | enum | yes | `allowed` · `pending_confirmation` · `blocked` |
| `source.confirmation_ref` | string | if `station_use` changed from pending | Pointer to the written confirmation (date + who), never the email text itself |
| `source.required_restrictions` | list | yes | Restrictions every track from this source must carry |
| `track.id` | string | yes | Stable key, `snake_case` |
| `track.title` | string | yes | Title exactly as published |
| `track.source` | string | yes | A `source.id` |
| `track.track_url` | URL | yes | The track's page on the source's own site |
| `track.licence_snapshot_url` | URL | yes | Usually = `source.licence_url`; differs if a track carries its own licence |
| `track.licence_snapshot_date` | date | yes | Date this track's terms were read |
| `track.credit_line` | string | yes | Final rendered credit, shown verbatim in-game |
| `track.allowed_uses` | list | yes | Subset of `game_bundle`, `game_station`, `animation`, `trailer` |
| `track.restrictions` | list | yes | Superset of `source.required_restrictions` |
| `track.station` | string | if `game_station` | Station id |
| `track.file` | path | yes | `assets/radio/...` path |
| `track.file_sha256` | hex | yes | Hash of the shipped file |
| `track.download_date` | date | yes | When the master was fetched |
| `track.acquired_via` | enum | yes | `free_download` · `paid_pack` · `paid_licence` · `commissioned` · `procedural` |
| `track.receipt_ref` | string | if paid | Pointer to the receipt (purchases are Sherif's call) |
| `track.modified` | bool | yes | Any edit beyond mixing gain |
| `track.modification_note` | string | if modified | What changed (shown on the credits screen) |
| `track.loudness_trim_db` | float | no | Per-track level match, default 0 |
| `track.isrc` | string | no | Where the source publishes one (incompetech does) |
| `track.status` | enum | yes | `approved` · `pending` · `blocked` |
| `track.verified` | enum | yes | `VERIFIED` (terms read on the source's own page on `licence_snapshot_date`) · `UNVERIFIED` |

Restriction vocabulary (PROPOSAL; extend only by card):

| Restriction | Meaning |
|---|---|
| `no_standalone_release` | Never sold, given away or distributed apart from the game or film (no OST, no soundtrack DLC, no album) |
| `no_streaming_service_upload` | Never uploaded to audio streaming services |
| `no_player_export` | The game offers no way to save, export or extract the track |
| `no_rebroadcast` | The game never transmits the audio to another device or over a network |
| `no_ai_training` | Never used to train or prompt-condition a music model (conservative house rule for every third-party track) |
| `no_obscene_media` | Not used in obscene or pornographic media (Soundimage licence) |
| `no_remix_use` | Source's remixes are excluded (Scott Buckley) |
| `in_game_credit_required` | Credit must appear inside the game, not only on a store page (Soundimage) |
| `mark_modifications` | Modifications must be stated in the credit (CC BY 4.0; incompetech) |

### 4.2 Example (PROPOSAL; illustrative values, not real entries)

```toml
schema_version = 1

[[source]]
id = "white_bat_audio"
name = "White Bat Audio — Karl Casey"
licence_id = "WBA-FAQ"
licence_url = "https://whitebataudio.com/pages/faq"
snapshot_date = 2026-10-26
snapshot_sha256 = "<sha256 of saved FAQ text>"
credit_template = "Music by Karl Casey @ White Bat Audio"
station_use = "pending_confirmation"
required_restrictions = [
  "no_standalone_release",
  "no_streaming_service_upload",
  "no_player_export",
  "no_rebroadcast",
  "no_ai_training",
]

[[source]]
id = "incompetech"
name = "Kevin MacLeod (incompetech.com)"
licence_id = "CC-BY-4.0"
licence_url = "https://incompetech.com/music/royalty-free/faq.html"
snapshot_date = 2026-10-26
snapshot_sha256 = "<sha256 of saved FAQ text>"
credit_template = "{title} Kevin MacLeod (incompetech.com)\nLicensed under Creative Commons: By Attribution 4.0\nhttps://creativecommons.org/licenses/by/4.0/"
station_use = "allowed"
required_restrictions = ["mark_modifications", "no_player_export", "no_ai_training"]

[[track]]
id = "example_orchestral_01"
title = "<exact published title>"
source = "incompetech"
track_url = "https://incompetech.com/<track page>"
licence_snapshot_url = "https://incompetech.com/music/royalty-free/faq.html"
licence_snapshot_date = 2026-10-26
credit_line = "<exact published title> Kevin MacLeod (incompetech.com)\nLicensed under Creative Commons: By Attribution 4.0\nhttps://creativecommons.org/licenses/by/4.0/"
allowed_uses = ["game_bundle", "game_station", "animation"]
restrictions = ["mark_modifications", "no_player_export", "no_ai_training", "no_rebroadcast"]
station = "orchestral"
file = "assets/radio/orchestral/example_orchestral_01.ogg"
file_sha256 = "<sha256>"
download_date = 2026-10-26
acquired_via = "free_download"
modified = false
loudness_trim_db = -1.5
status = "approved"
verified = "VERIFIED"
```

---

## 5. CI check (designed, not wired)

### 5.1 Where it would live (PROPOSAL)

- A Rust unit test module in `shared` (e.g. `shared/radio_manifest.rs` with `#[cfg(test)]`), so it runs inside the existing `cargo test -p shared` step in `.github/workflows/ci.yml` (FACT: L37–L38) with no new workflow and no new toolchain.
- Alternative (Qn): a small script step in `.github/workflows/validate-full-files.yml`. Rejected for now because that workflow only greps diffs and adding a parser there means a second language.
- **Not wired by this card.** The first asset card that creates `assets/radio/` adds the module. Until then there is nothing to check.

### 5.2 What it validates (PROPOSAL)

Dormant rule: if `assets/radio/` does not exist, the check passes with a note. Once it exists:

1. **Parse**: the manifest parses; `schema_version` is known; every required field (§4.1) is present and non-empty; dates are ISO and not in the future; hashes are 64 hex chars; URLs are `https://`.
2. **File ↔ entry, both ways**: every audio file under `assets/radio/**` has exactly one `[[track]]` with a matching `file`, and every `[[track]].file` exists. Its SHA-256 equals `file_sha256`.
3. **No stray audio**: no `.ogg/.wav/.flac/.mp3/.opus` anywhere in the repo outside `assets/radio/**` and the existing allowlist (`assets/audio/peace_yard_bed.ogg`, `assets/audio/peace_well_sting.ogg`, `assets/music/epic_dark_ambient.ogg`, each generated in-house; Q4).
4. **Source policy inherited**: `track.source` names a `[[source]]`; `track.restrictions ⊇ source.required_restrictions`; the four distribution restrictions (`no_standalone_release`, `no_streaming_service_upload`, `no_player_export`, `no_rebroadcast`) are present on every third-party track.
5. **Credit line**: `credit_line` equals `credit_template` with `{title}` replaced by `title`, byte for byte. Source-specific pins: `white_bat_audio` must contain `Music by Karl Casey @ White Bat Audio`; `soundimage` must contain `Eric Matyas` and `www.soundimage.org`; `creatorchords` must contain `https://creatorchords.com` and must not contain `serpentsoundstudios`.
6. **Station gate**: a track with `game_station` in `allowed_uses` needs `source.station_use = "allowed"` (or a `confirmation_ref`). `blocked` sources (Epidemic single-track, Tabletop Audio) fail outright. A station is `live` only if all its tracks are `approved`.
7. **Status**: every shipped file's track is `status = "approved"` and `verified = "VERIFIED"`. `UNVERIFIED` tracks may sit in the manifest but not ship.
8. **Modifications**: `modified = true` requires a non-empty `modification_note`; sources with `mark_modifications` show it in the generated credits.
9. **Freshness (warning only)**: a `snapshot_date` older than 365 days prints a warning asking for a re-read; it never fails the build.
10. **Credits parity**: the generated Music credits list (§2.5) has exactly one line per shipped track.

The check cannot judge whether a use is lawful. It only proves the manifest is complete, consistent and matches the files.

---

## 6. Future station slots

### 6.1 Procedural stations (PROPOSAL, future)

- `kind = "procedural"` stations play music generated on the player's machine (e.g. a seeded synth; `client/src/fundsp_audio.rs` exists on disk but is not a module in `client/src/lib.rs` and `fundsp` is not in `client/Cargo.toml`, so it is not live) or pre-rendered from an AI music tool and bundled like any track.
- Each procedural generator gets a `[[source]]` entry with `acquired_via = "procedural"`, its own licence terms (tool terms of service, ownership of output, cost), checked before any output ships, per the councils' 2026-10-14 audio decision.
- Prompts for any AI tool **never name real artists or tracks**; they describe mood, tempo and instruments only. No third-party track is ever used as a prompt reference or training input (`no_ai_training`).
- Rendered procedural tracks still get manifest entries and hashes, so the CI check covers them the same way.

### 6.2 Premium stations (PROPOSAL, future; Sherif's call)

- `kind = "premium"` is reserved. Whether premium exists at all, what it costs and which catalogue it uses are Sherif's call. No purchase is proposed here.
- A premium station may only carry tracks whose licence explicitly allows player-controlled in-game playback inside a paid feature (Epidemic's single-track licence does not, §1.2 S9), or original/commissioned music the studio owns.
- Free-with-credit tracks (S1–S8) are never placed behind a paywall as a "music pack": that is too close to selling the music separately (§7.1). Q5.

---

## 7. Terms the design must honour

### 7.1 The radio is never a standalone release (rule)

From White Bat Audio's FAQ (FACT: "the soundtrack CANNOT be sold or distributed separately from the film or game"), Scott Buckley's refusal of streaming-service albums (FACT), and Pixabay's "Standalone basis" ban (FACT), the design forbids, for every station and every source:

- re-streaming tracks as a standalone release: no web player, no public stream URL, no podcast feed, no streaming-service playlist or upload;
- selling or giving away a soundtrack, OST, "radio pack" DLC or album containing third-party tracks;
- an export, download, "save track" or "open folder" feature in the game;
- a desktop/background "radio only" mode that plays music without the game running;
- transmitting radio audio between players. When Online returns, each client still plays its own local files; at most the station id may be shared (online later).

Streamers capturing gameplay with the radio on are outside what the game controls; the credits screen and `CREDITS-MUSIC.txt` give them the credit lines (YouTube description credit is required by S1, S4, S5 per their pages).

### 7.2 Pending sources

`station_use = "pending_confirmation"` sources (S1 White Bat Audio, S6 HeatleyBros, S7 Purple Planet, S8 Pixabay) stay off stations until a written confirmation for "continuous in-game radio playback inside a commercial MMO" is recorded by `confirmation_ref`. Sending that request is an outbound message and therefore Sherif's call. If confirmation never comes, the station's genre is filled from S2–S5 instead.

### 7.3 Shared pipeline

The same manifest and credits generator serve the Ra-Thor⚡️Living Thunder series. A track gets `animation` in `allowed_uses` only where the source names film/animation use (S1 FACT; S2–S5 under CC BY 4.0). Series credits use the same `credit_line` verbatim.

---

## 8. Station names — creative-stuck ranking (PROPOSED · HELD for Sherif's OK)

Method (creative-stuck idea recycling): for each genre, several seeds were drafted, then ranked on (1) a hidden-in-plain-sight pun or double meaning, (2) a link to Powrush lore or systems already written (Draeks, Cydruid hollows, TAUN, Crownstone, RBE), (3) short enough for a dial strip (≤ 18 characters), (4) no real artist, song, album, brand or existing station name. Default is HOLD: no name ships until the council votes and Sherif says OK. Names have **not** been screened against trademarks or existing song titles (Q6).

The fictional frequencies are decoration; there is no real broadcast.

| Genre (likely sources) | Rank | Name (PROPOSED) | Why |
|---|---|---|---|
| **Synthwave / darksynth** (S1 once confirmed) | 1 | **Retro-Grade 88.8** | "Retrograde" (looking backwards, the genre's whole mood) hidden in a frequency-style name. |
| | 2 | **Neon Hollows FM** | The Cydruids' underground hollows lit like an 80s grid; cyborgs who kept their hearts. |
| | 3 | **Chrome Druid Drive** | Cydruid machine-nature mix plus the night-drive feel. Alliterative but less of a joke. |
| **Cinematic orchestral / epic** (S2, S4) | 1 | **TAUN Hall** | "Town hall" on the TAUN mothership: grand, civic, a place everyone gathers. |
| | 2 | **Epic-Centre** | Epicentre / epic: where the big feelings start. |
| | 3 | **Overture Drive** | "Overdrive" with an overture in it. |
| **Epic fantasy / medieval folk** (S5, S2) | 1 | **The Lute Box** | "Loot box" — except the only thing inside is a lute. A wink at a game with no loot-box mall. |
| | 2 | **Jester Minute** | "Just a minute" said by a court jester: short tunes, quick laughs. |
| | 3 | **Ye Olde Wavelength** | Mock-medieval radio. Safe, mild. |
| **Chiptune / 8-bit** (S3, S6 once confirmed) | 1 | **Chip on the Shoulder** | Chiptune + the idiom for a grudge; fits a world at war. |
| | 2 | **Low-Res-olution** | Low resolution hidden in "resolution" (as in resolve). |
| | 3 | **Bleep Talk** | "Sweet talk" / "cheap talk" in bleeps. |
| **Horror / dark ambient** (S2, S3, S1 horror once confirmed) | 1 | **Draek Matter** | "Dark matter" with the Draeks hidden in it: unseen, heavy, everywhere. |
| | 2 | **Static Cling** | Radio static plus dread that will not let go. |
| | 3 | **Hivelord Lullabies** | The Hivelord singing you to sleep; creepy by contrast. |
| **Jazz / lounge / funk** (S2) | 1 | **Brass Tax** | "Brass tacks" — and in an RBE world the only tax is a horn section. |
| | 2 | **Swing Shift** | Swing music for the people working the harvest and haul shifts. |
| | 3 | **Cool Fusion** | "Cold fusion" plus jazz fusion; a joke for the Engineering crowd. |
| **Sci-fi ambient / game loops** (S3, S4) | 1 | **Space Cadence** | "Space cadet" with a musical cadence inside. |
| | 2 | **Ambient Occlusion** | A graphics term reused as an ambient station name; a nerd joke. |
| | 3 | **Orbit-rary** | "Arbitrary" orbiting: drifting, no fixed destination. |
| **Chill / lo-fi** (S6 once confirmed, S3) | 1 | **Low-Fly** | Lo-fi plus ships flying low and slow over the yard. |
| | 2 | **Crownstone Cool-Down** | A place to cool down near the Crownstone. Copy must stay free of health claims. |
| | 3 | **Idle Hands FM** | Idle hands (a jab at idle play) in a cosy wrapper. |

Copy rules for every name and blurb: no health claims, no real artists or tracks, no faction slurs, English only for now (localisation follows the existing translations flow).

---

## 9. Open questions

- **Q1** — Which genres launch first? Proposal: orchestral, fantasy folk, sci-fi ambient, horror (all coverable by S2–S5 today); synthwave waits for S1 confirmation.
- **Q2** — Does Sherif want the S1/S6/S7/S8 confirmation emails sent? (Outbound: his call; draft text not included in this card.)
- **Q3** — Where are licence snapshots and source masters stored off-repo (asset store, Drive folder)? The manifest only holds hashes and pointers.
- **Q4** — Should the three in-house generated `.ogg` files get manifest entries too (`acquired_via = "procedural"`) instead of an allowlist?
- **Q5** — Is a premium station wanted at all, and with which catalogue? (Sherif's call.)
- **Q6** — Station-name screening against trademarks and existing song/station titles before Sherif's OK (the innovations-generator `screen-name` heuristic is English word-list only and is not trademark clearance).
- **Q7** — Default `radio_volume` and duck targets need a playtest pass; the numbers in §2.4 are starting points.
- **Q8** — Manifest format: TOML (adds a `toml` dependency to `shared`) or JSON (uses the existing `serde_json`)? Same schema either way.

## 10. Next

- AUDIO-RADIO-QUEST-DESIGN-1 — the radio's quest unlock (combat or Engineering path, account-bound). Separate card; not designed here.
- Code and asset slices only when Core names them.
