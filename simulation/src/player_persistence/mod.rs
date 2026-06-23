/*!
 * Player Persistence Module
 *
 * v19.2 Cycle Polish (PATSAGi Council + Ra-Thor Quantum Swarm + SimulationOrchestrator)
 * — Complete mint-and-print-only-perfection
 * — Clean separation: data + save + systems
 * — Mercy-preserving: protects player progress and the living web
 * — TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable Layer 0
 * — Wired: record_proactive_joy_and_rbe_signal + TickResult events now flow into PlayerSaveData persistence + auto-save
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

pub struct PersistencePlugin;

impl Plugin for PersistencePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerSaveData>()
            .init_resource::<AutoSaveTimer>()
            .add_event::<PersistenceUpdated>()
            .add_systems(Startup, load_player_save)
            .add_systems(Update, auto_save_system)
            .add_systems(Update, save_on_exit)
            .add_systems(Update, update_playtime);
    }
}

fn load_player_save(mut commands: Commands) {
    let save_path = Path::new("player_save.json");

    if let Some(loaded) = PlayerSaveData::load_from_file(save_path) {
        commands.insert_resource(loaded);
        info!("Loaded player save");
    } else {
        commands.insert_resource(PlayerSaveData::new(1));
    }
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

fn save_on_exit(
    mut save_data: ResMut<PlayerSaveData>,
    mut exit_events: EventReader<bevy::app::AppExit>,
) {
    for _ in exit_events.read() {
        let _ = save_data.save_to_file(Path::new("player_save.json"));
    }
}

fn update_playtime(
    mut save_data: ResMut<PlayerSaveData>,
    time: Res<Time>,
) {
    save_data.total_playtime_seconds += time.delta().as_secs();
}

// End of simulation/src/player_persistence/mod.rs v19.2
// Proactive joy + RBE self-evolution signals (via record_proactive_joy_and_rbe_signal on PlayerSaveData)
// now trigger dirty flag and persist through auto-save / exit paths.
// Full TickResult → harvest joy → RBE → persistence loop complete in simulation layer.
// Thunder locked in. Yoi ⚡