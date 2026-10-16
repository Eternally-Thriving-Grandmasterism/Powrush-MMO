# ITCH-EARLY-ACCESS-KIT-1

**Contact:** info@Rathor.ai  
**Checked against:** `main` `a2ca9aac`.  
**Seat:** docs only. One file. No `client/**`, `shared/**`, or workflow edits.

Sherif publishes the itch.io page. This file is the copy and the checklist. An agent does not create the account, does not upload, and does not press publish.

Paste sections marked **Paste** onto the page. Leave the rest in this repo.

---

## 1. Store page copy

### Tagline (Paste — itch short description)

Early access. Offline single-player for Windows and Linux. Many worlds and peoples. Online later.

### Short pitch (Paste — opening lines)

Powrush is a universe of many worlds and many peoples. This page is an early-access download: one player, one computer, offline, for Windows and Linux. You walk a yard, tend a well, and keep what the yard remembers. Online stays off. It is an offline prototype.

### Long description (Paste)

Powrush is a universe of many worlds and many peoples.

The peoples are Humans. Draek are tall plated humanoids. The plate is hard chitin, lit violet at the seams. They are never reptiles. Quellorians are people of resonance. Ambrosians are crystalline-organic people. Cydruids are humans who wear a cyborg frame. A Cydruid is never a tree.

The peaceful alliance in that universe is the Quellorian / Aetherion Luminari Alliance. It stands against the Hivelord's Draek Dominion. It is open to every peaceful people promoting TOLC — truth, order, love, compassion, service, abundance, joy, and cosmic harmony — including Draeks who rebelled against that Dominion. Vrendak the Unbound is the freed-Draek leader of that break. He held the Dominion's harvest office, broke his own Crownstone link, and left with his own voice. This download does not put him, the Alliance, or the Dominion on screen.

What you are buying is an early-access offline prototype. It is single-player. The title screen leaves Online off. There is no account and no public server. A later online game would be a different release.

Play starts in Sanctuary, a valley around a well. Continue returns you to a House name, or to an Unnamed House, when the yard already remembers you. You walk, jump, and sprint. By default, A and D strafe with the view. Settings also offers a Classic move preset, where A and D turn you and W and S follow the way you face. You tend or take at the well. The well tells you when it is idle, glowing, tended, resting, or stressed. You open a satchel, hide the guidance line, and allocate what you gathered: flow, or reserve. Saves stay on your machine.

You can look around by holding the right mouse button. Mouse look is a separate option in Settings. It stays off until you turn it on. The camera pulls back from walls, so it does not pass into them.

In Settings, on the Controls tab, you can give an action a new key. If that key is already used, the row tells you which action has it. Under Reset keys, a line lists key conflicts, or says there are none.

Stay with the same download and a further stretch opens: a frontier, a House, and a ledger. The game can mark a later hour as held, and hand you a book. After that book, pause can open other places in this world.

During play you can cycle lineage. You start as Human, then Cydruid, Quellorian, Draek, and Ambrosian, and back around. The body tint changes, and a tend or a take feels a little different. The title has no race-select screen. Lines on the loading screen name these peoples, including Draek plate as hard chitin. Those lines are not a character creator, and they are not a meeting with Vrendak.

Sound can be muted. Scenery stays off unless you turn it on.

### Feature bullets (Paste — only what this offline prototype does)

- Offline single-player for Windows and Linux. No account. The title's Online line stays off.
- Title buttons: Play, Continue, Settings. Continue keeps a House name, or Unnamed House, and the yard remembers.
- Walk, jump, and sprint. By default, A and D strafe. A Classic move preset turns with A and D instead.
- Hold the right mouse button to look around. Mouse look is optional and off until you turn it on.
- The camera pulls back from walls, so it does not pass into them.
- Controls in Settings: rebind keys. A taken key is refused, and a line under Reset keys warns about key conflicts.
- Tend or harvest the well. Open a satchel. Hide guidance. Allocate into flow or reserve.
- The well speaks Idle, Glowing, Tended, Resting, or Stressed.
- Saves stay on the computer. Quit and return. The yard remembers.
- After you allocate, the same download opens a frontier, a House, and a ledger. It can mark a later hour as held.
- After the book, pause can open other places in this world.
- Lineage cycle on the C key, starting as Human: Human, Cydruid, Quellorian, Draek, Ambrosian. Body tint, plus a small change in how tend or take feels.
- Loading lines name these peoples. Draek plate is named there as hard chitin.
- Settings can mute sound. Scenery stays off unless you turn it on.

### Keys (Paste)

| Key | Action |
|---|---|
| W / S | Walk forward / back |
| A / D | Strafe, in the default Modern move preset |
| Classic move preset | A / D turn you; W / S follow the way you face |
| Space | Jump |
| Shift | Sprint |
| Right mouse button | Look around |
| Mouse look | Optional. Off until you turn it on in Settings |
| E | Tend or harvest the well |
| I | Satchel |
| H | Hide guidance |
| R, then 1 or 2 | Allocate: 1 flow, 2 reserve |
| Tab, then Q, then L | After allocate: frontier, House, ledger |
| C | Cycle lineage |
| Esc | Pause. Controls is where keys are changed |

### Buyer launch notes (Paste)

Windows: unzip, and run `powrush-client.exe` from the folder that contains `assets/`.

Linux: unzip, then in that same folder unpack the Linux archive and run `powrush-client`. Linux needs `libxkbcommon-x11-0`, `libasound2t64`, `libudev1`, `libvulkan1`, `libxcursor1`, `libxi6`, and a Vulkan driver such as `mesa-vulkan-drivers`.

### Leave off the page

Do not count places, and do not list them as a closed set. Do not claim other players, matchmaking, or a public server. Do not claim a title-screen race select, or a scene with Vrendak, the Alliance, or the Dominion. Do not mention crypto, NFTs, or AGSi. Do not mention Steam. The first keys are peace. A later opt-in harm path exists in the same download. It is not the pitch. Online stays off.

---

## 2. Price

Sherif chooses. This kit does not recommend either option.

### Option A — Pay what you want, minimum $5 USD

The buyer types a price. The page refuses less than $5. The buyer may pay more.

### Option B — Fixed $10 USD

One number. No box to pay more.

---

## 3. Packaging checklist

Not for the itch page. Workflow file: `.github/workflows/offline-build.yml`. Workflow name: `offline-build`.

It runs on a push to `main`, on a manual run, and on a pull request only when that workflow file itself changes. An ordinary docs pull request does not build it. Upload a green run from `main`.

The job matrix is Ubuntu and Windows. Each leg builds the release client, then packs a folder. The packed files are kept for 14 days.

| Download name | What is inside |
|---|---|
| `powrush-offline-windows-<short>` | `powrush-client.exe`, `assets/`, `README.txt` |
| `powrush-offline-linux-<short>` | a Linux archive of the program only, plus `assets/`, plus `README.txt` |

`<short>` is the first 8 characters of the commit that built the run.

`README.txt` in both downloads begins:

```
Offline build only.
Online is grey and disabled.
This is a prototype CI artifact, not a release.
```

Leave that wording inside the download. Do not paste "CI" onto the itch page.

The Linux paragraph names the extract command and the library list in section 1. The Windows paragraph says to run `powrush-client.exe` next to `assets/`.

A later job on the Linux download must be green before the zip is a candidate. It starts the packaged Linux program and expects it still running after a short wait.

Checklist:

1. Open the Actions tab for this repo. Select the `offline-build` workflow. Open a run on `main` whose build legs and the later Linux boot check are green.
2. Read the download names and copy the short code. That code identifies the files. Do not print it as jargon on the itch page.
3. Download both before day 14. Each arrives as a zip. The files sit at the root: the program (or the Linux archive), `assets/`, `README.txt`.
4. Unzip once locally and confirm the Windows zip has `powrush-client.exe` beside `assets/`, and the Linux zip has the Linux archive beside `assets/`. Confirm `README.txt` still has the three lines above.
5. Upload those zips as they arrived. Re-zip only if a check forced an unpack. Upload the whole Linux zip. The inner archive has the program alone and will ship without `assets/` if it is uploaded by itself.
6. Do not rebuild by hand for the page, and do not swap in a different program. Do not turn Online on.
7. Put the Linux library line from section 1 on the page, next to the Linux file.
8. This workflow does not open a release on the repo. The itch upload is the publish step, and section 4 is who presses it.

---

## 4. Steps Sherif does himself

No agent, bot, or collaborator does these. No one publishes, uploads, or takes payment from this seat.

1. Create the itch.io account that will own the page, or sign in to the one he already owns. Use his own email and password. Do not give that login to an agent.
2. In the itch dashboard, open payouts. Connect the payout method itch offers. Complete the tax form itch shows. Keep bank details and tax numbers out of this repo and out of agent chat.
3. Set itch's revenue-share slider himself. This kit does not pick the share.
4. Create a new project. Kind: downloadable game. Title: Powrush. Paste the tagline, short pitch, long description, feature bullets, keys, and buyer launch notes from section 1.
5. Tag the project early access, and keep the words "Early access" in the first line of the description.
6. Choose option A or option B in section 2 and set that price. This kit does not choose.
7. Download the Windows and Linux zips in section 3 from a green `offline-build` run on `main`.
8. Upload the Windows zip and mark it Windows. Upload the Linux zip and mark it Linux.
9. Open each uploaded zip far enough to see `README.txt`, and confirm the offline / Online-off lines are still there.
10. Add a cover image he is willing to show in public. This kit has no image.
11. If itch asks a content question: default play is tending a well. A later opt-in harm path exists. Do not mention crypto, NFTs, or AGSi.
12. Leave the project draft until he has launched the uploaded Windows or Linux file and seen the title (Play / Continue / Settings, Online off).
13. Publish the page himself. An agent does not press publish.
14. When a newer `main` file is the one he wants on the page, repeat steps 7–9, upload the new files, and write the devlog himself.

---

## Sources (do not paste)

There is no `docs/lore` directory. Alliance wording is from `docs/FACTIONS_OVERVIEW.md` (the Quellorian / Aetherion Luminari Alliance against the Draek Dominion, led by the Hivelord) and `docs/design/FACTION_LEADERS.md` (Vrendak the Unbound; Solmaris wants redeemed Draeks who left the Dominion inside the mercy the Alliance already speaks for, and Alliance crews do not trust them yet). TOLC's eight words are in `docs/PERFECTION_PHILOSOPHY.md`. The page line that the Alliance is open to every peaceful people promoting TOLC, including rebel Draeks, is the steward's sentence for this kit, using those names.

Lived hour and keys: `README.md`, `docs/STRANGER_LOOP.md`.  
Peoples: `docs/PLAYABLE_RACES.md`. Draek plate and the lineage cycle are in the client. Vrendak's name is not in the client.  
Move preset: Modern is the default and its hint is "A/D strafe". Classic is the other move preset and its hint is "A/D turn".  
Controls: a taken key shows "taken by …". The row under Reset keys reads "Key conflicts · …".  
Mouse look: a Settings row, default off. Holding the right mouse button looks around without that option.  
Camera: the follow camera pulls in when a wall is closer, so it does not sit inside the wall.  
Scenery is the settings name for the light that stays off.  
Offline package: `.github/workflows/offline-build.yml`. Title Online stays grey.
