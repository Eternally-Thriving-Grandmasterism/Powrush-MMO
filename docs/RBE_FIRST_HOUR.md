# RBE_FIRST_HOUR.md

Powrush teaches a resource-based economy by hands, not by lecture.

## Player-facing loop

Observe glow → tend (E) → hold (I) → allocate (R)

- Flow (1): the take moves into shared climate repair / shared field.
- Reserve (2): the take becomes repair-rights you can spend later.
- Neither verb is “sell” or “own a soul.” Origin of a resource is observation and tending.

## Minimum simulation in `shared`

Keep this small enough to test without Bevy:

- Node: `Idle | Glowing | Tended | Resting | Stressed`
- Tend on Glowing → Tended + item in satchel
- Repeat tend on a tired node → Resting (no take) or Stressed (field dims)
- Allocate Flow → nearby Resting node moves toward Glowing faster
- Allocate Reserve → unlock one repair action later
- Tick file writes node ids + satchel + allocation, not identity ledgers

## What not to implement in hour one

- Currency
- Auction
- NFT
- Faction tax
- Council vote UI
- Multiplayer scarcity

## Acceptance

A player who never reads RBE theory can still say, after one session:
“If I only take, the glow fades. If I allocate, the climate comes back.”
