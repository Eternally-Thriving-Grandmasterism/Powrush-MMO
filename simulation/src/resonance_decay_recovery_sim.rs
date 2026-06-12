//! simulation/src/resonance_decay_recovery_sim.rs
//! Powrush-MMO — Resonance Decay & Recovery Simulation
//! Tests and demonstrates selfish penalty vs recovery mechanics for Ambrosian Ascension
//!
//! Mirrors production logic from:
//! - server/src/ascension_abilities.rs (selfish penalty)
//! - server/src/persistence_polish.rs (epiphany & council bloom recovery)
//!
//! Run with: cargo test --package simulation resonance_decay_recovery_sim

use std::fmt;

/// Simplified player state for simulation (mirrors real AscensionProgress + MuscleMemory)
#[derive(Clone, Debug)]
pub struct SimulatedPlayer {
    pub resonance_attunement: f32,
    pub total_epiphanies: u32,
    pub successful_blooms: u32,
}

impl SimulatedPlayer {
    pub fn new(resonance: f32) -> Self {
        Self {
            resonance_attunement: resonance.clamp(0.0, 5.0),
            total_epiphanies: 0,
            successful_blooms: 0,
        }
    }

    /// Applies selfish penalty (from ascension_abilities.rs)
    pub fn apply_selfish_penalty(&mut self, severity: f32) {
        let reduction = severity * 0.3;
        self.resonance_attunement = (self.resonance_attunement - reduction).max(0.0);
    }

    /// Applies epiphany recovery (from persistence_polish.rs MuscleMemory::apply_epiphany_boost)
    pub fn apply_epiphany(&mut self, boost: f32, intensity: f32) {
        let gain = boost * intensity * 0.1;
        self.resonance_attunement = (self.resonance_attunement + gain).min(5.0);
        self.total_epiphanies += 1;
    }

    /// Applies council bloom recovery (from persistence_polish.rs)
    pub fn apply_council_bloom(&mut self, collective_attunement: f32) {
        let gain = collective_attunement * 0.05;
        self.resonance_attunement = (self.resonance_attunement + gain).min(5.0);
        self.successful_blooms += 1;
    }

    pub fn current_resonance(&self) -> f32 {
        self.resonance_attunement
    }
}

impl fmt::Display for SimulatedPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Resonance: {:.2} | Epiphanies: {} | Blooms: {}",
            self.resonance_attunement,
            self.total_epiphanies,
            self.successful_blooms
        )
    }
}

// ============================================================
// SIMULATION TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selfish_decay_and_basic_recovery() {
        let mut player = SimulatedPlayer::new(4.0);
        println!("\n=== Scenario: Selfish Decay + Basic Recovery ===");
        println!("Start: {}", player);

        // Apply selfish penalties
        player.apply_selfish_penalty(1.0);
        player.apply_selfish_penalty(1.0);
        println!("After 2 selfish actions: {}", player);

        // Recover with epiphanies
        player.apply_epiphany(2.0, 0.85);
        player.apply_epiphany(1.8, 0.75);
        player.apply_epiphany(2.2, 0.9);
        println!("After 3 epiphanies: {}", player);

        assert!(player.resonance_attunement > 3.0, "Should recover above 3.0");
        println!("Test passed: Recovery successful.\n");
    }

    #[test]
    fn test_council_bloom_accelerates_recovery() {
        let mut player = SimulatedPlayer::new(2.5);
        println!("\n=== Scenario: Council Bloom Accelerated Recovery ===");
        println!("Start (penalized): {}", player);

        player.apply_council_bloom(0.82);
        player.apply_epiphany(2.1, 0.88);
        player.apply_council_bloom(0.79);
        player.apply_epiphany(1.9, 0.81);

        println!("After 2 blooms + 2 epiphanies: {}", player);

        assert!(player.resonance_attunement >= 3.2);
        println!("Test passed: Council participation speeds recovery.\n");
    }

    #[test]
    fn test_heavy_decay_vs_strong_recovery() {
        let mut player = SimulatedPlayer::new(4.2);
        println!("\n=== Scenario: Heavy Selfish Decay vs Strong Recovery ===");

        // Heavy selfishness
        for _ in 0..6 {
            player.apply_selfish_penalty(1.0);
        }
        println!("After heavy selfishness: {}", player);

        // Strong recovery session
        for i in 0..5 {
            let intensity = 0.82 + (i as f32 * 0.03);
            player.apply_epiphany(2.3, intensity);
        }
        player.apply_council_bloom(0.91);

        println!("After strong recovery session: {}", player);

        assert!(player.resonance_attunement > 3.8, "Should mostly recover");
        println!("Test passed: Even heavy decay is recoverable with focused play.\n");
    }

    #[test]
    fn test_resonance_cap_at_5() {
        let mut player = SimulatedPlayer::new(4.8);
        println!("\n=== Scenario: Resonance Cap at 5.0 ===");

        for _ in 0..10 {
            player.apply_epiphany(2.5, 0.95);
        }

        println!("After many strong epiphanies: {}", player);
        assert!((player.resonance_attunement - 5.0).abs() < 0.01);
        println!("Test passed: Cap at 5.0 enforced.\n");
    }
}
