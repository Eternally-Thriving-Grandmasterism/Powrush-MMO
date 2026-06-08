// server/src/spatial/benchmarks.rs
// Powrush-MMO v17.0 — Spatial Partitioning Performance Benchmarks
// Simple but useful benchmarks using std::time::Instant

use std::time::{Duration, Instant};
use shared::protocol::Vec3Ser;
use super::hierarchical_grid::HierarchicalGrid;
use super::octree::Octree;

pub struct BenchmarkResult {
    pub name: String,
    pub insert_time: Duration,
    pub query_time: Duration,
    pub entities_tested: usize,
}

/// Run basic performance comparison between HierarchicalGrid and Octree
pub fn benchmark_spatial_structures(
    num_entities: usize,
    query_radius: f32,
    num_queries: usize,
) -> (BenchmarkResult, BenchmarkResult) {
    // Generate random positions
    let positions: Vec<Vec3Ser> = (0..num_entities)
        .map(|i| {
            let x = (i % 500) as f32 * 2.0;
            let z = (i / 500) as f32 * 2.0;
            Vec3Ser { x, y: 0.0, z }
        })
        .collect();

    // === HierarchicalGrid Benchmark ===
    let mut hgrid = HierarchicalGrid::with_default_levels();
    let start = Instant::now();
    for (i, pos) in positions.iter().enumerate() {
        hgrid.insert_or_update(i as u64, *pos);
    }
    let h_insert = start.elapsed();

    let start = Instant::now();
    for i in 0..num_queries {
        let center = positions[i % num_entities];
        let _ = hgrid.query_radius(&center, query_radius);
    }
    let h_query = start.elapsed();

    let h_result = BenchmarkResult {
        name: "HierarchicalGrid".to_string(),
        insert_time: h_insert,
        query_time: h_query,
        entities_tested: num_entities,
    };

    // === Octree Benchmark ===
    let world_min = Vec3Ser { x: -2000.0, y: -100.0, z: -2000.0 };
    let world_max = Vec3Ser { x: 2000.0, y: 100.0, z: 2000.0 };
    let mut octree = Octree::new(world_min, world_max, 8, 8);

    let start = Instant::now();
    for (i, pos) in positions.iter().enumerate() {
        octree.insert(i as u64, *pos);
    }
    let o_insert = start.elapsed();

    let start = Instant::now();
    for i in 0..num_queries {
        let center = positions[i % num_entities];
        let _ = octree.query_radius(&center, query_radius);
    }
    let o_query = start.elapsed();

    let o_result = BenchmarkResult {
        name: "Octree".to_string(),
        insert_time: o_insert,
        query_time: o_query,
        entities_tested: num_entities,
    };

    (h_result, o_result)
}

// Example usage (can be called from tests or a benchmark binary):
// let (h, o) = benchmark_spatial_structures(5000, 150.0, 1000);
// println!("HierarchicalGrid: {:?}", h);
// println!("Octree:         {:?}", o);
//
// Thunder locked in. Benchmarking infrastructure ready. ⚡❤️🔥
