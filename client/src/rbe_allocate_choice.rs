/*!
 * Soft RBE Allocate Choice — credit face (CREDIT_RESERVE · H-2026-09-11-D4)
 *
 * After meaningful surplus, invite a voluntary allocation:
 *   • Flow — field restore / share credit into the lattice
 *   • Reserve — repair-rights steward hold for later mend
 *
 * Credit ≠ gold. Neither path is sell / price / ticker / Market.
 * Still-frame names logistics, not currency.
 *
 * Controls: **R** toggles panel when eligible · **1** Flow · **2** Reserve · Esc / R closes
 *
 * PATSAGi + TOLC 8 | AG-SML v1.0 | Contact: info@Rathor.ai
 * Thunder locked in. Yoi ⚡
 */

use bevy::input::gamepad::GamepadRumbleRequest;
use bevy::prelude::*;

use shared::climate_node::AllocKind;

use crate::first_session_guidance::{credit_share, FirstSessionGuidance};
use crate::harvest_feel::rumble_mercy_harvest;
use crate::lived_hour_bind::LivedHourBind;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocatePath {
    FlowOutward,
    StewardReserve,
}

impl AllocatePath {
    pub fn title(self) -> &'static str {
        match self {
            AllocatePath::FlowOutward => "Flow · field restore",
            AllocatePath::StewardReserve => "Reserve · repair-rights",
        }
    }

    pub fn line(self) -> &'static str {
        match self {
            AllocatePath::FlowOutward => {
                "Share credit into the lattice — restore shared field"
            }
            AllocatePath::StewardReserve => {
                "Hold repair-rights credit — steward for later mend"
            }
        }
    }
}

/// Allocate-face copy stays credit logistics — never gold / Market / sell / price.
pub fn allocate_copy_is_honest(s: &str) -> bool {
    let low = s.to_lowercase();
    !low.contains("gold")
        && !low.contains("market")
        && !low.contains("price")
        && !low.contains("sell")
        && !low.contains("ticker")
        && !low.contains("currency")
        && !low.contains("auction")
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
                    "Allocate credit · Flow or Reserve",
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
                        "Flow · field restore",
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
                        "Reserve · repair-rights",
                        TextStyle {
                            font_size: 14.0,
                            color: Color::srgb(0.90, 0.95, 1.0),
                            ..default()
                        },
                    ));
                });
            });

            p.spawn(TextBundle::from_section(
                "1 Flow · 2 Reserve · R close · credit logistics",
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
    if keyboard.just_pressed(soft_play_bindings::ALLOCATE) {
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
        "Allocate credit · ready {:.1}  ·  flowed {:.1}  ·  reserved {:.1}\nFlow restores field · Reserve holds repair-rights",
        allocate.surplus_signal, allocate.flow_total, allocate.reserve_total
    );
    for mut text in &mut q {
        if let Some(s) = text.sections.get_mut(0) {
            s.value = body.clone();
        }
    }
}

fn commit_allocate(
    allocate: &mut RbeAllocateChoice,
    moments: &mut ThrivingMoments,
    guidance: &mut FirstSessionGuidance,
    rumble: &mut EventWriter<GamepadRumbleRequest>,
    gamepads: &Gamepads,
    bind: &mut LivedHourBind,
    path: AllocatePath,
    now: f64,
) {
    allocate.apply(path, 1.0);
    rumble_mercy_harvest(rumble, gamepads);
    fire_thriving(moments, ThrivingKind::FirstShare, now);
    credit_share(guidance);
    let kind = match path {
        AllocatePath::FlowOutward => AllocKind::Flow,
        AllocatePath::StewardReserve => AllocKind::Reserve,
    };
    let _ = bind.allocate(kind);
    info!(target: "powrush::rbe", ?path, "Allocate committed");
}

fn handle_allocate_buttons(
    mut allocate: ResMut<RbeAllocateChoice>,
    mut moments: ResMut<ThrivingMoments>,
    mut guidance: ResMut<FirstSessionGuidance>,
    mut rumble: EventWriter<GamepadRumbleRequest>,
    gamepads: Res<Gamepads>,
    mut bind: ResMut<LivedHourBind>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    flow_q: Query<&Interaction, (Changed<Interaction>, With<AllocateFlowButton>)>,
    reserve_q: Query<&Interaction, (Changed<Interaction>, With<AllocateReserveButton>)>,
) {
    if !allocate.panel_open {
        return;
    }
    let now = time.elapsed_seconds_f64();
    let path = if keyboard.just_pressed(KeyCode::Digit1) {
        Some(AllocatePath::FlowOutward)
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        Some(AllocatePath::StewardReserve)
    } else {
        None
    };
    if let Some(path) = path {
        commit_allocate(
            &mut allocate, &mut moments, &mut guidance, &mut rumble, &gamepads, &mut bind, path, now,
        );
        return;
    }
    for inter in &flow_q {
        if *inter == Interaction::Pressed {
            commit_allocate(
                &mut allocate, &mut moments, &mut guidance, &mut rumble, &gamepads,
                &mut bind, AllocatePath::FlowOutward, now,
            );
            return;
        }
    }
    for inter in &reserve_q {
        if *inter == Interaction::Pressed {
            commit_allocate(
                &mut allocate, &mut moments, &mut guidance, &mut rumble, &gamepads,
                &mut bind, AllocatePath::StewardReserve, now,
            );
            return;
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

    #[test]
    fn digit_labels_are_flow_then_reserve() {
        assert_eq!(AllocatePath::FlowOutward.title(), "Flow · field restore");
        assert_eq!(AllocatePath::StewardReserve.title(), "Reserve · repair-rights");
    }

    #[test]
    fn allocate_face_names_credit_not_gold() {
        for path in [AllocatePath::FlowOutward, AllocatePath::StewardReserve] {
            assert!(allocate_copy_is_honest(path.title()), "{}", path.title());
            assert!(allocate_copy_is_honest(path.line()), "{}", path.line());
        }
        assert!(AllocatePath::FlowOutward.line().contains("field"));
        assert!(AllocatePath::StewardReserve.line().contains("repair-rights"));
        assert!(allocate_copy_is_honest(
            "Allocate credit · Flow or Reserve"
        ));
        assert!(allocate_copy_is_honest(
            "1 Flow · 2 Reserve · R close · credit logistics"
        ));
        assert!(!allocate_copy_is_honest("sell gold on Market"));
        assert!(!allocate_copy_is_honest("price ticker"));
    }
}
