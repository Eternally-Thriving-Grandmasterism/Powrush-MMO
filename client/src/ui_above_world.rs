//! Soft-GPU UI above the 3D yard (fail beat F2 — lavapipe / Mesa).
//!
//! Bevy 0.14 draws UI on the default UI camera's graph. When a second Camera3d
//! shares order 0 (climate race vs main), or when UI rides the world camera,
//! soft backends can composite the yard *over* Title / pause / Ledger so only
//! top strings read and Play needs Digit1. Fix: one Camera2d with higher order,
//! ClearColorConfig::None, IsDefaultUiCamera — lived plates always draw last.
//! Contact: info@Rathor.ai

use bevy::prelude::*;
use bevy::render::camera::ClearColorConfig;

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

pub struct UiAboveWorldPlugin;

impl Plugin for UiAboveWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_lived_ui_camera)
            .add_systems(Update, stamp_world_camera_order);
    }
}

fn spawn_lived_ui_camera(mut commands: Commands) {
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
}
