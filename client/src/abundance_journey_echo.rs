/*!
 * Abundance Journey Echo — soft session + durable memory (v21.96.0)
 *
 * Binds Living Practice seals and RBE allocate choices into a visible,
 * non-extractive journey log. Complements My Mercy Journey (M).
 *
 * Persistence: local `data/powrush_abundance_journey.json` (sovereign offline).
 * Loads on startup; saves when lines or allocate totals change.
 *
 * Toggle: **J** (Journey — ergonomic left-hand)
 *
 * TOLC 8 · no scarcity · Contact: info@Rathor.ai · Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::living_practice_loop::LivingPracticeLoop;
use crate::rbe_allocate_choice::{AllocatePath, RbeAllocateChoice};
use crate::soft_play_bindings;

const PERSIST_PATH: &str = "data/powrush_abundance_journey.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JourneyLine {
    pub text: String,
    pub kind: JourneyKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JourneyKind {
    PracticeSeal,
    FlowOutward,
    StewardReserve,
    Note,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct JourneyPersistBlob {
    schema: String,
    lines: Vec<JourneyLine>,
    practice_sealed: bool,
    flow_total: f32,
    reserve_total: f32,
    choices_made: u32,
}

#[derive(Resource, Debug, Default)]
pub struct AbundanceJourneyEcho {
    pub lines: Vec<JourneyLine>,
    pub panel_open: bool,
    pub last_practice_sealed: bool,
    pub last_choices_seen: u32,
    /// Dirty flag for soft disk write.
    pub dirty: bool,
    pub loaded: bool,
}

impl AbundanceJourneyEcho {
    pub fn push(&mut self, kind: JourneyKind, text: impl Into<String>) {
        self.lines.push(JourneyLine {
            text: text.into(),
            kind,
        });
        if self.lines.len() > 24 {
            self.lines.remove(0);
        }
        self.dirty = true;
    }
}

fn persist_path() -> PathBuf {
    PathBuf::from(PERSIST_PATH)
}

fn load_blob() -> Option<JourneyPersistBlob> {
    let path = persist_path();
    let bytes = fs::read(&path).ok()?;
    serde_json::from_slice(&bytes).ok()
}

fn save_blob(blob: &JourneyPersistBlob) {
    let path = persist_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(blob) {
        if let Err(e) = fs::write(&path, json) {
            warn!(target: "powrush::journey", "journey persist write failed: {e}");
        } else {
            info!(target: "powrush::journey", path = %path.display(), "journey echo saved");
        }
    }
}

#[derive(Component)]
pub struct JourneyEchoRoot;

#[derive(Component)]
pub struct JourneyEchoBody;

pub struct AbundanceJourneyEchoPlugin;

impl Plugin for AbundanceJourneyEchoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AbundanceJourneyEcho>()
            .add_systems(Startup, (load_journey_persist, spawn_echo_panel).chain())
            .add_systems(
                Update,
                (
                    absorb_practice_and_allocate,
                    toggle_echo_panel,
                    update_echo_visibility,
                    update_echo_body,
                    save_journey_persist,
                ),
            );
    }
}

fn load_journey_persist(
    mut echo: ResMut<AbundanceJourneyEcho>,
    mut allocate: ResMut<RbeAllocateChoice>,
) {
    if echo.loaded {
        return;
    }
    echo.loaded = true;
    if let Some(blob) = load_blob() {
        if blob.schema.starts_with("powrush_abundance_journey") {
            echo.lines = blob.lines;
            echo.last_practice_sealed = blob.practice_sealed;
            echo.last_choices_seen = blob.choices_made;
            allocate.flow_total = blob.flow_total;
            allocate.reserve_total = blob.reserve_total;
            allocate.choices_made = blob.choices_made;
            if blob.practice_sealed {
                info!(target: "powrush::journey", "restored practice seal from disk");
            }
            info!(
                target: "powrush::journey",
                lines = echo.lines.len(),
                flow = allocate.flow_total,
                reserve = allocate.reserve_total,
                "journey echo loaded"
            );
        }
    }
}

fn save_journey_persist(
    mut echo: ResMut<AbundanceJourneyEcho>,
    allocate: Res<RbeAllocateChoice>,
) {
    if allocate.is_changed() {
        echo.dirty = true;
    }
    if !echo.dirty {
        return;
    }
    let blob = JourneyPersistBlob {
        schema: "powrush_abundance_journey_v1".into(),
        lines: echo.lines.clone(),
        practice_sealed: echo.last_practice_sealed,
        flow_total: allocate.flow_total,
        reserve_total: allocate.reserve_total,
        choices_made: allocate.choices_made,
    };
    save_blob(&blob);
    echo.dirty = false;
}

fn spawn_echo_panel(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(12.0),
                    left: Val::Percent(2.0),
                    width: Val::Px(360.0),
                    max_height: Val::Px(280.0),
                    padding: UiRect::all(Val::Px(14.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    border: UiRect::all(Val::Px(1.5)),
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                background_color: Color::srgba(0.05, 0.07, 0.11, 0.94).into(),
                border_color: Color::srgba(0.70, 0.85, 0.55, 0.50).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            JourneyEchoRoot,
        ))
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                "ABUNDANCE JOURNEY",
                TextStyle {
                    font_size: 15.0,
                    color: Color::srgb(0.85, 0.95, 0.70),
                    ..default()
                },
            ));
            p.spawn((
                TextBundle::from_section(
                    "• Acts of thriving will echo here",
                    TextStyle {
                        font_size: 12.5,
                        color: Color::srgb(0.88, 0.92, 0.98),
                        ..default()
                    },
                ),
                JourneyEchoBody,
            ));
            p.spawn(TextBundle::from_section(
                "J toggle · soft durable memory · TOLC 8",
                TextStyle {
                    font_size: 11.0,
                    color: Color::srgb(0.55, 0.68, 0.75),
                    ..default()
                },
            ));
        });
}

fn absorb_practice_and_allocate(
    practice: Res<LivingPracticeLoop>,
    allocate: Res<RbeAllocateChoice>,
    mut echo: ResMut<AbundanceJourneyEcho>,
) {
    if practice.principle_sealed && !echo.last_practice_sealed {
        echo.last_practice_sealed = true;
        echo.push(
            JourneyKind::PracticeSeal,
            "Sealed Caps Across Climates — same principle, three climates",
        );
    }

    if allocate.choices_made > echo.last_choices_seen {
        echo.last_choices_seen = allocate.choices_made;
        match allocate.last_choice {
            Some(AllocatePath::FlowOutward) => echo.push(
                JourneyKind::FlowOutward,
                format!(
                    "Flowed outward · lattice share (total {:.1})",
                    allocate.flow_total
                ),
            ),
            Some(AllocatePath::StewardReserve) => echo.push(
                JourneyKind::StewardReserve,
                format!(
                    "Stewarded reserve · future thriving (total {:.1})",
                    allocate.reserve_total
                ),
            ),
            None => {}
        }
    }
}

fn toggle_echo_panel(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut echo: ResMut<AbundanceJourneyEcho>,
) {
    if keyboard.just_pressed(soft_play_bindings::JOURNEY_ECHO) {
        echo.panel_open = !echo.panel_open;
    }
}

fn update_echo_visibility(
    echo: Res<AbundanceJourneyEcho>,
    mut q: Query<&mut Visibility, With<JourneyEchoRoot>>,
) {
    for mut vis in &mut q {
        *vis = if echo.panel_open {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn update_echo_body(
    echo: Res<AbundanceJourneyEcho>,
    mut q: Query<&mut Text, With<JourneyEchoBody>>,
) {
    if !echo.is_changed() {
        return;
    }
    let body = if echo.lines.is_empty() {
        "• Acts of thriving will echo here".to_string()
    } else {
        echo.lines
            .iter()
            .rev()
            .take(10)
            .map(|l| {
                let mark = match l.kind {
                    JourneyKind::PracticeSeal => "◎",
                    JourneyKind::FlowOutward => "→",
                    JourneyKind::StewardReserve => "◇",
                    JourneyKind::Note => "•",
                };
                format!("{} {}", mark, l.text)
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    for mut text in &mut q {
        if let Some(s) = text.sections.get_mut(0) {
            s.value = body.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_caps_at_24() {
        let mut e = AbundanceJourneyEcho::default();
        for i in 0..30 {
            e.push(JourneyKind::Note, format!("n{i}"));
        }
        assert!(e.lines.len() <= 24);
    }

    #[test]
    fn blob_roundtrip_shape() {
        let blob = JourneyPersistBlob {
            schema: "powrush_abundance_journey_v1".into(),
            lines: vec![JourneyLine {
                text: "test".into(),
                kind: JourneyKind::Note,
            }],
            practice_sealed: true,
            flow_total: 2.0,
            reserve_total: 1.0,
            choices_made: 3,
        };
        let json = serde_json::to_string(&blob).unwrap();
        let back: JourneyPersistBlob = serde_json::from_str(&json).unwrap();
        assert_eq!(back.choices_made, 3);
        assert!(back.practice_sealed);
    }
}
