//! S0 Title + S2 skippable House naming (v23.2.50)
//!
//! Play / Continue / Settings. Online grey stub. No peer count. No login wall.
//! House name persist: data/powrush_house.json (beside climate).
//! Contact: info@Rathor.ai

use std::fs;
use std::path::Path;

use bevy::prelude::*;

use shared::house_name::{
    continue_cue, local_persist_present, HouseName, HOUSE_PATH, UNNAMED,
};

use crate::hour_sacred::{HourSacred, HOUR_TWO_PATH};
use crate::lived_hour_bind::{SHARD_CLIMATE_PATH, SHARD_STANDING_PATH};
use crate::net_mode::SessionNetMode;

/// Boot door. Title until Play/Continue. Naming is optional after Settled / quit.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchDoor {
    Title,
    InYard,
    NameHouse,
}

impl Default for LaunchDoor {
    fn default() -> Self {
        Self::Title
    }
}

#[derive(Resource, Debug, Clone)]
pub struct HouseLabel {
    pub house: HouseName,
    pub persist_present: bool,
    pub hour_two_held: bool,
    pub settings_open: bool,
    pub draft: String,
    /// Offer naming once after Settled / Escape.
    pub naming_offered: bool,
}

impl Default for HouseLabel {
    fn default() -> Self {
        let house = HouseName::load_or_default();
        let hour_two_held = Path::new(HOUR_TWO_PATH).exists()
            && fs::read_to_string(HOUR_TWO_PATH)
                .ok()
                .and_then(|r| serde_json::from_str::<serde_json::Value>(&r).ok())
                .and_then(|v| v.get("complete").and_then(|c| c.as_bool()))
                .unwrap_or(false);
        let persist_present = local_persist_present(
            Path::new(HOUR_TWO_PATH).exists(),
            Path::new(SHARD_CLIMATE_PATH).exists(),
            Path::new(SHARD_STANDING_PATH).exists(),
            house.resolved,
        );
        Self {
            house,
            persist_present,
            hour_two_held,
            settings_open: false,
            draft: String::new(),
            naming_offered: false,
        }
    }
}

#[derive(Component)]
struct TitleRoot;
#[derive(Component)]
struct TitleCueText;
#[derive(Component)]
struct TitleBreath;
#[derive(Component)]
struct TitlePlayBtn;
#[derive(Component)]
struct TitleContinueBtn;
#[derive(Component)]
struct TitleSettingsBtn;
#[derive(Component)]
#[allow(dead_code)]
struct TitleOnlineBtn;
#[derive(Component)]
struct SettingsStubRoot;
#[derive(Component)]
struct NameHouseRoot;
#[derive(Component)]
struct NameDraftText;
#[derive(Component)]
struct NameConfirmBtn;
#[derive(Component)]
struct NameSkipBtn;

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LaunchDoor>()
            .init_resource::<HouseLabel>()
            .add_systems(Startup, (spawn_title_screen, spawn_name_house_panel, spawn_settings_stub))
            .add_systems(
                Update,
                (
                    breath_title_border,
                    refresh_title_cue,
                    title_button_clicks,
                    title_keyboard_shortcuts,
                    sync_title_visibility,
                    watch_settled_for_naming,
                    quit_path_naming,
                    name_house_text_input,
                    name_house_buttons,
                    sync_name_house_visibility,
                    sync_settings_stub,
                ),
            );
    }
}

fn spawn_title_screen(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(14.0),
                    ..default()
                },
                background_color: Color::srgba(0.03, 0.05, 0.06, 0.82).into(),
                z_index: ZIndex::Global(40),
                ..default()
            },
            TitleRoot,
            TitleBreath,
        ))
        .with_children(|root| {
            root.spawn(
                NodeBundle {
                    style: Style {
                        width: Val::Px(420.0),
                        padding: UiRect::all(Val::Px(22.0)),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.0),
                        border: UiRect::all(Val::Px(1.5)),
                        align_items: AlignItems::Stretch,
                        ..default()
                    },
                    background_color: Color::srgba(0.05, 0.08, 0.09, 0.94).into(),
                    border_color: Color::srgba(0.48, 0.78, 0.58, 0.55).into(),
                    ..default()
                },
            )
            .with_children(|p| {
                p.spawn(TextBundle::from_section(
                    "POWRUSH",
                    TextStyle {
                        font_size: 28.0,
                        color: Color::srgb(0.88, 0.98, 0.92),
                        ..default()
                    },
                ));
                p.spawn(TextBundle::from_section(
                    "Steward House · offline first",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.62, 0.82, 0.72),
                        ..default()
                    },
                ));
                p.spawn((
                    TextBundle::from_section(
                        "",
                        TextStyle {
                            font_size: 13.0,
                            color: Color::srgb(0.78, 0.92, 0.84),
                            ..default()
                        },
                    ),
                    TitleCueText,
                ));
                spawn_menu_btn(p, "Play — first Hands", TitlePlayBtn, true);
                spawn_menu_btn(p, "Continue", TitleContinueBtn, true);
                spawn_menu_btn(p, "Settings", TitleSettingsBtn, true);
                spawn_menu_btn(p, "Online — not yet", TitleOnlineBtn, false);
                p.spawn(TextBundle::from_section(
                    "1 Play · 2 Continue · 3 Settings · no peer count",
                    TextStyle {
                        font_size: 11.0,
                        color: Color::srgb(0.50, 0.62, 0.58),
                        ..default()
                    },
                ));
            });
        });
}

fn spawn_menu_btn<C: Component>(
    p: &mut ChildBuilder,
    label: &str,
    marker: C,
    enabled: bool,
) {
    let bg = if enabled {
        Color::srgba(0.10, 0.16, 0.14, 0.95)
    } else {
        Color::srgba(0.08, 0.09, 0.10, 0.70)
    };
    let fg = if enabled {
        Color::srgb(0.90, 0.96, 0.92)
    } else {
        Color::srgb(0.45, 0.48, 0.50)
    };
    p.spawn((
        ButtonBundle {
            style: Style {
                padding: UiRect::axes(Val::Px(14.0), Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: bg.into(),
            border_color: Color::srgba(0.40, 0.70, 0.55, if enabled { 0.45 } else { 0.20 }).into(),
            ..default()
        },
        marker,
    ))
    .with_children(|b| {
        b.spawn(TextBundle::from_section(
            label,
            TextStyle {
                font_size: 15.0,
                color: fg,
                ..default()
            },
        ));
    });
}

fn spawn_settings_stub(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(18.0),
                    right: Val::Percent(4.0),
                    width: Val::Px(300.0),
                    padding: UiRect::all(Val::Px(14.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.05, 0.07, 0.10, 0.94).into(),
                border_color: Color::srgba(0.50, 0.75, 0.95, 0.45).into(),
                visibility: Visibility::Hidden,
                z_index: ZIndex::Global(45),
                ..default()
            },
            SettingsStubRoot,
        ))
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                "Settings",
                TextStyle {
                    font_size: 15.0,
                    color: Color::srgb(0.78, 0.94, 1.0),
                    ..default()
                },
            ));
            p.spawn(TextBundle::from_section(
                "Peace keys · WASD E I H R unchanged\nLethal stays opt-in after the book\n3 / Esc closes",
                TextStyle {
                    font_size: 13.0,
                    color: Color::srgb(0.86, 0.92, 0.96),
                    ..default()
                },
            ));
        });
}

fn spawn_name_house_panel(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgba(0.02, 0.04, 0.05, 0.72).into(),
                visibility: Visibility::Hidden,
                z_index: ZIndex::Global(50),
                ..default()
            },
            NameHouseRoot,
        ))
        .with_children(|root| {
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(400.0),
                    padding: UiRect::all(Val::Px(18.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    border: UiRect::all(Val::Px(1.5)),
                    ..default()
                },
                background_color: Color::srgba(0.05, 0.08, 0.09, 0.96).into(),
                border_color: Color::srgba(0.55, 0.88, 0.70, 0.50).into(),
                ..default()
            })
            .with_children(|p| {
                p.spawn(TextBundle::from_section(
                    "Name your House",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::srgb(0.88, 0.98, 0.92),
                        ..default()
                    },
                ));
                p.spawn(TextBundle::from_section(
                    "Optional. Skip keeps Unnamed House.",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::srgb(0.60, 0.78, 0.70),
                        ..default()
                    },
                ));
                p.spawn((
                    TextBundle::from_section(
                        "_",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::srgb(0.92, 0.98, 0.94),
                            ..default()
                        },
                    ),
                    NameDraftText,
                ));
                spawn_menu_btn(p, "Confirm", NameConfirmBtn, true);
                spawn_menu_btn(p, "Skip", NameSkipBtn, true);
            });
        });
}

fn breath_title_border(
    time: Res<Time>,
    door: Res<LaunchDoor>,
    mut q: Query<&mut BorderColor, With<TitleBreath>>,
) {
    if *door != LaunchDoor::Title {
        return;
    }
    let pulse = 0.45 + (time.elapsed_seconds() * 1.2).sin() * 0.12;
    for mut border in &mut q {
        // TitleBreath is on the full-screen root (no border) — keep noop-safe.
        *border = Color::srgba(0.48, 0.78, 0.58, pulse).into();
    }
}

fn refresh_title_cue(
    label: Res<HouseLabel>,
    mut q: Query<&mut Text, With<TitleCueText>>,
) {
    let cue = if label.persist_present {
        continue_cue(label.hour_two_held, &label.house)
            .unwrap_or_else(|| label.house.display_name().to_string())
    } else {
        "Play opens the yard · no account wall".into()
    };
    for mut text in &mut q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != cue {
                s.value = cue.clone();
            }
        }
    }
}

fn enter_yard(door: &mut LaunchDoor, label: &mut HouseLabel) {
    *door = LaunchDoor::InYard;
    label.settings_open = false;
}

fn title_button_clicks(
    mut door: ResMut<LaunchDoor>,
    mut label: ResMut<HouseLabel>,
    play: Query<&Interaction, (Changed<Interaction>, With<TitlePlayBtn>)>,
    cont: Query<&Interaction, (Changed<Interaction>, With<TitleContinueBtn>)>,
    settings: Query<&Interaction, (Changed<Interaction>, With<TitleSettingsBtn>)>,
) {
    if *door != LaunchDoor::Title {
        return;
    }
    for i in &play {
        if *i == Interaction::Pressed {
            enter_yard(&mut door, &mut label);
            return;
        }
    }
    for i in &cont {
        if *i == Interaction::Pressed {
            if label.persist_present {
                enter_yard(&mut door, &mut label);
            }
            return;
        }
    }
    for i in &settings {
        if *i == Interaction::Pressed {
            label.settings_open = !label.settings_open;
            return;
        }
    }
}

fn title_keyboard_shortcuts(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut door: ResMut<LaunchDoor>,
    mut label: ResMut<HouseLabel>,
) {
    match *door {
        LaunchDoor::Title => {
            if keyboard.just_pressed(KeyCode::Digit1) || keyboard.just_pressed(KeyCode::Enter) {
                enter_yard(&mut door, &mut label);
            } else if keyboard.just_pressed(KeyCode::Digit2) {
                if label.persist_present {
                    enter_yard(&mut door, &mut label);
                }
            } else if keyboard.just_pressed(KeyCode::Digit3) {
                label.settings_open = !label.settings_open;
            }
        }
        LaunchDoor::InYard => {
            if keyboard.just_pressed(KeyCode::Digit3) && label.settings_open {
                label.settings_open = false;
            }
        }
        LaunchDoor::NameHouse => {}
    }
}

fn sync_title_visibility(
    door: Res<LaunchDoor>,
    label: Res<HouseLabel>,
    mut root: Query<&mut Visibility, With<TitleRoot>>,
    mut cont_style: Query<(&mut BackgroundColor, &mut BorderColor), With<TitleContinueBtn>>,
    net: Res<SessionNetMode>,
) {
    let show = *door == LaunchDoor::Title;
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    // Continue dims when no persist
    let enabled = label.persist_present;
    for (mut bg, mut border) in &mut cont_style {
        *bg = if enabled {
            Color::srgba(0.10, 0.16, 0.14, 0.95).into()
        } else {
            Color::srgba(0.08, 0.09, 0.10, 0.55).into()
        };
        *border = Color::srgba(0.40, 0.70, 0.55, if enabled { 0.45 } else { 0.15 }).into();
    }
    let _ = net.mode; // Online stays Offline; peer count never shown
}

fn sync_settings_stub(
    label: Res<HouseLabel>,
    door: Res<LaunchDoor>,
    mut q: Query<&mut Visibility, With<SettingsStubRoot>>,
) {
    let show = label.settings_open && *door != LaunchDoor::NameHouse;
    for mut vis in &mut q {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn watch_settled_for_naming(
    hour: Option<Res<HourSacred>>,
    mut door: ResMut<LaunchDoor>,
    mut label: ResMut<HouseLabel>,
) {
    if *door != LaunchDoor::InYard {
        return;
    }
    if label.house.resolved || label.naming_offered {
        return;
    }
    let Some(hour) = hour else {
        return;
    };
    if hour.complete {
        label.naming_offered = true;
        label.draft.clear();
        *door = LaunchDoor::NameHouse;
    }
}

fn quit_path_naming(
    keyboard: Res<ButtonInput<KeyCode>>,
    hour: Option<Res<HourSacred>>,
    mut door: ResMut<LaunchDoor>,
    mut label: ResMut<HouseLabel>,
) {
    if *door != LaunchDoor::InYard {
        return;
    }
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }
    if label.house.resolved || label.naming_offered {
        return;
    }
    let settled = hour.map(|h| h.complete).unwrap_or(false);
    if settled {
        label.naming_offered = true;
        label.draft.clear();
        *door = LaunchDoor::NameHouse;
    }
}

fn name_house_text_input(
    door: Res<LaunchDoor>,
    mut label: ResMut<HouseLabel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut chars: EventReader<ReceivedCharacter>,
    mut draft_q: Query<&mut Text, With<NameDraftText>>,
) {
    if *door != LaunchDoor::NameHouse {
        return;
    }
    // Bevy 0.14: ReceivedCharacter.char is SmolStr (deprecated API; still compiles).
    for ev in chars.read() {
        for c in ev.char.chars() {
            if c.is_control() {
                continue;
            }
            if (c.is_alphanumeric() || c == ' ' || c == '-' || c == '\'') && label.draft.len() < 32 {
                label.draft.push(c);
            }
        }
    }
    if keyboard.just_pressed(KeyCode::Backspace) {
        label.draft.pop();
    }
    let shown = if label.draft.is_empty() {
        format!("_  ({UNNAMED})")
    } else {
        label.draft.clone()
    };
    for mut text in &mut draft_q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != shown {
                s.value = shown.clone();
            }
        }
    }
}

fn name_house_buttons(
    mut door: ResMut<LaunchDoor>,
    mut label: ResMut<HouseLabel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    confirm: Query<&Interaction, (Changed<Interaction>, With<NameConfirmBtn>)>,
    skip: Query<&Interaction, (Changed<Interaction>, With<NameSkipBtn>)>,
) {
    if *door != LaunchDoor::NameHouse {
        return;
    }
    let mut do_confirm = keyboard.just_pressed(KeyCode::Enter);
    let mut do_skip = keyboard.just_pressed(KeyCode::Escape);
    for i in &confirm {
        if *i == Interaction::Pressed {
            do_confirm = true;
        }
    }
    for i in &skip {
        if *i == Interaction::Pressed {
            do_skip = true;
        }
    }
    if do_confirm {
        let draft = label.draft.clone();
        label.house.confirm(&draft);
        label.house.persist();
        label.persist_present = true;
        *door = LaunchDoor::InYard;
    } else if do_skip {
        label.house.skip();
        label.house.persist();
        label.persist_present = true;
        *door = LaunchDoor::InYard;
    }
}

fn sync_name_house_visibility(
    door: Res<LaunchDoor>,
    mut q: Query<&mut Visibility, With<NameHouseRoot>>,
) {
    let show = *door == LaunchDoor::NameHouse;
    for mut vis in &mut q {
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
    use shared::stranger_loop_proof::{hour_two_held_fixture, peace_fixture};

    #[test]
    fn play_does_not_require_house_name() {
        let house = HouseName::default();
        assert!(!house.blocks_hands());
        assert_eq!(LaunchDoor::default(), LaunchDoor::Title);
    }

    #[test]
    fn continue_meaningful_when_persist() {
        assert!(local_persist_present(true, false, false, false));
        let mut house = HouseName::default();
        house.confirm("Yard");
        let cue = continue_cue(true, &house).unwrap();
        assert!(cue.contains("Yard"));
        assert!(cue.contains("yard remembers"));
    }

    #[test]
    fn house_path_constant() {
        assert_eq!(HOUSE_PATH, "data/powrush_house.json");
    }

    #[test]
    fn peace_fixture_lethal_untouched() {
        let (_p, _c, standing, _) = peace_fixture();
        assert!(!standing.declared_lethal);
        let h2 = hour_two_held_fixture();
        assert!(h2.complete);
    }

    #[test]
    fn online_stub_stays_offline_label() {
        // Title Online button is visual-only; SessionNetMode boots Offline.
        let net = SessionNetMode::default();
        assert!(net.mode.peer_count_for_peace_boot().is_none());
    }
}
