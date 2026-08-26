/*!
 * Living Body — v22.11.0
 *
 * ARK weight + real lungs:
 *   carry too much unused vitality → walk slower
 *   sprint spends breath → must walk to recover
 * Never a health bar. Never a starve tick.
 *
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::harvest_feel::SoftRbePool;
use crate::input::PlayerInput;

#[derive(Resource, Debug)]
pub struct LivingBody {
    pub breath: f32,
    pub heavy: bool,
    pub winded: bool,
}

impl Default for LivingBody {
    fn default() -> Self {
        Self {
            breath: 1.0,
            heavy: false,
            winded: false,
        }
    }
}

impl LivingBody {
    pub fn carry_mul(&self) -> f32 {
        if self.heavy {
            0.82
        } else {
            1.0
        }
    }

    pub fn can_sprint(&self) -> bool {
        self.breath > 0.08 && !self.winded
    }
}

pub struct LivingBodyPlugin;

impl Plugin for LivingBodyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LivingBody>()
            .add_systems(Update, breathe_and_weigh);
    }
}

fn breathe_and_weigh(
    time: Res<Time>,
    input: Res<PlayerInput>,
    pool: Res<SoftRbePool>,
    mut body: ResMut<LivingBody>,
) {
    let dt = time.delta_seconds();
    body.heavy = pool.vitality >= 3.2;
    let moving = input.movement.length_squared() > 0.04;
    let want_sprint = input.sprint && moving && body.can_sprint();
    if want_sprint {
        body.breath = (body.breath - dt * 0.28).max(0.0);
        if body.breath <= 0.08 {
            body.winded = true;
        }
    } else {
        let recover = if moving { 0.16 } else { 0.28 };
        body.breath = (body.breath + dt * recover).min(1.0);
        if body.breath >= 0.42 {
            body.winded = false;
        }
    }
}
