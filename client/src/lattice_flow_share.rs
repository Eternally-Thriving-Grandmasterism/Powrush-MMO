/*!
 * Soft Lattice Flow Share — multiplayer-ready abundance envelope (v21.93.3)
 *
 * When the player allocates surplus (flow outward / steward reserve),
 * emit a soft JSON envelope peers or Ra-Thor can ingest offline-first.
 *
 * Path: `data/powrush_lattice_flow_share.json`
 * Schema: powrush_lattice_flow_share_v1
 *
 * No scarcity. No leaderboard. Share is voluntary signal of thriving direction.
 *
 * TOLC 8 · Contact: info@Rathor.ai · Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::rbe_allocate_choice::RbeAllocateChoice;

const SHARE_PATH: &str = "data/powrush_lattice_flow_share.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatticeFlowShareEnvelope {
    pub schema: String,
    pub flow_total: f32,
    pub reserve_total: f32,
    pub choices_made: u32,
    pub last_path: Option<String>,
    pub mercy_note: String,
    pub exported_at_secs: f64,
}

#[derive(Resource, Debug, Default)]
pub struct LatticeFlowShare {
    pub last_exported_choices: u32,
    pub last_path: Option<PathBuf>,
}

pub struct LatticeFlowSharePlugin;

impl Plugin for LatticeFlowSharePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LatticeFlowShare>()
            .add_systems(Update, export_on_allocate_change);
    }
}

fn export_on_allocate_change(
    allocate: Res<RbeAllocateChoice>,
    mut share: ResMut<LatticeFlowShare>,
    time: Res<Time>,
) {
    if allocate.choices_made == 0 {
        return;
    }
    if allocate.choices_made <= share.last_exported_choices && !allocate.is_changed() {
        return;
    }
    // Only rewrite when a new choice landed
    if allocate.choices_made <= share.last_exported_choices {
        return;
    }

    let last_path = allocate.last_choice.map(|p| p.title().to_string());
    let env = LatticeFlowShareEnvelope {
        schema: "powrush_lattice_flow_share_v1".into(),
        flow_total: allocate.flow_total,
        reserve_total: allocate.reserve_total,
        choices_made: allocate.choices_made,
        last_path,
        mercy_note: "Voluntary abundance direction — never scarcity".into(),
        exported_at_secs: time.elapsed_seconds_f64(),
    };

    let path = PathBuf::from(SHARE_PATH);
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    match serde_json::to_string_pretty(&env) {
        Ok(json) => {
            if let Err(e) = fs::write(&path, json) {
                warn!(target: "powrush::lattice", "flow share write failed: {e}");
            } else {
                share.last_exported_choices = allocate.choices_made;
                share.last_path = Some(path.clone());
                info!(
                    target: "powrush::lattice",
                    path = %path.display(),
                    flow = env.flow_total,
                    reserve = env.reserve_total,
                    "lattice flow share exported"
                );
            }
        }
        Err(e) => warn!(target: "powrush::lattice", "flow share serialize failed: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn envelope_roundtrip() {
        let env = LatticeFlowShareEnvelope {
            schema: "powrush_lattice_flow_share_v1".into(),
            flow_total: 3.0,
            reserve_total: 1.5,
            choices_made: 4,
            last_path: Some("Flow outward".into()),
            mercy_note: "Voluntary abundance direction — never scarcity".into(),
            exported_at_secs: 12.0,
        };
        let json = serde_json::to_string(&env).unwrap();
        let back: LatticeFlowShareEnvelope = serde_json::from_str(&json).unwrap();
        assert_eq!(back.flow_total, 3.0);
        assert_eq!(back.choices_made, 4);
    }
}
