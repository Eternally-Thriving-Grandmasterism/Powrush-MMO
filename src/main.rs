use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng;
use crate::skill_tree::SkillTreePlugin;  // New

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Powrush-MMO — Skill Tree Thriving".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AudioPlugin)
        .add_plugins(Voice
