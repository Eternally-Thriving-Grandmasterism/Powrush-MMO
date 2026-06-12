/*!
 * poisson_disk_pcf.wgsl
 * Powrush-MMO — High Quality Poisson Disk PCF Shadow Filtering
 *
 * This shader implements Percentage Closer Filtering using a Poisson disk distribution
 * for soft, natural-looking shadows.
 *
 * Designed to be used with Bevy's custom shadow filtering pipeline.
 */

struct PoissonDiskUniforms {
    samples: array<vec2<f32>, 16>,
    sample_count: u32,
    _padding: vec3<u32>,
};

@group(1) @binding(0)
var<uniform> poisson_disk: PoissonDiskUniforms;

/// Performs Poisson Disk PCF shadow sampling
///
/// # Arguments
/// * `shadow_map` - The shadow map texture
/// * `shadow_sampler` - Comparison sampler for the shadow map
/// * `shadow_coords` - Shadow space coordinates (xy = uv, z = depth)
///
/// # Returns
/// Visibility value between 0.0 (fully in shadow) and 1.0 (fully lit)
fn poisson_disk_pcf(
    shadow_map: texture_depth_2d,
    shadow_sampler: sampler_comparison,
    shadow_coords: vec3<f32>,
) -> f32 {
    var visibility: f32 = 0.0;

    // Get texel size for proper offset scaling
    let texel_size: vec2<f32> = vec2<f32>(1.0) / vec2<f32>(textureDimensions(shadow_map));

    // Perform Poisson disk sampling
    for (var i: u32 = 0u; i < poisson_disk.sample_count; i = i + 1u) {
        let offset: vec2<f32> = poisson_disk.samples[i] * texel_size;
        visibility += textureSampleCompare(
            shadow_map,
            shadow_sampler,
            shadow_coords.xy + offset,
            shadow_coords.z
        );
    }

    return visibility / f32(poisson_disk.sample_count);
}

/// Alternative entry point that can be used when integrating as a custom
/// shadow filtering method in Bevy's render pipeline.
fn sample_shadow_poisson(shadow_coords: vec3<f32>) -> f32 {
    // This function assumes the shadow map and sampler are bound
    // in the appropriate bind group when used in Bevy's shadow pipeline.
    // For full integration, bind group layout must match Bevy's expectations.
    return 1.0; // Placeholder - real implementation requires pipeline integration
}
