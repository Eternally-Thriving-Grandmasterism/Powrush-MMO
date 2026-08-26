/*!
 * Living Day — v22.12.0
 *
 * Light moves. Depths stay night. No clock to fear.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::living_practice_loop::SoftPlayerRealm;

const DAY_SECS: f32 = 240.0;

#[derive(Resource, Debug)]
pub struct LivingDay {
    pub phase: f32,
    pub night: bool,
}

impl Default for LivingDay {
    fn default() -> Self {
        Self {
            phase: 0.18,
            night: false,
        }
    }
}

pub struct LivingDayPlugin;

impl Plugin for LivingDayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LivingDay>()
            .add_systems(Update, turn_the_clock);
    }
}

fn turn_the_clock(
    time: Res<Time>,
    realm: Res<SoftPlayerRealm>,
    mut day: ResMut<LivingDay>,
    mut ambient: ResMut<AmbientLight>,
) {
    day.phase = (day.phase + time.delta_seconds() / DAY_SECS) % 1.0;
    let abyss = realm.current == Some(3);
    let light = if abyss {
        0.22
    } else if day.phase < 0.42 {
        1.0
    } else if day.phase < 0.52 {
        0.55
    } else if day.phase < 0.88 {
        0.28
    } else {
        0.62
    };
    day.night = light < 0.40;
    ambient.brightness = if abyss {
        90.0
    } else {
        90.0 + 200.0 * light
    };
}
