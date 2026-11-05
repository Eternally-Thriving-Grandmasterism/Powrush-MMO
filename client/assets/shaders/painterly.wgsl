// VIS-GLUE-1 — forward fragment for ExtendedMaterial<StandardMaterial, PainterlyExtension>.
// Height weight is vertex colour red (paint_height_ramp). Stop colours, palette,
// and the warm specular live in the extension uniform. Mobile / Low / Medium
// are two stops. High / Ultra are three stops plus the warm specular vector.

import bevy_pbr::render::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
    pbr_types::STANDARD_MATERIAL_FLAGS_UNLIT_BIT,
};

@if(VISIBILITY_RANGE_DITHER)
import bevy_pbr::render::pbr_functions::visibility_range_dither;

struct PainterlyUniform {
    stops: u32,
    shadow: vec4<f32>,
    mid: vec4<f32>,
    tip: vec4<f32>,
    palette: vec4<f32>,
    specular: vec4<f32>,
}

@group(constants::MATERIAL_BIND_GROUP) @binding(100)
var<uniform> painterly: PainterlyUniform;

fn to_linear(c: vec3<f32>) -> vec3<f32> {
    return pow(max(c, vec3<f32>(0.0)), vec3<f32>(2.2));
}

fn ramp_srgb(height: f32) -> vec3<f32> {
    let weight = clamp(height, 0.0, 1.0);
    if (painterly.stops <= 1u) {
        return vec3<f32>(1.0, 1.0, 1.0);
    }
    if (painterly.stops == 2u) {
        return mix(painterly.shadow.rgb, painterly.mid.rgb, weight);
    }
    if (weight < 0.55) {
        return mix(painterly.shadow.rgb, painterly.mid.rgb, weight / 0.55);
    }
    return mix(painterly.mid.rgb, painterly.tip.rgb, (weight - 0.55) / 0.45);
}

@fragment
fn fragment(
    vertex_output: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var varyings = vertex_output;

    @if(VISIBILITY_RANGE_DITHER)
    visibility_range_dither(varyings.position, varyings.visibility_range_dither);

    var pbr_input = pbr_input_from_standard_material(varyings, is_front);

    var t = 1.0;
    @if(VERTEX_COLORS) {
        t = clamp(varyings.color.r, 0.0, 1.0);
    }
    let albedo = to_linear(painterly.palette.rgb) * to_linear(ramp_srgb(t));
    let alpha = pbr_input.material.base_color.a;
    pbr_input.material.base_color = vec4<f32>(albedo, alpha);
    pbr_input.material.reflectance = painterly.specular.rgb;

    pbr_input.material.base_color = alpha_discard(
        pbr_input.material.flags,
        pbr_input.material.alpha_cutoff,
        pbr_input.material.base_color
    );

    var out: FragmentOutput;
    if ((pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_UNLIT_BIT) == 0u) {
        out.color = apply_pbr_lighting(pbr_input);
    } else {
        out.color = pbr_input.material.base_color;
    }

    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
