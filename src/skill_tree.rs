use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Replicated)]
pub struct SkillTree {
    pub points: u32,
    pub trust_level: u32,
    pub lattice_level: u32,
    pub mercy_level: u32,
}

#[derive(Event, Replicated)]
pub struct SkillUpgradeEvent(pub SkillBranch, pub u32);  // Branch, level

#[derive(Clone, Copy, PartialEq, Replicated)]
pub enum SkillBranch {
    Trust,
    Lattice,
    Mercy,
}

pub struct SkillTreePlugin;

impl Plugin for SkillTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SkillUpgradeEvent>()
           .add_systems(Update, (skill_point_system, skill_ui_system));
    }
}

fn skill_point_system(
    mut query: Query<&mut SkillTree>,
    time: Res<Time>,
) {
    for mut tree in &mut query {
        tree.points += (time.delta_seconds() * 1.0) as u32;  // Mercy exp
    }
}

fn skill_ui_system(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut SkillTree>,
) {
    if keyboard.just_pressed(KeyCode::K) {  // Open skill tree
        if let Ok(mut tree) = query.get_single_mut() {
            if tree.points > 0 {
                tree.points -= 1;
                tree.trust_level += 1;  // Example upgrade
                info!("Trust upgraded to level {}", tree.trust_level);
            }
        }
    }
}
