//! First-launch 18+ gate. Month and year stay on this screen and are never stored.
//! Mouse, keyboard, touch, and gamepad (D-pad picks and sets, A continues). Contact: info@Rathor.ai
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use crate::local_settings::LocalSettingsState;
use crate::title_screen::{TITLE_BTN_ACTIVE_BG, TITLE_BTN_BG, TITLE_BTN_DISABLED_BG, TITLE_BTN_FG, TITLE_DIM_BG, TITLE_TEXT_PRIMARY, TITLE_TEXT_SECONDARY};
use crate::ui_above_world::{LivedUiPlate, LIVED_UI_Z_PAUSE};
use crate::ui_kit;
pub const AGE_GATE_HEADING: &str = "Before you start";
pub const AGE_GATE_ASK: &str = "Powrush is made for adults. Please enter your month and year of birth.";
pub const AGE_GATE_CONTINUE: &str = "Continue";
pub const AGE_GATE_NOTE: &str = "We only check this on your computer. Your date of birth is not saved or sent anywhere.";
pub const AGE_GATE_UNDER: &str = "Sorry — Powrush is made for players 18 and over.";
pub const AGE_GATE_QUIT: &str = "Quit";
#[derive(Resource, Debug, Clone, Copy)]
pub struct AgeGate { pub open: bool }
#[derive(Resource, Default)]
struct AgeEntry { month: Option<u8>, year: Option<i32>, focus: u8, denied: bool, close: bool }
#[derive(Component)] struct AgeRoot;
#[derive(Component, Clone, Copy, PartialEq, Eq)] enum Pane { Ask, Denied }
#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum AgeBtn { MonthDec, MonthInc, YearDec, YearInc, Continue, Quit }
#[derive(Component)] struct MonthFace;
#[derive(Component)] struct YearFace;
pub struct AgeGatePlugin;
impl Plugin for AgeGatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, boot_age_gate).add_systems(Update, (age_gate_input, age_gate_sync).chain()).add_systems(PostUpdate, age_gate_close);
    }
}
/// Absent resource counts as clear, so a schedule without this plugin plays as today.
pub fn age_gate_clear(gate: Option<Res<AgeGate>>) -> bool { gate.is_none_or(|gate| !gate.open) }
fn boot_age_gate(mut commands: Commands, settings: Res<LocalSettingsState>) {
    let open = settings.inner.should_show_age_gate();
    commands.insert_resource(AgeGate { open });
    if open { commands.insert_resource(AgeEntry::default()); spawn_age_gate(&mut commands); }
}
fn words(text: &str, px: f32, color: Color) -> (Text, TextFont, TextColor) {
    (Text::new(text), TextFont { font_size: FontSize::Px(px), ..default() }, TextColor(color))
}
fn chip(parent: &mut bevy::ecs::relationship::RelatedSpawnerCommands<ChildOf>, label: &str, btn: AgeBtn) {
    parent.spawn((
        ui_kit::button(TITLE_BTN_BG),
        Node { width: Val::Px(44.0), height: Val::Px(44.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, border: UiRect::all(Val::Px(ui_kit::BORDER_HAIRLINE)), ..default() },
        btn,
    )).with_children(|child| { child.spawn(words(label, 16.0, TITLE_BTN_FG)); });
}
fn column() -> Node {
    Node { width: Val::Px(440.0), max_width: Val::Percent(100.0), padding: UiRect::all(Val::Px(18.0)), flex_direction: FlexDirection::Column, row_gap: Val::Px(ui_kit::SPACE_M), border: UiRect::all(Val::Px(2.0)), ..default() }
}
fn spawn_age_gate(commands: &mut Commands) {
    commands.spawn((
        Node { position_type: PositionType::Absolute, width: Val::Percent(100.0), height: Val::Percent(100.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, padding: UiRect::all(Val::Px(24.0)), ..default() },
        BackgroundColor(TITLE_DIM_BG), FocusPolicy::Block, GlobalZIndex(LIVED_UI_Z_PAUSE + 3), AgeRoot, LivedUiPlate,
    )).with_children(|root| {
        root.spawn((column(), ui_kit::plate(), Pane::Ask)).with_children(|pane| {
            pane.spawn(words(AGE_GATE_HEADING, 22.0, TITLE_TEXT_PRIMARY));
            pane.spawn(words(AGE_GATE_ASK, 15.0, TITLE_TEXT_SECONDARY));
            pane.spawn(Node { flex_direction: FlexDirection::Row, column_gap: Val::Px(ui_kit::SPACE_M), align_items: AlignItems::Center, ..default() }).with_children(|row| {
                chip(row, "-", AgeBtn::MonthDec); row.spawn((words("", 16.0, TITLE_TEXT_PRIMARY), MonthFace)); chip(row, "+", AgeBtn::MonthInc);
                chip(row, "-", AgeBtn::YearDec); row.spawn((words("", 16.0, TITLE_TEXT_PRIMARY), YearFace)); chip(row, "+", AgeBtn::YearInc);
            });
            chip(pane, AGE_GATE_CONTINUE, AgeBtn::Continue);
            pane.spawn(words(AGE_GATE_NOTE, 13.0, TITLE_TEXT_SECONDARY));
        });
        root.spawn((column(), ui_kit::plate(), Visibility::Hidden, Pane::Denied)).with_children(|pane| {
            pane.spawn(words(AGE_GATE_UNDER, 18.0, TITLE_TEXT_PRIMARY));
            chip(pane, AGE_GATE_QUIT, AgeBtn::Quit);
        });
    });
}
fn step_month(cur: Option<u8>, up: bool) -> u8 {
    let month = cur.unwrap_or(if up { 0 } else { 13 });
    match (month, up) { (12, true) | (0, true) => 1, (_, true) => month + 1, (0 | 1, false) => 12, (_, false) => month - 1 }
}
fn step_year(cur: Option<i32>, today: i32, up: bool) -> i32 {
    let year = match (cur, up) {
        (None, true) => today.saturating_sub(19), (None, false) => today,
        (Some(year), true) => year.saturating_add(1), (Some(year), false) => year.saturating_sub(1),
    };
    year.clamp(today.saturating_sub(120), today.saturating_add(1))
}
fn try_continue(entry: &mut AgeEntry, settings: &mut LocalSettingsState) {
    let (Some(month), Some(year)) = (entry.month, entry.year) else { return };
    let Some((today_y, today_m, _)) = shared::civil_date::civil_today() else { return };
    entry.month = None; entry.year = None;
    if shared::civil_date::is_adult(today_y, today_m, year, u32::from(month)) {
        settings.inner.confirm_age(); settings.mark_and_persist(); entry.close = true;
    } else { entry.denied = true; }
}
fn age_gate_input(
    mut settings: ResMut<LocalSettingsState>, gate: Res<AgeGate>, mut entry: Option<ResMut<AgeEntry>>,
    keys: Res<ButtonInput<KeyCode>>, pads: Query<&Gamepad>, clicks: Query<(&Interaction, &AgeBtn), Changed<Interaction>>, mut exit: MessageWriter<AppExit>,
) {
    let Some(entry) = entry.as_deref_mut() else { return };
    if !gate.open || entry.close { return }
    let pressed = clicks.iter().find_map(|(i, btn)| (*i == Interaction::Pressed).then_some(*btn));
    let pad = |b| pads.iter().any(|g| g.just_pressed(b));
    let enter = keys.just_pressed(KeyCode::Enter) || pad(GamepadButton::South);
    if entry.denied {
        if pressed == Some(AgeBtn::Quit) || enter { exit.write(AppExit::Success); }
        return;
    }
    if keys.just_pressed(KeyCode::Tab) {
        let back = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
        entry.focus = (entry.focus + if back { 2 } else { 1 }) % 3;
    }
    if pad(GamepadButton::DPadLeft) != pad(GamepadButton::DPadRight) { entry.focus = (entry.focus + if pad(GamepadButton::DPadLeft) { 2 } else { 1 }) % 3; }
    let up = keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::ArrowRight);
    let down = keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::ArrowLeft);
    let (up, down) = (up || pad(GamepadButton::DPadUp), down || pad(GamepadButton::DPadDown));
    let nudge = (up != down && entry.focus < 2).then(|| match (entry.focus, up) {
        (0, true) => AgeBtn::MonthInc, (0, false) => AgeBtn::MonthDec, (_, true) => AgeBtn::YearInc, (_, false) => AgeBtn::YearDec,
    });
    let act = pressed.or(nudge).or_else(|| enter.then_some(AgeBtn::Continue));
    match act {
        Some(AgeBtn::MonthDec) => entry.month = Some(step_month(entry.month, false)),
        Some(AgeBtn::MonthInc) => entry.month = Some(step_month(entry.month, true)),
        Some(AgeBtn::YearDec | AgeBtn::YearInc) => if let Some((today, _, _)) = shared::civil_date::civil_today() {
            entry.year = Some(step_year(entry.year, today, act == Some(AgeBtn::YearInc)));
        },
        Some(AgeBtn::Continue) => try_continue(entry, &mut settings),
        Some(AgeBtn::Quit) => { exit.write(AppExit::Success); }
        None => {}
    }
}
fn age_gate_sync(
    gate: Res<AgeGate>, entry: Option<Res<AgeEntry>>,
    mut roots: Query<&mut Visibility, (With<AgeRoot>, Without<Pane>)>,
    mut panes: Query<(&Pane, &mut Visibility), Without<AgeRoot>>,
    mut month: Query<&mut Text, With<MonthFace>>, mut year: Query<&mut Text, (With<YearFace>, Without<MonthFace>)>,
    mut buttons: Query<(&AgeBtn, &mut BackgroundColor)>,
    pads: Query<(), With<Gamepad>>,
) {
    let denied = entry.as_ref().is_some_and(|entry| entry.denied);
    let ready = entry.as_ref().is_some_and(|entry| entry.month.is_some() && entry.year.is_some());
    let show = |on: bool| if on { Visibility::Visible } else { Visibility::Hidden };
    for mut root in &mut roots { *root = show(gate.open); }
    for (pane, mut vis) in &mut panes { *vis = show(gate.open && (*pane == Pane::Denied) == denied); }
    let month_s = entry.as_ref().and_then(|e| e.month).map(|m| m.to_string()).unwrap_or_default();
    let year_s = entry.as_ref().and_then(|e| e.year).map(|y| y.to_string()).unwrap_or_default();
    for mut text in &mut month { if text.as_str() != month_s { **text = month_s.clone(); } }
    for mut text in &mut year { if text.as_str() != year_s { **text = year_s.clone(); } }
    // The focused field shows only while a pad is connected; a not-ready Continue stays grey.
    let focus = entry.as_ref().filter(|_| !pads.is_empty()).map(|entry| entry.focus);
    for (btn, mut bg) in &mut buttons {
        let fill = match (*btn, focus) {
            (AgeBtn::Continue, _) if !ready => TITLE_BTN_DISABLED_BG,
            (AgeBtn::MonthDec | AgeBtn::MonthInc, Some(0)) | (AgeBtn::YearDec | AgeBtn::YearInc, Some(1)) | (AgeBtn::Continue, Some(2)) => TITLE_BTN_ACTIVE_BG,
            _ => TITLE_BTN_BG,
        };
        bg.set_if_neq(BackgroundColor(fill));
    }
}
fn age_gate_close(mut commands: Commands, mut gate: ResMut<AgeGate>, entry: Option<Res<AgeEntry>>, roots: Query<Entity, With<AgeRoot>>) {
    if entry.is_some_and(|entry| entry.close) {
        gate.open = false;
        for entity in &roots { commands.entity(entity).despawn(); }
        commands.remove_resource::<AgeEntry>();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::keyboard::{Key, KeyboardInput};
    use bevy::input::ButtonState;
    use shared::hex_listen::PowrushNet;
    use crate::input::InputPlugin;
    use crate::local_settings::LocalSettingsPlugin;
    use crate::title_screen::{LaunchDoor, TitleScreenPlugin};
    struct Tmp(std::path::PathBuf);
    impl Drop for Tmp { fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); } }
    fn tmp(tag: &str) -> Tmp {
        let n = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("powrush-age-{tag}-{}-{n}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        Tmp(dir)
    }
    /// Extra resources are what TitleScreenPlugin and LocalSettingsPlugin read. Bevy InputPlugin feeds KeyboardInput into ButtonInput.
    fn boot(gate: bool) -> App {
        let mut app = App::new();
        app.insert_resource(crate::lived_hour_bind::LivedHourBind::default()).insert_resource(crate::hud_presets::ActiveHudPreset::default()).insert_resource(crate::net_mode::SessionNetMode::from_powrush_net(PowrushNet::Off)).add_message::<bevy::input::gamepad::GamepadRumbleRequest>().add_plugins((MinimalPlugins, bevy::input::InputPlugin, InputPlugin, LocalSettingsPlugin, TitleScreenPlugin));
        if gate { app.add_plugins(AgeGatePlugin); }
        app
    }
    fn key(app: &mut App, code: KeyCode, logical: Key, state: ButtonState) {
        app.world_mut().write_message(KeyboardInput { key_code: code, logical_key: logical, state, repeat: false, text: None, window: Entity::PLACEHOLDER });
        app.update();
    }
    fn tap(app: &mut App, code: KeyCode, logical: Key) {
        key(app, code, logical.clone(), ButtonState::Pressed);
        key(app, code, logical, ButtonState::Released);
    }
    fn saved(dir: &std::path::Path) -> String { std::fs::read_to_string(dir.join("powrush_settings.json")).unwrap_or_default() }
    fn door(app: &App) -> LaunchDoor { *app.world().resource::<LaunchDoor>() }
    #[test]
    fn age_gate_schedule_blocks_fresh_enter_then_remembers() {
        let _lock = crate::test_env::lock();
        let prev = std::env::var(shared::user_persist::USER_DIR_OVERRIDE_ENV).ok();
        let run = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let fresh = tmp("fresh");
            std::fs::write(fresh.0.join("powrush_house.json"), "{}").unwrap();
            std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, &fresh.0);
            let mut app = boot(true);
            app.update();
            assert!(app.world().resource::<AgeGate>().open && door(&app) == LaunchDoor::Title);
            let copy = { let mut texts = app.world_mut().query::<&Text>(); texts.iter(app.world()).map(|t| t.as_str().to_string()).collect::<Vec<_>>().join("\n") };
            for line in [AGE_GATE_HEADING, AGE_GATE_ASK, AGE_GATE_CONTINUE, AGE_GATE_NOTE, AGE_GATE_UNDER, AGE_GATE_QUIT] { assert!(copy.contains(line), "{line}"); }
            assert!(AGE_GATE_UNDER.contains('\u{2014}'));
            tap(&mut app, KeyCode::Enter, Key::Enter); tap(&mut app, KeyCode::Digit1, Key::Character("1".into()));
            assert_eq!(door(&app), LaunchDoor::Title);
            tap(&mut app, KeyCode::ArrowUp, Key::ArrowUp); tap(&mut app, KeyCode::Enter, Key::Enter);
            assert!(!app.world().resource::<LocalSettingsState>().inner.age_confirmed && door(&app) == LaunchDoor::Title);
            tap(&mut app, KeyCode::Tab, Key::Tab); tap(&mut app, KeyCode::ArrowUp, Key::ArrowUp); tap(&mut app, KeyCode::Enter, Key::Enter);
            assert!(app.world().resource::<LocalSettingsState>().inner.age_confirmed && !app.world().resource::<AgeGate>().open);
            assert!(saved(&fresh.0).contains("\"age_confirmed\": true"));
            tap(&mut app, KeyCode::Enter, Key::Enter);
            assert_eq!(door(&app), LaunchDoor::InYard);
            drop(app);
            let mut again = boot(true); again.update(); assert!(!again.world().resource::<AgeGate>().open);
            let young = tmp("young");
            std::fs::write(young.0.join("powrush_house.json"), "{}").unwrap();
            std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, &young.0);
            let mut app = boot(true); app.update();
            tap(&mut app, KeyCode::ArrowUp, Key::ArrowUp); tap(&mut app, KeyCode::Tab, Key::Tab); tap(&mut app, KeyCode::ArrowDown, Key::ArrowDown);
            tap(&mut app, KeyCode::Enter, Key::Enter);
            assert!(app.world().resource::<AgeGate>().open);
            key(&mut app, KeyCode::Enter, Key::Enter, ButtonState::Pressed);
            assert_eq!(app.should_exit(), Some(AppExit::Success));
            assert!(!saved(&young.0).contains("\"age_confirmed\": true"));
            drop(app);
            let mut relaunch = boot(true); relaunch.update(); assert!(relaunch.world().resource::<AgeGate>().open);
            let bad = tmp("bad");
            std::fs::write(bad.0.join("powrush_settings.json"), "NOT JSON").unwrap();
            std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, &bad.0);
            let mut app = boot(true); app.update();
            assert!(app.world().resource::<AgeGate>().open && !app.world().resource::<LocalSettingsState>().inner.age_confirmed);
            let open = tmp("open");
            std::fs::write(open.0.join("powrush_house.json"), "{}").unwrap();
            std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, &open.0);
            let mut app = boot(false); app.update();
            assert!(app.world().get_resource::<AgeGate>().is_none());
            tap(&mut app, KeyCode::Enter, Key::Enter);
            assert_eq!(door(&app), LaunchDoor::InYard);
        }));
        match &prev { Some(value) => std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, value), None => std::env::remove_var(shared::user_persist::USER_DIR_OVERRIDE_ENV) }
        if let Err(payload) = run { std::panic::resume_unwind(payload); }
    }
    /// LAUNCH-MENU-1 fixture 7: a pad alone clears the gate (D-pad sets month, then year; A continues), then A on Title plays.
    #[test]
    fn pad_only_boot_clears_age_gate_then_plays() {
        use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent, RawGamepadButtonChangedEvent, RawGamepadEvent};
        let _lock = crate::test_env::lock();
        let prev = std::env::var(shared::user_persist::USER_DIR_OVERRIDE_ENV).ok();
        let fresh = tmp("pad");
        let run = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            std::fs::write(fresh.0.join("powrush_house.json"), "{}").unwrap();
            std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, &fresh.0);
            let mut app = boot(true);
            app.add_plugins(crate::launch_menu::LaunchMenuPlugin);
            let pad = app.world_mut().spawn_empty().id();
            app.world_mut().write_message(GamepadConnectionEvent::new(pad, GamepadConnection::Connected { name: "pad".into(), vendor_id: None, product_id: None }));
            app.update();
            let press = |app: &mut App, b| for v in [1.0, 0.0] { app.world_mut().write_message(RawGamepadEvent::Button(RawGamepadButtonChangedEvent::new(pad, b, v))); app.update(); };
            press(&mut app, GamepadButton::South);
            assert!(app.world().resource::<AgeGate>().open, "A with no date does nothing");
            for b in [GamepadButton::DPadUp, GamepadButton::DPadRight, GamepadButton::DPadUp, GamepadButton::South] { press(&mut app, b); }
            assert!(app.world().resource::<LocalSettingsState>().inner.age_confirmed && !app.world().resource::<AgeGate>().open);
            assert_eq!(door(&app), LaunchDoor::Title, "the A that clears the gate does not also play");
            press(&mut app, GamepadButton::South);
            assert_eq!(door(&app), LaunchDoor::InYard);
        }));
        match &prev { Some(value) => std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, value), None => std::env::remove_var(shared::user_persist::USER_DIR_OVERRIDE_ENV) }
        if let Err(payload) = run { std::panic::resume_unwind(payload); }
    }
}
