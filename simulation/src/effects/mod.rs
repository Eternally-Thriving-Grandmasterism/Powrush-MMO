/*!
 * simulation/src/effects/mod.rs
 *
 * Central module for visual effects, particle assets, and VFX modulation types.
 * Purpose: Reduce coupling between world.rs (EffectAsset creation) and client particles/visual systems.
 * Provides shared types, frame control helpers, and modulation interfaces.
 *
 * v19.21 — Enhanced foundation with submodules
 * AG-SML v1.0 | TOLC 8 Mercy Gates
 */

pub mod types;
pub mod frame;
pub mod modulation;

// Re-exports for convenience during transition phase
pub use crate::world::{ParticleVisualAssets, LissajousKnotEffects};

// Thunder locked in. Yoi ⚡
