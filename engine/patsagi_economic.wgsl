// patsagi_economic.wgsl
// Powrush-MMO v16.7 — Real GPU PATSAGi Economic Simulation Kernel
// Proper structured output matching Rust GpuNodeOutput (bytemuck layout)
// Multi-step future prediction for resource nodes
// AG-SML v1.0 | TOLC 8

struct Node {
    depletion: f32,
    regen_rate: f32,
    stress: f32,
    abundance_flow: f32,
    sustainability: f32,
    _padding: vec3<f32>, // 32-byte alignment
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

    // === Core economic simulation step ===
    let future_depletion = node.depletion + (node.stress * 0.02) - (node.regen_rate * 0.8);
    node.depletion = clamp(future_depletion, 0.0, 1.0);

    node.abundance_flow = (node.regen_rate * 2.0 - node.stress) * 0.5;

    node.sustainability = clamp(1.0 - node.depletion * 0.6 - node.stress * 0.3, 0.3, 1.0);

    if (node.depletion > 0.7) {
        node.stress = min(node.stress + 0.15, 1.0);
    }

    nodes[index] = node;

    // Write structured output (matches GpuNodeOutput in Rust)
    output[index] = OutputNode(
        node.depletion,
        node.regen_rate,
        node.abundance_flow,
        node.sustainability
    );
}
