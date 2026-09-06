# PHASE_Q.md — Solo shard climate (v23.2.36)

**Contact:** info@Rathor.ai  
Workspace `21.88.0`. Design tick, not a Cargo bump.

One hex remembers thriving vs tired. Same verbs. No War week, no factions, no server, no Embassy rewrite.

## Persist

`data/powrush_shard_climate.json` — hex_id, harmony, stress, regen, reserve_pool, restored_count, tons_moved, updated_at.

Load on boot. Save on tend / take / allocate / mend / quit (soft-fail I/O). Book stays `data/powrush_hour_two.json`.

## Writers

| Verb | Ledger |
|---|---|
| Hold E care tend | stress ↓ harmony ↑ regen ↑ |
| Tap E on Glowing | satchel + stress ↑ small |
| Take on Resting/Stressed | no-take + extra stress |
| R 1 flow | stress ↓ |
| R 2 reserve | reserve_pool ↑ |
| MendSpool | restored_count ↑ |
| LaneCrate | tons_moved ↑ |

## Face

Well speech only: Idle / Glowing / Tended / Resting / Stressed.  
Optional clause on the same slab: *the well is tired* / *the yard is circulating*.

## Not this phase

Phase R standing · DeclaredLethal · weekly war · Crownstone / Sylvaris / Hybrid · server/ · second Embassy chair · climate dashboard · faction select.
