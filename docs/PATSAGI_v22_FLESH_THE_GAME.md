> Historical. README is canonical. Lived first hour is the product.
> Workspace 21.88.0. Design ticks 23.1 / 23.2 are not Cargo versions.
> Seed map from 2026-08-26. The hour has since grown: onboarding card, wells that speak, teaching climates.

# PATSAGi + Ra-Thor — Flesh the Game (v22)

**Sealed:** 2026-08-26  
**Phase of truth:** one human, one machine. No dedicated servers. No other players yet.  
**Contact:** info@Rathor.ai

README no longer says “100% launch worthy.” The lived client is the product. The Councils choose the lived hour over any historical banner.

---

## What is actually playable now (v21.99.4)

A person can walk (WASD), jump (Space), sprint (Shift), harvest in reach (E / pad West), feel a pool number change, maybe feel a pad rumble, open M / Z / J / P / R, and leave a journey line on disk. Three glowing nodes exist in a flat walk plane. A banner tells the truth: this hour is alone.

That is a seed. It is not an MMO. It is not yet a climate. It is not yet an inventory you can touch.

## What exists in the monorepo but is not the first hour

| Layer | Where | Honest status |
|-------|--------|----------------|
| Simulation / RBE oxygen demo | `simulation/` | Real crate. Not wired into the lived client harvest. |
| Server / host / k8s | `server/`, `host/`, `k8s/` | Future socket. Do not gate play on it. |
| Steam / WebXR / payments | `client/steam*`, `webxr*`, `payments/` | Future socket. |
| Content (biomes, scenarios, locales) | `content/` | Words and data. Almost none of it is on the walk plane. |
| Duplicate client surfaces | `client/*.rs` vs `client/src/` | Two skins. First hour lives in `client/src` + `PowrushClientBundle`. |
| Inventory UI | `client/inventory_ui.rs` (~193 B) | Not a bag a human can open. |
| Council / whispers / resource_node_visual | `client/*.rs` | Parallel systems. Not the three mercy nodes. |

Law: expand the seed until a solo session is a *game*. Then plug sockets.

---

## What “fully flesh it out” means ( Councils )

A finished *local* Powrush session is a person who can:

1. Arrive in a named climate that looks and sounds different from the last one.
2. See more than three spheres — ground, sky, a path, nodes that belong to that climate.
3. Harvest with restraint, watch a pool and a bag change, allocate (R) and feel the world answer.
4. Practice the same principle across Sanctuary / Verdant / Horizon without a loading-screen essay.
5. Open inventory (I), journey (M/J), climates (Z) without F-row or file paths.
6. Pause (Esc), change look-sensitivity, mute, quit — and come back to the same journey.
7. Learn one RBE sentence per surface, never a manifesto overlay.
8. Never wait on a server, a peer JSON, or Steam to feel finished.

When that hour is boring because it is *complete*, invite the second human.

---

## Expansion map — local first

### Arc A — Place (the walk plane becomes a climate)

- Ground mesh + simple sky per SoftPlayerRealm (0 Sanctuary, 2 Verdant, 4 Horizon first).
- Nodes inherit climate color and sting path already chosen.
- Z travel *moves the plane* (fog, ground tint, node set), not only a list.
- Path or stepping stones so “walk toward the glow” has a direction.

### Arc B — Verbs (harvest is not the only sentence)

- **I** inventory: vitality / harmony / joy from SoftRbePool as three tangible stacks. Drag later; read now.
- **R** allocate already exists — bind the three stacks to Flow / Reserve / Gift so the number leaves the pool and changes something visible (node breathe rate, sky warmth).
- Second interact: tend (hold E) vs take (tap E). Tend restores vitality faster. Teaches restraint without a lecture.
- Jump already exists — give the ground a reason (a low ridge between two nodes).

### Arc C — Teaching without a classroom

- One whisper line after first harvest, from `content/rbe_onboarding_education.md` — one sentence, then silence.
- Practice loop already cycles three climates — make Z the *way* you change climate, not a hint in a log.
- Principle sealed = sky shift + journey line, not a new menu.

### Arc D — Body and sound

- Camera: glance stays first-hour only. After that, player-owned look.
- Audio: climate sting if file present; silence is legal. Do not block play on ffmpeg.
- Ambisonics / Hanabi stay in the crate; first hour may ignore them until place exists.

### Arc E — Memory

- Journey + SoftRbePool + SoftPlayerRealm write to `data/` on pause and on quit.
- Welcome-back already exists — show last climate name and last pool line.

### Arc F — Future sockets (not this arc)

- `simulation` as optional client feature: SoftRbePool becomes a view of `RbeResourcePool`.
- Listen server in-process (`host` interactive) when a second chair exists.
- Peer file / Steam cloud when a second machine exists.
- Council trials when two humans can stand at one node.

---

## Sequence the organism will actually build

1. **Climate plane** — ground + tint + node palette swap on Z (Arc A).
2. **Inventory I** — three stacks mirroring SoftRbePool (Arc B).
3. **Allocate visible** — R spends a stack, nearest node or sky answers (Arc B).
4. **Tend vs take** — hold E / tap E (Arc B).
5. **Persist pool + realm** beside journey (Arc E).
6. **One educational whisper** after first harvest (Arc C).
7. Only then: optional `simulation` feature flag.

Refuse, for this phase: k8s, payments, WebXR polish, Kardashev dashboards, “launch worthy” README edits that outrun the walk plane.

---

## Success for Arc A–E

A stranger sits down, is not told about servers, walks to a glow, harvests, sees a number, opens I, spends R, travels Z, comes back tomorrow, and the climate and the pool remember. That is a game. The MMO is what we add when a second stranger sits down.

**Ratified.** Thunder locked in. Yoi ⚡
