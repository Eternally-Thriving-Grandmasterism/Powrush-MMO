/*!
 * Local Sovereign Session — first hour is complete alone (v21.99.4)
 *
 * No dedicated servers. No other humans in the realm yet.
 * The nodes, journey, climates, and pool must still reward the person at the keys.
 * Multiplayer / Steam / peer files are future sockets — never first-hour gates.
 *
 * PATSAGi ruling: do not teach launch-ops during play.
 *
 * CARD FLESH-SOVEREIGN-HOUR — the existing banner may name the Place
 * (`HexTravelState::chip_name`). Absent travel keeps the banner byte for byte.
 * The log stays offline / single human. No second HUD. No Online.
 * Peak memory, cited: walked · tended · week was the bill · yard remembered.
 *
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::first_session_guidance::FirstSessionGuidance;

const BANNER_SECS: f64 = 8.5;

/// Existing banner. Absent travel keeps this byte for byte.
const BANNER_LINE: &str = "This hour is yours alone · no servers · the nodes still answer";
/// Existing log. Stays offline / single human. Place does not enter this line.
const LOG_LINE: &str = "local first session — offline, single human, complete without peers";

/// CARD FLESH-SOVEREIGN-HOUR — `{place} · {banner}` when a chip is present.
/// `None` returns the bare banner. One string. No second widget.
fn sovereign_banner_line(place: Option<&str>) -> String {
    match place {
        Some(place) => format!("{place} · {BANNER_LINE}"),
        None => BANNER_LINE.to_string(),
    }
}

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

fn spawn_banner(
    mut commands: Commands,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
) {
    let line = sovereign_banner_line(travel.as_ref().map(|state| state.chip_name()));
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
                    line,
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
        "{LOG_LINE}"
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
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    mut root: Query<&mut Visibility, With<SovereignBannerRoot>>,
    mut text_q: Query<&mut Text, With<SovereignBannerText>>,
) {
    let show = !session.dismissed && time.elapsed_seconds_f64() < session.banner_until;
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if !show {
        return;
    }
    let line = sovereign_banner_line(travel.as_ref().map(|state| state.chip_name()));
    for mut text in &mut text_q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != line {
                s.value = line.clone();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::hex_travel::PlaceId;

    /// CARD FLESH-SOVEREIGN-HOUR — Place prefixes the banner; no travel keeps it exact.
    #[test]
    fn flesh_sovereign_banner_names_chip_or_keeps_banner() {
        assert_eq!(sovereign_banner_line(None), BANNER_LINE);
        assert_eq!(
            sovereign_banner_line(None),
            "This hour is yours alone · no servers · the nodes still answer"
        );
        assert_eq!(
            LOG_LINE,
            "local first session — offline, single human, complete without peers"
        );
        assert!(LOG_LINE.contains("offline"));
        assert!(LOG_LINE.contains("single human"));
        assert!(!LOG_LINE.contains("Sanctuary"));
        assert!(!LOG_LINE.contains("Heartwood"));
        assert!(!LOG_LINE.contains("Depths"));

        let cases = [
            (PlaceId::Sanctuary, "Sanctuary Prime"),
            (PlaceId::Heartwood, "Heartwood"),
            (PlaceId::Depths, "Depths"),
        ];
        for (id, name) in cases {
            assert_eq!(id.chip_name(), name);
            let travel = crate::hex_travel::HexTravelState { current: id };
            assert_eq!(travel.chip_name(), id.chip_name());
            assert_eq!(
                sovereign_banner_line(Some(travel.chip_name())),
                format!("{name} · {BANNER_LINE}")
            );
        }

        for sample in [
            sovereign_banner_line(Some(PlaceId::Sanctuary.chip_name())),
            sovereign_banner_line(Some(PlaceId::Heartwood.chip_name())),
            sovereign_banner_line(Some(PlaceId::Depths.chip_name())),
            sovereign_banner_line(None),
        ] {
            let low = sample.to_lowercase();
            assert!(!low.contains("gold"), "{sample}");
            assert!(!low.contains("market"), "{sample}");
            assert!(!low.contains("xp"), "{sample}");
            assert!(!low.contains("hud"), "{sample}");
            assert!(!low.contains("online"), "{sample}");
            assert!(!sample.contains("Threshold"), "{sample}");
        }
    }
}
