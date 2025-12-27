// Mercy lattice shader — cyan glow + vibrational lines

@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> @builtin(position) vec4<f32> {
    let pos = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0)
    );
    return vec4<f32>(pos[idx], 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = pos.xy / vec2<f32>(800.0, 600.0); // resolution
    let d = length(uv - vec2<f32>(0.5));
    let glow = 1.0 - smoothstep(0.2, 0.5, d);

    // Lattice vibration
    let angle = atan2(uv.y - 0.5, uv.x - 0.5);
    let line = fract(angle / (3.14159 / 8.0));
    let lattice = step(0.95, line) + step(line, 0.05);

    return vec4<f32>(lattice * 0.2, glow * 0.8 + lattice * 0.3, glow, 1.0);
}
