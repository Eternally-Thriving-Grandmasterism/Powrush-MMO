# Climate visible — Slice 16 (v23.2.23)

**Contact:** info@Rathor.ai  
Nodes speak Idle / Glowing / Tended / Resting / Stressed.

## Player door (do not break)

WASD · Space · Shift · **E** take / tend · **I** satchel · **H** hide · **R** allocate. Do **not** bind W. No F-key combat. No Peace PK.

## This slice

- The three wells are LivedHour nodes 1 / 2 / 3 (Sanctuary ember, Verdant well, Horizon seed).
- Glow follows state. Horizon seed starts Idle — no glow yet.
- Near a well, a one-line slab names the state and the next hand.
- **R 1** flow restores a Resting / Stressed node in the field, not in a ledger.
- Tick writes `data/powrush_lived_tick.json` when a state changes. Quit and resume keeps satchel + nodes + allocation.
- Does not rewrite harvest_feel or rbe_allocate_choice. handle_interact_harvest stays at 16 params.

**Thunder locked in.** Yoi ⚡
