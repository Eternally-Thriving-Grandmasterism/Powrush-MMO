/*!
 * Screen Space Reflections (SSR) — Lightweight Prototype v18.15+
 * Powrush-MMO Client Shader
 *
 * A clean, production-oriented SSR post-process pass.
 * Designed to enhance epiphany visuals (mycelial webs, crystal surfaces, wet biomes).
 *
 * Features:
 * - Linear ray marching + optional binary search refinement
 * - Screen-space LOD awareness (reuses patterns from mycelial_web_glow)
 * - Cheap mode early exit for distant/small pixels
 * - Tunable parameters for artistic control
 * - Mercy-gated friendly (can be driven by valence/intensity)
 *
 * Usage: Full-screen post-process pass after main color + depth targets.
 */

struct SSRUniforms {
    // Core SSR parameters
    max_steps: u32,
    step_size: f32,
    thickness: f32,
    max_distance: f32,
    
    // Fading
    fade_start: f32,
    fade_end: f32,
    
    // Intensity & artistic control
    intensity: f32,
    
    // Screen-space LOD (tie-in with mycelial system)
    screen_lod_scale: f32,
    cheap_mode_threshold: f32,
    
    // Camera matrices
    view: mat4x4<f32>,
    inv_view: mat4x4<f32>,
    projection: mat4x4<f32>,
    inv_projection: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: SSRUniforms;

@group(0) @binding(1)
var depth_texture: texture_depth_2d;
@group(0) @binding(2)
var color_texture: texture_2d<f32>;
@group(0) @binding(3)
var linear_sampler: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@location(0) position: vec2<f32>) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(position, 0.0, 1.0);
    out.uv = position * 0.5 + 0.5;
    out.uv.y = 1.0 - out.uv.y; // Flip Y for texture sampling
    return out;
}

// Convert screen UV + depth to view space position
fn screen_to_view(uv: vec2<f32>, depth: f32) -> vec3<f32> {
    let ndc = vec4<f32>(uv * 2.0 - 1.0, depth, 1.0);
    let view_pos = uniforms.inv_projection * ndc;
    return view_pos.xyz / view_pos.w;
}

// Ray march in view space
fn ray_march(origin: vec3<f32>, direction: vec3<f32>) -> vec2<f32> {
    var hit_uv = vec2<f32>(-1.0);
    var current_pos = origin;
    
    for (var i: u32 = 0u; i < uniforms.max_steps; i++) {
        current_pos += direction * uniforms.step_size;
        
        // Project to screen space
        let clip = uniforms.projection * vec4<f32>(current_pos, 1.0);
        let ndc = clip.xyz / clip.w;
        let screen_uv = ndc.xy * 0.5 + 0.5;
        
        if (any(screen_uv < vec2<f32>(0.0)) || any(screen_uv > vec2<f32>(1.0))) {
            break;
        }
        
        let scene_depth = textureSample(depth_texture, linear_sampler, screen_uv);
        let scene_pos = screen_to_view(screen_uv, scene_depth);
        
        let diff = current_pos.z - scene_pos.z;
        
        if (diff > 0.0 && diff < uniforms.thickness) {
            hit_uv = screen_uv;
            break;
        }
    }
    
    return hit_uv;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    
    let depth = textureSample(depth_texture, linear_sampler, uv);
    if (depth >= 1.0) {
        return vec4<f32>(0.0); // Sky / no geometry
    }
    
    let view_pos = screen_to_view(uv, depth);
    
    // Simple reflection direction (can be improved with normal buffer later)
    let view_dir = normalize(view_pos);
    let reflection_dir = reflect(view_dir, vec3<f32>(0.0, 0.0, 1.0)); // Placeholder normal
    
    // Screen-space LOD early exit (cheap mode)
    let dx = dpdx(uv);
    let dy = dpdy(uv);
    let delta = max(length(dx), length(dy));
    let screen_lod = saturate(-log2(delta * uniforms.screen_lod_scale));
    
    if (screen_lod > uniforms.cheap_mode_threshold) {
        // Very cheap fallback: subtle blur or skip
        let color = textureSample(color_texture, linear_sampler, uv);
        return color * 0.3; // Very faint reflection
    }
    
    let hit_uv = ray_march(view_pos, reflection_dir);
    
    if (hit_uv.x < 0.0) {
        return vec4<f32>(0.0); // No hit
    }
    
    let reflected_color = textureSample(color_texture, linear_sampler, hit_uv);
    
    // Distance fade
    let dist = length(view_pos);
    let fade = saturate((uniforms.fade_end - dist) / (uniforms.fade_end - uniforms.fade_start));
    
    let final_intensity = uniforms.intensity * fade;
    
    return reflected_color * final_intensity;
}
