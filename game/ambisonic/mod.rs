/*!
 * game/ambisonic/mod.rs
 *
 * Ambisonic Spatial Audio Foundation for Powrush-MMO
 * Long-term Hybrid Architecture: Ambisonic Background + Selective HRTF
 *
 * Phase 1: Encoding sources into AmbisonicScene (A)
 *
 * AG-SML v1.0 | TOLC 8 Mercy Gates
 */

pub mod encoder;
pub mod decoder;

use bevy::prelude::*;
use glam::Vec3;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AmbisonicOrder {
    #[default]
    First = 1,
    Second = 2,
    Third = 3,
}

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

    pub fn zero() -> Self {
        Self::default()
    }
}

/// A single encoded source ready for the Ambisonic scene
#[derive(Clone, Debug)]
pub struct AmbisonicSource {
    pub coefficients: AmbisonicCoefficients,
    pub gain: f32,
    pub position: Vec3,
}

/// The Ambisonic scene resource.
#[derive(Resource, Default)]
pub struct AmbisonicScene {
    pub order: AmbisonicOrder,
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

    /// Add an encoded source to the scene this frame
    pub fn add_source(&mut self, coefficients: AmbisonicCoefficients, gain: f32, position: Vec3) {
        self.sources.push(AmbisonicSource {
            coefficients,
            gain,
            position,
        });
    }
}

// Thunder locked in. Yoi ⚡
