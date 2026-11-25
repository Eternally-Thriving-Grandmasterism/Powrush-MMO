# client/src/_parked

Card PARK-DEAD-1. These Rust files are parked: no `mod` line in the client crate reaches them, so
nothing in this folder is compiled, linted or tested.

- Rule: a file at `client/src/X` that no `mod` tree reaches moves here as `client/src/_parked/X`,
  with the same relative path and no content edit.
- Whole folders (`debug/`, `gpu/`, `gpu_simulation/`, `ships/`, `visual/`, `world_simulation/`)
  moved intact, so their own `mod` lines still name their siblings.
- To un-park a file, move it back to its old path and add the `mod` line that compiles it.
- The guard test `park_dead_1_every_unparked_src_rs_is_in_a_mod_tree` in
  `client/src/entity_policy.rs` fails when a `.rs` file outside this folder is not in a `mod` tree.
- Nothing here is deleted. Deleting a parked file is Sherif's call.
