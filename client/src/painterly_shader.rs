//! CARD VIS-GLUE-1 — painterly MaterialExtension for painted world meshes.
//!
//! The fragment lives in `assets/shaders/painterly.wgsl`. It is loaded as Wesl
//! (Bevy 0.20 extended-material imports) from that `.wgsl` path. Registration
//! stays on [`crate::painterly_mat::PainterlyPlugin`].
//!
//! Contact: info@Rathor.ai

use bevy::asset::uuid_handle;
use bevy::pbr::{ExtendedMaterial, MaterialExtension, MaterialPlugin};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;

const PAINTERLY_SHADER: Handle<Shader> =
    uuid_handle!("a7c4e1d2-6b38-4f91-8c0d-5e2a9b7f4d13");

/// sRGB stop colours and palette, plus the linear specular reflectance the
/// fragment writes into the PBR input. `stops` is 2 on the simple presets
/// and 3 on High / Ultra.
#[derive(Clone, Copy, Debug, PartialEq, Reflect, ShaderType)]
#[reflect(Default)]
pub struct PainterlyUniform {
    pub stops: u32,
    /// sRGB multiplier at the foot.
    pub shadow: Vec4,
    /// sRGB multiplier on the body.
    pub mid: Vec4,
    /// sRGB multiplier at the crown. Read when `stops` is 3.
    pub tip: Vec4,
    /// Authored sRGB after the palette step.
    pub palette: Vec4,
    /// Linear reflectance (specular tint times the scalar) for the fragment.
    pub specular: Vec4,
}

impl Default for PainterlyUniform {
    fn default() -> Self {
        Self {
            stops: 2,
            shadow: Vec4::ONE,
            mid: Vec4::ONE,
            tip: Vec4::ONE,
            palette: Vec4::ONE,
            specular: Vec4::new(0.12, 0.12, 0.12, 1.0),
        }
    }
}

/// Extra bindings for a painted [`StandardMaterial`]. Slot 100, clear of the
/// base material's bindings.
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
#[reflect(Default)]
pub struct PainterlyExtension {
    #[uniform(100)]
    pub paint: PainterlyUniform,
}

impl MaterialExtension for PainterlyExtension {
    fn fragment_shader() -> ShaderRef {
        PAINTERLY_SHADER.into()
    }
}

/// Painted world mesh material. Bark that must stay a plain [`StandardMaterial`]
/// (trunk fade) keeps `base` only.
pub type PainterlyMaterial = ExtendedMaterial<StandardMaterial, PainterlyExtension>;

/// Insert the fragment and the extended-material plugin. Called only from
/// [`crate::painterly_mat::PainterlyPlugin`].
pub fn install(app: &mut App) {
    if let Some(mut shaders) = app.world_mut().get_resource_mut::<Assets<Shader>>() {
        let _ = shaders.insert(
            &PAINTERLY_SHADER,
            Shader::from_wesl(
                include_str!("../assets/shaders/painterly.wgsl"),
                "shaders/painterly.wgsl",
            ),
        );
    }
    app.add_plugins(MaterialPlugin::<PainterlyMaterial>::default());
}
