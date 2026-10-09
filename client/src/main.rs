//! Lived first hour — the player door.
//! WASD walk · Space jump · Shift sprint · E harvest/tend · I satchel · H hide guidance · R allocate.
//! Contact: info@Rathor.ai

use std::fs::OpenOptions;
use std::io::Write;
use std::panic::PanicHookInfo;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use powrush_client::PowrushClientBundle;
use shared::peace_audio::audio_output_safe;

/// Crash log file name. Not a `powrush_*` live persist name, so count,
/// adopt, and the title screen never see it.
const CRASH_LOG_FILE: &str = "crash.log";

/// Argument to `persist_path`. The file name stays [`CRASH_LOG_FILE`].
const CRASH_LOG_PERSIST_NAME: &str = "data/crash.log";

/// Console log of FPS and frame time. Bevy defaults stay, including the log interval.
fn frame_time_log_plugin() -> LogDiagnosticsPlugin {
    LogDiagnosticsPlugin {
        filter: Some(vec![
            FrameTimeDiagnosticsPlugin::FPS,
            FrameTimeDiagnosticsPlugin::FRAME_TIME,
        ]),
        ..default()
    }
}

fn main() {
    // Resolve once, before `App::new()`. `persist_path("data/crash.log")`
    // creates the directory and may adopt cwd `data/`. Do not call it
    // from the hook. `CRASH_LOG_PERSIST_NAME` is that name.
    let crash_log = shared::user_persist::persist_path(CRASH_LOG_PERSIST_NAME);
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        append_crash_log(&crash_log, info);
        previous(info);
    }));
    // Lavapipe / Deck boxes often have no ALSA card. DefaultPlugins'
    // AudioPlugin talks to rodio/cpal — skip it when the fast probe
    // says there is no output so boot cannot hang. Mute is a later gate.
    let window = WindowPlugin {
        primary_window: Some(Window {
            title: "Powrush-MMO — first hour".into(),
            // U5: make the default window match the Steam Deck title proof.
            resolution: (
                powrush_client::title_screen::DECK_TITLE_WIDTH,
                powrush_client::title_screen::DECK_TITLE_HEIGHT,
            )
                .into(),
            ..default()
        }),
        ..default()
    };
    let default_plugins = if audio_output_safe() {
        DefaultPlugins.set(window)
    } else {
        DefaultPlugins
            .set(window)
            .disable::<bevy::audio::AudioPlugin>()
    };
    App::new()
        .add_plugins(default_plugins)
        .add_plugins(PowrushClientBundle)
        .add_plugins((FrameTimeDiagnosticsPlugin, frame_time_log_plugin()))
        .add_systems(Startup, spawn_sun_and_camera)
        .run();
}

fn spawn_sun_and_camera(mut commands: Commands) {
    use powrush_client::climate_plane::{
        spawn_sanctuary_sun_once, world_color_grading, WORLD_TONEMAPPING,
    };
    // World camera order 0 — lived UI Camera2d (order 10) draws above on soft GPU.
    // CARD VP-GRADE-1 — AgX + the one world grade; the UI camera stays bare.
    commands.spawn(Camera3dBundle {
        camera: Camera {
            order: powrush_client::ui_above_world::WORLD_CAMERA_ORDER,
            ..default()
        },
        tonemapping: WORLD_TONEMAPPING,
        color_grading: world_color_grading(),
        transform: Transform::from_xyz(0.0, 8.0, 14.0).looking_at(Vec3::ZERO, Vec3::Y),
        msaa: bevy::render::view::Msaa::Off,
        ..default()
    });
    // CARD VP-GRADE-1 — the one Sanctuary sun (climate_plane::SANCTUARY_SUN).
    spawn_sanctuary_sun_once(&mut commands);
}

/// One crash line: `{epoch} {message} {file}:{line}\n`, or without
/// `file:line` when `location` is `None`. `\r` and `\n` in the message
/// become spaces, so the result is a single line.
fn crash_log_line(epoch_secs: u64, message: &str, location: Option<(&str, u32)>) -> String {
    let message = message.replace(['\r', '\n'], " ");
    match location {
        Some((file, line)) => format!("{epoch_secs} {message} {file}:{line}\n"),
        None => format!("{epoch_secs} {message}\n"),
    }
}

fn panic_payload_message(info: &PanicHookInfo<'_>) -> String {
    let payload = info.payload();
    if let Some(message) = payload.downcast_ref::<&str>() {
        return (*message).to_string();
    }
    if let Some(message) = payload.downcast_ref::<String>() {
        return message.clone();
    }
    "<non-string panic payload>".to_string()
}

/// Best-effort append. Ignores io errors. Does not truncate, delete, or
/// rename. A `powrush_*` file name is a live save, so that rename writes
/// nothing. The previous hook is called by the caller after this returns.
fn append_crash_log(path: &Path, info: &PanicHookInfo<'_>) {
    if CRASH_LOG_FILE.starts_with("powrush_") || !CRASH_LOG_PERSIST_NAME.ends_with(CRASH_LOG_FILE)
    {
        return;
    }
    let epoch_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let message = panic_payload_message(info);
    let location = info.location().map(|loc| (loc.file(), loc.line()));
    let line = crash_log_line(epoch_secs, &message, location);
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    let _ = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .and_then(|mut file| file.write_all(line.as_bytes()));
}

#[cfg(test)]
mod crash_log_hook_tests {
    use super::{crash_log_line, CRASH_LOG_FILE, CRASH_LOG_PERSIST_NAME};

    #[test]
    fn line_starts_with_epoch_and_holds_message_location_and_one_newline() {
        let epoch = 1_700_000_123_u64;
        let line = crash_log_line(epoch, "satchel broke", Some(("client/src/main.rs", 40)));
        assert!(line.starts_with(&epoch.to_string()));
        assert!(line.contains("satchel broke"));
        assert!(line.contains("client/src/main.rs:40"));
        assert!(line.ends_with('\n'));
        assert_eq!(line.matches('\n').count(), 1);
        assert!(!line.contains('\r'));
    }

    #[test]
    fn embedded_newlines_stay_one_line() {
        let line = crash_log_line(42, "one\ntwo\rthree", Some(("src/main.rs", 9)));
        assert_eq!(line, "42 one two three src/main.rs:9\n");
        assert_eq!(line.matches('\n').count(), 1);
        assert!(line.ends_with('\n'));
        assert!(!line.contains('\r'));
    }

    #[test]
    fn no_location_omits_file_and_line() {
        let line = crash_log_line(7, "plain", None);
        assert_eq!(line, "7 plain\n");
        assert_eq!(line.matches('\n').count(), 1);
        assert!(!line.contains(':'));
    }

    #[test]
    fn crash_log_name_is_not_a_live_persist_name() {
        assert!(!CRASH_LOG_FILE.starts_with("powrush_"));
        assert_eq!(CRASH_LOG_FILE, "crash.log");
        assert_eq!(CRASH_LOG_PERSIST_NAME, "data/crash.log");
        assert!(CRASH_LOG_PERSIST_NAME.ends_with(CRASH_LOG_FILE));
    }
}

#[cfg(test)]
mod frame_time_log_tests {
    use super::frame_time_log_plugin;
    use bevy::diagnostic::{
        DiagnosticsPlugin, DiagnosticsStore, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
    };
    use bevy::prelude::*;

    #[test]
    fn frame_time_log_filters_fps_and_frame_time_only() {
        let plugin = frame_time_log_plugin();
        let filter = plugin.filter.expect("frame time log filter");
        assert_eq!(
            filter,
            vec![
                FrameTimeDiagnosticsPlugin::FPS,
                FrameTimeDiagnosticsPlugin::FRAME_TIME,
            ]
        );
        assert!(!filter.contains(&FrameTimeDiagnosticsPlugin::FRAME_COUNT));
    }

    #[test]
    fn frame_time_log_keeps_bevy_default_interval() {
        let plugin = frame_time_log_plugin();
        assert_eq!(
            plugin.wait_duration,
            LogDiagnosticsPlugin::default().wait_duration
        );
        assert!(!plugin.debug);
    }

    #[test]
    fn fps_and_frame_time_present_in_diagnostics_store_after_updates() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        // Bevy 0.14 MinimalPlugins does not add DiagnosticsPlugin. A second add panics.
        if !app.is_plugin_added::<DiagnosticsPlugin>() {
            app.add_plugins(DiagnosticsPlugin);
        }
        app.add_plugins((FrameTimeDiagnosticsPlugin, frame_time_log_plugin()));
        for _ in 0..3 {
            app.update();
        }
        let store = app.world().resource::<DiagnosticsStore>();
        assert!(store.get(&FrameTimeDiagnosticsPlugin::FPS).is_some());
        assert!(store.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME).is_some());
    }
}
