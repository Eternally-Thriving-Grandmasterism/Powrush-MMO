//! CARD NOTICES-1b — third-party licences, one notice section at a time.
//! Embedded at build time so an install cannot lose it. Contact: info@Rathor.ai
use std::sync::LazyLock;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy::ui_widgets::ScrollArea;
use crate::age_gate::AgeGate;
use crate::title_screen::LaunchDoor;
use crate::ui_kit::{self, TITLE_BTN_BG, TITLE_BTN_FG, TITLE_DIM_BG, TITLE_TEXT_PRIMARY, TITLE_TEXT_SECONDARY};
use crate::ui_above_world::{LivedUiPlate, LIVED_UI_Z_PAUSE};

pub const LICENCES_BUTTON: &str = "Licences";
pub const LICENCES_TITLE: &str = "Third-party licences";
pub const LICENCES_INTRO: &str = "Open-source software and fonts used in Powrush, with their licence texts.";
pub const LICENCES_BACK_HINT: &str = "Esc / B · Back";
pub const FIRA_SANS_HEADING: &str = "Fira Sans (SIL Open Font License 1.1)";
pub const NOTICES: &str = include_str!("../../THIRD-PARTY-NOTICES.txt");
/// Fira Sans is the face on screen; the notices only name Fira Mono.
pub const FIRA_SANS_OFL: &str = include_str!("../assets/fonts/OFL.txt");

const RULE_WIDTH: usize = 80;
const PLATE_MAX_WIDTH: f32 = 760.0;
const PLATE_PAD: f32 = ui_kit::SPACE_2XL;
const PLATE_BORDER: f32 = 2.0;
const PLATE_GAP: f32 = ui_kit::SPACE_M;
const BTN_PAD_Y: f32 = ui_kit::SPACE_S;
/// Logical px, fixed like the music credits, so text scale never grows the chrome.
const TITLE_PX: f32 = 20.0;
const INTRO_PX: f32 = ui_kit::TYPE_SMALL;
const NAV_PX: f32 = ui_kit::TYPE_LABEL;
const BODY_PX: f32 = ui_kit::TYPE_CAPTION;
/// D-pad scroll per held frame, logical px.
const PAD_SCROLL_PX: f32 = 18.0;

static SECTIONS: LazyLock<Vec<&'static str>> = LazyLock::new(|| split_notices(NOTICES));

/// Split on the 80-`=` rule lines; a CRLF checkout splits the same way.
pub fn split_notices(text: &str) -> Vec<&str> {
    let (mut out, mut start, mut at) = (Vec::new(), 0, 0);
    for line in text.split_inclusive('\n') {
        let bare = line.trim_end();
        if bare.len() == RULE_WIDTH && bare.bytes().all(|b| b == b'=') {
            out.push(text[start..at].trim());
            start = at + line.len();
        }
        at += line.len();
    }
    out.push(text[start..].trim());
    out
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LicencePage { Notice(usize), FiraSans, Music }

/// Notices, then Fira Sans, then Music only once a track is credited (MC5).
pub fn licence_pages(music_present: bool) -> Vec<LicencePage> {
    let mut pages: Vec<_> = (0..SECTIONS.len()).map(LicencePage::Notice).collect();
    pages.push(LicencePage::FiraSans);
    if crate::title_screen::music_credits_unlocked(music_present) { pages.push(LicencePage::Music); }
    pages
}

/// (heading, body). Music has no body text; its rows come from the credits table.
pub fn page_text(page: LicencePage) -> (&'static str, &'static str) {
    match page {
        LicencePage::Notice(i) => {
            let (head, body) = SECTIONS[i].split_once('\n').unwrap_or((SECTIONS[i], ""));
            (head.trim_end(), body.trim_start_matches(['\r', '\n']))
        }
        LicencePage::FiraSans => (FIRA_SANS_HEADING, FIRA_SANS_OFL),
        LicencePage::Music => ("Music", ""),
    }
}

#[derive(Resource, Debug, Default)]
pub struct LicencesScreen { pub open: bool, pub page: usize, music: bool, door: Option<LaunchDoor> }

impl LicencesScreen {
    /// Prev/next, wrapping at both ends.
    pub fn step(&mut self, delta: isize) {
        let n = licence_pages(self.music).len() as isize;
        self.page = (self.page as isize + delta).rem_euclid(n) as usize;
    }
}

/// The one way in: the Guide tab today, Title Credits later. Refused under the age gate.
pub(crate) fn open_licences(screen: &mut LicencesScreen, gate: Option<&AgeGate>, door: Option<LaunchDoor>, music_present: bool) -> bool {
    if gate.is_some_and(|g| g.open) { return false; }
    *screen = LicencesScreen { open: true, page: 0, music: music_present, door };
    true
}

#[derive(Component)] pub struct LicencesOpenBtn;
#[derive(Component)] pub struct LicencesRoot;
#[derive(Component)] pub struct LicencesBody;
#[derive(Component, Clone, Copy, PartialEq, Eq)] enum NavBtn { Prev, Next, Back }

pub struct LicencesScreenPlugin;

impl Plugin for LicencesScreenPlugin {
    fn build(&self, app: &mut App) {
        // Esc is read by the title guards (title_keyboard_shortcuts, esc_yard_pause) so one press closes only this.
        let input = licences_input.after(crate::input::InputMapSet);
        app.init_resource::<LicencesScreen>().add_systems(Update, (input, sync_licences).chain());
    }
}

fn words(text: impl Into<String>, px: f32, color: Color) -> (Text, TextFont, TextColor) {
    (Text::new(text), ui_kit::font(px), TextColor(color))
}

fn spawn_btn(p: &mut ChildSpawnerCommands, label: &str, marker: impl Component, width: Val) {
    let node = Node { width, padding: UiRect::axes(Val::Px(ui_kit::SPACE_XL), Val::Px(BTN_PAD_Y)), justify_content: JustifyContent::Center, border: UiRect::all(Val::Px(ui_kit::BORDER_HAIRLINE)), ..default() };
    p.spawn((ui_kit::button(TITLE_BTN_BG), node, marker))
        .with_children(|b| { b.spawn(words(label, NAV_PX, TITLE_BTN_FG)); });
}

/// Guide tab row. Not a fifth bottom row: the pause plate height math counts four.
pub fn spawn_licences_entry(p: &mut ChildSpawnerCommands) {
    spawn_btn(p, LICENCES_BUTTON, LicencesOpenBtn, Val::Percent(100.0));
}

fn licences_input(
    pads: Query<&Gamepad>,
    (gate, door, music): (Option<Res<AgeGate>>, Option<Res<LaunchDoor>>, Option<Res<crate::combat_core::MusicPresent>>),
    mut screen: ResMut<LicencesScreen>,
    entry: Query<&Interaction, (Changed<Interaction>, With<LicencesOpenBtn>)>,
    nav: Query<(&Interaction, &NavBtn), Changed<Interaction>>,
    mut body: Query<(&mut ScrollPosition, &ComputedNode), With<LicencesBody>>,
) {
    let door = door.map(|d| *d);
    if !screen.open {
        if entry.iter().any(|i| *i == Interaction::Pressed) {
            open_licences(&mut screen, gate.as_deref(), door, music.is_some_and(|m| m.0));
        }
        return;
    }
    let pad = pads.iter().next();
    let btn = |b| pad.is_some_and(|g| g.just_pressed(b));
    let clicked = |want| nav.iter().any(|(i, n)| *i == Interaction::Pressed && *n == want);
    if btn(GamepadButton::East) || clicked(NavBtn::Back) || screen.door != door {
        screen.open = false;
        return;
    }
    if btn(GamepadButton::RightTrigger) || clicked(NavBtn::Next) {
        screen.step(1);
    } else if btn(GamepadButton::LeftTrigger) || clicked(NavBtn::Prev) {
        screen.step(-1);
    }
    let held = |b| pad.is_some_and(|g| g.pressed(b)) as i8 as f32;
    let dy = held(GamepadButton::DPadDown) - held(GamepadButton::DPadUp);
    for (mut pos, node) in &mut body {
        let max = ((node.content_size.y - node.size.y) * node.inverse_scale_factor).max(0.0);
        pos.y = (pos.y + dy * PAD_SCROLL_PX).clamp(0.0, max);
    }
}

fn sync_licences(mut commands: Commands, screen: Res<LicencesScreen>, roots: Query<Entity, With<LicencesRoot>>, mut shown: Local<Option<usize>>) {
    let want = screen.open.then_some(screen.page);
    if want == *shown { return; }
    *shown = want;
    for e in &roots { commands.entity(e).despawn(); }
    if let Some(page) = want { spawn_licences(&mut commands, &screen, page); }
}

fn spawn_licences(commands: &mut Commands, screen: &LicencesScreen, page: usize) {
    let pages = licence_pages(screen.music);
    let current = pages[page.min(pages.len() - 1)];
    let (heading, body) = page_text(current);
    let full = Node { position_type: PositionType::Absolute, width: Val::Percent(100.0), height: Val::Percent(100.0), justify_content: JustifyContent::Center, padding: UiRect::vertical(Val::Percent(1.0)), ..default() };
    let plate = Node { width: Val::Percent(96.0), max_width: Val::Px(PLATE_MAX_WIDTH), height: Val::Percent(100.0), padding: UiRect::all(Val::Px(PLATE_PAD)), flex_direction: FlexDirection::Column, row_gap: Val::Px(PLATE_GAP), border: UiRect::all(Val::Px(PLATE_BORDER)), ..default() };
    let scroll = Node { flex_grow: 1.0, flex_basis: Val::Px(0.0), min_height: Val::Px(0.0), flex_direction: FlexDirection::Column, overflow: Overflow::scroll_y(), ..default() };
    commands.spawn((full, BackgroundColor(TITLE_DIM_BG), FocusPolicy::Block, GlobalZIndex(LIVED_UI_Z_PAUSE + 2), LicencesRoot, LivedUiPlate)).with_children(|root| {
        root.spawn((plate, ui_kit::plate())).with_children(|p| {
            p.spawn(words(LICENCES_TITLE, TITLE_PX, TITLE_TEXT_PRIMARY));
            p.spawn(words(LICENCES_INTRO, INTRO_PX, TITLE_TEXT_SECONDARY));
            p.spawn(Node { column_gap: Val::Px(ui_kit::SPACE_M), align_items: AlignItems::Center, ..default() }).with_children(|row| {
                spawn_btn(row, "LB · Prev", NavBtn::Prev, Val::Auto);
                let at = format!("{} / {} · {heading}", page + 1, pages.len());
                row.spawn((words(at, NAV_PX, TITLE_TEXT_PRIMARY), Node { flex_grow: 1.0, ..default() }));
                spawn_btn(row, "Next · RB", NavBtn::Next, Val::Auto);
            });
            p.spawn((scroll, ScrollArea, LicencesBody)).with_children(|b| match current {
                LicencePage::Music => crate::title_screen::spawn_music_credits_rows(b, screen.music),
                _ => { b.spawn(words(body, BODY_PX, TITLE_TEXT_SECONDARY)); }
            });
            spawn_btn(p, LICENCES_BACK_HINT, NavBtn::Back, Val::Percent(100.0));
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{combat_core::MusicPresent, local_settings::LocalSettingsState, title_screen::{title_plate_fits_surface, MUSIC_CREDITS}};

    /// H1: MinimalPlugins and this plugin only, on `door`.
    fn h1(door: LaunchDoor) -> App { let mut app = App::new(); app.add_plugins((MinimalPlugins, LicencesScreenPlugin)).insert_resource(door); app }

    /// Spawns the real Guide entry, presses it, and runs one frame.
    fn press_entry(app: &mut App) {
        app.world_mut().commands().spawn(Node::default()).with_children(spawn_licences_entry);
        app.world_mut().flush();
        let mut q = app.world_mut().query_filtered::<&mut Interaction, With<LicencesOpenBtn>>();
        q.iter_mut(app.world_mut()).for_each(|mut i| *i = Interaction::Pressed);
        app.update();
    }

    /// One pad frame: press, update, then drop the edge so it fires once.
    fn pad(app: &mut App, button: GamepadButton, release: bool) {
        let mut q = app.world_mut().query::<&mut Gamepad>();
        if q.iter(app.world()).next().is_none() { app.world_mut().spawn(Gamepad::default()); }
        q.iter_mut(app.world_mut()).for_each(|mut g| g.digital_mut().press(button));
        app.update();
        q.iter_mut(app.world_mut()).for_each(|mut g| { g.digital_mut().clear(); if release { g.digital_mut().release(button) } });
    }

    fn texts(app: &mut App) -> Vec<String> { app.world_mut().query::<&Text>().iter(app.world()).map(|t| t.0.clone()).collect() }
    fn roots(app: &mut App) -> Vec<i32> { app.world_mut().query_filtered::<&GlobalZIndex, With<LicencesRoot>>().iter(app.world()).map(|z| z.0).collect() }
    fn open(app: &App) -> (bool, usize) { let s = app.world().resource::<LicencesScreen>(); (s.open, s.page) }

    #[test]
    fn licences_copy_and_fira_sans_heading_are_verbatim() {
        assert_eq!(LICENCES_BUTTON, "Licences");
        assert_eq!(LICENCES_TITLE, "Third-party licences");
        assert_eq!(LICENCES_INTRO, "Open-source software and fonts used in Powrush, with their licence texts.");
        assert_eq!(LICENCES_BACK_HINT, "Esc / B · Back");
        assert_eq!(FIRA_SANS_HEADING, "Fira Sans (SIL Open Font License 1.1)");
        let (head, body) = page_text(LicencePage::FiraSans);
        assert_eq!(head, FIRA_SANS_HEADING);
        assert!(body.contains("SIL OPEN FONT LICENSE Version 1.1") && body.contains("Mozilla Foundation"));
    }

    #[test]
    fn licences_embed_header_and_split_count() {
        let mut lines = NOTICES.lines();
        assert_eq!(lines.next(), Some("Third-party notices for powrush-client"));
        assert!(lines.next().is_some_and(|l| l.starts_with("Cargo.lock sha256: ")));
        let rules = NOTICES.lines().filter(|l| l.len() == 80 && l.bytes().all(|b| b == b'=')).count();
        assert_eq!(split_notices(NOTICES).len(), rules + 1);
        assert_eq!(licence_pages(false).len(), rules + 2, "every notice section, then Fira Sans");
        assert_eq!(licence_pages(false).last(), Some(&LicencePage::FiraSans));
        assert_eq!(page_text(LicencePage::Notice(0)).0, "Third-party notices for powrush-client");
    }

    #[test]
    fn every_section_renders_and_prev_next_wrap() {
        let mut app = h1(LaunchDoor::Title);
        press_entry(&mut app);
        let (pages, n) = (licence_pages(false), licence_pages(false).len());
        for (i, page) in pages.iter().enumerate() {
            let ((head, body), shown) = (page_text(*page), texts(&mut app));
            assert!(!head.is_empty() && !body.trim().is_empty(), "page {i} is blank");
            assert!(shown.contains(&format!("{} / {n} · {head}", i + 1)), "page {i} heading");
            assert!(shown.iter().any(|t| t == body), "page {i} body");
            assert_eq!(roots(&mut app).len(), 1, "one plate at a time");
            pad(&mut app, GamepadButton::RightTrigger, true);
        }
        assert_eq!(open(&app), (true, 0), "RB wraps past the last page");
        pad(&mut app, GamepadButton::LeftTrigger, true);
        assert_eq!(open(&app), (true, n - 1), "LB wraps before the first");
    }

    #[test]
    fn east_backs_out_and_dpad_scrolls() {
        let mut app = h1(LaunchDoor::InYard);
        press_entry(&mut app);
        assert_eq!(open(&app), (true, 0), "the Guide entry opens Licences");
        let mut q = app.world_mut().query_filtered::<(&mut ComputedNode, &ScrollPosition), With<LicencesBody>>();
        q.single_mut(app.world_mut()).unwrap().0.content_size = Vec2::new(100.0, 1000.0);
        pad(&mut app, GamepadButton::DPadDown, false);
        app.update();
        assert_eq!(q.single(app.world()).unwrap().1.y, 2.0 * PAD_SCROLL_PX, "held D-pad down scrolls each frame");
        pad(&mut app, GamepadButton::DPadDown, true);
        (0..5).for_each(|_| pad(&mut app, GamepadButton::DPadUp, false));
        assert_eq!(q.single(app.world()).unwrap().1.y, 0.0, "D-pad up stops at the top");
        pad(&mut app, GamepadButton::East, true);
        assert_eq!((open(&app).0, roots(&mut app).len()), (false, 0), "East goes back");
        press_entry(&mut app);
        app.insert_resource(LaunchDoor::Title).update();
        assert!(!open(&app).0, "leaving the door closes Licences");
    }

    #[test]
    fn opens_from_guide_on_title_and_yard_never_under_age_gate() {
        for door in [LaunchDoor::Title, LaunchDoor::InYard] {
            let mut app = h1(door);
            press_entry(&mut app);
            assert_eq!(roots(&mut app), [LIVED_UI_Z_PAUSE + 2], "{door:?}");
            assert!([LICENCES_TITLE, LICENCES_INTRO, LICENCES_BACK_HINT].iter().all(|w| texts(&mut app).iter().any(|t| t == w)), "{door:?}");
        }
        let mut gated = h1(LaunchDoor::Title); gated.insert_resource(AgeGate { open: true });
        press_entry(&mut gated);
        assert!(!open(&gated).0 && roots(&mut gated).is_empty(), "nothing spawns under the age gate");
        assert!(!texts(&mut gated).iter().any(|t| t == LICENCES_TITLE));
    }

    #[test]
    fn music_page_only_once_unlocked() {
        assert!(!licence_pages(false).contains(&LicencePage::Music) && licence_pages(true).last() == Some(&LicencePage::Music));
        for present in [false, true] {
            let mut app = h1(LaunchDoor::Title); app.insert_resource(MusicPresent(present));
            press_entry(&mut app);
            pad(&mut app, GamepadButton::LeftTrigger, true);
            let shown = texts(&mut app);
            let music: Vec<&String> = shown.iter().filter(|t| t.starts_with("Music: ")).collect();
            let want: &[&str] = if present { &[MUSIC_CREDITS[12].line] } else { &[] };
            assert_eq!(music, want, "only a credited track, on the last page");
            assert_eq!(shown.iter().any(|t| t.ends_with(FIRA_SANS_HEADING)), !present);
        }
    }

    /// Fixed px: max text scale leaves the chrome as is, and the plate sits inside 98%.
    #[test]
    fn licences_plate_fits_720p_at_max_text_scale() {
        let mut app = h1(LaunchDoor::Title);
        let inner = shared::local_settings::LocalSettings { text_scale: shared::local_settings::TEXT_SCALE_MAX, ..shared::local_settings::LocalSettings::peace_defaults() };
        app.insert_resource(LocalSettingsState { inner, dirty: false });
        press_entry(&mut app);
        let title = app.world_mut().query::<(&Text, &TextFont)>().iter(app.world()).find(|(t, _)| t.0 == LICENCES_TITLE).map(|(_, f)| f.font_size);
        assert_eq!(title, Some(FontSize::Px(TITLE_PX / 1.2)));
        let root = app.world_mut().query_filtered::<&Node, With<LicencesRoot>>().single(app.world()).unwrap().clone();
        assert_eq!((root.padding.top, root.padding.bottom, root.height), (Val::Percent(1.0), Val::Percent(1.0), Val::Percent(100.0)));
        let btn = NAV_PX.ceil() + BTN_PAD_Y * 2.0 + 2.0;
        let chrome = PLATE_PAD * 2.0 + PLATE_BORDER * 2.0 + TITLE_PX.ceil() + INTRO_PX.ceil() + 2.0 * btn + 4.0 * PLATE_GAP;
        let plate = 720.0 * 0.98;
        assert!(plate - chrome >= 400.0, "body keeps {}px of {plate}", plate - chrome);
        assert!(title_plate_fits_surface(Vec2::new(1280.0, 720.0), Vec2::new(PLATE_MAX_WIDTH, plate), 0.0));
    }

    /// Opaque-title law: palette tokens only, no inline colour.
    #[test]
    fn licences_prod_uses_title_tokens_only() {
        let src = include_str!("licences_screen.rs");
        let prod = src.split("\n#[cfg(test)]\nmod tests {").next().unwrap();
        assert!(prod.len() < src.len() && !prod.contains(concat!("Color::", "srgb")));
        assert!(prod.contains("GlobalZIndex(LIVED_UI_Z_PAUSE + 2)"));
    }
}
