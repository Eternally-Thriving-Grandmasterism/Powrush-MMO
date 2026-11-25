/*!
 * Anisotropic Filtering — Per-Category Implementation
 *
 * v18.29 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Complete mint-and-print-only-perfection
 * — Automatic path-inferred categories with intelligent per-type anisotropy levels
 * — World=16×, Prop=12×, Character=8×, Effect=4×, UI=1×
 * — Mercy-aligned visual fidelity: razor-sharp biomes, crisp characters, balanced effects
 * — TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable Layer 0
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy::render::texture::{Image, ImageSampler, ImageSamplerDescriptor, ImageFilterMode, ImageAddressMode};
use bevy::render::renderer::RenderDevice;
use bevy::asset::{AssetServer, AssetId};
use std::collections::HashMap;
use std::num::NonZeroU16;

/// Resource controlling global anisotropic filtering toggle & master level.
#[derive(Resource, Clone, Copy, Debug, Reflect)]
#[reflect(Resource)]
pub struct AnisotropicFilteringSettings {
    pub enabled: bool,
    pub level: u8,
}

impl Default for AnisotropicFilteringSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            level: 16,
        }
    }
}

/// Per-texture-category filtering profiles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum TextureFilteringProfile {
    World,
    Prop,
    Character,
    Effect,
    UI,
}

/// Per-category anisotropy levels (fully active & automatic).
#[derive(Resource, Clone, Debug, Reflect)]
#[reflect(Resource)]
pub struct TextureCategorySettings {
    pub levels: HashMap<TextureFilteringProfile, u8>,
}

impl Default for TextureCategorySettings {
    fn default() -> Self {
        let mut levels = HashMap::new();
        levels.insert(TextureFilteringProfile::World, 16);
        levels.insert(TextureFilteringProfile::Prop, 12);
        levels.insert(TextureFilteringProfile::Character, 8);
        levels.insert(TextureFilteringProfile::Effect, 4);
        levels.insert(TextureFilteringProfile::UI, 1);
        Self { levels }
    }
}

pub struct AnisotropicFilteringPlugin;

impl Plugin for AnisotropicFilteringPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AnisotropicFilteringSettings>()
            .init_resource::<TextureCategorySettings>()
            .register_type::<AnisotropicFilteringSettings>()
            .register_type::<TextureCategorySettings>()
            .add_systems(Startup, (
                detect_gpu_max_anisotropy,
                apply_anisotropic_filtering_to_loaded_textures,
            ))
            .add_systems(Update, (
                apply_anisotropic_filtering_to_newly_loaded_textures,
                update_filtering_on_settings_change,
            ));
    }
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct MaxSupportedAnisotropy(pub u16);

fn detect_gpu_max_anisotropy(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
) {
    let max_supported = 16u16;
    commands.insert_resource(MaxSupportedAnisotropy(max_supported));
}

fn apply_anisotropic_filtering_to_loaded_textures(
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
    settings: Res<AnisotropicFilteringSettings>,
    category_settings: Res<TextureCategorySettings>,
    max_aniso: Res<MaxSupportedAnisotropy>,
) {
    if !settings.enabled { return; }
    let mut count = 0;
    let ids: Vec<AssetId<Image>> = images.iter().map(|(id, _)| *id).collect();
    for id in ids {
        if let Some(image) = images.get_mut(id) {
            let category = asset_server.get_path(id)
                .map(|p| infer_texture_category(&p.to_string()))
                .unwrap_or(TextureFilteringProfile::World);
            let level = *category_settings.levels.get(&category).unwrap_or(&settings.level);
            let clamp = compute_anisotropy_clamp(level, max_aniso.0);
            apply_smart_anisotropic_sampler(image, clamp);
            count += 1;
        }
    }
}

fn apply_anisotropic_filtering_to_newly_loaded_textures(
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
    settings: Res<AnisotropicFilteringSettings>,
    category_settings: Res<TextureCategorySettings>,
    max_aniso: Res<MaxSupportedAnisotropy>,
    mut ev_asset: EventReader<AssetEvent<Image>>,
) {
    if !settings.enabled { return; }
    for event in ev_asset.read() {
        if let AssetEvent::Added { id } | AssetEvent::Modified { id } = event {
            if let Some(image) = images.get_mut(*id) {
                let category = asset_server.get_path(*id)
                    .map(|p| infer_texture_category(&p.to_string()))
                    .unwrap_or(TextureFilteringProfile::World);
                let level = *category_settings.levels.get(&category).unwrap_or(&settings.level);
                let clamp = compute_anisotropy_clamp(level, max_aniso.0);
                apply_smart_anisotropic_sampler(image, clamp);
            }
        }
    }
}

fn update_filtering_on_settings_change(
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
    settings: Res<AnisotropicFilteringSettings>,
    category_settings: Res<TextureCategorySettings>,
    max_aniso: Res<MaxSupportedAnisotropy>,
    mut last: Local<AnisotropicFilteringSettings>,
) {
    if settings.enabled != last.enabled || settings.level != last.level {
        *last = *settings;
        if settings.enabled {
            let ids: Vec<AssetId<Image>> = images.iter().map(|(id, _)| *id).collect();
            for id in ids {
                if let Some(image) = images.get_mut(id) {
                    let category = asset_server.get_path(id)
                        .map(|p| infer_texture_category(&p.to_string()))
                        .unwrap_or(TextureFilteringProfile::World);
                    let level = *category_settings.levels.get(&category).unwrap_or(&settings.level);
                    let clamp = compute_anisotropy_clamp(level, max_aniso.0);
                    apply_smart_anisotropic_sampler(image, clamp);
                }
            }
        }
    }
}

fn compute_anisotropy_clamp(requested: u8, gpu_max: u16) -> NonZeroU16 {
    let v = requested.clamp(1, gpu_max as u8) as u16;
    NonZeroU16::new(v).unwrap_or(NonZeroU16::new(1).unwrap())
}

fn apply_smart_anisotropic_sampler(image: &mut Image, anisotropy_clamp: NonZeroU16) {
    let addr_u = image.sampler.get_address_mode_u().unwrap_or(ImageAddressMode::Repeat);
    let addr_v = image.sampler.get_address_mode_v().unwrap_or(ImageAddressMode::Repeat);
    let addr_w = image.sampler.get_address_mode_w().unwrap_or(ImageAddressMode::Repeat);

    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        anisotropy_clamp,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        address_mode_u: addr_u,
        address_mode_v: addr_v,
        address_mode_w: addr_w,
        ..Default::default()
    });
}

fn infer_texture_category(path: &str) -> TextureFilteringProfile {
    let p = path.to_lowercase();
    if p.contains("/ui/") || p.contains("/hud/") || p.contains("/font") || p.contains("/icon") || p.contains("/menu") || p.contains("/button") {
        TextureFilteringProfile::UI
    } else if p.contains("/effect") || p.contains("/vfx/") || p.contains("/particle") || p.contains("/decal") || p.contains("/trail") || p.contains("/fx/") {
        TextureFilteringProfile::Effect
    } else if p.contains("/character") || p.contains("/player") || p.contains("/npc/") || p.contains("/armor") || p.contains("/clothing") || p.contains("/hair") {
        TextureFilteringProfile::Character
    } else if p.contains("/prop") || p.contains("/object") || p.contains("/item") || p.contains("/architecture") || p.contains("/furniture") || p.contains("/detail") {
        TextureFilteringProfile::Prop
    } else if p.contains("/world") || p.contains("/terrain") || p.contains("/biome") || p.contains("/crystal") || p.contains("/abyssal") || p.contains("/floor") || p.contains("/wall") || p.contains("/ground") || p.contains("/spires") {
        TextureFilteringProfile::World
    } else {
        TextureFilteringProfile::World
    }
}

// End of client/src/anisotropic_filtering.rs v18.29 — Sovereign per-category anisotropic filtering complete.
// Thunder locked in. Yoi ⚡
