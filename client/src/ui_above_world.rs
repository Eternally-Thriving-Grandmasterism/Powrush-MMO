//! Soft-GPU UI above the 3D yard (fail beats F2 + F3 — lavapipe / Mesa).
//!
//! Bevy 0.14 draws UI on the default UI camera's graph. When a second Camera3d
//! shares order 0 (climate race vs main), or when UI rides the world camera,
//! soft backends can composite the yard *over* Title / pause / Ledger so only
//! top strings read and Play needs Digit1.
//!
//! F2: one Camera2d (order 10, ClearColorConfig::None, IsDefaultUiCamera) so
//! lived plates draw last; climate never spawns a second Camera3d.
//!
//! F3 (Settled / Continue): after hour-two Settled data + Continue into yard,
//! lavapipe could re-bury Esc pause + L face (HUD strings still readable). Soft
//! MSAA writeback across Camera3d→Camera2d + any lost IsDefaultUiCamera / order
//! stomp re-opens the bury. Fix: Msaa::Off; re-stamp UI+world camera orders
//! every frame; strip IsDefaultUiCamera from world cams; TargetCamera-bind lived
//! plates (pause / Ledger / Title / dress) to LivedUiCamera; respawn UI cam if
//! missing. No new Camera3d.
//!
//! CARD LIGHT-BLOOM-1 — [`copy_lived_ui_hdr_from_world`] runs in the same
//! Update, after [`TierBloomSet`], and copies the world `Camera3d` `hdr` onto
//! this camera. No preset of its own. `ClearColorConfig::None`, order 10,
//! `IsDefaultUiCamera`, and `Msaa::Off` stay. Contact: info@Rathor.ai

use bevy::prelude::*;
use bevy::camera::ClearColorConfig;
use bevy::render::view::{Hdr, Msaa};

use crate::climate_plane::TierBloomSet;

/// World / yard Camera3d order (drawn first).
pub const WORLD_CAMERA_ORDER: isize = 0;
/// Lived UI Camera2d order (drawn after world — soft GPU readable).
pub const UI_CAMERA_ORDER: isize = 10;

/// Title / dimmer Global z-index floor (matches title_screen TitleRoot).
pub const LIVED_UI_Z_TITLE: i32 = 120;
/// Ledger sash / I satchel Global z-index (above yard chips, under pause).
pub const LIVED_UI_Z_LEDGER: i32 = 125;
/// Pause / Settings plate Global z-index (matches title_screen SettingsStubRoot).
pub const LIVED_UI_Z_PAUSE: i32 = 130;

/// Marker on the dedicated lived UI camera (not a world Camera3d).
#[derive(Component, Debug, Clone, Copy)]
pub struct LivedUiCamera;

/// Marker on lived UI plate roots that must stay on the UI camera after Settled.
#[derive(Component, Debug, Clone, Copy)]
pub struct LivedUiPlate;

/// True when UI camera order is strictly above the world camera.
pub fn ui_camera_draws_above_world() -> bool {
    UI_CAMERA_ORDER > WORLD_CAMERA_ORDER
}

/// Pause plate z clears Title dimmer (soft-GPU stack honesty).
pub fn pause_z_above_title() -> bool {
    LIVED_UI_Z_PAUSE > LIVED_UI_Z_TITLE
}

/// Ledger / I face z clears Title dimmer but stays under pause.
pub fn ledger_z_between_title_and_pause() -> bool {
    LIVED_UI_Z_LEDGER > LIVED_UI_Z_TITLE && LIVED_UI_Z_LEDGER < LIVED_UI_Z_PAUSE
}

/// Soft GPU: MSAA writeback across world→UI cameras re-buries plates on lavapipe.
pub fn soft_gpu_msaa_is_off(msaa: Msaa) -> bool {
    matches!(msaa, Msaa::Off)
}

pub struct UiAboveWorldPlugin;

impl Plugin for UiAboveWorldPlugin {
    fn build(&self, app: &mut App) {
        // Sample4 MSAA writeback across Camera3d → Camera2d re-buries mid-screen
        // plates on lavapipe after Settled/Continue (F3). Off keeps UI honest.
        app.add_systems(Startup, spawn_lived_ui_camera)
            .add_systems(
                Update,
                (
                    ensure_lived_ui_camera,
                    stamp_soft_gpu_msaa,
                    stamp_world_camera_order,
                    stamp_lived_ui_camera,
                    strip_world_default_ui_camera,
                    bind_lived_ui_plates,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                copy_lived_ui_hdr_from_world
                    .after(TierBloomSet)
                    .after(stamp_world_camera_order)
                    .after(stamp_lived_ui_camera),
            );
    }
}

fn spawn_lived_ui_camera(mut commands: Commands) {
    spawn_ui_camera_entity(&mut commands);
}

fn spawn_ui_camera_entity(commands: &mut Commands) {
    commands.spawn((
        (
            Camera2d,
            Camera {
                order: UI_CAMERA_ORDER,
                // Keep the 3D yard; only composite Bevy UI on top.
                clear_color: ClearColorConfig::None,
                ..default()
            },
            Msaa::Off,
        ),
        IsDefaultUiCamera,
        LivedUiCamera,
        Name::new("LivedUiCamera"),
    ));
    info!(
        target: "powrush::ui",
        ui_order = UI_CAMERA_ORDER,
        world_order = WORLD_CAMERA_ORDER,
        "lived UI camera above world (soft GPU)"
    );
}

/// Settled / Continue must not leave the yard without a UI camera.
fn ensure_lived_ui_camera(
    existing: Query<Entity, With<LivedUiCamera>>,
    mut commands: Commands,
) {
    if existing.is_empty() {
        warn!(
            target: "powrush::ui",
            "LivedUiCamera missing — respawning (ui-above-world after Settled)"
        );
        spawn_ui_camera_entity(&mut commands);
    }
}

/// Bevy 0.15 moved `Msaa` off the global resource onto each camera.
/// The old resource was `Msaa::Off` for every camera (soft-GPU writeback).
fn stamp_soft_gpu_msaa(mut cameras: Query<&mut Msaa>) {
    for mut msaa in &mut cameras {
        if *msaa != Msaa::Off {
            *msaa = Msaa::Off;
        }
    }
}

/// Keep every Camera3d on the world order so it never races the UI camera.
fn stamp_world_camera_order(
    mut cams: Query<&mut Camera, (With<Camera3d>, Without<LivedUiCamera>)>,
) {
    for mut cam in &mut cams {
        if cam.order != WORLD_CAMERA_ORDER {
            cam.order = WORLD_CAMERA_ORDER;
        }
    }
}

/// Re-assert UI camera order / clear / active after Settled path stomps.
fn stamp_lived_ui_camera(
    mut cams: Query<&mut Camera, With<LivedUiCamera>>,
    missing_marker: Query<Entity, (With<LivedUiCamera>, Without<IsDefaultUiCamera>)>,
    mut commands: Commands,
) {
    for mut cam in &mut cams {
        if cam.order != UI_CAMERA_ORDER {
            cam.order = UI_CAMERA_ORDER;
        }
        if !matches!(cam.clear_color, ClearColorConfig::None) {
            cam.clear_color = ClearColorConfig::None;
        }
        if !cam.is_active {
            cam.is_active = true;
        }
    }
    for entity in &missing_marker {
        commands.entity(entity).insert(IsDefaultUiCamera);
    }
}

/// World Camera3d must never steal IsDefaultUiCamera from LivedUiCamera.
fn strip_world_default_ui_camera(
    worlds: Query<Entity, (With<Camera3d>, With<IsDefaultUiCamera>, Without<LivedUiCamera>)>,
    mut commands: Commands,
) {
    for entity in &worlds {
        commands.entity(entity).remove::<IsDefaultUiCamera>();
    }
}

/// CARD LIGHT-BLOOM-1 — same Update as [`TierBloomSet`], ordered after it.
/// The lived UI camera takes the world `Camera3d` `hdr` and nothing else.
/// No graphics preset. Order, clear, and `IsDefaultUiCamera` stay on the
/// stamp systems.
fn copy_lived_ui_hdr_from_world(
    mut commands: Commands,
    world_cams: Query<Has<Hdr>, (With<Camera3d>, Without<LivedUiCamera>)>,
    ui_cams: Query<(Entity, Has<Hdr>), With<LivedUiCamera>>,
) {
    let Ok(world_hdr) = world_cams.single() else {
        return;
    };
    for (entity, ui_hdr) in &ui_cams {
        if world_hdr && !ui_hdr {
            commands.entity(entity).insert(Hdr);
        } else if !world_hdr && ui_hdr {
            commands.entity(entity).remove::<Hdr>();
        }
    }
}

/// Parent lived plates to the UI camera so Settled HUD chips cannot retarget them.
fn bind_lived_ui_plates(
    ui_cam: Query<Entity, With<LivedUiCamera>>,
    roots: Query<Entity, (With<LivedUiPlate>, Without<UiTargetCamera>)>,
    mut commands: Commands,
) {
    let Ok(cam) = ui_cam.single() else {
        return;
    };
    for entity in &roots {
        commands.entity(entity).insert(UiTargetCamera(cam));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ui_order_above_world() {
        assert!(ui_camera_draws_above_world());
        assert_eq!(WORLD_CAMERA_ORDER, 0);
        assert_eq!(UI_CAMERA_ORDER, 10);
    }

    #[test]
    fn lived_z_stack_title_ledger_pause() {
        assert!(pause_z_above_title());
        assert!(ledger_z_between_title_and_pause());
        assert_eq!(LIVED_UI_Z_TITLE, 120);
        assert_eq!(LIVED_UI_Z_LEDGER, 125);
        assert_eq!(LIVED_UI_Z_PAUSE, 130);
    }

    #[test]
    fn soft_gpu_msaa_off_for_multi_camera() {
        assert!(soft_gpu_msaa_is_off(Msaa::Off));
        assert!(!soft_gpu_msaa_is_off(Msaa::Sample4));
    }

    #[test]
    fn plugin_stamps_msaa_off_and_ui_order() {
        use bevy::MinimalPlugins;
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(UiAboveWorldPlugin);
        app.update();
        let mut msaa = app.world_mut().query_filtered::<&Msaa, With<LivedUiCamera>>();
        assert!(soft_gpu_msaa_is_off(*msaa.single(app.world()).unwrap()));
        assert!(ui_camera_draws_above_world());
    }

    /// CARD LIGHT-BLOOM-1 — UI `hdr` matches the world camera on the first
    /// frame and on the frame a tier changes. Real plugins. Msaa stays Off.
    /// Clear and order stay.
    #[test]
    fn lived_ui_hdr_matches_world_camera_on_every_tier() {
        use crate::climate_plane::ClimatePlanePlugin;
        use crate::living_practice_loop::SoftPlayerRealm;
        use crate::local_settings::LocalSettingsState;
        use shared::local_settings::{GraphicsPreset, LocalSettings};

        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<SoftPlayerRealm>()
            .insert_resource(LocalSettingsState {
                inner: {
                    let mut settings = LocalSettings::default();
                    settings.set_graphics_preset(GraphicsPreset::High);
                    settings
                },
                dirty: false,
            })
            .add_plugins((UiAboveWorldPlugin, ClimatePlanePlugin));
        let world_cam = app
            .world_mut()
            .spawn((
                Camera3d::default(),
                Camera {
                    order: WORLD_CAMERA_ORDER,
                    ..default()
                },
                Msaa::Off,
            ))
            .id();

        app.update();
        assert_hdr_pair(&app, world_cam, true, "High first frame");

        for preset in GraphicsPreset::ALL {
            app.world_mut()
                .resource_mut::<LocalSettingsState>()
                .inner
                .set_graphics_preset(preset);
            app.update();
            // CARD VP-BLOOM-MED-1 — Medium joins High / Ultra: hdr on, world
            // bloom on (gentle), UI camera still no BloomSettings.
            let expect = preset == GraphicsPreset::Medium
                || preset == GraphicsPreset::High
                || preset == GraphicsPreset::Ultra;
            assert_hdr_pair(&app, world_cam, expect, preset.label());
        }
    }

    /// CARD VP-GRADE-1 — the lived UI camera stays bare: no tonemapping
    /// other than `Tonemapping::None`, no `ColorGrading`, on every tier.
    /// The world camera, spawned as `main.rs` does, keeps AgX + the grade.
    /// `hdr` is not pinned here: it follows the world camera (Medium / High / Ultra).
    #[test]
    fn vp_grade_lived_ui_camera_has_no_tonemapping_or_grade() {
        use crate::climate_plane::{world_color_grading, ClimatePlanePlugin, WORLD_TONEMAPPING};
        use crate::living_practice_loop::SoftPlayerRealm;
        use crate::local_settings::LocalSettingsState;
        use bevy::core_pipeline::tonemapping::Tonemapping;
        use bevy::render::view::ColorGrading;
        use shared::local_settings::{GraphicsPreset, LocalSettings};

        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<SoftPlayerRealm>()
            .insert_resource(LocalSettingsState {
                inner: LocalSettings::default(),
                dirty: false,
            })
            .add_plugins((UiAboveWorldPlugin, ClimatePlanePlugin));
        let world_cam = app
            .world_mut()
            .spawn((
                Camera3d::default(),
                Camera {
                    order: WORLD_CAMERA_ORDER,
                    ..default()
                },
                WORLD_TONEMAPPING,
                world_color_grading(),
                Msaa::Off,
            ))
            .id();

        for preset in GraphicsPreset::ALL {
            app.world_mut()
                .resource_mut::<LocalSettingsState>()
                .inner
                .set_graphics_preset(preset);
            app.update();
            let world_ref = app.world();
            let mut ui_seen = 0;
            for entity in world_ref.iter_entities() {
                if !entity.contains::<LivedUiCamera>() {
                    continue;
                }
                ui_seen += 1;
                let tonemapping = entity.get::<Tonemapping>().copied();
                assert!(
                    matches!(tonemapping, None | Some(Tonemapping::None)),
                    "{} ui tonemapping {tonemapping:?}",
                    preset.label()
                );
                assert!(
                    !entity.contains::<ColorGrading>(),
                    "{} ui has ColorGrading",
                    preset.label()
                );
            }
            assert_eq!(ui_seen, 1, "{} one lived UI camera", preset.label());
            assert_eq!(
                world_ref.get::<Tonemapping>(world_cam).copied(),
                Some(Tonemapping::AgX),
                "{} world tonemapping",
                preset.label()
            );
            let grade = world_ref.get::<ColorGrading>(world_cam).expect("world grade");
            assert_eq!(
                grade.global.post_saturation,
                world_color_grading().global.post_saturation
            );
            assert_eq!(grade.midtones.saturation, world_color_grading().midtones.saturation);
        }
    }

    fn assert_hdr_pair(app: &App, world_cam: Entity, expect_hdr: bool, label: &str) {
        use bevy::post_process::bloom::Bloom;
        use bevy::camera::ClearColorConfig;

        let world_ref = app.world();
        assert!(world_ref.get::<Camera>(world_cam).is_some(), "{label} world camera");
        let world_hdr = world_ref.get::<Hdr>(world_cam).is_some();
        assert_eq!(world_hdr, expect_hdr, "{label} world hdr");
        let mut ui_hdr = None;
        let mut ui_order = None;
        let mut ui_clear_none = false;
        let mut bloom_on_ui = false;
        for entity in world_ref.iter_entities() {
            if entity.contains::<LivedUiCamera>() {
                let cam = entity.get::<Camera>().unwrap();
                ui_hdr = Some(entity.contains::<Hdr>());
                ui_order = Some(cam.order);
                ui_clear_none = matches!(cam.clear_color, ClearColorConfig::None);
                bloom_on_ui = entity.contains::<Bloom>();
            }
        }
        assert_eq!(ui_hdr.expect("ui camera"), world_hdr, "{label} ui hdr");
        assert_eq!(ui_order.unwrap(), UI_CAMERA_ORDER, "{label}");
        assert!(ui_clear_none, "{label} clear");
        let mut saw_camera = false;
        for entity in world_ref.iter_entities() {
            if let Some(msaa) = entity.get::<Msaa>() {
                saw_camera = true;
                assert!(soft_gpu_msaa_is_off(*msaa), "{label}");
            }
        }
        assert!(saw_camera, "{label} msaa");
        assert!(!bloom_on_ui, "{label} bloom on ui");
        let world_bloom = world_ref.get::<Bloom>(world_cam).is_some();
        assert_eq!(world_bloom, expect_hdr, "{label} world bloom");
    }
}
