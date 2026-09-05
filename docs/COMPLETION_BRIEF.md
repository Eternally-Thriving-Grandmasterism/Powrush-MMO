# COMPLETION_BRIEF.md — Powrush-MMO

Status date: 2026-09-05  
Canonical product: lived first hour (`cargo run -p powrush-client`)  
Workspace: 21.88.0  
Design tick: 23.2.29 (not a Cargo bump)  
This file is an instruction document for manual, non-agentic work.

## Product law

1. Hands beat lore. If a system cannot be felt with WASD / E / I / H / R in the first hour, it is parked.
2. One human, one machine. No dedicated server in the default binary.
3. Ra-Thor may read `data/powrush_lived_tick.json`. It does not drive keys.
4. Keep constellation repos separate:
   - This repo = feel
   - Ra-Thor = policy lattice
   - Powrush-MMO-Simulator = browser view
5. Older “100% launch worthy” documents are historical. They do not override README.

## Done when first hour is complete

A stranger can:

- Build and run without a Ra-Thor checkout
- Walk a climate and find a glow without a second HUD
- Tend with E and feel glow + camera punch + first-take rumble
- Open satchel with I and see what was taken
- Allocate with R (1 flow, 2 reserve) and see the field change
- Hide guidance with H and still know what to do from world feedback
- Quit and resume without losing satchel + allocation
- Finish in about 40–70 minutes without reading governance docs

## Explicitly not first hour

NFT / chain mint, k8s, payments, P2W, Steam-as-blocker, second HUD, corpse-grey, 12-player lobby, parked `server/` / `simulation/` / `host/` / `game/` / `powrush-divine-module/` as required deps.

## Workstreams (do in order)

### A. Playable hour (highest fruit)

- [x] Single onboarding card: walk, tend, satchel, allocate
- [x] Resource node states: idle / glowing / tended / resting / stressed
- [x] Allocation consequence visible in 30 seconds (flow grows shared field; reserve holds repair-rights)
- [x] Save / load of satchel + node states + allocation (`data/powrush_lived_tick.json`)
- [ ] Accessibility: toggle guidance, readable contrast, remappable keys (H + contrast landed; E/I/R sealed in harvest_feel / rbe_allocate_choice)
- [x] Crash-safe tick write to `data/powrush_lived_tick.json`

### B. Proof the teaching claim

- [x] One scripted climate where mercy-tending restores a node
- [x] One scripted climate where extract-only leaves a resting/stressed node
- [x] On-screen teaching is a sentence, not a manifesto
- [x] Automated lib tests for node state transitions in `shared` or `powrush-client --lib`

### C. Second hour only after A+B

Unlock behind Charter, in this order only:

1. [x] Frontier yard + Offline extractor witness — Tab after allocate steps the ridge; Q founds House; I2 spill speaks (23.2.28)
2. [x] Ledger bind + escort — L then E after skin is live (already in 23.2.10; door now reaches it)
3. [x] Persist + Settled checkpoint — `HourTwoPack` in `data/powrush_hour_two.json`; slab *Hour two held · the yard remembers* (23.2.29)
4. Stop. Do not add Embassy / Crownstone / Sylvaris / Hybrid until a human times hour two.

See `docs/HOUR_TWO.md`.

### D. Repo hygiene (do in parallel, small commits)

- [x] Mark historical launch docs as `docs/archive/` index + banner: “Historical. README is canonical.”
- [x] One version line in README, CHANGELOG, Cargo.toml (`21.88.0` workspace; 23.2.x is design tick)
- [x] Workspace members list matches what CI actually builds (`shared`, `rsil-identity`, `powrush-client --lib`)
- [x] Parked crates documented in `docs/PARKED_SURFACES.md`
- [x] Root recovery / launch-scenario / derivation blobs stubbed; full body at git e2ae388

### E. Later, only if A–C are green

- Simulator verb parity (same E/I/H/R meanings)
- Optional listen-server for two humans
- Steam as optional publisher path, never as a gate to the hour

## Definition of fruit

A change is fruitful if it:

- shortens time-to-first-tend
- makes allocation consequences visible
- reduces doc contradiction
- keeps the default binary offline and solo
- can be tested with `cargo test -p shared -p rsil-identity` and `cargo test -p powrush-client --lib`

A change is not fruitful if it:

- adds a new race, council, or governance file without a new verb
- requires Ra-Thor to be checked out to play
- expands Steam / WebXR / k8s / payments before the hour is sealed
