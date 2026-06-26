// patsagi_economic.wgsl
// Powrush-MMO v16.9 — Real GPU PATSAGi Economic Simulation Kernel
//
// === GPU MEMORY LAYOUT CONTRACT (Rust ↔ WGSL) ===
// This file defines the exact memory layout used for GPU compute.
// It is deliberately kept in sync with the Rust side (engine/gpu_patsagi_bridge.rs).
//
// Rust side uses:
//   - #[repr(C)] + bytemuck::Pod + bytemuck::Zeroable
//   - GpuNode        (32 bytes) ↔ matches WGSL Node
//   - GpuNodeOutput  (16 bytes) ↔ matches WGSL OutputNode
//
// WGSL Alignment Rules Applied:
//   - f32                 : alignment 4, size 4
//   - array<f32, N>       : alignment 4, size 4*N
//   - vec3<f32>           : alignment 16 (avoided here for portability)
//   - Explicit padding    : used to guarantee 32-byte struct size for storage buffers
//
// Why array<f32, 3> instead of vec3<f32>?
//   - vec3<f32> has implementation-defined alignment (often 16 bytes)
//   - array<f32, 3> has predictable alignment of 4
//   - This maximizes cross-driver and cross-vendor robustness
//
// Node        total size = 32 bytes (5 f32 + 12-byte padding)
// OutputNode  total size = 16 bytes (4 f32)
//
// Storage buffer usage:
//   @group(0) @binding(0) var<storage, read_write> nodes: array<Node>;
//   @group(0) @binding(1) var<storage, read_write> output: array<OutputNode>;
//
// AG-SML v1.0 | TOLC 8 | PATSAGi + Ra-Thor aligned
// Thunder locked in. Yoi ⚡

struct Node {
    depletion: f32,
    regen_rate: f32,
    stress: f32,
    abundance_flow: f32,
    sustainability: f32,
    _padding: array<f32, 3>, // Explicit 12-byte padding → 32-byte alignment
};

// Matches Rust GpuNodeOutput exactly (16 bytes)
struct OutputNode {
    depletion: f32,
    regen_rate: f32,
    abundance_flow: f32,
    sustainability: f32,
};

@group(0) @binding(0) var<storage, read_write> nodes: array<Node>;
@group(0) @binding(1) var<storage, read_write> output: array<OutputNode>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index >= arrayLength(&nodes)) {
        return;
    }

    var node = nodes[index];

    let future_depletion = node.depletion + (node.stress * 0.02) - (node.regen_rate * 0.8);
    node.depletion = clamp(future_depletion, 0.0, 1.0);

    node.abundance_flow = (node.regen_rate * 2.0 - node.stress) * 0.5;

    node.sustainability = clamp(1.0 - node.depletion * 0.6 - node.stress * 0.3, 0.3, 1.0);

    if (node.depletion > 0.7) {
        node.stress = min(node.stress + 0.15, 1.0);
    }

    nodes[index] = node;

    output[index] = OutputNode(
        node.depletion,
        node.regen_rate,
        node.abundance_flow,
        node.sustainability
    );
}
