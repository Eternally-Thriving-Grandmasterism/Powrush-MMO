/*!
 * Human Soft Panels — M Mercy Journey · Z Realm Travel (v21.99.3)
 *
 * M / Z taught. F2 / F3 still heard as aliases.
 *
 * PATSAGi + TOLC 8 | Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::abundance_journey_echo::AbundanceJourneyEcho;
use crate::living_practice_loop::SoftPlayerRealm;
use crate::soft_play_bindings;

#[derive(Resource, Debug, Default)]
pub struct HumanSoftPanels {
    pub mercy_open: bool,
    pub realm_open: bool,
}

#[derive(Component)]
struct MercySoftRoot;
#[derive(Component)]
struct MercySoftBody;
#[derive(Component)]
struct RealmSoftRoot;
#[derive(Component)]
struct RealmSoftBody;

const REALMS: [(u8, &str); 5] = [
    (0, "Sanctuary Prime"),
    (1, "Synthetic Lattice"),
    (2, "Verdant Bloom"),
    (3, "Harmonic Chorus"),
    (4, "Voidfarer Horizon"),
];

pub struct HumanSoftPanelsPlugin;

impl Plugin for HumanSoftPanelsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HumanSoftPanels>()
            .add_systems(Startup, spawn_soft_panels)
            .add_systems(
                Update,
                (
                    toggle_soft_panels,
                    digit_realm_travel,
                    update_soft_visibility,
                    update_soft_bodies,
                ),
            );
    }
}

fn spawn_soft_panels(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(10.0),
                    right: Val::Percent(2.0),
                    width: Val::Px(360.0),
                    max_height: Val::Px(320.0),
                    padding: UiRect::all(Val::Px(14.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    border: UiRect::all(Val::Px(1.5)),
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                background_color: Color::srgba(0.04, 0.07, 0.10, 0.94).into(),
                border_color: Color::srgba(0.55, 0.90, 0.80, 0.50).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            MercySoftRoot,
        ))
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                "MY MERCY JOURNEY",
                TextStyle {
                    font_size: 15.0,
                    color: Color::srgb(0.75, 0.98, 0.88),
                    ..default()
                },
            ));
            p.spawn((
                TextBundle::from_section(
                    "Acts of thriving will gather here",
                    TextStyle {
                        font_size: 13.0,
                        color: Color::srgb(0.90, 0.94, 1.0),
                        ..default()
                    },
                ),
                MercySoftBody,
            ));
            p.spawn(TextBundle::from_section(
                "M toggle · J also opens the echo",
                TextStyle {
                    font_size: 11.0,
                    color: Color::srgb(0.55, 0.70, 0.75),
                    ..default()
                },
            ));
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(18.0),
                    left: Val::Percent(2.0),
                    width: Val::Px(300.0),
                    padding: UiRect::all(Val::Px(14.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    border: UiRect::all(Val::Px(1.5)),
                    ..default()
                },
                background_color: Color::srgba(0.05, 0.07, 0.12, 0.94).into(),
                border_color: Color::srgba(0.50, 0.75, 0.95, 0.50).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            RealmSoftRoot,
        ))
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                "REALM TRAVEL",
                TextStyle {
                    font_size: 15.0,
                    color: Color::srgb(0.78, 0.94, 1.0),
                    ..default()
                },
            ));
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 13.0,
                        color: Color::srgb(0.90, 0.95, 1.0),
                        ..default()
                    },
                ),
                RealmSoftBody,
            ));
            p.spawn(TextBundle::from_section(
                "Z toggle · 1–5 choose climate",
                TextStyle {
                    font_size: 11.0,
                    color: Color::srgb(0.55, 0.68, 0.80),
                    ..default()
                },
            ));
        });
}

fn toggle_soft_panels(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut panels: ResMut<HumanSoftPanels>,
) {
    if soft_play_bindings::mercy_journey_just_pressed(&keyboard) {
        panels.mercy_open = !panels.mercy_open;
    }
    if soft_play_bindings::realm_travel_just_pressed(&keyboard) {
        panels.realm_open = !panels.realm_open;
    }
}

fn digit_realm_travel(
    keyboard: Res<ButtonInput<KeyCode>>,
    panels: Res<HumanSoftPanels>,
    mut soft_realm: ResMut<SoftPlayerRealm>,
    mut echo: ResMut<AbundanceJourneyEcho>,
) {
    if !panels.realm_open {
        return;
    }
    let pick = if keyboard.just_pressed(KeyCode::Digit1) || keyboard.just_pressed(KeyCode::Digit1) {
        Some(0u8)
    } else if keyboard.just_pressed(KeyCode::Digit2) || keyboard.just_pressed(KeyCode::Digit2) {
        Some(1)
    } else if keyboard.just_pressed(KeyCode::Digit3) || keyboard.just_pressed(KeyCode::Digit3) {
        Some(2)
    } else if keyboard.just_pressed(KeyCode::Digit4) || keyboard.just_pressed(KeyCode::Digit4) {
        Some(3)
    } else if keyboard.just_pressed(KeyCode::Digit5) || keyboard.just_pressed(KeyCode::Digit5) {
        Some(4)
    } else {
        None
    };
    let Some(id) = pick else {
        return;
    };
    if soft_realm.current == Some(id) {
        return;
    }
    soft_realm.current = Some(id);
    let name = REALMS[id as usize].1;
    echo.push(
        crate::abundance_journey_echo::JourneyKind::Note,
        format!("Traveled to {name}"),
    );
    info!(target: "powrush::realm", id, name, "soft realm travel");
}

fn update_soft_visibility(
    panels: Res<HumanSoftPanels>,
    mut mercy: Query<&mut Visibility, (With<MercySoftRoot>, Without<RealmSoftRoot>)>,
    mut realm: Query<&mut Visibility, (With<RealmSoftRoot>, Without<MercySoftRoot>)>,
) {
    for mut vis in &mut mercy {
        *vis = if panels.mercy_open {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    for mut vis in &mut realm {
        *vis = if panels.realm_open {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn update_soft_bodies(
    echo: Res<AbundanceJourneyEcho>,
    soft_realm: Res<SoftPlayerRealm>,
    mut mercy: Query<&mut Text, (With<MercySoftBody>, Without<RealmSoftBody>)>,
    mut realm: Query<&mut Text, (With<RealmSoftBody>, Without<MercySoftBody>)>,
) {
    let mercy_body = if echo.lines.is_empty() {
        "Acts of thriving will gather here".to_string()
    } else {
        echo.lines
            .iter()
            .rev()
            .take(8)
            .map(|l| format!("· {}", l.text))
            .collect::<Vec<_>>()
            .join("\n")
    };
    for mut text in &mut mercy {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != mercy_body {
                s.value = mercy_body.clone();
            }
        }
    }

    let current = soft_realm.current.unwrap_or(0);
    let mut realm_body = String::from("1–5 choose a climate\n");
    for (id, name) in REALMS {
        let mark = if id == current { ">" } else { " " };
        realm_body.push_str(&format!("{mark} [{id}] {name}\n"));
    }
    for mut text in &mut realm {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != realm_body {
                s.value = realm_body.clone();
            }
        }
    }
}
