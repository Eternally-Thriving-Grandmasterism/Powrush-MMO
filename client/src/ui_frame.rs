//! CARD FRAME-NINESLICE-1. Code-built 24×24 frame. BorderColor stays the gold line.
//! Dark inner bevel and corner ornaments, padding box, original pixels only. Contact: info@Rathor.ai

use bevy::asset::{uuid_handle, RenderAssetUsages};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

pub const FRAME_PX: u32 = 24;
pub const FRAME_SLICE: f32 = 8.0;
pub const FRAME_IMAGE: Handle<Image> = uuid_handle!("af3a9427-4150-46f7-aed8-987749def338");
pub const FRAME_LIT_IMAGE: Handle<Image> = uuid_handle!("c4b7e2a1-8f63-4d15-9a70-6e1d4b8c2f59");

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

pub fn button_frame() -> ImageNode {
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

/// L3 lit frame: same geometry, bevel lifted by the active-button step, ornaments by the spark step.
pub fn frame_lit_rgba() -> Vec<u8> {
    let plate = bytes(crate::ui_kit::TITLE_PLATE_BG);
    let gold = bytes(crate::ui_kit::TITLE_BORDER);
    let btn = bytes(crate::ui_kit::TITLE_BTN_BG);
    let active = bytes(crate::ui_kit::TITLE_BTN_ACTIVE_BG);
    let bevel = [plate[0].saturating_sub(12), plate[1].saturating_sub(12), plate[2].saturating_sub(12)];
    let ring = [active[0].saturating_sub(btn[0]), active[1].saturating_sub(btn[1]), active[2].saturating_sub(btn[2])];
    let spark = [gold[0].saturating_add(28), gold[1].saturating_add(28), gold[2].saturating_add(18)];
    let lift = [spark[0].saturating_sub(gold[0]), spark[1].saturating_sub(gold[1]), spark[2].saturating_sub(gold[2])];
    let mut buf = frame_rgba();
    for px in buf.chunks_exact_mut(4) {
        if px[3] == 0 {
            continue;
        }
        let add = if px[0] == bevel[0] && px[1] == bevel[1] && px[2] == bevel[2] { ring } else { lift };
        px[0] = px[0].saturating_add(add[0]);
        px[1] = px[1].saturating_add(add[1]);
        px[2] = px[2].saturating_add(add[2]);
    }
    buf
}

fn frame_image() -> Image {
    Image::new(Extent3d { width: FRAME_PX, height: FRAME_PX, depth_or_array_layers: 1 }, TextureDimension::D2, frame_rgba(), TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::RENDER_WORLD)
}

fn frame_lit_image() -> Image {
    Image::new(Extent3d { width: FRAME_PX, height: FRAME_PX, depth_or_array_layers: 1 }, TextureDimension::D2, frame_lit_rgba(), TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::RENDER_WORLD)
}

fn install_frame(images: Option<ResMut<Assets<Image>>>) {
    let Some(mut images) = images else { return };
    images.insert(FRAME_IMAGE.id(), frame_image()).expect("ui_frame");
    images.insert(FRAME_LIT_IMAGE.id(), frame_lit_image()).expect("ui_frame");
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
        assert_eq!(lic.world_mut().query::<&ImageNode>().iter(lic.world()).count(), 4);
    }

    fn client_prod() -> Vec<(String, String)> {
        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        std::fs::read_dir(src).unwrap().flatten().filter_map(|ent| {
            let name = ent.file_name().into_string().ok()?;
            if !name.ends_with(".rs") { return None; }
            let text = std::fs::read_to_string(ent.path()).unwrap();
            Some((name, text.split("\n#[cfg(test)]\nmod tests {").next().unwrap().to_string()))
        }).collect()
    }
    #[test]
    fn button_frame_bf1_lit_variant_pin() {
        let (rest, lit) = (frame_rgba(), frame_lit_rgba());
        assert_eq!((lit.len(), lit == frame_lit_rgba(), fnv1a64(&rest)), ((FRAME_PX * FRAME_PX * 4) as usize, true, 0xd539683111b8e69d));
        assert_eq!(fnv1a64(&lit), 0x62ea015a2591cd05);
        let px = |b: &[u8], x: u32, y: u32| { let i = ((y * FRAME_PX + x) * 4) as usize; b[i..i + 4].to_vec() };
        assert_eq!(px(&lit, 12, 12)[3], 0, "centre");
        let up = |a: &[u8], b: &[u8]| a.iter().zip(b).any(|(p, q)| p > q) && a.iter().zip(b).all(|(p, q)| p >= q);
        assert!(up(&px(&lit, 0, 12)[..3], &px(&rest, 0, 12)[..3]) && up(&px(&lit, 2, 2)[..3], &px(&rest, 2, 2)[..3]) && up(&px(&lit, 4, 4)[..3], &px(&rest, 4, 4)[..3]), "L3");
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default())).init_asset::<Image>().add_plugins(UiFramePlugin).update();
        let images = app.world().resource::<Assets<Image>>();
        assert_eq!((images.get(FRAME_IMAGE.id()).and_then(|i| i.data.as_deref()), images.get(FRAME_LIT_IMAGE.id()).and_then(|i| i.data.as_deref())), (Some(rest.as_slice()), Some(lit.as_slice())));
        let node = button_frame();
        assert_eq!((node.image.id(), node.color, node.visual_box), (FRAME_IMAGE.id(), Color::WHITE, VisualBox::PaddingBox));
        assert_eq!(node.image_mode, NodeImageMode::Sliced(frame_slicer()));
    }
    #[test]
    fn button_frame_bf8_out_of_scope_untouched() {
        let kit = include_str!("ui_kit.rs").split("\n#[cfg(test)]\nmod tests {").next().unwrap();
        assert!(kit.contains("pub fn button(bg: Color) -> (Button, Interaction, BorderColor, BackgroundColor) {\n    (Button, Interaction::default(), BorderColor::all(TITLE_BORDER), BackgroundColor(bg))\n}"));
        let calls: Vec<_> = client_prod().into_iter().filter(|(n, _)| n != "ui_frame.rs").filter_map(|(n, p)| { let c = p.matches("button_frame()").count(); (c > 0).then(|| (n, c, p.split("fn spawn_title_plate_btn").nth(1).is_some_and(|b| b.contains("button_frame()")))) }).collect();
        assert_eq!({ let mut c = calls; c.sort(); c }, vec![("age_gate.rs".into(), 1, false), ("licences_screen.rs".into(), 1, false), ("title_screen.rs".into(), 2, true)]);
        let mut gate = App::new();
        gate.insert_resource(crate::local_settings::LocalSettingsState { inner: shared::local_settings::LocalSettings::default(), dirty: false }).add_plugins((MinimalPlugins, bevy::input::InputPlugin, crate::age_gate::AgeGatePlugin)).add_message::<AppExit>().update();
        assert_eq!(gate.world_mut().query::<&ImageNode>().iter(gate.world()).count(), 6, "age-gate chip");
    }
    #[test]
    fn button_frame_bf9_one_image_writer() {
        let found: Vec<_> = client_prod().into_iter().filter_map(|(n, p)| { let c = p.matches("&mut ImageNode").count(); (c > 0).then_some((n, c)) }).collect();
        assert_eq!(found, vec![("launch_menu.rs".into(), 1)]);
        let menu = include_str!("launch_menu.rs").split("\n#[cfg(test)]\nmod tests {").next().unwrap();
        assert!(menu.contains("&mut ImageNode), With<LaunchRow>") && menu.matches("&mut BackgroundColor").count() == 2 && menu.matches("&mut BorderColor").count() == 0);
    }
    fn bf2_gate(layout: Option<(u32, u32, f32)>) -> App {
        use bevy::camera::{ComputedCameraValues, RenderTargetInfo};
        let mut app = App::new(); app.insert_resource(crate::local_settings::LocalSettingsState { inner: shared::local_settings::LocalSettings::default(), dirty: false }).add_plugins((MinimalPlugins, bevy::input::InputPlugin, crate::age_gate::AgeGatePlugin)).add_message::<AppExit>();
        if let Some((w, h, s)) = layout {
            let info = Some(RenderTargetInfo { physical_size: UVec2::new(w, h), scale_factor: s });
            app.add_plugins((AssetPlugin::default(), bevy::text::TextPlugin, bevy::ui::UiPlugin, crate::ui_font::UiFontPlugin)).init_asset::<Image>().init_asset::<TextureAtlasLayout>().add_message::<bevy::picking::pointer::PointerInput>().add_message::<bevy::picking::backend::PointerHits>().init_resource::<bevy::picking::hover::HoverMap>().init_resource::<bevy::picking::events::PointerState>();
            app.world_mut().spawn((Camera2d, Camera { computed: ComputedCameraValues { target_info: info, ..default() }, ..default() })); (app.update(), app.update());
        } else { app.update(); }
        app
    }
    fn bf2_pad(app: &mut App, button: GamepadButton) {
        use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent, RawGamepadButtonChangedEvent, RawGamepadEvent};
        let found = app.world_mut().query_filtered::<Entity, With<Gamepad>>().iter(app.world()).next();
        let id = found.unwrap_or_else(|| { let (id, name) = (app.world_mut().spawn_empty().id(), "pad".into()); app.world_mut().write_message(GamepadConnectionEvent::new(id, GamepadConnection::Connected { name, vendor_id: None, product_id: None })); app.update(); id });
        for v in [1f32, 0.] { app.world_mut().write_message(RawGamepadEvent::Button(RawGamepadButtonChangedEvent::new(id, button, v))); app.update(); }
    }
    fn bf2_label(world: &World, e: Entity) -> String { world.get::<Children>(e).and_then(|c| c.iter().find_map(|k| world.get::<Text>(k).map(|t| t.0.clone()))).unwrap_or_default() }
    fn bf2_chips(world: &mut World) -> Vec<Entity> { let mut q = world.query::<(Entity, &bevy::ui_widgets::Button)>(); q.iter(world).map(|(e, _)| e).collect() }
    #[test] fn button_frame_bf2_1_age_chips_use_frame_image() {
        let mut gate = bf2_gate(None); let world = gate.world_mut(); let want = { let n = button_frame(); (n.image.id(), n.visual_box, n.color, n.image_mode) };
        assert_eq!(world.query::<(&bevy::ui_widgets::Button, Option<&ImageNode>)>().iter(world).map(|(_, n)| n.map(|i| (i.image.id(), i.visual_box, i.color, i.image_mode.clone()))).collect::<Vec<_>>(), vec![Some(want); 6], "BF2-1");
    }
    #[test] fn button_frame_bf2_2_licences_buttons_use_frame_image() {
        let mut lic = App::new(); lic.add_plugins((MinimalPlugins, crate::licences_screen::LicencesScreenPlugin));
        lic.world_mut().commands().spawn(Node::default()).with_children(crate::licences_screen::spawn_licences_entry); lic.world_mut().flush();
        lic.world_mut().query_filtered::<&mut Interaction, With<crate::licences_screen::LicencesOpenBtn>>().iter_mut(lic.world_mut()).for_each(|mut i| *i = Interaction::Pressed); lic.update();
        assert_eq!(lic.world_mut().query::<(&bevy::ui_widgets::Button, Option<&ImageNode>)>().iter(lic.world()).map(|(_, n)| n.map(|i| i.image.id())).collect::<Vec<_>>(), vec![Some(FRAME_IMAGE.id()); 4], "BF2-2");
    }
    #[test] fn button_frame_bf2_5_age_pad_focus_stays_unlit() {
        let mut app = bf2_gate(None); bf2_pad(&mut app, GamepadButton::DPadRight); let world = app.world_mut();
        let ids: Vec<_> = bf2_chips(world).into_iter().map(|e| world.get::<ImageNode>(e).map(|n| n.image.id())).collect();
        assert!(ids.len() == 6 && ids.into_iter().all(|id| id != Some(FRAME_LIT_IMAGE.id())));
    }
    #[test] fn button_frame_bf2_6_age_sync_keeps_fill() {
        let fill = |app: &mut App, label: &str| { let w = app.world_mut(); let e = bf2_chips(w).into_iter().find(|e| bf2_label(w, *e) == label).unwrap(); w.get::<BackgroundColor>(e).unwrap().0 };
        let mut bare = bf2_gate(None); assert_eq!(fill(&mut bare, "Continue"), crate::ui_kit::TITLE_BTN_DISABLED_BG);
        bf2_pad(&mut bare, GamepadButton::DPadRight); bf2_pad(&mut bare, GamepadButton::DPadRight); assert_eq!(fill(&mut bare, "Continue"), crate::ui_kit::TITLE_BTN_DISABLED_BG, "focused, not ready");
        let mut ready = bf2_gate(None); for b in [GamepadButton::DPadUp, GamepadButton::DPadRight, GamepadButton::DPadUp, GamepadButton::DPadRight] { bf2_pad(&mut ready, b); }
        assert_eq!(fill(&mut ready, "Continue"), crate::ui_kit::TITLE_BTN_ACTIVE_BG);
        assert!(include_str!("peace_audio.rs").split("fn play_ui_click(").nth(1).unwrap().contains("b.0 == crate::ui_kit::TITLE_BTN_DISABLED_BG"));
    }
    #[test] fn button_frame_bf2_7_age_computed_node_matches_without_image() {
        let chip = Node { min_width: Val::Px(44.), height: Val::Px(44.), padding: UiRect::horizontal(Val::Px(crate::ui_kit::SPACE_XL)), align_self: AlignSelf::FlexStart, justify_content: JustifyContent::Center, align_items: AlignItems::Center, border: UiRect::all(Val::Px(crate::ui_kit::BORDER_HAIRLINE)), ..default() };
        for (w, h, s) in [(1280u32, 720u32, 1f32), (2560, 1440, 2.)] {
            let mut app = bf2_gate(Some((w, h, s))); let chips = { let world = app.world_mut(); bf2_chips(world) }; assert!(chips.iter().all(|e| app.world().get::<Node>(*e) == Some(&chip)), "{w}x{h}");
            let before: Vec<_> = chips.iter().map(|e| (app.world().get::<ComputedNode>(*e).unwrap().size(), app.world().get::<bevy::ui::UiGlobalTransform>(*e).unwrap().translation)).collect();
            for e in &chips { app.world_mut().entity_mut(*e).remove::<ImageNode>(); } app.update();
            for (e, got) in chips.iter().zip(&before) { assert_eq!(&(app.world().get::<ComputedNode>(*e).unwrap().size(), app.world().get::<bevy::ui::UiGlobalTransform>(*e).unwrap().translation), got, "{w}x{h}"); }
            assert_eq!(chips.iter().map(|e| { let n = app.world().get::<ComputedNode>(*e).unwrap(); (n.padding_box().size() * n.inverse_scale_factor / FRAME_PX as f32).min_element().min(1.) }).collect::<Vec<_>>(), vec![1.; 6], "BF2-8 age {w}x{h}");
        }
    }
    #[test] fn button_frame_bf2_9_scope_writer_and_places_hold() {
        let mut found: Vec<_> = client_prod().into_iter().filter_map(|(n, p)| { let c = p.matches("&mut ImageNode").count(); (c > 0).then_some((n, c)) }).collect(); found.sort();
        let mut lit: Vec<_> = client_prod().into_iter().filter(|(_, src)| src.contains("FRAME_LIT_IMAGE")).map(|(n, _)| n).collect(); lit.sort();
        assert_eq!((found, lit), (vec![("launch_menu.rs".into(), 1)], vec!["launch_menu.rs".into(), "ui_frame.rs".into()]));
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")); for path in [root.join("Cargo.toml"), root.join("../Cargo.toml"), root.join("../Cargo.lock")] { assert!(!std::fs::read_to_string(&path).unwrap().contains("button_frame"), "{}", path.display()); }
        let mut places = App::new(); places.add_plugins((MinimalPlugins, crate::hex_travel::HexTravelPlugin)); places.world_mut().run_schedule(Startup); assert_eq!(places.world_mut().query::<&ImageNode>().iter(places.world()).count(), 0, "places");
        assert!(include_str!("ui_kit.rs").split("\n#[cfg(test)]\nmod tests {").next().unwrap().contains("pub fn button(bg: Color) -> (Button, Interaction, BorderColor, BackgroundColor) {\n    (Button, Interaction::default(), BorderColor::all(TITLE_BORDER), BackgroundColor(bg))\n}"));
    }
}
