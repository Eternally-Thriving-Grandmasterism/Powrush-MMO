//! Phase Q + R+ — climate/standing feel (v23.2.38)
//!
//! Hold-E care tend. Fog / ambient / well pulse answer climate stress & harmony.
//! No second HUD. Does not rewrite harvest_feel. Contact: info@Rathor.ai

use bevy::pbr::FogSettings;
use bevy::prelude::*;

use crate::lived_hour_bind::LivedHourBind;
use crate::mercy_harvest_nodes::{MercyHarvestNode, NearbyMercyNode};
use crate::soft_play_bindings;

#[derive(Resource, Default)]
struct CareTendHold {
    seconds: f32,
    fired: bool,
}

#[derive(Resource, Default)]
struct ClimateFeelMemory {
    last_stress: f32,
    sting_cooldown: f32,
}

pub struct ShardClimatePlugin;

impl Plugin for ShardClimatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CareTendHold>()
            .init_resource::<ClimateFeelMemory>()
            .add_systems(
                Update,
                (
                    hold_e_care_tend,
                    paint_climate_feel,
                    persist_climate_on_escape,
                ),
            );
    }
}

fn hold_e_care_tend(
    keyboard: Res<ButtonInput<KeyCode>>,
    nearby: Res<NearbyMercyNode>,
    epiphany: Option<Res<crate::first_harvest_epiphany::FirstHarvestEpiphany>>,
    mut bind: ResMut<LivedHourBind>,
    mut hold: ResMut<CareTendHold>,
    time: Res<Time>,
) {
    // At the Threshold pipe the Use belongs to the session node — it must not
    // reach the hex climate ledger or the week file.
    let at_threshold = epiphany.map(|e| e.threshold_near).unwrap_or(false);
    let pressing = keyboard.pressed(soft_play_bindings::INTERACT);
    if at_threshold || !pressing || !nearby.in_range {
        hold.seconds = 0.0;
        hold.fired = false;
        return;
    }
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

/// Fog closes when the hex is tired; ambient opens when harmony holds.
/// Pulse sting on a stress spike — ecological, not HP.
fn paint_climate_feel(
    bind: Res<LivedHourBind>,
    time: Res<Time>,
    nearby: Res<NearbyMercyNode>,
    mut ambient: ResMut<AmbientLight>,
    mut fogs: Query<&mut FogSettings>,
    mut nodes: Query<(Entity, &mut MercyHarvestNode)>,
    mut mem: ResMut<ClimateFeelMemory>,
) {
    let stress = bind.climate.stress;
    let harmony = bind.climate.harmony;
    let regen = bind.climate.regen;

    // Base ambient rides harmony (no new UI).
    let target_brightness = 240.0 + harmony * 160.0 - stress * 90.0;
    ambient.brightness += (target_brightness - ambient.brightness) * 0.08;

    let fog_start = 6.0 + (1.0 - stress) * 10.0 + regen * 2.0;
    let fog_end = 28.0 + (1.0 - stress) * 24.0 + harmony * 8.0;
    for mut fog in &mut fogs {
        fog.falloff = bevy::pbr::FogFalloff::Linear {
            start: fog_start,
            end: fog_end.max(fog_start + 8.0),
        };
        // Tired hex cools the fog color slightly.
        let t = stress.clamp(0.0, 1.0);
        fog.color = Color::srgba(
            0.28 + (1.0 - t) * 0.12,
            0.48 - t * 0.18,
            0.42 - t * 0.10,
            1.0,
        );
    }

    mem.sting_cooldown = (mem.sting_cooldown - time.delta_seconds()).max(0.0);
    let spike = stress - mem.last_stress;
    mem.last_stress = stress;
    if spike >= 0.08 && mem.sting_cooldown <= 0.0 {
        mem.sting_cooldown = 1.2;
        if let Some(e) = nearby.entity {
            if let Ok((_, mut node)) = nodes.get_mut(e) {
                node.pulse = (node.pulse + 0.45).min(1.0);
            }
        } else {
            for (_, mut node) in &mut nodes {
                node.pulse = (node.pulse + 0.25).min(1.0);
                break;
            }
        }
    }

    // Quiet thriving pulse when harmony high and stress low.
    if harmony >= 0.65 && stress <= 0.25 {
        let breath = (time.elapsed_seconds() * 1.6).sin().abs() * 0.08;
        for (_, mut node) in &mut nodes {
            if node.pulse < 0.2 {
                node.pulse = breath;
            }
        }
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
    use shared::shard_standing::ShardStanding;

    #[test]
    fn book_climate_standing_paths_are_siblings() {
        assert_eq!(
            crate::lived_hour_bind::SHARD_CLIMATE_PATH,
            "data/powrush_shard_climate.json"
        );
        assert_eq!(
            crate::lived_hour_bind::SHARD_STANDING_PATH,
            "data/powrush_shard_standing.json"
        );
        assert_eq!(crate::hour_sacred::HOUR_TWO_PATH, "data/powrush_hour_two.json");
        let mut pack = HourTwoPack::default();
        pack.hour_three_complete = true;
        assert!(pack.hour_three_complete);
        let _ = ShardStanding::default();
        let mut c = ShardClimate::default();
        c.on_care_tend();
        let raw = c.to_json().unwrap();
        let loaded = ShardClimate::from_json(&raw).unwrap();
        assert!((loaded.stress - c.stress).abs() < 1e-6);
    }

    #[test]
    fn tired_hex_closes_fog_math() {
        // Document the feel curve used by paint_climate_feel.
        let stress = 0.8_f32;
        let harmony = 0.3_f32;
        let regen = 0.05_f32;
        let fog_start = 6.0 + (1.0 - stress) * 10.0 + regen * 2.0;
        let fog_end = 28.0 + (1.0 - stress) * 24.0 + harmony * 8.0;
        assert!(fog_start < 10.0);
        assert!(fog_end < 40.0);
        assert!(fog_end > fog_start);
    }
}
