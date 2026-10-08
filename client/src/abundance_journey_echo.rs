/*!
 * Abundance Journey Echo — soft session + durable memory (v21.96.0)
 *
 * Binds Living Practice seals and RBE allocate choices into a visible,
 * non-extractive journey log. Complements My Mercy Journey (M).
 *
 * Persistence: local `data/powrush_abundance_journey.json` (sovereign offline).
 * Loads on startup. Writes only when the serialized journey changes, through
 * `shared::user_persist::write_named` (tmp + rename + `.bak`).
 *
 * Toggle: **J** (Journey — ergonomic left-hand)
 *
 * TOLC 8 · no scarcity · Contact: info@Rathor.ai · Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::hud_anchor_registry::{HudSlab, JOURNEY};
use crate::living_practice_loop::LivingPracticeLoop;
use crate::title_screen::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY, TITLE_TEXT_SECONDARY};
use crate::rbe_allocate_choice::{AllocatePath, RbeAllocateChoice};
use crate::soft_play_bindings;

const PERSIST_PATH: &str = "data/powrush_abundance_journey.json";
const JOURNEY_ECHO_FOOTER: &str = "J to close · your journey is kept on this device";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JourneyLine {
    pub text: String,
    pub kind: JourneyKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JourneyKind {
    PracticeSeal,
    FlowOutward,
    StewardReserve,
    Note,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct JourneyPersistBlob {
    schema: String,
    lines: Vec<JourneyLine>,
    practice_sealed: bool,
    flow_total: f32,
    reserve_total: f32,
    choices_made: u32,
}

#[derive(Resource, Debug, Default)]
pub struct AbundanceJourneyEcho {
    pub lines: Vec<JourneyLine>,
    pub panel_open: bool,
    pub last_practice_sealed: bool,
    pub last_choices_seen: u32,
    /// Dirty flag for soft disk write.
    pub dirty: bool,
    pub loaded: bool,
    /// Last serialized JSON written, or the startup serialization when no
    /// file was loaded. Compared so an unchanged journey does not rewrite.
    last_saved: Option<String>,
}

impl AbundanceJourneyEcho {
    pub fn push(&mut self, kind: JourneyKind, text: impl Into<String>) {
        self.lines.push(JourneyLine {
            text: text.into(),
            kind,
        });
        if self.lines.len() > 24 {
            self.lines.remove(0);
        }
        self.dirty = true;
    }
}

fn persist_path() -> PathBuf {
    shared::user_persist::persist_path(PERSIST_PATH)
}

fn journey_blob(echo: &AbundanceJourneyEcho, allocate: &RbeAllocateChoice) -> JourneyPersistBlob {
    JourneyPersistBlob {
        schema: "powrush_abundance_journey_v1".into(),
        lines: echo.lines.clone(),
        practice_sealed: echo.last_practice_sealed,
        flow_total: allocate.flow_total,
        reserve_total: allocate.reserve_total,
        choices_made: allocate.choices_made,
    }
}

/// True when `json` is not the last serialized save.
///
/// `None` means nothing has been saved yet, so the next blob should write.
fn should_write_journey(last_saved: Option<&str>, json: &str) -> bool {
    match last_saved {
        Some(saved) => saved != json,
        None => true,
    }
}

fn load_blob() -> Option<(JourneyPersistBlob, String)> {
    let raw = shared::user_persist::read_named(PERSIST_PATH).ok()?;
    let blob = serde_json::from_str(&raw).ok()?;
    Some((blob, raw))
}

/// Atomic write via `write_named` (tmp + rename + `.bak`). Returns whether
/// the write succeeded. The warn line stays the failure log.
fn save_blob(json: &str) -> bool {
    match shared::user_persist::write_named(PERSIST_PATH, json) {
        Ok(()) => {
            let path = persist_path();
            info!(target: "powrush::journey", path = %path.display(), "journey echo saved");
            true
        }
        Err(e) => {
            warn!(target: "powrush::journey", "journey persist write failed: {e}");
            false
        }
    }
}

#[derive(Component)]
pub struct JourneyEchoRoot;

#[derive(Component)]
pub struct JourneyEchoBody;

pub struct AbundanceJourneyEchoPlugin;

impl Plugin for AbundanceJourneyEchoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AbundanceJourneyEcho>()
            .add_systems(Startup, (load_journey_persist, spawn_echo_panel).chain())
            .add_systems(
                Update,
                (
                    absorb_practice_and_allocate,
                    toggle_echo_panel,
                    update_echo_visibility,
                    update_echo_body,
                    save_journey_persist,
                ),
            );
    }
}

fn load_journey_persist(
    mut echo: ResMut<AbundanceJourneyEcho>,
    mut allocate: ResMut<RbeAllocateChoice>,
) {
    if echo.loaded {
        return;
    }
    echo.loaded = true;
    let loaded = if let Some((blob, raw)) = load_blob() {
        if blob.schema.starts_with("powrush_abundance_journey") {
            echo.lines = blob.lines;
            echo.last_practice_sealed = blob.practice_sealed;
            echo.last_choices_seen = blob.choices_made;
            allocate.flow_total = blob.flow_total;
            allocate.reserve_total = blob.reserve_total;
            allocate.choices_made = blob.choices_made;
            if blob.practice_sealed {
                info!(target: "powrush::journey", "restored practice seal from disk");
            }
            info!(
                target: "powrush::journey",
                lines = echo.lines.len(),
                flow = allocate.flow_total,
                reserve = allocate.reserve_total,
                "journey echo loaded"
            );
            Some(raw)
        } else {
            None
        }
    } else {
        None
    };
    // Seed from the file text when the schema is ours, otherwise from the
    // serialized startup state. An idle boot then matches and writes nothing.
    match loaded {
        Some(raw) => echo.last_saved = Some(raw),
        None => {
            if let Ok(json) = serde_json::to_string_pretty(&journey_blob(&echo, &allocate)) {
                echo.last_saved = Some(json);
            }
        }
    }
}

fn save_journey_persist(
    mut echo: ResMut<AbundanceJourneyEcho>,
    allocate: Res<RbeAllocateChoice>,
) {
    if allocate.is_changed() {
        echo.dirty = true;
    }
    if !echo.dirty {
        return;
    }
    let blob = journey_blob(&echo, &allocate);
    if let Ok(json) = serde_json::to_string_pretty(&blob) {
        if should_write_journey(echo.last_saved.as_deref(), &json) && save_blob(&json) {
            echo.last_saved = Some(json);
        }
    }
    echo.dirty = false;
}

fn spawn_echo_panel(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: JOURNEY.top(),
                    left: JOURNEY.left(),
                    width: Val::Px(360.0),
                    max_height: Val::Px(280.0),
                    padding: UiRect::all(Val::Px(14.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    border: UiRect::all(Val::Px(1.5)),
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                background_color: TITLE_PLATE_BG.into(),
                border_color: TITLE_BORDER.into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            JourneyEchoRoot,
            HudSlab(JOURNEY.id),
        ))
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                "ABUNDANCE JOURNEY",
                TextStyle {
                    font_size: 15.0,
                    color: TITLE_TEXT_SECONDARY,
                    ..default()
                },
            ));
            p.spawn((
                TextBundle::from_section(
                    "• Acts of thriving will echo here",
                    TextStyle {
                        font_size: 12.5,
                        color: TITLE_TEXT_PRIMARY,
                        ..default()
                    },
                ),
                JourneyEchoBody,
            ));
            p.spawn(TextBundle::from_section(
                JOURNEY_ECHO_FOOTER,
                TextStyle {
                    font_size: 11.0,
                    color: TITLE_TEXT_SECONDARY,
                    ..default()
                },
            ));
        });
}

fn absorb_practice_and_allocate(
    practice: Res<LivingPracticeLoop>,
    allocate: Res<RbeAllocateChoice>,
    mut echo: ResMut<AbundanceJourneyEcho>,
) {
    if practice.principle_sealed && !echo.last_practice_sealed {
        echo.last_practice_sealed = true;
        echo.push(
            JourneyKind::PracticeSeal,
            "Sealed Caps Across Climates — same principle, three climates",
        );
    }

    if allocate.choices_made > echo.last_choices_seen {
        echo.last_choices_seen = allocate.choices_made;
        match allocate.last_choice {
            Some(AllocatePath::FlowOutward) => echo.push(
                JourneyKind::FlowOutward,
                format!(
                    "Flowed outward · lattice share (total {:.1})",
                    allocate.flow_total
                ),
            ),
            Some(AllocatePath::StewardReserve) => echo.push(
                JourneyKind::StewardReserve,
                format!(
                    "Stewarded reserve · future thriving (total {:.1})",
                    allocate.reserve_total
                ),
            ),
            None => {}
        }
    }
}

pub(crate) fn toggle_echo_panel(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut echo: ResMut<AbundanceJourneyEcho>,
) {
    if keyboard.just_pressed(soft_play_bindings::JOURNEY_ECHO) {
        echo.panel_open = !echo.panel_open;
    }
}

fn update_echo_visibility(
    echo: Res<AbundanceJourneyEcho>,
    mut q: Query<&mut Visibility, With<JourneyEchoRoot>>,
) {
    for mut vis in &mut q {
        *vis = if echo.panel_open {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn update_echo_body(
    echo: Res<AbundanceJourneyEcho>,
    mut q: Query<&mut Text, With<JourneyEchoBody>>,
) {
    if !echo.is_changed() {
        return;
    }
    let body = if echo.lines.is_empty() {
        "• Acts of thriving will echo here".to_string()
    } else {
        echo.lines
            .iter()
            .rev()
            .take(10)
            .map(|l| {
                let mark = match l.kind {
                    JourneyKind::PracticeSeal => "(•)",
                    JourneyKind::FlowOutward => "→",
                    JourneyKind::StewardReserve => "◊",
                    JourneyKind::Note => "•",
                };
                format!("{} {}", mark, l.text)
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    for mut text in &mut q {
        if let Some(s) = text.sections.get_mut(0) {
            s.value = body.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// CARD HUD-ANCHOR-REGISTRY-2B — Journey position is the registry, Style is the coded literal.
    #[test]
    fn journey_style_byte_identical_to_coded_place() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_echo_panel);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<&Style, With<JourneyEchoRoot>>();
        let style = q.single(app.world()).clone();
        let coded = Style {
            position_type: PositionType::Absolute,
            top: Val::Percent(12.0),
            left: Val::Percent(2.0),
            width: Val::Px(360.0),
            max_height: Val::Px(280.0),
            padding: UiRect::all(Val::Px(14.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(6.0),
            border: UiRect::all(Val::Px(1.5)),
            overflow: Overflow::clip_y(),
            ..default()
        };
        assert_eq!(style, coded);
        assert_eq!(style.top, JOURNEY.top());
        assert_eq!(style.left, JOURNEY.left());
        assert_eq!(style.margin, UiRect::default());
        assert_eq!(style.width, Val::Px(JOURNEY.width));
    }

    #[test]
    fn push_caps_at_24() {
        let mut e = AbundanceJourneyEcho::default();
        for i in 0..30 {
            e.push(JourneyKind::Note, format!("n{i}"));
        }
        assert!(e.lines.len() <= 24);
    }

    #[test]
    fn blob_roundtrip_shape() {
        let blob = JourneyPersistBlob {
            schema: "powrush_abundance_journey_v1".into(),
            lines: vec![JourneyLine {
                text: "test".into(),
                kind: JourneyKind::Note,
            }],
            practice_sealed: true,
            flow_total: 2.0,
            reserve_total: 1.0,
            choices_made: 3,
        };
        let json = serde_json::to_string(&blob).unwrap();
        let back: JourneyPersistBlob = serde_json::from_str(&json).unwrap();
        assert_eq!(back.choices_made, 3);
        assert!(back.practice_sealed);
    }

    #[test]
    fn journey_echo_footer_is_plain_one_line() {
        assert!(JOURNEY_ECHO_FOOTER.starts_with("J "));
        assert!(!JOURNEY_ECHO_FOOTER.contains("TOLC"));
        assert!(JOURNEY_ECHO_FOOTER.chars().count() <= 48);

        let chars: Vec<char> = JOURNEY_ECHO_FOOTER.chars().collect();
        assert!(
            !chars
                .windows(3)
                .any(|w| w[0] == '(' && w[1].is_ascii_uppercase() && w[2] == ')'),
            "footer names a parenthesised key"
        );

        let mut singles = Vec::new();
        let mut token = String::new();
        for c in chars {
            if c.is_ascii_alphabetic() {
                token.push(c);
            } else if token.chars().count() == 1
                && token.chars().next().is_some_and(|ch| ch.is_ascii_uppercase())
            {
                singles.push(std::mem::take(&mut token));
            } else {
                token.clear();
            }
        }
        if token.chars().count() == 1
            && token.chars().next().is_some_and(|ch| ch.is_ascii_uppercase())
        {
            singles.push(token);
        }
        assert_eq!(singles, vec!["J".to_string()]);
    }

    #[test]
    fn should_write_journey_skips_same_json_and_writes_when_new() {
        let same = r#"{"schema":"powrush_abundance_journey_v1"}"#;
        assert!(!should_write_journey(Some(same), same));
        assert!(should_write_journey(
            Some(same),
            r#"{"schema":"powrush_abundance_journey_v1","choices_made":1}"#
        ));
        assert!(should_write_journey(None, same));
    }

    struct ScratchDir(std::path::PathBuf);

    impl ScratchDir {
        fn new(tag: &str) -> Self {
            let dir = std::env::temp_dir().join(format!(
                "powrush-lr03-{}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos())
                    .unwrap_or(0),
                tag
            ));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).expect("scratch");
            // A live powrush_* file keeps persist_dir from adopting cwd data/.
            std::fs::write(dir.join("powrush_seal.json"), b"{}\n").expect("seal");
            Self(dir)
        }

        fn path(&self) -> &std::path::Path {
            &self.0
        }
    }

    impl Drop for ScratchDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn with_user_dir<R>(dir: &std::path::Path, body: impl FnOnce() -> R) -> R {
        let _guard = crate::test_env::lock();
        let prev = std::env::var(shared::user_persist::USER_DIR_OVERRIDE_ENV).ok();
        std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, dir);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(body));
        match &prev {
            Some(value) => std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, value),
            None => std::env::remove_var(shared::user_persist::USER_DIR_OVERRIDE_ENV),
        }
        match result {
            Ok(value) => value,
            Err(payload) => std::panic::resume_unwind(payload),
        }
    }

    fn journey_file(dir: &std::path::Path) -> std::path::PathBuf {
        dir.join(shared::user_persist::persist_file_name(PERSIST_PATH))
    }

    fn journey_sibling(dir: &std::path::Path, suffix: &str) -> std::path::PathBuf {
        let file = journey_file(dir);
        let mut name = file.file_name().expect("file name").to_os_string();
        name.push(suffix);
        file.with_file_name(name)
    }

    fn journey_app() -> bevy::prelude::App {
        let mut app = bevy::prelude::App::new();
        app.init_resource::<AbundanceJourneyEcho>()
            .init_resource::<RbeAllocateChoice>()
            .init_resource::<crate::living_practice_loop::LivingPracticeLoop>()
            .add_systems(bevy::prelude::Startup, load_journey_persist)
            .add_systems(
                bevy::prelude::Update,
                (absorb_practice_and_allocate, save_journey_persist).chain(),
            );
        app
    }

    /// CARD LR-03 — idle boot, including allocate change ticks that do not
    /// alter the blob, must not create the journey file.
    #[test]
    fn idle_boot_does_not_write_journey_file() {
        let scratch = ScratchDir::new("idle");
        with_user_dir(scratch.path(), || {
            let mut app = journey_app();
            for _ in 0..8 {
                app.update();
            }
            {
                let mut allocate = app.world_mut().resource_mut::<RbeAllocateChoice>();
                allocate.surplus_signal = 4.0;
                allocate.panel_open = true;
                allocate.eligible = true;
            }
            for _ in 0..4 {
                app.update();
            }
            let path = journey_file(scratch.path());
            assert!(!path.exists(), "idle boot created {}", path.display());
            assert!(!journey_sibling(scratch.path(), ".bak").exists());
            assert!(!journey_sibling(scratch.path(), ".tmp").exists());
            let echo = app.world().resource::<AbundanceJourneyEcho>();
            assert!(echo.last_saved.is_some());
            assert!(!echo.dirty);
        });
    }

    /// CARD LR-03 — a practice seal and a later allocation each write in the
    /// same frame, through write_named (second write leaves `.bak`, no `.tmp`).
    #[test]
    fn practice_seal_and_allocate_save_same_frame() {
        let scratch = ScratchDir::new("change");
        with_user_dir(scratch.path(), || {
            let mut app = journey_app();
            app.update();
            assert!(!journey_file(scratch.path()).exists());

            app.world_mut()
                .resource_mut::<crate::living_practice_loop::LivingPracticeLoop>()
                .principle_sealed = true;
            app.update();

            let path = journey_file(scratch.path());
            let first = std::fs::read_to_string(&path).expect("seal write");
            assert!(first.contains("powrush_abundance_journey_v1"));
            assert!(first.contains("Sealed Caps Across Climates"));
            assert!(!journey_sibling(scratch.path(), ".bak").exists());
            assert!(!journey_sibling(scratch.path(), ".tmp").exists());
            assert_eq!(
                app.world().resource::<AbundanceJourneyEcho>().last_saved.as_deref(),
                Some(first.as_str())
            );

            {
                let mut allocate = app.world_mut().resource_mut::<RbeAllocateChoice>();
                allocate.surplus_signal = 2.0;
                allocate.apply(crate::rbe_allocate_choice::AllocatePath::FlowOutward, 1.0);
            }
            app.update();

            let second = std::fs::read_to_string(&path).expect("allocate write");
            assert!(second.contains("Flowed outward"));
            assert_ne!(second, first);
            let bak = std::fs::read_to_string(journey_sibling(scratch.path(), ".bak"))
                .expect("write_named keeps previous good as .bak");
            assert_eq!(bak, first);
            assert!(!journey_sibling(scratch.path(), ".tmp").exists());
            assert_eq!(
                app.world().resource::<AbundanceJourneyEcho>().last_saved.as_deref(),
                Some(second.as_str())
            );
        });
    }

    /// CARD LR-03 — a loaded file is the last_saved seed. Idle frames after
    /// reload must not rewrite it (no new `.bak`).
    #[test]
    fn reload_of_saved_journey_does_not_rewrite() {
        let scratch = ScratchDir::new("reload");
        let path = journey_file(scratch.path());
        with_user_dir(scratch.path(), || {
            let mut app = journey_app();
            app.update();
            app.world_mut()
                .resource_mut::<AbundanceJourneyEcho>()
                .push(JourneyKind::Note, "kept line");
            app.update();
            assert!(path.is_file());
        });
        let before = std::fs::read(&path).expect("saved");
        let bak = journey_sibling(scratch.path(), ".bak");
        let _ = std::fs::remove_file(&bak);
        with_user_dir(scratch.path(), || {
            let mut app = journey_app();
            for _ in 0..8 {
                app.update();
            }
            let echo = app.world().resource::<AbundanceJourneyEcho>();
            assert_eq!(echo.lines.len(), 1);
            assert_eq!(echo.lines[0].text, "kept line");
            assert_eq!(
                echo.last_saved.as_deref(),
                Some(std::str::from_utf8(&before).unwrap())
            );
            assert!(!echo.dirty);
        });
        assert_eq!(std::fs::read(&path).unwrap(), before);
        assert!(!bak.exists(), "idle reload must not write");
        assert!(!journey_sibling(scratch.path(), ".tmp").exists());
    }
}
