/*!
 * Thriving Moments — soft, non-extractive joy feedback
 *
 * Celebrates meaningful firsts without achievement-hunting pressure.
 * Moments are presence markers, not leaderboard fuel.
 *
 * PATSAGi + TOLC 8: Joy gate without scarcity framing.
 * AG-SML v1.0 | Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;
use crate::hud_anchor_registry::{HudSlab, THRIVING};
use crate::title_screen::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThrivingKind {
    FirstMercyHarvest,
    SurfaceCleared,
    PrincipleSealed,
    CouncilInvite,
    FirstInventoryOpen,
    FirstShare,
    FirstArrival,
    FirstVoice,
    FirstSpillWitness,
    FirstBind,
    FirstProofPack,
    FirstEmbassy,
    FirstWarWeek,
    FirstCrownstone,
    FirstRedemption,
    FirstHybrid,
    FirstCompass,
    FirstWell,
}

impl ThrivingKind {
    pub fn line(&self) -> &'static str {
        match self {
            ThrivingKind::FirstMercyHarvest => {
                "Thriving moment · A harvest tended with restraint — the node still glows"
            }
            ThrivingKind::SurfaceCleared => {
                "Thriving moment · One climate practiced — the principle travels with you"
            }
            ThrivingKind::PrincipleSealed => {
                "Thriving moment · Caps Across Climates sealed — same truth, three faces"
            }
            ThrivingKind::CouncilInvite => {
                "Invitation · A soft Council seat is open when you are ready"
            }
            ThrivingKind::FirstInventoryOpen => {
                "Thriving moment · Inventory open — abundance is held, not hoarded"
            }
            ThrivingKind::FirstShare => {
                "Thriving moment · Surplus shared — the yard remembered"
            }
            ThrivingKind::FirstArrival => {
                "The machine exists — a crate arrived"
            }
            ThrivingKind::FirstVoice => {
                "The card carried — the yard voted"
            }
            ThrivingKind::FirstSpillWitness => {
                "Spill is the witness — the pack is readable"
            }
            ThrivingKind::FirstBind => {
                "Bind, not a corpse — escort delivered"
            }
            ThrivingKind::FirstProofPack => {
                "The graph unlocked — repair and logi"
            }
            ThrivingKind::FirstEmbassy => {
                "Seated at the lamp — the book is yours"
            }
            ThrivingKind::FirstWarWeek => {
                "Hex gone green — tons plus restored"
            }
            ThrivingKind::FirstCrownstone => {
                "The stone is seen — path waits"
            }
            ThrivingKind::FirstRedemption => {
                "A tend offered — the grove answers"
            }
            ThrivingKind::FirstHybrid => {
                "Double vision — the ledger still holds"
            }
            ThrivingKind::FirstCompass => {
                "The air shifted — a cited wind"
            }
            ThrivingKind::FirstWell => {
                "You walked to the well — Mira stepped back"
            }
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct ThrivingMoments {
    pub fired: Vec<ThrivingKind>,
    pub queue: Vec<(ThrivingKind, f64)>,
    pub display_until: f64,
    pub current: Option<ThrivingKind>,
}

impl ThrivingMoments {
    pub fn try_fire(&mut self, kind: ThrivingKind, now: f64) {
        if self.fired.contains(&kind) {
            return;
        }
        self.fired.push(kind);
        self.queue.push((kind, now));
    }

    pub fn tick(&mut self, now: f64) {
        if self.current.is_some() && now < self.display_until {
            return;
        }
        self.current = None;
        if let Some((kind, _)) = self.queue.first().copied() {
            self.queue.remove(0);
            self.current = Some(kind);
            self.display_until = now + 5.0;
        }
    }
}

#[derive(Component)]
pub struct ThrivingToastRoot;

#[derive(Component)]
pub struct ThrivingToastText;

pub struct ThrivingMomentsPlugin;

impl Plugin for ThrivingMomentsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ThrivingMoments>()
            .add_systems(Startup, spawn_toast)
            .add_systems(
                Update,
                (tick_moments, update_toast_ui, soft_inventory_moment),
            );
    }
}

fn spawn_toast(mut commands: Commands) {
    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: THRIVING.top(),
                    left: THRIVING.left(),
                    width: Val::Px(620.0),
                    margin: THRIVING.margin(),
                    padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG.with_alpha(1.0)),
                BorderColor(TITLE_BORDER.with_alpha(1.0)),
                Visibility::Hidden,
            ),
            ThrivingToastRoot,
            HudSlab(THRIVING.id),
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(""),
TextFont { font_size: 15.0 / 1.2, ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                ThrivingToastText,
            ));
        });
}

fn tick_moments(time: Res<Time>, mut moments: ResMut<ThrivingMoments>) {
    moments.tick(time.elapsed_secs_f64());
}

fn update_toast_ui(
    moments: Res<ThrivingMoments>,
    mut root: Query<&mut Visibility, With<ThrivingToastRoot>>,
    mut text_q: Query<&mut Text, With<ThrivingToastText>>,
) {
    let show = moments.current.is_some();
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if let Some(kind) = moments.current {
        for mut text in &mut text_q {
            **text = kind.line().to_string();
        }
    }
}

fn soft_inventory_moment(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        moments.try_fire(ThrivingKind::FirstInventoryOpen, time.elapsed_secs_f64());
    }
}

/// Fire from practice loop / harvest paths.
pub fn fire_thriving(moments: &mut ThrivingMoments, kind: ThrivingKind, now: f64) {
    moments.try_fire(kind, now);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// CARD HUD-ANCHOR-REGISTRY-2B — Thriving position is the registry, Style is the coded literal.
    #[test]
    fn thriving_style_byte_identical_to_coded_place() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_toast);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<&Node, With<ThrivingToastRoot>>();
        let style = q.single(app.world()).unwrap().clone();
        let coded = Node {
            position_type: PositionType::Absolute,
            top: Val::Px(48.0),
            left: Val::Percent(50.0),
            width: Val::Px(620.0),
            margin: UiRect::left(Val::Px(-310.0)),
            padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };
        assert_eq!(style, coded);
        assert_eq!(style.top, THRIVING.top());
        assert_eq!(style.left, THRIVING.left());
        assert_eq!(style.margin, THRIVING.margin());
        assert_eq!(style.width, Val::Px(THRIVING.width));
    }

    /// CARD VP-SLABS-REGAL-2 — the thriving moments toast rests on the title palette: opaque
    /// TITLE_PLATE_BG plate, TITLE_BORDER rim at alpha 1, TITLE_TEXT_PRIMARY text.
    #[test]
    fn slabs_regal2_thriving_toast_rests_on_title_palette() {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .add_systems(Startup, spawn_toast);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<(&BorderColor, &BackgroundColor), With<ThrivingToastRoot>>();
        let (border, bg) = q.single(app.world()).unwrap();
        let (border, bg) = (border.0.to_srgba(), bg.0.to_srgba());
        let mut t = app.world_mut().query_filtered::<&TextColor, With<ThrivingToastText>>();
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

    #[test]
    fn fires_once() {
        let mut m = ThrivingMoments::default();
        m.try_fire(ThrivingKind::FirstMercyHarvest, 1.0);
        m.try_fire(ThrivingKind::FirstMercyHarvest, 2.0);
        assert_eq!(m.fired.len(), 1);
        assert_eq!(m.queue.len(), 1);
    }

    #[test]
    fn council_invite_drops_c_hint_and_locked_lines_hold() {
        let council = ThrivingKind::CouncilInvite.line();
        assert!(!council.contains("(C"), "{council}");

        let changed = [
            ThrivingKind::CouncilInvite.line(),
            ThrivingKind::FirstMercyHarvest.line(),
            ThrivingKind::FirstShare.line(),
            ThrivingKind::FirstWell.line(),
        ];
        for line in changed {
            assert!(!line.contains("XP"), "{line}");
            assert!(!line.chars().any(|c| c.is_ascii_digit()), "{line}");
        }

        assert_one_peak_phrase(ThrivingKind::FirstMercyHarvest.line(), "tended");
        assert_one_peak_phrase(ThrivingKind::FirstShare.line(), "yard remembered");
        assert_one_peak_phrase(ThrivingKind::FirstWell.line(), "walked");

        assert_eq!(
            ThrivingKind::FirstVoice.line(),
            "The card carried — the yard voted"
        );
        assert_eq!(
            ThrivingKind::FirstWarWeek.line(),
            "Hex gone green — tons plus restored"
        );
        assert_eq!(
            ThrivingKind::FirstCrownstone.line(),
            "The stone is seen — path waits"
        );
        assert_eq!(
            ThrivingKind::FirstRedemption.line(),
            "A tend offered — the grove answers"
        );
        assert_eq!(
            ThrivingKind::FirstHybrid.line(),
            "Double vision — the ledger still holds"
        );
    }

    fn assert_one_peak_phrase(line: &str, phrase: &str) {
        const PHRASES: [&str; 4] = [
            "walked",
            "tended",
            "week was the bill",
            "yard remembered",
        ];
        assert!(line.contains(phrase), "{line}");
        assert_eq!(line.matches(phrase).count(), 1, "{line}");
        for other in PHRASES {
            if other == phrase {
                continue;
            }
            assert!(!line.contains(other), "{line} also cites {other}");
        }
    }
}
