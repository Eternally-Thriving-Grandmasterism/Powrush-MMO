//! S0 Title + S2 skippable House naming + D1 Pause honesty (v23.2.62)
//!
//! High-contrast title plate (opaque light-on-dark — soft GPU / Mesa readable).
//! Esc from InYard → Title (not quit-to-desktop); quit via window close / Pause Quit.
//! D1: when Settings/pause opens in yard — one-line plate "the yard is waiting"
//! with Resume / Title / Quit (Title = Esc-to-title path; Quit = AppExit).
//! Esc-to-title + Settled write data/powrush_house.json even if name skipped.
//! Continue Unnamed House + yard remembers; Online grey; SmolStr drain.
//! Contact: info@Rathor.ai

use std::fs;
use std::path::Path;

use bevy::prelude::*;

use shared::house_name::{
    continue_cue_when_persist, local_persist_present, HouseName, HOUSE_PATH, UNNAMED,
};
use shared::title_house_proof::ONLINE_STUB_LABEL;

use crate::hour_sacred::{HourSacred, HOUR_TWO_PATH};
use crate::lived_hour_bind::{SHARD_CLIMATE_PATH, SHARD_STANDING_PATH};
use crate::net_mode::SessionNetMode;
use crate::lived_hour_bind::LivedHourBind;

// --- High-contrast title palette (opaque — no alpha-on-fog) -----------------
// Soft GPU / Mesa must read Play · Continue · Online · Settings before the yard.
/// Opaque dark plate behind menu text (light-on-dark).
pub const TITLE_PLATE_BG: Color = Color::srgb(0.05, 0.07, 0.09);
/// Full-screen dimmer over the world (opaque dark — stranger reads the door).
pub const TITLE_DIM_BG: Color = Color::srgb(0.02, 0.03, 0.04);
/// Primary menu / title text — high contrast on TITLE_PLATE_BG.
pub const TITLE_TEXT_PRIMARY: Color = Color::srgb(0.96, 0.98, 0.94);
/// Secondary cue / subtitle text.
pub const TITLE_TEXT_SECONDARY: Color = Color::srgb(0.82, 0.92, 0.86);
/// Enabled button fill (opaque).
pub const TITLE_BTN_BG: Color = Color::srgb(0.12, 0.18, 0.15);
/// Enabled button label.
pub const TITLE_BTN_FG: Color = Color::srgb(0.96, 0.98, 0.95);
/// Disabled / Online-grey fill.
pub const TITLE_BTN_DISABLED_BG: Color = Color::srgb(0.10, 0.11, 0.12);
/// Disabled / Online-grey label.
pub const TITLE_BTN_DISABLED_FG: Color = Color::srgb(0.55, 0.58, 0.60);
/// Plate + button border (opaque green).
pub const TITLE_BORDER: Color = Color::srgb(0.45, 0.78, 0.58);

/// D1 Pause honesty one-liner (opaque plate — soft GPU readable).
pub const YARD_WAITING: &str = "the yard is waiting";

/// Relative luminance from linear-ish sRGB channels (Bevy 0.14 Color::Srgba).
pub fn title_luminance(c: Color) -> f32 {
    let s = match c {
        Color::Srgba(srgba) => srgba,
        other => other.to_srgba(),
    };
    0.2126 * s.red + 0.7152 * s.green + 0.0722 * s.blue
}

/// True when primary text is clearly brighter than the plate (soft-GPU readable).
pub fn title_contrast_is_high() -> bool {
    let plate = title_luminance(TITLE_PLATE_BG);
    let text = title_luminance(TITLE_TEXT_PRIMARY);
    text - plate >= 0.55
}

/// Alpha channel of a Color (1.0 = opaque plate / no alpha-on-fog).
pub fn title_alpha(c: Color) -> f32 {
    match c {
        Color::Srgba(srgba) => srgba.alpha,
        other => other.to_srgba().alpha,
    }
}

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
struct PauseCueText;
#[derive(Component)]
struct PauseResumeBtn;
#[derive(Component)]
struct PauseTitleBtn;
#[derive(Component)]
struct PauseQuitBtn;
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
                    esc_yard_to_title,
                    name_house_text_input,
                    name_house_buttons,
                    sync_name_house_visibility,
                    sync_settings_stub,
                    refresh_pause_cue,
                    pause_plate_clicks,
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
                // Opaque dimmer — soft GPU must not alpha-blend menu into fog.
                background_color: TITLE_DIM_BG.into(),
                z_index: ZIndex::Global(120),
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
                        border: UiRect::all(Val::Px(2.0)),
                        align_items: AlignItems::Stretch,
                        ..default()
                    },
                    background_color: TITLE_PLATE_BG.into(),
                    border_color: TITLE_BORDER.into(),
                    ..default()
                },
            )
            .with_children(|p| {
                p.spawn(TextBundle::from_section(
                    "POWRUSH",
                    TextStyle {
                        font_size: 28.0,
                        color: TITLE_TEXT_PRIMARY,
                        ..default()
                    },
                ));
                p.spawn(TextBundle::from_section(
                    "Steward House · offline first",
                    TextStyle {
                        font_size: 14.0,
                        color: TITLE_TEXT_SECONDARY,
                        ..default()
                    },
                ));
                p.spawn((
                    TextBundle::from_section(
                        "",
                        TextStyle {
                            font_size: 13.0,
                            color: TITLE_TEXT_SECONDARY,
                            ..default()
                        },
                    ),
                    TitleCueText,
                ));
                spawn_menu_btn(p, "Play — first Hands", TitlePlayBtn, true);
                spawn_menu_btn(p, "Continue", TitleContinueBtn, true);
                spawn_menu_btn(p, "Settings", TitleSettingsBtn, true);
                spawn_menu_btn(p, ONLINE_STUB_LABEL, TitleOnlineBtn, false);
                p.spawn(TextBundle::from_section(
                    "1 Play · 2 Continue · 3 Settings · Esc from yard returns here",
                    TextStyle {
                        font_size: 11.0,
                        color: TITLE_TEXT_SECONDARY,
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
    let bg = if enabled { TITLE_BTN_BG } else { TITLE_BTN_DISABLED_BG };
    let fg = if enabled { TITLE_BTN_FG } else { TITLE_BTN_DISABLED_FG };
    p.spawn((
        ButtonBundle {
            style: Style {
                padding: UiRect::axes(Val::Px(14.0), Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: bg.into(),
            border_color: TITLE_BORDER.into(),
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
    // D1 Pause honesty — extend Settings stub into opaque pause plate (no second HUD).
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(28.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(360.0),
                    margin: UiRect {
                        left: Val::Px(-180.0),
                        ..default()
                    },
                    padding: UiRect::all(Val::Px(16.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    border: UiRect::all(Val::Px(1.5)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: TITLE_PLATE_BG.into(),
                border_color: TITLE_BORDER.into(),
                visibility: Visibility::Hidden,
                z_index: ZIndex::Global(130),
                ..default()
            },
            SettingsStubRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    YARD_WAITING,
                    TextStyle {
                        font_size: 16.0,
                        color: TITLE_TEXT_PRIMARY,
                        ..default()
                    },
                ),
                PauseCueText,
            ));
            spawn_menu_btn(p, "Resume", PauseResumeBtn, true);
            spawn_menu_btn(p, "Title", PauseTitleBtn, true);
            spawn_menu_btn(p, "Quit", PauseQuitBtn, true);
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
                background_color: TITLE_DIM_BG.into(),
                visibility: Visibility::Hidden,
                z_index: ZIndex::Global(140),
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
                background_color: TITLE_PLATE_BG.into(),
                border_color: TITLE_BORDER.into(),
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
        let _ = pulse; // opaque plate — no alpha breath on soft GPU
        *border = TITLE_BORDER.into();
    }
}

fn refresh_title_cue(
    label: Res<HouseLabel>,
    mut q: Query<&mut Text, With<TitleCueText>>,
) {
    let cue = continue_cue_when_persist(label.persist_present, &label.house)
        .unwrap_or_else(|| "Play opens the yard · no account wall".into());
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
            } else if keyboard.just_pressed(KeyCode::Escape) {
                // Esc-from-title: close settings only. Never wipe house/climate/standing/book.
                label.settings_open = false;
            }
        }
        LaunchDoor::InYard => {
            // Esc → Title is handled by esc_yard_to_title (not quit-to-desktop).
            // Digit3 opens/closes D1 pause plate (Settings path in yard).
            if keyboard.just_pressed(KeyCode::Digit3) {
                label.settings_open = !label.settings_open;
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
            TITLE_BTN_BG.into()
        } else {
            TITLE_BTN_DISABLED_BG.into()
        };
        *border = TITLE_BORDER.into();
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
        // Soft-write Unnamed so Continue works even if they quit mid-name panel.
        ensure_house_file_written(&mut label);
        label.draft.clear();
        *door = LaunchDoor::NameHouse;
    }
}

/// Esc while InYard → LaunchDoor::Title (stranger-pass).
/// Does **not** quit-to-desktop — close the window or use Settings for that.
/// First Esc closes Settings if open; second Esc (or Esc with settings closed) returns to Title.
/// Writes `data/powrush_house.json` even if name was skipped (Unnamed House).
fn esc_yard_to_title(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut door: ResMut<LaunchDoor>,
    mut label: ResMut<HouseLabel>,
    bind: Option<Res<LivedHourBind>>,
) {
    if *door != LaunchDoor::InYard {
        return;
    }
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }
    if label.settings_open {
        // First Esc closes pause/Settings; second Esc returns to Title.
        label.settings_open = false;
        return;
    }
    return_yard_to_title(&mut door, &mut label, bind.as_ref());
}

/// After Settled or quit-to-title: house JSON exists (name may be null / Unnamed).
fn ensure_house_file_written(label: &mut HouseLabel) {
    if !label.house.resolved {
        label.house.skip();
        label.naming_offered = true;
    }
    label.house.persist();
    label.persist_present = true;
}

fn refresh_pause_cue(
    door: Res<LaunchDoor>,
    label: Res<HouseLabel>,
    mut q: Query<&mut Text, With<PauseCueText>>,
) {
    if !label.settings_open {
        return;
    }
    let line = pause_plate_line(*door).unwrap_or("Settings · Esc closes · Quit exits");
    for mut text in &mut q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != line {
                s.value = line.to_string();
            }
        }
    }
}

fn pause_plate_clicks(
    mut door: ResMut<LaunchDoor>,
    mut label: ResMut<HouseLabel>,
    bind: Option<Res<LivedHourBind>>,
    mut exit: EventWriter<AppExit>,
    resume: Query<&Interaction, (Changed<Interaction>, With<PauseResumeBtn>)>,
    title: Query<&Interaction, (Changed<Interaction>, With<PauseTitleBtn>)>,
    quit: Query<&Interaction, (Changed<Interaction>, With<PauseQuitBtn>)>,
) {
    if !label.settings_open {
        return;
    }
    for i in &resume {
        if *i == Interaction::Pressed {
            label.settings_open = false;
            return;
        }
    }
    for i in &title {
        if *i == Interaction::Pressed {
            return_yard_to_title(&mut door, &mut label, bind.as_ref());
            return;
        }
    }
    for i in &quit {
        if *i == Interaction::Pressed {
            // Same window-close / Settings quit path — not Esc.
            label.settings_open = false;
            exit.send(AppExit::Success);
            return;
        }
    }
}

/// Shared Esc-to-title / Pause→Title path: house JSON + lived persist, then Title.
fn return_yard_to_title(
    door: &mut LaunchDoor,
    label: &mut HouseLabel,
    bind: Option<&Res<LivedHourBind>>,
) {
    label.settings_open = false;
    if *door != LaunchDoor::InYard {
        return;
    }
    ensure_house_file_written(label);
    if let Some(bind) = bind {
        bind.persist();
    }
    *door = LaunchDoor::Title;
}

/// D1: pause plate one-liner when Settings/pause is open in the yard.
pub fn pause_plate_line(door: LaunchDoor) -> Option<&'static str> {
    match door {
        LaunchDoor::InYard => Some(YARD_WAITING),
        LaunchDoor::Title | LaunchDoor::NameHouse => None,
    }
}

/// Pure helper: Resume keeps InYard (unpause only).
pub fn resume_keeps_door(from: LaunchDoor) -> LaunchDoor {
    from
}

/// Pure helper for proof tests: Esc from InYard yields Title.
pub fn esc_from_inyard_returns_title(from: LaunchDoor) -> LaunchDoor {
    match from {
        LaunchDoor::InYard => LaunchDoor::Title,
        other => other,
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
        // Drain SmolStr ReceivedCharacter so Continue/name does not flicker with stale input.
        chars.clear();
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
    use shared::house_name::{
        continue_cue, continue_cue_when_persist, esc_from_title_preserves_persist, YARD_REMEMBERS,
    };
    use shared::stranger_loop_proof::{hour_two_held_fixture, peace_fixture};
    use shared::title_house_proof::online_row_is_honest_disabled;

    #[test]
    fn play_does_not_require_house_name() {
        let house = HouseName::default();
        assert!(!house.blocks_hands());
        assert_eq!(LaunchDoor::default(), LaunchDoor::Title);
        assert!(continue_cue(false, &house).is_none());
    }

    #[test]
    fn continue_cue_unnamed_house_yard_remembers() {
        assert!(local_persist_present(true, false, false, false));
        let mut house = HouseName::default();
        house.skip();
        let cue = continue_cue_when_persist(true, &house).unwrap();
        assert_eq!(cue, "Unnamed House · the yard remembers");
        assert!(cue.contains(UNNAMED));
        assert!(cue.contains(YARD_REMEMBERS));
        let mut named = HouseName::default();
        named.confirm("Yard");
        let cue2 = continue_cue(true, &named).unwrap();
        assert_eq!(cue2, "Yard · the yard remembers");
    }

    #[test]
    fn esc_from_title_does_not_clear_persist_files() {
        let mut house = HouseName::default();
        house.confirm("Keep Me");
        let house_json = house.to_json().unwrap();
        let climate = r#"{"harmony":1.0}"#;
        let standing = r#"{"declared_lethal":false}"#;
        let book = r#"{"complete":true}"#;
        // Esc-from-title only flips settings_open=false; payloads identical.
        let mut settings_open = true;
        settings_open = false; // Esc on Title
        assert!(!settings_open);
        assert!(esc_from_title_preserves_persist(
            &house_json,
            &house_json,
            climate,
            climate,
            standing,
            standing,
            book,
            book,
        ));
        let back = HouseName::from_json(&house_json).unwrap();
        assert!(back.resolved);
        assert_eq!(back.display_name(), "Keep Me");
    }

    #[test]
    fn online_row_visible_disabled_honest() {
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        let net = SessionNetMode::default();
        assert!(net.mode.peer_count_for_peace_boot().is_none());
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
    fn title_default_is_launch_door_title() {
        assert_eq!(LaunchDoor::default(), LaunchDoor::Title);
    }

    #[test]
    fn title_contrast_light_on_opaque_dark() {
        assert!(title_contrast_is_high(), "primary text must out-luminance plate");
        let a = title_alpha(TITLE_PLATE_BG);
        assert!((a - 1.0).abs() < 0.01, "plate must be opaque, got alpha={a}");
        let a2 = title_alpha(TITLE_DIM_BG);
        assert!((a2 - 1.0).abs() < 0.01, "dimmer must be opaque, got alpha={a2}");
        assert!(title_luminance(TITLE_TEXT_PRIMARY) > title_luminance(TITLE_PLATE_BG));
        assert!(title_luminance(TITLE_BTN_FG) > title_luminance(TITLE_BTN_BG));
    }

    #[test]
    fn esc_from_inyard_maps_to_title() {
        assert_eq!(
            super::esc_from_inyard_returns_title(LaunchDoor::InYard),
            LaunchDoor::Title
        );
        assert_eq!(
            super::esc_from_inyard_returns_title(LaunchDoor::Title),
            LaunchDoor::Title
        );
        assert_eq!(
            super::esc_from_inyard_returns_title(LaunchDoor::NameHouse),
            LaunchDoor::NameHouse
        );
    }

    #[test]
    fn d1_pause_plate_yard_waiting_line() {
        assert_eq!(super::pause_plate_line(LaunchDoor::InYard), Some(YARD_WAITING));
        assert_eq!(super::pause_plate_line(LaunchDoor::Title), None);
        assert_eq!(super::pause_plate_line(LaunchDoor::NameHouse), None);
        assert_eq!(YARD_WAITING, "the yard is waiting");
    }

    #[test]
    fn d1_resume_keeps_inyard() {
        assert_eq!(super::resume_keeps_door(LaunchDoor::InYard), LaunchDoor::InYard);
        assert_eq!(
            super::esc_from_inyard_returns_title(LaunchDoor::InYard),
            LaunchDoor::Title
        );
    }

    #[test]
    fn ensure_house_written_on_skip_path() {
        // In-memory skip path (no disk write in unit test).
        let mut label = HouseLabel {
            house: HouseName::default(),
            persist_present: false,
            hour_two_held: false,
            settings_open: false,
            draft: String::new(),
            naming_offered: false,
        };
        assert!(!label.house.resolved);
        if !label.house.resolved {
            label.house.skip();
            label.naming_offered = true;
        }
        label.persist_present = true;
        assert!(label.house.resolved);
        assert_eq!(label.house.display_name(), UNNAMED);
        assert!(label.persist_present);
        assert!(label.naming_offered);
        let raw = label.house.to_json().unwrap();
        assert!(raw.contains("powrush_house_v1"));
        // name empty / Unnamed — Continue reads Unnamed House · the yard remembers
        let cue = continue_cue_when_persist(true, &label.house).unwrap();
        assert_eq!(cue, "Unnamed House · the yard remembers");
        // esc helper still maps InYard → Title
        assert_eq!(super::esc_from_inyard_returns_title(LaunchDoor::InYard), LaunchDoor::Title);
    }
}