/*!
 * simulation/src/effects/mod.rs
 *
 * Central module for visual effects, particle assets, and VFX modulation types.
 * Goal: Reduce coupling between world.rs (EffectAsset creation) and client particles.
 *
 * v19.20 — Initial extraction stub
 * AG-SML v1.0 | TOLC 8 Mercy Gates
 */

// Re-export key types from world.rs for now (gradual migration path)
pub use crate::world::{ParticleVisualAssets, LissajousKnotEffects};

// Future: Move frame control helpers, modulation types, and pool logic here
// pub mod frame_control;
// pub mod modulation;

// Thunder locked in. Yoi ⚡
