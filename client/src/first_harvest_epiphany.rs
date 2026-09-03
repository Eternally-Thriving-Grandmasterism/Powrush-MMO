/*!
 * First Harvest Epiphany — lived first-hour loop (v22.2.0)
 *
 * Tap E / pad West = take. Hold E ~0.42s = tend (node recovers).
 *
 * PATSAGi + TOLC 8 | Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::input::gamepad::GamepadRumbleRequest;
use bevy::prelude::*;

use crate::abundance_journey_echo::{AbundanceJourneyEcho, JourneyKind};
use crate::first_session_guidance::{credit_epiphany, credit_harvest, FirstSessionGuidance, GuidanceObjective};
use crate::harvest_feel::{credit_soft_and_global, rumble_harvest, rumble_mercy_harvest, SoftRbePool};
use crate::hour_sacred::HourSacred;
use crate::input::PlayerInput;
use crate::mercy_harvest_nodes::{apply_node_harvest, apply_node_tend, MercyHarvestNode, NearbyMercyNode};
use crate::lived_hour_support::RbeGlobalState;
use crate::lived_hour_support::RbeUiSync;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};
use crate::world_answer::{fire_world_answer, AnswerKind, WorldAnswer};

const PROMPT_LINGER: f64 = 2.4;
const PULSE_SECS: f64 = 4.2;
const WELCOME_SECS: f64 = 6.0;
const REPEAT_COOLDOWN: f64 = 1.2;
const TEND_HOLD: f64 = 0.42;

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
    pub tends_this_session: u32,
    /// Frontier+ hex, no charter_id. E must not harvest.
    pub peace_visitor: bool,
    /// Voice sash open with a live card. E votes; do not harvest.
    pub beacon_voice: bool,
    /// Ledger sash open. E Bind/Escort; do not harvest.
    pub ledger_bind: bool,
    /// Embassy lamp live, not yet seated. E Request seat; do not harvest.
    pub embassy_lamp: bool,
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
            tends_this_session: 0,
            peace_visitor: false,
            beacon_voice: false,
            ledger_bind: false,
            embassy_lamp: false,
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

#[derive(Default)]
struct InteractHold {
    holding: bool,
    started: f64,
    tended: bool,
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
                    mark_peace_visitor,
                    maybe_welcome_back,
                    handle_interact_harvest,
                    update_world_care_prompt,
                    update_harvest_pulse,
                    update_welcome_back,
                ).chain(),
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
                    width: Val::Px(460.0),
                    margin: UiRect::left(Val::Px(-230.0)),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
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
                        font_size: 16.0,
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
}

fn welcome_visible(state: &FirstHarvestEpiphany, now: f64) -> bool {
    state.welcome_shown && state.last_interact_at < 0.0 && now < (-state.last_interact_at)
}

fn mark_peace_visitor(hour: Res<HourSacred>, mut state: ResMut<FirstHarvestEpiphany>) {
    state.peace_visitor = hour.session.peace_visitor_on_frontier();
}

fn handle_interact_harvest(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_input: Res<PlayerInput>,
    mut hold: Local<InteractHold>,
    mut state: ResMut<FirstHarvestEpiphany>,
    mut guidance: ResMut<FirstSessionGuidance>,
    mut moments: ResMut<ThrivingMoments>,
    mut echo: ResMut<AbundanceJourneyEcho>,
    mut nearby: ResMut<NearbyMercyNode>,
    mut nodes: Query<&mut MercyHarvestNode>,
    mut pool: ResMut<SoftRbePool>,
    mut global: Option<ResMut<RbeGlobalState>>,
    mut rumble: EventWriter<GamepadRumbleRequest>,
    gamepads: Res<Gamepads>,
    mut rbe_ui: Option<ResMut<RbeUiSync>>,
    mut answer: ResMut<WorldAnswer>,
    time: Res<Time>,
) {
    let now = time.elapsed_seconds_f64();
    let e_down = keyboard.pressed(soft_play_bindings::INTERACT);
    let e_up = keyboard.just_released(soft_play_bindings::INTERACT);
    let pad_tap = player_input.interact;

    if state.peace_visitor {
        if pad_tap || keyboard.just_pressed(soft_play_bindings::INTERACT) || e_up {
            state.pulse_until = now + 2.4;
            state.pulse_line = "Not your charter / Peace visitor".into();
        }
        return;
    }

    if state.beacon_voice {
        return;
    }

    if state.ledger_bind {
        return;
    }

    if state.embassy_lamp {
        return;
    }

    if pad_tap {
        resolve_take(
            now, &mut state, &mut guidance, &mut moments, &mut echo,
            &mut nearby, &mut nodes, &mut pool, global.as_deref_mut(),
            &mut rumble, &gamepads, rbe_ui.as_deref_mut(), &mut answer,
        );
        return;
    }

    if e_down && !hold.holding {
        hold.holding = true;
        hold.started = now;
        hold.tended = false;
        if nearby.nodes_exist && !nearby.in_range {
            state.pulse_until = now + 2.0;
            let name = nearby.name.unwrap_or("the glowing node");
            state.pulse_line = format!("Step closer to {name} · tap E take · hold E tend");
        }
    }

    if hold.holding && e_down && !hold.tended && nearby.in_range && now - hold.started >= TEND_HOLD {
        resolve_tend(
            now, &mut state, &mut nearby, &mut nodes, &mut pool,
            &mut rumble, &gamepads, &mut answer,
        );
        hold.tended = true;
    }

    if e_up {
        if hold.holding && !hold.tended {
            resolve_take(
                now, &mut state, &mut guidance, &mut moments, &mut echo,
                &mut nearby, &mut nodes, &mut pool, global.as_deref_mut(),
                &mut rumble, &gamepads, rbe_ui.as_deref_mut(), &mut answer,
            );
        }
        hold.holding = false;
        hold.tended = false;
    }
}

fn resolve_take(
    now: f64,
    state: &mut FirstHarvestEpiphany,
    guidance: &mut FirstSessionGuidance,
    moments: &mut ThrivingMoments,
    echo: &mut AbundanceJourneyEcho,
    nearby: &mut NearbyMercyNode,
    nodes: &mut Query<&mut MercyHarvestNode>,
    pool: &mut SoftRbePool,
    global: Option<&mut RbeGlobalState>,
    rumble: &mut EventWriter<GamepadRumbleRequest>,
    gamepads: &Gamepads,
    rbe_ui: Option<&mut RbeUiSync>,
    answer: &mut WorldAnswer,
) {
    if now - state.last_interact_at.abs() < REPEAT_COOLDOWN && state.last_interact_at > 0.0 {
        return;
    }
    if nearby.nodes_exist && !nearby.in_range {
        state.pulse_until = now + 2.2;
        let name = nearby.name.unwrap_or("the glowing node");
        state.pulse_line = format!("Step closer to {name} · then E");
        return;
    }

    state.last_interact_at = now;
    state.harvests_this_session = state.harvests_this_session.saturating_add(1);

    let mut node_vitality = 1.0;
    if let Some(entity) = nearby.entity {
        if let Ok(mut node) = nodes.get_mut(entity) {
            node_vitality = node.vitality;
            apply_node_harvest(&mut node);
            nearby.last_harvested = Some(entity);
        }
    }

    let credited = credit_soft_and_global(pool, global, node_vitality);
    let first = !state.first_harvest_lived;
    rumble_harvest(rumble, gamepads, first);
    pool.punch(first);
    credit_harvest(guidance);
    if first {
        credit_epiphany(guidance);
        state.first_epiphany_lived = true;
    }
    fire_thriving(moments, ThrivingKind::FirstMercyHarvest, now);
    fire_world_answer(answer, AnswerKind::Take, now, "taken with mercy");

    state.first_harvest_lived = true;
    state.prompt_until = now + PROMPT_LINGER;
    state.pulse_until = now + PULSE_SECS;
    let node_name = nearby.name.unwrap_or("the node");
    state.pulse_line = if first {
        format!("{node_name} still glows · +{credited:.1} vitality · hold E to tend")
    } else {
        format!("Take at {node_name} · +{credited:.1} · {}", pool.line())
    };

    if first {
        echo.push(
            JourneyKind::Note,
            format!("First mercy harvest at {node_name} — left glowing (+{credited:.1})"),
        );
    }
    if let Some(ui) = rbe_ui {
        ui.last_harvest_feedback = Some(format!("Sustainable +{credited:.1} at {node_name}"));
    }
}

fn resolve_tend(
    now: f64,
    state: &mut FirstHarvestEpiphany,
    nearby: &mut NearbyMercyNode,
    nodes: &mut Query<&mut MercyHarvestNode>,
    pool: &mut SoftRbePool,
    rumble: &mut EventWriter<GamepadRumbleRequest>,
    gamepads: &Gamepads,
    answer: &mut WorldAnswer,
) {
    if now - state.last_interact_at.abs() < REPEAT_COOLDOWN && state.last_interact_at > 0.0 {
        return;
    }
    if !nearby.in_range {
        return;
    }
    state.last_interact_at = now;
    state.tends_this_session = state.tends_this_session.saturating_add(1);

    let mut node_vitality = 1.0;
    if let Some(entity) = nearby.entity {
        if let Ok(mut node) = nodes.get_mut(entity) {
            node_vitality = node.vitality;
            apply_node_tend(&mut node);
        }
    }
    let credited = pool.credit_tend(node_vitality);
    rumble_mercy_harvest(rumble, gamepads);
    fire_world_answer(answer, AnswerKind::Tend, now, "tended — the node breathes");

    state.pulse_until = now + PULSE_SECS;
    let node_name = nearby.name.unwrap_or("the node");
    state.pulse_line = format!("Tended {node_name} · +{credited:.1} harmony · vitality returns");
    info!(target: "powrush::epiphany", node = node_name, "hold-E tend");
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
        "tap E take  ·  hold E tend"
    } else if nearby.nodes_exist {
        "Walk toward the glowing node"
    } else if state.first_harvest_lived {
        "The node remembers your care"
    } else {
        "tap E take  ·  hold E tend"
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
    use crate::hour_sacred::HourSacred;
    use shared::space_law::HexFlag;

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

    #[test]
    fn peace_hour_is_not_a_visitor() {
        let h = HourSacred::default();
        assert!(!h.session.peace_visitor_on_frontier());
        let s = FirstHarvestEpiphany::default();
        assert!(!s.peace_visitor);
    }

    #[test]
    fn frontier_without_charter_is_a_visitor() {
        let mut h = HourSacred::default();
        h.session.hex = HexFlag::Frontier;
        assert!(h.session.peace_visitor_on_frontier());
        h.session.charter_id = Some("iec-1".into());
        assert!(!h.session.peace_visitor_on_frontier());
    }
}
