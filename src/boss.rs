use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Replicated)]
pub struct Boss {
    pub health: f32,
    pub max_health: f32,
    pub phase: u32,
}

#[derive(Event, Replicated)]
pub struct BossDamageEvent(pub Entity, pub f32);  // Player, damage

pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BossDamageEvent>()
           .add_systems(Update, (boss_damage_system, boss_phase_system));
    }
}

fn boss_damage_system(
    mut events: EventReader<BossDamageEvent>,
    mut bosses: Query<&mut Boss>,
) {
    for event in events.read() {
        if let Ok(mut boss) = bosses.get_mut(event.0) {
            boss.health -= event.1;
            if boss.health <= 0.0 {
                boss.phase += 1;
                boss.health = boss.max_health * (boss.phase as f32 + 1.0);
                info!("Boss phase {} — mercy challenge escalates", boss.phase);
            }
        }
    }
}

fn boss_phase_system(
    bosses: Query<&Boss>,
) {
    for boss in &bosses {
        // Phase-specific effects (particles, audio, global mercy wave)
        info!("Boss phase {} active — lattice trembles", boss.phase);
    }
}
