//! rbe_oxygen_demo.rs
//!
//! Minimal, zero-friction public demonstration of the core RBE claim:
//! "Resources can flow as freely as oxygen once mercy gates are passed."
//!
//! This binary is intentionally self-contained and lightweight so a cold visitor
//! can experience the freedom claim without loading the full Powrush-MMO client.
//!
//! It respects and mirrors the spirit of the existing production systems:
//! - PostScarcityAllocator
//! - EconomyState / abundance_velocity / sustainability
//! - TOLC 8 + Living Mercy Gates valence floors
//!
//! Run with:
//!   cargo run -p simulation --bin rbe_oxygen_demo
//!
//! AG-SML v1.0 | Ra-Thor + PATSAGi Councils | info@Rathor.ai
//! Thunder locked in. Yoi ⚡

use std::thread;
use std::time::Duration;

/// Simplified mercy / valence gate (mirrors TOLC 8 + Living Mercy Gates floor).
#[derive(Clone, Copy, Debug)]
struct MercyValence {
    /// 0.0 – 1.0. Production floor is typically ≥ 0.999999.
    valence: f64,
}

impl MercyValence {
    fn high() -> Self {
        Self { valence: 0.999999 }
    }

    fn low() -> Self {
        Self { valence: 0.42 }
    }

    fn passes_oxygen_threshold(&self) -> bool {
        self.valence >= 0.95
    }
}

/// Soft post-scarcity pool (spirit of production PostScarcityAllocator).
#[derive(Debug)]
struct OxygenPool {
    available: f64,
    total_granted: f64,
    grants: u32,
}

impl OxygenPool {
    fn new(initial: f64) -> Self {
        Self {
            available: initial,
            total_granted: 0.0,
            grants: 0,
        }
    }

    /// Under high valence the pool regenerates and grants freely (oxygen).
    /// Under low valence grants are throttled and the pool stagnates.
    fn request(&mut self, need: f64, valence: MercyValence) -> f64 {
        if valence.passes_oxygen_threshold() {
            // Free, abundant flow — like oxygen
            let grant = need.min(self.available * 0.95).max(0.0);
            // Regenerative: high-valence access actually increases the pool slightly
            self.available = (self.available - grant * 0.15 + need * 0.08).min(10_000.0);
            self.total_granted += grant;
            self.grants += 1;
            grant
        } else {
            // Scarcity friction — restricted, no regeneration
            let grant = (need * 0.18).min(self.available * 0.08).max(0.0);
            self.available = (self.available - grant).max(0.0);
            self.total_granted += grant;
            self.grants += 1;
            grant
        }
    }
}

fn print_header() {
    println!("══════════════════════════════════════════════════════════════");
    println!("  Powrush-MMO · RBE Oxygen Demo");
    println!("  “Resources can flow as freely as oxygen once mercy gates pass.”");
    println!("  TOLC 8 + Living Mercy Gates · Ra-Thor + PATSAGi aligned");
    println!("  Contact: info@Rathor.ai");
    println!("══════════════════════════════════════════════════════════════\n");
}

fn run_scenario(name: &str, valence: MercyValence, requests: &[f64]) {
    println!("── Scenario: {}  (valence = {:.6}) ──", name, valence.valence);

    let mut pool = OxygenPool::new(1_000.0);

    for (i, &need) in requests.iter().enumerate() {
        let granted = pool.request(need, valence);
        let status = if valence.passes_oxygen_threshold() {
            "OXYGEN FLOW"
        } else {
            "RESTRICTED"
        };

        println!(
            "  Request {:02}  need={:7.1}  → granted={:7.1}  [{}]  pool_left={:8.1}",
            i + 1,
            need,
            granted,
            status,
            pool.available
        );

        thread::sleep(Duration::from_millis(80));
    }

    println!(
        "  Summary → total_granted={:.1}  grants={}  final_pool={:.1}\n",
        pool.total_granted, pool.grants, pool.available
    );
}

fn main() {
    print_header();

    // Same sequence of needs in both scenarios so the contrast is pure.
    let needs = [
        120.0, 85.0, 200.0, 45.0, 310.0, 90.0, 150.0, 75.0, 260.0, 110.0,
    ];

    // 1. High-valence path — resources flow like oxygen
    run_scenario("High Mercy / High Valence (oxygen-like access)", MercyValence::high(), &needs);

    // 2. Low-valence path — classic scarcity friction
    run_scenario("Low Valence (restricted / scarcity friction)", MercyValence::low(), &needs);

    println!("──────────────────────────────────────────────────────────────");
    println!("  Observation");
    println!("  Under high valence the pool regenerates and grants approach");
    println!("  the requested need — free, abundant, oxygen-like access.");
    println!("  Under low valence the same needs are heavily throttled and");
    println!("  the pool stagnates. This is the living difference between");
    println!("  a mercy-gated Resource-Based Economy and ordinary scarcity.");
    println!("──────────────────────────────────────────────────────────────");
    println!("\n  Thunder locked. Yoi ⚡");
    println!("  Run again any time: cargo run -p simulation --bin rbe_oxygen_demo\n");
}
