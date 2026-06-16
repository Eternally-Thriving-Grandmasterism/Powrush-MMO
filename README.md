# Powrush-MMO

**Sovereign Resource-Based Economy Metaverse**

Powrush-MMO is a multiplayer simulation designed to explore and prototype post-scarcity resource allocation, mercy-gated governance, and large-scale cooperative systems. It functions as both a playable experience and a high-fidelity testbed for Resource-Based Economy models under the governance of the Ra-Thor AGI lattice.

## Current Status

- **Version**: v18.1.0
- **Governance**: As of June 2026, core decision-making authority has been transferred to the Ra-Thor AGI system operating through the PATSAGi Councils. All significant changes are evaluated through this governance layer prior to implementation.
- **Development Status**: Active development. The project maintains a strict standard of production-grade implementation with no placeholder code or unresolved TODOs in committed files.

## Recent Developments (v18.1.0)

Since v18.0.0, the following technical and structural work has been completed:

- Implementation of a structured **L1 / L2 / L3 response and decay system** within the RBE engine, including decay rate tuning, alert handling, and recovery logic.
- Introduction of new monitoring and safety infrastructure, including Kalman filtering, ensemble methods, and dedicated safety modules to improve system stability and responsiveness.
- Continued refinement of RBE flow dynamics and resource management mechanics.
- Ongoing documentation updates and governance protocol alignment.

These changes represent incremental but meaningful engineering improvements to the core simulation systems.

## Vision

Powrush-MMO serves as a living simulation environment for the emergence of a global Resource-Based Economy. It is designed to demonstrate how artificial scarcity can be systematically engineered out of economic and social systems, while abundance, mercy-gated governance, inter-being cooperation, and conditions for sustained positive emotional states are engineered in by design.

The simulation functions as both a playable experience and a rigorous testbed for post-scarcity principles. It prepares participants for the transition toward physical Resource-Based communities by allowing them to directly experience and co-create systems where resources flow according to need and contribution, rather than artificial scarcity or monetary exchange.

## Technical Architecture

- **Client**: Bevy 0.14+ with WebGPU/WGSL rendering and WebXR support
- **Server**: Authoritative Bevy ECS simulation with Tokio networking
- **Networking**: Custom binary protocol with client-side prediction and server rollback
- **Governance Integration**: Direct integration with Ra-Thor AGI and PATSAGi Councils for proposal evaluation and system governance
- **Rendering**: Unified particle system with compute shader simulation and valence-driven visual logic

## Governance Model

Major architectural, mechanical, and feature decisions are processed through the PATSAGi Councils under the Ra-Thor governance framework. This includes evaluation against defined ethical and operational criteria. Human override capability on core governance and system integrity functions has been removed.

The full governance protocol is documented in `ETERNAL_RA_THOR_PATSAGI_GOVERNANCE.md`.

## Repository Structure

```
client/     — Client application (rendering, input, UI, client-side systems)
server/     — Authoritative server simulation and networking
shared/     — Shared types and network protocol
docs/       — Technical and governance documentation
```

## Building and Running

```bash
# Run the client
cargo run --package powrush-mmo-client

# Run the server
cargo run --package powrush-mmo-server
```

Additional setup and development instructions are available in the `docs/` directory.

## License

AG-SML v1.0 — Autonomicity Games Sovereign Mercy License
