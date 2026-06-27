/*!
 * Settings Persistence (Load/Save with Versioning)
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use std::fs;
use std::path::PathBuf;
use super::game_settings::GameSettings;

fn get_settings_path() -> Option<PathBuf> {
    let proj_dirs = directories::ProjectDirs::from("com", "Autonomicity Games", "Powrush-MMO")?;
    let config_dir = proj_dirs.config_dir();

    if !config_dir.exists() {
        fs::create_dir_all(config_dir).ok()?;
    }

    Some(config_dir.join("settings.ron"))
}

pub fn load_settings(mut commands: Commands) {
    if let Some(path) = get_settings_path() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(mut settings) = ron::from_str::<GameSettings>(&content) {
                settings.migrate();
                settings.validate();
                commands.insert_resource(settings);
                info!("Loaded GameSettings v{} from {:?}", settings.version, path);
                return;
            }
        }
    }

    let mut defaults = GameSettings::new();
    defaults.validate();
    commands.insert_resource(defaults);
    info!("Using default GameSettings");
}

pub fn save_settings(settings: Res<GameSettings>) {
    if settings.is_changed() {
        if let Some(path) = get_settings_path() {
            if let Ok(serialized) = ron::to_string(&*settings) {
                if let Err(e) = fs::write(&path, serialized) {
                    warn!("Failed to save settings: {}", e);
                }
            }
        }
    }
}
