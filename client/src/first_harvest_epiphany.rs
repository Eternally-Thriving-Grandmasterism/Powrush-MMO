/*!
 * First Harvest Epiphany — lived first-hour loop (v21.99.0)
 *
 * Walk to a glowing node → E in reach → world answers.
 * If nodes exist, E at empty air is a gentle step-closer, not a harvest.
 *
 * PATSAGi + TOLC 8 | Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::abundance_journey_echo::{AbundanceJourneyEcho, JourneyKind};
use crate::first_session_guidance::{credit_epiphany, credit_harvest, FirstSessionGuidance, GuidanceObjective};
use crate::mercy_harvest_nodes::{apply_node_harvest, MercyHarvestNode, NearbyMercyNode};
use crate::rbe_client_ui_sync::RbeUiSync;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

const PROMPT_LINGER: f64 = 2.4;
const PULSE_SECS: f64 = 4.2;
const WELCOME_SECS: f64 = 6.0;
const REPEAT_COOLDOWN: f64 = 1.6;

#[derive(Resource, Debug)]
pub struct FirstHarvestEpiphany {
    pub first_harvest_lived: bool,
    pub first_epiphany_lived: bool,
    pub welcome_shown: bool,
    pub prompt_until: f64,
    pub pulse_until: f64,
    pub pulse_line: String,
    pub last_interact_at: f64,
    pub harvests_this_session: u32,
}

impl Default for FirstHarvestEpiphany {
    fn default() -> Self {
        Self {
            first_harvest_lived: false,
            first_epiphany_lived: false,
            welcome_shown: false,
            prompt_until: 9999.0,
            pulse_until: 0.0,
            pulse_line: String::new(),
            last_interact_at: -999.0,
            harvests_this_session: 0,
        }
    }
}

impl FirstHarvestEpiphany {
    pub fn prompt_visible(&self, now: f64, guidance: &FirstSessionGuidance) -> bool {
        if self.first_harvest_lived && now > self.prompt_until {
            return false;
        }
        matches!(
            guidance.objective,
            GuidanceObjective::ApproachGlowingNode
                | GuidanceObjective::HarvestWithInteract
                | GuidanceObjective::MoveAround
        ) || !self.first_harvest_lived
            || now < self.prompt_until
    }
}

#[derive(Component)]
pub struct WorldCarePromptRoot;
#[derive(Component)]
pub struct WorldCarePromptText;
#[derive(Component)]
pub struct HarvestPulseRoot;
#[derive(Component)]
pub struct HarvestPulseText;
#[derive(Component)]
pub struct WelcomeBackRoot;
#[derive(Component)]
pub struct WelcomeBackText;

pub struct FirstHarvestEpiphanyPlugin;

impl Plugin for FirstHarvestEpiphanyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FirstHarvestEpiphany>()
            .add_systems(Startup, spawn_lived_surfaces)
            .add_systems(
                Update,
                (
                    maybe_welcome_back,
                    handle_interact_harvest,
                    update_world_care_prompt,
                    update_harvest_pulse,
                    update_welcome_back,
                ),
            );
    }
}

fn spawn_lived_surfaces(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(128.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(420.0),
                    margin: UiRect::left(Val::Px(-210.0)),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::srgba(0.03, 0.10, 0.07, 0.78).into(),
                border_color: Color::srgba(0.55, 0.95, 0.72, 0.50).into(),
                visibility: Visibility::Visible,
                ..default()
            },
            WorldCarePromptRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "Walk toward a glowing node",
                    TextStyle {
                        font_size: 17.0,
                        color: Color::srgb(0.82, 1.0, 0.90),
                        ..default()
                    },
                ),
                WorldCarePromptText,
            ));
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(118.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(560.0),
                    margin: UiRect::left(Val::Px(-280.0)),
                    padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.2)),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..default()
                },
                background_color: Color::srgba(0.07, 0.12, 0.08, 0.90).into(),
                border_color: Color::srgba(0.85, 0.98, 0.55, 0.62).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            HarvestPulseRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(0.96, 1.0, 0.82),
                        ..default()
                    },
                ),
                HarvestPulseText,
            ));
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(16.0),
                    left: Val::Px(16.0),
                    width: Val::Px(380.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::srgba(0.05, 0.07, 0.10, 0.88).into(),
                border_color: Color::srgba(0.70, 0.82, 0.95, 0.40).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            WelcomeBackRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 13.5,
                        color: Color::srgb(0.86, 0.92, 1.0),
                        ..default()
                    },
                ),
                WelcomeBackText,
            ));
        });
}

fn maybe_welcome_back(
    echo: Res<AbundanceJourneyEcho>,
    mut state: ResMut<FirstHarvestEpiphany>,
    time: Res<Time>,
    mut text_q: Query<&mut Text, With<WelcomeBackText>>,
) {
    if state.welcome_shown || !echo.loaded {
        return;
    }
    if echo.lines.is_empty() && !echo.last_practice_sealed {
        state.welcome_shown = true;
        return;
    }
    state.welcome_shown = true;
    let now = time.elapsed_seconds_f64();
    let line = if echo.last_practice_sealed {
        "Welcome back · your sealed practice still travels with you · J to remember".to_string()
    } else if let Some(last) = echo.lines.last() {
        format!("Welcome back · last echo: {} · J to open journey", last.text)
    } else {
        "Welcome back · the Lattice held your place".to_string()
    };
    for mut text in &mut text_q {
        if let Some(s) = text.sections.get_mut(0) {
            s.value = line.clone();
        }
    }
    state.last_interact_at = -(now + WELCOME_SECS);
    info!(target: "powrush::epiphany", "welcome-back shown from journey memory");
}

fn welcome_visible(state: &FirstHarvestEpiphany, now: f64) -> bool {
    state.welcome_shown && state.last_interact_at < 0.0 && now < (-state.last_interact_at)
}

fn handle_interact_harvest(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<FirstHarvestEpiphany>,
    mut guidance: ResMut<FirstSessionGuidance>,
    mut moments: ResMut<ThrivingMoments>,
    mut echo: ResMut<AbundanceJourneyEcho>,
    mut nearby: ResMut<NearbyMercyNode>,
    mut nodes: Query<&mut MercyHarvestNode>,
    mut rbe_ui: Option<ResMut<RbeUiSync>>,
    time: Res<Time>,
) {
    if !keyboard.just_pressed(soft_play_bindings::INTERACT) {
        return;
    }
    let now = time.elapsed_seconds_f64();
    if now - state.last_interact_at.abs() < REPEAT_COOLDOWN && state.last_interact_at > 0.0 {
        return;
    }

    if nearby.nodes_exist && !nearby.in_range {
        state.pulse_until = now + 2.2;
        let name = nearby.name.unwrap_or("the glowing node");
        state.pulse_line = format!("Step closer to {} · then E", name);
        info!(target: "powrush::epiphany", dist = nearby.distance, "E out of reach");
        return;
    }

    state.last_interact_at = now;
    state.harvests_this_session = state.harvests_this_session.saturating_add(1);

    if let Some(entity) = nearby.entity {
        if let Ok(mut node) = nodes.get_mut(entity) {
            apply_node_harvest(&mut node);
            nearby.last_harvested = Some(entity);
        }
    }

    credit_harvest(&mut guidance);
    credit_epiphany(&mut guidance);
    fire_thriving(&mut moments, ThrivingKind::FirstMercyHarvest, now);

    let first = !state.first_harvest_lived;
    state.first_harvest_lived = true;
    state.first_epiphany_lived = true;
    state.prompt_until = now + PROMPT_LINGER;
    state.pulse_until = now + PULSE_SECS;
    let node_name = nearby.name.unwrap_or("the node");
    state.pulse_line = if first {
        format!("{node_name} still glows · you took with restraint · abundance remains")
    } else {
        format!(
            "Mercy harvest at {node_name} · climate still thrives · care {}",
            state.harvests_this_session
        )
    };

    if first {
        echo.push(
            JourneyKind::Note,
            format!("First mercy harvest at {node_name} — left glowing"),
        );
    }

    if let Some(ref mut ui) = rbe_ui {
        ui.last_harvest_feedback = Some(if first {
            format!("Epiphany · Sustainable harvest at {node_name}")
        } else {
            format!("Sustainable harvest at {node_name} · mercy refinement active")
        });
    }

    info!(
        target: "powrush::epiphany",
        first,
        n = state.harvests_this_session,
        node = node_name,
        "lived harvest on E"
    );
}

fn update_world_care_prompt(
    state: Res<FirstHarvestEpiphany>,
    guidance: Res<FirstSessionGuidance>,
    nearby: Res<NearbyMercyNode>,
    time: Res<Time>,
    mut root: Query<&mut Visibility, With<WorldCarePromptRoot>>,
    mut text_q: Query<&mut Text, With<WorldCarePromptText>>,
) {
    let now = time.elapsed_seconds_f64();
    let approaching = nearby.nodes_exist && !state.first_harvest_lived;
    let show = (state.prompt_visible(now, &guidance) || approaching) && !guidance.dismissed;
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    let line = if nearby.nodes_exist && nearby.in_range {
        "E  —  harvest with mercy"
    } else if nearby.nodes_exist {
        "Walk toward the glowing node"
    } else if state.first_harvest_lived {
        "The node remembers your care"
    } else {
        "E  —  harvest with mercy"
    };
    for mut text in &mut text_q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != line {
                s.value = line.to_string();
            }
        }
    }
}

fn update_harvest_pulse(
    state: Res<FirstHarvestEpiphany>,
    time: Res<Time>,
    mut root: Query<&mut Visibility, With<HarvestPulseRoot>>,
    mut text_q: Query<&mut Text, With<HarvestPulseText>>,
) {
    let now = time.elapsed_seconds_f64();
    let show = now < state.pulse_until && !state.pulse_line.is_empty();
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if show {
        for mut text in &mut text_q {
            if let Some(s) = text.sections.get_mut(0) {
                if s.value != state.pulse_line {
                    s.value = state.pulse_line.clone();
                }
            }
        }
    }
}

fn update_welcome_back(
    state: Res<FirstHarvestEpiphany>,
    time: Res<Time>,
    mut root: Query<&mut Visibility, With<WelcomeBackRoot>>,
) {
    let show = welcome_visible(&state, time.elapsed_seconds_f64());
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_shows_before_first_harvest() {
        let state = FirstHarvestEpiphany::default();
        let g = FirstSessionGuidance::default();
        assert!(state.prompt_visible(0.5, &g));
    }

    #[test]
    fn prompt_hides_after_linger() {
        let mut state = FirstHarvestEpiphany::default();
        state.first_harvest_lived = true;
        state.prompt_until = 2.0;
        let mut g = FirstSessionGuidance::default();
        g.objective = GuidanceObjective::FreeExploration;
        assert!(!state.prompt_visible(5.0, &g));
    }
}
