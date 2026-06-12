// assets/shaders/taa_compute.wgsl
// Compute TAA with Workgroup Shared Memory + True Integer YCoCg-R
// Powrush-MMO — High Quality Temporal Stability
// AG-SML v1.0 | TOLC 8 Mercy Gates aligned

// ==================== STRUCTS ====================

struct TaaComputeParams {
    jitter_offset: vec2<f32>,
    blend_alpha: f32,
    variance_clip_k: f32,
    motion_reject_threshold: f32,
    pad0: f32,
    pad1: vec2<f32>,
};

struct CameraMatrices {
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
    prev_view_proj: mat4x4<f32>,
    jitter: vec2<f32>,
    frame_index: u32,
};

struct TaaSettings {
    enable_static_optimization: u32, // 0 = off, 1 = on
    pad: vec3<u32>,
};

// ==================== BINDINGS ====================

@group(0) @binding(0) var current_color: texture_2d<f32>;
@group(0) @binding(1) var history_color: texture_2d<f32>;
@group(0) @binding(2) var velocity: texture_2d<f32>;
@group(0) @binding(3) var output_tex: texture_storage_2d<rgba16float, write>;
@group(0) @binding(4) var<uniform> params: TaaComputeParams;
@group(0) @binding(5) var history_sampler: sampler;

@group(0) @binding(6) var<uniform> camera_matrices: CameraMatrices;
@group(0) @binding(7) var<uniform> taa_settings: TaaSettings;

// ==================== SHARED MEMORY ====================

var<workgroup> shared_neighborhood: array<array<vec3<f32>, 10>, 10>;

// ==================== COLOR SPACE FUNCTIONS ====================

// True Integer YCoCg-R (lifting-based, reversible)
fn rgb_to_ycocg_r(rgb: vec3<f32>) -> vec3<f32> {
    let co = rgb.r - rgb.b;
    let tmp = rgb.b + round(co * 0.5);
    let cg = rgb.g - tmp;
    let y = tmp + round(cg * 0.5);
    return vec3<f32>(y, co, cg);
}

fn ycocg_r_to_rgb(ycocg_r: vec3<f32>) -> vec3<f32> {
    let y = ycocg_r.x;
    let co = ycocg_r.y;
    let cg = ycocg_r.z;
    let tmp = y - round(cg * 0.5);
    let g = cg + tmp;
    let b = tmp - round(co * 0.5);
    let r = b + co;
    return vec3<f32>(r, g, b);
}

// Camera-only velocity synthesis for static objects
fn compute_camera_velocity(current_ndc: vec2<f32>, prev_ndc: vec2<f32>) -> vec2<f32> {
    return current_ndc - prev_ndc;
}

// ==================== MAIN COMPUTE ====================

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) gid: vec3<u32>, @builtin(local_invocation_id) lid: vec3<u32>) {
    let dims = textureDimensions(current_color);
    let coord = vec2<i32>(gid.xy);

    if (any(coord >= vec2<i32>(dims))) {
        return;
    }

    let uv = (vec2<f32>(coord) + vec2<f32>(0.5)) / vec2<f32>(dims);

    let current_rgb = textureLoad(current_color, coord, 0).rgb;

    // === Load 3x3 neighborhood into shared memory ===
    let local = vec2<i32>(lid.xy) + vec2<i32>(1);

    for (var dy: i32 = -1; dy <= 1; dy++) {
        for (var dx: i32 = -1; dx <= 1; dx++) {
            let load_coord = coord + vec2<i32>(dx, dy);
            let clamped = clamp(load_coord, vec2<i32>(0), vec2<i32>(dims) - vec2<i32>(1));
            let c = textureLoad(current_color, clamped, 0).rgb;
            shared_neighborhood[local.y + dy][local.x + dx] = c;
        }
    }

    workgroupBarrier();

    // === Velocity + Static Object Handling ===
    var vel = textureLoad(velocity, coord, 0).rg;

    // Synthesize camera velocity for static objects
    if (taa_settings.enable_static_optimization != 0u && length(vel) < 0.0001) {
        // Simplified camera velocity synthesis (can be improved with proper reprojection)
        let current_ndc = uv * 2.0 - 1.0;
        let prev_ndc = current_ndc; // placeholder — replace with proper prev reprojection
        vel = compute_camera_velocity(current_ndc, prev_ndc);
    }

    let history_uv = uv - vel * 0.5;
    let history_sampled = textureSample(history_color, history_sampler, clamp(history_uv, vec2<f32>(0.0), vec2<f32>(1.0)));
    var history_rgb = history_sampled.rgb;

    // === TRUE INTEGER YCoCg-R Variance Clipping ===
    var mean = vec3<f32>(0.0);
    var sq_sum = vec3<f32>(0.0);

    for (var dy: i32 = -1; dy <= 1; dy++) {
        for (var dx: i32 = -1; dx <= 1; dx++) {
            let c_rgb = shared_neighborhood[local.y + dy][local.x + dx];
            let c_ycocg = rgb_to_ycocg_r(c_rgb);
            mean += c_ycocg;
            sq_sum += c_ycocg * c_ycocg;
        }
    }

    mean = mean / 9.0;
    let variance = max(vec3<f32>(0.0), (sq_sum / 9.0) - (mean * mean));
    let std_dev = sqrt(variance);

    let k = params.variance_clip_k;
    let min_bound = mean - k * std_dev;
    let max_bound = mean + k * std_dev;

    let history_ycocg = rgb_to_ycocg_r(history_rgb);
    let clipped_ycocg = clamp(history_ycocg, min_bound, max_bound);
    history_rgb = ycocg_r_to_rgb(clipped_ycocg);

    // === Motion-adaptive blending ===
    let motion_len = length(vel);
    var alpha = params.blend_alpha;

    if (motion_len > params.motion_reject_threshold) {
        let t = smoothstep(params.motion_reject_threshold, params.motion_reject_threshold * 2.0, motion_len);
        alpha = mix(alpha, 1.0, t);
    }

    let blended = mix(history_rgb, current_rgb, alpha);
    textureStore(output_tex, coord, vec4<f32>(blended, 1.0));
}
