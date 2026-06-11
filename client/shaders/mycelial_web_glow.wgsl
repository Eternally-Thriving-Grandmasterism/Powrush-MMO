/*!
 * Mycelial Web Glow Shader v18.15+
 * For Abyssal Depths Epiphany: Mycelium Surge
 *
 * Bioluminescent glowing mycelial web effect.
 * Organic, pulsing, interconnected network of glowing threads.
 * Used for epiphany particle effects and world feedback in Abyssal Depths.
 *
 * Features:
 * - Fractal Brownian Motion (FBM) for organic web structure
 * - Pulsing glow with time-based intensity
 * - Distance-based falloff for soft edges
 * - Biome-resonant colors (deep cyan, violet, bioluminescent green)
 * - Ready for Bevy custom material or particle material
 */

struct MycelialWebGlowUniforms {
    time: f32,
    intensity: f32,
    color: vec3<f32>,
    pulse_speed: f32,
    web_scale: f32,
    glow_width: f32,
    _padding: vec2<f32>,
};

@group(1) @binding(0)
var<uniform> uniforms: MycelialWebGlowUniforms;

@group(1) @binding(1)
var texture: texture_2d<f32>;
@group(1) @binding(2)
var sampler: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) world_pos: vec3<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) normal: vec3<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(position, 1.0);
    out.uv = uv;
    out.world_pos = position;
    return out;
}

// Simple hash for noise
fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

// 2D value noise
fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let a = hash(i);
    let b = hash(i + vec2<f32>(1.0, 0.0));
    let c = hash(i + vec2<f32>(0.0, 1.0));
    let d = hash(i + vec2<f32>(1.0, 1.0));
    let u = f * f * (3.0 - 2.0 * f);
    return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}

// Fractal Brownian Motion for organic web structure
fn fbm(p: vec2<f32>) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var frequency = 1.0;
    for (var i = 0; i < 5; i = i + 1) {
        value += amplitude * noise(p * frequency);
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    return value;
}

// Creates web-like line pattern
fn web_pattern(uv: vec2<f32>, scale: f32, time: f32) -> f32 {
    let p = uv * scale;
    
    // Multiple rotated layers for interconnected web
    let layer1 = fbm(p + vec2<f32>(time * 0.1, 0.0));
    let layer2 = fbm(p * 1.3 + vec2<f32>(-time * 0.07, time * 0.05));
    let layer3 = fbm(p * 0.7 + vec2<f32>(time * 0.03, -time * 0.04));
    
    let combined = (layer1 + layer2 * 0.7 + layer3 * 0.5) / 2.2;
    
    // Create thin glowing lines by taking high frequency detail
    let lines = smoothstep(0.45, 0.55, fract(combined * 8.0));
    let web = 1.0 - abs(lines - 0.5) * 2.0;
    
    return pow(web, 1.5);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let time = uniforms.time;
    
    // Organic web structure
    let web = web_pattern(uv, uniforms.web_scale, time);
    
    // Pulsing glow
    let pulse = sin(time * uniforms.pulse_speed) * 0.5 + 0.5;
    let glow_intensity = web * uniforms.intensity * (0.7 + pulse * 0.3);
    
    // Soft radial glow falloff from web lines
    let dist = 1.0 - web;
    let glow = exp(-dist * 8.0 / uniforms.glow_width) * glow_intensity;
    
    // Bioluminescent color palette for Abyssal Depths
    // Deep cyan to violet with green highlights
    let base_color = mix(
        vec3<f32>(0.1, 0.6, 0.9),   // Cyan
        vec3<f32>(0.4, 0.2, 0.9),   // Violet
        sin(uv.x * 3.0 + time * 0.5) * 0.5 + 0.5
    );
    
    // Add subtle green mycelial tint in some areas
    let green_tint = smoothstep(0.3, 0.7, fbm(uv * 4.0 + time * 0.2));
    let final_color = mix(base_color, vec3<f32>(0.2, 0.9, 0.5), green_tint * 0.3);
    
    // Apply glow
    let color = final_color * glow;
    
    // Alpha with soft edges
    let alpha = glow * 0.85 + web * 0.4;
    
    return vec4<f32>(color, clamp(alpha, 0.0, 1.0));
}
