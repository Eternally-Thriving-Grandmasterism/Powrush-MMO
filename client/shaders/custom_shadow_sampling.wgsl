/*!
 * custom_shadow_sampling.wgsl
 * Powrush-MMO — Custom Shadow Sampling with Poisson Disk PCF
 *
 * This shader provides a drop-in replacement for Bevy's shadow sampling.
 * When Poisson Disk mode is active, it uses our high-quality PCF implementation.
 * Otherwise it falls back to Bevy's default hardware PCF.
 */

#import bevy_pbr::shadow_sampling as bevy_shadow
#import "poisson_disk_pcf.wgsl"

/// Whether to use Poisson Disk PCF (set from CPU side via specialization or uniform)
var<private> use_poisson_disk: bool = false;

/// Main shadow sampling function
/// This is the function Bevy's lighting pipeline will call.
fn sample_shadow(
    shadow_map: texture_depth_2d,
    shadow_sampler: sampler_comparison,
    shadow_coords: vec3<f32>,
) -> f32 {
    if (use_poisson_disk) {
        return poisson_disk_pcf(shadow_map, shadow_sampler, shadow_coords);
    } else {
        return bevy_shadow::sample_shadow(shadow_map, shadow_sampler, shadow_coords);
    }
}

/// Optional: Function to enable/disable Poisson Disk mode from CPU
fn set_poisson_disk_mode(enabled: bool) {
    use_poisson_disk = enabled;
}
