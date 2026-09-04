# SECOND_HOUR_CHARTER_FRONTIER.md

Design tick: 23.2.19 companion  
Unlock only after first-hour playtest is green (`docs/FIRST_HOUR_PLAYTEST.md`).

## Law

Second hour is **Charter → Frontier only**.
Embassy, Crownstone, Sylvaris, Hybrid, Compass, War week stay locked.

Peace visitor keys stay WASD / E / I / H / R.
New keys **Tab / G / L / Q** stay dead until `charter_id` exists **and** the human is on a Frontier hex.

## Beat sheet (45–70 minutes)

### Beat 1 — Charter (8–12 min)

Goal: the human founds a charter without a second HUD lecture.

- Trigger: after first successful Flow allocation in Peace.
- One card: name the charter. No faction picker.
- Persist `charter_id` through `rsil-identity` only. Peace live W stays 0.
- Fail if this beat asks for email, wallet, Steam, or a council vote.

Done when: a charter_id exists and Peace harvest still works.

### Beat 2 — Cross the flag (5 min)

Goal: teach hex flags by walking, not by menu.

- Peace hex: E still tends.
- Frontier hex as Peace visitor: E says `Not your charter / Peace visitor`.
- No combat. W stays walk.

Done when: the human can walk back to Peace and tend again.

### Beat 3 — Frontier yard witness (10–15 min)

Goal: see an Offline extractor with spill. Do not attack it.

- Existing I2 witness / `infra_spill` evidence pack is enough.
- E on the slab reads life + pack hash.
- Teaching sentence: “This machine is tired. Repair is later.”

Done when: the human has seen spill and has not been given an attack key.

### Beat 4 — House tutorial, then stop the factory (12–18 min)

Goal: Q founds a local House on Frontier.

Order only:

1. Q founds
2. extractor → depot → hauler
3. two stops
4. arrival chime: *The machine exists*

Do **not** open Ledger, Voice, Embassy, or Proof Pack in this hour.
If Q content is not stable, skip Beat 4 and end the hour after Beat 3.

Done when: one local machine chain has run once.

### Beat 5 — Return to Peace (5 min)

Goal: prove the hour did not eat the first hour.

- Walk back to Peace.
- E tends. I still shows satchel. R still allocates.
- Frontier keys go dead again.

Done when: first-hour verbs are intact.

## Explicit freeze

Do not implement in second hour:

- G Voice quorum
- L Ledger Bind / Escort
- Q fabricator Proof Pack
- Tab War week
- Embassy seat
- Crownstone Witness
- DeclaredLethal

Those already have later slice notes in CHANGELOG 23.2.8–23.2.18. They wait.

## Acceptance

A stranger can finish Charter → Frontier without reading Ra-Thor and can say:

“Peace is tend and allocate. Frontier is a yard I do not own yet.”

## Tests to add later (not in this paste)

- Peace visitor E on Frontier returns the visitor string
- charter_id missing ⇒ Tab/G/L/Q no-op
- After Beat 5, `LivedHour` JSON still loads
