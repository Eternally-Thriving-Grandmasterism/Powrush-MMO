//! CARD LOOK-ULTRA-PAINT-1 — Ultra-only painterly post pass.
//!
//! One [`FullscreenMaterial`] with an inline WGSL fragment shader written in-house
//! for this card (no third-party shader text). The smoothing is a from-scratch
//! "Kuwahara-lite": four quadrant means, keep the one with the least luminance
//! variance. Edges gate on luminance only; every colour change is one scalar on
//! all three channels (no hue shift). Procedural paper grain. VISUAL_TARGET L68.

use bevy::asset::uuid_handle;
use bevy::core_pipeline::fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin};
use bevy::core_pipeline::{tonemapping::tonemapping, Core3dSystems};
use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::BoxedSystem;
use bevy::post_process::bloom::bloom;
use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_resource::ShaderType;
use bevy::render::RenderApp;
use bevy::shader::ShaderRef;
use shared::local_settings::GraphicsPreset;
use crate::local_settings::LocalSettingsState;

/// Quadrant radius in pixels (each quadrant is (R+1)^2 taps).
pub const ULTRA_PAINT_RADIUS: u32 = 2;
/// Luminance-edge darkening strength (0 = none).
pub const ULTRA_PAINT_EDGE_STRENGTH: f32 = 0.25;
/// Procedural paper grain amplitude, applied as a luminance scale.
pub const ULTRA_PAINT_GRAIN: f32 = 0.02;

const ULTRA_PAINT_SHADER: Handle<Shader> = uuid_handle!("6f1c2d6e-8a43-4b55-9e0b-5d7c1a2f3e91");

const ULTRA_PAINT_WGSL: &str = r#"
struct UltraPaint { radius: u32, edge_strength: f32, grain: f32 }
@group(0) @binding(0) var src: texture_2d<f32>;
@group(0) @binding(1) var src_sampler: sampler;
@group(0) @binding(2) var<uniform> paint: UltraPaint;

fn luma(c: vec3<f32>) -> f32 { let l = dot(c, vec3<f32>(0.2126, 0.7152, 0.0722)); return l / (1.0 + l); }

fn tap(p: vec2<i32>) -> vec3<f32> {
    let size = vec2<i32>(textureDimensions(src));
    return textureLoad(src, clamp(p, vec2<i32>(0), size - 1), 0).rgb;
}

@fragment
fn fragment(@builtin(position) pos: vec4<f32>, @location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let p = vec2<i32>(pos.xy);
    let r = i32(paint.radius);
    let centre = textureLoad(src, p, 0);
    var best = centre.rgb;
    var best_var = 1e30;
    for (var q = 0; q < 4; q++) {
        let dir = vec2<i32>(select(-1, 1, (q & 1) == 1), select(-1, 1, q >= 2));
        var sum = vec3<f32>(0.0); var l1 = 0.0; var l2 = 0.0;
        for (var y = 0; y <= r; y++) {
            for (var x = 0; x <= r; x++) {
                let c = tap(p + dir * vec2<i32>(x, y));
                let l = luma(c); sum += c; l1 += l; l2 += l * l;
            }
        }
        let n = f32((r + 1) * (r + 1)); let v = l2 / n - (l1 / n) * (l1 / n);
        if (v < best_var) { best_var = v; best = sum / n; }
    }
    let e = vec2<i32>(1, 0);
    let g = vec2<f32>(luma(tap(p + e)) - luma(tap(p - e)), luma(tap(p + e.yx)) - luma(tap(p - e.yx)));
    let edge = smoothstep(0.04, 0.2, length(g));
    let crisp = mix(best, centre.rgb, edge);
    let h = fract(sin(dot(floor(pos.xy), vec2<f32>(12.9898, 78.233))) * 43758.5453);
    let scale = (1.0 - paint.edge_strength * edge) * (1.0 + (h - 0.5) * paint.grain);
    return vec4<f32>(crisp * scale, centre.a);
}
"#;

/// Painterly pass settings carried on an Ultra [`Camera3d`].
#[derive(Component, ExtractComponent, Clone, Copy, ShaderType, Debug, PartialEq)]
#[extract_app(RenderApp)]
pub struct UltraPaint {
    pub radius: u32,
    pub edge_strength: f32,
    pub grain: f32,
}

impl Default for UltraPaint {
    fn default() -> Self {
        let (radius, edge_strength, grain) = (ULTRA_PAINT_RADIUS, ULTRA_PAINT_EDGE_STRENGTH, ULTRA_PAINT_GRAIN);
        Self { radius, edge_strength, grain }
    }
}

impl FullscreenMaterial for UltraPaint {
    fn fragment_shader() -> ShaderRef {
        ULTRA_PAINT_SHADER.into()
    }

    /// Bevy's default (PostProcess, before tonemapping), plus after bloom so the
    /// order against bloom is fixed rather than ambiguous.
    fn schedule_configs(system: ScheduleConfigs<BoxedSystem>) -> ScheduleConfigs<BoxedSystem> {
        system.in_set(Core3dSystems::PostProcess).after(bloom).before(tonemapping)
    }
}

/// [`Some`] only on [`GraphicsPreset::Ultra`].
pub fn ultra_paint_for(preset: GraphicsPreset) -> Option<UltraPaint> {
    (preset == GraphicsPreset::Ultra).then(UltraPaint::default)
}

/// Mirrors `sync_ultra_volumetric`: insert on every [`Camera3d`] on Ultra, remove off Ultra.
fn sync_ultra_paint(
    mut commands: Commands,
    settings: Option<Res<LocalSettingsState>>,
    cameras: Query<(Entity, Has<UltraPaint>), With<Camera3d>>,
) {
    let preset = settings.map(|s| s.inner.graphics_preset).unwrap_or(GraphicsPreset::Medium);
    let paint = ultra_paint_for(preset);
    for (entity, has) in &cameras {
        match (paint, has) {
            (Some(paint), false) => { commands.entity(entity).insert(paint); }
            (None, true) => { commands.entity(entity).remove::<UltraPaint>(); }
            _ => {}
        }
    }
}

/// Registered on every tier; render work only runs on views carrying [`UltraPaint`].
pub struct UltraPaintPlugin;

impl Plugin for UltraPaintPlugin {
    fn build(&self, app: &mut App) {
        if let Some(mut shaders) = app.world_mut().get_resource_mut::<Assets<Shader>>() {
            let _ = shaders.insert(&ULTRA_PAINT_SHADER, Shader::from_wgsl(ULTRA_PAINT_WGSL, file!()));
        }
        app.add_plugins(FullscreenMaterialPlugin::<UltraPaint>::default())
            .add_systems(Update, sync_ultra_paint);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ultra_paint_consts_exact() {
        assert_eq!(ULTRA_PAINT_RADIUS, 2);
        assert_eq!(ULTRA_PAINT_EDGE_STRENGTH, 0.25);
        assert_eq!(ULTRA_PAINT_GRAIN, 0.02);
    }

    #[test]
    fn ultra_paint_for_some_only_on_ultra() {
        assert_eq!(GraphicsPreset::ALL.len(), 5);
        for preset in GraphicsPreset::ALL {
            assert_eq!(ultra_paint_for(preset).is_some(), preset == GraphicsPreset::Ultra);
        }
        assert_eq!(ultra_paint_for(GraphicsPreset::Ultra), Some(UltraPaint::default()));
    }

    #[test]
    fn ultra_paint_inserts_on_ultra_and_removes_on_high() {
        use shared::local_settings::LocalSettings;
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default(), UltraPaintPlugin));
        app.insert_resource(LocalSettingsState { inner: LocalSettings::default(), dirty: false });
        // Real cameras already carry SyncToRenderWorld; the extract plugin requires it.
        let sync = bevy::render::sync_world::SyncToRenderWorld::default();
        let camera = app.world_mut().spawn((Camera3d::default(), sync)).id();
        let set = |app: &mut App, p| {
            app.world_mut().resource_mut::<LocalSettingsState>().inner.set_graphics_preset(p);
            app.update();
            app.world().get::<UltraPaint>(camera).copied()
        };
        assert_eq!(set(&mut app, GraphicsPreset::High), None);
        let before = app.world().entity(camera).archetype().component_count();
        assert_eq!(set(&mut app, GraphicsPreset::Ultra), Some(UltraPaint::default()));
        assert_eq!(app.world().entity(camera).archetype().component_count(), before + 1);
        assert_eq!(set(&mut app, GraphicsPreset::High), None);
        assert_eq!(app.world().entity(camera).archetype().component_count(), before);
    }
}
