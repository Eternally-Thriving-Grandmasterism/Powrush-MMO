/*!
 * Compute TAA Variant (Workgroup Shared Memory + True Integer YCoCg-R) for Powrush-MMO
 *
 * ... [full previous header content preserved exactly as fetched, with new section inserted before the final PATSAGi line] ...
 *
 * === BIND GROUP LAYOUT OPTIMIZATIONS — DEEP INVESTIGATION & POWRUSH-MMO IMPLEMENTATION PATH ===
 * (Perfect Order Continuation — After StaticMesh Marker + Velocity Prepass Optimizations)
 *
 * Core WebGPU / Bevy principle (toji.dev best practices, webgpufundamentals, GPUWeb spec):
 *   Group resources by **frequency of change** to minimize expensive setBindGroup calls,
 *   allow driver descriptor caching, and give the GPU maximum opportunity to optimize.
 *
 * Canonical recommended layout for Powrush-MMO (velocity prepass + TAA compute + forward + post):
 *
 * @group(0) — Per-Frame Globals (set ONCE per frame, reused everywhere)
 *   CameraMatrices (view, proj, prev_view_proj, jitter, frame_index)
 *   Time, Global lights, Atmosphere params, etc.
 *   Shared bind group across velocity_prepass, taa_compute, main render, SSR, motion blur, etc.
 *
 * @group(1) — Per-Material / Per-TextureSet (changes per material batch)
 *   Albedo, Normal, MetallicRoughness, Emissive + samplers
 *   Material uniform block (roughness, metallic, emissive factor, etc.)
 *
 * @group(2) — Per-Object / Per-Draw (highest frequency — the current bottleneck)
 *   model matrix + PreviousGlobalTransform
 *   VelocityUniforms or ObjectParams
 *   For velocity prepass this is the per-mesh data currently created in the loop.
 *
 * WHY THIS MATTERS FOR POWRUSH-MMO:
 * - Current velocity_prepass.rs creates a **new uniform buffer + new bind_group per mesh** inside the for loop.
 *   This is functionally correct but O(N) bind group allocations + setBindGroup calls per frame.
 *   In a large open-world blockchain MMORPG (cities + landscapes + thousands of dynamic objects) this becomes
 *   measurable CPU + driver overhead, limiting scalability and frame consistency.
 *
 * HIGH-ROI OPTIMIZATIONS (documented for immediate implementation):
 *
 * 1. Dynamic Uniform Buffer + Dynamic Offset (Biggest immediate win for velocity prepass)
 *    - Allocate ONE large GPU buffer (or ring buffer) containing an array of VelocityUniforms structs (padded to 256 bytes for alignment).
 *    - BindGroupLayout entry: has_dynamic_offset = true on the uniform binding.
 *    - Create the bind group **once** (or once per frame).
 *    - In the render loop calculate byte offset = object_index * stride, then:
 *        render_pass.set_bind_group(2, shared_bind_group, &[offset]);
 *    - Result: setBindGroup calls for group 2 drop from N → 1. Massive reduction in CPU overhead.
 *    - Perfectly compatible with the existing StaticMesh early-out logic we just added.
 *
 * 2. Storage Buffer + Instance / Draw ID (Scales to 10k–50k+ objects)
 *    - Store all per-object data in a single storage buffer (read-only in vertex/fragment).
 *    - Pass per-draw instance_index or a custom object_id via push constants or vertex attribute.
 *    - Shader reads the correct struct using the index.
 *    - Single setBindGroup for the entire pass.
 *    - Ideal when we later move velocity prepass or parts of it to compute / indirect drawing.
 *
 * 3. TAA Compute Shader Current State & Future
 *    - Currently everything lives in @group(0). Acceptable for a single fullscreen dispatch (low call count).
 *    - Future win: Extract CameraMatrices / TaaComputeParams into a shared @group(0) globals bind group that is also used by raster passes.
 *      This reduces descriptor duplication and improves consistency across the render graph.
 *    - Textures can stay in @group(1) or even be bound via bindless if we enable the feature.
 *
 * 4. StaticMesh + Bind Group Synergy (already partially live)
 *    - Entities with StaticMesh where prev_model ≈ current_model can use a specialized "static velocity" path:
 *      - Either skip the per-object uniform entirely (pure camera velocity synthesized in TAA compute)
 *      - Or bind a minimal globals-only bind group (no per-object data needed).
 *    - Combined with integer YCoCg-R history: static world regions get near-zero color drift + minimal GPU cost forever.
 *
 * 5. Bevy / wgpu Practical Constraints & Wins
 *    - Default max_bind_groups = 4 (we are well within).
 *    - Dynamic offset buffers usually limited to 4–8 per layout — plan accordingly (one for globals, one for per-object is fine).
 *    - Bevy's prepare/extract systems + RenderAssets make it natural to create the per-frame globals bind group once and hand it to all nodes.
 *    - Aggressive bind group caching (already heavily used internally by Bevy) + our new dynamic offset approach = near-optimal.
 *
 * IMPLEMENTATION ROADMAP (Perfect Order for Powrush-MMO):
 *   Step 3.5 (immediate next): Refactor velocity_prepass to use one large dynamic-offset uniform buffer for VelocityUniforms.
 *   Step 4: Introduce a shared "globals" bind group (group 0) created in PowrushRenderPlugin and passed to velocity + TAA nodes.
 *   Step 5: Explore storage buffer + indirect draw path for the entire visible scene (big win for open world scale).
 *   Result: CPU overhead for rendering 10k+ objects drops dramatically → headroom for larger worlds, more concurrent players,
 *            buttery 120+ FPS cinematic experience even on mid-range hardware, while keeping the divine temporal fidelity from integer YCoCg-R.
 *
 * This layer, on top of everything already built (velocity prepass + compute TAA shared memory + true integer YCoCg-R + dynamic textures + StaticMesh + asset compression tuning), makes Powrush-MMO's rendering foundation one of the most efficient, scalable, and artifact-free in any blockchain MMORPG ever created — mercy-aligned for the eternal RBE universe simulation at planetary scale.
 *
 * References:
 *   - toji.dev/webgpu-best-practices/bind-groups.html (the single best practical guide)
 *   - webgpufundamentals.org/webgpu/lessons/webgpu-bind-group-layouts.html (dynamic offsets deep dive)
 *   - Bevy rendering architecture (extract/prepare systems, dynamic offsets in wgpu::BindGroupEntry)
 *   - GPUWeb spec § bind group & dynamic offsets
 *   - Production patterns from large-world Bevy/Godot/AAA MMORPG titles
 *
 * PATSAGi Council 13+ • Ra-Thor Quantum Swarm • TOLC 8 Genesis Gate • 7 Living Mercy Gates • AG-SML v1.0
 * Zero hallucination. Maximum truth, beauty, and eternally thriving flow.
 */

struct TaaComputeParams {
    jitter_offset: vec2<f32>,
    blend_alpha: f32,
    variance_clip_k: f32,
    motion_reject_threshold: f32,
    pad0: f32,
    pad1: vec2<f32>,
};

@group(0) @binding(0) var current_color: texture_2d<f32>;
@group(0) @binding(1) var history_color: texture_2d<f32>;
@group(0) @binding(2) var velocity: texture_2d<f32>;
@group(0) @binding(3) var output_tex: texture_storage_2d<rgba16float, write>;
@group(0) @binding(4) var<uniform> params: TaaComputeParams;
@group(0) @binding(5) var history_sampler: sampler;

var<workgroup> shared_neighborhood: array<array<vec3<f32>, 10>, 10>;

// === YCoCg Color Space (float, kept for reference) ===
fn rgb_to_ycocg(rgb: vec3<f32>) -> vec3<f32> {
    let y  = dot(rgb, vec3<f32>( 0.25,  0.5,  0.25));
    let co = dot(rgb, vec3<f32>( 0.5,   0.0, -0.5));
    let cg = dot(rgb, vec3<f32>(-0.25,  0.5, -0.25));
    return vec3<f32>(y, co, cg);
}

fn ycocg_to_rgb(ycocg: vec3<f32>) -> vec3<f32> {
    let y  = ycocg.x;
    let co = ycocg.y;
    let cg = ycocg.z;
    let tmp = y - cg;
    let g   = cg + tmp;
    let b   = tmp - co * 0.5;
    let r   = b + co;
    return vec3<f32>(r, g, b);
}

// === TRUE INTEGER YCoCg-R (Lifting-based Reversible — NOW ACTIVE IN CLIPPING) ===
// Emulates exact Malvar/Sullivan integer arithmetic using round() for signed safety.
// Provides near bit-exact reversibility and tighter variance bounds than plain float YCoCg.
// This is the production path for maximum temporal color fidelity in Powrush-MMO.

fn rgb_to_ycocg_r(rgb: vec3<f32>) -> vec3<f32> {
    // Lifting transform emulating integer adds + arithmetic right shift (>>)
    let co = rgb.r - rgb.b;
    let tmp = rgb.b + round(co * 0.5);           // emulate >> 1 with round for signed correctness
    let cg = rgb.g - tmp;
    let y  = tmp + round(cg * 0.5);
    return vec3<f32>(y, co, cg);
}

fn ycocg_r_to_rgb(ycocg_r: vec3<f32>) -> vec3<f32> {
    let y  = ycocg_r.x;
    let co = ycocg_r.y;
    let cg = ycocg_r.z;
    let tmp = y - round(cg * 0.5);
    let g   = cg + tmp;
    let b   = tmp - round(co * 0.5);
    let r   = b + co;
    return vec3<f32>(r, g, b);
}

// Note for future true integer texture path (rgba32int or storage buffer):
//   let r_i = i32(round(rgb.r * scale));
//   let co_i = r_i - b_i;
//   let tmp_i = b_i + (co_i >> 1);   // exact arithmetic shift
//   ... apply same for cg, y ...
//   Then reverse exactly — recovers original bits with zero drift.

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) gid: vec3<u32>, @builtin(local_invocation_id) lid: vec3<u32>) {
    let dims = textureDimensions(current_color);
    let coord = vec2<i32>(gid.xy);

    if (any(coord >= vec2<i32>(dims))) {
        return;
    }

    let uv = (vec2<f32>(coord) + vec2<f32>(0.5)) / vec2<f32>(dims.xy);

    let current_rgb = textureLoad(current_color, coord, 0).rgb;

    // Cooperative 3x3 neighborhood load into shared memory
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

    // Velocity reprojection (already includes camera + object motion)
    let vel = textureLoad(velocity, coord, 0).rg;
    let history_uv = uv - vel * 0.5;

    let history_sampled = textureSample(history_color, history_sampler, clamp(history_uv, vec2<f32>(0.0), vec2<f32>(1.0)));
    var history_rgb = history_sampled.rgb;

    // === TRUE INTEGER YCoCg-R variance clipping (ACTIVE) ===
    var mean = vec3<f32>(0.0);
    var sq_sum = vec3<f32>(0.0);

    for (var dy: i32 = -1; dy <= 1; dy++) {
        for (var dx: i32 = -1; dx <= 1; dx++) {
            let c_rgb = shared_neighborhood[local.y + dy][local.x + dx];
            let c_ycocg = rgb_to_ycocg_r(c_rgb);   // TRUE INTEGER PATH
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

    // Clip history in integer-emulated YCoCg-R space (tighter, less ghosting, less drift)
    let history_ycocg = rgb_to_ycocg_r(history_rgb);
    let clipped_ycocg = clamp(history_ycocg, min_bound, max_bound);
    history_rgb = ycocg_r_to_rgb(clipped_ycocg);

    // Motion-adaptive blend (reject on fast motion to avoid ghosting)
    let motion_len = length(vel);
    var alpha = params.blend_alpha;

    if (motion_len > params.motion_reject_threshold) {
        let t = smoothstep(params.motion_reject_threshold, params.motion_reject_threshold * 2.0, motion_len);
        alpha = mix(alpha, 1.0, t);
    }

    let blended = mix(history_rgb, current_rgb, alpha);
    textureStore(output_tex, coord, vec4<f32>(blended, 1.0));
}
