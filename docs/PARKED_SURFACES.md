# PARKED_SURFACES.md

Parked means: keep on disk, do not make the default lived hour depend on it.

| Path | Why it exists | When to unpark |
| --- | --- | --- |
| `server/` | Authoritative multiplayer / AOI experiments | After first hour save/load + two-player need |
| `simulation/` | RBE demos (`rbe_oxygen_demo`) | After first-hour client shows the same teaching claim |
| `host/` | Hosting / wrapper experiments | After a real second machine needs hosting |
| `game/` | Older game-layer slice | Only to mine code into `client` / `shared` |
| `powrush-divine-module/` | Council / whisper experiments | After first hour has one quiet world sentence, not a second HUD |
| `payments/` | Commercial rails | Never in first hour |
| `k8s/` | Cluster deploy | After there is a service worth deploying |
| `deployment/steam`, `publishing/steam` | Store packaging | After playtest hour is sealed |
| `web-portal/`, `website/` | Public site | Copy verbs from README; do not invent a second game |

Rule: if a parked crate is imported by `powrush-client` default features, that is a defect.
