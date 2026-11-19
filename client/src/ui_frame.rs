//! CARD FRAME-NINESLICE-1. Code-built 24×24 frame. BorderColor stays the gold line.
//! Dark inner bevel and corner ornaments, padding box, original pixels only. Contact: info@Rathor.ai

use bevy::asset::{uuid_handle, RenderAssetUsages};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

pub const FRAME_PX: u32 = 24;
pub const FRAME_SLICE: f32 = 8.0;
pub const FRAME_IMAGE: Handle<Image> = uuid_handle!("af3a9427-4150-46f7-aed8-987749def338");

pub struct UiFramePlugin;

impl Plugin for UiFramePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, install_frame);
    }
}

pub fn frame_slicer() -> TextureSlicer {
    TextureSlicer { border: BorderRect::all(FRAME_SLICE), center_scale_mode: SliceScaleMode::Stretch, sides_scale_mode: SliceScaleMode::Stretch, max_corner_scale: 1.0 }
}

pub fn plate_frame() -> ImageNode {
    ImageNode { image: FRAME_IMAGE, image_mode: NodeImageMode::Sliced(frame_slicer()), visual_box: VisualBox::PaddingBox, ..default() }
}

fn bytes(color: Color) -> [u8; 3] {
    let s = color.to_srgba();
    let q = |v: f32| (v * 255.0).round() as u8;
    [q(s.red), q(s.green), q(s.blue)]
}

fn paint(buf: &mut [u8], x: u32, y: u32, rgb: [u8; 3]) {
    let i = ((y * FRAME_PX + x) * 4) as usize;
    buf[i..i + 3].copy_from_slice(&rgb);
    buf[i + 3] = 255;
}

pub fn frame_rgba() -> Vec<u8> {
    let plate = bytes(crate::ui_kit::TITLE_PLATE_BG);
    let gold = bytes(crate::ui_kit::TITLE_BORDER);
    let bevel = [plate[0].saturating_sub(12), plate[1].saturating_sub(12), plate[2].saturating_sub(12)];
    let lit = [gold[0].saturating_add(28), gold[1].saturating_add(28), gold[2].saturating_add(18)];
    let edge = FRAME_SLICE as u32;
    let mut buf = vec![0u8; (FRAME_PX * FRAME_PX * 4) as usize];
    for y in 0..FRAME_PX {
        for x in 0..FRAME_PX {
            let (nx, ny) = (x.min(FRAME_PX - 1 - x), y.min(FRAME_PX - 1 - y));
            if nx >= edge && ny >= edge {
                continue;
            } else if nx <= 1 || ny <= 1 {
                paint(&mut buf, x, y, bevel);
            } else if nx < edge && ny < edge && ((nx == 4 && ny == 4) || (nx >= 2 && ny >= 2 && ((ny <= 3 && nx <= 5) || (nx <= 3 && ny <= 5)))) {
                paint(&mut buf, x, y, if nx == 4 && ny == 4 { lit } else { gold });
            }
        }
    }
    buf
}

fn frame_image() -> Image {
    Image::new(Extent3d { width: FRAME_PX, height: FRAME_PX, depth_or_array_layers: 1 }, TextureDimension::D2, frame_rgba(), TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::RENDER_WORLD)
}

fn install_frame(images: Option<ResMut<Assets<Image>>>) {
    let Some(mut images) = images else { return };
    images.insert(FRAME_IMAGE.id(), frame_image()).expect("ui_frame");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fnv1a64(data: &[u8]) -> u64 {
        let mut hash = 0xcbf29ce484222325u64;
        for byte in data {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }

    fn colour_literals(src: &str) -> usize {
        src.match_indices("Color::").filter(|(i, _)| !src[..*i].ends_with(|c: char| c.is_alphanumeric())).count()
    }

    fn assert_sliced(image: &ImageNode) {
        assert_eq!((image.image.id(), image.visual_box), (FRAME_IMAGE.id(), VisualBox::PaddingBox));
        assert_eq!(image.image_mode, NodeImageMode::Sliced(frame_slicer()));
    }

    #[test]
    fn frame_nineslice_fn1_origin_and_determinism() {
        let bytes = frame_rgba();
        assert_eq!(bytes.len(), (FRAME_PX * FRAME_PX * 4) as usize);
        assert_eq!(bytes, frame_rgba());
        assert_eq!(fnv1a64(&bytes), 0xd539683111b8e69d);
        let at = |x: u32, y: u32| { let i = ((y * FRAME_PX + x) * 4) as usize; bytes[i..i + 4].to_vec() };
        let s = crate::ui_kit::TITLE_PLATE_BG.to_srgba();
        let q = |v: f32| (v * 255.0).round() as u8;
        let plate = [q(s.red), q(s.green), q(s.blue)];
        let bevel = at(0, 12);
        assert_eq!(at(2, 2)[3], 255, "ornament");
        assert_eq!(at(12, 12)[3], 0, "centre");
        assert!(bevel[0] < plate[0] && bevel[1] < plate[1] && bevel[2] < plate[2] && bevel[3] == 255, "bevel");
        let prod = include_str!("ui_frame.rs").split("\n#[cfg(test)]\nmod tests {").next().unwrap();
        assert_eq!(colour_literals(prod), 0);
        let mut bare = App::new();
        bare.add_plugins((MinimalPlugins, UiFramePlugin)).update();
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default())).init_asset::<Image>().add_plugins(UiFramePlugin).update();
        let image = app.world().resource::<Assets<Image>>().get(FRAME_IMAGE.id()).unwrap();
        assert_eq!((image.sampler.clone(), image.data.as_deref()), (bevy::image::ImageSampler::Default, Some(bytes.as_slice())));
    }

    #[test]
    fn frame_nineslice_fn2_corners_stay_one_to_one() {
        let slicer = frame_slicer();
        assert_eq!((FRAME_PX, FRAME_SLICE, slicer.max_corner_scale, slicer.border), (24, 8.0, 1.0, BorderRect::all(FRAME_SLICE)));
        assert!(FRAME_SLICE <= 8.0 && slicer.center_scale_mode == SliceScaleMode::Stretch && slicer.sides_scale_mode == SliceScaleMode::Stretch);
        let image = Vec2::splat(FRAME_PX as f32);
        let min_coeff = |target: Vec2| (target / image).min_element().min(slicer.max_corner_scale);
        for (name, target) in [("title", Vec2::new(416.0, 128.0)), ("title-clerk", Vec2::new(372.0, 128.0)), ("guide", Vec2::new(516.0, 41.0)), ("fab", Vec2::new(518.0, 30.0))] {
            let coeff = min_coeff(target);
            assert_eq!((coeff, FRAME_SLICE * coeff), (1.0, FRAME_SLICE), "{name} {target}");
        }
    }

    #[test]
    fn frame_nineslice_fn3_fn6_fn8_guide() {
        for assets in [false, true] {
            let mut app = App::new();
            app.add_plugins(MinimalPlugins);
            if assets { app.add_plugins(AssetPlugin::default()).init_asset::<Image>(); }
            app.add_systems(Startup, crate::first_session_guidance::spawn_guidance_strip).update();
            if assets { assert!(app.world().resource::<Assets<Image>>().get(FRAME_IMAGE.id()).is_none()); }
            let world = app.world_mut();
            let root = world.query_filtered::<Entity, With<crate::first_session_guidance::FirstSessionGuidanceStrip>>().single(world).unwrap();
            assert_sliced(world.get::<ImageNode>(root).expect("FN3"));
            let (border, bg) = world.query_filtered::<(&BorderColor, &BackgroundColor), With<crate::first_session_guidance::FirstSessionGuidanceStrip>>().single(world).unwrap();
            assert_eq!((border.top, border.right, border.bottom, border.left, bg.0), (crate::title_screen::TITLE_BORDER.with_alpha(0.82), crate::title_screen::TITLE_BORDER.with_alpha(0.82), crate::title_screen::TITLE_BORDER.with_alpha(0.82), crate::title_screen::TITLE_BORDER.with_alpha(0.82), crate::title_screen::TITLE_PLATE_BG.with_alpha(1.0)));
            let text = world.query_filtered::<(&TextFont, &TextLayout, &Node, Option<&ImageNode>), With<crate::first_session_guidance::FirstSessionGuidanceText>>().single(world).unwrap();
            let plain = TextLayout::default();
            assert_eq!((text.0.font_size, text.0.font.clone(), text.1.justify, text.1.linebreak, text.2.clone(), text.3.is_none()), (FontSize::Px(17.0 / 1.2), TextFont::default().font, plain.justify, plain.linebreak, Node::default(), true));
            assert_eq!(world.query::<&ImageNode>().iter(world).count(), 1);
        }
    }

    #[test]
    fn frame_nineslice_fn4_plate_places_and_licences() {
        assert_eq!(crate::ui_kit::plate(), (BackgroundColor(crate::ui_kit::TITLE_PLATE_BG), BorderColor::all(crate::ui_kit::TITLE_BORDER)));
        let mut places = App::new();
        places.add_plugins((MinimalPlugins, crate::hex_travel::HexTravelPlugin));
        places.world_mut().run_schedule(Startup);
        assert!(places.world_mut().query::<&Name>().iter(places.world()).any(|n| n.as_str() == "PlacesPlate"));
        assert_eq!(places.world_mut().query::<&ImageNode>().iter(places.world()).count(), 0);
        let mut lic = App::new();
        lic.add_plugins((MinimalPlugins, crate::licences_screen::LicencesScreenPlugin));
        lic.world_mut().commands().spawn(Node::default()).with_children(crate::licences_screen::spawn_licences_entry);
        lic.world_mut().flush();
        lic.world_mut().query_filtered::<&mut Interaction, With<crate::licences_screen::LicencesOpenBtn>>().iter_mut(lic.world_mut()).for_each(|mut i| *i = Interaction::Pressed);
        lic.update();
        assert!(lic.world_mut().query_filtered::<Entity, With<crate::licences_screen::LicencesRoot>>().iter(lic.world()).next().is_some());
        assert_eq!(lic.world_mut().query::<&ImageNode>().iter(lic.world()).count(), 0);
    }
}
