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
//! missing. No new Camera3d. Contact: info@Rathor.ai

use bevy::prelude::*;
use bevy::render::camera::ClearColorConfig;
use bevy::render::view::Msaa;

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
        app.insert_resource(Msaa::Off)
            .add_systems(Startup, spawn_lived_ui_camera)
            .add_systems(
                Update,
                (
                    ensure_lived_ui_camera,
                    stamp_world_camera_order,
                    stamp_lived_ui_camera,
                    strip_world_default_ui_camera,
                    bind_lived_ui_plates,
                )
                    .chain(),
            );
    }
}

fn spawn_lived_ui_camera(mut commands: Commands) {
    spawn_ui_camera_entity(&mut commands);
}

fn spawn_ui_camera_entity(commands: &mut Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: UI_CAMERA_ORDER,
                // Keep the 3D yard; only composite Bevy UI on top.
                clear_color: ClearColorConfig::None,
                ..default()
            },
            ..default()
        },
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

/// Parent lived plates to the UI camera so Settled HUD chips cannot retarget them.
fn bind_lived_ui_plates(
    ui_cam: Query<Entity, With<LivedUiCamera>>,
    roots: Query<Entity, (With<LivedUiPlate>, Without<TargetCamera>)>,
    mut commands: Commands,
) {
    let Ok(cam) = ui_cam.get_single() else {
        return;
    };
    for entity in &roots {
        commands.entity(entity).insert(TargetCamera(cam));
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
            .insert_resource(Msaa::Sample4)
            .add_plugins(UiAboveWorldPlugin);
        // Plugin build inserts Msaa::Off immediately.
        assert!(soft_gpu_msaa_is_off(*app.world().resource::<Msaa>()));
        assert!(ui_camera_draws_above_world());
    }
}
