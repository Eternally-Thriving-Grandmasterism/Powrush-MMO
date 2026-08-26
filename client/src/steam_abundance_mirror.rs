/*!
 * Steam Auto-Cloud triggers — abundance journey + lattice share (v21.96.0)
 *
 * Force flush: **Shift+T** (rare power action — ergonomic chord)
 *
 * TOLC 8 · Contact: info@Rathor.ai · Yoi ⚡
 */

use bevy::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

use crate::abundance_journey_echo::AbundanceJourneyEcho;
use crate::lattice_flow_share::LatticeFlowShare;
use crate::rbe_allocate_choice::RbeAllocateChoice;
use crate::soft_play_bindings;

pub const ABUNDANCE_SUBDIR: &str = "steam_cloud/abundance";
pub const REMOTE_JOURNEY: &str = "powrush_abundance_journey.json";
pub const REMOTE_LATTICE: &str = "powrush_lattice_flow_share.json";

const LOCAL_JOURNEY: &str = "data/powrush_abundance_journey.json";
const LOCAL_LATTICE: &str = "data/powrush_lattice_flow_share.json";

pub fn preferred_abundance_stage_root() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            return Path::new(&local)
                .join("Powrush-MMO")
                .join("steam_cloud")
                .join("abundance");
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("Powrush-MMO")
                .join("steam_cloud")
                .join("abundance");
        }
    }
    #[cfg(target_os = "linux")]
    {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home)
                .join(".local")
                .join("share")
                .join("Powrush-MMO")
                .join("steam_cloud")
                .join("abundance");
        }
    }
    PathBuf::from(ABUNDANCE_SUBDIR)
}

#[derive(Resource, Debug)]
pub struct SteamAbundanceMirror {
    pub enabled: bool,
    pub stage_root: PathBuf,
    pub last_staged_choices: u32,
    pub last_stage_ok: bool,
    pub last_note: Option<String>,
    pub force_pending: bool,
    pub exports: u32,
}

impl Default for SteamAbundanceMirror {
    fn default() -> Self {
        Self {
            enabled: true,
            stage_root: preferred_abundance_stage_root(),
            last_staged_choices: 0,
            last_stage_ok: false,
            last_note: None,
            force_pending: false,
            exports: 0,
        }
    }
}

pub struct SteamAbundanceMirrorPlugin;

impl Plugin for SteamAbundanceMirrorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamAbundanceMirror>()
            .add_systems(Startup, ensure_abundance_stage_dirs)
            .add_systems(
                Update,
                (
                    force_flush_key,
                    auto_cloud_trigger_on_progress,
                ),
            )
            .add_systems(Last, flush_on_exit_hint);
    }
}

fn ensure_abundance_stage_dirs(mirror: Res<SteamAbundanceMirror>) {
    let _ = fs::create_dir_all(&mirror.stage_root);
    let _ = fs::create_dir_all(ABUNDANCE_SUBDIR);
    info!(
        target: "powrush::steam_autocloud",
        stage = %mirror.stage_root.display(),
        portable = ABUNDANCE_SUBDIR,
        "Abundance Auto-Cloud stage directories ready"
    );
}

fn atomic_stage_copy(src: &str, dest: &Path) -> Result<(), String> {
    let bytes = fs::read(src).map_err(|e| format!("{src}: {e}"))?;
    if let Some(parent) = dest.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let tmp = dest.with_extension("json.tmp");
    fs::write(&tmp, &bytes).map_err(|e| format!("tmp {}: {e}", tmp.display()))?;
    fs::rename(&tmp, dest).map_err(|e| format!("rename {}: {e}", dest.display()))?;
    Ok(())
}

fn stage_all(mirror: &mut SteamAbundanceMirror) {
    if !mirror.enabled {
        return;
    }

    let os_journey = mirror.stage_root.join(REMOTE_JOURNEY);
    let os_lattice = mirror.stage_root.join(REMOTE_LATTICE);
    let portable_journey = PathBuf::from(ABUNDANCE_SUBDIR).join(REMOTE_JOURNEY);
    let portable_lattice = PathBuf::from(ABUNDANCE_SUBDIR).join(REMOTE_LATTICE);

    let mut notes = Vec::new();
    let mut any_ok = false;

    for (src, dest, label) in [
        (LOCAL_JOURNEY, &os_journey, "journey/os"),
        (LOCAL_JOURNEY, &portable_journey, "journey/portable"),
        (LOCAL_LATTICE, &os_lattice, "lattice/os"),
        (LOCAL_LATTICE, &portable_lattice, "lattice/portable"),
    ] {
        match atomic_stage_copy(src, dest) {
            Ok(()) => {
                notes.push(format!("{label}✓"));
                any_ok = true;
            }
            Err(e) => {
                notes.push(format!("{label}·"));
                info!(target: "powrush::steam_autocloud", "{e}");
            }
        }
    }

    mirror.last_stage_ok = any_ok;
    mirror.last_note = Some(notes.join(" "));
    if any_ok {
        mirror.exports = mirror.exports.saturating_add(1);
        info!(
            target: "powrush::steam_autocloud",
            note = ?mirror.last_note,
            exports = mirror.exports,
            stage = %mirror.stage_root.display(),
            "Auto-Cloud abundance trigger fired"
        );
    }
}

fn auto_cloud_trigger_on_progress(
    allocate: Res<RbeAllocateChoice>,
    echo: Res<AbundanceJourneyEcho>,
    lattice: Res<LatticeFlowShare>,
    mut mirror: ResMut<SteamAbundanceMirror>,
) {
    if !mirror.enabled {
        return;
    }

    let force = mirror.force_pending;
    if force {
        mirror.force_pending = false;
    }

    let progressed = allocate.choices_made > mirror.last_staged_choices
        || (lattice.last_exported_choices > mirror.last_staged_choices)
        || (echo.is_changed() && !echo.lines.is_empty());

    if !force && !progressed {
        return;
    }

    stage_all(&mut mirror);
    if allocate.choices_made > 0 {
        mirror.last_staged_choices = allocate.choices_made.max(lattice.last_exported_choices);
    }
}

fn force_flush_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mirror: ResMut<SteamAbundanceMirror>,
) {
    let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    if shift && keyboard.just_pressed(soft_play_bindings::FORCE_CLOUD_FLUSH) {
        mirror.force_pending = true;
        info!(target: "powrush::steam_autocloud", "Shift+T — force Auto-Cloud abundance flush requested");
    }
}

fn flush_on_exit_hint(
    mut exit: EventReader<AppExit>,
    mut mirror: ResMut<SteamAbundanceMirror>,
) {
    if exit.is_empty() {
        return;
    }
    for _ in exit.read() {}
    stage_all(&mut mirror);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remote_names_stable() {
        assert_eq!(REMOTE_JOURNEY, "powrush_abundance_journey.json");
        assert_eq!(REMOTE_LATTICE, "powrush_lattice_flow_share.json");
    }
}
