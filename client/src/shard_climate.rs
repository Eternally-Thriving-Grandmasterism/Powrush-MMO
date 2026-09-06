//! Phase Q — hold-E care tend (v23.2.36)
//!
//! Does not rewrite harvest_feel. Tap-E take stays there.
//! Hold-E writes the climate ledger only. Optional slab line rides the
//! existing well-speech slab in climate_visible (not a second HUD).
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use crate::lived_hour_bind::LivedHourBind;
use crate::mercy_harvest_nodes::NearbyMercyNode;
use crate::soft_play_bindings;

#[derive(Resource, Default)]
struct CareTendHold {
    seconds: f32,
    fired: bool,
}

pub struct ShardClimatePlugin;

impl Plugin for ShardClimatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CareTendHold>()
            .add_systems(Update, (hold_e_care_tend, persist_climate_on_escape));
    }
}

fn hold_e_care_tend(
    keyboard: Res<ButtonInput<KeyCode>>,
    nearby: Res<NearbyMercyNode>,
    mut bind: ResMut<LivedHourBind>,
    mut hold: ResMut<CareTendHold>,
    time: Res<Time>,
) {
    let pressing = keyboard.pressed(soft_play_bindings::INTERACT);
    if !pressing || !nearby.in_range {
        hold.seconds = 0.0;
        hold.fired = false;
        return;
    }
    // just_pressed is the tap path (harvest_feel). Care starts after a short hold.
    if keyboard.just_pressed(soft_play_bindings::INTERACT) {
        hold.seconds = 0.0;
        hold.fired = false;
        return;
    }
    hold.seconds += time.delta_seconds();
    if hold.seconds >= 0.45 && !hold.fired {
        bind.care_tend();
        hold.fired = true;
    }
}

fn persist_climate_on_escape(keyboard: Res<ButtonInput<KeyCode>>, bind: Res<LivedHourBind>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        bind.persist();
    }
}

#[cfg(test)]
mod tests {
    use shared::hour_two::HourTwoPack;
    use shared::shard_climate::ShardClimate;

    #[test]
    fn book_and_climate_paths_are_siblings() {
        assert_eq!(
            crate::lived_hour_bind::SHARD_CLIMATE_PATH,
            "data/powrush_shard_climate.json"
        );
        assert_eq!(crate::hour_sacred::HOUR_TWO_PATH, "data/powrush_hour_two.json");
        let mut pack = HourTwoPack::default();
        pack.hour_three_complete = true;
        assert!(pack.hour_three_complete);
        let mut c = ShardClimate::default();
        c.on_care_tend();
        let raw = c.to_json().unwrap();
        let loaded = ShardClimate::from_json(&raw).unwrap();
        assert!((loaded.stress - c.stress).abs() < 1e-6);
    }
}
