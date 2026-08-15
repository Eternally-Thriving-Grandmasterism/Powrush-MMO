/*!
 * Abundance Journey Echo — soft session memory for human play (v21.93.1)
 *
 * Binds Living Practice seals and RBE allocate choices into a visible,
 * non-extractive journey log. Complements My Mercy Journey (F2) without
 * requiring hard LegacyJournal coupling on every host path.
 *
 * Toggle: **F4**
 *
 * TOLC 8 · no scarcity · Contact: info@Rathor.ai · Yoi ⚡
 */

use bevy::prelude::*;

use crate::living_practice_loop::LivingPracticeLoop;
use crate::rbe_allocate_choice::{AllocatePath, RbeAllocateChoice};

#[derive(Debug, Clone)]
pub struct JourneyLine {
    pub text: String,
    pub kind: JourneyKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JourneyKind {
    PracticeSeal,
    FlowOutward,
    StewardReserve,
    Note,
}

#[derive(Resource, Debug, Default)]
pub struct AbundanceJourneyEcho {
    pub lines: Vec<JourneyLine>,
    pub panel_open: bool,
    pub last_practice_sealed: bool,
    pub last_choices_seen: u32,
}

impl AbundanceJourneyEcho {
    pub fn push(&mut self, kind: JourneyKind, text: impl Into<String>) {
        self.lines.push(JourneyLine {
            text: text.into(),
            kind,
        });
        // Keep soft — no grind log explosion
        if self.lines.len() > 24 {
            self.lines.remove(0);
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
            .add_systems(Startup, spawn_echo_panel)
            .add_systems(
                Update,
                (
                    absorb_practice_and_allocate,
                    toggle_echo_panel,
                    update_echo_visibility,
                    update_echo_body,
                ),
            );
    }
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
                    border_radius: BorderRadius::all(Val::Px(12.0)),
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
                "F4 toggle · soft session memory · TOLC 8",
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
    if keyboard.just_pressed(KeyCode::F4) {
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
}
