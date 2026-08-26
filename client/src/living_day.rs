/*!
 * Living Day — v22.12.0
 *
 * Light moves. Depths stay night. No clock to fear.
 * Cycle ~4 minutes so the first hour feels a dusk.
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
    mut clear: ResMut<ClearColor>,
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
        80.0 + 220.0 * light
    };
    if !abyss && day.night {
        let c = clear.0;
        clear.0 = Color::srgb(
            (c.to_srgba().red * 0.55).clamp(0.03, 1.0),
            (c.to_srgba().green * 0.50).clamp(0.03, 1.0),
            (c.to_srgba().blue * 0.62).clamp(0.04, 1.0),
        );
    }
}
