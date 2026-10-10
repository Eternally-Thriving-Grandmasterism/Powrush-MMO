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
use crate::hud_anchor_registry::{HudSlab, SOVEREIGN};
use crate::first_session_guidance::FirstSessionGuidance;
use crate::title_screen::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};

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
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: SOVEREIGN.top(),
                    left: SOVEREIGN.left(),
                    width: Val::Px(520.0),
                    margin: SOVEREIGN.margin(),
                    padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG.with_alpha(1.0)),
                BorderColor::all(TITLE_BORDER.with_alpha(1.0)),
                Visibility::Visible,
            ),
            SovereignBannerRoot,
            HudSlab(SOVEREIGN.id),
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(line),
TextFont { font_size: FontSize::Px(14.0 / 1.2), ..default() },
TextColor(TITLE_TEXT_PRIMARY),
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
    let show = !session.dismissed && time.elapsed_secs_f64() < session.banner_until;
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
        if text.as_str() != line {
            **text = line.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::hex_travel::PlaceId;

    /// CARD HUD-ANCHOR-REGISTRY-2B — Sovereign position is the registry, Style is the coded literal.
    #[test]
    fn sovereign_style_byte_identical_to_coded_place() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_banner);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<&Node, With<SovereignBannerRoot>>();
        let style = q.single(app.world()).unwrap().clone();
        let coded = Node {
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
        };
        assert_eq!(style, coded);
        assert_eq!(style.top, SOVEREIGN.top());
        assert_eq!(style.left, SOVEREIGN.left());
        assert_eq!(style.margin, SOVEREIGN.margin());
        assert_eq!(style.width, Val::Px(SOVEREIGN.width));
    }

    /// CARD VP-SLABS-REGAL-1 — the sovereign session banner rests on the title palette: opaque
    /// TITLE_PLATE_BG plate, TITLE_BORDER rim at alpha 1, TITLE_TEXT_PRIMARY text.
    #[test]
    fn slabs_regal_sovereign_banner_rests_on_title_palette() {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .add_systems(Startup, spawn_banner);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<(&BorderColor, &BackgroundColor), With<SovereignBannerRoot>>();
        let (border, bg) = q.single(app.world()).unwrap();
        let (border, bg) = (({
            assert_eq!(border.top, border.right, "border edges");
            assert_eq!(border.top, border.bottom, "border edges");
            assert_eq!(border.top, border.left, "border edges");
            border.top
        }).to_srgba(), bg.0.to_srgba());
        let mut t = app.world_mut().query_filtered::<&TextColor, With<SovereignBannerText>>();
        let txt = t.single(app.world()).unwrap().0.to_srgba();
        for (got, want, what) in [
            (bg, TITLE_PLATE_BG.to_srgba(), "plate"),
            (border, TITLE_BORDER.to_srgba(), "rim"),
            (txt, TITLE_TEXT_PRIMARY.to_srgba(), "text"),
        ] {
            assert!((got.red - want.red).abs() < 1e-6, "{what} red");
            assert!((got.green - want.green).abs() < 1e-6, "{what} green");
            assert!((got.blue - want.blue).abs() < 1e-6, "{what} blue");
            assert_eq!(got.alpha, 1.0, "{what} alpha");
        }
    }

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
