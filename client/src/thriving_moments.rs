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
}

impl ThrivingKind {
    pub fn line(&self) -> &'static str {
        match self {
            ThrivingKind::FirstMercyHarvest => {
                "Thriving moment · A harvest taken with restraint — the node still glows"
            }
            ThrivingKind::SurfaceCleared => {
                "Thriving moment · One climate practiced — the principle travels with you"
            }
            ThrivingKind::PrincipleSealed => {
                "Thriving moment · Caps Across Climates sealed — same truth, three faces"
            }
            ThrivingKind::CouncilInvite => {
                "Invitation · A soft Council seat is open when you are ready (C to note)"
            }
            ThrivingKind::FirstInventoryOpen => {
                "Thriving moment · Inventory open — abundance is held, not hoarded"
            }
            ThrivingKind::FirstShare => {
                "Thriving moment · Surplus shared — others may thrive now"
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
                (
                    tick_moments,
                    update_toast_ui,
                    soft_inventory_moment,
                    soft_council_note,
                ),
            );
    }
}

fn spawn_toast(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
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
                },
                background_color: Color::srgba(0.08, 0.06, 0.12, 0.92).into(),
                border_color: Color::srgba(0.95, 0.82, 0.45, 0.55).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            ThrivingToastRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 15.0,
                        color: Color::srgb(1.0, 0.94, 0.78),
                        ..default()
                    },
                ),
                ThrivingToastText,
            ));
        });
}

fn tick_moments(time: Res<Time>, mut moments: ResMut<ThrivingMoments>) {
    moments.tick(time.elapsed_seconds_f64());
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
            if let Some(s) = text.sections.get_mut(0) {
                s.value = kind.line().to_string();
            }
        }
    }
}

fn soft_inventory_moment(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        moments.try_fire(ThrivingKind::FirstInventoryOpen, time.elapsed_seconds_f64());
    }
}

fn soft_council_note(
    keyboard: Res<ButtonInput<KeyCode>>,
    moments: Res<ThrivingMoments>,
) {
    // C acknowledges the invite — soft log only (no forced UI jump)
    if keyboard.just_pressed(KeyCode::KeyC) && moments.fired.contains(&ThrivingKind::CouncilInvite)
    {
        info!(target: "powrush::thrive", "Player noted Council invitation — voluntary");
    }
}

/// Fire from practice loop / harvest paths.
pub fn fire_thriving(moments: &mut ThrivingMoments, kind: ThrivingKind, now: f64) {
    moments.try_fire(kind, now);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fires_once() {
        let mut m = ThrivingMoments::default();
        m.try_fire(ThrivingKind::FirstMercyHarvest, 1.0);
        m.try_fire(ThrivingKind::FirstMercyHarvest, 2.0);
        assert_eq!(m.fired.len(), 1);
        assert_eq!(m.queue.len(), 1);
    }
}
