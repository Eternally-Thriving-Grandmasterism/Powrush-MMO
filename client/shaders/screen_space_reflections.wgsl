/*!
 * Screen Space Reflections (SSR) v18.15+ — Upgraded Prototype
 * Powrush-MMO Client Shader
 *
 * Improvements:
 * - Binary search refinement after linear ray march (much higher quality hits)
 * - Normal texture support (for accurate reflection direction)
 * - Screen-space LOD + aggressive cheap mode
 * - Epiphany-reactive intensity modulation ready
 *
 * This version is significantly more production-ready while staying lightweight.
 */

struct SSRUniforms {
    max_steps: u32,
    step_size: f32,
    thickness: f32,
    max_distance: f32,
    
    fade_start: f32,
    fade_end: f32,
    intensity: f32,
    
    screen_lod_scale: f32,
    cheap_mode_threshold: f32,
    
    // Epiphany modulation
    epiphany_boost: f32,        // > 1.0 during epiphanies
    
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
var normal_texture: texture_2d<f32>;   // Optional but recommended
@group(0) @binding(4)
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
    out.uv.y = 1.0 - out.uv.y;
    return out;
}

fn screen_to_view(uv: vec2<f32>, depth: f32) -> vec3<f32> {
    let ndc = vec4<f32>(uv * 2.0 - 1.0, depth, 1.0);
    let view_pos = uniforms.inv_projection * ndc;
    return view_pos.xyz / view_pos.w;
}

// Improved ray march with binary search refinement
fn ray_march(origin: vec3<f32>, direction: vec3<f32>) -> vec2<f32> {
    var hit_uv = vec2<f32>(-1.0);
    var current_pos = origin;
    var last_good = origin;
    
    // Linear search
    for (var i: u32 = 0u; i < uniforms.max_steps; i++) {
        current_pos += direction * uniforms.step_size;
        
        let clip = uniforms.projection * vec4<f32>(current_pos, 1.0);
        let ndc = clip.xyz / clip.w;
        let screen_uv = ndc.xy * 0.5 + 0.5;
        
        if (any(screen_uv < vec2<f32>(0.0)) || any(screen_uv > vec2<f32>(1.0))) { break; }
        
        let scene_depth = textureSample(depth_texture, linear_sampler, screen_uv);
        let scene_pos = screen_to_view(screen_uv, scene_depth);
        
        let diff = current_pos.z - scene_pos.z;
        
        if (diff > 0.0 && diff < uniforms.thickness) {
            last_good = current_pos;
            hit_uv = screen_uv;
            break;
        }
        last_good = current_pos;
    }
    
    // Binary search refinement (if we found a candidate)
    if (hit_uv.x > 0.0) {
        var low = last_good;
        var high = current_pos;
        
        for (var j: u32 = 0u; j < 6u; j++) {  // 6 refinement steps
            let mid = mix(low, high, 0.5);
            let clip = uniforms.projection * vec4<f32>(mid, 1.0);
            let ndc = clip.xyz / clip.w;
            let mid_uv = ndc.xy * 0.5 + 0.5;
            
            let scene_depth = textureSample(depth_texture, linear_sampler, mid_uv);
            let scene_pos = screen_to_view(mid_uv, scene_depth);
            
            if (mid.z > scene_pos.z) {
                low = mid;
            } else {
                high = mid;
            }
        }
        
        let final_clip = uniforms.projection * vec4<f32>(low, 1.0);
        let final_ndc = final_clip.xyz / final_clip.w;
        hit_uv = final_ndc.xy * 0.5 + 0.5;
    }
    
    return hit_uv;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    
    let depth = textureSample(depth_texture, linear_sampler, uv);
    if (depth >= 1.0) { return vec4<f32>(0.0); }
    
    let view_pos = screen_to_view(uv, depth);
    
    // Sample normal (fallback to flat if no normal texture bound)
    let normal_sample = textureSample(normal_texture, linear_sampler, uv);
    let normal = normalize(normal_sample.xyz * 2.0 - 1.0);
    
    let view_dir = normalize(view_pos);
    let reflection_dir = reflect(view_dir, normal);
    
    // Screen-space LOD + cheap mode
    let dx = dpdx(uv);
    let dy = dpdy(uv);
    let delta = max(length(dx), length(dy));
    let screen_lod = saturate(-log2(delta * uniforms.screen_lod_scale));
    
    if (screen_lod > uniforms.cheap_mode_threshold) {
        let color = textureSample(color_texture, linear_sampler, uv);
        return color * 0.25;
    }
    
    let hit_uv = ray_march(view_pos, reflection_dir);
    
    if (hit_uv.x < 0.0) { return vec4<f32>(0.0); }
    
    let reflected_color = textureSample(color_texture, linear_sampler, hit_uv);
    
    let dist = length(view_pos);
    let fade = saturate((uniforms.fade_end - dist) / (uniforms.fade_end - uniforms.fade_start));
    
    // Epiphany boost modulation
    let final_intensity = uniforms.intensity * fade * uniforms.epiphany_boost;
    
    return reflected_color * final_intensity;
}
