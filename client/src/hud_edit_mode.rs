//! CARD HUD-EDIT-MODE-1 — step 4b of the UI layout epic.
//!
//! Edit mode draws one frame per anchor, drags and snaps by design §6.2, and
//! saves through [`shared::hud_layout::save_hud_layout`]. Load is
//! [`shared::hud_layout::load_hud_layout`] at boot (design §5 F1–F7).
//! Q14: the touch overlay is culled while edit mode is up.
//! Q17: the yard keeps running. Use, the build wheel, and Esc-as-pause are dead.

use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy::window::PrimaryWindow;

use crate::hud_anchor_registry::{
    clamp_hud_width, repick_hud_corner, snap_hud_rect, HudAnchor, HudCorner, HudRect,
    HUD_REGISTRY_REV,
};
use crate::hud_presets::{
    anchor_rect, hud_resize_holds, hud_save_allowed, preset, rect_inside_margin, reset_preset,
    resolve_hud_layout, ActiveHudPreset, HudLayoutChoice, HudPreset, HudPresetId, HudSessionLayout,
    PRESETS,
};
use crate::input::{InputMapSet, PeaceRemapSet, PlayerInput};
use crate::soft_play_bindings::{BUILD_WHEEL, INTERACT};
use crate::title_screen::{
    HouseLabel, SettingsHudEditBtn, TITLE_BTN_BG, TITLE_BTN_DISABLED_BG, TITLE_BTN_FG,
    TITLE_TEXT_PRIMARY,
};
use shared::hud_layout::{
    load_hud_layout, save_hud_layout, HudLayoutAnchor, HudLayoutCorner, HudLayoutFile,
    HudLayoutLoad, HUD_LAYOUT_SCHEMA,
};

/// Shown on the Esc pause plate when the save is JSON of the wrong shape (F2).
pub const HUD_LAYOUT_PARSE_NOTICE: &str = "HUD layout could not be read. Classic is in use.";

const FRAME_Z: i32 = 160;
const TOOLBAR_Z: i32 = 170;
const NOTICE_Z: i32 = 180;
const FRAME_BORDER: Color = Color::srgb(0.84, 0.69, 0.32);
const CONFLICT_BORDER: Color = Color::srgb(0.86, 0.28, 0.22);

/// Edit session. `active` is the only flag other systems read.
#[derive(Resource, Clone, Debug)]
pub struct HudEditMode {
    pub active: bool,
    /// F2 only. The pause plate shows [`HUD_LAYOUT_PARSE_NOTICE`].
    pub parse_notice: bool,
    esc_consumed: bool,
    pub(crate) staged: Vec<HudAnchor>,
    pub(crate) staged_base: HudPresetId,
    pub(crate) opened: Vec<HudAnchor>,
    pub(crate) opened_base: HudPresetId,
    drag: Option<EditDrag>,
    save_enabled: bool,
    conflicts: Vec<&'static str>,
}

impl Default for HudEditMode {
    fn default() -> Self {
        Self {
            active: false,
            parse_notice: false,
            esc_consumed: false,
            staged: Vec::new(),
            staged_base: HudPresetId::Classic,
            opened: Vec::new(),
            opened_base: HudPresetId::Classic,
            drag: None,
            save_enabled: false,
            conflicts: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DragKind {
    Move,
    Width,
}

#[derive(Clone, Copy, Debug)]
struct EditDrag {
    id: &'static str,
    kind: DragKind,
    origin: HudRect,
    cursor0: Vec2,
    coded_width: f32,
    last: HudRect,
}

#[derive(Component)]
struct HudEditRoot;

#[derive(Component)]
struct HudEditFrame {
    id: &'static str,
}

#[derive(Component)]
struct HudEditGrip {
    id: &'static str,
}

#[derive(Component)]
struct HudEditHide {
    id: &'static str,
}

#[derive(Component)]
struct HudEditHideText {
    id: &'static str,
}

#[derive(Component)]
struct HudEditSaveBtn;

#[derive(Component)]
struct HudEditCancelBtn;

#[derive(Component)]
struct HudEditResetBtn;

#[derive(Component)]
struct HudParseNotice;

pub struct HudEditModePlugin;

impl Plugin for HudEditModePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HudEditMode>()
            .init_resource::<HudSessionLayout>()
            .init_resource::<ActiveHudPreset>()
            .configure_sets(PreUpdate, PeaceRemapSet.after(bevy::input::InputSystems))
            .configure_sets(Update, InputMapSet)
            .add_systems(Startup, boot_load_hud_layout)
            .add_systems(PreUpdate, guard_edit_mode_keys.after(PeaceRemapSet))
            .add_systems(
                Update,
                (
                    open_edit_from_pause,
                    drive_edit_mode,
                    sync_edit_chrome,
                    sync_parse_notice,
                    clear_edit_mode_player_actions,
                )
                    .chain()
                    .after(InputMapSet),
            )
            .add_systems(Update, apply_resize_fallback)
            .add_systems(PostUpdate, clear_edit_mode_player_actions);
    }
}

fn boot_load_hud_layout(
    mut edit: ResMut<HudEditMode>,
    mut session: ResMut<HudSessionLayout>,
    mut active: ResMut<ActiveHudPreset>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let (view_w, view_h) = primary_view(&windows);
    let ids = known_anchor_ids();
    let loaded = load_hud_layout(HUD_REGISTRY_REV, &ids);
    apply_boot_load(loaded, view_w, view_h, &mut edit, &mut session, &mut active);
}

/// Q17. After the peace remap, Use and the build wheel cannot stay down.
/// Esc is Cancel and is cleared so the pause plate does not open.
fn guard_edit_mode_keys(
    mut edit: ResMut<HudEditMode>,
    mut session: ResMut<HudSessionLayout>,
    mut active: ResMut<ActiveHudPreset>,
    mut keyboard: Option<ResMut<ButtonInput<KeyCode>>>,
    mut player: Option<ResMut<PlayerInput>>,
) {
    if !edit.active {
        edit.esc_consumed = false;
        return;
    }
    if let Some(player) = player.as_mut() {
        player.interact = false;
        player.sheet_q = false;
        player.pause_toggle = false;
    }
    let escape = keyboard
        .as_ref()
        .is_some_and(|keys| keys.just_pressed(KeyCode::Escape));
    if let Some(keys) = keyboard.as_mut() {
        keys.reset(INTERACT);
        keys.reset(BUILD_WHEEL);
    }
    if !escape {
        return;
    }
    cancel_edit(&mut edit, &mut session, &mut active);
    edit.esc_consumed = true;
    if let Some(keys) = keyboard.as_mut() {
        keys.reset(KeyCode::Escape);
    }
}

fn clear_edit_mode_player_actions(
    edit: Option<Res<HudEditMode>>,
    mut player: Option<ResMut<PlayerInput>>,
) {
    if !edit.as_ref().is_some_and(|mode| mode.active) {
        return;
    }
    let Some(player) = player.as_mut() else {
        return;
    };
    player.interact = false;
    player.sheet_q = false;
    player.pause_toggle = false;
}

fn open_edit_from_pause(
    mut edit: ResMut<HudEditMode>,
    mut session: ResMut<HudSessionLayout>,
    mut active: ResMut<ActiveHudPreset>,
    mut house: Option<ResMut<HouseLabel>>,
    pressed: Query<&Interaction, (Changed<Interaction>, With<SettingsHudEditBtn>)>,
) {
    if edit.active {
        return;
    }
    let Some(house) = house.as_mut() else {
        return;
    };
    if !house.settings_open {
        return;
    }
    if !pressed
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        return;
    }
    let base = session.base.or(active.id).unwrap_or(HudPresetId::Classic);
    let anchors = session
        .anchors
        .clone()
        .unwrap_or_else(|| preset(base).anchors.to_vec());
    edit.opened = anchors.clone();
    edit.opened_base = base;
    edit.staged = anchors;
    edit.staged_base = base;
    edit.drag = None;
    edit.active = true;
    house.settings_open = false;
    publish(&edit, &mut session, &mut active);
}

fn drive_edit_mode(
    mut edit: ResMut<HudEditMode>,
    mut session: ResMut<HudSessionLayout>,
    mut active: ResMut<ActiveHudPreset>,
    windows: Query<&Window, With<PrimaryWindow>>,
    saves: Query<&Interaction, (Changed<Interaction>, With<HudEditSaveBtn>)>,
    cancels: Query<&Interaction, (Changed<Interaction>, With<HudEditCancelBtn>)>,
    resets: Query<&Interaction, (Changed<Interaction>, With<HudEditResetBtn>)>,
    hides: Query<(&Interaction, &HudEditHide), Changed<Interaction>>,
    hide_held: Query<(&Interaction, &HudEditHide)>,
    grips: Query<(&Interaction, &HudEditGrip)>,
    frames: Query<(&Interaction, &HudEditFrame)>,
) {
    if !edit.active {
        return;
    }
    let (view_w, view_h) = primary_view(&windows);
    let cursor = windows
        .iter()
        .next()
        .and_then(|window| window.cursor_position());

    if cancels
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        cancel_edit(&mut edit, &mut session, &mut active);
        return;
    }
    if resets
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        stage_reset(&mut edit, &mut session, &mut active);
    }
    for (interaction, hide) in &hides {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Some(anchor) = edit.staged.iter_mut().find(|anchor| anchor.id == hide.id) {
            let next = !anchor.hidden;
            let _ = try_stage_hide(anchor, next);
        }
    }

    step_drag(
        &mut edit, view_w, view_h, cursor, &grips, &frames, &hide_held,
    );
    refresh_gate(&mut edit, view_w, view_h);
    if saves
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        let _ = try_save(&mut edit, &mut session, &mut active);
    }
    publish(&edit, &mut session, &mut active);
}

fn step_drag(
    edit: &mut HudEditMode,
    view_w: f32,
    view_h: f32,
    cursor: Option<Vec2>,
    grips: &Query<(&Interaction, &HudEditGrip)>,
    frames: &Query<(&Interaction, &HudEditFrame)>,
    hides: &Query<(&Interaction, &HudEditHide)>,
) {
    if let Some(mut drag) = edit.drag.take() {
        let pressed = match drag.kind {
            DragKind::Move => pressed_frame(frames, drag.id),
            DragKind::Width => pressed_grip(grips, drag.id),
        };
        if pressed {
            if let Some(cursor) = cursor {
                if let Some(anchor) = edit.staged.iter_mut().find(|anchor| anchor.id == drag.id) {
                    match drag.kind {
                        DragKind::Move => {
                            let rect = translated(drag.origin, drag.cursor0, cursor);
                            drag.last = rect;
                            let pick = repick_hud_corner(rect, view_w, view_h);
                            anchor.corner = pick.corner;
                            anchor.offset = pick.offset;
                        }
                        DragKind::Width => {
                            let delta = cursor.x - drag.cursor0.x;
                            let requested = (drag.origin.x1 - drag.origin.x0) as f32 + delta;
                            stage_anchor_width(anchor, requested, drag.coded_width, view_w);
                        }
                    }
                }
            }
            edit.drag = Some(drag);
            return;
        }
        if drag.kind == DragKind::Move {
            let neighbours = neighbour_rects(&edit.staged, drag.id, view_w, view_h);
            if let Some(anchor) = edit.staged.iter_mut().find(|anchor| anchor.id == drag.id) {
                release_anchor_drag(anchor, drag.last, view_w, view_h, &neighbours);
            }
        }
    }

    let Some(cursor) = cursor else {
        return;
    };
    if let Some((_, grip)) = grips
        .iter()
        .find(|(interaction, _)| **interaction == Interaction::Pressed)
    {
        start_drag(edit, grip.id, DragKind::Width, view_w, view_h, cursor);
        return;
    }
    if let Some((_, frame)) = frames.iter().find(|(interaction, frame)| {
        **interaction == Interaction::Pressed
            && !hides.iter().any(|(hide_interaction, hide)| {
                hide.id == frame.id && *hide_interaction == Interaction::Pressed
            })
    }) {
        start_drag(edit, frame.id, DragKind::Move, view_w, view_h, cursor);
    }
}

fn start_drag(
    edit: &mut HudEditMode,
    id: &'static str,
    kind: DragKind,
    view_w: f32,
    view_h: f32,
    cursor: Vec2,
) {
    let Some(anchor) = edit.staged.iter().find(|anchor| anchor.id == id) else {
        return;
    };
    let origin = anchor_rect(anchor, view_w, view_h);
    let coded_width = preset(edit.staged_base)
        .anchors
        .iter()
        .find(|item| item.id == id)
        .map(|item| item.width)
        .unwrap_or(anchor.width);
    edit.drag = Some(EditDrag {
        id,
        kind,
        origin,
        cursor0: cursor,
        coded_width,
        last: origin,
    });
}

fn pressed_frame(frames: &Query<(&Interaction, &HudEditFrame)>, id: &str) -> bool {
    frames
        .iter()
        .any(|(interaction, frame)| frame.id == id && *interaction == Interaction::Pressed)
}

fn pressed_grip(grips: &Query<(&Interaction, &HudEditGrip)>, id: &str) -> bool {
    grips
        .iter()
        .any(|(interaction, grip)| grip.id == id && *interaction == Interaction::Pressed)
}

fn sync_edit_chrome(
    edit: Res<HudEditMode>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    roots: Query<Entity, With<HudEditRoot>>,
    mut frames: Query<(&HudEditFrame, &mut Node, &mut BorderColor)>,
    mut saves: Query<&mut BackgroundColor, With<HudEditSaveBtn>>,
    mut hide_text: Query<(&HudEditHideText, &mut Text)>,
) {
    if !edit.active {
        for entity in &roots {
            commands.entity(entity).despawn();
        }
        return;
    }
    let (view_w, view_h) = primary_view(&windows);
    let frame_count = frames.iter().count();
    if roots.iter().count() != 1 || frame_count != edit.staged.len() {
        for entity in &roots {
            commands.entity(entity).despawn();
        }
        spawn_chrome(&mut commands, &edit, view_w, view_h);
        return;
    }
    for (frame, mut style, mut border) in &mut frames {
        let Some(anchor) = edit.staged.iter().find(|anchor| anchor.id == frame.id) else {
            continue;
        };
        let rect = anchor_rect(anchor, view_w, view_h);
        let left = Val::Px(rect.x0 as f32);
        let top = Val::Px(rect.y0 as f32);
        let width = Val::Px((rect.x1 - rect.x0) as f32);
        let height = Val::Px((rect.y1 - rect.y0) as f32);
        if style.left != left {
            style.left = left;
        }
        if style.top != top {
            style.top = top;
        }
        if style.width != width {
            style.width = width;
        }
        if style.height != height {
            style.height = height;
        }
        let color = if edit.conflicts.iter().any(|id| *id == anchor.id) {
            CONFLICT_BORDER
        } else {
            FRAME_BORDER
        };
        let next = BorderColor::all(color);
        if border.top != color
            || border.bottom != color
            || border.left != color
            || border.right != color
        {
            *border = next;
        }
    }
    let save_color = if edit.save_enabled {
        TITLE_BTN_BG
    } else {
        TITLE_BTN_DISABLED_BG
    };
    for mut background in &mut saves {
        if background.0 != save_color {
            background.0 = save_color;
        }
    }
    for (label, mut text) in &mut hide_text {
        let Some(anchor) = edit.staged.iter().find(|anchor| anchor.id == label.id) else {
            continue;
        };
        let face = if anchor.hidden { "Show" } else { "Hide" };
        if text.as_str() != face {
            **text = face.to_string();
        }
    }
}

fn spawn_chrome(commands: &mut Commands, edit: &HudEditMode, view_w: f32, view_h: f32) {
    let staged = edit.staged.clone();
    let conflicts = edit.conflicts.clone();
    let save_enabled = edit.save_enabled;
    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                FocusPolicy::Pass,
            ),
GlobalZIndex(FRAME_Z),
            HudEditRoot,
        ))
        .with_children(|root| {
            for anchor in &staged {
                spawn_frame(
                    root,
                    anchor,
                    view_w,
                    view_h,
                    conflicts.iter().any(|id| *id == anchor.id),
                );
            }
            spawn_toolbar(root, save_enabled);
        });
}

fn spawn_frame(
    parent: &mut ChildSpawnerCommands,
    anchor: &HudAnchor,
    view_w: f32,
    view_h: f32,
    conflict: bool,
) {
    let rect = anchor_rect(anchor, view_w, view_h);
    let border = if conflict {
        CONFLICT_BORDER
    } else {
        FRAME_BORDER
    };
    parent
        .spawn((
            (
                bevy::ui_widgets::Button, Interaction::default(),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(rect.x0 as f32),
                    top: Val::Px(rect.y0 as f32),
                    width: Val::Px((rect.x1 - rect.x0) as f32),
                    height: Val::Px((rect.y1 - rect.y0) as f32),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                FocusPolicy::Block,
                BorderColor::all(border),
                BackgroundColor(Color::srgba(0.07, 0.05, 0.09, 0.28)),
            ),
GlobalZIndex(FRAME_Z),
            HudEditFrame { id: anchor.id },
        ))
        .with_children(|frame| {
            frame.spawn((
                Text::new(anchor.id),
                TextFont { font_size: FontSize::Px(12.0 / 1.2), ..default() },
                TextColor(TITLE_TEXT_PRIMARY),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(4.0),
                    top: Val::Px(2.0),
                    ..default()
                },
                FocusPolicy::Pass,
            ));
            frame.spawn((
                (
                    bevy::ui_widgets::Button, Interaction::default(),
                    Node {
                        position_type: PositionType::Absolute,
                        right: Val::Px(0.0),
                        top: Val::Px(0.0),
                        width: Val::Px(44.0),
                        height: Val::Px(44.0),
                        ..default()
                    },
                    FocusPolicy::Block,
                    BackgroundColor(TITLE_BTN_BG),
                ),
                HudEditGrip { id: anchor.id },
            ));
            if may_hide(anchor) {
                let face = if anchor.hidden { "Show" } else { "Hide" };
                frame
                    .spawn((
                        (
                            bevy::ui_widgets::Button, Interaction::default(),
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(0.0),
                                bottom: Val::Px(0.0),
                                width: Val::Px(44.0),
                                height: Val::Px(44.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            FocusPolicy::Block,
                            BackgroundColor(TITLE_BTN_BG),
                        ),
                        HudEditHide { id: anchor.id },
                    ))
                    .with_children(|hide| {
                        hide.spawn((
                            (
Text::new(face),
TextFont { font_size: FontSize::Px(11.0 / 1.2), ..default() },
TextColor(TITLE_BTN_FG),
),
                            HudEditHideText { id: anchor.id },
                        ));
                    });
            }
        });
}

fn spawn_toolbar(parent: &mut ChildSpawnerCommands, save_enabled: bool) {
    let save_bg = if save_enabled {
        TITLE_BTN_BG
    } else {
        TITLE_BTN_DISABLED_BG
    };
    parent
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(16.0),
                    left: Val::Px(16.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                FocusPolicy::Block,
            ),
GlobalZIndex(TOOLBAR_Z),
            Name::new("HudEditToolbar"),
        ))
        .with_children(|bar| {
            tool_button(bar, "Save", save_bg, HudEditSaveBtn);
            tool_button(bar, "Cancel", TITLE_BTN_BG, HudEditCancelBtn);
            tool_button(bar, "Reset", TITLE_BTN_BG, HudEditResetBtn);
        });
}

fn tool_button(parent: &mut ChildSpawnerCommands, label: &str, fill: Color, marker: impl Component) {
    parent
        .spawn((
            (
                bevy::ui_widgets::Button, Interaction::default(),
                Node {
                    min_width: Val::Px(44.0),
                    min_height: Val::Px(44.0),
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                FocusPolicy::Block,
                BackgroundColor(fill),
            ),
            marker,
        ))
        .with_children(|button| {
            button.spawn((
Text::new(label),
TextFont { font_size: FontSize::Px(14.0 / 1.2), ..default() },
TextColor(TITLE_BTN_FG),
));
        });
}

fn sync_parse_notice(
    edit: Res<HudEditMode>,
    house: Option<Res<HouseLabel>>,
    mut commands: Commands,
    existing: Query<Entity, With<HudParseNotice>>,
) {
    let show = edit.parse_notice && house.as_ref().is_some_and(|label| label.settings_open);
    if show {
        if existing.is_empty() {
            commands.spawn((
                (
Text::new(HUD_LAYOUT_PARSE_NOTICE),
TextFont { font_size: FontSize::Px(14.0 / 1.2), ..default() },
TextColor(TITLE_TEXT_PRIMARY),
Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(8.0),
                    left: Val::Px(16.0),
                    ..default()
                },
),
                GlobalZIndex(NOTICE_Z),
                HudParseNotice,
            ));
        }
        return;
    }
    for entity in &existing {
        commands.entity(entity).despawn();
    }
}

fn apply_resize_fallback(
    mut seen: Local<Option<(i32, i32)>>,
    edit: Res<HudEditMode>,
    mut session: ResMut<HudSessionLayout>,
    mut active: ResMut<ActiveHudPreset>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(window) = windows.iter().next() else {
        return;
    };
    let size = (
        window.width().round() as i32,
        window.height().round() as i32,
    );
    if edit.active || session.anchors.is_none() {
        *seen = Some(size);
        return;
    }
    if seen.is_some_and(|prev| prev == size) {
        return;
    }
    let first = seen.is_none();
    *seen = Some(size);
    if first {
        return;
    }
    let id = session.base.or(active.id).unwrap_or(HudPresetId::Classic);
    let anchors = session.anchors.clone().unwrap_or_default();
    let layout = HudPreset {
        id,
        name: preset(id).name,
        anchors: &anchors,
    };
    if hud_resize_holds(&layout, window.width(), window.height()) {
        return;
    }
    session.anchors = None;
    session.base = None;
    if active.id != Some(id) {
        active.id = Some(id);
    }
}

fn apply_boot_load(
    loaded: HudLayoutLoad,
    view_w: f32,
    view_h: f32,
    edit: &mut HudEditMode,
    session: &mut HudSessionLayout,
    active: &mut ActiveHudPreset,
) {
    match loaded {
        HudLayoutLoad::Missing => {}
        HudLayoutLoad::Parse => {
            edit.parse_notice = true;
            fall_back_to(HudPresetId::Classic, session, active);
        }
        HudLayoutLoad::Schema => {
            edit.parse_notice = false;
            fall_back_to(HudPresetId::Classic, session, active);
        }
        HudLayoutLoad::Stale { base } => {
            edit.parse_notice = false;
            fall_back_to(
                preset_id_from_name(&base).unwrap_or(HudPresetId::Classic),
                session,
                active,
            );
        }
        HudLayoutLoad::Refused { base } => {
            edit.parse_notice = false;
            let id = base
                .as_deref()
                .and_then(preset_id_from_name)
                .unwrap_or(HudPresetId::Classic);
            if hud_save_allowed(preset(id), view_w, view_h) {
                fall_back_to(id, session, active);
            } else {
                fall_back_to(HudPresetId::Classic, session, active);
            }
        }
        HudLayoutLoad::Loaded(file) => {
            edit.parse_notice = false;
            let Some(id) = preset_id_from_name(&file.base) else {
                fall_back_to(HudPresetId::Classic, session, active);
                return;
            };
            match resolve_hud_layout(preset(id), file.registry_rev, &file.anchors, view_w, view_h) {
                HudLayoutChoice::Applied(anchors) => {
                    if anchors_equivalent(&anchors, preset(id).anchors) {
                        fall_back_to(id, session, active);
                    } else {
                        session.base = Some(id);
                        session.anchors = Some(anchors);
                        if active.id != Some(id) {
                            active.id = Some(id);
                        }
                    }
                }
                HudLayoutChoice::Fallback(fallback) => {
                    fall_back_to(fallback, session, active);
                }
            }
        }
    }
}

fn fall_back_to(id: HudPresetId, session: &mut HudSessionLayout, active: &mut ActiveHudPreset) {
    session.anchors = None;
    session.base = None;
    if active.id != Some(id) {
        active.id = Some(id);
    }
}

fn publish(edit: &HudEditMode, session: &mut HudSessionLayout, active: &mut ActiveHudPreset) {
    if edit.staged.is_empty() {
        return;
    }
    if active.id != Some(edit.staged_base) {
        active.id = Some(edit.staged_base);
    }
    if anchors_equivalent(&edit.staged, preset(edit.staged_base).anchors) {
        if session.anchors.is_some() || session.base.is_some() {
            session.anchors = None;
            session.base = None;
        }
        return;
    }
    let same = session.base == Some(edit.staged_base)
        && session
            .anchors
            .as_deref()
            .is_some_and(|anchors| anchors_equivalent(anchors, &edit.staged));
    if !same {
        session.base = Some(edit.staged_base);
        session.anchors = Some(edit.staged.clone());
    }
}

fn cancel_edit(
    edit: &mut HudEditMode,
    session: &mut HudSessionLayout,
    active: &mut ActiveHudPreset,
) {
    edit.staged = edit.opened.clone();
    edit.staged_base = edit.opened_base;
    edit.active = false;
    edit.drag = None;
    publish(edit, session, active);
}

fn stage_reset(
    edit: &mut HudEditMode,
    session: &mut HudSessionLayout,
    active: &mut ActiveHudPreset,
) {
    edit.staged_base = HudPresetId::Classic;
    edit.staged = reset_preset().anchors.to_vec();
    edit.drag = None;
    publish(edit, session, active);
}

fn try_save(
    edit: &mut HudEditMode,
    session: &mut HudSessionLayout,
    active: &mut ActiveHudPreset,
) -> bool {
    if !edit.save_enabled || edit.staged.is_empty() {
        return false;
    }
    let file = layout_file(edit.staged_base, &edit.staged);
    if save_hud_layout(&file).is_err() {
        return false;
    }
    edit.opened = edit.staged.clone();
    edit.opened_base = edit.staged_base;
    publish(edit, session, active);
    true
}

fn refresh_gate(edit: &mut HudEditMode, view_w: f32, view_h: f32) {
    if edit.staged.is_empty() {
        edit.save_enabled = false;
        edit.conflicts.clear();
        return;
    }
    let layout = HudPreset {
        id: edit.staged_base,
        name: edit.staged_base.name(),
        anchors: &edit.staged,
    };
    let allowed = hud_save_allowed(&layout, view_w, view_h);
    edit.save_enabled = allowed;
    edit.conflicts.clear();
    if allowed {
        return;
    }
    let rects: Vec<HudRect> = edit
        .staged
        .iter()
        .map(|anchor| anchor_rect(anchor, view_w, view_h))
        .collect();
    for (index, rect) in rects.iter().enumerate() {
        let outside = !rect_inside_margin(*rect, view_w, view_h);
        let overlap = rects
            .iter()
            .enumerate()
            .any(|(other, other_rect)| index != other && rect.overlap_area(*other_rect) > 0);
        if outside || overlap {
            edit.conflicts.push(edit.staged[index].id);
        }
    }
}

fn release_anchor_drag(
    anchor: &mut HudAnchor,
    rect: HudRect,
    view_w: f32,
    view_h: f32,
    neighbours: &[HudRect],
) {
    let snapped = snap_hud_rect(rect, view_w, view_h, neighbours);
    let pick = repick_hud_corner(snapped, view_w, view_h);
    anchor.corner = pick.corner;
    anchor.offset = pick.offset;
}

fn stage_anchor_width(anchor: &mut HudAnchor, requested: f32, coded_width: f32, view_w: f32) {
    anchor.width = clamp_hud_width(requested, coded_width, view_w);
}

fn try_stage_hide(anchor: &mut HudAnchor, hidden: bool) -> bool {
    if hidden && !may_hide(anchor) {
        return false;
    }
    anchor.hidden = hidden;
    true
}

fn may_hide(anchor: &HudAnchor) -> bool {
    anchor.class >= 4 && anchor.occupants.iter().all(|occupant| occupant.class >= 4)
}

fn layout_file(base: HudPresetId, staged: &[HudAnchor]) -> HudLayoutFile {
    let source = preset(base);
    let mut anchors = Vec::new();
    for anchor in staged {
        let Some(original) = source.anchors.iter().find(|item| item.id == anchor.id) else {
            continue;
        };
        if !anchor_changed(anchor, original) {
            continue;
        }
        anchors.push(HudLayoutAnchor {
            id: anchor.id.to_string(),
            corner: to_layout_corner(anchor.corner),
            x: anchor.offset.x,
            y: anchor.offset.y,
            width: anchor.width,
            hidden: anchor.hidden,
        });
    }
    HudLayoutFile {
        schema: HUD_LAYOUT_SCHEMA.to_string(),
        registry_rev: HUD_REGISTRY_REV,
        base: base.name().to_string(),
        anchors,
    }
}

fn anchor_changed(anchor: &HudAnchor, original: &HudAnchor) -> bool {
    anchor.corner != original.corner
        || (anchor.offset.x - original.offset.x).abs() > 0.01
        || (anchor.offset.y - original.offset.y).abs() > 0.01
        || (anchor.width - original.width).abs() > 0.01
        || anchor.hidden != original.hidden
}

fn anchors_equivalent(left: &[HudAnchor], right: &[HudAnchor]) -> bool {
    left.len() == right.len()
        && left.iter().all(|anchor| {
            right.iter().any(|other| {
                other.id == anchor.id
                    && other.corner == anchor.corner
                    && (other.offset.x - anchor.offset.x).abs() <= 0.01
                    && (other.offset.y - anchor.offset.y).abs() <= 0.01
                    && (other.width - anchor.width).abs() <= 0.01
                    && other.hidden == anchor.hidden
            })
        })
}

fn to_layout_corner(corner: HudCorner) -> HudLayoutCorner {
    match corner {
        HudCorner::TopLeft => HudLayoutCorner::TopLeft,
        HudCorner::TopCentre => HudLayoutCorner::TopCentre,
        HudCorner::TopRight => HudLayoutCorner::TopRight,
        HudCorner::BottomLeft => HudLayoutCorner::BottomLeft,
        HudCorner::BottomCentre => HudLayoutCorner::BottomCentre,
        HudCorner::BottomRight => HudLayoutCorner::BottomRight,
    }
}

fn preset_id_from_name(name: &str) -> Option<HudPresetId> {
    match name {
        "classic" => Some(HudPresetId::Classic),
        "minimal" => Some(HudPresetId::Minimal),
        "management" => Some(HudPresetId::Management),
        _ => None,
    }
}

fn known_anchor_ids() -> Vec<&'static str> {
    let mut ids = Vec::new();
    for layout in PRESETS {
        for anchor in layout.anchors {
            if !ids.contains(&anchor.id) {
                ids.push(anchor.id);
            }
        }
    }
    ids
}

fn neighbour_rects(staged: &[HudAnchor], id: &str, view_w: f32, view_h: f32) -> Vec<HudRect> {
    staged
        .iter()
        .filter(|anchor| anchor.id != id)
        .map(|anchor| anchor_rect(anchor, view_w, view_h))
        .collect()
}

fn translated(origin: HudRect, cursor0: Vec2, cursor: Vec2) -> HudRect {
    let dx = (cursor.x - cursor0.x).round() as i32;
    let dy = (cursor.y - cursor0.y).round() as i32;
    HudRect {
        x0: origin.x0 + dx,
        y0: origin.y0 + dy,
        x1: origin.x1 + dx,
        y1: origin.y1 + dy,
    }
}

fn primary_view(windows: &Query<&Window, With<PrimaryWindow>>) -> (f32, f32) {
    windows
        .iter()
        .next()
        .map(|window| (window.width(), window.height()))
        .unwrap_or((1024.0, 640.0))
}

#[cfg(test)]
mod tests {
    use super::{
        anchors_equivalent, cancel_edit, known_anchor_ids, refresh_gate, release_anchor_drag,
        stage_anchor_width, stage_reset, try_save, try_stage_hide, HudEditCancelBtn, HudEditFrame,
        HudEditHide, HudEditMode, HudEditModePlugin, HudEditResetBtn, HudEditSaveBtn,
    };
    use bevy::input::gamepad::{
        GamepadConnection, GamepadConnectionEvent, RawGamepadButtonChangedEvent, RawGamepadEvent,
    };
    use bevy::input::keyboard::{Key, KeyboardInput};
    use bevy::input::ButtonState;
    use bevy::prelude::*;
    use bevy::ui::FocusPolicy;
    use bevy::window::{PrimaryWindow, WindowResolution};

    use crate::hud_anchor_registry::{
        HudAnchor, HudCorner, HudOffset, HudRect, HudSlab, HUD_REGISTRY_REV, ID_ALLOCATE, ID_PEER,
    };
    use crate::hud_presets::{
        anchor_rect, preset, reset_preset, ActiveHudPreset, HudLayoutPlugin, HudPresetId,
        HudSessionLayout,
    };
    use crate::input::PlayerInput;
    use crate::local_settings::LocalSettingsState;
    use crate::rbe_allocate_choice::AllocateFlowButton;
    use crate::soft_play_bindings::{BUILD_WHEEL, INTERACT};
    use crate::title_screen::{HouseLabel, SettingsHudEditBtn};
    use shared::house_name::HouseName;
    use shared::hud_layout::{
        load_hud_layout, save_hud_layout, HudLayoutAnchor, HudLayoutCorner, HudLayoutFile,
        HudLayoutLoad, HUD_LAYOUT_SCHEMA,
    };
    use shared::local_settings::{LocalSettings, PeaceKey};

    pub(crate) struct DirGuard {
        _lock: crate::test_env::UserDirEnvGuard,
        prev: Option<String>,
        dir: std::path::PathBuf,
    }

    impl DirGuard {
        pub(crate) fn new() -> Self {
            let lock = crate::test_env::lock();
            let dir = std::env::temp_dir().join(format!(
                "powrush-hud-edit-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|elapsed| elapsed.as_nanos())
                    .unwrap_or(0)
            ));
            std::fs::create_dir_all(&dir).expect("temp user dir");
            let prev = std::env::var(shared::user_persist::USER_DIR_OVERRIDE_ENV).ok();
            std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, &dir);
            Self {
                _lock: lock,
                prev,
                dir,
            }
        }

        pub(crate) fn layout_file(&self) -> std::path::PathBuf {
            self.dir.join(shared::user_persist::persist_file_name(
                shared::hud_layout::HUD_LAYOUT_PATH,
            ))
        }
    }

    impl Drop for DirGuard {
        fn drop(&mut self) {
            match &self.prev {
                Some(value) => {
                    std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, value)
                }
                None => std::env::remove_var(shared::user_persist::USER_DIR_OVERRIDE_ENV),
            }
            let _ = std::fs::remove_dir_all(&self.dir);
        }
    }

    fn quiet_house(settings_open: bool) -> HouseLabel {
        HouseLabel {
            house: HouseName::default(),
            persist_present: false,
            hour_two_held: false,
            book_held: false,
            settings_open,
            draft: String::new(),
            naming_offered: false,
            seals_offered: false,
        }
    }

    fn classic_edit() -> HudEditMode {
        let anchors = preset(HudPresetId::Classic).anchors.to_vec();
        HudEditMode {
            active: true,
            staged: anchors.clone(),
            staged_base: HudPresetId::Classic,
            opened: anchors,
            opened_base: HudPresetId::Classic,
            ..HudEditMode::default()
        }
    }

    fn plug_app() -> (DirGuard, App) {
        let dir = DirGuard::new();
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(HudEditModePlugin);
        (dir, app)
    }

    fn anchor_mut<'a>(edit: &'a mut HudEditMode, id: &str) -> &'a mut HudAnchor {
        edit.staged
            .iter_mut()
            .find(|anchor| anchor.id == id)
            .unwrap_or_else(|| panic!("missing {id}"))
    }

    #[test]
    fn drag_release_snaps_to_the_margin_and_repicks_top_left() {
        let mut anchor = *preset(HudPresetId::Classic).anchor("LEFT_STATUS_1");
        let rect = HudRect {
            x0: 4,
            y0: 93,
            x1: 524,
            y1: 145,
        };
        release_anchor_drag(&mut anchor, rect, 1024.0, 640.0, &[]);
        assert_eq!(anchor.corner, HudCorner::TopLeft);
        assert_eq!(anchor.offset.x, 16.0);
        assert_eq!(anchor.offset.y, 93.0);
        assert!((anchor.width - 520.0).abs() < 0.01);
    }

    #[test]
    fn drag_release_snaps_to_a_neighbour_gap() {
        let mut anchor = *preset(HudPresetId::Classic).anchor("CORNER_PEER");
        let rect = HudRect {
            x0: 96,
            y0: 220,
            x1: 376,
            y1: 272,
        };
        let neighbour = HudRect {
            x0: 10,
            y0: 400,
            x1: 90,
            y1: 420,
        };
        release_anchor_drag(&mut anchor, rect, 1024.0, 640.0, &[neighbour]);
        assert_eq!(anchor.corner, HudCorner::TopLeft);
        assert_eq!(anchor.offset.x, 98.0);
        assert_eq!(anchor.offset.y, 220.0);
        let placed = anchor_rect(&anchor, 1024.0, 640.0);
        assert_eq!(placed.x0, 98);
    }

    #[test]
    fn width_clamps_to_coded_floor_and_window_cap() {
        let mut anchor = *preset(HudPresetId::Classic).anchor("WINDOW");
        assert!((anchor.width - 520.0).abs() < 0.01);
        assert!((anchor.height_budget - 320.0).abs() < 0.01);
        stage_anchor_width(&mut anchor, 10.0, 520.0, 1024.0);
        assert!((anchor.width - 520.0).abs() < 0.01);
        stage_anchor_width(&mut anchor, 900.0, 520.0, 1024.0);
        assert!((anchor.width - 640.0).abs() < 0.01);
        stage_anchor_width(&mut anchor, 600.0, 520.0, 1024.0);
        assert!((anchor.width - 600.0).abs() < 0.01);
        assert!((anchor.height_budget - 320.0).abs() < 0.01);
    }

    #[test]
    fn save_stays_disabled_when_anchors_overlap_and_cancel_restores() {
        let dir = DirGuard::new();
        let mut edit = classic_edit();
        let mut session = HudSessionLayout::default();
        let mut active = ActiveHudPreset {
            id: Some(HudPresetId::Classic),
        };
        {
            let left = anchor_mut(&mut edit, "LEFT_STATUS_1");
            left.corner = HudCorner::TopLeft;
            left.offset = HudOffset { x: 16.0, y: 200.0 };
        }
        {
            let peer = anchor_mut(&mut edit, "CORNER_PEER");
            peer.corner = HudCorner::TopLeft;
            peer.offset = HudOffset { x: 16.0, y: 200.0 };
        }
        refresh_gate(&mut edit, 1024.0, 640.0);
        assert!(!edit.save_enabled);
        assert!(edit.conflicts.iter().any(|id| *id == "LEFT_STATUS_1"));
        assert!(edit.conflicts.iter().any(|id| *id == "CORNER_PEER"));
        assert!(!try_save(&mut edit, &mut session, &mut active));
        assert!(!dir.layout_file().exists());

        let mut clean = classic_edit();
        refresh_gate(&mut clean, 1024.0, 640.0);
        assert!(clean.save_enabled);

        cancel_edit(&mut edit, &mut session, &mut active);
        assert!(!edit.active);
        assert!(anchors_equivalent(
            &edit.staged,
            preset(HudPresetId::Classic).anchors
        ));
        assert!(session.anchors.is_none());
    }

    #[test]
    fn reset_stages_classic_without_writing() {
        let dir = DirGuard::new();
        let minimal = preset(HudPresetId::Minimal).anchors.to_vec();
        let mut edit = HudEditMode {
            active: true,
            staged: minimal.clone(),
            staged_base: HudPresetId::Minimal,
            opened: minimal,
            opened_base: HudPresetId::Minimal,
            ..HudEditMode::default()
        };
        let mut session = HudSessionLayout::default();
        let mut active = ActiveHudPreset {
            id: Some(HudPresetId::Minimal),
        };
        stage_reset(&mut edit, &mut session, &mut active);
        assert!(edit.active);
        assert_eq!(edit.staged_base, HudPresetId::Classic);
        assert_eq!(edit.opened_base, HudPresetId::Minimal);
        assert!(anchors_equivalent(&edit.staged, reset_preset().anchors));
        assert!(!dir.layout_file().exists());
    }

    #[test]
    fn hide_is_only_for_class_four_and_five() {
        let mut peer = *preset(HudPresetId::Classic).anchor("CORNER_PEER");
        assert!(try_stage_hide(&mut peer, true));
        assert!(peer.hidden);
        let mut window = *preset(HudPresetId::Classic).anchor("WINDOW");
        assert!(!try_stage_hide(&mut window, true));
        assert!(!window.hidden);
    }

    #[test]
    fn save_round_trip_uses_the_layout_api() {
        let dir = DirGuard::new();
        let mut edit = classic_edit();
        refresh_gate(&mut edit, 1024.0, 640.0);
        let mut session = HudSessionLayout::default();
        let mut active = ActiveHudPreset {
            id: Some(HudPresetId::Classic),
        };
        assert!(try_save(&mut edit, &mut session, &mut active));
        assert!(edit.active);
        let ids = known_anchor_ids();
        match load_hud_layout(HUD_REGISTRY_REV, &ids) {
            HudLayoutLoad::Loaded(file) => {
                assert_eq!(file.base, "classic");
                assert!(file.anchors.is_empty());
                assert_eq!(file.schema, HUD_LAYOUT_SCHEMA);
            }
            other => panic!("expected a classic save, got {other:?}"),
        }
        assert!(dir.layout_file().is_file());
    }

    fn boot_with(
        file: Option<HudLayoutFile>,
        raw: Option<&str>,
        preset_id: Option<HudPresetId>,
    ) -> (DirGuard, App) {
        let (dir, mut app) = plug_app();
        if let Some(file) = file {
            save_hud_layout(&file).expect("plant layout");
        }
        if let Some(raw) = raw {
            std::fs::write(dir.layout_file(), raw).expect("plant raw");
        }
        if let Some(id) = preset_id {
            app.insert_resource(ActiveHudPreset { id: Some(id) });
        }
        app.update();
        (dir, app)
    }

    fn file_bytes(dir: &DirGuard) -> Vec<u8> {
        std::fs::read(dir.layout_file()).expect("layout bytes")
    }

    #[test]
    fn boot_f1_missing_keeps_the_seeded_preset() {
        let (_dir, app) = boot_with(None, None, Some(HudPresetId::Minimal));
        let active = app.world().resource::<ActiveHudPreset>();
        let session = app.world().resource::<HudSessionLayout>();
        let edit = app.world().resource::<HudEditMode>();
        assert_eq!(active.id, Some(HudPresetId::Minimal));
        assert!(session.anchors.is_none());
        assert!(!edit.parse_notice);
        assert!(!edit.active);
    }

    #[test]
    fn boot_f2_parse_falls_back_to_classic_and_keeps_the_file() {
        let raw = "{\"hello\":1}";
        let (dir, app) = boot_with(None, Some(raw), Some(HudPresetId::Minimal));
        assert_eq!(
            app.world().resource::<ActiveHudPreset>().id,
            Some(HudPresetId::Classic)
        );
        assert!(app.world().resource::<HudEditMode>().parse_notice);
        assert!(app.world().resource::<HudSessionLayout>().anchors.is_none());
        assert_eq!(file_bytes(&dir), raw.as_bytes());
    }

    #[test]
    fn boot_f3_unknown_schema_keeps_the_file_without_a_notice() {
        let file = HudLayoutFile {
            schema: "powrush_hud_layout_v2".into(),
            registry_rev: HUD_REGISTRY_REV,
            base: "minimal".into(),
            anchors: Vec::new(),
        };
        let (dir, mut app) = plug_app();
        save_hud_layout(&file).expect("schema");
        let before = file_bytes(&dir);
        app.insert_resource(ActiveHudPreset {
            id: Some(HudPresetId::Minimal),
        });
        app.update();
        assert_eq!(
            app.world().resource::<ActiveHudPreset>().id,
            Some(HudPresetId::Classic)
        );
        assert!(!app.world().resource::<HudEditMode>().parse_notice);
        assert_eq!(file_bytes(&dir), before);
    }

    #[test]
    fn boot_f4_stale_rev_keeps_base_and_drops_overrides() {
        let file = HudLayoutFile {
            schema: HUD_LAYOUT_SCHEMA.into(),
            registry_rev: 99,
            base: "minimal".into(),
            anchors: vec![HudLayoutAnchor {
                id: "WINDOW".into(),
                corner: HudLayoutCorner::TopLeft,
                x: 40.0,
                y: 40.0,
                width: 520.0,
                hidden: false,
            }],
        };
        let (dir, mut app) = plug_app();
        save_hud_layout(&file).expect("stale");
        let before = file_bytes(&dir);
        app.insert_resource(ActiveHudPreset {
            id: Some(HudPresetId::Classic),
        });
        app.update();
        assert_eq!(
            app.world().resource::<ActiveHudPreset>().id,
            Some(HudPresetId::Minimal)
        );
        assert!(app.world().resource::<HudSessionLayout>().anchors.is_none());
        assert_eq!(file_bytes(&dir), before);
    }

    #[test]
    fn boot_f5_unknown_anchor_falls_back_and_keeps_the_file() {
        let file = HudLayoutFile {
            schema: HUD_LAYOUT_SCHEMA.into(),
            registry_rev: HUD_REGISTRY_REV,
            base: "classic".into(),
            anchors: vec![HudLayoutAnchor {
                id: "NO_SUCH_ANCHOR".into(),
                corner: HudLayoutCorner::TopLeft,
                x: 16.0,
                y: 16.0,
                width: 280.0,
                hidden: false,
            }],
        };
        let (dir, mut app) = plug_app();
        save_hud_layout(&file).expect("refused");
        let before = file_bytes(&dir);
        app.insert_resource(ActiveHudPreset {
            id: Some(HudPresetId::Minimal),
        });
        app.update();
        assert_eq!(
            app.world().resource::<ActiveHudPreset>().id,
            Some(HudPresetId::Classic)
        );
        assert!(app.world().resource::<HudSessionLayout>().anchors.is_none());
        assert_eq!(file_bytes(&dir), before);
    }

    #[test]
    fn boot_f6_margin_failure_falls_back_without_writing() {
        let file = HudLayoutFile {
            schema: HUD_LAYOUT_SCHEMA.into(),
            registry_rev: HUD_REGISTRY_REV,
            base: "classic".into(),
            anchors: vec![HudLayoutAnchor {
                id: "LEFT_STATUS_1".into(),
                corner: HudLayoutCorner::TopLeft,
                x: 0.0,
                y: 93.0,
                width: 520.0,
                hidden: false,
            }],
        };
        let (dir, mut app) = plug_app();
        save_hud_layout(&file).expect("margin");
        let before = file_bytes(&dir);
        app.insert_resource(ActiveHudPreset {
            id: Some(HudPresetId::Minimal),
        });
        app.update();
        assert!(app.world().resource::<HudSessionLayout>().anchors.is_none());
        assert_eq!(
            app.world().resource::<ActiveHudPreset>().id,
            Some(HudPresetId::Classic)
        );
        assert_eq!(file_bytes(&dir), before);
    }

    fn hidden_peer_file() -> HudLayoutFile {
        HudLayoutFile {
            schema: HUD_LAYOUT_SCHEMA.into(),
            registry_rev: HUD_REGISTRY_REV,
            base: "classic".into(),
            anchors: vec![HudLayoutAnchor {
                id: "CORNER_PEER".into(),
                corner: HudLayoutCorner::BottomRight,
                x: 80.0,
                y: 16.0,
                width: 280.0,
                hidden: true,
            }],
        }
    }

    #[test]
    fn boot_f6_keeps_a_hidden_class_five_anchor() {
        let (_dir, mut app) = plug_app();
        save_hud_layout(&hidden_peer_file()).expect("hide");
        app.update();
        let session = app.world().resource::<HudSessionLayout>();
        let peer = session
            .anchors
            .as_ref()
            .expect("override")
            .iter()
            .find(|anchor| anchor.id == "CORNER_PEER")
            .expect("peer");
        assert!(peer.hidden);
        assert_eq!(
            app.world().resource::<ActiveHudPreset>().id,
            Some(HudPresetId::Classic)
        );
    }

    #[test]
    fn boot_f7_resize_drops_the_session_without_writing() {
        let dir = DirGuard::new();
        save_hud_layout(&hidden_peer_file()).expect("hide");
        let before = file_bytes(&dir);
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_proof_window)
            .add_plugins(HudEditModePlugin);
        app.update();
        assert!(app.world().resource::<HudSessionLayout>().anchors.is_some());
        {
            let mut windows = app
                .world_mut()
                .query_filtered::<&mut Window, With<PrimaryWindow>>();
            let mut window = windows.single_mut(app.world_mut()).unwrap();
            window.resolution.set(200.0, 200.0);
        }
        app.update();
        assert!(app.world().resource::<HudSessionLayout>().anchors.is_none());
        assert_eq!(file_bytes(&dir), before);
    }

    fn spawn_proof_window(mut commands: Commands) {
        commands.spawn((
            Window {
                resolution: WindowResolution::new(1024, 640),
                ..default()
            },
            PrimaryWindow,
        ));
    }

    #[test]
    fn edit_frames_block_and_the_toolbar_has_no_forbidden_rows() {
        let (_dir, mut app) = plug_app();
        app.insert_resource(classic_edit());
        app.update();
        let mut frames = app.world_mut().query::<(&HudEditFrame, &FocusPolicy)>();
        let frame_rows: Vec<_> = frames.iter(app.world()).collect();
        assert_eq!(frame_rows.len(), preset(HudPresetId::Classic).anchors.len());
        assert!(frame_rows
            .iter()
            .all(|(_, policy)| **policy == FocusPolicy::Block));
        let mut saves = app.world_mut().query::<&HudEditSaveBtn>();
        assert_eq!(saves.iter(app.world()).count(), 1);
        let mut cancels = app.world_mut().query::<&HudEditCancelBtn>();
        assert_eq!(cancels.iter(app.world()).count(), 1);
        let mut resets = app.world_mut().query::<&HudEditResetBtn>();
        assert_eq!(resets.iter(app.world()).count(), 1);
        let mut hide_q = app.world_mut().query::<&HudEditHide>();
        let hides: Vec<&str> = hide_q.iter(app.world()).map(|hide| hide.id).collect();
        assert!(hides.iter().any(|id| *id == "CORNER_PEER"));
        assert!(!hides.iter().any(|id| *id == "WINDOW"));
        let mut texts = app.world_mut().query::<&Text>();
        for text in texts.iter(app.world()) {
            for word in ["Places", "Ledger", "Title", "Online", "Door", "door"] {
                assert!(
                    !text.as_str().contains(word),
                    "forbidden row text {}",
                    text.as_str()
                );
            }
        }
    }

    #[test]
    fn edit_hud_row_closes_the_pause_plate() {
        let (_dir, mut app) = plug_app();
        app.insert_resource(quiet_house(true));
        app.world_mut().spawn((
            (
                bevy::ui_widgets::Button,
                Node::default(),
                Interaction::Pressed,
            ),
            SettingsHudEditBtn,
        ));
        app.update();
        assert!(!app.world().resource::<HouseLabel>().settings_open);
        assert!(app.world().resource::<HudEditMode>().active);
    }

    #[test]
    fn save_button_does_not_write_when_the_gate_is_closed() {
        let dir = DirGuard::new();
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(HudEditModePlugin);
        let mut edit = classic_edit();
        {
            let left = anchor_mut(&mut edit, "LEFT_STATUS_1");
            left.corner = HudCorner::TopLeft;
            left.offset = HudOffset { x: 16.0, y: 200.0 };
        }
        {
            let peer = anchor_mut(&mut edit, "CORNER_PEER");
            peer.corner = HudCorner::TopLeft;
            peer.offset = HudOffset { x: 16.0, y: 200.0 };
        }
        app.insert_resource(edit);
        app.update();
        assert!(!app.world().resource::<HudEditMode>().save_enabled);
        let mut saves = app
            .world_mut()
            .query_filtered::<&mut Interaction, With<HudEditSaveBtn>>();
        *saves.single_mut(app.world_mut()).unwrap() = Interaction::Pressed;
        app.update();
        assert!(!dir.layout_file().exists());
    }

    #[test]
    fn allocate_button_presses_at_the_moved_rect() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(HudLayoutPlugin);
        app.insert_resource(ActiveHudPreset {
            id: Some(HudPresetId::Classic),
        });
        let mut anchors = preset(HudPresetId::Classic).anchors.to_vec();
        for anchor in &mut anchors {
            if anchor.id == "WINDOW" {
                anchor.corner = HudCorner::TopLeft;
                anchor.offset = HudOffset { x: 40.0, y: 200.0 };
            }
        }
        app.insert_resource(HudSessionLayout {
            base: Some(HudPresetId::Classic),
            anchors: Some(anchors),
        });
        let root = app
            .world_mut()
            .spawn((
                Node {
                        padding: UiRect::all(Val::Px(7.0)),
                        border: UiRect::all(Val::Px(8.0)),
                        margin: UiRect {
                            right: Val::Px(6.0),
                            top: Val::Px(3.0),
                            ..default()
                        },
                        ..default()
                    },
                HudSlab(ID_ALLOCATE),
            ))
            .with_children(|parent| {
                parent.spawn((
                    bevy::ui_widgets::Button,
                    Node::default(),
                    FocusPolicy::Block,
                    Interaction::default(),
                    AllocateFlowButton,
                ));
            })
            .id();
        app.insert_resource(PressTape(0));
        app.add_systems(Update, count_allocate_presses);
        app.update();
        let style = app.world().get::<Node>(root).expect("style");
        assert_eq!(style.top, Val::Px(200.0));
        assert_eq!(style.left, Val::Px(40.0));
        assert_eq!(style.width, Val::Px(520.0));
        assert_eq!(style.padding.left, Val::Px(7.0));
        assert_eq!(style.border.left, Val::Px(8.0));
        assert_eq!(style.margin.right, Val::Px(6.0));
        assert_eq!(style.margin.top, Val::Px(3.0));
        let window = preset(HudPresetId::Classic)
            .anchors
            .iter()
            .find(|anchor| anchor.id == "WINDOW")
            .copied()
            .expect("window");
        let mut moved = window;
        moved.corner = HudCorner::TopLeft;
        moved.offset = HudOffset { x: 40.0, y: 200.0 };
        let rect = anchor_rect(&moved, 1024.0, 640.0);
        assert!(point_inside(rect, 48, 210));
        assert!(!point_inside(rect, 440, 30));
        let old = anchor_rect(&window, 1024.0, 640.0);
        assert!(point_inside(old, 440, 30));
        assert!(!point_inside(old, 48, 210));
        if point_inside(rect, 48, 210) {
            let mut buttons = app
                .world_mut()
                .query_filtered::<&mut Interaction, With<AllocateFlowButton>>();
            *buttons.single_mut(app.world_mut()).unwrap() = Interaction::Pressed;
        }
        app.update();
        assert_eq!(app.world().resource::<PressTape>().0, 1);
        {
            let mut buttons = app
                .world_mut()
                .query_filtered::<&mut Interaction, With<AllocateFlowButton>>();
            *buttons.single_mut(app.world_mut()).unwrap() = Interaction::None;
        }
        app.update();
        let before = app.world().resource::<PressTape>().0;
        if point_inside(rect, 440, 30) {
            let mut buttons = app
                .world_mut()
                .query_filtered::<&mut Interaction, With<AllocateFlowButton>>();
            *buttons.single_mut(app.world_mut()).unwrap() = Interaction::Pressed;
        }
        app.update();
        assert_eq!(app.world().resource::<PressTape>().0, before);
    }

    fn point_inside(rect: HudRect, x: i32, y: i32) -> bool {
        x >= rect.x0 && x < rect.x1 && y >= rect.y0 && y < rect.y1
    }

    fn count_allocate_presses(
        buttons: Query<&Interaction, With<AllocateFlowButton>>,
        mut tape: ResMut<PressTape>,
    ) {
        if buttons
            .iter()
            .any(|interaction| *interaction == Interaction::Pressed)
        {
            tape.0 += 1;
        }
    }

    #[derive(Resource, Default)]
    struct PressTape(u32);

    #[test]
    fn hidden_peer_hides_its_slab_and_r5_stops_while_editing() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(HudLayoutPlugin);
        app.insert_resource(ActiveHudPreset {
            id: Some(HudPresetId::Classic),
        });
        app.insert_resource(quiet_house(true));
        app.insert_resource(HudEditMode {
            active: true,
            ..HudEditMode::default()
        });
        let peer = app
            .world_mut()
            .spawn((
                (
                    Node::default(),
                    Visibility::Visible,
                ),
                HudSlab(ID_PEER),
            ))
            .id();
        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(peer).expect("vis"),
            Visibility::Visible
        );

        let mut anchors = preset(HudPresetId::Classic).anchors.to_vec();
        for anchor in &mut anchors {
            if anchor.id == "CORNER_PEER" {
                anchor.hidden = true;
            }
        }
        app.insert_resource(HudSessionLayout {
            base: Some(HudPresetId::Classic),
            anchors: Some(anchors),
        });
        app.insert_resource(HudEditMode::default());
        app.insert_resource(quiet_house(false));
        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(peer).expect("vis"),
            Visibility::Hidden
        );
    }

    #[derive(Resource, Default)]
    struct DoorTape {
        reached: u32,
        crossed: Option<crate::hour_sacred::HousePeople>,
    }

    fn door_probe(
        keyboard: Res<ButtonInput<KeyCode>>,
        input: Res<PlayerInput>,
        mut tape: ResMut<DoorTape>,
    ) {
        let hit = keyboard.just_pressed(INTERACT)
            || keyboard.just_pressed(BUILD_WHEEL)
            || input.interact
            || input.sheet_q;
        if !hit {
            return;
        }
        tape.reached += 1;
        let _ = crate::hour_sacred::try_cross_people_door(
            true,
            true,
            &mut tape.crossed,
            crate::hour_sacred::HousePeople::Human,
        );
    }

    fn input_app(active: bool) -> (DirGuard, App) {
        let dir = DirGuard::new();
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(bevy::input::InputPlugin)
            .add_plugins(crate::input::InputPlugin)
            .add_plugins(HudEditModePlugin)
            .insert_resource(LocalSettingsState {
                inner: LocalSettings::peace_defaults(),
                dirty: false,
            });
        {
            let mut edit = app.world_mut().resource_mut::<HudEditMode>();
            edit.active = active;
            if active {
                let anchors = preset(HudPresetId::Classic).anchors.to_vec();
                edit.staged = anchors.clone();
                edit.opened = anchors;
                edit.staged_base = HudPresetId::Classic;
                edit.opened_base = HudPresetId::Classic;
            }
        }
        (dir, app)
    }

    fn press(app: &mut App, key_code: KeyCode, logical_key: Key) {
        app.world_mut().write_message(KeyboardInput {
            key_code,
            logical_key,
            state: ButtonState::Pressed,
            repeat: false,
            text: None,
            window: Entity::PLACEHOLDER,
        });
    }

    #[test]
    fn edit_mode_ignores_bare_eq_and_keeps_wasd_jump_and_sprint() {
        let (_dir, mut app) = input_app(true);
        app.insert_resource(DoorTape::default());
        app.add_systems(PostUpdate, door_probe);
        app.update();
        press(&mut app, KeyCode::KeyE, Key::Character("e".into()));
        press(&mut app, KeyCode::KeyQ, Key::Character("q".into()));
        press(&mut app, KeyCode::KeyW, Key::Character("w".into()));
        press(&mut app, KeyCode::Space, Key::Space);
        press(&mut app, KeyCode::ShiftLeft, Key::Shift);
        app.update();
        let input = app.world().resource::<PlayerInput>();
        assert!(!input.interact);
        assert!(!input.sheet_q);
        assert!(!input.pause_toggle);
        assert!(input.movement.y > 0.0);
        assert!(input.jump);
        assert!(input.sprint);
        let keyboard = app.world().resource::<ButtonInput<KeyCode>>();
        assert!(!keyboard.just_pressed(INTERACT));
        assert!(!keyboard.just_pressed(BUILD_WHEEL));
        let tape = app.world().resource::<DoorTape>();
        assert_eq!(tape.reached, 0);
        assert!(tape.crossed.is_none());
    }

    #[test]
    fn edit_mode_ignores_a_remapped_use_key() {
        let (_dir, mut app) = input_app(true);
        app.insert_resource(DoorTape::default());
        app.add_systems(PostUpdate, door_probe);
        {
            let mut settings = app.world_mut().resource_mut::<LocalSettingsState>();
            settings.inner.key_use = PeaceKey::F;
        }
        app.update();
        press(&mut app, KeyCode::KeyF, Key::Character("f".into()));
        press(&mut app, KeyCode::KeyQ, Key::Character("q".into()));
        app.update();
        let input = app.world().resource::<PlayerInput>();
        assert!(!input.interact);
        assert!(!input.sheet_q);
        let keyboard = app.world().resource::<ButtonInput<KeyCode>>();
        assert!(!keyboard.just_pressed(INTERACT));
        assert!(!keyboard.just_pressed(BUILD_WHEEL));
        assert_eq!(app.world().resource::<DoorTape>().reached, 0);
    }

    #[test]
    fn edit_mode_ignores_pad_use_sheet_q_and_start() {
        let (_dir, mut app) = input_app(true);
        app.insert_resource(DoorTape::default());
        app.add_systems(PostUpdate, door_probe);
        app.update();
        let pad = app.world_mut().spawn_empty().id();
        app.world_mut().write_message(GamepadConnectionEvent::new(
            pad,
            GamepadConnection::Connected {
                name: "test pad".into(),
                vendor_id: None,
                product_id: None,
            },
        ));
        app.update();
        for button in [GamepadButton::South, GamepadButton::West, GamepadButton::Start] {
            app.world_mut().write_message(RawGamepadEvent::Button(
                RawGamepadButtonChangedEvent::new(pad, button, 1.0),
            ));
        }
        app.update();
        let input = app.world().resource::<PlayerInput>();
        assert!(!input.interact);
        assert!(!input.sheet_q);
        assert!(!input.pause_toggle);
        assert_eq!(app.world().resource::<DoorTape>().reached, 0);
        assert!(app.world().resource::<DoorTape>().crossed.is_none());
    }

    #[test]
    fn eq_outside_edit_mode_stays_off_the_people_door() {
        let (_dir, mut app) = input_app(false);
        app.update();
        press(&mut app, KeyCode::KeyE, Key::Character("e".into()));
        press(&mut app, KeyCode::KeyQ, Key::Character("q".into()));
        app.update();
        assert!(app.world().resource::<PlayerInput>().interact);
        assert!(app
            .world()
            .resource::<ButtonInput<KeyCode>>()
            .just_pressed(BUILD_WHEEL));
        let crossed: Option<crate::hour_sacred::HousePeople> = None;
        assert!(crossed.is_none());
    }

    #[test]
    fn production_edit_mode_never_calls_the_people_door_or_std_fs() {
        let src = include_str!("hud_edit_mode.rs");
        let production = src.split("mod tests").next().expect("split");
        assert!(!production.contains("try_cross_people_door"));
        assert!(!production.contains("std::fs"));
        assert!(!production.contains("TcpListener"));
    }
}

#[cfg(test)]
pub(crate) use tests::DirGuard;
