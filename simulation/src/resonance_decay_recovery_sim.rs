//! simulation/src/resonance_decay_recovery_sim.rs
//! Powrush-MMO — Resonance Decay & Recovery Simulation (Advanced)
//! Comprehensive simulation of Ambrosian selfish penalty vs recovery mechanics
//!
//! Run with: cargo test --package simulation resonance_decay_recovery_sim -- --nocapture

use std::fmt;

#[derive(Clone, Debug)]
pub struct SimulatedPlayer {
    pub resonance_attunement: f32,
    pub total_epiphanies: u32,
    pub successful_blooms: u32,
    pub selfish_actions: u32,
}

impl SimulatedPlayer {
    pub fn new(resonance: f32) -> Self {
        Self {
            resonance_attunement: resonance.clamp(0.0, 5.0),
            total_epiphanies: 0,
            successful_blooms: 0,
            selfish_actions: 0,
        }
    }

    pub fn apply_selfish_penalty(&mut self, severity: f32) {
        let reduction = severity * 0.3;
        self.resonance_attunement = (self.resonance_attunement - reduction).max(0.0);
        self.selfish_actions += 1;
    }

    pub fn apply_epiphany(&mut self, boost: f32, intensity: f32) {
        let gain = boost * intensity * 0.1;
        self.resonance_attunement = (self.resonance_attunement + gain).min(5.0);
        self.total_epiphanies += 1;
    }

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
            "Resonance: {:.2} | Epiphanies: {} | Blooms: {} | Selfish: {}",
            self.resonance_attunement,
            self.total_epiphanies,
            self.successful_blooms,
            self.selfish_actions
        )
    }
}

// ============================================================
// TABLE OUTPUT HELPERS
// ============================================================

fn print_header(title: &str) {
    println!("\n{}", "=".repeat(70));
    println!("{:^70}", title);
    println!("{}", "=".repeat(70));
}

fn print_table_row(label: &str, before: f32, after: f32, note: &str) {
    println!(
        "| {:<28} | {:>6.2} → {:>6.2} | {}",
        label, before, after, note
    );
}

// ============================================================
// PUBLIC RUNNER (for orchestrator / external calls)
// ============================================================

pub fn run_resonance_decay_recovery_simulation() {
    println!("\n\n=== RESONANCE DECAY & RECOVERY SIMULATION (Advanced) ===");
    run_all_scenarios();
}

fn run_all_scenarios() {
    scenario_low_starting_resonance();
    scenario_mixed_playstyle();
    scenario_long_term_trend();
    scenario_heavy_decay_recovery();
}

// ============================================================
// ADVANCED SCENARIOS
// ============================================================

fn scenario_low_starting_resonance() {
    print_header("SCENARIO 1: Very Low Starting Resonance (1.2)");
    let mut player = SimulatedPlayer::new(1.2);
    println!("Start: {}", player);

    // Recovery with high-quality Epiphanies + Council
    player.apply_epiphany(2.4, 0.92);
    print_table_row("Strong Epiphany", 1.20, player.current_resonance(), "High intensity");
    player.apply_council_bloom(0.85);
    print_table_row("Council Bloom", 1.20, player.current_resonance(), "Group harmony");
    player.apply_epiphany(2.5, 0.88);
    player.apply_epiphany(2.3, 0.90);

    println!("Final: {}", player);
    println!("Insight: Even heavily penalized Ambrosians can recover with focused, high-quality play.\n");
}

fn scenario_mixed_playstyle() {
    print_header("SCENARIO 2: Mixed Playstyle (Selfish + Cooperative)");
    let mut player = SimulatedPlayer::new(3.8);
    println!("Start: {}", player);

    // Some selfish actions
    player.apply_selfish_penalty(1.0);
    player.apply_selfish_penalty(0.8);
    print_table_row("2 selfish actions", 3.80, player.current_resonance(), "Mild penalty");

    // Then switches to good play
    player.apply_council_bloom(0.78);
    player.apply_epiphany(2.1, 0.85);
    player.apply_epiphany(2.0, 0.82);
    player.apply_council_bloom(0.81);

    println!("Final: {}", player);
    println!("Insight: Switching back to cooperative play allows steady recovery even after some selfishness.\n");
}

fn scenario_long_term_trend() {
    print_header("SCENARIO 3: Long-Term Trend (Multiple Sessions)");
    let mut player = SimulatedPlayer::new(4.0);
    println!("Starting resonance: {:.2}", player.current_resonance());

    // Session 1: Mostly good
    for _ in 0..4 { player.apply_epiphany(2.0, 0.80); }
    player.apply_council_bloom(0.75);
    println!("After Session 1 (mostly good): {:.2}", player.current_resonance());

    // Session 2: Some selfishness
    player.apply_selfish_penalty(1.2);
    player.apply_selfish_penalty(0.9);
    for _ in 0..3 { player.apply_epiphany(1.8, 0.75); }
    println!("After Session 2 (mixed): {:.2}", player.current_resonance());

    // Session 3: Strong recovery focus
    for _ in 0..5 { player.apply_epiphany(2.2, 0.88); }
    player.apply_council_bloom(0.89);
    println!("After Session 3 (strong recovery): {:.2}", player.current_resonance());

    println!("Long-term trend shows resilience when player returns to harmony-focused play.\n");
}

fn scenario_heavy_decay_recovery() {
    print_header("SCENARIO 4: Heavy Decay + Aggressive Recovery");
    let mut player = SimulatedPlayer::new(4.5);

    // Heavy selfishness
    for _ in 0..7 {
        player.apply_selfish_penalty(1.0);
    }
    println!("After heavy selfishness (7 actions): {:.2}", player.current_resonance());

    // Aggressive high-quality recovery
    for i in 0..6 {
        let intensity = 0.85 + (i as f32 * 0.02);
        player.apply_epiphany(2.4, intensity);
    }
    player.apply_council_bloom(0.92);
    player.apply_council_bloom(0.88);

    println!("After aggressive recovery: {}", player);
    println!("Insight: Even severe decay is recoverable with dedicated high-quality sessions.\n");
}

// ============================================================
// TESTS (with nice output)
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_advanced_scenarios() {
        run_resonance_decay_recovery_simulation();
        // Basic sanity assertions
        let mut p = SimulatedPlayer::new(2.0);
        p.apply_selfish_penalty(1.0);
        assert!(p.current_resonance() < 2.0);
        p.apply_epiphany(2.5, 0.9);
        assert!(p.current_resonance() > 2.1);
    }
}
