/*!
 * Mycelial Web Glow Shader v18.15+ (Distance-based LOD)
 * For Abyssal Depths Epiphany: Mycelium Surge
 *
 * Features:
 * - Distance-based Level of Detail for FBM/web complexity
 * - Reduces octaves and web layers for distant particles
 * - Maintains high detail up close, big performance win far away
 * - Ideal for large epiphany particle bursts
 */

struct MycelialWebGlowUniforms {
    time: f32,
    intensity: f32,
    color: vec3<f32>,
    pulse_speed: f32,
    web_scale: f32,
    glow_width: f32,
    camera_pos: vec3<f32>,
    max_lod_distance: f32,
};

@group(1) @binding(0)
var<uniform> uniforms: MycelialWebGlowUniforms;

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

fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    
    let a = hash(i);
    let b = hash(i + vec2<f32>(1.0, 0.0));
    let c = hash(i + vec2<f32>(0.0, 1.0));
    let d = hash(i + vec2<f32>(1.0, 1.0));
    
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

// FBM with variable octaves (LOD aware)
fn fbm(p: vec2<f32>, octaves: i32) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var frequency = 1.0;
    
    // Unrolled up to 5, but we stop early for distant pixels
    value += amplitude * noise(p); amplitude *= 0.5; frequency *= 2.0;
    if (octaves > 1) { value += amplitude * noise(p * frequency); amplitude *= 0.5; frequency *= 2.0; }
    if (octaves > 2) { value += amplitude * noise(p * frequency); amplitude *= 0.5; frequency *= 2.0; }
    if (octaves > 3) { value += amplitude * noise(p * frequency); amplitude *= 0.5; frequency *= 2.0; }
    if (octaves > 4) { value += amplitude * noise(p * frequency); }
    
    return value;
}

fn web_pattern(uv: vec2<f32>, scale: f32, time: f32, lod: f32) -> f32 {
    let p = uv * scale;
    
    // Reduce number of layers based on distance (LOD)
    let layer_count = i32(3.0 - lod * 1.5); // 3 layers close, down to ~1.5 far
    
    var combined = fbm(p + vec2<f32>(time * 0.1, 0.0), layer_count);
    
    if (layer_count > 1) {
        combined += fbm(p * 1.3 + vec2<f32>(-time * 0.07, time * 0.05), layer_count) * 0.7;
    }
    if (layer_count > 2) {
        combined += fbm(p * 0.7 + vec2<f32>(time * 0.03, -time * 0.04), layer_count) * 0.5;
    }
    
    combined *= 0.5;
    
    let detail = fract(combined * 8.0);
    let web = 1.0 - abs(detail - 0.5) * 2.0;
    
    return pow(saturate(web), 1.6);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let time = uniforms.time;
    
    // Distance-based LOD
    let dist = length(in.world_pos - uniforms.camera_pos);
    let lod = saturate(dist / uniforms.max_lod_distance); // 0.0 = close, 1.0 = far
    
    // Use fewer octaves and layers when far
    let octaves = max(2, 5 - i32(lod * 3.0));
    
    let web = web_pattern(uv, uniforms.web_scale, time, lod);
    
    let pulse = sin(time * uniforms.pulse_speed) * 0.5 + 0.5;
    let glow_intensity = web * uniforms.intensity * (0.75 + pulse * 0.25);
    
    let dist_falloff = 1.0 - web;
    let soft_glow = exp(-dist_falloff * 9.0 / uniforms.glow_width) * glow_intensity;
    
    // Color palette
    let t = sin(uv.x * 2.5 + time * 0.4) * 0.5 + 0.5;
    var base_color = mix(
        vec3<f32>(0.15, 0.65, 0.95),
        vec3<f32>(0.45, 0.25, 0.85),
        t
    );
    
    let vein = smoothstep(0.35, 0.65, fbm(uv * 3.5 + time * 0.15, 3));
    base_color = mix(base_color, vec3<f32>(0.25, 0.85, 0.55), vein * 0.25);
    
    let color = base_color * soft_glow;
    let alpha = soft_glow * 0.9 + web * 0.35;
    
    return vec4<f32>(color, saturate(alpha));
}
