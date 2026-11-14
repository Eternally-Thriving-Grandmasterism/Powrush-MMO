//! CARD LAUNCH-MENU-1 — Title menu rows Credits and Quit, gamepad focus, and the
//! Medium-and-up scrim over the static yard view. Rows in plate order: Play ·
//! Continue · Settings · Persona · Credits · Quit, then Online grey. Online carries
//! no [`LaunchRow`], so a pad can never focus or press it. No new keys, no new copy.
//! Contact: info@Rathor.ai
use bevy::prelude::*;
use shared::local_settings::GraphicsPreset;
use shared::persona::PERSONA_CREATOR_ENABLED;

use crate::age_gate::{age_gate_clear, AgeGate};
use crate::combat_core::MusicPresent;
use crate::hex_travel::PlacesPlate;
use crate::input::InputMapSet;
use crate::licences_screen::{open_licences, LicencesScreen};
use crate::lived_hour_bind::LivedHourBind;
use crate::local_settings::LocalSettingsState;
use crate::title_screen::{
    quit_to_desktop, skip_persona_to_nameless, spawn_title_plate_btn, sync_title_visibility, title_button_clicks,
    HouseLabel, LaunchDoor, PersonaCreatorState, TitleRoot, TITLE_CHROME_CREDITS, TITLE_CHROME_QUIT,
};
use crate::ui_kit::{TITLE_BTN_ACTIVE_BG, TITLE_BTN_BG, TITLE_BTN_DISABLED_BG, TITLE_DIM_BG, TITLE_SCRIM_BG};

/// A Title row a pad can focus, in plate order.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaunchRow { Play, Continue, Settings, Persona, Credits, Quit }

impl LaunchRow {
    pub const ALL: [LaunchRow; 6] = [Self::Play, Self::Continue, Self::Settings, Self::Persona, Self::Credits, Self::Quit];

    /// Continue needs a save and Persona needs its flag; the other rows are always live.
    pub fn enabled(self, persist_present: bool) -> bool {
        match self {
            Self::Continue => persist_present,
            Self::Persona => PERSONA_CREATOR_ENABLED,
            _ => true,
        }
    }
}

/// The pad's row. Painted only while a pad is connected, so mouse and keys see today's plate.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LaunchFocus(pub LaunchRow);

impl Default for LaunchFocus {
    fn default() -> Self {
        Self(LaunchRow::Play)
    }
}

/// Medium, High and Ultra show the yard through the scrim. Mobile and Low keep the opaque dimmer.
pub fn title_scrim(preset: GraphicsPreset) -> Color {
    match preset {
        GraphicsPreset::Mobile | GraphicsPreset::Low => TITLE_DIM_BG,
        GraphicsPreset::Medium | GraphicsPreset::High | GraphicsPreset::Ultra => TITLE_SCRIM_BG,
    }
}

pub struct LaunchMenuPlugin;

impl Plugin for LaunchMenuPlugin {
    fn build(&self, app: &mut App) {
        // A pad press lands as a click: nav marks the row Pressed before the click systems read it.
        let nav = launch_pad_nav.after(InputMapSet).before(title_button_clicks);
        app.init_resource::<LaunchFocus>()
            .add_systems(Update, (nav, launch_row_clicks).chain().run_if(age_gate_clear))
            .add_systems(Update, (paint_launch_focus.after(sync_title_visibility).after(launch_pad_nav), sync_title_scrim));
    }
}

/// Credits and Quit through the Title plate button, so they share its chrome and text scale.
pub(crate) fn spawn_launch_rows(p: &mut ChildSpawnerCommands, text_scale: f32) {
    spawn_title_plate_btn(p, TITLE_CHROME_CREDITS, LaunchRow::Credits, true, text_scale);
    spawn_title_plate_btn(p, TITLE_CHROME_QUIT, LaunchRow::Quit, true, text_scale);
}

/// D-pad up/down walks the live rows and South presses the focused one. East backs out of
/// Settings or Persona the way Esc does. Licences owns the pad while open and for one frame
/// after, so the East that closed it closes nothing else.
fn launch_pad_nav(
    pads: Query<&Gamepad>,
    (door, licences): (Res<LaunchDoor>, Option<Res<LicencesScreen>>),
    mut label: ResMut<HouseLabel>,
    mut persona: ResMut<PersonaCreatorState>,
    mut focus: ResMut<LaunchFocus>,
    mut rows: Query<(Entity, &LaunchRow, &mut Interaction)>,
    (mut pressed, mut licences_was_open): (Local<Option<Entity>>, Local<bool>),
) {
    if let Some((_, _, mut i)) = pressed.take().and_then(|e| rows.get_mut(e).ok()) {
        if *i == Interaction::Pressed {
            *i = Interaction::None;
        }
    }
    let licences_open = licences.is_some_and(|l| l.open);
    if std::mem::replace(&mut *licences_was_open, licences_open) || licences_open || *door != LaunchDoor::Title {
        return;
    }
    let edge = |b| pads.iter().any(|g| g.just_pressed(b));
    if persona.open || label.settings_open {
        if edge(GamepadButton::East) && persona.open {
            skip_persona_to_nameless(&mut persona);
        } else if edge(GamepadButton::East) {
            label.settings_open = false;
        }
        return;
    }
    let ring: Vec<LaunchRow> = LaunchRow::ALL.into_iter().filter(|r| r.enabled(label.persist_present)).collect();
    let at = ring.iter().position(|r| *r == focus.0).unwrap_or(0) as isize;
    let step = edge(GamepadButton::DPadDown) as isize - edge(GamepadButton::DPadUp) as isize;
    let next = ring[(at + step).rem_euclid(ring.len() as isize) as usize];
    if focus.0 != next {
        focus.0 = next;
    }
    if edge(GamepadButton::South) {
        if let Some((e, _, mut i)) = rows.iter_mut().find(|(_, r, _)| **r == next) {
            *i = Interaction::Pressed;
            *pressed = Some(e);
        }
    }
}

/// Credits opens NOTICES-1b's Licences on page 0 (refused under the age gate). Quit is
/// pause Quit's fn: AppExit, and on Title no save is written.
fn launch_row_clicks(
    rows: Query<(&Interaction, &LaunchRow), Changed<Interaction>>,
    (door, persona): (Res<LaunchDoor>, Res<PersonaCreatorState>),
    (licences, gate, music): (Option<ResMut<LicencesScreen>>, Option<Res<AgeGate>>, Option<Res<MusicPresent>>),
    (mut label, mut places, bind): (ResMut<HouseLabel>, Option<ResMut<PlacesPlate>>, Option<Res<LivedHourBind>>),
    mut exit: MessageWriter<AppExit>,
) {
    if *door != LaunchDoor::Title || persona.open {
        return;
    }
    match rows.iter().find_map(|(i, r)| (*i == Interaction::Pressed).then_some(*r)) {
        Some(LaunchRow::Credits) => {
            if let Some(mut screen) = licences {
                open_licences(&mut screen, gate.as_deref(), Some(*door), music.is_some_and(|m| m.0));
            }
        }
        Some(LaunchRow::Quit) => quit_to_desktop(*door, &mut label, places.as_deref_mut(), bind.as_deref(), &mut exit),
        _ => {}
    }
}

/// Fill per row: Online grey when off, the active purple under pad focus, else the button fill.
fn paint_launch_focus(
    pads: Query<(), With<Gamepad>>,
    focus: Res<LaunchFocus>,
    label: Res<HouseLabel>,
    mut rows: Query<(&LaunchRow, &mut BackgroundColor)>,
) {
    for (row, mut bg) in &mut rows {
        let fill = if !row.enabled(label.persist_present) {
            TITLE_BTN_DISABLED_BG
        } else if *row == focus.0 && !pads.is_empty() {
            TITLE_BTN_ACTIVE_BG
        } else {
            TITLE_BTN_BG
        };
        bg.set_if_neq(BackgroundColor(fill));
    }
}

/// Repaints the one Title root when the preset changes; no respawn.
fn sync_title_scrim(settings: Res<LocalSettingsState>, mut roots: Query<&mut BackgroundColor, With<TitleRoot>>) {
    let want = BackgroundColor(title_scrim(settings.inner.graphics_preset));
    for mut bg in &mut roots {
        bg.set_if_neq(want);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent, RawGamepadButtonChangedEvent, RawGamepadEvent};
    use bevy::input::keyboard::{Key, KeyboardInput, NativeKey};
    use bevy::input::ButtonState;
    use shared::user_persist::USER_DIR_OVERRIDE_ENV as DIR_ENV;
    use crate::hud_anchor_registry::HudSlab;
    use crate::hud_presets::{ActiveHudPreset, HudPresetId};
    use crate::licences_screen::{LicencesRoot, LicencesScreenPlugin};

    /// Scratch user dir; drop restores the env var and removes the dir. Take `test_env::lock()` first.
    struct Scratch(std::path::PathBuf, Option<String>);
    impl Drop for Scratch {
        fn drop(&mut self) { match self.1.take() { Some(v) => std::env::set_var(DIR_ENV, v), None => std::env::remove_var(DIR_ENV) }; let _ = std::fs::remove_dir_all(&self.0); }
    }
    fn scratch(tag: &str) -> Scratch {
        let n = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_nanos());
        let dir = std::env::temp_dir().join(format!("powrush-launch-{tag}-{}-{n}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let prev = std::env::var(DIR_ENV).ok();
        std::env::set_var(DIR_ENV, &dir);
        Scratch(dir, prev)
    }
    /// Title, settings, Licences and this plugin over Bevy input; no age gate unless a test inserts one.
    fn h1() -> App {
        let mut app = App::new();
        app.insert_resource(LivedHourBind::default()).insert_resource(ActiveHudPreset::default())
            .insert_resource(crate::net_mode::SessionNetMode::from_powrush_net(shared::hex_listen::PowrushNet::Off))
            .add_message::<bevy::input::gamepad::GamepadRumbleRequest>()
            .add_plugins((MinimalPlugins, bevy::input::InputPlugin, crate::input::InputPlugin, crate::local_settings::LocalSettingsPlugin))
            .add_plugins((crate::title_screen::TitleScreenPlugin, LicencesScreenPlugin, LaunchMenuPlugin));
        app.update();
        app
    }
    fn tap(app: &mut App, code: KeyCode) {
        for state in [ButtonState::Pressed, ButtonState::Released] {
            let logical_key = Key::Unidentified(NativeKey::Unidentified);
            app.world_mut().write_message(KeyboardInput { key_code: code, logical_key, state, repeat: false, text: None, window: Entity::PLACEHOLDER });
            app.update();
        }
    }
    /// The first call connects a pad. Each call is one press frame and one release frame.
    fn pad(app: &mut App, button: GamepadButton) {
        let found = app.world_mut().query_filtered::<Entity, With<Gamepad>>().iter(app.world()).next();
        let id = found.unwrap_or_else(|| {
            let (id, name) = (app.world_mut().spawn_empty().id(), "pad".into());
            app.world_mut().write_message(GamepadConnectionEvent::new(id, GamepadConnection::Connected { name, vendor_id: None, product_id: None }));
            app.update();
            id
        });
        for value in [1.0, 0.0] {
            app.world_mut().write_message(RawGamepadEvent::Button(RawGamepadButtonChangedEvent::new(id, button, value)));
            app.update();
        }
    }
    /// Mouse press, then release, a frame each.
    fn click(app: &mut App, want: LaunchRow) {
        let mut rows = app.world_mut().query::<(&LaunchRow, &mut Interaction)>();
        for state in [Interaction::Pressed, Interaction::None] {
            rows.iter_mut(app.world_mut()).filter(|(r, _)| **r == want).for_each(|(_, mut i)| *i = state);
            app.update();
        }
    }
    fn licences(app: &App) -> (bool, usize) { let s = app.world().resource::<LicencesScreen>(); (s.open, s.page) }
    fn licence_roots(app: &mut App) -> usize { app.world_mut().query_filtered::<(), With<LicencesRoot>>().iter(app.world()).count() }
    fn door(app: &App) -> LaunchDoor { *app.world().resource::<LaunchDoor>() }
    fn focus(app: &App) -> LaunchRow { app.world().resource::<LaunchFocus>().0 }
    fn fill(app: &mut App, want: LaunchRow) -> Color {
        app.world_mut().query::<(&LaunchRow, &BackgroundColor)>().iter(app.world()).find(|(r, _)| **r == want).map(|(_, bg)| bg.0).unwrap()
    }

    /// Fixture 2: Credits opens Licences on page 0 by click and by pad; refused under the age gate; Esc and B back out to Title.
    #[test]
    fn credits_opens_licences_page_zero_then_backs_out_to_title() {
        let _lock = crate::test_env::lock();
        let _dir = scratch("credits");
        let mut app = h1();
        click(&mut app, LaunchRow::Credits);
        assert_eq!((licences(&app), licence_roots(&mut app)), ((true, 0), 1), "click opens page 0");
        app.world_mut().resource_mut::<LicencesScreen>().page = 1;
        tap(&mut app, KeyCode::Escape);
        let settings_open = app.world().resource::<HouseLabel>().settings_open;
        assert_eq!((licences(&app).0, door(&app), settings_open), (false, LaunchDoor::Title, false), "Esc backs out to Title");
        LaunchRow::ALL.iter().for_each(|_| if focus(&app) != LaunchRow::Credits { pad(&mut app, GamepadButton::DPadDown) });
        assert_eq!(focus(&app), LaunchRow::Credits);
        pad(&mut app, GamepadButton::South);
        assert_eq!(licences(&app), (true, 0), "A on Credits opens page 0 again");
        pad(&mut app, GamepadButton::East);
        assert_eq!((licences(&app).0, door(&app), licence_roots(&mut app)), (false, LaunchDoor::Title, 0), "B backs out to Title");
        app.insert_resource(AgeGate { open: true });
        click(&mut app, LaunchRow::Credits);
        assert_eq!((licences(&app).0, licence_roots(&mut app)), (false, 0), "refused under the age gate");
    }

    /// Fixture 3: with Licences up on Title, keys 1 / 2 / 3 / Enter and pad A change nothing.
    #[test]
    fn licences_up_on_title_swallows_menu_keys_and_pad_a() {
        let _lock = crate::test_env::lock();
        let _dir = scratch("guard");
        let mut app = h1();
        app.world_mut().resource_mut::<HouseLabel>().persist_present = true;
        click(&mut app, LaunchRow::Credits);
        [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Enter].into_iter().for_each(|code| tap(&mut app, code));
        pad(&mut app, GamepadButton::South);
        let settings_open = app.world().resource::<HouseLabel>().settings_open;
        assert_eq!((door(&app), settings_open, licences(&app)), (LaunchDoor::Title, false, (true, 0)));
    }

    /// Fixture 4: Title Quit is pause Quit's fn: AppExit, and the user dir is unchanged.
    #[test]
    fn title_quit_exits_through_pause_quit_and_writes_nothing() {
        let _lock = crate::test_env::lock();
        let dir = scratch("quit");
        let mut app = h1();
        let files = |d: &std::path::Path| {
            std::fs::read_dir(d).unwrap().flatten().map(|e| (e.file_name(), std::fs::read(e.path()).ok())).collect::<std::collections::BTreeMap<_, _>>()
        };
        let before = files(&dir.0);
        click(&mut app, LaunchRow::Quit);
        assert_eq!(app.should_exit(), Some(AppExit::Success));
        assert_eq!(files(&dir.0), before, "Title Quit writes no save");
        let prod = |src: &'static str| src.split("\n#[cfg(test)]\nmod tests {").next().unwrap();
        assert!(prod(include_str!("title_screen.rs")).contains("// Pause Quit (not Esc).\n            quit_to_desktop(*door, &mut label,"));
        assert!(prod(include_str!("launch_menu.rs")).contains("Some(LaunchRow::Quit) => quit_to_desktop(*door, &mut label,"));
    }

    /// Fixture 5: Medium and up show the scrim; Mobile and Low keep TITLE_DIM_BG exactly; a live change repaints the same root.
    #[test]
    fn scrim_follows_the_graphics_preset_live() {
        let _lock = crate::test_env::lock();
        let _dir = scratch("scrim");
        let mut app = h1();
        let mut roots = app.world_mut().query_filtered::<Entity, With<TitleRoot>>();
        let root = roots.single(app.world()).unwrap();
        assert!(TITLE_SCRIM_BG.alpha() < 1.0 && TITLE_DIM_BG.alpha() == 1.0);
        for preset in GraphicsPreset::ALL {
            app.world_mut().resource_mut::<LocalSettingsState>().inner.graphics_preset = preset;
            app.update();
            let want = if matches!(preset, GraphicsPreset::Mobile | GraphicsPreset::Low) { TITLE_DIM_BG } else { TITLE_SCRIM_BG };
            assert_eq!(app.world().get::<BackgroundColor>(root).map(|bg| bg.0), Some(want), "{preset:?}");
        }
        assert_eq!(roots.iter(app.world()).collect::<Vec<_>>(), [root], "no respawn");
    }

    /// Fixture 6: the shipping bundle on a fresh install takes its preset from settings (Classic), so no HudSlab
    /// shows on Title, before or after a visit to the yard. The yard shows slabs, so the probe can see them.
    #[test]
    fn no_hud_slab_shows_on_title_with_the_default_preset() {
        let _lock = crate::test_env::lock();
        let _dir = scratch("hud");
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default(), bevy::input::InputPlugin)).set_error_handler(bevy::ecs::error::ignore)
            .init_asset::<Mesh>().init_asset::<StandardMaterial>().init_asset::<Image>().init_asset::<Shader>()
            .init_asset::<crate::painterly_shader::PainterlyMaterial>().init_asset::<bevy::audio::AudioSource>()
            .add_message::<bevy::input::gamepad::GamepadRumbleRequest>().add_plugins(crate::PowrushClientBundle);
        let shown = |app: &mut App| {
            app.world_mut().query::<(&HudSlab, &Visibility)>().iter(app.world()).filter(|(_, v)| **v != Visibility::Hidden).map(|(s, _)| s.0).collect::<Vec<_>>()
        };
        (0..5).for_each(|_| app.update());
        assert_eq!((ActiveHudPreset::default().id, app.world().resource::<ActiveHudPreset>().id), (None, Some(HudPresetId::Classic)));
        assert_eq!(shown(&mut app), Vec::<&str>::new(), "fresh Title");
        *app.world_mut().resource_mut::<LaunchDoor>() = LaunchDoor::InYard;
        (0..30).for_each(|_| app.update());
        assert!(!shown(&mut app).is_empty(), "the yard shows slabs");
        *app.world_mut().resource_mut::<LaunchDoor>() = LaunchDoor::Title;
        (0..5).for_each(|_| app.update());
        assert_eq!(shown(&mut app), Vec::<&str>::new(), "back on Title");
    }

    /// Fixture 7 (rows): the D-pad walks every live row in plate order and wraps; Online grey has no row tag;
    /// the focus fill shows only once a pad is connected.
    #[test]
    fn pad_focus_reaches_every_live_row_and_never_online() {
        let _lock = crate::test_env::lock();
        let _dir = scratch("focus");
        let mut app = h1();
        app.world_mut().resource_mut::<HouseLabel>().persist_present = true;
        app.update();
        assert_eq!(fill(&mut app, LaunchRow::Play), TITLE_BTN_BG, "no pad, no focus fill");
        let live: Vec<LaunchRow> = LaunchRow::ALL.into_iter().filter(|r| r.enabled(true)).collect();
        let mut seen = vec![focus(&app)];
        for _ in 0..live.len() {
            pad(&mut app, GamepadButton::DPadDown);
            let at = focus(&app);
            seen.push(at);
            assert_eq!(fill(&mut app, at), TITLE_BTN_ACTIVE_BG, "{seen:?}");
        }
        assert_eq!(seen, [live.as_slice(), &[LaunchRow::Play]].concat(), "down walks every live row, then wraps");
        pad(&mut app, GamepadButton::DPadUp);
        assert_eq!(focus(&app), LaunchRow::Quit, "up from Play wraps to Quit");
        app.world_mut().resource_mut::<HouseLabel>().persist_present = false;
        (0..2).for_each(|_| pad(&mut app, GamepadButton::DPadDown));
        assert_eq!(focus(&app), LaunchRow::Settings, "Continue without a save is skipped");
        assert_eq!(fill(&mut app, LaunchRow::Continue), TITLE_BTN_DISABLED_BG);
        assert_eq!(app.world_mut().query::<&LaunchRow>().iter(app.world()).count(), LaunchRow::ALL.len(), "Online carries no LaunchRow");
    }
}
