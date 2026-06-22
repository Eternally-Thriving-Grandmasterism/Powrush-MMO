/*!
 * Comprehensive Bevy 0.14 compatible refactor of the Lissajous knot UI system.
 */

use bevy::prelude::*;
use bevy::input::pointer::Pointer;

// === Event ===
#[derive(Event, Clone, Debug)]
pub struct SwitchLissajousKnotPreset {
    pub preset: LissajousKnotPreset,
}

// === Resources ===
#[derive(Resource, Default)]
pub struct CurrentLissajousKnotPreset {
    pub preset: LissajousKnotPreset,
}

#[derive(Resource, Default)]
pub struct LissajousKnotEffects {
    pub trefoil: Handle<EffectAsset>,
    pub high_writhe: Handle<EffectAsset>,
    pub symmetric: Handle<EffectAsset>,
    pub complex: Handle<EffectAsset>,
}

// === Components ===
#[derive(Component)]
pub struct PresetButton {
    pub preset: LissajousKnotPreset,
}

#[derive(Component)]
pub struct CurrentPresetText;

#[derive(Component)]
pub struct HarmonyKnotMarker;

// === Systems ===

pub fn handle_switch_lissajous_knot_preset(
    mut events: EventReader<SwitchLissajousKnotPreset>,
    mut current: ResMut<CurrentLissajousKnotPreset>,
) {
    for event in events.read() {
        if current.preset != event.preset {
            current.preset = event.preset;
        }
    }
}

pub fn highlight_active_preset_button(
    current: Res<CurrentLissajousKnotPreset>,
    mut buttons: Query<(&PresetButton, &mut BackgroundColor)>,
) {
    for (button, mut bg) in &mut buttons {
        let is_active = button.preset == current.preset;
        let target = if is_active {
            Color::srgb(0.25, 0.35, 0.55)
        } else {
            Color::srgb(0.15, 0.15, 0.22)
        };
        if bg.0 != target {
            *bg = target.into();
        }
    }
}

pub fn update_lissajous_knot_ui(
    current: Res<CurrentLissajousKnotPreset>,
    mut text_query: Query<&mut Text, With<CurrentPresetText>>,
) {
    if current.is_changed() {
        for mut text in &mut text_query {
            text.sections[0].value = format!("Current: {:?}", current.preset);
        }
    }
}

pub fn update_active_lissajous_knot(
    knot_effects: Res<LissajousKnotEffects>,
    current: Res<CurrentLissajousKnotPreset>,
    mut query: Query<&mut ParticleEffect, With<HarmonyKnotMarker>>,
) {
    if current.is_changed() {
        let handle = match current.preset {
            LissajousKnotPreset::TrefoilLike => knot_effects.trefoil.clone(),
            LissajousKnotPreset::HighWrithe => knot_effects.high_writhe.clone(),
            LissajousKnotPreset::Symmetric => knot_effects.symmetric.clone(),
            LissajousKnotPreset::Complex5_3_4 => knot_effects.complex.clone(),
        };
        for mut effect in &mut query {
            effect.effect = handle.clone();
        }
    }
}

// Debug keyboard input
pub fn debug_lissajous_knot_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<SwitchLissajousKnotPreset>,
) {
    if keyboard.just_pressed(KeyCode::Digit1) {
        events.send(SwitchLissajousKnotPreset { preset: LissajousKnotPreset::TrefoilLike });
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        events.send(SwitchLissajousKnotPreset { preset: LissajousKnotPreset::HighWrithe });
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        events.send(SwitchLissajousKnotPreset { preset: LissajousKnotPreset::Symmetric });
    }
    if keyboard.just_pressed(KeyCode::Digit4) {
        events.send(SwitchLissajousKnotPreset { preset: LissajousKnotPreset::Complex5_3_4 });
    }
}
