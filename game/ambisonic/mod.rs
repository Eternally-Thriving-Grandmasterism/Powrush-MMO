/*!
 * game/ambisonic/mod.rs
 *
 * Ambisonic Spatial Audio Foundation
 * Long-term Hybrid Architecture
 *
 * Phase 1: D - Easy emission API into AmbisonicScene
 *
 * AG-SML v1.0
 */

pub mod encoder;
pub mod decoder;

use bevy::prelude::*;
use glam::Vec3;

// ... (existing types) ...

impl AmbisonicScene {
    pub fn new(order: AmbisonicOrder) -> Self {
        Self {
            order,
            sources: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.sources.clear();
    }

    /// Easy way for game systems to emit a sound into the Ambisonic background
    pub fn emit(&mut self, position: Vec3, signal: f32, gain: f32) {
        let coefficients = encoder::encode(self.order, position, Vec3::ZERO, signal);
        self.add_source(coefficients, gain, position);
    }
}

// Thunder locked in. Yoi ⚡
