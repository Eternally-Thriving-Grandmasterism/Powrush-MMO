use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub lattice_stats: LatticeStats,
    pub player_trust: f32,
}

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (save_system, load_system));
    }
}

fn save_system(
    stats: Res<LatticeStats>,
    trust: Query<&TrustCredits>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::S) {
        let data = SaveData {
            lattice_stats: stats.clone(),
            player_trust: trust.single().0,
        };
        let serialized = ron::to_string(&data).unwrap();
        std::fs::write("save.ron", serialized).unwrap();
        info!("Saved");
    }
}

fn load_system(
    mut stats: ResMut<LatticeStats>,
    mut trust: Query<&mut TrustCredits>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::L) {
        if let Ok(serialized) = std::fs::read_to_string("save.ron") {
            if let Ok(data) = ron::from_str::<SaveData>(&serialized) {
                *stats = data.lattice_stats;
                trust.single_mut().0 = data.player_trust;
                info!("Loaded");
            }
        }
    }
}
