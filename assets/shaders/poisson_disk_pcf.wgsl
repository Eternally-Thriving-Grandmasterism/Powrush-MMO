// assets/shaders/poisson_disk_pcf.wgsl
// High-Quality Poisson Disk PCF Shadow Filtering
// With support for Temporal Accumulation (future expansion)
// AG-SML v1.0 | TOLC 8 Mercy Gates aligned

struct PoissonDiskUniform {
    samples: array<vec2<f32>, 16>,
    sample_count: u32,
};

@group(1) @binding(0)
var<uniform> poisson: PoissonDiskUniform;

@group(2) @binding(0)
var shadow_map: texture_depth_2d;
@group(2) @binding(1)
var shadow_sampler: sampler_comparison;

fn poisson_disk_pcf(
    shadow_coords: vec3<f32>,
    shadow_map_size: f32,
    bias: f32
) -> f32 {
    var visibility: f32 = 0.0;
    let texel_size = 1.0 / shadow_map_size;

    for (var i: u32 = 0u; i < poisson.sample_count; i = i + 1u) {
        let offset = poisson.samples[i] * texel_size;
        let sample_coords = shadow_coords.xy + offset;

        visibility += textureSampleCompare(
            shadow_map,
            shadow_sampler,
            sample_coords,
            shadow_coords.z - bias
        );
    }

    return visibility / f32(poisson.sample_count);
}

// Main shadow function used by the lighting shader
fn get_shadow_visibility(
    world_position: vec3<f32>,
    light_view_proj: mat4x4<f32>,
    shadow_map_size: f32,
    bias: f32
) -> f32 {
    let light_space_pos = light_view_proj * vec4<f32>(world_position, 1.0);
    var shadow_coords = light_space_pos.xyz / light_space_pos.w;
    shadow_coords = shadow_coords * 0.5 + 0.5;

    // Flip Y for texture coordinates
    shadow_coords.y = 1.0 - shadow_coords.y;

    return poisson_disk_pcf(shadow_coords, shadow_map_size, bias);
}
