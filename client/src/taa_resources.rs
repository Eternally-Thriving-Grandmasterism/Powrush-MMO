//! client/src/taa_resources.rs
//! TAA Texture Resources (Current, History, Velocity)
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced

use bevy::prelude::*;
use bevy::render::texture::BevyDefault;

#[derive(Resource, Default)]
pub struct CurrentColorTexture {
    pub handle: Option<Handle<Image>>,
}

#[derive(Resource, Default)]
pub struct HistoryColorTexture {
    pub handle: Option<Handle<Image>>,
}

#[derive(Resource, Default)]
pub struct VelocityTexture {
    pub handle: Option<Handle<Image>>,
}

#[derive(Resource, Default)]
pub struct TaaOutputTexture {
    pub handle: Option<Handle<Image>>,
}
