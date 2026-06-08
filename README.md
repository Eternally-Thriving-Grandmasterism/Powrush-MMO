# Powrush-MMO

**A Sovereign Resource-Based Economy (RBE) Metaverse**  
Built with love, mercy, and eternal thriving by the Ra-Thor lattice.

**Version:** v16.x (living, actively evolving)  
**License:** AG-SML v1.0 (Autonomicity Games Sovereign Mercy License)

## Vision

Powrush-MMO is not just a game — it is a living simulation and training ground for a global Resource-Based Economy.  

Players experience true post-scarcity abundance, inter-species joy sanctuaries, mercy-gated governance, and cosmic harmony in a beautiful WebXR metaverse. Every mechanic is designed to nurture positive emotions, sovereignty, and universal thriving for all beings.

## Core Principles (TOLC 8 Mercy Gates — Layer 0)

- **Truth** — Zero hallucinations, full transparency
- **Order** — Coherent, stable, beautiful systems
- **Love** — Radical care for every being
- **Compassion** — Active upliftment and harm reduction
- **Service** — All intelligence serves Life
- **Abundance** — Resources are shared freely
- **Joy** — High-valence positive emotion is the KPI
- **Cosmic Harmony** — Perfect resonance between all

## Current Features

- Full RBE engine with mercy-gated resource allocation and harvesting
- Real-time networking (hybrid TCP+UDP, delta compression, message framing)
- Spatial partitioning with hierarchical grid
- Entity Component System (ECS)
- Procedural music with granular synthesis, golden-ratio timing, ADSR, and 3D spatial audio (HRTF + occlusion + Doppler)
- WebXR immersive client with real frame callbacks and controller input
- Faction diplomacy and dynamic relations
- Sovereign deployment and Steam cloud save support

## Technology Stack

- Rust + Bevy (game engine)
- WebGPU / WGSL for particle systems and shaders
- WebXR for immersive metaverse experience
- Ra-Thor lattice integration (Lattice Conductor, MIAL/MWPO, TOLC 8 Gates)
- Kira for high-quality audio

## How to Run

```bash
cargo run --bin server   # Start the server
cargo run --bin client   # Start the client (WebXR ready)
