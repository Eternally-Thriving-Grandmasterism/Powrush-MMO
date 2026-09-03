/*!
 * Soft Lattice Flow Share — local export + future peer socket (v21.99.4)
 *
 * First hour is solo. Own export is not dressed up as a peer.
 * Chip appears only if a *peer* file exists. U without a peer is honest, not an ops ticket.
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
const AMBIENT_POLL: f32 = 4.0;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub ambient_seen_choices: Option<u32>,
    pub poll_accum: f32,
    pub chip_visible: bool,
}

#[derive(Component)]
struct PeerPresenceRoot;
#[derive(Component)]
struct PeerPresenceText;

pub struct LatticeFlowSharePlugin;

impl Plugin for LatticeFlowSharePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LatticeFlowShare>()
            .add_systems(Startup, spawn_peer_presence_chip)
            .add_systems(
                Update,
                (
                    export_on_allocate_change,
                    ambient_peer_poll,
                    soft_peer_ingest,
                    update_peer_presence_chip,
                ),
            );
    }
}

fn spawn_peer_presence_chip(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(16.0),
                    right: Val::Px(16.0),
                    width: Val::Px(280.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.04, 0.06, 0.09, 0.82).into(),
                border_color: Color::srgba(0.55, 0.72, 0.90, 0.35).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            PeerPresenceRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 12.5,
                        color: Color::srgb(0.82, 0.90, 1.0),
                        ..default()
                    },
                ),
                PeerPresenceText,
            ));
        });
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

fn ambient_peer_poll(mut share: ResMut<LatticeFlowShare>, time: Res<Time>) {
    share.poll_accum += time.delta_seconds();
    if share.poll_accum < AMBIENT_POLL {
        return;
    }
    share.poll_accum = 0.0;
    // Solo first hour: only a *peer* file is presence. Own export stays local.
    match try_read_envelope(PEER_PATH) {
        Some(env) => {
            share.ambient_seen_choices = Some(env.choices_made);
            share.last_peer = Some(env);
            share.chip_visible = true;
        }
        None => {
            share.chip_visible = false;
        }
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

    match try_read_envelope(PEER_PATH) {
        Some(env) => {
            let note = format!(
                "A fellow traveler left flow {:.1} · reserve {:.1}",
                env.flow_total, env.reserve_total
            );
            echo.push(JourneyKind::Note, note.clone());
            share.last_ingest_note = Some(note);
            share.last_peer = Some(env);
            share.chip_visible = true;
        }
        None => {
            let note =
                "This hour is yours alone — other travelers will appear here later".to_string();
            echo.push(JourneyKind::Note, note.clone());
            share.last_ingest_note = Some(note);
        }
    }
}

fn update_peer_presence_chip(
    share: Res<LatticeFlowShare>,
    mut root: Query<&mut Visibility, With<PeerPresenceRoot>>,
    mut text_q: Query<&mut Text, With<PeerPresenceText>>,
) {
    let show = share.chip_visible && share.last_peer.is_some();
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if let Some(env) = &share.last_peer {
        let line = format!(
            "A traveler shares flow {:.0} · U to remember",
            env.flow_total
        );
        for mut text in &mut text_q {
            if let Some(s) = text.sections.get_mut(0) {
                if s.value != line {
                    s.value = line.clone();
                }
            }
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
