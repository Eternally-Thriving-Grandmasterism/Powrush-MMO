/*!
 * Soft Lattice Flow Share — export + peer ingest (v21.96.0)
 *
 * **U** — soft-ingest peer envelope (Unity / Us — ergonomic)
 *
 * TOLC 8 · Contact: info@Rathor.ai · Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::abundance_journey_echo::{AbundanceJourneyEcho, JourneyKind};
use crate::rbe_allocate_choice::RbeAllocateChoice;
use crate::soft_play_bindings;

const SHARE_PATH: &str = "data/powrush_lattice_flow_share.json";
const PEER_PATH: &str = "data/powrush_lattice_flow_share_peer.json";

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
    pub last_peer: Option<LatticeFlowShareEnvelope>,
    pub last_ingest_note: Option<String>,
}

pub struct LatticeFlowSharePlugin;

impl Plugin for LatticeFlowSharePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LatticeFlowShare>()
            .add_systems(Update, (export_on_allocate_change, soft_peer_ingest));
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

fn try_read_envelope(path: &str) -> Option<LatticeFlowShareEnvelope> {
    let bytes = fs::read(path).ok()?;
    let env: LatticeFlowShareEnvelope = serde_json::from_slice(&bytes).ok()?;
    if env.schema.starts_with("powrush_lattice_flow_share") {
        Some(env)
    } else {
        None
    }
}

fn soft_peer_ingest(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut share: ResMut<LatticeFlowShare>,
    mut echo: ResMut<AbundanceJourneyEcho>,
) {
    if !keyboard.just_pressed(soft_play_bindings::PEER_INGEST) {
        return;
    }

    let env = try_read_envelope(PEER_PATH).or_else(|| try_read_envelope(SHARE_PATH));

    match env {
        Some(env) => {
            let note = format!(
                "Peer lattice signal · flow {:.1} · reserve {:.1} · choices {} · {}",
                env.flow_total,
                env.reserve_total,
                env.choices_made,
                env.last_path.as_deref().unwrap_or("—")
            );
            echo.push(JourneyKind::Note, note.clone());
            share.last_ingest_note = Some(note.clone());
            share.last_peer = Some(env);
            info!(target: "powrush::lattice", "{note}");
        }
        None => {
            let note = "No peer lattice envelope yet — drop one at data/powrush_lattice_flow_share_peer.json"
                .to_string();
            share.last_ingest_note = Some(note.clone());
            info!(target: "powrush::lattice", "{note}");
        }
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
    }
}
