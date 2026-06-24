/*!
 * Player Persistence Module
 *
 * v19.3.14: Integrated crash recovery lifecycle (mark_session_started / mark_clean_shutdown).
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

pub mod data;
pub mod save;

pub use data::*;
pub use save::*;

use bevy::prelude::*;
use std::path::Path;
use std::time::Duration;

#[derive(Resource)]
pub struct AutoSaveTimer {
    pub timer: Timer,
}

impl Default for AutoSaveTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(60), TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct PersistenceFlushTimer {
    pub timer: Timer,
}

impl Default for PersistenceFlushTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(15), TimerMode::Repeating),
        }
    }
}

pub struct PersistencePlugin;

impl Plugin for PersistencePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerSaveData>()
            .init_resource::<AutoSaveTimer>()
            .init_resource::<PersistenceFlushTimer>()
            .add_event::<PersistenceUpdated>()
            .add_systems(Startup, load_player_save)
            .add_systems(Update, auto_save_system)
            .add_systems(Update, persistence_flush_system)
            .add_systems(Update, save_on_exit)
            .add_systems(Update, update_playtime);
    }
}

fn load_player_save(mut commands: Commands) {
    let save_path = Path::new("player_save.json");

    let mut save_data = if let Some(loaded) = PlayerSaveData::load_from_file(save_path) {
        if !loaded.last_shutdown_was_clean {
            warn!("Previous session did not shut down cleanly. Loading from backup if available.");
        }
        loaded
    } else {
        PlayerSaveData::new(1)
    };

    // Mark that a new session has started (not yet cleanly shut down)
    save_data.mark_session_started();
    commands.insert_resource(save_data);
    info!("Player save loaded");
}

fn auto_save_system(
    mut save_data: ResMut<PlayerSaveData>,
    mut timer: ResMut<AutoSaveTimer>,
    time: Res<Time>,
    mut persistence_events: EventWriter<PersistenceUpdated>,
) {
    timer.timer.tick(time.delta());
    if timer.timer.just_finished() && save_data.dirty {
        if let Err(e) = save_data.save_to_file(Path::new("player_save.json")) {
            error!("Auto-save failed: {}", e);
        } else {
            persistence_events.send(PersistenceUpdated { reason: "auto_save".into() });
        }
    }
}

fn persistence_flush_system(
    mut save_data: ResMut<PlayerSaveData>,
    mut flush_timer: ResMut<PersistenceFlushTimer>,
    time: Res<Time>,
) {
    flush_timer.timer.tick(time.delta());

    if flush_timer.timer.just_finished() && save_data.pending_persistence_updates > 0 {
        save_data.force_dirty();

        if let Err(e) = save_data.save_to_file(Path::new("player_save.json")) {
            error!("Secondary persistence flush failed: {}", e);
        } else {
            save_data.dirty = false;
            save_data.pending_persistence_updates = 0;
        }
    }
}

fn save_on_exit(
    mut save_data: ResMut<PlayerSaveData>,
    mut exit_events: EventReader<bevy::app::AppExit>,
) {
    for _ in exit_events.read() {
        save_data.mark_clean_shutdown();
        if save_data.pending_persistence_updates > 0 {
            save_data.force_dirty();
        }
        let _ = save_data.save_to_file(Path::new("player_save.json"));
    }
}

fn update_playtime(
    mut save_data: ResMut<PlayerSaveData>,
    time: Res<Time>,
) {
    save_data.total_playtime_seconds += time.delta().as_secs();
}

// End of simulation/src/player_persistence/mod.rs v19.3.14
// Crash recovery lifecycle fully integrated.
// Thunder locked in. Yoi ⚡
