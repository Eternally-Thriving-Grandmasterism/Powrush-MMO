//! Touch / on-screen sticks overlay (I0 — INPUT_CANON).
//!
//! Left virtual stick + right Use + top-right Pause / Settings / Q / L.
//! Hit targets ≥ 44 logical px. Shown only when `on_screen_sticks` resolves on
//! (`on`, or `auto` + last pointer Touch). **Hidden on pure mouse Title.**
//! Entire overlay culls when Title / pause / Settings / L / Q / I plates open
//! (same spirit as light_gen cull). Buttons consume press (no camera look).
//! No second Camera3d — LivedUiPlate + ui_order law. Soft GPU first-class.
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use crate::input::{overlay_sticks_visible, InputMapSet, LastPointerKind, PlayerInput};
use crate::local_settings::LocalSettingsState;
use crate::title_screen::{HouseLabel, LaunchDoor};
use crate::ui_above_world::{LivedUiPlate, LIVED_UI_Z_LEDGER};

/// Minimum touch target (logical px) — INPUT_CANON ≥44dp.
pub const TOUCH_HIT_MIN: f32 = 44.0;

#[derive(Component)]
struct TouchOverlayRoot;

#[derive(Component)]
struct TouchUseBtn;

#[derive(Component)]
struct TouchPauseBtn;

#[derive(Component)]
struct TouchQBtn;

#[derive(Component)]
struct TouchLBtn;

#[derive(Component)]
struct TouchStickZone;

/// Virtual left-stick state (normalized −1..1).
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct TouchStickState {
    pub axis: Vec2,
    pub active: bool,
}

pub struct TouchControlsPlugin;

impl Plugin for TouchControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TouchStickState>()
            .add_systems(Startup, spawn_touch_overlay)
            .add_systems(
                Update,
                (
                    sync_touch_overlay_visibility,
                    touch_overlay_button_clicks,
                    touch_stick_drag,
                )
                    .after(InputMapSet),
            );
    }
}

fn spawn_touch_overlay(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
                visibility: Visibility::Hidden,
                z_index: ZIndex::Global(LIVED_UI_Z_LEDGER - 1),
                ..default()
            },
            TouchOverlayRoot,
            LivedUiPlate,
            Name::new("TouchOverlay"),
        ))
        .with_children(|root| {
            // Left stick zone (bottom-left)
            root.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(24.0),
                        bottom: Val::Px(24.0),
                        width: Val::Px(120.0),
                        height: Val::Px(120.0),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgba(0.12, 0.16, 0.14, 0.45).into(),
                    border_color: Color::srgba(0.55, 0.72, 0.60, 0.55).into(),
                    ..default()
                },
                TouchStickZone,
                Interaction::default(),
            ))
            .with_children(|z| {
                z.spawn(TextBundle::from_section(
                    "⊕",
                    TextStyle {
                        font_size: 22.0,
                        color: Color::srgba(0.85, 0.95, 0.88, 0.85),
                        ..default()
                    },
                ));
            });

            // Right Use (≥44px)
            spawn_overlay_btn(
                root,
                "Use",
                TouchUseBtn,
                UiRect {
                    right: Val::Px(28.0),
                    bottom: Val::Px(36.0),
                    ..default()
                },
                Color::srgba(0.18, 0.42, 0.28, 0.72),
            );

            // Top-right cluster: Pause / Q / L
            spawn_overlay_btn(
                root,
                "❚❚",
                TouchPauseBtn,
                UiRect {
                    right: Val::Px(24.0),
                    top: Val::Px(24.0),
                    ..default()
                },
                Color::srgba(0.20, 0.22, 0.28, 0.75),
            );
            spawn_overlay_btn(
                root,
                "Q",
                TouchQBtn,
                UiRect {
                    right: Val::Px(24.0),
                    top: Val::Px(24.0 + TOUCH_HIT_MIN + 8.0),
                    ..default()
                },
                Color::srgba(0.20, 0.22, 0.28, 0.75),
            );
            spawn_overlay_btn(
                root,
                "L",
                TouchLBtn,
                UiRect {
                    right: Val::Px(24.0),
                    top: Val::Px(24.0 + 2.0 * (TOUCH_HIT_MIN + 8.0)),
                    ..default()
                },
                Color::srgba(0.20, 0.22, 0.28, 0.75),
            );
        });
}

fn spawn_overlay_btn<B: Component>(
    p: &mut ChildBuilder,
    label: &str,
    marker: B,
    inset: UiRect,
    bg: Color,
) {
    p.spawn((
        ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: inset.left,
                right: inset.right,
                top: inset.top,
                bottom: inset.bottom,
                width: Val::Px(TOUCH_HIT_MIN),
                height: Val::Px(TOUCH_HIT_MIN),
                min_width: Val::Px(TOUCH_HIT_MIN),
                min_height: Val::Px(TOUCH_HIT_MIN),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.5)),
                ..default()
            },
            background_color: bg.into(),
            border_color: Color::srgba(0.70, 0.80, 0.72, 0.65).into(),
            ..default()
        },
        marker,
    ))
    .with_children(|b| {
        b.spawn(TextBundle::from_section(
            label,
            TextStyle {
                font_size: 15.0,
                color: Color::srgb(0.92, 0.96, 0.90),
                ..default()
            },
        ));
    });
}

/// Same cull spirit as light_gen: hide overlay when any lived plate is open.
pub fn overlay_should_cull(
    title_open: bool,
    pause_or_settings: bool,
    ledger_open: bool,
    q_hint: bool,
    inventory_open: bool,
) -> bool {
    title_open || pause_or_settings || ledger_open || q_hint || inventory_open
}

fn sync_touch_overlay_visibility(
    settings: Res<LocalSettingsState>,
    last: Res<LastPointerKind>,
    door: Res<LaunchDoor>,
    house: Res<HouseLabel>,
    places: Option<Res<crate::hex_travel::PlacesPlate>>,
    ledger: Option<Res<crate::ledger_bind::LedgerYard>>,
    inv: Option<Res<crate::human_inventory::HumanInventory>>,
    factory: Option<Res<crate::vertical_factory::FactoryYard>>,
    mut roots: Query<&mut Visibility, With<TouchOverlayRoot>>,
    mut stick: ResMut<TouchStickState>,
) {
    let title_open = matches!(
        *door,
        LaunchDoor::Title | LaunchDoor::NameHouse | LaunchDoor::HouseDress
    );
    let pause = house.settings_open
        || places.map(|p| crate::hex_travel::places_culls_sticks(&p)).unwrap_or(false);
    let ledger_open = ledger.map(|l| l.sash_open).unwrap_or(false);
    let inv_open = inv.map(|i| i.open).unwrap_or(false);
    let q_hint = factory
        .map(|f| f.factory.founded && !f.factory.tutorial_complete())
        .unwrap_or(false);
    let cull = overlay_should_cull(title_open, pause, ledger_open, q_hint, inv_open);
    let want = !cull && overlay_sticks_visible(&settings.inner, *last);

    let vis = if want {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
    for mut v in &mut roots {
        if *v != vis {
            *v = vis;
        }
    }
    if !want {
        stick.axis = Vec2::ZERO;
        stick.active = false;
    }
}

fn touch_overlay_button_clicks(
    mut player_input: ResMut<PlayerInput>,
    mut house: ResMut<HouseLabel>,
    door: Res<LaunchDoor>,
    use_q: Query<&Interaction, (Changed<Interaction>, With<TouchUseBtn>)>,
    pause_q: Query<&Interaction, (Changed<Interaction>, With<TouchPauseBtn>)>,
    q_q: Query<&Interaction, (Changed<Interaction>, With<TouchQBtn>)>,
    l_q: Query<&Interaction, (Changed<Interaction>, With<TouchLBtn>)>,
) {
    for i in &use_q {
        if *i == Interaction::Pressed {
            // Same Use verb as E / South — one edge this frame.
            player_input.interact = true;
        }
    }
    for i in &pause_q {
        if *i == Interaction::Pressed {
            // Same as Esc / Start when InYard; on Title also toggles settings plate.
            if *door == LaunchDoor::InYard || *door == LaunchDoor::Title {
                house.settings_open = !house.settings_open;
            }
        }
    }
    for i in &q_q {
        if *i == Interaction::Pressed {
            player_input.sheet_q = true;
        }
    }
    for i in &l_q {
        if *i == Interaction::Pressed {
            player_input.sheet_l = true;
        }
    }
}

fn touch_stick_drag(
    roots: Query<&Visibility, With<TouchOverlayRoot>>,
    zones: Query<(&Interaction, &Node, &GlobalTransform), With<TouchStickZone>>,
    windows: Query<&Window>,
    touches: Res<Touches>,
    mut stick: ResMut<TouchStickState>,
    mut player_input: ResMut<PlayerInput>,
) {
    let visible = roots.iter().any(|v| *v == Visibility::Visible);
    if !visible {
        stick.axis = Vec2::ZERO;
        stick.active = false;
        return;
    }

    let Ok(window) = windows.get_single() else {
        return;
    };
    let cursor = window.cursor_position();

    for (interaction, node, gt) in &zones {
        let size = node.size();
        let center = gt.translation().truncate();
        // Bevy UI GlobalTransform origin is top-left-ish; approximate center.
        let half = size * 0.5;
        let mid = Vec2::new(center.x + half.x, center.y + half.y);

        let mut pointer: Option<Vec2> = None;
        if *interaction == Interaction::Pressed || *interaction == Interaction::Hovered {
            if let Some(c) = cursor {
                pointer = Some(c);
            }
        }
        for touch in touches.iter() {
            pointer = Some(touch.position());
            break;
        }

        if let Some(pos) = pointer {
            if *interaction != Interaction::None || touches.iter().next().is_some() {
                let delta = (pos - mid) / half.max(Vec2::splat(1.0));
                // UI Y grows down; movement Y grows up.
                let axis = Vec2::new(delta.x, -delta.y).clamp(Vec2::splat(-1.0), Vec2::splat(1.0));
                let dead = 0.18;
                let axis = if axis.length() < dead {
                    Vec2::ZERO
                } else {
                    axis
                };
                stick.axis = axis;
                stick.active = axis != Vec2::ZERO;
                if stick.active {
                    let mut m = player_input.movement + axis;
                    if m.length_squared() > 1.0 {
                        m = m.normalize();
                    }
                    player_input.movement = m;
                }
                return;
            }
        }
    }
    stick.axis = Vec2::ZERO;
    stick.active = false;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::LastPointerKind;
    use shared::local_settings::LocalSettings;

    #[test]
    fn hit_targets_meet_44dp() {
        assert!(TOUCH_HIT_MIN >= 44.0);
    }

    #[test]
    fn cull_hides_on_any_plate() {
        assert!(!overlay_should_cull(false, false, false, false, false));
        assert!(overlay_should_cull(true, false, false, false, false));
        assert!(overlay_should_cull(false, true, false, false, false));
        assert!(overlay_should_cull(false, false, true, false, false));
        assert!(overlay_should_cull(false, false, false, true, false));
        assert!(overlay_should_cull(false, false, false, false, true));
    }

    #[test]
    fn mouse_title_keeps_overlay_off_by_default() {
        let s = LocalSettings::peace_defaults();
        assert!(!overlay_sticks_visible(&s, LastPointerKind::Mouse));
        // Even if not culled, auto+mouse ⇒ hidden (Title click-clean).
        assert!(
            overlay_should_cull(true, false, false, false, false)
                || !overlay_sticks_visible(&s, LastPointerKind::Mouse)
        );
    }
}
