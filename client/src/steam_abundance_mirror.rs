/*!
 * Soft Steam Abundance Mirror — Auto-Cloud stage (v21.94.1)
 *
 * Feature-agnostic: always stages journey + lattice share under
 * `data/steam_stage/` so Steam Auto-Cloud (or a later SDK write) can pick them up.
 *
 * When built with `steam` feature, host may also push these remote names via
 * existing SteamCloudBackend (see client/steamworks_remote_storage.rs).
 *
 * Remote names (SDK):
 *   - powrush_abundance_journey.json
 *   - powrush_lattice_flow_share.json
 *
 * TOLC 8 · no scarcity · Contact: info@Rathor.ai · Yoi ⚡
 */

use bevy::prelude::*;
use std::fs;
use std::path::PathBuf;

use crate::lattice_flow_share::LatticeFlowShare;
use crate::rbe_allocate_choice::RbeAllocateChoice;
use crate::abundance_journey_echo::AbundanceJourneyEcho;

pub const STEAM_STAGE_DIR: &str = "data/steam_stage";
pub const REMOTE_JOURNEY: &str = "powrush_abundance_journey.json";
pub const REMOTE_LATTICE: &str = "powrush_lattice_flow_share.json";

const LOCAL_JOURNEY: &str = "data/powrush_abundance_journey.json";
const LOCAL_LATTICE: &str = "data/powrush_lattice_flow_share.json";

#[derive(Resource, Debug, Default)]
pub struct SteamAbundanceMirror {
    pub last_staged_choices: u32,
    pub last_stage_ok: bool,
    pub last_note: Option<String>,
}

pub struct SteamAbundanceMirrorPlugin;

impl Plugin for SteamAbundanceMirrorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamAbundanceMirror>()
            .add_systems(Update, stage_abundance_blobs);
    }
}

fn stage_one(local: &str, remote_name: &str) -> Result<(), String> {
    let bytes = fs::read(local).map_err(|e| format!("{local}: {e}"))?;
    let dir = PathBuf::from(STEAM_STAGE_DIR);
    let _ = fs::create_dir_all(&dir);
    let dest = dir.join(remote_name);
    fs::write(&dest, &bytes).map_err(|e| format!("{}: {e}", dest.display()))?;
    Ok(())
}

fn stage_abundance_blobs(
    allocate: Res<RbeAllocateChoice>,
    echo: Res<AbundanceJourneyEcho>,
    lattice: Res<LatticeFlowShare>,
    mut mirror: ResMut<SteamAbundanceMirror>,
) {
    // Stage when allocate advanced or journey dirtied (echo dirty already flushed to disk by then)
    let should = allocate.choices_made > mirror.last_staged_choices
        || (echo.is_changed() && allocate.choices_made > 0)
        || (lattice.is_changed() && lattice.last_exported_choices > mirror.last_staged_choices);

    if !should {
        return;
    }

    let mut ok = true;
    let mut notes = Vec::new();

    match stage_one(LOCAL_JOURNEY, REMOTE_JOURNEY) {
        Ok(()) => notes.push("journey staged"),
        Err(e) => {
            // Journey may not exist yet — soft
            notes.push("journey skip");
            info!(target: "powrush::steam_stage", "{e}");
        }
    }

    match stage_one(LOCAL_LATTICE, REMOTE_LATTICE) {
        Ok(()) => notes.push("lattice staged"),
        Err(e) => {
            notes.push("lattice skip");
            info!(target: "powrush::steam_stage", "{e}");
            ok = false;
        }
    }

    if allocate.choices_made > 0 {
        mirror.last_staged_choices = allocate.choices_made;
    }
    mirror.last_stage_ok = ok || notes.iter().any(|n| n.contains("staged"));
    mirror.last_note = Some(notes.join(", "));

    if mirror.last_stage_ok {
        info!(
            target: "powrush::steam_stage",
            dir = STEAM_STAGE_DIR,
            note = ?mirror.last_note,
            "abundance blobs staged for Auto-Cloud / RemoteStorage"
        );
    }
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
