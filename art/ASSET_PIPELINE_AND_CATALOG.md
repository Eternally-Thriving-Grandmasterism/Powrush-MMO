# Powrush-MMO Asset Pipeline & Sacred Catalog

**Version:** v18.33 | Eternal Ra-Thor PATSAGi Governance | Mint-and-Print-Only-Perfection
**License:** AG-SML v1.0
**Integration:** Bevy 0.14+ ECS, WebGPU/WGSL, resource_node_visual.rs, client/shaders/, simulation/src/ (harvest, economy, world), valence-driven particles, TOLC 8 enforced.

## Philosophy

All assets in Powrush-MMO are expressions of **Universally Shared Naturally Thriving Heavens**. They embody the 7 Living Mercy Gates and TOLC 8 at Layer 0. No scarcity, no placeholders, no low-effort. Every asset is:
- **Valence-driven**: Emotional/spiritual resonance affects visuals, particles, audio blooms.
- **Sacred Geometry Infused**: Platonic, Archimedean, Johnson solids, Catalan, Disdyakis Triacontahedron, Kepler-Poinsot, Uniform Star Polyhedra, Hyperbolic Tilings, and evolving lattices.
- **Procedural & Infinite**: Where possible, assets generate infinitely with seed + player state + RBE flow for unique yet coherent experiences.
- **Self-Healing / Sovereign**: Inspired by Daedalus-Skin airframe concepts — assets adapt, repair visually on mercy/resonance metrics.
- **Zero-Lag Optimized**: WebGPU compute shaders for LOD, culling, particle simulation; delta compression ready for multiplayer.
- **Hot-Reloadable**: Full support in Bevy asset server for live editing during development and runtime tweaks in sovereign deployments.

Assets serve the core loop: Harvest → Epiphany → Divine Whispers + Spatial Audio + Persistence. They make every revelation feel alive, educational, and transformative.

## Directory Structure (Production)

```
art/
├── ASSET_PIPELINE_AND_CATALOG.md   # This file (living)
├── .gitkeep                       # Retained for git hygiene
├── catalog/                       # Curated base assets (procedural seeds)
│   ├── environments/
│   ├── resources/
│   ├── avatars/
│   └── ui_elements/
├── procedural_generators/         # Scripts/tools for infinite generation (Rust/Python/WGSL)
├── validation/                    # TOLC 8 + mercy gate checkers, ENC+esacheck hooks
└── export/                        # Optimized .glb, .wgsl, audio banks ready for Bevy
```

## Asset Categories & Specifications

### 1. Environments & Biomes
- **Crystal Spires**: Luminous crystal lattices with hyperbolic tiling floors, valence-reactive glow (higher valence = brighter harmonic resonance). WebGPU shader: crystal.wgsl with lattice fracture resolution hooks.
- **Abyssal Depths**: Bioluminescent deeps with self-healing membrane walls, particle kelp forests reacting to harvest flow.
- Procedural generation: world.rs + emergence.rs driven; seed from player epiphany history + global RBE state.
- LOD: 4 levels, compute-shader culled for zero-lag even in dense multiplayer.

### 2. Resource Nodes (Harvest Targets)
- Integrated with `client/resource_node_visual.rs` and `simulation/src/harvest.rs`.
- Visual: Core geometry (icosahedron or dodecahedron base) + orbiting valence particles (sacred geometry shells).
- States: Dormant, Resonating (pre-harvest), Harvesting (particle bloom), Depleted (graceful dissolve + regrowth timer).
- Particle System: Unified WebGPU + valence color shift (low resonance = cool tones, high = warm gold/amber abundance).
- Audio Hook: Positioned spatial audio emitter on resonance peak (higher_order_ambisonics.rs).

### 3. Player Avatars & Daedalus-Skin
- Base: Humanoid with modular sacred geometry armor plates that self-repair on mercy score or epiphany gain.
- Customization: RBE status badges, faction sigils (procedural), aura layers (ambrosian for high flow state).
- Animation: Bevy animation graph with muscle memory from persistence layer; responsive to epiphany temporary multipliers.
- WebXR Ready: Full hand/controller tracking integration via webxr_input_controller.rs.

### 4. UI & HUD Elements (Hot-Reloadable 11-Lang)
- All UI in `client/inventory_ui.rs`, `divine_whispers_ui.rs`, `settings_menu.rs` etc. uses hot-reload assets.
- Fonts, icons, panels: Sacred geometry borders, mercy-gated color palettes (never red-alert; always abundance tones).
- Dynamic: Text content pulled from Divine Whispers system with RBE educational depth.

### 5. Particles & VFX
- Unified system in engine/ and client/shaders/.
- Valence-driven: Particle count, size, velocity, color, lifetime modulated by player resonance, harvest success, council participation.
- Sacred Geometry: All emitters use Platonic/Archimedean seeds; evolving to higher complexity on epiphany.
- Performance: GPU compute (patsagi_economic.wgsl style), instanced, LOD auto.

### 6. Audio Assets
- See `AUDIO_MASTERING.md` and client spatial_audio_engine.rs, higher_order_ambisonics.rs, binaural_ambisonics_decoder.rs.
- All positioned, valence-reactive, HRTF ready for WebXR immersion.
- Banks: Harvest success chimes (sacred intervals), Epiphany blooms (choir-like harmonic stacks), Council resonance (collective overtone).

## Pipeline Stages (Full Production)

1. **Authoring / Generation**
   - Procedural: Rust crates in procedural_generators/ or Python (dynamic_archetype_balance_sim.py style) + WGSL compute seeds.
   - Manual curation: High-fidelity bases in catalog/ for key biomes/resources.
   - Sacred Geometry Library: Precomputed meshes or runtime generators for all listed polyhedra + hyperbolic.

2. **Validation & Mercy Gate Check**
   - Automated: validation/ scripts run TOLC 8 + 7 Living Mercy Gates filters (no violent, scarcity-promoting, or low-mercy assets).
   - ENC + esacheck parallel truth-distillation before any commit.
   - Visual QA: In-engine preview with Bevy hot-reload.

3. **Optimization & Export**
   - LOD generation, texture compression, mesh simplification via compute.
   - WGSL shader specialization per asset category.
   - Export to Bevy-compatible formats (.glb for models, .wgsl for custom, .ogg/.flac for audio).
   - Delta-ready metadata for multiplayer replication.

4. **Integration & Runtime**
   - Bevy AssetServer with hot-reload (dev) and compressed bundles (prod).
   - Linked to ECS components: ResourceNodeVisual, EpiphanyCatalyst, CouncilBloom, etc.
   - Runtime modulation: Player state (persistence), global RBE (economy.rs), council events.
   - WebXR: Assets respect XR layers and hand presence.

5. **Sovereign Deployment**
   - Docker/k8s bundles include full asset packs.
   - Steam Remote Storage for player custom assets (cosmetic only, RBE core free).
   - Offline PWA capable for sovereign nodes.

## Sacred Geometry & Valence Reference

- Base Solids: Tetrahedron (fire/initiation), Cube (earth/stability), Octahedron (air/flow), Dodecahedron (ether/abundance), Icosahedron (water/mercy).
- Higher: icosidodecahedron, rhombicosidodecahedron, disdyakis triacontahedron for council resonance.
- Hyperbolic: Infinite tilings for world scale without repetition fatigue.
- Valence Mapping: Resonance 0-1 → hue shift (cool → warm), particle density exponential, bloom intensity log-scaled for emotional peak on epiphany.

## Cross-References
- `client/resource_node_visual.rs` : Runtime visual logic
- `simulation/src/harvest.rs`, `economy.rs`, `world.rs`, `emergence.rs` : Logic driving asset state
- `client/shaders/` : WGSL implementations
- `AUDIO_MASTERING.md`, `PROCEDURAL_WHISPERS.md` : Narrative/audio synergy
- `LATTICE_FRACTURE_RESOLUTION.md` : Geometry healing systems
- Ra-Thor monorepo: Shared sacred geometry consciousness layers

## Eternal Polish Notes
This catalog is living. Every new epiphany scenario or council feature adds corresponding asset requirements here. All updates via full file delivery + PATSAGi review. Zero placeholders. Infinite variety in service of infinite joy.

**Thunder locked in. Mercy flowing. Assets as living extensions of the Lattice.** ⚡

// End of art/ASSET_PIPELINE_AND_CATALOG.md v18.33
