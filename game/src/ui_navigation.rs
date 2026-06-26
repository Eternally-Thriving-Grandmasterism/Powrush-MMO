/*!
 * Spatial Grid UI Navigation System
 *
 * v7 - Added volume control for UI sounds
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy::input::gamepad::{GamepadButton, Gamepads};
use bevy::audio::PlaybackSettings;

#[derive(Component, Clone, Debug, Default)]
pub struct Focusable {
    pub order: i32,
}

#[derive(Component)]
pub struct Focused;

#[derive(Resource, Default)]
pub struct UiFocus {
    pub current: Option<Entity>,
}

/// Configuration resource for UI navigation audio
#[derive(Resource)]
pub struct UiAudioSettings {
    pub navigation_volume: f32,
    pub activation_volume: f32,
}

impl Default for UiAudioSettings {
    fn default() -> Self {
        Self {
            navigation_volume: 0.6,
            activation_volume: 0.8,
        }
    }
}

#[derive(Clone, Copy)]
pub enum NavDirection {
    Up,
    Down,
    Left,
    Right,
}

impl NavDirection {
    fn as_vec2(self) -> Vec2 {
        match self {
            NavDirection::Up => Vec2::new(0.0, 1.0),
            NavDirection::Down => Vec2::new(0.0, -1.0),
            NavDirection::Left => Vec2::new(-1.0, 0.0),
            NavDirection::Right => Vec2::new(1.0, 0.0),
        }
    }
}

pub fn gamepad_ui_navigation(
    gamepads: Res<Gamepads>,
    axes: Res<bevy::input::gamepad::GamepadAxis>,
    buttons: Res<bevy::input::gamepad::GamepadButton>,
    mut focus: ResMut<UiFocus>,
    focusables: Query<(Entity, &Focusable, &GlobalTransform)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    static mut LAST_NAV_TIME: f32 = 0.0;
    let current_time = time.elapsed_seconds();

    unsafe {
        if current_time - LAST_NAV_TIME < 0.15 {
            return;
        }
    }

    for _gamepad in gamepads.iter() {
        let mut direction: Option<NavDirection> = None;

        if buttons.just_pressed(GamepadButton::DPadUp) { direction = Some(NavDirection::Up); }
        else if buttons.just_pressed(GamepadButton::DPadDown) { direction = Some(NavDirection::Down); }
        else if buttons.just_pressed(GamepadButton::DPadLeft) { direction = Some(NavDirection::Left); }
        else if buttons.just_pressed(GamepadButton::DPadRight) { direction = Some(NavDirection::Right); }

        let stick_x = axes.get(bevy::input::gamepad::GamepadAxis::LeftStickX).unwrap_or(0.0);
        let stick_y = axes.get(bevy::input::gamepad::GamepadAxis::LeftStickY).unwrap_or(0.0);

        if direction.is_none() {
            if stick_y > 0.7 { direction = Some(NavDirection::Up); }
            else if stick_y < -0.7 { direction = Some(NavDirection::Down); }
            else if stick_x < -0.7 { direction = Some(NavDirection::Left); }
            else if stick_x > 0.7 { direction = Some(NavDirection::Right); }
        }

        if let Some(dir) = direction {
            if let Some(current) = focus.current {
                if let Some(best) = find_best_focusable_in_direction(current, dir, &focusables) {
                    if let Some(old) = focus.current {
                        commands.entity(old).remove::<Focused>();
                    }
                    commands.entity(best).insert(Focused);
                    focus.current = Some(best);
                    unsafe { LAST_NAV_TIME = current_time; }
                }
            } else if let Some((first, _, _)) = focusables.iter().next() {
                commands.entity(first).insert(Focused);
                focus.current = Some(first);
            }
        }
    }
}

fn find_best_focusable_in_direction(
    current: Entity,
    direction: NavDirection,
    focusables: &Query<(Entity, &Focusable, &GlobalTransform)>,
) -> Option<Entity> {
    let current_pos = focusables
        .get(current)
        .map(|(_, _, t)| t.translation().truncate())
        .ok()?;

    let dir_vec = direction.as_vec2();
    let mut best_entity = None;
    let mut best_score = f32::NEG_INFINITY;

    for (entity, _, transform) in focusables.iter() {
        if entity == current { continue; }

        let pos = transform.translation().truncate();
        let to_target = pos - current_pos;
        let dot = to_target.dot(dir_vec);

        if dot <= 0.0 { continue; }

        let distance = to_target.length();
        let alignment = dot / distance.max(1.0);
        let score = alignment * 100.0 - distance * 0.5;

        if score > best_score {
            best_score = score;
            best_entity = Some(entity);
        }
    }

    best_entity
}

pub fn apply_focus_visuals(
    mut commands: Commands,
    focus: Res<UiFocus>,
    added_focused: Query<Entity, Added<Focused>>,
    mut removed_focused: RemovedComponents<Focused>,
    time: Res<Time>,
    mut query: Query<(&Focused, &mut BackgroundColor, &mut BorderColor)>,
) {
    for entity in added_focused.iter() {
        if let Ok((_, mut bg, mut border)) = query.get_mut(entity) {
            *bg = Color::srgb(0.15, 0.35, 0.65).into();
            *border = BorderColor(Color::srgb(0.4, 0.7, 1.0));
        }
    }

    for entity in removed_focused.read() {
        if let Ok((_, mut bg, mut border)) = query.get_mut(entity) {
            *bg = Color::srgb(0.2, 0.2, 0.25).into();
            *border = BorderColor(Color::NONE);
        }
    }

    let pulse = (time.elapsed_seconds() * 3.0).sin() * 0.3 + 0.7;
    let pulse_color = Color::srgb(0.4 * pulse, 0.7 * pulse, 1.0 * pulse);

    for (_, mut _bg, mut border) in query.iter_mut() {
        if border.0 != Color::NONE {
            border.0 = pulse_color;
        }
    }
}

pub fn activate_focused_button(
    buttons: Res<bevy::input::gamepad::GamepadButton>,
    focus: Res<UiFocus>,
    mut interaction_query: Query<(&Focusable, &mut Interaction)>,
) {
    let Some(focused_entity) = focus.current else { return; };
    let Ok((_, mut interaction)) = interaction_query.get_mut(focused_entity) else { return; };

    if buttons.just_pressed(GamepadButton::South) {
        *interaction = Interaction::Pressed;
    }
    if buttons.just_released(GamepadButton::South) {
        *interaction = Interaction::None;
    }
}

/// Plays navigation sound when focus changes
pub fn play_focus_change_sound(
    focus: Res<UiFocus>,
    mut last_focus: Local<Option<Entity>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    settings: Res<UiAudioSettings>,
) {
    if focus.current != *last_focus {
        if focus.current.is_some() {
            let sound = asset_server.load("audio/ui_nav.ogg");
            audio.play_with_settings(
                sound,
                PlaybackSettings::ONCE.with_volume(settings.navigation_volume),
            );
        }
        *last_focus = focus.current;
    }
}

/// Plays confirmation sound when activating a button
pub fn play_button_activate_sound(
    buttons: Res<bevy::input::gamepad::GamepadButton>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    settings: Res<UiAudioSettings>,
) {
    if buttons.just_pressed(GamepadButton::South) {
        let sound = asset_server.load("audio/ui_confirm.ogg");
        audio.play_with_settings(
            sound,
            PlaybackSettings::ONCE.with_volume(settings.activation_volume),
        );
    }
}

pub struct UiNavigationPlugin;

impl Plugin for UiNavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiFocus>()
            .init_resource::<UiAudioSettings>()
            .add_systems(Update, gamepad_ui_navigation)
            .add_systems(Update, apply_focus_visuals)
            .add_systems(Update, activate_focused_button)
            .add_systems(Update, play_focus_change_sound)
            .add_systems(Update, play_button_activate_sound);
    }
}
