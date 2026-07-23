/*!
 * First Session Guidance — Powrush-MMO End-User Experience Layer
 *
 * Soft, non-blocking objective strip that makes the first 5–15 minutes of
 * human play instantly clear without walls of text or forced tutorials.
 *
 * Principles (PATSAGi + TOLC 8):
 * - Progressive disclosure only
 * - Dismissible at any time (mercy skip)
 * - Never blocks movement, harvest, or joy
 * - Language-aware via Localization keys when available
 *
 * AG-SML v1.0 | Contact: info@Rathor.ai
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

/// Soft objective the player is gently invited to try next.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuidanceObjective {
    MoveAround,
    ApproachGlowingNode,
    HarvestWithSpace,
    OpenInventory,
    FeelFirstEpiphany,
    MeetCouncilWhisper,
    FreeExploration,
}

impl GuidanceObjective {
    pub fn prompt(&self) -> &'static str {
        match self {
            GuidanceObjective::MoveAround => "WASD or Arrow keys to walk · feel the Lattice",
            GuidanceObjective::ApproachGlowingNode => "Walk toward the soft glowing harvest node",
            GuidanceObjective::HarvestWithSpace => "Press Space near the node to harvest with mercy",
            GuidanceObjective::OpenInventory => "Press I to open your inventory & hotbar",
            GuidanceObjective::FeelFirstEpiphany => "Stay present — a Divine Whisper may bloom",
            GuidanceObjective::MeetCouncilWhisper => "Listen for the Council’s first soft invitation",
            GuidanceObjective::FreeExploration => "You are sovereign. Explore, nurture, thrive.",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            GuidanceObjective::MoveAround => GuidanceObjective::ApproachGlowingNode,
            GuidanceObjective::ApproachGlowingNode => GuidanceObjective::HarvestWithSpace,
            GuidanceObjective::HarvestWithSpace => GuidanceObjective::OpenInventory,
            GuidanceObjective::OpenInventory => GuidanceObjective::FeelFirstEpiphany,
            GuidanceObjective::FeelFirstEpiphany => GuidanceObjective::MeetCouncilWhisper,
            GuidanceObjective::MeetCouncilWhisper => GuidanceObjective::FreeExploration,
            GuidanceObjective::FreeExploration => GuidanceObjective::FreeExploration,
        }
    }
}

#[derive(Resource)]
pub struct FirstSessionGuidance {
    pub active: bool,
    pub dismissed: bool,
    pub objective: GuidanceObjective,
    pub harvests_completed: u32,
    pub moved_distance: f32,
    pub inventory_opened: bool,
    pub epiphany_felt: bool,
    pub shown_at_seconds: f64,
}

impl Default for FirstSessionGuidance {
    fn default() -> Self {
        Self {
            active: true,
            dismissed: false,
            objective: GuidanceObjective::MoveAround,
            harvests_completed: 0,
            moved_distance: 0.0,
            inventory_opened: false,
            epiphany_felt: false,
            shown_at_seconds: 0.0,
        }
    }
}

impl FirstSessionGuidance {
    pub fn dismiss(&mut self) {
        self.dismissed = true;
        self.active = false;
    }

    pub fn advance_if_ready(&mut self) {
        if self.dismissed {
            return;
        }
        let should_advance = match self.objective {
            GuidanceObjective::MoveAround => self.moved_distance > 4.0,
            GuidanceObjective::ApproachGlowingNode => self.moved_distance > 12.0,
            GuidanceObjective::HarvestWithSpace => self.harvests_completed >= 1,
            GuidanceObjective::OpenInventory => self.inventory_opened,
            GuidanceObjective::FeelFirstEpiphany => self.epiphany_felt,
            GuidanceObjective::MeetCouncilWhisper => self.epiphany_felt && self.harvests_completed >= 1,
            GuidanceObjective::FreeExploration => false,
        };
        if should_advance {
            self.objective = self.objective.next();
        }
    }
}

#[derive(Component)]
pub struct FirstSessionGuidanceStrip;

#[derive(Component)]
pub struct FirstSessionGuidanceText;

pub struct FirstSessionGuidancePlugin;

impl Plugin for FirstSessionGuidancePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FirstSessionGuidance>()
            .add_systems(Startup, spawn_guidance_strip)
            .add_systems(
                Update,
                (
                    update_guidance_visibility,
                    update_guidance_text,
                    handle_guidance_dismiss_input,
                    track_simple_progress_signals,
                ),
            );
    }
}

fn spawn_guidance_strip(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(72.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(560.0),
                    margin: UiRect::left(Val::Px(-280.0)),
                    padding: UiRect::axes(Val::Px(18.0), Val::Px(12.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.5)),
                    border_radius: BorderRadius::all(Val::Px(14.0)),
                    ..default()
                },
                background_color: Color::srgba(0.04, 0.07, 0.11, 0.88).into(),
                border_color: Color::srgba(0.45, 0.78, 0.95, 0.55).into(),
                visibility: Visibility::Visible,
                ..default()
            },
            FirstSessionGuidanceStrip,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    GuidanceObjective::MoveAround.prompt(),
                    TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(0.88, 0.94, 1.0),
                        ..default()
                    },
                ),
                FirstSessionGuidanceText,
            ));
        });
}

fn update_guidance_visibility(
    guidance: Res<FirstSessionGuidance>,
    mut query: Query<&mut Visibility, With<FirstSessionGuidanceStrip>>,
) {
    let show = guidance.active && !guidance.dismissed;
    for mut vis in &mut query {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn update_guidance_text(
    guidance: Res<FirstSessionGuidance>,
    mut query: Query<&mut Text, With<FirstSessionGuidanceText>>,
) {
    if !guidance.is_changed() {
        return;
    }
    let prompt = if guidance.dismissed {
        ""
    } else {
        guidance.objective.prompt()
    };
    for mut text in &mut query {
        if let Some(section) = text.sections.get_mut(0) {
            section.value = prompt.to_string();
        }
    }
}

fn handle_guidance_dismiss_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut guidance: ResMut<FirstSessionGuidance>,
) {
    // H hides the strip permanently for this session (mercy skip)
    if keyboard.just_pressed(KeyCode::KeyH) && guidance.active {
        guidance.dismiss();
    }
}

/// Lightweight progress signals that do not require deep system coupling.
fn track_simple_progress_signals(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut guidance: ResMut<FirstSessionGuidance>,
    time: Res<Time>,
) {
    if guidance.dismissed {
        return;
    }

    let moving = keyboard.pressed(KeyCode::KeyW)
        || keyboard.pressed(KeyCode::KeyA)
        || keyboard.pressed(KeyCode::KeyS)
        || keyboard.pressed(KeyCode::KeyD)
        || keyboard.pressed(KeyCode::ArrowUp)
        || keyboard.pressed(KeyCode::ArrowDown)
        || keyboard.pressed(KeyCode::ArrowLeft)
        || keyboard.pressed(KeyCode::ArrowRight);

    if moving {
        guidance.moved_distance += time.delta_seconds() * 6.0;
    }

    if keyboard.just_pressed(KeyCode::KeyI) {
        guidance.inventory_opened = true;
    }

    if keyboard.just_pressed(KeyCode::Space)
        && matches!(
            guidance.objective,
            GuidanceObjective::HarvestWithSpace | GuidanceObjective::ApproachGlowingNode
        )
    {
        guidance.harvests_completed = guidance.harvests_completed.saturating_add(1);
    }

    guidance.advance_if_ready();
}

/// Call from harvest success path to credit a real harvest.
pub fn credit_harvest(guidance: &mut FirstSessionGuidance) {
    guidance.harvests_completed = guidance.harvests_completed.saturating_add(1);
    guidance.advance_if_ready();
}

/// Call when a Divine Whisper / epiphany is presented to the player.
pub fn credit_epiphany(guidance: &mut FirstSessionGuidance) {
    guidance.epiphany_felt = true;
    guidance.advance_if_ready();
}
