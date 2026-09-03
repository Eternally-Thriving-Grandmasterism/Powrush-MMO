/*!
 * Soft RBE Allocate Choice — human playability depth (v21.93.0)
 *
 * After meaningful surplus, invite a voluntary allocation:
 *   • Flow outward — share into the living lattice
 *   • Steward reserve — hold for future thriving
 *
 * No scarcity language. No punishment for either path.
 * Both are abundance-aligned; the difference is timing and direction.
 *
 * Controls: **R** toggles panel when eligible · buttons choose · Esc / R closes
 *
 * PATSAGi + TOLC 8 | AG-SML v1.0 | Contact: info@Rathor.ai
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocatePath {
    FlowOutward,
    StewardReserve,
}

impl AllocatePath {
    pub fn title(self) -> &'static str {
        match self {
            AllocatePath::FlowOutward => "Flow outward",
            AllocatePath::StewardReserve => "Steward reserve",
        }
    }

    pub fn line(self) -> &'static str {
        match self {
            AllocatePath::FlowOutward => {
                "Share surplus into the lattice — others may thrive now"
            }
            AllocatePath::StewardReserve => {
                "Hold surplus with care — future thriving stays possible"
            }
        }
    }
}

#[derive(Resource, Debug)]
pub struct RbeAllocateChoice {
    pub panel_open: bool,
    pub eligible: bool,
    pub surplus_signal: f32,
    pub last_choice: Option<AllocatePath>,
    pub choices_made: u32,
    pub flow_total: f32,
    pub reserve_total: f32,
    pub auto_offered: bool,
    /// Soft threshold to become eligible after harvest feedback / practice.
    pub eligibility_threshold: f32,
}

impl Default for RbeAllocateChoice {
    fn default() -> Self {
        Self {
            panel_open: false,
            eligible: false,
            surplus_signal: 0.0,
            last_choice: None,
            choices_made: 0,
            flow_total: 0.0,
            reserve_total: 0.0,
            auto_offered: false,
            eligibility_threshold: 1.0,
        }
    }
}

impl RbeAllocateChoice {
    pub fn note_surplus(&mut self, amount: f32) {
        if amount <= 0.0 {
            return;
        }
        self.surplus_signal = (self.surplus_signal + amount).min(32.0);
        if self.surplus_signal >= self.eligibility_threshold {
            self.eligible = true;
        }
    }

    pub fn apply(&mut self, path: AllocatePath, portion: f32) {
        let take = portion.clamp(0.1, self.surplus_signal.max(0.1));
        match path {
            AllocatePath::FlowOutward => self.flow_total += take,
            AllocatePath::StewardReserve => self.reserve_total += take,
        }
        self.surplus_signal = (self.surplus_signal - take).max(0.0);
        self.last_choice = Some(path);
        self.choices_made = self.choices_made.saturating_add(1);
        if self.surplus_signal < self.eligibility_threshold * 0.25 {
            self.eligible = false;
            self.panel_open = false;
        }
    }
}

#[derive(Component)]
pub struct AllocatePanelRoot;

#[derive(Component)]
pub struct AllocateBodyText;

#[derive(Component)]
pub struct AllocateFlowButton;

#[derive(Component)]
pub struct AllocateReserveButton;

pub struct RbeAllocateChoicePlugin;

impl Plugin for RbeAllocateChoicePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RbeAllocateChoice>()
            .add_systems(Startup, spawn_allocate_panel)
            .add_systems(
                Update,
                (
                    soft_surplus_from_rbe_feedback,
                    toggle_allocate_panel,
                    update_allocate_visibility,
                    update_allocate_body,
                    handle_allocate_buttons,
                ),
            );
    }
}

fn spawn_allocate_panel(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(140.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(520.0),
                    margin: UiRect::left(Val::Px(-260.0)),
                    padding: UiRect::all(Val::Px(16.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    border: UiRect::all(Val::Px(1.5)),
                    ..default()
                },
                background_color: Color::srgba(0.04, 0.08, 0.10, 0.94).into(),
                border_color: Color::srgba(0.55, 0.88, 0.70, 0.55).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            AllocatePanelRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "Abundance choice · surplus is ready",
                    TextStyle {
                        font_size: 15.0,
                        color: Color::srgb(0.85, 0.98, 0.90),
                        ..default()
                    },
                ),
                AllocateBodyText,
            ));

            p.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|row| {
                row.spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        background_color: Color::srgba(0.12, 0.28, 0.22, 0.95).into(),
                        border_color: Color::srgb(0.45, 0.90, 0.70).into(),
                        ..default()
                    },
                    AllocateFlowButton,
                ))
                .with_children(|b| {
                    b.spawn(TextBundle::from_section(
                        "Flow outward",
                        TextStyle {
                            font_size: 14.0,
                            color: Color::srgb(0.90, 1.0, 0.95),
                            ..default()
                        },
                    ));
                });

                row.spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        background_color: Color::srgba(0.14, 0.18, 0.28, 0.95).into(),
                        border_color: Color::srgb(0.55, 0.75, 0.95).into(),
                        ..default()
                    },
                    AllocateReserveButton,
                ))
                .with_children(|b| {
                    b.spawn(TextBundle::from_section(
                        "Steward reserve",
                        TextStyle {
                            font_size: 14.0,
                            color: Color::srgb(0.90, 0.95, 1.0),
                            ..default()
                        },
                    ));
                });
            });

            p.spawn(TextBundle::from_section(
                "R toggle · both paths are abundance — never scarcity",
                TextStyle {
                    font_size: 12.0,
                    color: Color::srgb(0.65, 0.80, 0.75),
                    ..default()
                },
            ));
        });
}

fn soft_surplus_from_rbe_feedback(
    rbe_ui: Option<Res<crate::lived_hour_support::RbeUiSync>>,
    mut allocate: ResMut<RbeAllocateChoice>,
    mut last: Local<Option<String>>,
) {
    let Some(rbe_ui) = rbe_ui else {
        return;
    };
    let Some(ref fb) = rbe_ui.last_harvest_feedback else {
        return;
    };
    if last.as_ref() == Some(fb) {
        return;
    }
    *last = Some(fb.clone());

    // Soft parse: any positive harvest line adds a unit of surplus signal
    let positive = fb.contains('+')
        || fb.contains("Sustainable")
        || fb.contains("Epiphany")
        || fb.contains("abundance")
        || fb.contains("Council");
    if positive && !fb.contains("failed") && !fb.contains("Failed") {
        allocate.note_surplus(1.0);
        if allocate.eligible && !allocate.auto_offered && !allocate.panel_open {
            allocate.panel_open = true;
            allocate.auto_offered = true;
        }
    }
}

fn toggle_allocate_panel(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut allocate: ResMut<RbeAllocateChoice>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        if allocate.panel_open {
            allocate.panel_open = false;
        } else if allocate.eligible || allocate.surplus_signal > 0.0 {
            allocate.eligible = true;
            allocate.panel_open = true;
        }
    }
    if keyboard.just_pressed(KeyCode::Escape) && allocate.panel_open {
        allocate.panel_open = false;
    }
}

fn update_allocate_visibility(
    allocate: Res<RbeAllocateChoice>,
    mut q: Query<&mut Visibility, With<AllocatePanelRoot>>,
) {
    let show = allocate.panel_open;
    for mut vis in &mut q {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn update_allocate_body(
    allocate: Res<RbeAllocateChoice>,
    mut q: Query<&mut Text, With<AllocateBodyText>>,
) {
    if !allocate.is_changed() {
        return;
    }
    let body = format!(
        "Abundance choice · surplus {:.1}  ·  flowed {:.1}  ·  reserved {:.1}\nBoth paths thrive — pick a direction",
        allocate.surplus_signal, allocate.flow_total, allocate.reserve_total
    );
    for mut text in &mut q {
        if let Some(s) = text.sections.get_mut(0) {
            s.value = body.clone();
        }
    }
}

fn handle_allocate_buttons(
    mut allocate: ResMut<RbeAllocateChoice>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
    flow_q: Query<&Interaction, (Changed<Interaction>, With<AllocateFlowButton>)>,
    reserve_q: Query<&Interaction, (Changed<Interaction>, With<AllocateReserveButton>)>,
) {
    if !allocate.panel_open {
        return;
    }
    let now = time.elapsed_seconds_f64();
    for inter in &flow_q {
        if *inter == Interaction::Pressed {
            allocate.apply(AllocatePath::FlowOutward, 1.0);
            fire_thriving(&mut moments, ThrivingKind::FirstMercyHarvest, now);
            info!(target: "powrush::rbe", "Allocate: flow outward");
        }
    }
    for inter in &reserve_q {
        if *inter == Interaction::Pressed {
            allocate.apply(AllocatePath::StewardReserve, 1.0);
            fire_thriving(&mut moments, ThrivingKind::FirstInventoryOpen, now);
            info!(target: "powrush::rbe", "Allocate: steward reserve");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surplus_unlocks_and_allocate_reduces() {
        let mut a = RbeAllocateChoice::default();
        a.note_surplus(1.5);
        assert!(a.eligible);
        a.apply(AllocatePath::FlowOutward, 1.0);
        assert!(a.flow_total >= 1.0);
        assert!(a.surplus_signal < 1.5);
    }

    #[test]
    fn both_paths_valid() {
        let mut a = RbeAllocateChoice::default();
        a.note_surplus(2.0);
        a.apply(AllocatePath::StewardReserve, 1.0);
        a.apply(AllocatePath::FlowOutward, 1.0);
        assert_eq!(a.choices_made, 2);
    }
}
