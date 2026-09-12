/*!
 * Climate Plane — v22.7.0 + Threshold pipe/edge dress (H-2026-09-11-F3)
 *
 * PLACE_DRESS_SPEC: Threshold look/tend/door — one pipe/edge iron material
 * family + tend-seam accent so Place reads before any slab. F1 Sanctuary
 * warm-yard and F2 Heartwood living-wood stay intact (do not freestyle-reopen).
 * Four Places stay four. No mesh · no instance-portal chrome · no fifth Place.
 * Online grey. Tag 11c577e.
 * Z travel moves the place. Climate 3 = Abyssal Depths (night, close fog).
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::pbr::{FogFalloff, FogSettings};
use bevy::prelude::*;

use crate::living_practice_loop::SoftPlayerRealm;
use crate::mercy_harvest_nodes::MercyHarvestNode;

const NODE_ANCHORS: [Vec3; 3] = [
    Vec3::new(3.6, 0.55, 0.0),
    Vec3::new(-2.4, 0.55, 3.1),
    Vec3::new(1.2, 0.55, -3.4),
];

/// ART_BIBLE HANDS Sanctuary accent — warm gold well (not currency gold).
/// Rhymes with `human_presence::SANCTUARY_GOLD`; climate owns its copy so
/// this file stays the only F1 edit path.
const SANCTUARY_WELL_GOLD: Color = Color::srgb(0.86, 0.66, 0.29);

/// Shared greybox roughness for Sanctuary yard ground + path stones
/// (one material family — PLACE_DRESS_SPEC).
const SANCTUARY_YARD_ROUGHNESS: f32 = 0.90;

/// ART_BIBLE HANDS Heartwood accent — amber lamp (not Sanctuary warm-gold,
/// not currency gold). Climate owns its copy so this file stays the only F2
/// edit path.
const HEARTWOOD_AMBER_LAMP: Color = Color::srgb(0.90, 0.52, 0.14);

/// Shared living-wood roughness for Heartwood ground + ring path stones
/// (one material family — PLACE_DRESS_SPEC).
const HEARTWOOD_WOOD_ROUGHNESS: f32 = 0.82;

/// ART_BIBLE HANDS Threshold accent — iron + tend seam (not Sanctuary
/// warm-gold, not Heartwood amber, not currency gold). Climate owns its
/// copy so this file stays the only F3 edit path.
const THRESHOLD_TEND_SEAM: Color = Color::srgb(0.68, 0.36, 0.22);

/// Shared pipe/edge iron roughness for Threshold ground + edge path stones
/// (one material family — PLACE_DRESS_SPEC).
const THRESHOLD_PIPE_ROUGHNESS: f32 = 0.70;

#[derive(Clone, Copy)]
struct ClimateLook {
    name: &'static str,
    ground: Color,
    sky: Color,
    fog: Color,
    ambient: Color,
    node: Color,
    stone: Color,
    fog_start: f32,
    fog_end: f32,
    ambient_bright: f32,
    /// One roughness for ground + stone in this Place's material family.
    roughness: f32,
}

fn look_for(realm: Option<u8>) -> ClimateLook {
    match realm {
        // Verdant Heartwood — live / seal room (PLACE_DRESS_SPEC).
        // One material family: living-wood bark ground + sapwood ring paths;
        // well node is the single amber-lamp accent (ART_BIBLE). Not Sanctuary
        // warm-gold carpet, not lawn green second biome, not Market chrome.
        Some(2) => ClimateLook {
            name: "Verdant Heartwood",
            ground: Color::srgb(0.20, 0.15, 0.07),
            sky: Color::srgb(0.36, 0.55, 0.40),
            fog: Color::srgba(0.28, 0.42, 0.30, 1.0),
            ambient: Color::srgb(0.62, 0.58, 0.42),
            node: HEARTWOOD_AMBER_LAMP,
            stone: Color::srgb(0.30, 0.22, 0.11),
            fog_start: 10.0,
            fog_end: 42.0,
            ambient_bright: 260.0,
            roughness: HEARTWOOD_WOOD_ROUGHNESS,
        },
        // Threshold (Crystal Spires / Voidfarer Horizon) — look / tend / door
        // (PLACE_DRESS_SPEC). One material family: cool pipe/edge iron ground +
        // edge path stones; well node is the single tend-seam accent (ART_BIBLE
        // iron + tend seam). Not Sanctuary warm-yard, not Heartwood living-wood,
        // not instance-portal chrome, not Market / fifth Place.
        Some(4) | Some(1) => ClimateLook {
            name: if realm == Some(1) {
                "Crystal Spires"
            } else {
                "Voidfarer Horizon"
            },
            ground: Color::srgb(0.12, 0.12, 0.15),
            sky: Color::srgb(0.16, 0.18, 0.24),
            fog: Color::srgba(0.12, 0.14, 0.20, 1.0),
            ambient: Color::srgb(0.45, 0.48, 0.58),
            node: THRESHOLD_TEND_SEAM,
            stone: Color::srgb(0.20, 0.20, 0.25),
            fog_start: 12.0,
            fog_end: 40.0,
            ambient_bright: 200.0,
            roughness: THRESHOLD_PIPE_ROUGHNESS,
        },
        Some(3) => ClimateLook {
            name: "Abyssal Depths",
            ground: Color::srgb(0.04, 0.07, 0.08),
            sky: Color::srgb(0.04, 0.06, 0.09),
            fog: Color::srgba(0.03, 0.08, 0.09, 1.0),
            ambient: Color::srgb(0.18, 0.42, 0.38),
            node: Color::srgb(0.22, 0.92, 0.68),
            stone: Color::srgb(0.10, 0.16, 0.16),
            fog_start: 3.5,
            fog_end: 16.0,
            ambient_bright: 90.0,
            roughness: 0.92,
        },
        // Sanctuary Prime — warm yard / teaching Peace (PLACE_DRESS_SPEC).
        // One material family: warm grey-gold earth ground + path stones;
        // well node is the single warm-gold accent. Not Heartwood green,
        // not Brood Spire, not Market chrome.
        _ => ClimateLook {
            name: "Sanctuary Prime",
            ground: Color::srgb(0.22, 0.20, 0.16),
            sky: Color::srgb(0.72, 0.68, 0.58),
            fog: Color::srgba(0.62, 0.58, 0.48, 1.0),
            ambient: Color::srgb(0.82, 0.76, 0.62),
            node: SANCTUARY_WELL_GOLD,
            stone: Color::srgb(0.30, 0.27, 0.22),
            fog_start: 10.0,
            fog_end: 42.0,
            ambient_bright: 280.0,
            roughness: SANCTUARY_YARD_ROUGHNESS,
        },
    }
}

#[derive(Resource, Debug)]
pub struct ClimatePlane {
    pub applied: Option<u8>,
}

impl Default for ClimatePlane {
    fn default() -> Self {
        Self { applied: None }
    }
}

#[derive(Component)]
struct ClimateGround;
#[derive(Component)]
struct ClimateStone;
#[derive(Component)]
struct ClimateNameRoot;
#[derive(Component)]
struct ClimateNameText;

pub struct ClimatePlanePlugin;

impl Plugin for ClimatePlanePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClimatePlane>()
            .insert_resource(ClearColor(look_for(Some(0)).sky))
            .insert_resource(AmbientLight {
                color: look_for(Some(0)).ambient,
                brightness: look_for(Some(0)).ambient_bright,
            })
            .add_systems(Startup, (ensure_sanctuary, spawn_climate_place, spawn_climate_chip))
            .add_systems(
                Update,
                (
                    attach_fog_when_world_camera_arrives,
                    apply_climate_look,
                    update_climate_chip,
                ),
            );
    }
}

fn ensure_sanctuary(mut realm: ResMut<SoftPlayerRealm>) {
    if realm.current.is_none() {
        realm.current = Some(0);
    }
}

fn spawn_climate_place(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cameras: Query<Entity, With<Camera3d>>,
    lights: Query<Entity, With<DirectionalLight>>,
) {
    let look = look_for(Some(0));
    let ground = meshes.add(Plane3d::default().mesh().size(56.0, 56.0));
    commands.spawn((
        PbrBundle {
            mesh: ground,
            material: materials.add(StandardMaterial {
                base_color: look.ground,
                perceptual_roughness: SANCTUARY_YARD_ROUGHNESS,
                metallic: 0.0,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        ClimateGround,
        Name::new("ClimateGround"),
    ));

    let stone_mesh = meshes.add(Cylinder::new(0.18, 0.08));
    // Same roughness family as ground — greybox yard stones, not a second biome.
    let stone_mat = materials.add(StandardMaterial {
        base_color: look.stone,
        perceptual_roughness: SANCTUARY_YARD_ROUGHNESS,
        metallic: 0.0,
        ..default()
    });
    for target in NODE_ANCHORS {
        let dir = Vec3::new(target.x, 0.0, target.z);
        let steps = 4;
        for i in 1..=steps {
            let t = i as f32 / (steps as f32 + 0.35);
            let p = dir * t;
            commands.spawn((
                PbrBundle {
                    mesh: stone_mesh.clone(),
                    material: stone_mat.clone(),
                    transform: Transform::from_xyz(p.x, 0.04, p.z),
                    ..default()
                },
                ClimateStone,
            ));
        }
    }

    // Never spawn a second Camera3d here — main owns the yard camera.
    // A duplicate at order 0 races the lived UI camera on soft GPU (lavapipe)
    // and can bury Title / pause / Ledger under the world pass.
    if cameras.iter().next().is_none() {
        warn!(
            target: "powrush::climate",
            "no Camera3d yet — fog waits for world camera (ui-above-world)"
        );
    } else {
        for entity in &cameras {
            commands.entity(entity).insert(FogSettings {
                color: look.fog,
                falloff: FogFalloff::Linear {
                    start: look.fog_start,
                    end: look.fog_end,
                },
                ..default()
            });
        }
    }

    if lights.iter().next().is_none() {
        commands.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 8_500.0,
                shadows_enabled: false,
                color: Color::srgb(1.0, 0.96, 0.88),
                ..default()
            },
            transform: Transform::from_xyz(8.0, 18.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });
    }

    info!(target: "powrush::climate", "climate plane seeded — Sanctuary Prime warm yard");
}

fn spawn_climate_chip(mut commands: Commands) {
    // Existing place-name chip only — not a second HUD. Warm Sanctuary chrome
    // so the boot yard reads warm-gold before any climate slab.
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(18.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(280.0),
                    margin: UiRect::left(Val::Px(-140.0)),
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.08, 0.07, 0.05, 0.72).into(),
                border_color: Color::srgba(0.86, 0.66, 0.29, 0.40).into(),
                ..default()
            },
            ClimateNameRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "Sanctuary Prime",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.94, 0.90, 0.78),
                        ..default()
                    },
                ),
                ClimateNameText,
            ));
        });
}

fn attach_fog_when_world_camera_arrives(
    mut commands: Commands,
    cameras: Query<Entity, (With<Camera3d>, Without<FogSettings>)>,
    realm: Res<SoftPlayerRealm>,
) {
    if cameras.is_empty() {
        return;
    }
    let look = look_for(realm.current.or(Some(0)));
    for entity in &cameras {
        commands.entity(entity).insert(FogSettings {
            color: look.fog,
            falloff: FogFalloff::Linear {
                start: look.fog_start,
                end: look.fog_end,
            },
            ..default()
        });
    }
}

fn apply_climate_look(
    realm: Res<SoftPlayerRealm>,
    mut plane: ResMut<ClimatePlane>,
    mut clear: ResMut<ClearColor>,
    mut ambient: ResMut<AmbientLight>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grounds: Query<&Handle<StandardMaterial>, With<ClimateGround>>,
    stones: Query<&Handle<StandardMaterial>, With<ClimateStone>>,
    nodes: Query<&Handle<StandardMaterial>, With<MercyHarvestNode>>,
    mut fogs: Query<&mut FogSettings>,
) {
    let id = realm.current.unwrap_or(0);
    if plane.applied == Some(id) && !realm.is_changed() {
        return;
    }
    plane.applied = Some(id);
    let look = look_for(Some(id));
    clear.0 = look.sky;
    ambient.color = look.ambient;
    ambient.brightness = look.ambient_bright;

    for handle in &grounds {
        if let Some(mat) = materials.get_mut(handle) {
            mat.base_color = look.ground;
            mat.perceptual_roughness = look.roughness;
            mat.metallic = 0.0;
        }
    }
    for handle in &stones {
        if let Some(mat) = materials.get_mut(handle) {
            mat.base_color = look.stone;
            mat.perceptual_roughness = look.roughness;
            mat.metallic = 0.0;
        }
    }
    for handle in &nodes {
        if let Some(mat) = materials.get_mut(handle) {
            mat.base_color = look.node;
            mat.emissive = LinearRgba::from(look.node).with_alpha(1.0) * 2.2;
        }
    }
    for mut fog in &mut fogs {
        fog.color = look.fog;
        fog.falloff = FogFalloff::Linear {
            start: look.fog_start,
            end: look.fog_end,
        };
    }
    info!(target: "powrush::climate", climate = look.name, id, "place shifted");
}

fn update_climate_chip(
    realm: Res<SoftPlayerRealm>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    mut text_q: Query<&mut Text, With<ClimateNameText>>,
) {
    // U2 Heartwood is a disk stub — do not switch SoftPlayerRealm to 2
    // (that would dress Heartwood look onto the Sanctuary boot map).
    let name = travel
        .as_ref()
        .map(|t| t.chip_name())
        .unwrap_or_else(|| look_for(realm.current).name);
    for mut text in &mut text_q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != name {
                s.value = name.to_string();
            }
        }
    }
}

/// Presentation-copy honesty for Place dress (refuse Brood / Market / Online /
/// currency-gold / NFT). Warm-gold *color* is ART_BIBLE accent, not this copy.
fn climate_dress_copy_is_honest(s: &str) -> bool {
    let lower = s.to_ascii_lowercase();
    !(lower.contains("brood")
        || lower.contains("market")
        || lower.contains("online")
        || lower.contains("nft")
        || lower.contains("auction")
        || lower.contains("gold sink")
        || lower.contains("currency")
        || lower.contains("portal")
        || lower.contains("instance"))
}

fn srgb3(c: Color) -> (f32, f32, f32) {
    let s = c.to_srgba();
    (s.red, s.green, s.blue)
}

/// Warm-yard earth: desaturated warm grey-gold (R≈G > B), not bark and not lawn.
fn is_warm_yard_earth(c: Color) -> bool {
    let (r, g, b) = srgb3(c);
    r + 0.02 >= g
        && g > b
        && r > b
        && (r - g) <= 0.045
        && (g - b) < 0.12
        && (r - b) < 0.10
}

/// Warm-gold well accent: R > G > B with gold chroma (ART_BIBLE Sanctuary).
/// Amber lamp is warmer/orange ((r-g) larger, G lower) and must not match.
fn is_warm_gold_well(c: Color) -> bool {
    let (r, g, b) = srgb3(c);
    r > g && g > b && r > 0.7 && (r - b) > 0.35 && (r - g) <= 0.28 && g > 0.55
}

/// Living-wood / ring earth: warm bark–sapwood (R > G > B), not Sanctuary
/// grey-gold yard and not lawn where G strongly dominates.
fn is_living_wood_earth(c: Color) -> bool {
    let (r, g, b) = srgb3(c);
    r > g + 0.015
        && g > b
        && (r - b) > 0.08
        && r >= 0.14
        && !is_warm_yard_earth(c)
}

/// Amber lamp accent: warmer/orange than Sanctuary well gold (ART_BIBLE).
fn is_amber_lamp(c: Color) -> bool {
    let (r, g, b) = srgb3(c);
    r > g && g > b && r > 0.75 && (r - g) > 0.28 && (r - b) > 0.50
}

/// Pipe / edge iron: cool near-neutral steel (B ≥ G), not warm yard and not
/// living-wood bark. Says leave-enter door, not Sanctuary carpet.
fn is_pipe_edge_iron(c: Color) -> bool {
    let (r, g, b) = srgb3(c);
    let chroma = r.max(g).max(b) - r.min(g).min(b);
    chroma <= 0.08
        && b + 0.005 >= g
        && r >= 0.08
        && r <= 0.32
        && !is_warm_yard_earth(c)
        && !is_living_wood_earth(c)
}

/// Tend-seam accent: dry bronze / iron-tend (ART_BIBLE). Not Sanctuary
/// warm-gold (g lower), not Heartwood amber (r lower / less orange flare).
fn is_tend_seam(c: Color) -> bool {
    let (r, g, b) = srgb3(c);
    r > g
        && g > b
        && r > 0.55
        && r < 0.82
        && (r - g) > 0.18
        && (r - g) < 0.45
        && (r - b) > 0.30
        && g < 0.50
        && !is_amber_lamp(c)
        && !is_warm_gold_well(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn climates_disagree() {
        assert_ne!(look_for(Some(0)).name, look_for(Some(2)).name);
        assert_ne!(look_for(Some(2)).name, look_for(Some(3)).name);
        assert_eq!(look_for(Some(3)).name, "Abyssal Depths");
        assert_eq!(look_for(Some(0)).name, "Sanctuary Prime");
    }

    #[test]
    fn sanctuary_boot_does_not_use_heartwood_look() {
        assert_eq!(look_for(Some(0)).name, "Sanctuary Prime");
        assert_eq!(look_for(None).name, "Sanctuary Prime");
        assert_ne!(look_for(Some(0)).name, look_for(Some(2)).name);
    }

    #[test]
    fn sanctuary_greybox_is_one_warm_yard_family() {
        let s = look_for(Some(0));
        assert!(
            is_warm_yard_earth(s.ground),
            "Sanctuary ground must read warm yard earth, got {:?}",
            srgb3(s.ground)
        );
        assert!(
            is_warm_yard_earth(s.stone),
            "Sanctuary stones must share yard earth family, got {:?}",
            srgb3(s.stone)
        );
        assert!(
            is_warm_gold_well(s.node),
            "Sanctuary well must be warm-gold accent, got {:?}",
            srgb3(s.node)
        );
        // Ground + stone stay one family: same warm bias; path stones catch
        // a touch more light so they read as yard dressing, not a second biome.
        let (gr, gg, gb) = srgb3(s.ground);
        let (sr, sg, sb) = srgb3(s.stone);
        let g_lum = (gr + gg + gb) / 3.0;
        let s_lum = (sr + sg + sb) / 3.0;
        assert!(
            s_lum > g_lum,
            "path stones should sit slightly above ground luminance"
        );
        assert!(
            (sr - gr).abs() < 0.12 && (sg - gg).abs() < 0.12 && (sb - gb).abs() < 0.12,
            "stone drifted out of the yard earth family"
        );
        // F1 Sanctuary stays warm-yard; F2 Heartwood is living-wood — not
        // the same family, and Sanctuary must not wear Heartwood dress.
        let h = look_for(Some(2));
        assert!(
            is_living_wood_earth(h.ground),
            "Heartwood must stay living-wood, got {:?}",
            srgb3(h.ground)
        );
        assert!(!is_living_wood_earth(s.ground));
        assert_ne!(srgb3(s.ground), srgb3(h.ground));
        assert_ne!(srgb3(s.node), srgb3(h.node));
        assert!(!is_amber_lamp(s.node));
    }

    #[test]
    fn four_places_stay_four_material_moods() {
        let sanctuary = look_for(Some(0));
        let heartwood = look_for(Some(2));
        let threshold = look_for(Some(1));
        let depths = look_for(Some(3));
        assert_eq!(sanctuary.name, "Sanctuary Prime");
        assert_eq!(heartwood.name, "Verdant Heartwood");
        assert_eq!(threshold.name, "Crystal Spires");
        assert_eq!(depths.name, "Abyssal Depths");
        // Threshold realm 4 shares Threshold dress (not a fifth Place).
        assert_eq!(look_for(Some(4)).name, "Voidfarer Horizon");
        assert_ne!(srgb3(sanctuary.ground), srgb3(heartwood.ground));
        assert_ne!(srgb3(sanctuary.ground), srgb3(threshold.ground));
        assert_ne!(srgb3(sanctuary.ground), srgb3(depths.ground));
        assert_ne!(srgb3(heartwood.ground), srgb3(depths.ground));
        assert_ne!(srgb3(threshold.ground), srgb3(depths.ground));
    }

    #[test]
    fn sanctuary_well_accent_matches_art_bible_warm_gold() {
        let node = look_for(Some(0)).node;
        assert_eq!(srgb3(node), srgb3(SANCTUARY_WELL_GOLD));
        assert!(is_warm_gold_well(node));
        // Currency-gold refuse is about copy / Market — accent color is law.
        assert!(climate_dress_copy_is_honest(look_for(Some(0)).name));
    }

    #[test]
    fn climate_place_names_refuse_brood_market_online_gold() {
        for id in [0_u8, 1, 2, 3, 4] {
            let name = look_for(Some(id)).name;
            assert!(
                climate_dress_copy_is_honest(name),
                "climate name not dress-honest: {name}"
            );
            let lower = name.to_ascii_lowercase();
            assert!(!lower.contains("brood"));
            assert!(!lower.contains("market"));
            assert!(!lower.contains("online"));
            assert!(!lower.contains("gold"));
        }
    }

    #[test]
    fn place_readable_from_sanctuary_dress_before_slab() {
        // Nameable from presentation alone: warm yard + warm-gold well + name.
        let s = look_for(Some(0));
        assert_eq!(s.name, "Sanctuary Prime");
        assert!(is_warm_yard_earth(s.ground));
        assert!(is_warm_gold_well(s.node));
        assert!(s.fog_end > s.fog_start);
        assert!(climate_dress_copy_is_honest(s.name));
    }

    #[test]
    fn heartwood_greybox_is_one_living_wood_family() {
        let h = look_for(Some(2));
        assert_eq!(h.name, "Verdant Heartwood");
        assert!(
            is_living_wood_earth(h.ground),
            "Heartwood ground must read living-wood, got {:?}",
            srgb3(h.ground)
        );
        assert!(
            is_living_wood_earth(h.stone),
            "Heartwood ring paths must share living-wood family, got {:?}",
            srgb3(h.stone)
        );
        assert!(
            is_amber_lamp(h.node),
            "Heartwood well must be amber-lamp accent, got {:?}",
            srgb3(h.node)
        );
        assert_eq!(srgb3(h.node), srgb3(HEARTWOOD_AMBER_LAMP));
        assert_eq!(h.roughness, HEARTWOOD_WOOD_ROUGHNESS);
        // Ground + stone stay one family: same wood bias; ring paths catch
        // a touch more light so they read as seal-room dressing, not a second biome.
        let (gr, gg, gb) = srgb3(h.ground);
        let (sr, sg, sb) = srgb3(h.stone);
        let g_lum = (gr + gg + gb) / 3.0;
        let s_lum = (sr + sg + sb) / 3.0;
        assert!(
            s_lum > g_lum,
            "ring paths should sit slightly above ground luminance"
        );
        assert!(
            (sr - gr).abs() < 0.14 && (sg - gg).abs() < 0.14 && (sb - gb).abs() < 0.14,
            "stone drifted out of the living-wood family"
        );
        // Not Sanctuary warm-yard carpet pasted over.
        let s = look_for(Some(0));
        assert!(!is_warm_yard_earth(h.ground));
        assert!(!is_warm_gold_well(h.node));
        assert_ne!(srgb3(h.ground), srgb3(s.ground));
        assert_ne!(srgb3(h.node), srgb3(s.node));
        assert_ne!(h.roughness, s.roughness);
    }

    #[test]
    fn place_readable_from_heartwood_dress_before_slab() {
        // Nameable from presentation alone: living-wood + amber lamp + name.
        let h = look_for(Some(2));
        assert_eq!(h.name, "Verdant Heartwood");
        assert!(is_living_wood_earth(h.ground));
        assert!(is_living_wood_earth(h.stone));
        assert!(is_amber_lamp(h.node));
        assert!(h.fog_end > h.fog_start);
        assert!(climate_dress_copy_is_honest(h.name));
        let lower = h.name.to_ascii_lowercase();
        assert!(!lower.contains("brood"));
        assert!(!lower.contains("market"));
        assert!(!lower.contains("online"));
        assert!(!lower.contains("gold"));
        assert!(!lower.contains("sanctuary"));
    }

    #[test]
    fn heartwood_amber_refuses_sanctuary_gold_carpet() {
        let h = look_for(Some(2));
        let s = look_for(Some(0));
        assert_eq!(srgb3(h.node), srgb3(HEARTWOOD_AMBER_LAMP));
        assert_eq!(srgb3(s.node), srgb3(SANCTUARY_WELL_GOLD));
        assert_ne!(srgb3(HEARTWOOD_AMBER_LAMP), srgb3(SANCTUARY_WELL_GOLD));
        assert!(is_amber_lamp(h.node));
        assert!(is_warm_gold_well(s.node));
        assert!(!is_amber_lamp(s.node));
        assert!(!is_warm_gold_well(h.node));
        assert!(climate_dress_copy_is_honest(h.name));
        assert!(climate_dress_copy_is_honest(s.name));
    }

    #[test]
    fn threshold_greybox_is_one_pipe_edge_family() {
        let t = look_for(Some(1));
        assert_eq!(t.name, "Crystal Spires");
        assert!(
            is_pipe_edge_iron(t.ground),
            "Threshold ground must read pipe/edge iron, got {:?}",
            srgb3(t.ground)
        );
        assert!(
            is_pipe_edge_iron(t.stone),
            "Threshold edge paths must share pipe/edge iron family, got {:?}",
            srgb3(t.stone)
        );
        assert!(
            is_tend_seam(t.node),
            "Threshold well must be tend-seam accent, got {:?}",
            srgb3(t.node)
        );
        assert_eq!(srgb3(t.node), srgb3(THRESHOLD_TEND_SEAM));
        assert_eq!(t.roughness, THRESHOLD_PIPE_ROUGHNESS);
        // Ground + stone stay one family: same cool iron bias; edge paths catch
        // a touch more light so they read as door dressing, not a second biome.
        let (gr, gg, gb) = srgb3(t.ground);
        let (sr, sg, sb) = srgb3(t.stone);
        let g_lum = (gr + gg + gb) / 3.0;
        let s_lum = (sr + sg + sb) / 3.0;
        assert!(
            s_lum > g_lum,
            "edge paths should sit slightly above ground luminance"
        );
        assert!(
            (sr - gr).abs() < 0.14 && (sg - gg).abs() < 0.14 && (sb - gb).abs() < 0.14,
            "stone drifted out of the pipe/edge iron family"
        );
        // Realm 4 shares the same Threshold dress (not a fifth Place).
        let v = look_for(Some(4));
        assert_eq!(v.name, "Voidfarer Horizon");
        assert_eq!(srgb3(v.ground), srgb3(t.ground));
        assert_eq!(srgb3(v.stone), srgb3(t.stone));
        assert_eq!(srgb3(v.node), srgb3(t.node));
        assert_eq!(v.roughness, t.roughness);
        // Not Sanctuary warm-yard or Heartwood living-wood carpet pasted over.
        let s = look_for(Some(0));
        let h = look_for(Some(2));
        assert!(!is_warm_yard_earth(t.ground));
        assert!(!is_living_wood_earth(t.ground));
        assert!(!is_warm_gold_well(t.node));
        assert!(!is_amber_lamp(t.node));
        assert_ne!(srgb3(t.ground), srgb3(s.ground));
        assert_ne!(srgb3(t.ground), srgb3(h.ground));
        assert_ne!(srgb3(t.node), srgb3(s.node));
        assert_ne!(srgb3(t.node), srgb3(h.node));
        assert_ne!(t.roughness, s.roughness);
        assert_ne!(t.roughness, h.roughness);
    }

    #[test]
    fn place_readable_from_threshold_dress_before_slab() {
        // Nameable from presentation alone: pipe/edge iron + tend seam + name.
        let t = look_for(Some(1));
        assert_eq!(t.name, "Crystal Spires");
        assert!(is_pipe_edge_iron(t.ground));
        assert!(is_pipe_edge_iron(t.stone));
        assert!(is_tend_seam(t.node));
        assert!(t.fog_end > t.fog_start);
        assert!(climate_dress_copy_is_honest(t.name));
        let lower = t.name.to_ascii_lowercase();
        assert!(!lower.contains("brood"));
        assert!(!lower.contains("market"));
        assert!(!lower.contains("online"));
        assert!(!lower.contains("gold"));
        assert!(!lower.contains("portal"));
        assert!(!lower.contains("instance"));
        assert!(!lower.contains("sanctuary"));
        assert!(!lower.contains("heartwood"));
        let v = look_for(Some(4));
        assert!(climate_dress_copy_is_honest(v.name));
        let vlower = v.name.to_ascii_lowercase();
        assert!(!vlower.contains("portal"));
        assert!(!vlower.contains("instance"));
        assert!(!vlower.contains("market"));
    }

    #[test]
    fn threshold_tend_seam_refuses_sanctuary_and_heartwood_accents() {
        let t = look_for(Some(1));
        let s = look_for(Some(0));
        let h = look_for(Some(2));
        assert_eq!(srgb3(t.node), srgb3(THRESHOLD_TEND_SEAM));
        assert_eq!(srgb3(s.node), srgb3(SANCTUARY_WELL_GOLD));
        assert_eq!(srgb3(h.node), srgb3(HEARTWOOD_AMBER_LAMP));
        assert_ne!(srgb3(THRESHOLD_TEND_SEAM), srgb3(SANCTUARY_WELL_GOLD));
        assert_ne!(srgb3(THRESHOLD_TEND_SEAM), srgb3(HEARTWOOD_AMBER_LAMP));
        assert!(is_tend_seam(t.node));
        assert!(is_warm_gold_well(s.node));
        assert!(is_amber_lamp(h.node));
        assert!(!is_tend_seam(s.node));
        assert!(!is_tend_seam(h.node));
        assert!(!is_warm_gold_well(t.node));
        assert!(!is_amber_lamp(t.node));
        assert!(climate_dress_copy_is_honest(t.name));
        // F1 / F2 dress stays intact under F3.
        assert!(is_warm_yard_earth(s.ground));
        assert!(is_living_wood_earth(h.ground));
        assert_eq!(s.roughness, SANCTUARY_YARD_ROUGHNESS);
        assert_eq!(h.roughness, HEARTWOOD_WOOD_ROUGHNESS);
    }
}
