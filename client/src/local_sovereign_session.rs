/*!
 * Local Sovereign Session — first hour is complete alone (v21.99.4)
 *
 * No dedicated servers. No other humans in the realm yet.
 * The nodes, journey, climates, and pool must still reward the person at the keys.
 * Multiplayer / Steam / peer files are future sockets — never first-hour gates.
 *
 * PATSAGi ruling: do not teach launch-ops during play.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::first_session_guidance::FirstSessionGuidance;

const BANNER_SECS: f64 = 8.5;

#[derive(Resource, Debug)]
pub struct LocalSovereignSession {
    pub banner_until: f64,
    pub dismissed: bool,
    pub announced: bool,
}

impl Default for LocalSovereignSession {
    fn default() -> Self {
        Self {
            banner_until: BANNER_SECS,
            dismissed: false,
            announced: false,
        }
    }
}

#[derive(Component)]
struct SovereignBannerRoot;
#[derive(Component)]
struct SovereignBannerText;

pub struct LocalSovereignSessionPlugin;

impl Plugin for LocalSovereignSessionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LocalSovereignSession>()
            .add_systems(Startup, spawn_banner)
            .add_systems(Update, (announce_once, dismiss_on_intent, update_banner));
    }
}

fn spawn_banner(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(52.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(520.0),
                    margin: UiRect::left(Val::Px(-260.0)),
                    padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.04, 0.06, 0.09, 0.86).into(),
                border_color: Color::srgba(0.62, 0.78, 0.92, 0.38).into(),
                visibility: Visibility::Visible,
                ..default()
            },
            SovereignBannerRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "This hour is yours alone · no servers · the nodes still answer",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.84, 0.91, 1.0),
                        ..default()
                    },
                ),
                SovereignBannerText,
            ));
        });
}

fn announce_once(mut session: ResMut<LocalSovereignSession>) {
    if session.announced {
        return;
    }
    session.announced = true;
    info!(
        target: "powrush::sovereign",
        "local first session — offline, single human, complete without peers"
    );
}

fn dismiss_on_intent(
    keyboard: Res<ButtonInput<KeyCode>>,
    harvest: Res<FirstHarvestEpiphany>,
    guidance: Res<FirstSessionGuidance>,
    mut session: ResMut<LocalSovereignSession>,
) {
    if session.dismissed {
        return;
    }
    let moving = keyboard.pressed(KeyCode::KeyW)
        || keyboard.pressed(KeyCode::KeyA)
        || keyboard.pressed(KeyCode::KeyS)
        || keyboard.pressed(KeyCode::KeyD);
    if moving || harvest.first_harvest_lived || guidance.dismissed {
        session.dismissed = true;
    }
}

fn update_banner(
    time: Res<Time>,
    session: Res<LocalSovereignSession>,
    mut root: Query<&mut Visibility, With<SovereignBannerRoot>>,
) {
    let show = !session.dismissed && time.elapsed_seconds_f64() < session.banner_until;
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
