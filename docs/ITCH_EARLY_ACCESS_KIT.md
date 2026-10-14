# ITCH-EARLY-ACCESS-KIT-1

**Contact:** info@Rathor.ai  
**Tip this kit was written against:** `ad77140d` on `main`. The file a buyer downloads is whatever green `offline-build` run Sherif uploads, named by that run's 8-character sha.  
**Seat:** docs only. One new file. No `client/**`, `shared/**`, or workflow edits.

Sherif publishes the itch.io page. This file is the copy and the checklist. An agent does not create the account, does not upload, and does not press publish.

Paste sections marked **Paste** onto the page. Leave the rest in this repo.

---

## 1. Store page copy

### Tagline (Paste — itch short description)

Early access. Offline single-player for Windows and Linux. Many worlds, five peoples. Online later.

### Short pitch (Paste — opening lines)

Powrush is a universe of many worlds and many peoples. This page is the early-access download: one player, one computer, offline, for Windows and Linux. You walk a yard, tend a well, and keep what the yard remembers. Online play is later. It is not in this file.

### Long description (Paste)

Powrush is a universe of many worlds and many peoples.

The peoples are Humans. Draek are tall plated humanoids. The plate is hard chitin, lit violet at the seams. They are never reptiles. Quellorians are people of resonance. Ambrosians are crystalline-organic people. Cydruids are humans who wear a cyborg frame. A Cydruid is never a tree. The freed-Draek leader in that universe is Vrendak the Unbound. He held the Dominion's harvest office, broke his own Crownstone link, and left with his own voice. His camp is written lore. This download does not put him on screen, and his camp is not one of the places in the build.

What you are buying is an early-access prototype. It is single-player and offline. The title screen leaves Online off. There is no account and no public server. A later online game would be a different release. This build has no crypto, no NFT, and no real-money item.

Play starts in Sanctuary, a valley around a well. Continue returns you to a House name, or to an Unnamed House, when the yard already remembers you. You walk, jump, and sprint. You tend or take at the well. The well tells you when it is idle, glowing, tended, resting, or stressed. You open a satchel, hide the guidance line, and allocate what you gathered: flow, or reserve. Saves stay on your machine.

Stay with the same build and a further stretch opens: a frontier, a House stake, and a ledger. The game can mark a second hour, then a third, and hand you a book. After that book, the pause menu lists four places: Sanctuary, Heartwood, Threshold, and Depths.

During play you can cycle five lineages. You start as Human. The cycle is Human, Cydruid, Quellorian, Draek, Ambrosian. The body tint changes, and a tend or a take feels a little different. The title has no race-select screen. Lines on the loading screen name the five peoples, including Draek plate as hard chitin. Those lines are not a character creator, and they are not a meeting with Vrendak.

Sound can be muted. A Grove light stays off unless you turn it on. A LAN switch can stay off, or open a loopback lab on the same computer. Loopback does not turn Online on.

The packaged file says, in its own README: offline build only, Online grey and disabled, a prototype CI artifact.

### Feature bullets (Paste — only what this repo's offline build does)

- Offline single-player for Windows and Linux. No account. The title's Online line stays off.
- Title buttons: Play, Continue, Settings. Continue keeps a House name, or Unnamed House, and the yard remembers.
- Walk, jump, and sprint. Tend or harvest the well. Open a satchel. Hide guidance. Allocate into flow or reserve.
- The well speaks Idle, Glowing, Tended, Resting, or Stressed.
- Saves stay on the computer: House, settings, and place files in a user folder. Quit and return.
- After you allocate, the same build opens a frontier, a House stake, and a ledger Bind. It can mark hour two held, then hour three held.
- After the book, four places: Sanctuary, Heartwood, Threshold, Depths.
- Lineage cycle on the C key, starting as Human: Human, Cydruid, Quellorian, Draek, Ambrosian. Body tint, plus a small change in how tend or take feels.
- Loading lines name those five peoples. Draek plate is named there as hard chitin.
- Settings: mute, Grove light off by default, LAN off or a same-machine loopback lab.

### Keys (Paste)

| Key | Action |
|---|---|
| W A S D | Walk |
| Space | Jump |
| Shift | Sprint |
| E | Tend or harvest the well |
| I | Satchel |
| H | Hide guidance |
| R, then 1 or 2 | Allocate: 1 flow, 2 reserve |
| Tab, then Q, then L | After allocate: frontier, House stake, ledger Bind |
| C | Cycle lineage |
| Esc | Pause |

### Buyer launch notes (Paste)

Windows: unzip, and run `powrush-client.exe` from the folder that contains `assets/`.

Linux: unzip, then in that same folder run `tar xzf powrush-offline-linux-<sha8>.tar.gz` and `./powrush-client`. The `<sha8>` matches the file name. Linux needs `libxkbcommon-x11-0`, `libasound2t64`, `libudev1`, `libvulkan1`, `libxcursor1`, `libxi6`, and a Vulkan driver such as `mesa-vulkan-drivers`.

### Leave off the page

Do not claim other players, matchmaking, a public server, a title-screen race select, a market, or a scene with Vrendak. Do not claim crypto, NFTs, token sales, or real-money trade inside the game. Do not use the old press-kit list (clans, Steam achievements, global events). Those sentences are not this download. The first keys are peace. A later opt-in harm path exists in the same build. It is not the default, and it is not the pitch.

---

## 2. Price

Two options. One recommendation.

### Option A — Pay what you want, minimum $5 USD

The buyer types a price. Itch refuses less than $5. They may pay more.

### Option B — Fixed $10 USD

One number. No box to pay more.

### Recommendation

Use **option A: pay what you want, minimum $5 USD.**

The workflow that builds the zip calls it a prototype CI artifact. The repo README calls this tree not a launch candidate. The file is an offline single-player early hour. Online is a later release, and this price does not include it.

A fixed $10 tells a buyer the page is a finished game with one value. This download is the first offline stretch of a larger universe, with Online still off.

Pay what you want with no minimum lets most people pay nothing. The page then reads as a free demo, and a later paid file is harder to explain.

$5 is a real purchase, and it is low enough that a missing online mode is not folded into the price. A buyer who wants to support the universe pays more on the same button. Itch pays Sherif through the payout account in section 4. The game has no wallet and no real-money item.

If he would rather show one number and skip the name-your-price box, use option B at $10, and keep "Early access" as the first words on the page so the fixed price does not imply a finished online game.

---

## 3. Packaging checklist

Workflow file: [`.github/workflows/offline-build.yml`](../.github/workflows/offline-build.yml).  
Workflow name: `offline-build`.

It runs on a push to `main`, on `workflow_dispatch`, and on a pull request only when that workflow file itself changes. An ordinary docs pull request does not build it. Upload a green run from `main`.

The job matrix is `ubuntu-latest` and `windows-latest`. Each leg runs `cargo build --release -p powrush-client` with default features, then packs a dist folder. Artifact retention is **14 days**.

| Artifact name | What is inside |
|---|---|
| `powrush-offline-windows-<sha8>` | `powrush-client.exe`, `assets/`, `README.txt` |
| `powrush-offline-linux-<sha8>` | `powrush-offline-linux-<sha8>.tar.gz` (binary only), `assets/`, `README.txt` |

`<sha8>` is the first 8 characters of the commit that built the run.

`README.txt` in both artifacts begins:

```
Offline build only.
Online is grey and disabled.
This is a prototype CI artifact, not a release.
```

The Linux paragraph names the extract command and the library list in section 1. The Windows paragraph says to run `powrush-client.exe` next to `assets/`.

A `clean-boot` job on the Linux artifact must be green before the zip is a candidate. It boots the packaged Linux binary under a 25-second timeout and expects the process still alive.

Checklist:

1. Open GitHub Actions for this repo. Select the `offline-build` workflow. Open a run on `main` whose `build` legs and `clean-boot` job are green.
2. Read the artifact names and copy the 8-character sha. That sha is the build id to type on the itch page.
3. Download both artifacts before day 14. GitHub delivers each as a zip. The files sit at the root of that zip: binary (or Linux tar), `assets/`, `README.txt`.
4. Unzip once locally and confirm the Windows zip has `powrush-client.exe` beside `assets/`, and the Linux zip has `powrush-offline-linux-<sha8>.tar.gz` beside `assets/`. Confirm `README.txt` still has the three prototype lines.
5. Upload those zips as they came from Actions. Re-zip only if a check forced an unpack. Upload the whole Linux zip. The inner `.tar.gz` has the binary alone and will ship without `assets/` if it is uploaded by itself.
6. Do not rebuild by hand for the page, do not swap in a debug binary, and do not set a listen or a public bind. `POWRUSH_NET` stays off.
7. Put the Linux library line from section 1 on the page, next to the Linux file.
8. The workflow does not open a GitHub Release. The itch upload is the publish step, and section 4 is who presses it.

---

## 4. Steps Sherif does himself

No agent, bot, or collaborator does these.

1. Create the itch.io account that will own the page, or sign in to the one he already owns. Use his own email and password. Do not give that login to an agent.
2. In the itch dashboard, open payouts. Connect the payout method itch offers (Stripe or PayPal). Complete the tax form itch shows. Keep bank details and tax numbers out of this repo and out of agent chat.
3. Set itch's revenue-share slider himself. This kit does not pick the share.
4. Create a new project. Kind: downloadable game. Title: Powrush. Paste the tagline, short pitch, long description, feature bullets, keys, and buyer launch notes from section 1.
5. Tag the project `early-access`, and keep the words "Early access" in the first line of the description.
6. Set the price to section 2. Recommended: pay what you want, minimum $5 USD.
7. Download the Windows and Linux zips in section 3 from a green `offline-build` run on `main`.
8. Upload the Windows zip and mark it Windows. Upload the Linux zip and mark it Linux.
9. Open each uploaded zip far enough to see `README.txt`, and confirm the offline / Online-grey / prototype lines are still there.
10. Add a cover image he is willing to show in public. This kit has no image.
11. If itch asks a content question: default play is tending a well. A later opt-in harm path exists. There is no crypto, no NFT, and no real-money item in the game.
12. Leave the project draft until he has launched the uploaded Windows or Linux file and seen the title (Play / Continue / Settings, Online off).
13. Publish the page himself. An agent does not press publish.
14. When a newer `main` build is the one he wants on the page, repeat steps 7–9, upload the new files, and write the devlog himself.

---

## Sources (do not paste)

Lived hour and keys: `README.md`, `docs/STRANGER_LOOP.md`.  
Peoples and the never-reptile / never-tree lines: `docs/PLAYABLE_RACES.md`, `client/src/loading_lines.rs`, `client/src/player_lineage.rs` (C cycles lineage; the plugin is on the default client).  
Vrendak the Unbound: `docs/design/FACTION_LEADERS.md` §8. His name is not in the client Rust tree.  
Offline package: `.github/workflows/offline-build.yml`.  
SKU: Title Online stays grey. `docs/OFFLINE_SKU.md`.
