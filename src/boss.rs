use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Replicated)]
pub struct Boss {
    pub health: f32,
    pub max_health: f32,
    pub phase: u32,
    pub phase_timer: Timer,
}

#[derive(Event, Replicated)]
pub struct BossPhaseChange(pub u32);

pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BossPhaseChange>()
           .add_systems(Update, (boss_phase_system, boss_phase_effects));
    }
}

fn boss_phase_system(
    mut bosses: Query<&mut Boss>,
    time: Res<Time>,
    mut events: EventWriter<BossPhaseChange>,
) {
    for mut boss in &mut bosses {
        boss.phase_timer.tick(time.delta());
        if boss.phase_timer.finished() {
            boss.phase += 1;
            boss.max_health *= 1.5;
            boss.health = boss.max_health;
            events.send(BossPhaseChange(boss.phase));
            info!("Boss phase {} — mercy challenge escalates", boss.phase);
            boss.phase_timer = Timer::from_seconds(60.0, TimerMode::Once);
        }
    }
}

fn boss_phase_effects(
    mut events: EventReader<BossPhaseChange>,
    mut commands: Commands,
) {
    for event in events.read() {
        match event.0 {
            1 => {
                // Phase 1: Mercy wave
                info!("Phase 1 — mercy wave ripple");
            }
            2 => {
                // Phase 2: Lattice storm
                info!("Phase 2 — lattice storm");
            }
            _ => {
                // Final: Eternal bloom
                info!("Boss defeated — lattice eternal bloom");
            }
        }
    }
}
