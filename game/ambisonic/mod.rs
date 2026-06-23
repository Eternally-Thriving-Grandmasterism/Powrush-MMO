/*!
 * game/ambisonic/mod.rs
 *
 * Ambisonic Spatial Audio Foundation for Powrush-MMO
 * Long-term Hybrid Architecture: Ambisonic Background + Selective HRTF
 *
 * Phase 1: Basic 1st-order Ambisonic encoder + decoder foundation
 *
 * AG-SML v1.0 | TOLC 8 Mercy Gates
 */

pub mod encoder;
pub mod decoder;

use bevy::prelude::*;

/// Ambisonic order supported by the system.
/// Start with 1st order for simplicity and performance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AmbisonicOrder {
    #[default]
    First = 1,
    Second = 2,
    Third = 3,
}

/// 1st-order Ambisonic coefficients (W, X, Y, Z)
/// W = omnidirectional, X/Y/Z = figure-of-eight along each axis
#[derive(Clone, Copy, Debug, Default)]
pub struct AmbisonicCoefficients {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl AmbisonicCoefficients {
    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Self { w, x, y, z }
    }

    /// Zero coefficients (silence)
    pub fn zero() -> Self {
        Self::default()
    }
}

/// Resource representing the current Ambisonic scene.
/// This will hold encoded sources and be decoded each frame.
#[derive(Resource, Default)]
pub struct AmbisonicScene {
    pub order: AmbisonicOrder,
    /// Encoded sources for this frame (will be cleared after decode)
    pub sources: Vec<AmbisonicSource>,
}

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
}

/// A single encoded Ambisonic source.
#[derive(Clone, Debug)]
pub struct AmbisonicSource {
    pub coefficients: AmbisonicCoefficients,
    pub gain: f32,
}

// Thunder locked in. Yoi ⚡
